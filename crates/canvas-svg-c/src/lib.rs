//! C FFI for canvas-svg. Standalone: no dependency on canvas-2d/canvas-c, so rendering
//! targets caller-supplied pixels rather than a 2D context.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[cfg(any(feature = "gl", feature = "vulkan", feature = "metal"))]
pub mod gpu;

pub struct SvgDocument(pub(crate) canvas_svg::SvgDocument);

pub struct SvgNode(pub(crate) canvas_svg::SvgElementHandle);

fn c_str_to_string(value: *const c_char) -> Option<String> {
    if value.is_null() {
        return None;
    }
    Some(unsafe { CStr::from_ptr(value) }.to_string_lossy().into_owned())
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_create() -> *mut SvgDocument {
    Box::into_raw(Box::new(SvgDocument(canvas_svg::SvgDocument::new())))
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_create_with_string(
    svg: *const c_char,
) -> *mut SvgDocument {
    let Some(svg) = c_str_to_string(svg) else {
        return std::ptr::null_mut();
    };
    match canvas_svg::SvgDocument::from_bytes(svg.as_bytes()) {
        Ok(doc) => Box::into_raw(Box::new(SvgDocument(doc))),
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_release(doc: *mut SvgDocument) {
    if doc.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(doc) };
}

/// `row_bytes` of 0 means tightly packed (`width * 4`).
fn render_into(
    doc: &mut SvgDocument,
    slice: &mut [u8],
    width: i32,
    height: i32,
    row_bytes: usize,
    scale: f32,
    shared: bool,
) {
    let info = skia_safe::ImageInfo::new_n32_premul(skia_safe::ISize::new(width, height), None);
    let row_bytes = if row_bytes == 0 {
        info.min_row_bytes()
    } else {
        row_bytes
    };
    if let Some(mut surface) =
        skia_safe::surface::surfaces::wrap_pixels(&info, slice, row_bytes, None)
    {
        let canvas = surface.canvas();
        // Callers reuse buffers across frames and drawing composites over existing pixels.
        canvas.clear(skia_safe::Color::TRANSPARENT);
        if shared {
            doc.0.draw(canvas, width, height, scale);
        } else {
            doc.0.render_frame(canvas, width, height, scale);
        }
    }
}

/// Promotes a node out of the static content; null clears. See `SvgDocument::set_layer`.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_set_layer(doc: *mut SvgDocument, id: *const c_char) {
    if doc.is_null() {
        return;
    }
    let doc = unsafe { &mut *doc };
    let id = c_str_to_string(id);
    doc.0.set_layer(id.as_deref());
}

/// Drops the cached static raster; call when anything outside the promoted node changes.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_invalidate_backdrop(doc: *mut SvgDocument) {
    if doc.is_null() {
        return;
    }
    let doc = unsafe { &mut *doc };
    doc.0.invalidate_backdrop();
}

/// Views share one recording per frame; direct node mutations then need `invalidate_frames`.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_set_frame_sharing(doc: *mut SvgDocument, enabled: bool) {
    if doc.is_null() {
        return;
    }
    unsafe { &mut *doc }.0.set_frame_sharing(enabled);
}

/// Drops the shared recordings after a mutation the document could not see.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_invalidate_frames(doc: *mut SvgDocument) {
    if doc.is_null() {
        return;
    }
    unsafe { &mut *doc }.0.invalidate_frames();
}

/// `width`/`height` are physical pixels; `scale` maps logical (`set_container_size`) units onto them.
/// A one-off snapshot, so it bypasses the frame cache that views share.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_render_to_buffer(
    doc: *mut SvgDocument,
    pixels: *mut u8,
    pixels_len: usize,
    width: i32,
    height: i32,
    scale: f32,
) {
    if doc.is_null() || pixels.is_null() || pixels_len == 0 || width <= 0 || height <= 0 {
        return;
    }
    let doc = unsafe { &mut *doc };
    let slice = unsafe { std::slice::from_raw_parts_mut(pixels, pixels_len) };
    render_into(doc, slice, width, height, 0, scale, false);
}

/// As above, into caller-owned pixels (e.g. a locked Android `Bitmap`) that may be row-padded.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_render_to_pixels(
    doc: *mut SvgDocument,
    pixels: *mut u8,
    pixels_len: usize,
    width: i32,
    height: i32,
    row_bytes: usize,
    scale: f32,
) {
    if doc.is_null() || pixels.is_null() || pixels_len == 0 || width <= 0 || height <= 0 {
        return;
    }
    let doc = unsafe { &mut *doc };
    let slice = unsafe { std::slice::from_raw_parts_mut(pixels, pixels_len) };
    render_into(doc, slice, width, height, row_bytes, scale, true);
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_set_container_size(
    doc: *mut SvgDocument,
    width: f32,
    height: f32,
) {
    if doc.is_null() {
        return;
    }
    let doc = unsafe { &mut *doc };
    doc.0.set_container_size(width, height);
}

/// Wrapped fresh each call with an empty children mirror.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_root(doc: *mut SvgDocument) -> *mut SvgNode {
    if doc.is_null() {
        return std::ptr::null_mut();
    }
    let doc = unsafe { &*doc };
    let typed = doc.0.root().typed();
    Box::into_raw(Box::new(SvgNode(canvas_svg::SvgElementHandle::new(typed))))
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_register_id(
    doc: *mut SvgDocument,
    id: *const c_char,
    node: *const SvgNode,
) {
    if doc.is_null() || node.is_null() {
        return;
    }
    let Some(id) = c_str_to_string(id) else {
        return;
    };
    let doc = unsafe { &mut *doc };
    let node = unsafe { &*node };
    doc.0.register_id(id, node.0.node.clone().into_node());
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_unregister_id(
    doc: *mut SvgDocument,
    id: *const c_char,
) {
    if doc.is_null() {
        return;
    }
    let Some(id) = c_str_to_string(id) else {
        return;
    };
    let doc = unsafe { &mut *doc };
    doc.0.unregister_id(&id);
}

/// Resolves an id registered either by us or by Skia's parser.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_get_element_by_id(
    doc: *mut SvgDocument,
    id: *const c_char,
) -> *mut SvgNode {
    if doc.is_null() {
        return std::ptr::null_mut();
    }
    let Some(id) = c_str_to_string(id) else {
        return std::ptr::null_mut();
    };
    let doc = unsafe { &mut *doc };
    match doc.0.get_element_by_id(&id) {
        Some(node) => {
            let typed = node.typed();
            Box::into_raw(Box::new(SvgNode(canvas_svg::SvgElementHandle::new(typed))))
        }
        None => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_has_animations(doc: *const SvgDocument) -> bool {
    if doc.is_null() {
        return false;
    }
    unsafe { &*doc }.0.has_animations()
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_animation_count(doc: *const SvgDocument) -> usize {
    if doc.is_null() {
        return 0;
    }
    unsafe { &*doc }.0.animation_count()
}

/// Seconds until every animation ends; negative if any repeats indefinitely.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_animation_duration(doc: *const SvgDocument) -> f64 {
    if doc.is_null() {
        return 0.0;
    }
    unsafe { &*doc }.0.animation_duration().unwrap_or(-1.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_current_time(doc: *const SvgDocument) -> f64 {
    if doc.is_null() {
        return 0.0;
    }
    unsafe { &*doc }.0.current_time()
}

/// Returns a bitmask: bit 0 while anything is still animating (keep scheduling frames),
/// bit 1 when a value changed this frame (redraw).
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_document_set_current_time(
    doc: *mut SvgDocument,
    seconds: f64,
) -> i32 {
    if doc.is_null() {
        return 0;
    }
    let applied = unsafe { &mut *doc }.0.advance(seconds);
    (applied.running as i32) | ((applied.changed as i32) << 1)
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_node_create(tag: *const c_char) -> *mut SvgNode {
    let Some(tag) = c_str_to_string(tag) else {
        return std::ptr::null_mut();
    };
    match canvas_svg::create_element(&tag) {
        Some(typed) => Box::into_raw(Box::new(SvgNode(canvas_svg::SvgElementHandle::new(typed)))),
        None => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_node_create_text(text: *const c_char) -> *mut SvgNode {
    let Some(text) = c_str_to_string(text) else {
        return std::ptr::null_mut();
    };
    let typed = canvas_svg::create_text_node(&text);
    Box::into_raw(Box::new(SvgNode(canvas_svg::SvgElementHandle::new(typed))))
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_node_release(node: *mut SvgNode) {
    if node.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(node) };
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_node_tag_name(node: *const SvgNode) -> *mut c_char {
    if node.is_null() {
        return std::ptr::null_mut();
    }
    let node = unsafe { &*node };
    CString::new(canvas_svg::tag_name(&node.0.node)).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_node_set_attribute(
    node: *mut SvgNode,
    name: *const c_char,
    value: *const c_char,
) -> bool {
    if node.is_null() {
        return false;
    }
    let (Some(name), Some(value)) = (c_str_to_string(name), c_str_to_string(value)) else {
        return false;
    };
    let node = unsafe { &mut *node };
    canvas_svg::set_attribute(&mut node.0.node, &name, &value)
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_node_get_attribute(
    node: *const SvgNode,
    name: *const c_char,
) -> *mut c_char {
    if node.is_null() {
        return std::ptr::null_mut();
    }
    let Some(name) = c_str_to_string(name) else {
        return std::ptr::null_mut();
    };
    let node = unsafe { &*node };
    match canvas_svg::get_attribute(&node.0.node, &name) {
        Some(value) => CString::new(value).unwrap_or_default().into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_string_destroy(value: *mut c_char) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { CString::from_raw(value) };
}

/// Does not take ownership of `child`: only its node handle is cloned into `parent`,
/// and the caller still releases `child` itself.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_node_append_child(
    parent: *mut SvgNode,
    child: *const SvgNode,
) -> bool {
    if parent.is_null() || child.is_null() {
        return false;
    }
    let parent = unsafe { &mut *parent };
    let child = unsafe { &*child };
    parent.0.append_child(&child.0).is_ok()
}

/// A text node's text, or null for any other node. Free with `canvas_native_string_destroy`.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_node_get_text(node: *const SvgNode) -> *mut c_char {
    if node.is_null() {
        return std::ptr::null_mut();
    }
    let node = unsafe { &*node };
    match canvas_svg::text(&node.0.node) {
        Some(text) => CString::new(text).unwrap_or_default().into_raw(),
        None => std::ptr::null_mut(),
    }
}

/// False for any node that is not a text node.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_node_set_text(node: *mut SvgNode, text: *const c_char) -> bool {
    if node.is_null() {
        return false;
    }
    let text = c_str_to_string(text).unwrap_or_default();
    let node = unsafe { &mut *node };
    canvas_svg::set_text(&mut node.0.node, &text)
}

/// Returns a fresh `SvgNode` the caller owns, or null on an out-of-range index.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_node_remove_child(
    parent: *mut SvgNode,
    index: usize,
) -> *mut SvgNode {
    if parent.is_null() {
        return std::ptr::null_mut();
    }
    let parent = unsafe { &mut *parent };
    match parent.0.remove_child(index) {
        Ok(typed) => Box::into_raw(Box::new(SvgNode(canvas_svg::SvgElementHandle::new(typed)))),
        Err(_) => std::ptr::null_mut(),
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;

    const SMIL: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="150" height="150" viewBox="0 0 150 150">
        <circle cx="30" cy="30" r="10" fill="crimson"><animate attributeName="r" values="6;18;6" dur="1.5s" repeatCount="indefinite"/></circle>
        <rect x="90" y="15" width="30" height="30" fill="seagreen"/>
    </svg>"#;

    fn opaque(pixels: &[u8]) -> usize {
        pixels.chunks(4).filter(|p| p[3] != 0).count()
    }

    fn snapshot(doc: &mut SvgDocument, w: i32, h: i32, scale: f32) -> usize {
        let mut buf = vec![0u8; (w * h * 4) as usize];
        canvas_native_svg_document_render_to_buffer(doc, buf.as_mut_ptr(), buf.len(), w, h, scale);
        opaque(&buf)
    }

    #[test]
    fn snapshot_of_a_shared_animating_document_draws() {
        let mut doc = SvgDocument(canvas_svg::SvgDocument::from_bytes(SMIL.as_bytes()).unwrap());
        doc.0.set_frame_sharing(true);
        doc.0.set_container_size(150., 150.);
        for t in 0..5 {
            doc.0.advance(t as f64 * 0.1);
            // A view records its frame first, as the render-thread commit does.
            let _ = doc.0.frame(403, 403, 2.685);
            assert!(snapshot(&mut doc, 270, 270, 1.8) > 0, "whole view, frame {t}");
            assert!(snapshot(&mut doc, 540, 540, 3.6) > 0, "crop size, frame {t}");
        }
    }
}
