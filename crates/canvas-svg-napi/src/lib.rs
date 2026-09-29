//! `global.SVGModule` for Node-API hosts: the same `SVGDocument` / `SVGNode` classes and module
//! functions the V8 bindings install (packages/canvas-svg/platforms/ios/src/cpp/SVG*), over
//! canvas-svg-c, so packages/canvas-svg's document layer (NativeNode.ts) runs unchanged.

#![allow(non_snake_case)]

use std::ffi::{c_char, CStr, CString};
use std::ptr;

use canvas_svg_c::{SvgDocument, SvgNode};
use napi::bindgen_prelude::{ClassInstance, FromNapiValue, JsObjectValue, Object, ObjectFinalize, Unknown, ValidateNapiValue};
use napi::{sys, Env, JsValue, Result};
use napi_derive::napi;

fn c_string(value: &str) -> Option<CString> {
  CString::new(value).ok()
}

/// Takes a string canvas-svg-c allocated.
fn take_string(value: *mut c_char) -> Option<String> {
  if value.is_null() {
    return None;
  }
  let string = unsafe { CStr::from_ptr(value) }.to_string_lossy().into_owned();
  canvas_svg_c::canvas_native_string_destroy(value);
  Some(string)
}

/// The bytes of any typed array (read and written in place).
fn typed_array_bytes<'a>(env: sys::napi_env, value: &Unknown) -> Option<&'a mut [u8]> {
  let mut is_typed_array = false;
  unsafe { sys::napi_is_typedarray(env, value.raw(), &mut is_typed_array) };
  if !is_typed_array {
    return None;
  }
  let (mut kind, mut length, mut data, mut buffer, mut offset) = (0, 0usize, ptr::null_mut(), ptr::null_mut(), 0usize);
  let status = unsafe { sys::napi_get_typedarray_info(env, value.raw(), &mut kind, &mut length, &mut data, &mut buffer, &mut offset) };
  if status != sys::Status::napi_ok || data.is_null() {
    return None;
  }
  let element = match kind {
    sys::TypedarrayType::int8_array | sys::TypedarrayType::uint8_array | sys::TypedarrayType::uint8_clamped_array => 1,
    sys::TypedarrayType::int16_array | sys::TypedarrayType::uint16_array => 2,
    sys::TypedarrayType::int32_array | sys::TypedarrayType::uint32_array | sys::TypedarrayType::float32_array => 4,
    _ => 8,
  };
  Some(unsafe { std::slice::from_raw_parts_mut(data as *mut u8, length * element) })
}

/// An element or text node. Owns its handle; the tree keeps the node alive while it is attached.
#[napi(js_name = "SVGNode", custom_finalize)]
pub struct SVGNode {
  node: *mut SvgNode,
}

impl ObjectFinalize for SVGNode {
  fn finalize(self, _: Env) -> Result<()> {
    canvas_svg_c::canvas_native_svg_node_release(self.node);
    Ok(())
  }
}

impl SVGNode {
  fn wrap(node: *mut SvgNode) -> Option<SVGNode> {
    (!node.is_null()).then_some(SVGNode { node })
  }
}

#[napi]
impl SVGNode {
  #[napi]
  pub fn tag_name(&self) -> Option<String> {
    take_string(canvas_svg_c::canvas_native_svg_node_tag_name(self.node))
  }

  #[napi]
  pub fn set_attribute(&self, name: Unknown, value: Unknown) -> Result<bool> {
    let (Some(name), Some(value)) = (as_string(&name)?, to_string(&value)?) else {
      return Ok(false);
    };
    let (Some(name), Some(value)) = (c_string(&name), c_string(&value)) else {
      return Ok(false);
    };
    Ok(canvas_svg_c::canvas_native_svg_node_set_attribute(self.node, name.as_ptr(), value.as_ptr()))
  }

  #[napi]
  pub fn get_attribute(&self, name: Unknown) -> Result<Option<String>> {
    let Some(name) = as_string(&name)?.and_then(|name| c_string(&name)) else {
      return Ok(None);
    };
    Ok(take_string(canvas_svg_c::canvas_native_svg_node_get_attribute(self.node, name.as_ptr())))
  }

  #[napi]
  pub fn append_child(&self, child: Unknown) -> bool {
    match unwrap::<SVGNode>(&child) {
      Some(child) => canvas_svg_c::canvas_native_svg_node_append_child(self.node, child.node),
      None => false,
    }
  }

  /// Detaches the child at `index` and returns it.
  #[napi]
  pub fn remove_child(&self, index: u32) -> Option<SVGNode> {
    SVGNode::wrap(canvas_svg_c::canvas_native_svg_node_remove_child(self.node, index as usize))
  }

  /// A text node's text; null for any other node.
  #[napi]
  pub fn text(&self) -> Option<String> {
    take_string(canvas_svg_c::canvas_native_svg_node_get_text(self.node))
  }

  #[napi]
  pub fn set_text(&self, text: Unknown) -> Result<bool> {
    let text = as_string(&text)?.unwrap_or_default();
    let Some(text) = c_string(&text) else {
      return Ok(false);
    };
    Ok(canvas_svg_c::canvas_native_svg_node_set_text(self.node, text.as_ptr()))
  }
}

/// A parsed (or empty) SVG document.
#[napi(js_name = "SVGDocument", custom_finalize)]
pub struct SVGDocument {
  document: *mut SvgDocument,
}

impl ObjectFinalize for SVGDocument {
  fn finalize(self, _: Env) -> Result<()> {
    canvas_svg_c::canvas_native_svg_document_release(self.document);
    Ok(())
  }
}

fn create_document(source: Option<&str>) -> *mut SvgDocument {
  match source {
    Some(source) => match c_string(source) {
      Some(source) => canvas_svg_c::canvas_native_svg_document_create_with_string(source.as_ptr()),
      None => ptr::null_mut(),
    },
    None => canvas_svg_c::canvas_native_svg_document_create(),
  }
}

#[napi]
impl SVGDocument {
  /// `new SVGDocument(source?)`; throws when `source` does not parse (`createSVGDocument`
  /// returns null instead).
  #[napi(constructor)]
  pub fn new(source: Option<String>) -> Result<Self> {
    let document = create_document(source.as_deref());
    if document.is_null() {
      return Err(napi::Error::from_reason("Could not parse the SVG document"));
    }
    Ok(Self { document })
  }

  #[napi]
  pub fn root(&self) -> Option<SVGNode> {
    SVGNode::wrap(canvas_svg_c::canvas_native_svg_document_root(self.document))
  }

  #[napi]
  pub fn create_element(&self, tag: Unknown) -> Result<Option<SVGNode>> {
    create_element(tag)
  }

  #[napi]
  pub fn create_text_node(&self, text: Unknown) -> Result<Option<SVGNode>> {
    create_text_node(text)
  }

  #[napi]
  pub fn get_element_by_id(&self, id: Unknown) -> Result<Option<SVGNode>> {
    let Some(id) = as_string(&id)?.and_then(|id| c_string(&id)) else {
      return Ok(None);
    };
    Ok(SVGNode::wrap(canvas_svg_c::canvas_native_svg_document_get_element_by_id(self.document, id.as_ptr())))
  }

  #[napi]
  pub fn register_id(&self, id: Unknown, node: Unknown) -> Result<()> {
    let (Some(id), Some(node)) = (as_string(&id)?.and_then(|id| c_string(&id)), unwrap::<SVGNode>(&node)) else {
      return Ok(());
    };
    canvas_svg_c::canvas_native_svg_document_register_id(self.document, id.as_ptr(), node.node);
    Ok(())
  }

  #[napi]
  pub fn unregister_id(&self, id: Unknown) -> Result<()> {
    if let Some(id) = as_string(&id)?.and_then(|id| c_string(&id)) {
      canvas_svg_c::canvas_native_svg_document_unregister_id(self.document, id.as_ptr());
    }
    Ok(())
  }

  #[napi]
  pub fn set_container_size(&self, width: f64, height: f64) {
    canvas_svg_c::canvas_native_svg_document_set_container_size(self.document, width as f32, height as f32);
  }

  /// Promotes a node out of the static content; anything but a string clears it.
  #[napi]
  pub fn set_layer(&self, id: Unknown) -> Result<()> {
    match as_string(&id)?.and_then(|id| c_string(&id)) {
      Some(id) => canvas_svg_c::canvas_native_svg_document_set_layer(self.document, id.as_ptr()),
      None => canvas_svg_c::canvas_native_svg_document_set_layer(self.document, ptr::null()),
    }
    Ok(())
  }

  #[napi]
  pub fn invalidate_backdrop(&self) {
    canvas_svg_c::canvas_native_svg_document_invalidate_backdrop(self.document);
  }

  #[napi]
  pub fn set_frame_sharing(&self, enabled: Option<bool>) {
    canvas_svg_c::canvas_native_svg_document_set_frame_sharing(self.document, enabled.unwrap_or(false));
  }

  #[napi]
  pub fn invalidate_frames(&self) {
    canvas_svg_c::canvas_native_svg_document_invalidate_frames(self.document);
  }

  /// The canvas-svg-c document pointer (a native renderer takes it), as a decimal string like
  /// the V8 bindings: a double can't hold every 64-bit pointer.
  #[napi]
  pub fn native_pointer(&self) -> String {
    (self.document as usize).to_string()
  }

  /// Renders the current frame into `buffer` (any typed array, `width` x `height` premultiplied
  /// pixels): RGBA, or BGRA with `bgra` (Windows' XAML bitmaps), whatever the platform's order.
  #[napi(ts_args_type = "buffer: ArrayBufferView, width: number, height: number, scale?: number, bgra?: boolean")]
  pub fn render_to_buffer(&self, env: Env, buffer: Unknown, width: i32, height: i32, scale: Option<f64>, bgra: Option<bool>) {
    let Some(pixels) = typed_array_bytes(env.raw(), &buffer) else {
      return;
    };
    canvas_svg_c::canvas_native_svg_document_render_to_buffer_ordered(
      self.document,
      pixels.as_mut_ptr(),
      pixels.len(),
      width,
      height,
      scale.unwrap_or(1.) as f32,
      bgra.unwrap_or(false),
    );
  }

  #[napi]
  pub fn has_animations(&self) -> bool {
    canvas_svg_c::canvas_native_svg_document_has_animations(self.document)
  }

  /// CSS `@keyframes` from a stylesheet outside the document.
  #[napi]
  pub fn add_stylesheet(&self, css: Unknown) -> Result<bool> {
    let Some(css) = as_string(&css)?.and_then(|css| c_string(&css)) else {
      return Ok(false);
    };
    Ok(canvas_svg_c::canvas_native_svg_document_add_stylesheet(self.document, css.as_ptr()))
  }

  /// Seconds until every animation has finished; -1 when one repeats forever.
  #[napi]
  pub fn animation_duration(&self) -> f64 {
    canvas_svg_c::canvas_native_svg_document_animation_duration(self.document)
  }

  #[napi]
  pub fn current_time(&self) -> f64 {
    canvas_svg_c::canvas_native_svg_document_current_time(self.document)
  }

  /// Bit 0: still animating; bit 1: something changed.
  #[napi]
  pub fn set_current_time(&self, seconds: Option<f64>) -> i32 {
    canvas_svg_c::canvas_native_svg_document_set_current_time(self.document, seconds.unwrap_or(0.)) as i32
  }
}

/// `createSVGDocument(source?)`: null when `source` does not parse.
#[napi(js_name = "createSVGDocument")]
pub fn create_svg_document(source: Option<Unknown>) -> Result<Option<SVGDocument>> {
  let source = match source.as_ref() {
    Some(value) => as_string(value)?,
    None => None,
  };
  let document = create_document(source.as_deref());
  Ok((!document.is_null()).then_some(SVGDocument { document }))
}

#[napi(js_name = "createElement")]
pub fn create_element(tag: Unknown) -> Result<Option<SVGNode>> {
  let Some(tag) = as_string(&tag)?.and_then(|tag| c_string(&tag)) else {
    return Ok(None);
  };
  Ok(SVGNode::wrap(canvas_svg_c::canvas_native_svg_node_create(tag.as_ptr())))
}

#[napi(js_name = "createTextNode")]
pub fn create_text_node(text: Unknown) -> Result<Option<SVGNode>> {
  let Some(text) = as_string(&text)?.and_then(|text| c_string(&text)) else {
    return Ok(None);
  };
  Ok(SVGNode::wrap(canvas_svg_c::canvas_native_svg_node_create_text(text.as_ptr())))
}

/// A string argument, or None for anything else (the V8 bindings' `IsString()` checks).
fn as_string(value: &Unknown) -> Result<Option<String>> {
  if value.get_type()? != napi::ValueType::String {
    return Ok(None);
  }
  Ok(Some(unsafe { value.cast::<napi::JsString>() }?.into_utf8()?.as_str()?.to_owned()))
}

/// Any value converted as JS `String(value)` would (attribute values).
fn to_string(value: &Unknown) -> Result<Option<String>> {
  match value.get_type()? {
    napi::ValueType::Undefined => Ok(None),
    _ => Ok(Some(value.coerce_to_string()?.into_utf8()?.as_str()?.to_owned())),
  }
}

/// The native object behind a wrapper of class `T`, if `value` is one (type-checked).
fn unwrap<'a, T: 'a>(value: &Unknown<'a>) -> Option<ClassInstance<'a, T>>
where
  ClassInstance<'a, T>: FromNapiValue + ValidateNapiValue,
{
  let raw = value.value();
  unsafe {
    <ClassInstance<'a, T> as ValidateNapiValue>::validate(raw.env, raw.value).ok()?;
    <ClassInstance<'a, T> as FromNapiValue>::from_napi_value(raw.env, raw.value).ok()
  }
}

/// Like the V8 bindings' `install()`: `globalThis.SVGModule = exports` unless one is already
/// installed.
#[napi(module_exports)]
pub fn install_global(exports: Object, env: Env) -> Result<()> {
  let mut global = env.get_global()?;
  if !global.has_named_property("SVGModule")? {
    global.set_named_property("SVGModule", exports)?;
  }
  Ok(())
}
