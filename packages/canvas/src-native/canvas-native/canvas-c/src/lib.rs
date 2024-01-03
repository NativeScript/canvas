#![allow(non_snake_case)]

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::io::{Read, Write};
use std::os::raw::{c_char, c_int, c_uint};
use std::os::raw::c_ulong;
use std::os::raw::c_void;
use std::sync::Arc;

use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock};

use canvas_2d::context::{Context, ContextWrapper};
use canvas_2d::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_2d::context::drawing_paths::fill_rule::FillRule;
use canvas_2d::context::fill_and_stroke_styles::paint::paint_style_set_color_with_string;
pub use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_2d::context::image_smoothing::ImageSmoothingQuality;
use canvas_2d::context::line_styles::line_cap::LineCap;
use canvas_2d::context::line_styles::line_join::LineJoin;
use canvas_2d::context::text_styles::text_align::TextAlign;
use canvas_2d::context::text_styles::text_baseline::TextBaseLine;
use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::utils::color::{parse_color, to_parsed_color};
use canvas_2d::utils::image::{from_backend_texture, from_bitmap_slice, from_image_slice, from_image_slice_encoded, from_image_slice_no_copy};
use canvas_core::image_asset::OutputFormat;
use canvas_webgl::prelude::WebGLVersion;
#[cfg(target_os = "android")]
use once_cell::sync::OnceCell;

use crate::buffers::{F32Buffer, I32Buffer, StringBuffer, U32Buffer, U8Buffer};

/* Utils */

#[cfg(target_os = "android")]
pub static API_LEVEL: OnceCell<i32> = OnceCell::new();

#[cfg(target_os = "android")]
pub mod choreographer;

mod buffers;
#[cfg(any(target_os = "android", target_os = "ios"))]
mod raf;

pub mod url;

/* Raf */
#[cfg(any(target_os = "android", target_os = "ios"))]
#[derive(Clone)]
pub struct Raf(raf::Raf);
/* Raf */

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum GLConstants {
    UNPACK_FLIP_Y_WEBGL = 0x9240,
    UNPACK_PREMULTIPLY_ALPHA_WEBGL = 0x9241,
    UNPACK_COLORSPACE_CONVERSION_WEBGL = 0x9243,
}

#[repr(C)]
pub enum InvalidateState {
    InvalidateStateNone,
    InvalidateStatePending,
    InvalidateStateInvalidating,
}

#[repr(C)]
pub enum PaintStyleType {
    PaintStyleTypeNone,
    PaintStyleTypeColor,
    PaintStyleTypeGradient,
    PaintStyleTypePattern,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum ImageBitmapPremultiplyAlpha {
    ImageBitmapPremultiplyAlphaDefault,
    ImageBitmapPremultiplyAlphaPremultiply,
    ImageBitmapPremultiplyAlphaNone,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum ImageBitmapColorSpaceConversion {
    ImageBitmapColorSpaceConversionDefault,
    ImageBitmapColorSpaceConversionNone,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum ImageBitmapResizeQuality {
    ImageBitmapResizeQualityLow,
    ImageBitmapResizeQualityMedium,
    ImageBitmapResizeQualityHigh,
    ImageBitmapResizeQualityPixelated,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum WebGLResultType {
    WebGLResultTypeBoolean,
    WebGLResultTypeI32Array,
    WebGLResultTypeU32Array,
    WebGLResultTypeF32Array,
    WebGLResultTypeBooleanArray,
    WebGLResultTypeU32,
    WebGLResultTypeI32,
    WebGLResultTypeF32,
    WebGLResultTypeString,
    WebGLResultTypeNone,
}

#[derive(Clone)]
pub struct ImageFilter(canvas_2d::context::filters::ImageFilter);

#[no_mangle]
pub extern "C" fn canvas_native_image_filter_destroy(value: *mut ImageFilter) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

/* Helpers */

#[no_mangle]
pub extern "C" fn canvas_native_font_add_family(
    alias: *const c_char,
    filenames: *const *const c_char,
    length: usize,
) {
    let names = unsafe { std::slice::from_raw_parts(filenames, length) };
    let names = names
        .iter()
        .map(|value| unsafe { CStr::from_ptr(*value).to_string_lossy().to_string() })
        .collect::<Vec<_>>();
    let tmp = names.iter().map(String::as_ref).collect::<Vec<&str>>();
    if alias.is_null() {
        let _ = canvas_2d::context::drawing_text::global_fonts::FontLibrary::add_family(
            None,
            tmp.as_slice(),
        );
    } else {
        let alias = unsafe { CStr::from_ptr(alias) };
        let alias = alias.to_string_lossy();
        let _ = canvas_2d::context::drawing_text::global_fonts::FontLibrary::add_family(
            Some(alias.as_ref()),
            tmp.as_slice(),
        );
    }
}

#[derive(Clone, Default)]
pub struct FileHelper {
    data: Option<U8Buffer>,
    error: Option<String>,
}

#[no_mangle]
pub extern "C" fn canvas_native_helper_destroy(value: *mut FileHelper) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[no_mangle]
pub extern "C" fn canvas_native_helper_read_file(path: *const c_char) -> *mut FileHelper {
    assert!(!path.is_null());
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();
    let mut ret = FileHelper::default();
    match std::fs::File::open(path.as_ref()) {
        Ok(mut file) => {
            let size = file.metadata().map(|meta| meta.len()).unwrap_or(0) as usize;
            let mut buf = Vec::with_capacity(size);
            let _ = file.read_to_end(&mut buf);
            let buf = U8Buffer::from(buf);
            ret.data = Some(buf);
        }
        Err(error) => ret.error = Some(error.to_string()),
    }
    Box::into_raw(Box::new(ret))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_read_file_has_error(file: *const FileHelper) -> bool {
    if file.is_null() {
        return false;
    };
    let file = &*file;
    return file.error.is_some();
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_read_file_get_data(
    file: *mut FileHelper,
) -> *mut U8Buffer {
    assert!(!file.is_null());
    let file = &*file;
    let data = match &file.data {
        None => U8Buffer::default(),
        Some(data) => data.clone(),
    };
    Box::into_raw(Box::new(data))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_read_file_get_error(
    file: *const FileHelper,
) -> *const c_char {
    if file.is_null() {
        return std::ptr::null();
    };
    let file = &*file;
    match file.error.as_ref() {
        None => std::ptr::null(),
        Some(data) => CString::new(data.as_str()).unwrap().into_raw(),
    }
}

/* Helpers */

/* Utils */

/* TextEncoder */
#[derive(Clone)]
pub struct TextEncoder(canvas_2d::context::text_encoder::TextEncoder);

#[no_mangle]
pub extern "C" fn canvas_native_text_encoder_destroy(value: *mut TextEncoder) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

/* TextEncoder */

/* TextDecoder */
#[derive(Clone)]
pub struct TextDecoder(canvas_2d::context::text_decoder::TextDecoder);

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_destroy(value: *mut TextDecoder) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

/* TextDecoder */

/* CanvasRenderingContext2D */

#[allow(dead_code)]
pub struct CanvasRenderingContext2D {
    context: ContextWrapper,
    gl_context: canvas_core::gl::GLContext,
    alpha: bool,
}

#[no_mangle]
pub extern "C" fn canvas_native_context_destroy(value: *mut CanvasRenderingContext2D) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

fn to_data_url(context: &mut CanvasRenderingContext2D, format: &str, quality: u32) -> String {
    canvas_2d::to_data_url(&mut context.context, format, quality)
}

impl CanvasRenderingContext2D {
    pub fn new(
        context: ContextWrapper,
        gl_context: canvas_core::gl::GLContext,
        alpha: bool,
    ) -> Self {
        Self {
            context,
            gl_context,
            alpha,
        }
    }

    pub fn reset(&mut self) {
        self.context.get_context_mut().reset();
    }

    pub fn render(&self) {
        self.gl_context.make_current();
        {
            self.get_context_mut().flush();
        }
        self.gl_context.swap_buffers();
    }

    pub fn get_context(&self) -> MappedRwLockReadGuard<Context> {
        self.context.get_context()
    }

    pub fn get_context_mut(&self) -> MappedRwLockWriteGuard<Context> {
        self.context.get_context_mut()
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.gl_context.make_current();
        self.context.resize_gl(width, height);
    }

    pub fn make_current(&self) -> bool {
        self.gl_context.make_current()
    }

    pub fn swap_buffers(&self) -> bool {
        self.gl_context.swap_buffers()
    }

    pub fn remove_if_current(&self) {
        self.gl_context.remove_if_current();
    }
}

#[derive(Clone)]
pub struct PaintStyle(Option<canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle>);

impl PaintStyle {
    // useful when creating styles using platform images e.g Bitmap/UIImage ... test??
    pub fn from_ptr(ptr: i64) -> Self {
        if ptr == 0 {
            return Self::new(None);
        }

        unsafe { *Box::from_raw(ptr as *mut PaintStyle) }
    }

    pub fn new_with_color(color: *const c_char) -> Self {
        assert!(!color.is_null());
        let color = unsafe { CStr::from_ptr(color) };
        let color = color.to_string_lossy();
        Self(
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::new_color_str(
                color.as_ref(),
            ),
        )
    }

    pub fn new(
        style: Option<canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle>,
    ) -> Self {
        Self(style)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    #[inline]
    pub fn style_type(&self) -> PaintStyleType {
        if let Some(style) = self.0.as_ref() {
            return match style {
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(_) => {
                    PaintStyleType::PaintStyleTypeColor
                }
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
                    PaintStyleType::PaintStyleTypeGradient
                }
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
                    PaintStyleType::PaintStyleTypePattern
                }
            };
        }
        return PaintStyleType::PaintStyleTypeNone;
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_destroy(value: *mut PaintStyle) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[derive(Clone, Copy)]
pub struct TextMetrics(canvas_2d::context::drawing_text::text_metrics::TextMetrics);

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_destroy(value: *mut TextMetrics) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

/* Raf */
#[cfg(any(target_os = "android", target_os = "ios"))]
#[no_mangle]
pub extern "C" fn canvas_native_raf_create(
    callback: isize,
    on_frame_callback: Option<extern "C" fn(callback: isize, ts: i64)>,
) -> *mut Raf {
    Box::into_raw(Box::new(Raf(raf::Raf::new(Some(Box::new(move |ts| {
        if let Some(on_frame_callback) = on_frame_callback {
            on_frame_callback(callback, ts)
        }
    }))))))
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[no_mangle]
pub extern "C" fn canvas_native_raf_destroy(value: *mut Raf) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[no_mangle]
pub extern "C" fn canvas_native_raf_start(raf: *mut Raf) {
    if raf.is_null() {
        return;
    }
    let raf = unsafe { &mut *raf };
    raf.0.start();
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[no_mangle]
pub extern "C" fn canvas_native_raf_stop(raf: *mut Raf) {
    if raf.is_null() {
        return;
    }
    let raf = unsafe { &mut *raf };
    raf.0.stop()
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[no_mangle]
pub extern "C" fn canvas_native_raf_get_started(raf: *const Raf) -> bool {
    if raf.is_null() {
        return false;
    }
    let raf = unsafe { &*raf };
    raf.0.started()
}
/* Raf */

impl Into<i32> for ImageBitmapPremultiplyAlpha {
    fn into(self) -> i32 {
        if self == ImageBitmapPremultiplyAlpha::ImageBitmapPremultiplyAlphaPremultiply {
            return 1;
        } else if self == ImageBitmapPremultiplyAlpha::ImageBitmapPremultiplyAlphaNone {
            return 2;
        }
        return 0;
    }
}

impl Into<i32> for ImageBitmapColorSpaceConversion {
    fn into(self) -> i32 {
        if self == ImageBitmapColorSpaceConversion::ImageBitmapColorSpaceConversionNone {
            return 1;
        }
        0
    }
}

impl Into<i32> for ImageBitmapResizeQuality {
    fn into(self) -> i32 {
        if self == ImageBitmapResizeQuality::ImageBitmapResizeQualityMedium {
            return 1;
        } else if self == ImageBitmapResizeQuality::ImageBitmapResizeQualityHigh {
            return 2;
        } else if self == ImageBitmapResizeQuality::ImageBitmapResizeQualityPixelated {
            return 3;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_with_wrapper(
    context: i64,
    gl_context: i64,
) -> *mut CanvasRenderingContext2D {
    let wrapper = context as *mut ContextWrapper;
    let wrapper = unsafe { &mut *wrapper };

    let gl_context = gl_context as *const RwLock<canvas_core::gl::GLContextInner>;
    let gl_context = canvas_core::gl::GLContext::from_raw_inner(gl_context);
    let alpha;
    {
        let lock = wrapper.get_context();
        alpha = lock.device().alpha;
    }
    let clone = ContextWrapper::from_inner(Arc::clone(wrapper.get_inner()));
    Box::into_raw(Box::new(CanvasRenderingContext2D {
        context: clone,
        gl_context,
        alpha,
    }))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_resize(
    context: *mut CanvasRenderingContext2D,
    width: f32,
    height: f32,
) {
    let context = unsafe { &mut *context };
    context.resize(width, height);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create(
    width: f32,
    height: f32,
    density: f32,
    alpha: bool,
    font_color: i32,
    ppi: f32,
    direction: u32,
) -> *mut CanvasRenderingContext2D {
    Box::into_raw(Box::new(CanvasRenderingContext2D {
        context: ContextWrapper::new(Context::new(
            width,
            height,
            density,
            alpha,
            font_color,
            ppi,
            TextDirection::from(direction),
        )),
        gl_context: canvas_core::gl::GLContext::default(),
        alpha,
    }))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_gl(
    width: f32,
    height: f32,
    density: f32,
    gl_context: i64,
    samples: i32,
    alpha: bool,
    font_color: i32,
    ppi: f32,
    direction: u32,
) -> *mut CanvasRenderingContext2D {
    let gl_context = gl_context as *const RwLock<canvas_core::gl::GLContextInner>;
    let gl_context = canvas_core::gl::GLContext::from_raw_inner(gl_context);

    gl_context.make_current();
    let mut frame_buffers = [0];
    unsafe {
        gl_bindings::GetIntegerv(gl_bindings::FRAMEBUFFER_BINDING, frame_buffers.as_mut_ptr())
    };

    Box::into_raw(Box::new(CanvasRenderingContext2D {
        context: ContextWrapper::new(Context::new_gl(
            width,
            height,
            density,
            frame_buffers[0],
            samples,
            alpha,
            font_color,
            ppi,
            TextDirection::from(direction),
        )),
        gl_context,
        alpha,
    }))
}

#[cfg(feature = "webgl")]
#[no_mangle]
pub extern "C" fn canvas_native_context_create_with_pointer(
    pointer: i64,
) -> *mut CanvasRenderingContext2D {
    assert_ne!(pointer, 0);
    pointer as *mut CanvasRenderingContext2D
}

#[cfg(feature = "webgl")]
#[no_mangle]
pub extern "C" fn canvas_native_context_create_gl_no_window(
    width: f32,
    height: f32,
    density: f32,
    font_color: i32,
    ppi: f32,
    direction: u32,
    alpha: bool,
) -> *mut CanvasRenderingContext2D {
    canvas_native_webgl_create_no_window_internal(
        width as i32,
        height as i32,
        "canvas",
        alpha,
        false,
        false,
        false,
        "default",
        true,
        false,
        false,
        false,
        false,
        true,
    );

    let mut attr = canvas_core::context_attributes::ContextAttributes::new(
        alpha, false, false, false, "default", true, false, false, false, false, true,
    );

    let gl_context = canvas_core::gl::GLContext::create_offscreen_context(
        &mut attr,
        width as i32,
        height as i32,
    )
        .unwrap();

    gl_context.make_current();

    let mut buffer_id = [0i32];

    unsafe { gl_bindings::GetIntegerv(gl_bindings::FRAMEBUFFER_BINDING, buffer_id.as_mut_ptr()) }

    let context = ContextWrapper::new(Context::new_gl(
        width,
        height,
        density,
        buffer_id[0],
        0,
        alpha,
        font_color,
        ppi,
        TextDirection::from(direction),
    ));
    gl_context.remove_if_current();

    Box::into_raw(Box::new(CanvasRenderingContext2D {
        context,
        gl_context,
        alpha,
    }))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_filter(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = context.get_context().get_filter().to_string();

    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_filter(
    context: *mut CanvasRenderingContext2D,
    filter: *const c_char,
) {
    let context = unsafe { &mut *context };
    let filter = unsafe { CStr::from_ptr(filter) };
    context
        .get_context_mut()
        .set_filter(filter.to_string_lossy().as_ref())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_font(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = context.get_context().font().to_string();
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_font(
    context: *mut CanvasRenderingContext2D,
    font: *const c_char,
) {
    if font.is_null() {
        return;
    }
    let context = unsafe { &mut *context };
    let font = unsafe { CStr::from_ptr(font) };
    context
        .get_context_mut()
        .set_font(font.to_string_lossy().as_ref());
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_letter_spacing(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = context.get_context().get_letter_spacing().to_string();
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_letter_spacing(
    context: *mut CanvasRenderingContext2D,
    spacing: *const c_char,
) {
    if spacing.is_null() {
        return;
    }
    let context = unsafe { &mut *context };
    let spacing = unsafe { CStr::from_ptr(spacing) };
    context
        .get_context_mut()
        .set_letter_spacing(spacing.to_string_lossy().as_ref());
}


#[no_mangle]
pub extern "C" fn canvas_native_context_get_word_spacing(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = context.get_context().get_word_spacing().to_string();
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_word_spacing(
    context: *mut CanvasRenderingContext2D,
    spacing: *const c_char,
) {
    if spacing.is_null() {
        return;
    }
    let context = unsafe { &mut *context };
    let spacing = unsafe { CStr::from_ptr(spacing) };
    context
        .get_context_mut()
        .set_word_spacing(spacing.to_string_lossy().as_ref());
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_global_alpha(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    let context = unsafe { &*context };
    context.get_context().global_alpha()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_global_alpha(
    context: *mut CanvasRenderingContext2D,
    alpha: f32,
) {
    let context = unsafe { &mut *context };
    context.get_context_mut().set_global_alpha(alpha);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_image_smoothing_enabled(
    context: *const CanvasRenderingContext2D,
) -> bool {
    let context = unsafe { &*context };
    context.get_context().get_image_smoothing_enabled()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_image_smoothing_enabled(
    context: *mut CanvasRenderingContext2D,
    enabled: bool,
) {
    let context = unsafe { &mut *context };
    context
        .get_context_mut()
        .set_image_smoothing_enabled(enabled)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_image_smoothing_quality(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = match context.get_context().get_image_smoothing_quality() {
        ImageSmoothingQuality::Low => "low",
        ImageSmoothingQuality::Medium => "medium",
        ImageSmoothingQuality::High => "high",
    };
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_image_smoothing_quality(
    context: *mut CanvasRenderingContext2D,
    quality: *const c_char,
) {
    if quality.is_null() {
        return;
    }
    let context = unsafe { &mut *context };
    let quality = unsafe { CStr::from_ptr(quality) };
    match quality.to_string_lossy().as_ref() {
        "low" => context
            .get_context_mut()
            .set_image_smoothing_quality(ImageSmoothingQuality::Low),
        "medium" => context
            .get_context_mut()
            .set_image_smoothing_quality(ImageSmoothingQuality::Medium),
        "high" => context
            .get_context_mut()
            .set_image_smoothing_quality(ImageSmoothingQuality::High),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_join(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = match context.get_context().line_join() {
        LineJoin::JoinBevel => "bevel",
        LineJoin::JoinMiter => "miter",
        LineJoin::JoinRound => "round",
    };

    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_join(
    context: *mut CanvasRenderingContext2D,
    join: *const c_char,
) {
    if join.is_null() {
        return;
    }
    let context = unsafe { &mut *context };
    let join = unsafe { CStr::from_ptr(join) };
    match join.to_string_lossy().as_ref() {
        "bevel" => context.get_context_mut().set_line_join(LineJoin::JoinBevel),
        "miter" => context.get_context_mut().set_line_join(LineJoin::JoinMiter),
        "round" => context.get_context_mut().set_line_join(LineJoin::JoinRound),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_cap(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = match context.get_context().line_cap() {
        LineCap::CapRound => "round",
        LineCap::CapButt => "butt",
        LineCap::CapSquare => "square",
    };

    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_cap(
    context: *mut CanvasRenderingContext2D,
    cap: *const c_char,
) {
    if cap.is_null() {
        return;
    }
    let context = unsafe { &mut *context };
    let cap = unsafe { CStr::from_ptr(cap) };
    match cap.to_string_lossy().as_ref() {
        "round" => context.get_context_mut().set_line_cap(LineCap::CapRound),
        "butt" => context.get_context_mut().set_line_cap(LineCap::CapButt),
        "square" => context.get_context_mut().set_line_cap(LineCap::CapSquare),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_miter_limit(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    let context = unsafe { &*context };
    context.get_context().miter_limit()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_miter_limit(
    context: *mut CanvasRenderingContext2D,
    limit: f32,
) {
    let context = unsafe { &*context };
    context.get_context_mut().set_miter_limit(limit);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_color(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let value = context.get_context().shadow_color();
    let ret = to_parsed_color(value);
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_color_buf(
    context: *const CanvasRenderingContext2D,
) -> *mut U8Buffer {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let value = context.get_context().shadow_color();
    Box::into_raw(Box::new(U8Buffer::from(
        to_parsed_color(value).into_bytes(),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_color_rgba(
    context: *const CanvasRenderingContext2D,
    r: &mut u8,
    g: &mut u8,
    b: &mut u8,
    a: &mut u8,
) {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.get_context().shadow_color_rgba(r, g, b, a);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_color(
    context: *mut CanvasRenderingContext2D,
    color: *const c_char,
) {
    assert!(!context.is_null());
    assert!(!color.is_null());
    let context = unsafe { &mut *context };
    let mut lock = context.get_context_mut();
    let color = unsafe { CStr::from_ptr(color) };
    if let Some(color) = parse_color(color.to_string_lossy().as_ref()) {
        lock.set_shadow_color(color);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_color_rgba(
    context: *mut CanvasRenderingContext2D,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    let mut lock = context.get_context_mut();
    lock.set_shadow_color_rgba(r, g, b, a);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_blur(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.get_context().shadow_blur()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_blur(
    context: *mut CanvasRenderingContext2D,
    blur: f32,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.get_context_mut().set_shadow_blur(blur)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_offset_x(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.get_context().shadow_offset_x()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_offset_x(
    context: *mut CanvasRenderingContext2D,
    x: f32,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.get_context_mut().set_shadow_offset_x(x)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_offset_y(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.get_context().shadow_offset_y()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_offset_y(
    context: *mut CanvasRenderingContext2D,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context.get_context_mut().set_shadow_offset_y(y)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_text_align(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = match context.get_context().text_align() {
        TextAlign::START => "start",
        TextAlign::LEFT => "left",
        TextAlign::CENTER => "center",
        TextAlign::RIGHT => "right",
        TextAlign::END => "end",
    };
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_text_align(
    context: *mut CanvasRenderingContext2D,
    alignment: *const c_char,
) {
    if alignment.is_null() {
        return;
    }
    let context = unsafe { &mut *context };
    let alignment = unsafe { CStr::from_ptr(alignment) };
    match alignment.to_string_lossy().as_ref() {
        "start" => context.get_context_mut().set_text_align(TextAlign::START),
        "left" => context.get_context_mut().set_text_align(TextAlign::LEFT),
        "center" => context.get_context_mut().set_text_align(TextAlign::CENTER),
        "right" => context.get_context_mut().set_text_align(TextAlign::RIGHT),
        "end" => context.get_context_mut().set_text_align(TextAlign::END),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_text_baseline(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = match context.get_context().text_baseline() {
        TextBaseLine::ALPHABETIC => "alphabetic",
        TextBaseLine::BOTTOM => "bottom",
        TextBaseLine::HANGING => "hanging",
        TextBaseLine::IDEOGRAPHIC => "ideographic",
        TextBaseLine::MIDDLE => "middle",
        TextBaseLine::TOP => "top",
    };
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_text_baseline(
    context: *mut CanvasRenderingContext2D,
    baseline: *const c_char,
) {
    if baseline.is_null() {
        return;
    }
    let context = unsafe { &mut *context };
    let baseline = unsafe { CStr::from_ptr(baseline) };
    match baseline.to_string_lossy().as_ref() {
        "alphabetic" => context
            .get_context_mut()
            .set_text_baseline(TextBaseLine::ALPHABETIC),
        "bottom" => context.get_context_mut().set_text_baseline(TextBaseLine::BOTTOM),
        "hanging" => context.get_context_mut().set_text_baseline(TextBaseLine::HANGING),
        "ideographic" => context
            .get_context_mut()
            .set_text_baseline(TextBaseLine::IDEOGRAPHIC),
        "middle" => context.get_context_mut().set_text_baseline(TextBaseLine::MIDDLE),
        "top" => context.get_context_mut().set_text_baseline(TextBaseLine::TOP),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_global_composition(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = context.get_context().global_composite_operation().to_str();
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_global_composition(
    context: *mut CanvasRenderingContext2D,
    composition: *const c_char,
) {
    if composition.is_null() {
        return;
    }
    let composition = unsafe { CStr::from_ptr(composition) };
    if let Some(composition) =
        CompositeOperationType::from_str(composition.to_string_lossy().as_ref())
    {
        let context = unsafe { &mut *context };
        context
            .get_context_mut()
            .set_global_composite_operation(composition)
    }
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_paint_style_set_fill_color_with_c_string(
    context: *mut CanvasRenderingContext2D,
    color: *const c_char,
) {
    if color.is_null() {
        return;
    }
    let context = unsafe { &mut *context };
    let color = unsafe { CStr::from_ptr(color) };
    let color = color.to_string_lossy();
    paint_style_set_color_with_string(&mut context.context, true, color.as_ref());
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_set_stroke_color_with_c_string(
    context: *mut CanvasRenderingContext2D,
    color: *const c_char,
) {
    if color.is_null() {
        return;
    }
    let context = unsafe { &mut *context };
    let color = unsafe { CStr::from_ptr(color) };
    let color = color.to_string_lossy();
    paint_style_set_color_with_string(&mut context.context, false, color.as_ref());
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_paint_style_set_stroke_color_with_rgba(
    context: *mut CanvasRenderingContext2D,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    let context = unsafe { &mut *context };
    canvas_2d::context::fill_and_stroke_styles::paint::paint_style_set_color_with_rgba(
        &mut context.context,
        false,
        r,
        g,
        b,
        a,
    );
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_paint_style_set_fill_color_with_rgba(
    context: *mut CanvasRenderingContext2D,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    let context = unsafe { &mut *context };
    canvas_2d::context::fill_and_stroke_styles::paint::paint_style_set_color_with_rgba(
        &mut context.context,
        true,
        r,
        g,
        b,
        a,
    );
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_parse_css_color_rgba(
    value: *const c_char,
    r: &mut u8,
    g: &mut u8,
    b: &mut u8,
    a: &mut u8,
) -> bool {
    if value.is_null() {
        return false;
    }
    let value = unsafe { CStr::from_ptr(value) };
    canvas_2d::utils::color::parse_color_rgba(value.to_string_lossy().as_ref(), r, g, b, a)
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_paint_style_get_color_string(
    color: *const PaintStyle,
) -> *const c_char {
    if color.is_null() {
        return std::ptr::null();
    }
    let color = unsafe { &*color };
    if let Some(color) = color.0.as_ref() {
        return match color {
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(color) => {
                CString::new(to_parsed_color(color.clone()))
                    .unwrap()
                    .into_raw()
            }
            _ => std::ptr::null(),
        };
    }
    std::ptr::null()
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_get_current_stroke_color_string(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let lock = context.get_context();
    match lock.stroke_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            CString::new(to_parsed_color(*stroke)).unwrap().into_raw()
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            std::ptr::null()
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_get_current_stroke_color_buf(
    context: *const CanvasRenderingContext2D,
) -> *mut U8Buffer {
    let context = unsafe { &*context };
    let lock = context.get_context();
    let ret = match lock.stroke_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            to_parsed_color(*stroke).into_bytes()
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            Vec::with_capacity(0)
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            Vec::with_capacity(0)
        }
    };

    Box::into_raw(Box::new(U8Buffer::from(ret)))
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_get_current_stroke_color_r_g_b_a(
    context: *const CanvasRenderingContext2D,
    r: &mut u8,
    g: &mut u8,
    b: &mut u8,
    a: &mut u8,
) {
    let context = unsafe { &*context };
    let lock = context.get_context();
    match lock.stroke_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            *r = stroke.r();
            *g = stroke.g();
            *b = stroke.b();
            *a = stroke.a();
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {}
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_get_current_fill_color_r_g_b_a(
    context: *const CanvasRenderingContext2D,
    r: &mut u8,
    g: &mut u8,
    b: &mut u8,
    a: &mut u8,
) {
    let context = unsafe { &*context };
    let lock = context.get_context();
    match lock.fill_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            *r = stroke.r();
            *g = stroke.g();
            *b = stroke.b();
            *a = stroke.a();
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {}
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_get_current_fill_color_string(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let lock = context.get_context();
    match lock.fill_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            CString::new(to_parsed_color(*stroke)).unwrap().into_raw()
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            std::ptr::null()
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_get_current_fill_color_buf(
    context: *mut CanvasRenderingContext2D,
) -> *mut U8Buffer {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let lock = context.get_context();
    let ret = match lock.fill_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(stroke) => {
            to_parsed_color(*stroke).into_bytes()
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            Vec::with_capacity(0)
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            Vec::with_capacity(0)
        }
    };
    Box::into_raw(Box::new(U8Buffer::from(ret)))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_style_type(style: *const PaintStyle) -> PaintStyleType {
    assert!(!style.is_null());
    let style = unsafe { &*style };
    style.style_type()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_current_fill_style_type(
    context: *mut CanvasRenderingContext2D,
) -> PaintStyleType {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let lock = context.get_context();
    return match lock.fill_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(_) => {
            PaintStyleType::PaintStyleTypeColor
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            PaintStyleType::PaintStyleTypeGradient
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            PaintStyleType::PaintStyleTypePattern
        }
    };
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_current_stroke_style_type(
    context: *mut CanvasRenderingContext2D,
) -> PaintStyleType {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let lock = context.get_context();
    return match lock.fill_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(_) => {
            PaintStyleType::PaintStyleTypeColor
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            PaintStyleType::PaintStyleTypeGradient
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            PaintStyleType::PaintStyleTypePattern
        }
    };
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_context_get_fill_style(
    context: *const CanvasRenderingContext2D,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    Box::into_raw(Box::new(PaintStyle(Some(
        context.get_context().fill_style().clone(),
    ))))
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_context_set_fill_style(
    context: *mut CanvasRenderingContext2D,
    style: *const PaintStyle,
) {
    assert!(!context.is_null());
    assert!(!style.is_null());
    let context = unsafe { &mut *context };
    let style = unsafe { &*style };
    if !style.is_empty() {
        context
            .get_context_mut()
            .set_fill_style(style.0.clone().unwrap())
    }
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_context_get_stroke_style(
    context: *const CanvasRenderingContext2D,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    Box::into_raw(Box::new(PaintStyle(Some(
        context.get_context().stroke_style().clone(),
    ))))
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_context_set_stroke_style(
    context: *mut CanvasRenderingContext2D,
    style: *const PaintStyle,
) {
    assert!(!context.is_null());
    assert!(!style.is_null());
    let context = unsafe { &mut *context };
    let style = unsafe { &*style };
    if !style.is_empty() {
        context
            .get_context_mut()
            .set_stroke_style(style.0.clone().unwrap())
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_width(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.get_context().line_width()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_width(
    context: *mut CanvasRenderingContext2D,
    width: f32,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.get_context_mut().set_line_width(width);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_dash_offset(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.get_context().line_dash_offset()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_dash_offset(
    context: *mut CanvasRenderingContext2D,
    offset: f32,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.get_context_mut().set_line_dash_offset(offset)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_dash(
    context: *const CanvasRenderingContext2D,
) -> *mut F32Buffer {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    Box::into_raw(Box::new(F32Buffer::from(
        context.get_context().line_dash().to_vec(),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_dash(
    context: *mut CanvasRenderingContext2D,
    dash: *const f32,
    size: usize,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    let dash = unsafe { std::slice::from_raw_parts(dash, size) };
    context.get_context_mut().set_line_dash(dash)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_arc(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: bool,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context
        .get_context_mut()
        .arc(x, y, radius, start_angle, end_angle, anticlockwise)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_arc_to(
    context: *mut CanvasRenderingContext2D,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    radius: f32,
) {
    let context = unsafe { &mut *context };
    context.get_context_mut().arc_to(x1, y1, x2, y2, radius)
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_context_begin_path(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    context.get_context_mut().begin_path()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_bezier_curve_to(
    context: *mut CanvasRenderingContext2D,
    cp1x: f32,
    cp1y: f32,
    cp2x: f32,
    cp2y: f32,
    x: f32,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context
        .get_context_mut()
        .bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_clear_rect(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().clear_rect(x, y, width, height);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_clip(
    context: *mut CanvasRenderingContext2D,
    path: *mut Path,
    rule: *const c_char,
) {
    assert!(!rule.is_null());
    let rule = unsafe { CStr::from_ptr(rule) };
    let path = unsafe { &mut *path };
    if let Ok(rule) = FillRule::try_from(rule.to_string_lossy().as_ref()) {
        let context = unsafe { &mut *context };
        context.make_current();
        context
            .get_context_mut()
            .clip(Some(path.inner_mut()), Some(rule));
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_clip_rule(
    context: *mut CanvasRenderingContext2D,
    rule: *const c_char,
) {
    assert!(!context.is_null());
    assert!(!rule.is_null());
    let context = unsafe { &mut *context };
    let rule = unsafe { CStr::from_ptr(rule) };
    if let Ok(rule) = FillRule::try_from(rule.to_string_lossy().as_ref()) {
        context.make_current();
        context.get_context_mut().clip(None, Some(rule));
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_context_close_path(context: *mut CanvasRenderingContext2D) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.get_context_mut().close_path()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_image_data(
    width: i32,
    height: i32,
) -> *mut ImageData {
    Box::into_raw(Box::new(ImageData::new(Context::create_image_data(
        width, height,
    ))))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_image_data_with_data(
    width: i32,
    height: i32,
    data: *const u8,
    size: usize,
) -> *mut ImageData {
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    Box::into_raw(Box::new(ImageData::new(
        canvas_2d::context::pixel_manipulation::image_data::ImageData::new_with_data(
            width, height, data,
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_linear_gradient(
    context: *mut CanvasRenderingContext2D,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    Box::into_raw(Box::new(PaintStyle(Some(
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(
            context.get_context().create_linear_gradient(x0, y0, x1, y1),
        ),
    ))))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_conic_gradient(
    context: *mut CanvasRenderingContext2D,
    start_angle: f32,
    x: f32,
    y: f32,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    Box::into_raw(Box::new(PaintStyle(Some(
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(
            context
                .get_context()
                .create_conic_gradient(start_angle, x, y),
        ),
    ))))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern(
    context: *mut CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    width: i32,
    height: i32,
    repetition: *const c_char,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    assert!(!repetition.is_null());
    let context = unsafe { &mut *context };
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let repetition = unsafe { CStr::from_ptr(repetition) };
    Box::into_raw(Box::new(PaintStyle(
        Repetition::try_from(repetition.to_string_lossy().as_ref()).map_or(None, |repetition| {
            from_image_slice(data, width, height).map(|image| {
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context.get_context().create_pattern(image, repetition),
                )
            })
        }),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern_asset(
    context: *mut CanvasRenderingContext2D,
    asset: *mut ImageAsset,
    repetition: *const c_char,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    assert!(!asset.is_null());
    assert!(!repetition.is_null());
    let context = unsafe { &*context };
    let asset = unsafe { &*asset };
    let repetition = unsafe { CStr::from_ptr(repetition) };
    let repetition = repetition.to_string_lossy();
    let has_alpha = asset.get_channels() == 4;
    if let Some(bytes) = asset.get_bytes() {
        return if has_alpha {
            Box::into_raw(Box::new(PaintStyle(
                Repetition::try_from(repetition.as_ref()).map_or(None, |repetition| {
                    from_image_slice(bytes, asset.width() as i32, asset.height() as i32).map(
                        |image| {
                            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                                context.get_context().create_pattern(image, repetition),
                            )
                        },
                    )
                }),
            )))
        } else {
            Box::into_raw(Box::new(PaintStyle(
                Repetition::try_from(repetition.as_ref()).map_or(None, |repetition| {
                    from_bitmap_slice(bytes, asset.width() as i32, asset.height() as i32).map(
                        |image| {
                            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                                context.get_context().create_pattern(image, repetition),
                            )
                        },
                    )
                }),
            )))
        };
    }

    Box::into_raw(Box::new(PaintStyle::new(None)))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern_encoded(
    context: *mut CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    repetition: *const c_char,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    assert!(!repetition.is_null());
    let context = unsafe { &*context };
    let repetition = unsafe { CStr::from_ptr(repetition) };
    let repetition = repetition.to_string_lossy();
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    Box::into_raw(Box::new(PaintStyle(
        Repetition::try_from(repetition.as_ref()).map_or(None, |repetition| {
            from_image_slice_encoded(data).map(|image| {
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context.get_context().create_pattern(image, repetition),
                )
            })
        }),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern_canvas2d(
    source: *mut CanvasRenderingContext2D,
    context: *mut CanvasRenderingContext2D,
    repetition: *const c_char,
) -> *mut PaintStyle {
    assert!(!source.is_null());
    assert!(!context.is_null());
    assert!(!repetition.is_null());
    let source = unsafe { &*source };
    let context = unsafe { &*context };
    let repetition = unsafe { CStr::from_ptr(repetition) };
    let repetition = repetition.to_string_lossy();

    Box::into_raw(Box::new(PaintStyle(
        Repetition::try_from(repetition.as_ref()).map_or(None, |repetition| {
            context.remove_if_current();

            let width;
            let height;
            let source_non_gpu;
            let non_gpu;
            {
                let ctx = source.get_context();
                let device = ctx.device();
                width = device.width as i32;
                height = device.height as i32;
                source_non_gpu = device.non_gpu;
            }

            let mut source_ctx = source.get_context_mut();
            let buf;
            if !source_non_gpu {
                source.make_current();



                #[cfg(any(target_os = "ios", target_os = "macos"))] {
                    let snapshot = source_ctx.snapshot();
                    let info = snapshot.image_info();

                    if let Some((image, origin)) = snapshot.backend_texture(false) {
                        let mut ctx = context.get_context_mut();

                        if let Some(image) = from_backend_texture(&mut ctx, &image, origin, info) {
                            return Some(canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                                context.get_context().create_pattern(image, repetition),
                            ))
                        }
                    }

                    return None;
                }


                // todo use gpu image created from snapshot ... need single or shared context or transfer to a texture
                //let data = source_ctx.snapshot_to_raster_data();

                //   let mut data = source_ctx.snapshot_to_raster_data();
                let image = source_ctx.image_snapshot_to_non_texture_image();
                context.make_current();

                return image.map(|image| {
                    canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                        context.get_context().create_pattern(image, repetition),
                    )
                });

                // return from_image_slice(data.as_slice(), width, height).map(|image| {
                //     canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                //         context.get_context().create_pattern(image, repetition),
                //     )
                // });
            } else {
                source.make_current();
                buf = source_ctx.read_pixels();
            }

            {
                let ctx = context.get_context();
                let device = ctx.device();
                non_gpu = device.non_gpu;
            }

            if !non_gpu {
                context.make_current();
            }

            let ret = from_image_slice(buf.as_slice(), width, height).map(|image| {
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context.get_context().create_pattern(image, repetition),
                )
            });
            ret
        }),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_radial_gradient(
    context: *mut CanvasRenderingContext2D,
    x0: f32,
    y0: f32,
    r0: f32,
    x1: f32,
    y1: f32,
    r1: f32,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    Box::into_raw(Box::new(PaintStyle(Some(
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(
            context
                .get_context()
                .create_radial_gradient(x0, y0, r0, x1, y1, r1),
        ),
    ))))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_paint(
    context: *mut CanvasRenderingContext2D,
    color: *const c_char,
) {
    assert!(!color.is_null());
    let color = unsafe { CStr::from_ptr(color) };
    let color = color.to_string_lossy();
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().draw_paint(color.as_ref());
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_point(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().draw_point(x, y);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_points(
    context: *mut CanvasRenderingContext2D,
    mode: i32,
    points: *const f32,
    size: usize,
) {
    assert!(!context.is_null());
    let points = unsafe { std::slice::from_raw_parts(points, size) };
    let context = unsafe { &mut *context };
    context.make_current();
    context
        .get_context_mut()
        .draw_points(mode.try_into().unwrap(), points);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy(
    context: *mut CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
) {
    let context = unsafe { &mut *context };
    canvas_native_context_draw_image(
        context, data, size, width, height, 0.0, 0.0, width, height, dx, dy, width, height,
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_dw_dh(
    context: *mut CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    let context = unsafe { &mut *context };
    canvas_native_context_draw_image(
        context, data, size, width, height, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image(
    context: *mut CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    width: f32,
    height: f32,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    if let Some(image) = from_image_slice(data, width as i32, height as i32) {
        let context = unsafe { &mut *context };
        context.make_current();
        context.get_context_mut().draw_image_src_xywh_dst_xywh(
            &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
        );
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_encoded_dx_dy(
    context: *mut CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    dx: f32,
    dy: f32,
) {
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    if let Some(image) = from_image_slice_encoded(data) {
        let context = unsafe { &mut *context };
        context.make_current();
        let width = image.width() as f32;
        let height = image.height() as f32;
        context
            .get_context_mut()
            .draw_image_src_xywh_dst_xywh(&image, 0.0, 0.0, width, height, dx, dy, width, height);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_encoded_dx_dy_dw_dh(
    context: *mut CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    if let Some(image) = from_image_slice_encoded(data) {
        let context = unsafe { &mut *context };
        context.make_current();
        let width = image.width() as f32;
        let height = image.height() as f32;
        context.get_context_mut().draw_image_src_xywh_dst_xywh(
            &image, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
        );
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_encoded(
    context: *mut CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    if let Some(image) = from_image_slice_encoded(data) {
        let context = unsafe { &mut *context };
        context.make_current();
        context.get_context_mut().draw_image_src_xywh_dst_xywh(
            &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
        );
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_asset(
    context: *mut CanvasRenderingContext2D,
    asset: *mut ImageAsset,
    dx: f32,
    dy: f32,
) {
    let width: f32;
    let height: f32;
    {
        let asset = unsafe { &*asset };
        width = asset.width() as f32;
        height = asset.height() as f32;
    }

    canvas_native_context_draw_image_asset(
        context, asset, 0.0, 0.0, width, height, dx, dy, width, height,
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_dw_dh_asset(
    context: *mut CanvasRenderingContext2D,
    asset: *mut ImageAsset,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    assert!(!asset.is_null());
    let width: f32;
    let height: f32;
    {
        let asset = unsafe { &*asset };
        width = asset.width() as f32;
        height = asset.height() as f32;
    }
    canvas_native_context_draw_image_asset(
        context, asset, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_asset(
    context: *mut CanvasRenderingContext2D,
    asset: *mut ImageAsset,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    assert!(!context.is_null());
    assert!(!asset.is_null());
    let context = unsafe { &mut *context };
    let asset = unsafe { &*asset };

    let width = asset.width();
    let height = asset.height();
    let has_alpha = asset.get_channels() == 4;
    if let Some(bytes) = asset.get_bytes() {
        if has_alpha {
            if let Some(image) = from_image_slice_no_copy(bytes, width as c_int, height as c_int) {
                context.make_current();
                context.get_context_mut().draw_image_src_xywh_dst_xywh(
                    &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
                );
            }
        } else {
            if let Some(image) = from_bitmap_slice(bytes, width as c_int, height as c_int) {
                context.make_current();
                context.get_context_mut().draw_image_src_xywh_dst_xywh(
                    &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_context(
    context: *mut CanvasRenderingContext2D,
    source: *mut CanvasRenderingContext2D,
    dx: f32,
    dy: f32,
) {
    assert!(!source.is_null());
    let device;
    {
        let source = unsafe { &*source };
        let source_ctx = source.get_context();
        device = *source_ctx.device();
    }

    let width = device.width;
    let height = device.height;

    canvas_native_context_draw_image_context(
        context, source, 0.0, 0.0, width, height, dx, dy, width, height,
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_dw_dh_context(
    context: *mut CanvasRenderingContext2D,
    source: *mut CanvasRenderingContext2D,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    assert!(!context.is_null());
    assert!(!source.is_null());

    let device;
    {
        let source = unsafe { &*source };
        let source_ctx = source.get_context();
        device = *source_ctx.device();
    }

    let width = device.width;
    let height = device.height;
    canvas_native_context_draw_image_context(
        context, source, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_context(
    context: *mut CanvasRenderingContext2D,
    source: *mut CanvasRenderingContext2D,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    assert!(!context.is_null());
    assert!(!source.is_null());
    let context = unsafe { &mut *context };
    let source = unsafe { &*source };
    source.make_current();
    // gl context is shared so snapshots should work
    let mut source_ctx = source.get_context_mut();


    #[cfg(any(target_os = "ios", target_os = "macos"))] {
        let snapshot = source_ctx.snapshot();
        let info = snapshot.image_info();

        if let Some((image, origin)) = snapshot.backend_texture(false) {
            let mut ctx = context.get_context_mut();

            if let Some(image) = from_backend_texture(&mut ctx, &image, origin, info) {
                ctx.draw_image_src_xywh_dst_xywh(
                    &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
                );
            }
        }
    }

    #[cfg(target_os = "ios")] {
        let width = source_ctx.device().width;
        let height = source_ctx.device().height;

        let bytes = source_ctx.snapshot_to_raster_data();

        if let Some(image) = from_image_slice_no_copy(bytes.as_slice(), width as c_int, height as c_int)
        {
            let context = unsafe { &mut *context };
            context.make_current();
            context.get_context_mut().draw_image_src_xywh_dst_xywh(
                &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
            );
        }

    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_ellipse(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    radius_x: f32,
    radius_y: f32,
    rotation: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: bool,
) {
    let context = unsafe { &mut *context };
    context.get_context_mut().ellipse(
        x,
        y,
        radius_x,
        radius_y,
        rotation,
        start_angle,
        end_angle,
        anticlockwise,
    )
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_context_fill(
    context: *mut CanvasRenderingContext2D,
    rule: *const c_char,
) {
    if rule.is_null() {
        return;
    }
    let rule = unsafe { CStr::from_ptr(rule) };
    let rule = rule.to_string_lossy();
    if let Ok(rule) = FillRule::try_from(rule.as_ref()) {
        let context = unsafe { &mut *context };
        context.make_current();
        context.get_context_mut().fill_rule(None, rule);
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_context_fill_with_path(
    context: *mut CanvasRenderingContext2D,
    path: &mut Path,
    rule: *const c_char,
) {
    if rule.is_null() {
        return;
    }
    let rule = unsafe { CStr::from_ptr(rule) };
    let rule = rule.to_string_lossy();
    if let Ok(rule) = FillRule::try_from(rule.as_ref()) {
        let context = unsafe { &mut *context };
        context.make_current();
        context
            .get_context_mut()
            .fill_rule(Some(path.inner_mut()), rule);
    }
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_context_fill_rect(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    let context = unsafe { &mut *context };
    context.make_current();
    context
        .get_context_mut()
        .fill_rect_xywh(x, y, width, height);
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_context_fill_text(
    context: *mut CanvasRenderingContext2D,
    text: *const c_char,
    x: f32,
    y: f32,
) {
    if text.is_null() {
        return;
    }
    let text = unsafe { CStr::from_ptr(text) };
    let text = text.to_string_lossy();
    let context = unsafe { &mut *context };
    context.make_current();
    context
        .get_context_mut()
        .fill_text(text.as_ref(), x, y, None);
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_context_fill_text_width(
    context: *mut CanvasRenderingContext2D,
    text: *const c_char,
    x: f32,
    y: f32,
    width: f32,
) {
    if text.is_null() {
        return;
    }
    let text = unsafe { CStr::from_ptr(text) };
    let text = text.to_string_lossy();
    let context = unsafe { &mut *context };
    context.make_current();
    context
        .get_context_mut()
        .fill_text(text.as_ref(), x, y, Some(width));
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_image_data(
    context: *mut CanvasRenderingContext2D,
    sx: f32,
    sy: f32,
    sw: f32,
    sh: f32,
) -> *mut ImageData {
    let context = unsafe { &mut *context };
    let context = unsafe { &mut *context };
    context.make_current();
    Box::into_raw(Box::new(ImageData(
        context.get_context_mut().get_image_data(sx, sy, sw, sh),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_transform(
    context: *mut CanvasRenderingContext2D,
) -> *mut Matrix {
    let context = unsafe { &mut *context };
    context.make_current();
    Box::into_raw(Box::new(Matrix(canvas_2d::context::matrix::Matrix::from(
        context.get_context_mut().get_transform(),
    ))))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_path(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    rule: *const c_char,
) -> bool {
    if rule.is_null() {
        return false;
    }
    let rule = unsafe { CStr::from_ptr(rule) };
    let rule = rule.to_string_lossy();

    FillRule::try_from(rule.as_ref()).map_or(false, |rule| {
        let context = unsafe { &mut *context };
        context.make_current();
        let ret = context.get_context_mut().is_point_in_path(None, x, y, rule);

        ret
    })
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_path_with_path(
    context: *mut CanvasRenderingContext2D,
    path: &mut Path,
    x: f32,
    y: f32,
    rule: *const c_char,
) -> bool {
    if rule.is_null() {
        return false;
    }
    let rule = unsafe { CStr::from_ptr(rule) };
    let rule = rule.to_string_lossy();

    FillRule::try_from(rule.as_ref()).map_or(false, |rule| {
        let context = unsafe { &mut *context };
        context.make_current();
        let ret = context
            .get_context_mut()
            .is_point_in_path(Some(path.inner()), x, y, rule);

        ret
    })
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_stroke(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) -> bool {
    let context = unsafe { &mut *context };

    context.make_current();
    let ret = context.get_context_mut().is_point_in_stroke(None, x, y);

    ret
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_stroke_with_path(
    context: *mut CanvasRenderingContext2D,
    path: &mut Path,
    x: f32,
    y: f32,
) -> bool {
    let context = unsafe { &mut *context };
    context.make_current();
    let ret = context
        .get_context_mut()
        .is_point_in_stroke(Some(path.inner()), x, y);

    ret
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_context_line_to(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context.get_context_mut().line_to(x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_measure_text(
    context: *mut CanvasRenderingContext2D,
    text: *const c_char,
) -> *mut TextMetrics {
    if text.is_null() {
        return std::ptr::null_mut();
    }
    let text = unsafe { CStr::from_ptr(text) };
    let text = text.to_string_lossy();
    let context = unsafe { &mut *context };
    Box::into_raw(Box::new(TextMetrics(
        context.get_context_mut().measure_text(text.as_ref()),
    )))
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_context_move_to(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context.get_context_mut().move_to(x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_put_image_data(
    context: *mut CanvasRenderingContext2D,
    image_data: &mut ImageData,
    dx: f32,
    dy: f32,
    dirty_x: f32,
    dirty_y: f32,
    dirty_width: f32,
    dirty_height: f32,
) {
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().put_image_data(
        image_data.inner(),
        dx,
        dy,
        dirty_x,
        dirty_y,
        dirty_width,
        dirty_height,
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_context_quadratic_curve_to(
    context: *mut CanvasRenderingContext2D,
    cpx: f32,
    cpy: f32,
    x: f32,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context.get_context_mut().quadratic_curve_to(cpx, cpy, x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_rect(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    let context = unsafe { &mut *context };
    context.get_context_mut().rect(x, y, width, height)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_round_rect(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radii: *const f32,
    size: usize,
) {
    let radii = unsafe { std::slice::from_raw_parts(radii, size) };
    let context = unsafe { &mut *context };
    if radii.len() == 8 {
        context
            .get_context_mut()
            .round_rect(x, y, width, height, radii)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_round_rect_tl_tr_br_bl(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
) {
    let context = unsafe { &mut *context };
    let radii = [
        top_left,
        top_left,
        top_right,
        top_right,
        bottom_right,
        bottom_right,
        bottom_left,
        bottom_left,
    ];
    context
        .get_context_mut()
        .round_rect(x, y, width, height, radii.as_ref())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_reset_transform(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().reset_transform();
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_context_restore(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    context.get_context_mut().restore()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_rotate(context: *mut CanvasRenderingContext2D, angle: f32) {
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().rotate(angle);
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_context_save(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    context.get_context_mut().save()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_scale(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().scale(x, y);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_transform(
    context: *mut CanvasRenderingContext2D,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
) {
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().set_transform(a, b, c, d, e, f);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_transform_matrix(
    context: *mut CanvasRenderingContext2D,
    matrix: *mut Matrix,
) {
    let context = unsafe { &mut *context };
    context.make_current();
    let matrix = unsafe { &mut *matrix };
    let matrix = matrix.inner_mut().to_m33();
    context.get_context_mut().set_transform_matrix(&matrix);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_stroke(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().stroke(None);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_stroke_with_path(
    context: *mut CanvasRenderingContext2D,
    path: &mut Path,
) {
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().stroke(Some(path.inner_mut()));
}

#[no_mangle]
pub extern "C" fn canvas_native_context_stroke_rect(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    let context = unsafe { &mut *context };
    context.make_current();
    context
        .get_context_mut()
        .stroke_rect_xywh(x, y, width, height);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_stroke_text(
    context: *mut CanvasRenderingContext2D,
    text: *const c_char,
    x: f32,
    y: f32,
) {
    if text.is_null() {
        return;
    }
    let text = unsafe { CStr::from_ptr(text) };
    let text = text.to_string_lossy();
    let context = unsafe { &mut *context };
    context.make_current();
    context
        .get_context_mut()
        .stroke_text(text.as_ref(), x, y, None);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_stroke_text_width(
    context: *mut CanvasRenderingContext2D,
    text: *const c_char,
    x: f32,
    y: f32,
    width: f32,
) {
    if text.is_null() {
        return;
    }
    let text = unsafe { CStr::from_ptr(text) };
    let text = text.to_string_lossy();
    let context = unsafe { &mut *context };
    context.make_current();
    context
        .get_context_mut()
        .stroke_text(text.as_ref(), x, y, Some(width));
}

#[no_mangle]
pub extern "C" fn canvas_native_context_transform(
    context: *mut CanvasRenderingContext2D,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
) {
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().transform(a, b, c, d, e, f);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_translate(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().translate(x, y);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_flush(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    context.make_current();
    context.get_context_mut().flush();
}

#[no_mangle]
pub extern "C" fn canvas_native_context_render(context: *const CanvasRenderingContext2D) {
    let context = unsafe { &*context };
    context.render();
}

#[no_mangle]
pub extern "C" fn canvas_native_to_data_url(
    context: *mut CanvasRenderingContext2D,
    format: *const c_char,
    quality: u32,
) -> *const c_char {
    if format.is_null() {
        return std::ptr::null_mut();
    }
    let format = unsafe { CStr::from_ptr(format) };
    let format = format.to_string_lossy();
    let context = unsafe { &mut *context };
    context.make_current();
    CString::new(to_data_url(context, format.as_ref(), quality))
        .unwrap()
        .into_raw()
}

/* CanvasRenderingContext2D */

/* ImageBitmap */

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_asset(
    asset: *mut ImageAsset,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> *mut ImageAsset {
    if asset.is_null() {
        return std::ptr::null_mut();
    }
    let asset = unsafe { &mut *asset };

    Box::into_raw(Box::new(ImageAsset(
        canvas_2d::image_bitmap::create_from_image_asset_src_rect(
            &mut asset.0,
            None,
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_asset_src_rect(
    asset: *mut ImageAsset,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> *mut ImageAsset {
    if asset.is_null() {
        return std::ptr::null_mut();
    }
    let asset = unsafe { &mut *asset };
    Box::into_raw(Box::new(ImageAsset(
        canvas_2d::image_bitmap::create_from_image_asset_src_rect(
            &mut asset.0,
            Some((sx, sy, s_width, s_height).into()),
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_encoded_bytes(
    bytes: *const u8,
    size: usize,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> *mut ImageAsset {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, size) };
    Box::into_raw(Box::new(ImageAsset(
        canvas_2d::image_bitmap::create_image_asset_encoded(
            bytes,
            None,
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
    bytes: *const u8,
    size: usize,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
    output: *mut ImageAsset,
) -> bool {
    assert!(!output.is_null());
    let bytes = unsafe { std::slice::from_raw_parts(bytes, size) };
    let output = unsafe { &mut *output };
    canvas_2d::image_bitmap::create_image_asset_with_output(
        bytes,
        None,
        flip_y,
        premultiply_alpha.into(),
        color_space_conversion.into(),
        resize_quality.into(),
        resize_width,
        resize_height,
        &mut output.0,
    );

    output.is_valid()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_encoded_bytes_src_rect(
    bytes: *const u8,
    size: usize,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
) -> *mut ImageAsset {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, size) };
    Box::into_raw(Box::new(ImageAsset(
        canvas_2d::image_bitmap::create_image_asset_encoded(
            bytes,
            Some((sx, sy, s_width, s_height).into()),
            flip_y,
            premultiply_alpha.into(),
            color_space_conversion.into(),
            resize_quality.into(),
            resize_width,
            resize_height,
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
    bytes: *const u8,
    size: usize,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: f32,
    resize_height: f32,
    output: *mut ImageAsset,
) -> bool {
    assert!(!output.is_null());
    let bytes = unsafe { std::slice::from_raw_parts(bytes, size) };
    let output = unsafe { &mut *output };
    canvas_2d::image_bitmap::create_image_asset_with_output(
        bytes,
        Some((sx, sy, s_width, s_height).into()),
        flip_y,
        premultiply_alpha.into(),
        color_space_conversion.into(),
        resize_quality.into(),
        resize_width,
        resize_height,
        &mut output.0,
    );

    output.is_valid()
}

/* ImageBitmap */

/* Path2D */
#[derive(Clone)]
pub struct Path(canvas_2d::context::paths::path::Path);

impl Default for Path {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Path {
    pub(crate) fn inner(&self) -> &canvas_2d::context::paths::path::Path {
        &self.0
    }

    pub(crate) fn inner_mut(&mut self) -> &mut canvas_2d::context::paths::path::Path {
        &mut self.0
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_path_add_path(path: *mut Path, path_to_add: *const Path) {
    if path.is_null() || path_to_add.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    let path_to_add = unsafe { &*path_to_add };
    path.0.add_path(&path_to_add.0, None);
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_create() -> *mut Path {
    Box::into_raw(Box::new(Path::default()))
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_create_with_path(path: *const Path) -> *mut Path {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { &*path };
    Box::into_raw(Box::new(path.clone()))
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_create_with_string(string: *const c_char) -> *mut Path {
    if string.is_null() {
        return std::ptr::null_mut();
    }
    let string = unsafe { CStr::from_ptr(string) };
    let string = string.to_string_lossy();
    Box::into_raw(Box::new(Path(
        canvas_2d::context::paths::path::Path::from_str(string.as_ref()),
    )))
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_close_path(path: *mut Path) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.close_path()
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_move_to(path: *mut Path, x: f32, y: f32) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.move_to(x, y)
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_line_to(path: *mut Path, x: f32, y: f32) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.line_to(x, y)
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_bezier_curve_to(
    path: *mut Path,
    cp1x: f32,
    cp1y: f32,
    cp2x: f32,
    cp2y: f32,
    x: f32,
    y: f32,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_quadratic_curve_to(
    path: *mut Path,
    cpx: f32,
    cpy: f32,
    x: f32,
    y: f32,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.quadratic_curve_to(cpx, cpy, x, y)
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_arc(
    path: *mut Path,
    x: f32,
    y: f32,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    anti_clockwise: bool,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0
        .arc(x, y, radius, start_angle, end_angle, anti_clockwise)
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_arc_to(
    path: *mut Path,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    radius: f32,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.arc_to(x1, y1, x2, y2, radius)
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_ellipse(
    path: *mut Path,
    x: f32,
    y: f32,
    radius_x: f32,
    radius_y: f32,
    rotation: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: bool,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.ellipse(
        x,
        y,
        radius_x,
        radius_y,
        rotation,
        start_angle,
        end_angle,
        anticlockwise,
    )
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_rect(
    path: *mut Path,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    if path.is_null() {
        return;
    }
    let path = unsafe { &mut *path };
    path.0.rect(x, y, width, height)
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_round_rect(
    path: *mut Path,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radii: *const f32,
    size: usize,
) {
    assert!(!path.is_null());
    let radii = unsafe { std::slice::from_raw_parts(radii, size) };
    let path = unsafe { &mut *path };

    let size = radii.len();
    if size == 0 {
        return;
    }
    /*
    [all-corners]
    [top-left-and-bottom-right, top-right-and-bottom-left]
    [top-left, top-right-and-bottom-left, bottom-right]
    [top-left, top-right, bottom-right, bottom-left]
     */
    let mut top_left = 0.;
    let mut top_right = 0.;
    let mut bottom_right = 0.;
    let mut bottom_left = 0.;

    match size {
        1 => {
            top_left = radii[0];
            top_right = top_left;
            bottom_right = top_left;
            bottom_left = top_left;
        }
        2 => {
            top_left = radii[0];
            top_right = radii[1];
            bottom_right = top_left;
            bottom_left = top_right;
        }

        3 => {
            top_left = radii[0];
            top_right = radii[1];
            bottom_right = radii[2];
            bottom_left = top_right
        }
        4 => {
            top_left = radii[0];
            top_right = radii[1];
            bottom_right = radii[2];
            bottom_left = radii[3];
        }
        _ => {}
    }

    if size > 0 && size <= 4 {
        path.0.round_rect(
            x,
            y,
            width,
            height,
            &[
                top_left,
                top_left,
                top_right,
                top_right,
                bottom_right,
                bottom_right,
                bottom_left,
                bottom_left,
            ],
        )
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_round_rect_tl_tr_br_bl(
    path: *mut Path,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
) {
    assert!(!path.is_null());
    let path = unsafe { &mut *path };

    path.0.round_rect(
        x,
        y,
        width,
        height,
        &[
            top_left,
            top_left,
            top_right,
            top_right,
            bottom_right,
            bottom_right,
            bottom_left,
            bottom_left,
        ],
    )
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_path_to_string(path: *const Path) -> *const c_char {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path = unsafe { &*path };
    let svg = path.0.path().to_svg();
    CString::new(svg).unwrap().into_raw()
}

/* Path 2D */

/* DOMMatrix */
#[derive(Clone)]
pub struct Matrix(canvas_2d::context::matrix::Matrix);

impl Matrix {
    pub fn inner(&self) -> &canvas_2d::context::matrix::Matrix {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut canvas_2d::context::matrix::Matrix {
        &mut self.0
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_create() -> *mut Matrix {
    Box::into_raw(Box::new(Matrix(canvas_2d::context::matrix::Matrix::new())))
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_update(matrix: *mut Matrix, slice: *const f32, size: usize) {
    assert!(!matrix.is_null());
    assert!(!slice.is_null(), "values cannot be null");
    let slice = unsafe { std::slice::from_raw_parts(slice, size) };
    let matrix = unsafe { &mut *matrix };
    let mut affine = [0f32; 6];
    affine.copy_from_slice(slice);
    matrix.inner_mut().set_affine(&affine);
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_update_3d(
    matrix: *mut Matrix,
    slice: *const f32,
    size: usize,
) {
    assert!(!matrix.is_null());
    assert!(!slice.is_null(), "values cannot be null");
    let slice = unsafe { std::slice::from_raw_parts(slice, size) };
    assert_eq!(slice.len(), 16);
    let matrix = unsafe { &mut *matrix };
    let matrix = matrix.inner_mut();
    matrix.set_m11(slice[0]);
    matrix.set_m12(slice[1]);
    matrix.set_m13(slice[2]);
    matrix.set_m14(slice[3]);

    matrix.set_m21(slice[4]);
    matrix.set_m22(slice[5]);
    matrix.set_m23(slice[6]);
    matrix.set_m24(slice[7]);

    matrix.set_m31(slice[8]);
    matrix.set_m32(slice[9]);
    matrix.set_m33(slice[10]);
    matrix.set_m34(slice[11]);

    matrix.set_m41(slice[12]);
    matrix.set_m42(slice[13]);
    matrix.set_m43(slice[14]);
    matrix.set_m44(slice[15]);
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_a(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().a()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_a(matrix: *mut Matrix, a: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_a(a)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_b(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().b()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_b(matrix: *mut Matrix, b: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_b(b)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_c(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().c()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_c(matrix: *mut Matrix, c: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_c(c)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_d(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().d()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_d(matrix: *mut Matrix, d: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_d(d)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_e(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().e()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_e(matrix: *mut Matrix, e: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_e(e)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_f(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().f()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_f(matrix: *mut Matrix, f: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_f(f)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m11(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m11()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m11(matrix: *mut Matrix, m11: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m11(m11)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m12(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m12()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m12(matrix: *mut Matrix, m12: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m12(m12)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m13(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m13()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m13(matrix: *mut Matrix, m13: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m13(m13)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m14(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m14()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m14(matrix: *mut Matrix, m14: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m14(m14)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m21(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m21()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m21(matrix: *mut Matrix, m21: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m21(m21)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m22(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m22()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m22(matrix: *mut Matrix, m22: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m22(m22)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m23(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m23()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m23(matrix: *mut Matrix, m23: f32) {
    let mut matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m23(m23)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m24(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m24()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m24(matrix: *mut Matrix, m24: f32) {
    let matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m24(m24)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m31(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m31()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m31(matrix: *mut Matrix, m31: f32) {
    let matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m31(m31)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m32(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m32()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m32(matrix: *mut Matrix, m32: f32) {
    let matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m32(m32)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m33(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m33()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m33(matrix: *mut Matrix, m33: f32) {
    let matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m33(m33)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m34(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m34()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m34(matrix: *mut Matrix, m34: f32) {
    let matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m34(m34)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m41(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m41()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m41(matrix: *mut Matrix, m41: f32) {
    let matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m41(m41)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m42(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m42()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m42(matrix: *mut Matrix, m42: f32) {
    let matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m42(m42)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m43(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m43()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m43(matrix: *mut Matrix, m43: f32) {
    let matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m43(m43)
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_get_m44(matrix: *const Matrix) -> f32 {
    let matrix = unsafe { &*matrix };
    matrix.inner().m44()
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_set_m44(matrix: *mut Matrix, m44: f32) {
    let matrix = unsafe { &mut *matrix };
    matrix.inner_mut().set_m44(m44)
}

/* DOMMatrix */

/* ImageData */
#[derive(Clone)]
pub struct ImageData(canvas_2d::context::pixel_manipulation::image_data::ImageData);

impl ImageData {
    pub(crate) fn new(data: canvas_2d::context::pixel_manipulation::image_data::ImageData) -> Self {
        ImageData(data)
    }

    pub fn inner(&self) -> &canvas_2d::context::pixel_manipulation::image_data::ImageData {
        &self.0
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_create(width: i32, height: i32) -> Box<ImageData> {
    let data = canvas_2d::context::pixel_manipulation::ImageData::new(width, height);
    Box::new(ImageData::new(data))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_width(image_data: &ImageData) -> i32 {
    image_data.0.width()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_height(image_data: &ImageData) -> i32 {
    image_data.0.height()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_data(image_data: &mut ImageData) -> *mut U8Buffer {
    Box::into_raw(Box::new(U8Buffer::from(image_data.0.bytes_mut())))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_shared_instance(
    image_data: &mut ImageData,
) -> Box<ImageData> {
    Box::new(image_data.clone())
}

/* ImageData */

/* ImageAsset */

#[derive(Clone)]
pub struct ImageAsset(pub(crate) canvas_core::image_asset::ImageAsset);

impl ImageAsset {
    pub fn new(asset: canvas_core::image_asset::ImageAsset) -> Self {
        Self(asset)
    }
}

impl Default for ImageAsset {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl ImageAsset {
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    pub fn width(&self) -> c_uint {
        self.0.width()
    }

    pub fn height(&self) -> c_uint {
        self.0.height()
    }

    pub fn save_path(&mut self, path: &str, format: OutputFormat) -> bool {
        self.0.save_path(path, format)
    }

    // pub fn size(&self) -> usize {
    //     self.0.size()
    // }

    #[cfg(any(unix))]
    pub fn load_from_fd(&mut self, fd: c_int) -> bool {
        self.0.load_from_fd(fd)
    }

    pub fn load_from_reader<R>(&mut self, reader: &mut R) -> bool
        where
            R: Read + std::io::Seek + std::io::BufRead,
    {
        self.0.load_from_reader(reader)
    }

    pub fn load_from_path(&mut self, path: &str) -> bool {
        self.0.load_from_path(path)
    }

    pub fn load_from_bytes(&mut self, buf: &[u8]) -> bool {
        self.0.load_from_bytes(buf)
    }

    pub fn scale(&mut self, x: c_uint, y: c_uint) -> bool {
        self.0.scale(x, y)
    }

    pub fn get_bytes(&self) -> Option<&[u8]> {
        self.0.get_bytes()
    }

    pub fn copy_bytes(&self) -> Option<Vec<u8>> {
        self.0.copy_bytes()
    }

    pub fn get_channels(&self) -> i32 {
        //self.0.get_channels()
        // always rgba
        4
    }

    pub fn error(&self) -> Cow<'_, str> {
        self.0.error()
    }

    pub fn set_error(&mut self, error: &str) {
        self.0.set_error(error);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_create() -> *mut ImageAsset {
    Box::into_raw(Box::new(ImageAsset::default()))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_shared_clone(
    asset: *mut ImageAsset,
) -> *mut ImageAsset {
    assert!(!asset.is_null());
    let asset = unsafe { &*asset };
    Box::into_raw(Box::new(asset.clone()))
}

// fn canvas_native_image_asset_get_size(asset: &ImageAsset) -> usize {
//     asset.size()
// }

#[cfg(any(unix))]
#[no_mangle]
pub extern "C" fn canvas_native_image_asset_load_from_fd(
    asset: *mut ImageAsset,
    fd: c_int,
) -> bool {
    if asset.is_null() {
        return false;
    }
    let asset = unsafe { &mut *asset };
    asset.load_from_fd(fd)
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_load_from_path(
    asset: *mut ImageAsset,
    path: *const c_char,
) -> bool {
    if asset.is_null() {
        return false;
    }
    let asset = unsafe { &mut *asset };
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();
    asset.load_from_path(path.as_ref())
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_load_from_raw(
    asset: *mut ImageAsset,
    array: *const u8,
    size: usize,
) -> bool {
    let array = unsafe { std::slice::from_raw_parts(array, size) };
    let asset = unsafe { &mut *asset };
    asset.load_from_bytes(array)
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_load_from_url(
    asset: *mut ImageAsset,
    url: *const c_char,
) -> bool {
    if asset.is_null() || url.is_null() {
        return false;
    }
    let asset = unsafe { &mut *asset };
    let url = unsafe { CStr::from_ptr(url) };
    let url = url.to_string_lossy().to_string();
    canvas_native_image_asset_load_from_url_internal(&mut asset.0, url.as_str())
}

pub(crate) fn canvas_native_image_asset_load_from_url_internal(
    asset: &mut canvas_core::image_asset::ImageAsset,
    url: &str,
) -> bool {
    use std::io::prelude::*;

    let resp = ureq::AgentBuilder::new()
        .redirects(10)
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .get(url);

    if let Ok(res) = resp.call() {
        if res.status() != 200 {
            return false;
        }
        // assert!(!res.has("Content-Length"));
        let len: usize;

        if let Some(value) = res.header("Content-Length") {
            if let Ok(length) = value.parse::<usize>() {
                len = length;
            } else {
                return false;
            }
        } else {
            return false;
        }

        let mut bytes: Vec<u8> = Vec::with_capacity(len);
        if let Ok(_) = res.into_reader().read_to_end(&mut bytes) {
            //  assert_eq!(bytes.len(), len);
        } else {
            return false;
        }

        return asset.load_from_bytes(bytes.as_slice());
    }
    false
}

// #[no_mangle]
//pub extern "C" fn canvas_native_image_asset_get_bytes(asset: *mut ImageAsset) -> Option<*const U8Buffer> {
//     asset.get_bytes()
// }

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_addr(asset: *mut ImageAsset) -> i64 {
    asset as i64
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_width(asset: *mut ImageAsset) -> u32 {
    if asset.is_null() {
        return 0;
    }
    let asset = unsafe { &*asset };
    asset.width()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_height(asset: *mut ImageAsset) -> u32 {
    if asset.is_null() {
        return 0;
    }
    let asset = unsafe { &*asset };
    asset.height()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_get_error(asset: *mut ImageAsset) -> *const c_char {
    if asset.is_null() {
        return std::ptr::null_mut();
    }
    let asset = unsafe { &*asset };
    CString::new(asset.error().to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_has_error(asset: *mut ImageAsset) -> bool {
    if asset.is_null() {
        return false;
    }
    let asset = unsafe { &*asset };
    if asset.error().is_empty() {
        return false;
    }
    true
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_scale(asset: *mut ImageAsset, x: u32, y: u32) -> bool {
    if asset.is_null() {
        return false;
    }
    let asset = unsafe { &mut *asset };
    asset.scale(x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_save_path(
    asset: *mut ImageAsset,
    path: *const c_char,
    format: u32,
) -> bool {
    if asset.is_null() || path.is_null() {
        return false;
    }
    let asset = unsafe { &mut *asset };
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();
    asset.save_path(path.as_ref(), OutputFormat::from(format))
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ThreadCallbackData {
    id: i64,
    done: bool,
}

impl ThreadCallbackData {
    #[allow(dead_code)]
    pub fn new(id: i64, done: bool) -> Self {
        Self { id, done }
    }
}

impl Default for ThreadCallbackData {
    fn default() -> Self {
        Self { id: 0, done: false }
    }
}

/* ImageAsset */

/* TextMetrics */

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_width(metrics: *const TextMetrics) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.width()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_left(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.actual_bounding_box_left()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_right(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.actual_bounding_box_right()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_ascent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.actual_bounding_box_ascent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_actual_bounding_box_descent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.actual_bounding_box_descent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_font_bounding_box_ascent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.font_bounding_box_ascent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_font_bounding_box_descent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.font_bounding_box_descent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_em_height_ascent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.em_height_ascent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_em_height_descent(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.em_height_descent()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_hanging_baseline(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.hanging_baseline()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_alphabetic_baseline(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.alphabetic_baseline()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_metrics_get_ideographic_baseline(
    metrics: *const TextMetrics,
) -> f32 {
    if metrics.is_null() {
        return 0.;
    }
    let metrics = unsafe { &*metrics };
    metrics.0.ideographic_baseline()
}

/* TextMetrics */

/* PaintStyle */
#[no_mangle]
pub extern "C" fn canvas_native_paint_style_from_bytes(
    context: *const CanvasRenderingContext2D,
    repetition: i32,
    width: i32,
    height: i32,
    bytes: *const u8,
    size: usize,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let bytes = unsafe { std::slice::from_raw_parts(bytes, size) };
    Box::into_raw(Box::new(PaintStyle(
        from_image_slice(bytes, width, height).map(|image| {
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                context.get_context().create_pattern(
                    image,
                    canvas_2d::context::fill_and_stroke_styles::pattern::Repetition::from(
                        repetition,
                    ),
                ),
            )
        }),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_pattern_from_ptr(ptr: i64) -> *mut PaintStyle {
    Box::into_raw(Box::new(PaintStyle::from_ptr(ptr)))
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_empty() -> *mut PaintStyle {
    Box::into_raw(Box::new(PaintStyle(None)))
}
/* PaintStyle */
/* CanvasGradient */
#[no_mangle]
pub extern "C" fn canvas_native_gradient_add_color_stop(
    style: *mut PaintStyle,
    stop: f32,
    color: *const c_char,
) {
    if style.is_null() || color.is_null() {
        return;
    }
    let style = unsafe { &mut *style };
    let color = unsafe { CStr::from_ptr(color) };
    let color = color.to_string_lossy();
    if let Some(style) = style.0.as_mut() {
        match style {
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(gradient) => {
                gradient.add_color_stop_str(stop, color.as_ref())
            }
            _ => {}
        }
    }
}

/* CanvasGradient */

/* CanvasPattern */
#[no_mangle]
pub extern "C" fn canvas_native_pattern_set_transform(
    pattern: *mut PaintStyle,
    matrix: *const Matrix,
) {
    if pattern.is_null() || matrix.is_null() {
        return;
    }
    let pattern = unsafe { &mut *pattern };
    let matrix = unsafe { &*matrix };
    if let Some(pattern) = pattern.0.as_mut() {
        match pattern {
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(pattern) => {
                pattern.set_pattern_transform(matrix.inner())
            }
            _ => {}
        }
    }
}

/* CanvasPattern */

/* TextDecoder */
#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_create(decoding: *const c_char) -> *mut TextDecoder {
    assert!(!decoding.is_null());
    let decoding = unsafe { CStr::from_ptr(decoding) };
    let decoding = decoding.to_string_lossy();
    Box::into_raw(Box::new(TextDecoder(
        canvas_2d::context::text_decoder::TextDecoder::new(Some(decoding.as_ref())),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode(
    decoder: *const TextDecoder,
    data: *const u8,
    size: usize,
) -> *const c_char {
    assert!(!decoder.is_null());
    assert!(!data.is_null());
    let decoder = unsafe { &*decoder };

    decoder.0.decode(data, size).into_raw()
}

#[derive(Clone)]
pub struct CCow(Cow<'static, str>);

#[no_mangle]
pub extern "C" fn canvas_native_ccow_destroy(cow: *mut CCow) {
    if cow.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(cow) };
}

#[no_mangle]
pub extern "C" fn canvas_native_ccow_get_bytes(cow: *const CCow) -> *const u8 {
    assert!(!cow.is_null());
    let cow = unsafe { &*cow };
    cow.0.as_ptr()
}

#[no_mangle]
pub extern "C" fn canvas_native_ccow_get_length(cow: *const CCow) -> usize {
    assert!(!cow.is_null());
    let cow = unsafe { &*cow };
    cow.0.len()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_as_cow(
    decoder: *const TextDecoder,
    data: *const u8,
    size: usize,
) -> *mut CCow {
    assert!(!decoder.is_null());
    assert!(!data.is_null());
    let decoder = unsafe { &*decoder };

    let bytes = decoder.0.decode_as_cow(data, size);

    Box::into_raw(Box::new(CCow(bytes)))
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_as_bytes(
    decoder: *const TextDecoder,
    data: *const u8,
    size: usize,
) -> *mut U8Buffer {
    assert!(!decoder.is_null());
    assert!(!data.is_null());
    let decoder = unsafe { &*decoder };

    let bytes = decoder.0.decode_as_bytes(data, size);

    Box::into_raw(Box::new(U8Buffer::new_with_vec(bytes)))
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_c_string(
    decoder: *const TextDecoder,
    data: *const c_char,
) -> *const c_char {
    assert!(!decoder.is_null());
    assert!(!data.is_null());
    let decoder = unsafe { &*decoder };

    decoder.0.decode_c_string(data).into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_get_encoding(
    decoder: *const TextDecoder,
) -> *const c_char {
    assert!(!decoder.is_null());
    let decoder = unsafe { &*decoder };
    CString::new(decoder.0.encoding().to_string().to_lowercase())
        .unwrap()
        .into_raw()
}
/* TextDecoder */

/* TextEncoder */
#[no_mangle]
pub extern "C" fn canvas_native_text_encoder_create(encoding: *const c_char) -> *mut TextEncoder {
    assert!(!encoding.is_null());
    let encoding = unsafe { CStr::from_ptr(encoding) };
    let encoding = encoding.to_string_lossy();

    Box::into_raw(Box::new(TextEncoder(
        canvas_2d::context::text_encoder::TextEncoder::new(Some(encoding.as_ref())),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_text_encoder_encode(
    encoder: *const TextEncoder,
    text: *const c_char,
) -> *mut U8Buffer {
    assert!(!encoder.is_null());
    assert!(!text.is_null());
    let encoder = unsafe { &*encoder };
    let text = unsafe { CStr::from_ptr(text) };
    let text = text.to_string_lossy();
    let encoded = encoder.0.encode(text.as_ref());
    Box::into_raw(Box::new(U8Buffer::from(encoded)))
}

#[no_mangle]
pub extern "C" fn canvas_native_text_encoder_get_encoding(
    encoder: *const TextEncoder,
) -> *const c_char {
    assert!(!encoder.is_null());
    let encoder = unsafe { &*encoder };
    CString::new(encoder.0.encoding().to_string().to_lowercase())
        .unwrap()
        .into_raw()
}

/* TextEncoder */

/* GL */
#[no_mangle]
pub extern "C" fn canvas_native_context_gl_make_current(
    context: *const CanvasRenderingContext2D,
) -> bool {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.make_current()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_gl_swap_buffers(
    context: *const CanvasRenderingContext2D,
) -> bool {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.swap_buffers()
}
/* GL */

pub struct GLContext(canvas_core::gl::GLContext);

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub enum WebGLExtensionType {
    WebGLExtensionTypeEXT_blend_minmax,
    WebGLExtensionTypeEXT_color_buffer_half_float,
    WebGLExtensionTypeEXT_disjoint_timer_query,
    WebGLExtensionTypeEXT_sRGB,
    WebGLExtensionTypeEXT_shader_texture_lod,
    WebGLExtensionTypeEXT_texture_filter_anisotropic,
    WebGLExtensionTypeOES_element_index_uint,
    WebGLExtensionTypeOES_standard_derivatives,
    WebGLExtensionTypeOES_texture_float,
    WebGLExtensionTypeOES_texture_float_linear,
    WebGLExtensionTypeOES_texture_half_float,
    WebGLExtensionTypeOES_texture_half_float_linear,
    WebGLExtensionTypeOES_vertex_array_object,
    WebGLExtensionTypeWEBGL_color_buffer_float,
    WebGLExtensionTypeWEBGL_compressed_texture_atc,
    WebGLExtensionTypeWEBGL_compressed_texture_etc1,
    WebGLExtensionTypeWEBGL_compressed_texture_s3tc,
    WebGLExtensionTypeWEBGL_compressed_texture_s3tc_srgb,
    WebGLExtensionTypeWEBGL_compressed_texture_etc,
    WebGLExtensionTypeWEBGL_compressed_texture_pvrtc,
    WebGLExtensionTypeWEBGL_lose_context,
    WebGLExtensionTypeANGLE_instanced_arrays,
    WebGLExtensionTypeWEBGL_depth_texture,
    WebGLExtensionTypeWEBGL_draw_buffers,
    WebGLExtensionTypeOES_fbo_render_mipmap,
    WebGLExtensionTypeNone,
}

impl Into<WebGLExtensionType> for canvas_webgl::prelude::WebGLExtensionType {
    fn into(self) -> WebGLExtensionType {
        match self {
            canvas_webgl::prelude::WebGLExtensionType::EXT_blend_minmax => {
                WebGLExtensionType::WebGLExtensionTypeEXT_blend_minmax
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_color_buffer_half_float => {
                WebGLExtensionType::WebGLExtensionTypeEXT_color_buffer_half_float
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_disjoint_timer_query => {
                WebGLExtensionType::WebGLExtensionTypeEXT_disjoint_timer_query
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_sRGB => {
                WebGLExtensionType::WebGLExtensionTypeEXT_sRGB
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_shader_texture_lod => {
                WebGLExtensionType::WebGLExtensionTypeEXT_shader_texture_lod
            }
            canvas_webgl::prelude::WebGLExtensionType::EXT_texture_filter_anisotropic => {
                WebGLExtensionType::WebGLExtensionTypeEXT_texture_filter_anisotropic
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_element_index_uint => {
                WebGLExtensionType::WebGLExtensionTypeOES_element_index_uint
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_standard_derivatives => {
                WebGLExtensionType::WebGLExtensionTypeOES_standard_derivatives
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_float => {
                WebGLExtensionType::WebGLExtensionTypeOES_texture_float
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_float_linear => {
                WebGLExtensionType::WebGLExtensionTypeOES_texture_float_linear
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_half_float => {
                WebGLExtensionType::WebGLExtensionTypeOES_texture_half_float
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_texture_half_float_linear => {
                WebGLExtensionType::WebGLExtensionTypeOES_texture_half_float_linear
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_vertex_array_object => {
                WebGLExtensionType::WebGLExtensionTypeOES_vertex_array_object
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_color_buffer_float => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_color_buffer_float
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_atc => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_atc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_etc1 => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_etc1
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_s3tc => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_s3tc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_s3tc_srgb => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_s3tc_srgb
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_etc => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_etc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_compressed_texture_pvrtc => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_compressed_texture_pvrtc
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_lose_context => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_lose_context
            }
            canvas_webgl::prelude::WebGLExtensionType::ANGLE_instanced_arrays => {
                WebGLExtensionType::WebGLExtensionTypeANGLE_instanced_arrays
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_depth_texture => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_depth_texture
            }
            canvas_webgl::prelude::WebGLExtensionType::WEBGL_draw_buffers => {
                WebGLExtensionType::WebGLExtensionTypeWEBGL_draw_buffers
            }
            canvas_webgl::prelude::WebGLExtensionType::None => {
                WebGLExtensionType::WebGLExtensionTypeNone
            }
            canvas_webgl::prelude::WebGLExtensionType::OES_fbo_render_mipmap => {
                WebGLExtensionType::WebGLExtensionTypeOES_fbo_render_mipmap
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern_webgl(
    source: *mut WebGLState,
    context: *mut CanvasRenderingContext2D,
    repetition: *const c_char,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    assert!(!repetition.is_null());
    assert!(!source.is_null());
    let context = unsafe { &mut *context };
    let repetition = unsafe { CStr::from_ptr(repetition) };
    let repetition = repetition.to_string_lossy();
    let source = unsafe { &*source };
    Box::into_raw(Box::new(PaintStyle(
        Repetition::try_from(repetition.as_ref()).map_or(None, |repetition| {
            context.remove_if_current();
            let state = source.get_inner();
            state.make_current();
            let width = state.get_drawing_buffer_width();
            let height = state.get_drawing_buffer_height();

            let mut buf = vec![0u8; (width * height * 4) as usize];

            unsafe {
                gl_bindings::Flush();
                gl_bindings::ReadPixels(
                    0,
                    0,
                    width,
                    height,
                    gl_bindings::RGBA,
                    gl_bindings::UNSIGNED_BYTE,
                    buf.as_mut_ptr() as *mut c_void,
                );
            }

            context.make_current();
            let ret = from_image_slice(buf.as_slice(), width, height).map(|image| {
                canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                    context.get_context().create_pattern(image, repetition),
                )
            });

            ret
        }),
    )))
}

/* GL */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_make_current(state: *mut WebGLState) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    state.get_inner().make_current()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_swap_buffers(state: *mut WebGLState) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    state.get_inner().swap_buffers()
}
/* GL */

/* WebGL */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_resized(_state: *mut WebGLState) {
    //state.get_inner_mut().resized();
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_to_data_url(
    state: *mut WebGLState,
    format: *const c_char,
    quality: u32,
) -> *const c_char {
    assert!(!state.is_null());
    assert!(!format.is_null());
    let format = unsafe { CStr::from_ptr(format) };
    let format = format.to_string_lossy();
    let state = unsafe { &mut *state };

    let info = state.get_inner();
    let width = info.drawing_buffer_width();
    let height = info.drawing_buffer_height();
    info.make_current();
    // gl_bindings::PixelStorei(gl_bindings::UNPACK_ALIGNMENT, 1);
    let mut buffer = vec![0u8; (width * height * 4) as usize];
    unsafe {
        gl_bindings::ReadPixels(
            0,
            0,
            width,
            height,
            gl_bindings::RGBA,
            gl_bindings::UNSIGNED_BYTE,
            buffer.as_mut_ptr() as *mut c_void,
        );
    }

    CString::new(canvas_2d::bytes_to_data_url(
        width,
        height,
        buffer.as_slice(),
        format.as_ref(),
        quality,
    ))
        .unwrap()
        .into_raw()
}

#[derive(Debug)]
pub struct WebGLState(canvas_webgl::prelude::WebGLState);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_destroy(state: *mut WebGLState) {
    if state.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(state) };
}

impl WebGLState {
    pub fn new_with_context(
        context: canvas_core::gl::GLContext,
        version: WebGLVersion,
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: &str,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
        is_canvas: bool,
    ) -> Self {
        Self(
            canvas_webgl::prelude::WebGLState::new_with_context_attributes(
                context,
                version,
                alpha,
                antialias,
                depth,
                fail_if_major_performance_caveat,
                power_preference,
                premultiplied_alpha,
                preserve_drawing_buffer,
                stencil,
                desynchronized,
                xr_compatible,
                is_canvas,
            ),
        )
    }
    pub fn get_inner(&self) -> &canvas_webgl::prelude::WebGLState {
        &self.0
    }

    pub fn get_inner_mut(&mut self) -> &mut canvas_webgl::prelude::WebGLState {
        &mut self.0
    }
}

pub struct WebGLActiveInfo(canvas_webgl::prelude::WebGLActiveInfo);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_info_destroy(info: *mut WebGLActiveInfo) {
    if info.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(info) };
}

impl WebGLActiveInfo {
    pub fn get_name(&self) -> &str {
        self.0.get_name()
    }

    pub fn get_size(&self) -> i32 {
        self.0.get_size()
    }

    pub fn get_type(&self) -> u32 {
        self.0.get_type()
    }

    pub fn get_is_empty(&self) -> bool {
        self.0.get_is_empty()
    }
}

pub struct ContextAttributes(canvas_webgl::prelude::ContextAttributes);

#[no_mangle]
pub extern "C" fn canvas_native_context_attributes_destroy(attr: *mut ContextAttributes) {
    if attr.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(attr) };
}

impl ContextAttributes {
    pub fn get_alpha(&self) -> bool {
        self.0.get_alpha()
    }
    pub fn get_antialias(&self) -> bool {
        self.0.get_antialias()
    }
    pub fn get_depth(&self) -> bool {
        self.0.get_depth()
    }
    pub fn get_fail_if_major_performance_caveat(&self) -> bool {
        self.0.get_fail_if_major_performance_caveat()
    }
    pub fn get_power_preference(&self) -> String {
        self.0.get_power_preference()
    }
    pub fn get_premultiplied_alpha(&self) -> bool {
        self.0.get_premultiplied_alpha()
    }
    pub fn get_preserve_drawing_buffer(&self) -> bool {
        self.0.get_preserve_drawing_buffer()
    }
    pub fn get_stencil(&self) -> bool {
        self.0.get_stencil()
    }
    pub fn get_desynchronized(&self) -> bool {
        self.0.get_desynchronized()
    }
    pub fn get_xr_compatible(&self) -> bool {
        self.0.get_xr_compatible()
    }
}

pub struct WebGLFramebufferAttachmentParameter(
    canvas_webgl::prelude::WebGLFramebufferAttachmentParameter,
);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_attachment_parameter_destroy(
    parameter: *mut WebGLFramebufferAttachmentParameter,
) {
    if parameter.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(parameter) };
}

impl WebGLFramebufferAttachmentParameter {
    pub fn get_is_texture(&self) -> bool {
        self.0.get_is_texture()
    }

    pub fn get_is_renderbuffer(&self) -> bool {
        self.0.get_is_renderbuffer()
    }

    pub fn get_value(&self) -> i32 {
        self.0.get_value()
    }
}

pub struct WebGLShaderPrecisionFormat(canvas_webgl::prelude::WebGLShaderPrecisionFormat);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_shader_precision_format_destroy(
    value: *mut WebGLShaderPrecisionFormat,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

impl WebGLShaderPrecisionFormat {
    pub fn get_precision(&self) -> i32 {
        self.0.get_precision()
    }

    pub fn get_range_min(&self) -> i32 {
        self.0.get_range_min()
    }

    pub fn get_range_max(&self) -> i32 {
        self.0.get_range_max()
    }
}

pub struct WebGLExtension(Option<Box<dyn canvas_webgl::prelude::WebGLExtension>>);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_extension_destroy(value: *mut WebGLExtension) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

impl WebGLExtension {
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn extension_type(&self) -> WebGLExtensionType {
        if self.is_none() {
            return WebGLExtensionType::WebGLExtensionTypeNone;
        }
        self.0.as_ref().unwrap().extension_type().into()
    }

    pub fn into_ext_disjoint_timer_query(self) -> Box<EXT_disjoint_timer_query> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext =
            unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::EXT_disjoint_timer_query) };
        Box::new(EXT_disjoint_timer_query(*ext))
    }

    pub fn into_angle_instanced_arrays(self) -> Box<ANGLE_instanced_arrays> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext =
            unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::ANGLE_instanced_arrays) };
        Box::new(ANGLE_instanced_arrays(*ext))
    }

    pub fn into_lose_context(self) -> Box<WEBGL_lose_context> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext = unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::WEBGL_lose_context) };
        Box::new(WEBGL_lose_context(*ext))
    }

    pub fn into_draw_buffers(self) -> Box<WEBGL_draw_buffers> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext = unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::WEBGL_draw_buffers) };
        Box::new(WEBGL_draw_buffers(*ext))
    }

    pub fn into_oes_vertex_array_object(self) -> Box<OES_vertex_array_object> {
        let ext = Box::into_raw(self.0.unwrap());
        let ext =
            unsafe { Box::from_raw(ext as *mut canvas_webgl::prelude::OES_vertex_array_object) };
        Box::new(OES_vertex_array_object(*ext))
    }
}

#[allow(non_camel_case_types)]
pub struct EXT_blend_minmax(canvas_webgl::prelude::EXT_blend_minmax);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_blend_minmax_destroy(value: *mut EXT_blend_minmax) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct EXT_color_buffer_half_float(canvas_webgl::prelude::EXT_color_buffer_half_float);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_color_buffer_half_float_destroy(
    value: *mut EXT_color_buffer_half_float,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct EXT_disjoint_timer_query(canvas_webgl::prelude::EXT_disjoint_timer_query);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_disjoint_timer_query_destroy(
    value: *mut EXT_disjoint_timer_query,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct EXT_sRGB(canvas_webgl::prelude::EXT_sRGB);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_sRGB_destroy(value: *mut EXT_disjoint_timer_query) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct EXT_shader_texture_lod(canvas_webgl::prelude::EXT_shader_texture_lod);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_shader_texture_lod_destroy(
    value: *mut EXT_shader_texture_lod,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct EXT_texture_filter_anisotropic(canvas_webgl::prelude::EXT_texture_filter_anisotropic);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_EXT_texture_filter_anisotropic_destroy(
    value: *mut EXT_texture_filter_anisotropic,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_element_index_uint(canvas_webgl::prelude::OES_element_index_uint);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_element_index_uint_destroy(
    value: *mut OES_element_index_uint,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_standard_derivatives(canvas_webgl::prelude::OES_standard_derivatives);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_standard_derivatives_destroy(
    value: *mut OES_standard_derivatives,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_texture_float(canvas_webgl::prelude::OES_texture_float);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_texture_float_destroy(value: *mut OES_texture_float) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_texture_float_linear(canvas_webgl::prelude::OES_texture_float_linear);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_texture_float_linear_destroy(
    value: *mut OES_texture_float_linear,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_texture_half_float(canvas_webgl::prelude::OES_texture_half_float);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_texture_half_float_destroy(
    value: *mut OES_texture_half_float,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_texture_half_float_linear(canvas_webgl::prelude::OES_texture_half_float_linear);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_texture_half_float_linear_destroy(
    value: *mut OES_texture_half_float_linear,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct OES_vertex_array_object(canvas_webgl::prelude::OES_vertex_array_object);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_OES_vertex_array_object_destroy(
    value: *mut OES_vertex_array_object,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_color_buffer_float(canvas_webgl::prelude::WEBGL_color_buffer_float);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_color_buffer_float_destroy(
    value: *mut WEBGL_color_buffer_float,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_atc(canvas_webgl::prelude::WEBGL_compressed_texture_atc);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_atc_destroy(
    value: *mut WEBGL_compressed_texture_atc,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_etc1(canvas_webgl::prelude::WEBGL_compressed_texture_etc1);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_etc1_destroy(
    value: *mut WEBGL_compressed_texture_etc1,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_s3tc(canvas_webgl::prelude::WEBGL_compressed_texture_s3tc);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_s3tc_destroy(
    value: *mut WEBGL_compressed_texture_s3tc,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_s3tc_srgb(
    canvas_webgl::prelude::WEBGL_compressed_texture_s3tc_srgb,
);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_s3tc_srgb_destroy(
    value: *mut WEBGL_compressed_texture_s3tc_srgb,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_etc(canvas_webgl::prelude::WEBGL_compressed_texture_etc);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_etc_destroy(
    value: *mut WEBGL_compressed_texture_etc,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_compressed_texture_pvrtc(canvas_webgl::prelude::WEBGL_compressed_texture_pvrtc);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_compressed_texture_pvrtc_destroy(
    value: *mut WEBGL_compressed_texture_pvrtc,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_lose_context(canvas_webgl::prelude::WEBGL_lose_context);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_lose_context_destroy(value: *mut WEBGL_lose_context) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct ANGLE_instanced_arrays(canvas_webgl::prelude::ANGLE_instanced_arrays);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ANGLE_instanced_arrays_destroy(
    value: *mut ANGLE_instanced_arrays,
) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_depth_texture(canvas_webgl::prelude::WEBGL_depth_texture);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_depth_texture_destroy(value: *mut WEBGL_depth_texture) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[allow(non_camel_case_types)]
pub struct WEBGL_draw_buffers(canvas_webgl::prelude::WEBGL_draw_buffers);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WEBGL_draw_buffers_destroy(value: *mut WEBGL_draw_buffers) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

pub struct WebGLResult(canvas_webgl::prelude::WebGLResult);

#[no_mangle]
pub extern "C" fn canvas_native_webgl_WebGLResult_destroy(value: *mut WebGLResult) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

/* WebGLActiveInfo */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_info_get_name(
    info: *const WebGLActiveInfo,
) -> *const c_char {
    if info.is_null() {
        return std::ptr::null();
    }
    let info = unsafe { &*info };
    CString::new(info.get_name()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_info_get_size(info: *const WebGLActiveInfo) -> i32 {
    if info.is_null() {
        return 0;
    }
    let info = unsafe { &*info };
    info.get_size()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_info_get_type(info: *const WebGLActiveInfo) -> u32 {
    if info.is_null() {
        return 0;
    }
    let info = unsafe { &*info };
    info.get_type()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_info_get_is_empty(
    info: *const WebGLActiveInfo,
) -> bool {
    if info.is_null() {
        return false;
    }
    let info = unsafe { &*info };
    info.get_is_empty()
}

/* WebGLActiveInfo */

/* WebGLShaderPrecisionFormat */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_shader_precision_format_get_range_min(
    shader: *const WebGLShaderPrecisionFormat,
) -> i32 {
    if shader.is_null() {
        return 0;
    }
    let shader = unsafe { &*shader };
    shader.get_range_min()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_shader_precision_format_get_range_max(
    shader: *const WebGLShaderPrecisionFormat,
) -> i32 {
    if shader.is_null() {
        return 0;
    }
    let shader = unsafe { &*shader };
    shader.get_range_max()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_shader_precision_format_get_precision(
    shader: *const WebGLShaderPrecisionFormat,
) -> i32 {
    if shader.is_null() {
        return 0;
    }
    let shader = unsafe { &*shader };
    shader.get_precision()
}

/* WebGLShaderPrecisionFormat */

/* ContextAttributes */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_alpha(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_alpha()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_antialias(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_antialias()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_depth(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_depth()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_fail_if_major_performance_caveat()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_power_preference(
    attr: *const ContextAttributes,
) -> *const c_char {
    if attr.is_null() {
        return std::ptr::null();
    }
    let attr = unsafe { &*attr };
    CString::new(attr.get_power_preference())
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_premultiplied_alpha()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_preserve_drawing_buffer()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_stencil(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_stencil()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_desynchronized(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_desynchronized()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_attribute_get_get_xr_compatible(
    attr: *const ContextAttributes,
) -> bool {
    if attr.is_null() {
        return false;
    }
    let attr = unsafe { &*attr };
    attr.get_xr_compatible()
}

/* ContextAttributes */

/* WebGLExtension */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_is_none(
    extension: *const WebGLExtension,
) -> bool {
    assert!(!extension.is_null());
    let extension = unsafe { &*extension };
    extension.is_none()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_get_type(
    extension: *const WebGLExtension,
) -> WebGLExtensionType {
    let extension = unsafe { &*extension };
    extension.extension_type().into()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(
    extension: *mut WebGLExtension,
) -> *mut EXT_disjoint_timer_query {
    let extension = unsafe { Box::from_raw(&mut *extension) };
    Box::into_raw(extension.into_ext_disjoint_timer_query())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_to_angle_instanced_arrays(
    extension: *mut WebGLExtension,
) -> *mut ANGLE_instanced_arrays {
    let extension = unsafe { Box::from_raw(&mut *extension) };
    Box::into_raw(extension.into_angle_instanced_arrays())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_to_lose_context(
    extension: *mut WebGLExtension,
) -> *mut WEBGL_lose_context {
    let extension = unsafe { Box::from_raw(&mut *extension) };
    Box::into_raw(extension.into_lose_context())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_to_draw_buffers(
    extension: *mut WebGLExtension,
) -> *mut WEBGL_draw_buffers {
    let extension = unsafe { Box::from_raw(&mut *extension) };
    Box::into_raw(extension.into_draw_buffers())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_context_extension_to_oes_vertex_array_object(
    extension: *mut WebGLExtension,
) -> *mut OES_vertex_array_object {
    let extension = unsafe { Box::from_raw(&mut *extension) };
    Box::into_raw(extension.into_oes_vertex_array_object())
}

/* WebGLExtension */

/* WebGLResult */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_type(
    result: *const WebGLResult,
) -> WebGLResultType {
    assert!(!result.is_null());
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::Boolean(_) => WebGLResultType::WebGLResultTypeBoolean,
        canvas_webgl::prelude::WebGLResult::I32Array(_) => WebGLResultType::WebGLResultTypeI32Array,
        canvas_webgl::prelude::WebGLResult::U32Array(_) => WebGLResultType::WebGLResultTypeU32Array,
        canvas_webgl::prelude::WebGLResult::F32Array(_) => WebGLResultType::WebGLResultTypeF32Array,
        canvas_webgl::prelude::WebGLResult::BooleanArray(_) => {
            WebGLResultType::WebGLResultTypeBooleanArray
        }
        canvas_webgl::prelude::WebGLResult::U32(_) => WebGLResultType::WebGLResultTypeU32,
        canvas_webgl::prelude::WebGLResult::I32(_) => WebGLResultType::WebGLResultTypeI32,
        canvas_webgl::prelude::WebGLResult::F32(_) => WebGLResultType::WebGLResultTypeF32,
        canvas_webgl::prelude::WebGLResult::String(_) => WebGLResultType::WebGLResultTypeString,
        canvas_webgl::prelude::WebGLResult::None => WebGLResultType::WebGLResultTypeNone,
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_bool(result: *const WebGLResult) -> bool {
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::Boolean(value) => value,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_i32_array(
    result: *const WebGLResult,
) -> *mut I32Buffer {
    let result = unsafe { &*result };
    Box::into_raw(Box::new(I32Buffer::from(match &result.0 {
        canvas_webgl::prelude::WebGLResult::I32Array(value) => value.to_vec(),
        _ => Vec::new(),
    })))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_u32_array(
    result: *const WebGLResult,
) -> *mut U32Buffer {
    let result = unsafe { &*result };
    Box::into_raw(Box::new(U32Buffer::from(match &result.0 {
        canvas_webgl::prelude::WebGLResult::U32Array(value) => value.to_vec(),
        _ => Vec::new(),
    })))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_f32_array(
    result: *const WebGLResult,
) -> *mut F32Buffer {
    let result = unsafe { &*result };
    Box::into_raw(Box::new(F32Buffer::from(match &result.0 {
        canvas_webgl::prelude::WebGLResult::F32Array(value) => value.to_vec(),
        _ => Vec::new(),
    })))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_bool_array(
    result: *const WebGLResult,
) -> *mut U8Buffer {
    let result = unsafe { &*result };
    Box::into_raw(Box::new(U8Buffer::from(match &result.0 {
        canvas_webgl::prelude::WebGLResult::BooleanArray(value) => unsafe {
            std::slice::from_raw_parts(value.as_ptr() as *const u8, value.len()).to_vec()
        },
        _ => Vec::new(),
    })))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_u32(result: *const WebGLResult) -> u32 {
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::U32(value) => value,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_i32(result: *const WebGLResult) -> i32 {
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::I32(value) => value,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_f32(result: *const WebGLResult) -> f32 {
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::F32(value) => value,
        _ => 0.,
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_string(
    result: *const WebGLResult,
) -> *const c_char {
    let result = unsafe { &*result };
    let ret;
    match result.0 {
        canvas_webgl::prelude::WebGLResult::String(ref result) => {
            let val = result.to_string_lossy();

            if val.contains("OpenGL ES 3.0") {
                return CString::new("WebGL 2.0 (OpenGL ES 3.0 NativeScript)")
                    .unwrap()
                    .into_raw();
            }

            ret = CString::new("WebGL 1.0 (OpenGL ES 2.0 NativeScript)")
                .unwrap()
                .into_raw()
        }
        _ => {
            ret = std::ptr::null_mut();
        }
    };
    ret
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_result_get_is_none(result: *const WebGLResult) -> bool {
    let result = unsafe { &*result };
    match result.0 {
        canvas_webgl::prelude::WebGLResult::None => true,
        _ => false,
    }
}

/* WebGLResult */

/* WebGLState */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &*state };
    state.get_inner().get_unpack_colorspace_conversion_webgl()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_get_flip_y(state: *mut WebGLState) -> bool {
    let state = unsafe { &*state };
    state.get_inner().get_flip_y()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_get_premultiplied_alpha(
    state: *mut WebGLState,
) -> bool {
    let state = unsafe { &*state };
    state.get_inner().get_premultiplied_alpha()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_get_drawing_buffer_width(
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &*state };
    state.get_inner().get_drawing_buffer_width()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_state_get_drawing_buffer_height(
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &*state };
    state.get_inner().get_drawing_buffer_height()
}

/* WebGLState */

/* EXT_disjoint_timer_query */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(
    query: *const EXT_disjoint_timer_query,
) -> u32 {
    let query = unsafe { &*query };
    query.0.create_query_ext()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(
    value: u32,
    query: *const EXT_disjoint_timer_query,
) {
    let query = unsafe { &*query };
    query.0.delete_query_ext(value)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(
    value: u32,
    query: *const EXT_disjoint_timer_query,
) -> bool {
    let query = unsafe { &*query };
    query.0.is_query_ext(value)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(
    target: u32,
    value: u32,
    query: *const EXT_disjoint_timer_query,
) {
    let query = unsafe { &*query };
    query.0.begin_query_ext(target, value)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(
    target: u32,
    query: *const EXT_disjoint_timer_query,
) {
    let query = unsafe { &*query };
    query.0.end_query_ext(target)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(
    value: u32,
    target: u32,
    query: *const EXT_disjoint_timer_query,
) {
    let query = unsafe { &*query };
    query.0.query_counter_ext(value, target)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(
    target: u32,
    pname: u32,
    query: *const EXT_disjoint_timer_query,
) -> i32 {
    let query = unsafe { &*query };
    query.0.get_query_ext(target, pname)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(
    target: u32,
    pname: u32,
    query: *const EXT_disjoint_timer_query,
) -> *mut WebGLResult {
    let query = unsafe { &*query };
    Box::into_raw(Box::new(WebGLResult(
        query.0.get_query_object_ext(target, pname),
    )))
}

/* EXT_disjoint_timer_query */

/* ANGLE_instanced_arrays */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(
    mode: u32,
    first: i32,
    count: i32,
    primcount: i32,
    arrays: *const ANGLE_instanced_arrays,
) {
    let arrays = unsafe { &*arrays };
    arrays
        .0
        .draw_arrays_instanced_angle(mode, first, count, primcount)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(
    mode: u32,
    count: i32,
    type_: u32,
    offset: i32,
    primcount: i32,
    arrays: *const ANGLE_instanced_arrays,
) {
    let arrays = unsafe { &*arrays };
    arrays
        .0
        .draw_elements_instanced_angle(mode, count, type_, offset, primcount)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(
    index: u32,
    divisor: u32,
    arrays: *const ANGLE_instanced_arrays,
) {
    let arrays = unsafe { &*arrays };
    arrays.0.vertex_attrib_divisor_angle(index, divisor)
}
/* ANGLE_instanced_arrays */

/* WEBGL_lose_context */
#[no_mangle]
pub extern "C" fn canvas_native_webgl_lose_context_lose_context(
    context: *const WEBGL_lose_context,
) {
    let context = unsafe { &*context };

    context.0.lose_context()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_lose_context_restore_context(
    context: *const WEBGL_lose_context,
) {
    let context = unsafe { &*context };
    context.0.restore_context()
}
/* WEBGL_lose_context */

/* WEBGL_draw_buffers */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_draw_buffers_draw_buffers_webgl(
    buffers: *const u32,
    size: usize,
    context: *const WEBGL_draw_buffers,
) {
    assert!(!context.is_null());
    let buffers = unsafe { std::slice::from_raw_parts(buffers, size) };
    let context = unsafe { &*context };
    context.0.draw_buffers_webgl(buffers);
}

/* WEBGL_draw_buffers */

/* OES_vertex_array_object */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(
    object: *const OES_vertex_array_object,
) -> u32 {
    let object = unsafe { &*object };
    object.0.create_vertex_array_oes()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(
    array_object: u32,
    object: *const OES_vertex_array_object,
) {
    let object = unsafe { &*object };
    object.0.delete_vertex_array_oes(array_object)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(
    array_object: u32,
    object: *const OES_vertex_array_object,
) -> bool {
    let object = unsafe { &*object };
    object.0.is_vertex_array_oes(array_object)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(
    array_object: u32,
    object: *const OES_vertex_array_object,
) {
    let object = unsafe { &*object };
    object.0.bind_vertex_array_oes(array_object)
}

/* OES_vertex_array_object */

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create(
    gl_context: i64,
    version: *const c_char,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: *const c_char,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
) -> *mut WebGLState {
    if version.is_null() || power_preference.is_null() {
        return std::ptr::null_mut();
    }
    let version = unsafe { CStr::from_ptr(version) };
    let version = version.to_string_lossy();

    let power_preference = unsafe { CStr::from_ptr(power_preference) };
    let power_preference = power_preference.to_string_lossy();

    let version = if version.eq("v1") {
        WebGLVersion::V1
    } else if version.eq("v2") {
        WebGLVersion::V2
    } else {
        WebGLVersion::NONE
    };

    let gl_context = gl_context as *const RwLock<canvas_core::gl::GLContextInner>;
    let gl_context = canvas_core::gl::GLContext::from_raw_inner(gl_context);

    let inner = WebGLState::new_with_context(
        gl_context,
        version,
        alpha,
        antialias,
        depth,
        fail_if_major_performance_caveat,
        power_preference.as_ref(),
        premultiplied_alpha,
        preserve_drawing_buffer,
        stencil,
        desynchronized,
        xr_compatible,
        false,
    );
    Box::into_raw(Box::new(inner))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_no_window(
    width: i32,
    height: i32,
    version: *const c_char,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: *const c_char,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    is_canvas: bool,
) -> *mut WebGLState {
    if version.is_null() || power_preference.is_null() {
        return std::ptr::null_mut();
    }
    let version = unsafe { CStr::from_ptr(version) };
    let version = version.to_string_lossy();

    let power_preference = unsafe { CStr::from_ptr(power_preference) };
    let power_preference = power_preference.to_string_lossy();

    Box::into_raw(Box::new(canvas_native_webgl_create_no_window_internal(
        width,
        height,
        version.as_ref(),
        alpha,
        antialias,
        depth,
        fail_if_major_performance_caveat,
        power_preference.as_ref(),
        premultiplied_alpha,
        preserve_drawing_buffer,
        stencil,
        desynchronized,
        xr_compatible,
        is_canvas,
    )))
}

fn canvas_native_webgl_create_no_window_internal(
    width: i32,
    height: i32,
    version: &str,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: &str,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    is_canvas: bool,
) -> WebGLState {
    let version = if version.eq("v1") || version.eq("canvas") {
        WebGLVersion::V1
    } else if version.eq("v2") {
        WebGLVersion::V2
    } else {
        WebGLVersion::NONE
    };

    let mut attrs = canvas_core::context_attributes::ContextAttributes::new(
        alpha,
        antialias,
        depth,
        fail_if_major_performance_caveat,
        power_preference,
        premultiplied_alpha,
        preserve_drawing_buffer,
        stencil,
        desynchronized,
        xr_compatible,
        is_canvas,
    );

    let ctx = canvas_core::gl::GLContext::create_offscreen_context(&mut attrs, width, height)
        .unwrap_or_default();

    WebGLState::new_with_context(
        ctx,
        version,
        attrs.get_alpha(),
        attrs.get_antialias(),
        attrs.get_depth(),
        attrs.get_fail_if_major_performance_caveat(),
        attrs.get_power_preference().as_str(),
        attrs.get_premultiplied_alpha(),
        attrs.get_preserve_drawing_buffer(),
        attrs.get_stencil(),
        attrs.get_desynchronized(),
        attrs.get_xr_compatible(),
        attrs.get_is_canvas(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_active_texture(texture: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_active_texture(texture, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_attach_shader(
    program: u32,
    shader: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_attach_shader(program, shader, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_bind_attrib_location(
    program: u32,
    index: u32,
    name: *const c_char,
    state: *mut WebGLState,
) {
    if name.is_null() {
        return;
    }
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_bind_attrib_location(
        program,
        index,
        name.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_bind_buffer(
    target: u32,
    buffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_bind_buffer(target, buffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_bind_frame_buffer(
    target: u32,
    framebuffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_bind_frame_buffer(
        target,
        framebuffer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_bind_render_buffer(
    target: u32,
    renderbuffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_bind_render_buffer(
        target,
        renderbuffer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_bind_texture(
    target: u32,
    texture: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_bind_texture(target, texture, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_blend_color(
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_blend_color(
        red,
        green,
        blue,
        alpha,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_blend_equation_separate(
    mode_rgb: u32,
    mode_alpha: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_blend_equation_separate(
        mode_rgb,
        mode_alpha,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_blend_equation(mode: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_blend_equation(mode, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_blend_func_separate(
    src_rgb: u32,
    dst_rgb: u32,
    src_alpha: u32,
    dst_alpha: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_blend_func_separate(
        src_rgb,
        dst_rgb,
        src_alpha,
        dst_alpha,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_blend_func(
    sfactor: u32,
    dfactor: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_blend_func(sfactor, dfactor, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data(
    target: u32,
    src_data: *const u8,
    size: usize,
    usage: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data_u16(
    target: u32,
    src_data: *const u16,
    size: usize,
    usage: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_u16(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data_f32(
    target: u32,
    src_data: *const f32,
    size: usize,
    usage: u32,
    state: *mut WebGLState,
) {
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_f32(
        target,
        src_data,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_data_none(
    target: u32,
    size: isize,
    usage: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data_none(
        target,
        size,
        usage,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_sub_data(
    target: u32,
    offset: isize,
    src_data: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src_data = unsafe { std::slice::from_raw_parts(src_data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data(
        target,
        offset,
        src_data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_buffer_sub_data_none(
    target: u32,
    offset: isize,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_buffer_sub_data_none(
        target,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_check_frame_buffer_status(
    target: u32,
    state: *mut WebGLState,
) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_check_frame_buffer_status(
        target,
        state.get_inner_mut(),
    )
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn canvas_native_webgl_clear(mask: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_clear(mask, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_clear_color(
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_clear_color(
        red,
        green,
        blue,
        alpha,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_clear_depth(depth: f32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_clear_depth(depth, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_clear_stencil(stencil: i32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_clear_stencil(stencil, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_color_mask(
    red: bool,
    green: bool,
    blue: bool,
    alpha: bool,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_color_mask(
        red,
        green,
        blue,
        alpha,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_commit(_: *mut WebGLState) {
    // noop
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_compile_shader(shader: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_compile_shader(shader, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_compressed_tex_image2d(
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    border: i32,
    pixels: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let pixels = unsafe { std::slice::from_raw_parts(pixels, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_image2d(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        pixels,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_compressed_tex_image2d_none(
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    border: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_image2d_none(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_compressed_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    pixels: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let pixels = unsafe { std::slice::from_raw_parts(pixels, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_compressed_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        width,
        height,
        format,
        pixels,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_copy_tex_image2d(
    target: u32,
    level: i32,
    internalformat: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    border: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_copy_tex_image2d(
        target,
        level,
        internalformat,
        x,
        y,
        width,
        height,
        border,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_copy_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_copy_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        x,
        y,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_buffer(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_buffer(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_framebuffer(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_framebuffer(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_program(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_program(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_renderbuffer(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_renderbuffer(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_shader(
    shader_type: u32,
    state: *mut WebGLState,
) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_shader(shader_type, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_create_texture(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_create_texture(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_cull_face(mode: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_cull_face(mode, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_buffer(buffer: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_buffer(buffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_framebuffer(
    frame_buffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_framebuffer(frame_buffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_program(program: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_program(program, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_renderbuffer(
    render_buffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_renderbuffer(
        render_buffer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_shader(shader: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_shader(shader, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_delete_texture(texture: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_delete_texture(texture, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_depth_func(func: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_depth_func(func, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_depth_mask(flag: bool, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_depth_mask(flag, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_depth_range(z_near: f32, z_far: f32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_depth_range(z_near, z_far, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_detach_shader(
    program: u32,
    shader: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_detach_shader(program, shader, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_disable(cap: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_disable(cap, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_disable_vertex_attrib_array(
    index: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_disable_vertex_attrib_array(
        index,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_draw_arrays(
    mode: u32,
    first: i32,
    count: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_draw_arrays(mode, first, count, state.get_inner_mut())
    // Flush Context
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_draw_elements(
    mode: u32,
    count: i32,
    element_type: u32,
    offset: isize,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_draw_elements(
        mode,
        count,
        element_type,
        offset,
        state.get_inner_mut(),
    )
    // Flush Context
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_enable(cap: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_enable(cap, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_enable_vertex_attrib_array(
    index: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_enable_vertex_attrib_array(
        index,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_finish(state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_finish(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_flush(state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_flush(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_renderbuffer(
    target: u32,
    attachment: u32,
    renderbuffertarget: u32,
    renderbuffer: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_framebuffer_renderbuffer(
        target,
        attachment,
        renderbuffertarget,
        renderbuffer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_texture2d(
    target: u32,
    attachment: u32,
    textarget: u32,
    texture: u32,
    level: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_framebuffer_texture2d(
        target,
        attachment,
        textarget,
        texture,
        level,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_front_face(mode: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_front_face(mode, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_generate_mipmap(target: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_generate_mipmap(target, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_active_attrib(
    program: u32,
    index: u32,
    state: *mut WebGLState,
) -> *mut WebGLActiveInfo {
    let state = unsafe { &mut *state };
    let info = canvas_webgl::webgl::canvas_native_webgl_get_active_attrib(
        program,
        index,
        state.get_inner_mut(),
    );

    Box::into_raw(Box::new(WebGLActiveInfo(info)))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_active_uniform(
    program: u32,
    index: u32,
    state: *mut WebGLState,
) -> *mut WebGLActiveInfo {
    let state = unsafe { &mut *state };
    let info = canvas_webgl::webgl::canvas_native_webgl_get_active_uniform(
        program,
        index,
        state.get_inner_mut(),
    );

    Box::into_raw(Box::new(WebGLActiveInfo(info)))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_attached_shaders(
    program: u32,
    state: *mut WebGLState,
) -> *mut U32Buffer {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(U32Buffer::from(
        canvas_webgl::webgl::canvas_native_webgl_get_attached_shaders(
            program,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_attrib_location(
    program: u32,
    name: *const c_char,
    state: *mut WebGLState,
) -> i32 {
    assert!(!state.is_null());
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_attrib_location(
        program,
        name.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_buffer_parameter(
    target: u32,
    pname: u32,
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_buffer_parameter(
        target,
        pname,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_context_attributes(
    state: *mut WebGLState,
) -> *mut ContextAttributes {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(ContextAttributes(
        canvas_webgl::webgl::canvas_native_webgl_get_context_attributes(state.get_inner()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_error(state: *mut WebGLState) -> u32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_error(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_extension(
    name: *const c_char,
    state: *mut WebGLState,
) -> *mut WebGLExtension {
    assert!(!state.is_null());
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_string_lossy();
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLExtension(
        canvas_webgl::webgl::canvas_native_webgl_get_extension(
            name.as_ref(),
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_framebuffer_attachment_parameter(
    target: u32,
    attachment: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLFramebufferAttachmentParameter {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLFramebufferAttachmentParameter(
        canvas_webgl::webgl::canvas_native_webgl_get_framebuffer_attachment_parameter(
            target,
            attachment,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(
    param: *const WebGLFramebufferAttachmentParameter,
) -> bool {
    assert!(!param.is_null());
    let param = unsafe { &*param };
    param.get_is_texture()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(
    param: *const WebGLFramebufferAttachmentParameter,
) -> bool {
    assert!(!param.is_null());
    let param = unsafe { &*param };

    param.get_is_renderbuffer()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_framebuffer_attachment_parameter_get_value(
    param: *const WebGLFramebufferAttachmentParameter,
) -> i32 {
    assert!(!param.is_null());
    let param = unsafe { &*param };
    param.get_value()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_parameter(
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_parameter(pname, state.get_inner_mut()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_program_info_log(
    program: u32,
    state: *mut WebGLState,
) -> *const c_char {
    let state = unsafe { &mut *state };
    CString::new(
        canvas_webgl::webgl::canvas_native_webgl_get_program_info_log(
            program,
            state.get_inner_mut(),
        ),
    )
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_program_parameter(
    program: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_program_parameter(
            program,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_renderbuffer_parameter(
    target: u32,
    pname: u32,
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_renderbuffer_parameter(
        target,
        pname,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_shader_info_log(
    shader: u32,
    state: *mut WebGLState,
) -> *const c_char {
    let state = unsafe { &mut *state };
    CString::new(
        canvas_webgl::webgl::canvas_native_webgl_get_shader_info_log(shader, state.get_inner_mut()),
    )
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_shader_parameter(
    shader: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_shader_parameter(
            shader,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_shader_precision_format(
    shader_type: u32,
    precision_type: u32,
    state: *mut WebGLState,
) -> *mut WebGLShaderPrecisionFormat {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLShaderPrecisionFormat(
        canvas_webgl::webgl::canvas_native_webgl_get_shader_precision_format(
            shader_type,
            precision_type,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_shader_source(
    shader: u32,
    state: *mut WebGLState,
) -> *const c_char {
    let state = unsafe { &mut *state };
    CString::new(canvas_webgl::webgl::canvas_native_webgl_get_shader_source(
        shader,
        state.get_inner_mut(),
    ))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_supported_extensions(
    state: *mut WebGLState,
) -> *mut StringBuffer {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(StringBuffer::from(
        canvas_webgl::webgl::canvas_native_webgl_get_supported_extensions(state.get_inner_mut()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_supported_extensions_to_string(
    state: *mut WebGLState,
) -> *const c_char {
    let state = unsafe { &mut *state };
    let ret =
        canvas_webgl::webgl::canvas_native_webgl_get_supported_extensions(state.get_inner_mut());

    CString::new(ret.join(",").to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_tex_parameter(
    target: u32,
    pname: u32,
    state: *mut WebGLState,
) -> i32 {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_tex_parameter(target, pname, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_uniform_location(
    program: u32,
    name: *const c_char,
    state: *mut WebGLState,
) -> i32 {
    assert!(!state.is_null());
    assert!(!name.is_null());
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_uniform_location(
        program,
        name.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_uniform(
    program: u32,
    location: i32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    let state = unsafe { &mut *state };

    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_uniform(
            program,
            location,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_vertex_attrib_offset(
    index: u32,
    pname: u32,
    state: *mut WebGLState,
) -> usize {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_get_vertex_attrib_offset(
        index,
        pname,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_vertex_attrib(
    index: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl::canvas_native_webgl_get_vertex_attrib(
            index,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_get_is_context_lost(_: *mut WebGLState) -> bool {
    // TODO improve
    false
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_hint(target: u32, mode: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_hint(target, mode, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_buffer(buffer: u32, state: *mut WebGLState) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_buffer(buffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_enabled(cap: u32, state: *mut WebGLState) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_enabled(cap, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_framebuffer(
    framebuffer: u32,
    state: *mut WebGLState,
) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_framebuffer(framebuffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_program(program: u32, state: *mut WebGLState) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_program(program, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_renderbuffer(
    renderbuffer: u32,
    state: *mut WebGLState,
) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_renderbuffer(renderbuffer, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_shader(shader: u32, state: *mut WebGLState) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_shader(shader, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_is_texture(texture: u32, state: *mut WebGLState) -> bool {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_is_texture(texture, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_line_width(width: f32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_line_width(width, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_link_program(program: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_link_program(program, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_pixel_storei(pname: u32, param: i32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_pixel_storei(pname, param, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_polygon_offset(
    factor: f32,
    units: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_polygon_offset(factor, units, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_read_pixels_u8(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: *mut u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let pixels = unsafe { std::slice::from_raw_parts_mut(pixels, size) };
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_u8(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_read_pixels_u16(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: *mut u16,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let pixels = unsafe { std::slice::from_raw_parts_mut(pixels, size) };
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_u16(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_read_pixels_f32(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    pixel_type: u32,
    pixels: *mut f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let pixels = unsafe { std::slice::from_raw_parts_mut(pixels, size) };
    canvas_webgl::webgl::canvas_native_webgl_read_pixels_f32(
        x,
        y,
        width,
        height,
        format,
        pixel_type,
        pixels,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_renderbuffer_storage(
    target: u32,
    internal_format: u32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_renderbuffer_storage(
        target,
        internal_format,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_sample_coverage(
    value: f32,
    invert: bool,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_sample_coverage(value, invert, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_scissor(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_scissor(x, y, width, height, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_shader_source(
    shader: u32,
    source: *const c_char,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!source.is_null());
    let source = unsafe { CStr::from_ptr(source) };
    let source = source.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_shader_source(
        shader,
        source.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_func(
    func: u32,
    reference: i32,
    mask: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_func(
        func,
        reference,
        mask,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_func_separate(
    face: u32,
    func: u32,
    reference: i32,
    mask: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_func_separate(
        face,
        func,
        reference,
        mask,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_mask(mask: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_mask(mask, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_mask_separate(
    face: u32,
    mask: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_mask_separate(
        face,
        mask,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_op(
    fail: u32,
    zfail: u32,
    zpass: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_op(fail, zfail, zpass, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_stencil_op_separate(
    face: u32,
    fail: u32,
    zfail: u32,
    zpass: u32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_stencil_op_separate(
        face,
        fail,
        zfail,
        zpass,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d_image_none(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    state: *mut WebGLState,
) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_image_none(
        target,
        level,
        internalformat,
        format,
        image_type,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d_canvas2d(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    canvas: *mut CanvasRenderingContext2D,
    state: *mut WebGLState,
) {
    assert!(!canvas.is_null());
    assert!(!state.is_null());
    let canvas = unsafe { &mut *canvas };
    let state = unsafe { &mut *state };

    let width: i32;
    let height: i32;
    let source_non_gpu;
    {
        let ctx = canvas.get_context();
        let device = ctx.device();
        width = device.width as i32;
        height = device.height as i32;
        source_non_gpu = device.non_gpu;
    }

    if !source_non_gpu {
        canvas.make_current();
        let mut source_ctx = canvas.get_context_mut();

        let snapshot = source_ctx.snapshot();

        if let Some(pixels) = snapshot.peek_pixels() {
            if let Some(buf) = pixels.bytes() {
                let buf = buf.to_vec();
                state.0.make_current();

                canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
                    target,
                    level,
                    internalformat,
                    width,
                    height,
                    0,
                    format,
                    image_type,
                    buf.as_slice(),
                    state.get_inner_mut(),
                );
            }
        } else {
            if let Some(buf) = source_ctx.to_raster_pixels(
                snapshot,
                if internalformat == gl_bindings::RGBA as i32 {
                    true
                } else {
                    false
                },
                true,
            ) {
                state.0.make_current();

                canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
                    target,
                    level,
                    internalformat,
                    width,
                    height,
                    0,
                    format,
                    image_type,
                    buf.as_slice(),
                    state.get_inner_mut(),
                );
            }
        }
    } else {
        let mut source_ctx = canvas.get_context_mut();
        canvas.make_current();
        let buf = source_ctx.read_pixels();
        canvas.remove_if_current();
        canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            image_type,
            buf.as_slice(),
            state.get_inner_mut(),
        );
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d_webgl(
    target: i32,
    level: i32,
    _internalformat: i32,
    _format: i32,
    _image_type: i32,
    webgl: *mut WebGLState,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!webgl.is_null());
    let state = unsafe { &mut *state };
    let webgl = unsafe { &mut *webgl };
    let mut pixels =
        canvas_webgl::webgl::canvas_native_webgl_read_webgl_pixels(&mut webgl.0, &mut state.0);
    //   gl_bindings::RGBA,
    //             gl_bindings::UNSIGNED_BYTE,
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        // internalformat,
        gl_bindings::RGBA as _,
        pixels.0,
        pixels.1,
        0,
        // format,
        gl_bindings::RGBA as _,
        //image_type,
        gl_bindings::UNSIGNED_BYTE as _,
        pixels.2.as_mut_slice(),
        state.get_inner_mut(),
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d(
    target: i32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: i32,
    image_type: i32,
    buf: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        format,
        image_type,
        buf,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d_none(
    target: i32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: i32,
    image_type: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_none(
        target,
        level,
        internalformat,
        width,
        height,
        border,
        format,
        image_type,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_image2d_image_asset(
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    image_type: i32,
    image_asset: *mut ImageAsset,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!image_asset.is_null());
    let image_asset = unsafe { &*image_asset };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_tex_image2d_asset(
        target,
        level,
        internalformat,
        format,
        image_type,
        &image_asset.0,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_parameterf(
    target: u32,
    pname: u32,
    param: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_tex_parameterf(target, pname, param, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_parameteri(
    target: u32,
    pname: u32,
    param: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_tex_parameteri(target, pname, param, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_sub_image2d_asset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    format: u32,
    image_type: i32,
    asset: *mut ImageAsset,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!asset.is_null());
    let asset = unsafe { &*asset };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d_asset(
        target,
        level,
        xoffset,
        yoffset,
        format,
        image_type,
        &asset.0,
        state.get_inner(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_sub_image2d_canvas2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    format: u32,
    image_type: i32,
    canvas: *mut CanvasRenderingContext2D,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!canvas.is_null());
    let state = unsafe { &mut *state };
    let canvas = unsafe { &mut *canvas };

    let source_non_gpu;
    {
        let ctx = canvas.get_context();
        let device = ctx.device();
        source_non_gpu = device.non_gpu;
    }

    let mut buf;
    if !source_non_gpu {
        canvas.make_current();
        let mut source_ctx = canvas.get_context_mut();
        source_ctx.flush_and_sync_cpu();
        let snapshot = source_ctx.snapshot();
        let width = snapshot.width();
        let height = snapshot.height();

        if let Some(pixels) = snapshot.peek_pixels() {
            if let Some(buf) = pixels.bytes() {
                let mut buf = buf.to_vec();
                state.0.make_current();

                canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    width,
                    height,
                    format,
                    image_type,
                    buf.as_mut_slice(),
                    state.get_inner_mut(),
                );
            }
        } else {
            if let Some(mut buf) = source_ctx.to_raster_pixels(
                snapshot,
                if format == gl_bindings::RGBA {
                    true
                } else {
                    false
                },
                true,
            ) {
                state.0.make_current();

                canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d(
                    target,
                    level,
                    xoffset,
                    yoffset,
                    width,
                    height,
                    format,
                    image_type,
                    buf.as_mut_slice(),
                    state.get_inner_mut(),
                );
            }
        }
    } else {
        let mut source_ctx = canvas.get_context_mut();
        canvas.make_current();
        let width = source_ctx.device().width;
        let height = source_ctx.device().height;
        buf = source_ctx.read_pixels();
        canvas.remove_if_current();

        canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d(
            target,
            level,
            xoffset,
            yoffset,
            width as _,
            height as _,
            format,
            image_type,
            buf.as_mut_slice(),
            state.get_inner_mut(),
        );
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_sub_image2d_webgl(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    _format: u32,
    _image_type: i32,
    webgl: *mut WebGLState,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!webgl.is_null());
    let state = unsafe { &mut *state };
    let webgl = unsafe { &mut *webgl };
    {
        let state = &state.0;
        state.remove_if_current();
    }
    let source = webgl.get_inner();
    source.make_current();
    let width = source.drawing_buffer_width();
    let height = source.drawing_buffer_height();

    let mut pixels =
        canvas_webgl::webgl::canvas_native_webgl_read_webgl_pixels(&mut webgl.0, &mut state.0);

    canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        width,
        height,
        // format,
        gl_bindings::RGBA as _,
        // image_type,
        gl_bindings::UNSIGNED_BYTE as _,
        pixels.2.as_mut_slice(),
        state.get_inner_mut(),
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_tex_sub_image2d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    image_type: i32,
    buf: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_tex_sub_image2d(
        target,
        level,
        xoffset,
        yoffset,
        width,
        height,
        format,
        image_type,
        buf,
        state.get_inner(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform1f(location: i32, v0: f32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform1f(location, v0, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform1fv(
    location: i32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform1fv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform1i(location: i32, v0: i32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform1i(location, v0, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform1iv(
    location: i32,
    value: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform1iv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform2f(
    location: i32,
    v0: f32,
    v1: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform2f(location, v0, v1, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform2fv(
    location: i32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };

    canvas_webgl::webgl::canvas_native_webgl_uniform2fv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform2i(
    location: i32,
    v0: i32,
    v1: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform2i(location, v0, v1, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform2iv(
    location: i32,
    value: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform2iv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform3f(
    location: i32,
    v0: f32,
    v1: f32,
    v2: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform3f(location, v0, v1, v2, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform3fv(
    location: i32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform3fv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform3i(
    location: i32,
    v0: i32,
    v1: i32,
    v2: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform3i(location, v0, v1, v2, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform3iv(
    location: i32,
    value: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &*state };
    canvas_webgl::webgl::canvas_native_webgl_uniform3iv(location, value, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform4f(
    location: i32,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform4f(location, v0, v1, v2, v3, state.get_inner())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform4fv(
    location: i32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform4fv(location, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform4i(
    location: i32,
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform4i(
        location,
        v0,
        v1,
        v2,
        v3,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform4iv(
    location: i32,
    value: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform4iv(location, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform_matrix2fv(
    location: i32,
    transpose: bool,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix2fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform_matrix3fv(
    location: i32,
    transpose: bool,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix3fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_uniform_matrix4fv(
    location: i32,
    transpose: bool,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_uniform_matrix4fv(
        location,
        transpose,
        value,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_use_program(program: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_use_program(program, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_validate_program(program: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_validate_program(program, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib1f(index: u32, v0: f32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib1f(index, v0, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib1fv(
    index: u32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib1fv(index, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib2f(
    index: u32,
    v0: f32,
    v1: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib2f(index, v0, v1, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib2fv(
    index: u32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib2fv(index, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib3f(
    index: u32,
    v0: f32,
    v1: f32,
    v2: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib3f(
        index,
        v0,
        v1,
        v2,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib3fv(
    index: u32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib3fv(index, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib4f(
    index: u32,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib4f(
        index,
        v0,
        v1,
        v2,
        v3,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib4fv(
    index: u32,
    value: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib4fv(index, value, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_vertex_attrib_pointer(
    index: u32,
    size: i32,
    d_type: u32,
    normalized: bool,
    stride: i32,
    offset: isize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib_pointer(
        index,
        size,
        d_type,
        normalized,
        stride,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl_viewport(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl::canvas_native_webgl_viewport(x, y, width, height, state.get_inner_mut())
}

/* WebGL */

/* WEBGL IMPL */

/* WEBGL2 IMPL */

/* WebGL2 */

pub struct WebGLSync(canvas_webgl::webgl2::GLSync);

pub struct WebGLIndexedParameter(canvas_webgl::prelude::WebGLIndexedParameter);

/* WebGLIndexedParameter */
#[no_mangle]
pub extern "C" fn canvas_native_webgl2_indexed_parameter_get_value(
    param: &WebGLIndexedParameter,
) -> isize {
    param.0.get_value()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_indexed_parameter_get_buffer_value(
    param: &WebGLIndexedParameter,
) -> isize {
    param.0.get_buffer_value()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_indexed_parameter_get_is_buffer(
    param: &WebGLIndexedParameter,
) -> bool {
    param.0.get_is_buffer()
}
/* WebGLIndexedParameter */

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_begin_query(target: u32, id: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_begin_query(target, id, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_begin_transform_feedback(
    primitive_mode: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_begin_transform_feedback(
        primitive_mode,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_bind_buffer_base(
    target: u32,
    index: u32,
    buffer: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_bind_buffer_base(
        target,
        index,
        buffer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_bind_buffer_range(
    target: u32,
    index: u32,
    buffer: u32,
    offset: isize,
    size: isize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_bind_buffer_range(
        target,
        index,
        buffer,
        offset,
        size,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_bind_sampler(
    unit: u32,
    sampler: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_bind_sampler(unit, sampler, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_bind_transform_feedback(
    target: u32,
    transform_feedback: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_bind_sampler(
        target,
        transform_feedback,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_bind_vertex_array(
    vertex_array: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_bind_vertex_array(
        vertex_array,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_blit_framebuffer(
    src_x0: i32,
    src_y0: i32,
    src_x1: i32,
    src_y1: i32,
    dst_x0: i32,
    dst_y0: i32,
    dst_x1: i32,
    dst_y1: i32,
    mask: u32,
    filter: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_blit_framebuffer(
        src_x0,
        src_y0,
        src_x1,
        src_y1,
        dst_x0,
        dst_y0,
        dst_x1,
        dst_y1,
        mask,
        filter,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_clear_bufferfi(
    buffer: u32,
    drawbuffer: i32,
    depth: f32,
    stencil: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferfi(
        buffer,
        drawbuffer,
        depth,
        stencil,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_clear_bufferfv(
    buffer: u32,
    drawbuffer: i32,
    values: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let values = unsafe { std::slice::from_raw_parts(values, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferfv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_clear_bufferiv(
    buffer: u32,
    drawbuffer: i32,
    values: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let values = unsafe { std::slice::from_raw_parts(values, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferiv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_clear_bufferuiv(
    buffer: u32,
    drawbuffer: i32,
    values: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let values = unsafe { std::slice::from_raw_parts(values, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_clear_bufferuiv(
        buffer,
        drawbuffer,
        values,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_client_wait_sync(
    sync: *const WebGLSync,
    flags: u32,
    timeout: isize,
    state: *mut WebGLState,
) -> u32 {
    assert!(!state.is_null());
    assert!(!sync.is_null());
    let state = unsafe { &mut *state };
    let sync = unsafe { &*sync };
    canvas_webgl::webgl2::canvas_native_webgl2_client_wait_sync(
        &sync.0,
        flags,
        timeout as c_ulong,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_compressed_tex_sub_image3d_none(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    image_size: i32,
    offset: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_compressed_tex_sub_image3d_none(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        image_size,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_compressed_tex_sub_image3d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    src: *const u8,
    size: usize,
    src_offset: usize,
    src_length_override: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let src = unsafe { std::slice::from_raw_parts(src, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_compressed_tex_sub_image3d(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        src,
        src_offset,
        src_length_override,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_copy_buffer_sub_data(
    read_target: u32,
    write_target: u32,
    read_offset: isize,
    write_offset: isize,
    size: isize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_copy_buffer_sub_data(
        read_target,
        write_target,
        read_offset,
        write_offset,
        size,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_copy_tex_sub_image3d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_copy_tex_sub_image3d(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        x,
        y,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_create_query(state: *mut WebGLState) -> u32 {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_create_query(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_create_sampler(state: *mut WebGLState) -> u32 {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_create_sampler(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_create_transform_feedback(state: *mut WebGLState) -> u32 {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_create_transform_feedback(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_create_vertex_array(state: *mut WebGLState) -> u32 {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_create_vertex_array(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_delete_query_with_query(id: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_delete_query_with_query(id, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_delete_sampler_with_sampler(
    sampler: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_delete_sampler_with_sampler(
        sampler,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_delete_sync_with_sync(
    sync: *const WebGLSync,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    assert!(!sync.is_null());
    let state = unsafe { &mut *state };
    let sync = unsafe { &*sync };
    canvas_webgl::webgl2::canvas_native_webgl2_delete_sync_with_sync(&sync.0, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_delete_transform_feedback(
    transform_feedback: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_delete_transform_feedback(
        transform_feedback,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_delete_vertex_array_with_vertex_array(
    vertex_array: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_delete_vertex_array_with_vertex_array(
        vertex_array,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_draw_arrays_instanced(
    mode: u32,
    first: i32,
    count: i32,
    instance_count: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_draw_arrays_instanced(
        mode,
        first,
        count,
        instance_count,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_draw_buffers(
    buffers: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buffers = unsafe { std::slice::from_raw_parts(buffers, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_draw_buffers(buffers, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_draw_elements_instanced(
    mode: u32,
    count: i32,
    type_: u32,
    offset: isize,
    instance_count: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_draw_elements_instanced(
        mode,
        count,
        type_,
        offset,
        instance_count,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_draw_range_elements(
    mode: u32,
    start: u32,
    end: u32,
    count: i32,
    type_: u32,
    offset: isize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_draw_range_elements(
        mode,
        start,
        end,
        count,
        type_,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_end_query(target: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_end_query(target, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_end_transform_feedback(state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_end_transform_feedback(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_fence_sync(
    condition: u32,
    flags: u32,
    state: *mut WebGLState,
) -> *mut WebGLSync {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLSync(
        canvas_webgl::webgl2::canvas_native_webgl2_fence_sync(
            condition,
            flags,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_framebuffer_texture_layer(
    target: u32,
    attachment: u32,
    texture: u32,
    level: i32,
    layer: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_framebuffer_texture_layer(
        target,
        attachment,
        texture,
        level,
        layer,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_active_uniform_block_name(
    program: u32,
    uniform_block_index: u32,
    state: *mut WebGLState,
) -> *const c_char {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    CString::new(
        canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniform_block_name(
            program,
            uniform_block_index,
            state.get_inner_mut(),
        ),
    )
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_active_uniform_block_parameter(
    program: u32,
    uniform_block_index: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniform_block_parameter(
            program,
            uniform_block_index,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_active_uniforms(
    program: u32,
    uniform_indices: *const u32,
    size: usize,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let uniform_indices = unsafe { std::slice::from_raw_parts(uniform_indices, size) };
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_active_uniforms(
            program,
            uniform_indices,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_buffer_sub_data(
    target: u32,
    src_byte_offset: isize,
    dst_data: *mut u8,
    size: usize,
    dst_offset: usize,
    length: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let dst_data = unsafe { std::slice::from_raw_parts_mut(dst_data, size) };
    canvas_webgl::webgl2::canvas_native_webgl2_get_buffer_sub_data(
        target,
        src_byte_offset,
        dst_data,
        dst_offset,
        length,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_frag_data_location(
    program: u32,
    name: *const c_char,
    state: *mut WebGLState,
) -> i32 {
    assert!(!state.is_null());
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_get_frag_data_location(
        program,
        name.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_indexed_parameter(
    target: u32,
    index: u32,
    state: *mut WebGLState,
) -> *mut WebGLIndexedParameter {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLIndexedParameter(
        canvas_webgl::webgl2::canvas_native_webgl2_get_indexed_parameter(
            target,
            index,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_internalformat_parameter(
    target: u32,
    internalformat: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_internalformat_parameter(
            target,
            internalformat,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_parameter(
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_parameter(pname, state.get_inner_mut()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_query_parameter(
    query: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_query_parameter(
            query,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_query(
    target: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_query(target, pname, state.get_inner_mut()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_sampler_parameter(
    sampler: u32,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_sampler_parameter(
            sampler,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_sync_parameter(
    sync: *const WebGLSync,
    pname: u32,
    state: *mut WebGLState,
) -> *mut WebGLResult {
    assert!(!sync.is_null());
    assert!(!state.is_null());
    let sync = unsafe { &*sync };
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLResult(
        canvas_webgl::webgl2::canvas_native_webgl2_get_sync_parameter(
            &sync.0,
            pname,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_transform_feedback_varying(
    program: u32,
    index: u32,
    state: *mut WebGLState,
) -> *mut WebGLActiveInfo {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(WebGLActiveInfo(
        canvas_webgl::webgl2::canvas_native_webgl2_get_transform_feedback_varying(
            program,
            index,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_uniform_block_index(
    program: u32,
    uniform_block_name: *const c_char,
    state: *mut WebGLState,
) -> u32 {
    assert!(!state.is_null());
    let uniform_block_name = unsafe { CStr::from_ptr(uniform_block_name) };
    let uniform_block_name = uniform_block_name.to_string_lossy();
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_get_uniform_block_index(
        program,
        uniform_block_name.as_ref(),
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_get_uniform_indices(
    program: u32,
    uniform_names: *const *const c_char,
    size: usize,
    state: *mut WebGLState,
) -> *mut U32Buffer {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    Box::into_raw(Box::new(U32Buffer::from(
        canvas_webgl::webgl2::canvas_native_webgl2_get_uniform_indices_raw(
            program,
            uniform_names,
            size,
            state.get_inner_mut(),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_invalidate_framebuffer(
    target: u32,
    attachments: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let attachments = unsafe { std::slice::from_raw_parts(attachments, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_invalidate_framebuffer(
        target,
        attachments,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_invalidate_sub_framebuffer(
    target: u32,
    attachments: *const u32,
    size: usize,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let attachments = unsafe { std::slice::from_raw_parts(attachments, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_invalidate_sub_framebuffer(
        target,
        attachments,
        x,
        y,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_is_query(query: u32, state: *mut WebGLState) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_is_query(query, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_is_sampler(sampler: u32, state: *mut WebGLState) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_is_sampler(sampler, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_is_sync(
    sync: *const WebGLSync,
    state: *mut WebGLState,
) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    let sync = unsafe { &*sync };
    canvas_webgl::webgl2::canvas_native_webgl2_is_sync(&sync.0, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_is_transform_feedback(
    transform_feedback: u32,
    state: *mut WebGLState,
) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_is_transform_feedback(
        transform_feedback,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_is_vertex_array(
    vertex_array: u32,
    state: *mut WebGLState,
) -> bool {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_is_vertex_array(vertex_array, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_pause_transform_feedback(state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_pause_transform_feedback(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_read_buffer(src: u32, state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_read_buffer(src, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_renderbuffer_storage_multisample(
    target: u32,
    samples: i32,
    internal_format: u32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_renderbuffer_storage_multisample(
        target,
        samples,
        internal_format,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_resume_transform_feedback(state: *mut WebGLState) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_resume_transform_feedback(state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_sampler_parameterf(
    sampler: u32,
    pname: u32,
    param: f32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_sampler_parameterf(
        sampler,
        pname,
        param,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_sampler_parameteri(
    sampler: u32,
    pname: u32,
    param: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_sampler_parameteri(
        sampler,
        pname,
        param,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image3d_none(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    offset: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d_none(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image3d_asset(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    asset: &ImageAsset,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d_asset(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        &asset.0,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image3d(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    buf: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        buf,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_image3d_offset(
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    format: u32,
    type_: u32,
    buf: *const u8,
    size: usize,
    offset: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_image3d_offset(
        target,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format,
        type_,
        buf,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_storage2d(
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_storage2d(
        target,
        levels,
        internalformat,
        width,
        height,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_storage3d(
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    depth: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_storage3d(
        target,
        levels,
        internalformat,
        width,
        height,
        depth,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_sub_image3d_none(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    offset: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d_none(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        type_,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_sub_image3d(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    buf: *const u8,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        type_,
        buf,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_sub_image3d_asset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    asset: &ImageAsset,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d_asset(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        type_,
        &asset.0,
        state.get_inner_mut(),
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_tex_sub_image3d_offset(
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    type_: u32,
    buf: *const u8,
    size: usize,
    offset: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let buf = unsafe { std::slice::from_raw_parts(buf, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_tex_sub_image3d_offset(
        target,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format,
        type_,
        buf,
        offset,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_transform_feedback_varyings(
    program: u32,
    varyings: *const *const c_char,
    size: usize,
    buffer_mode: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_transform_feedback_varyings_raw(
        program,
        varyings,
        size,
        buffer_mode,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform1ui(location: i32, v0: u32, state: *mut WebGLState) {
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform1ui(location, v0, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform1uiv(
    location: i32,
    data: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform1uiv(location, data, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform2ui(
    location: i32,
    v0: u32,
    v1: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform2ui(location, v0, v1, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform2uiv(
    location: i32,
    data: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform2uiv(location, data, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform3ui(
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform3ui(
        location,
        v0,
        v1,
        v2,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform3uiv(
    location: i32,
    data: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform3uiv(location, data, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform4ui(
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
    v3: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform4ui(
        location,
        v0,
        v1,
        v2,
        v3,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform4uiv(
    location: i32,
    data: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform4uiv(location, data, state.get_inner_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_block_binding(
    program: u32,
    uniform_block_index: u32,
    uniform_block_binding: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_block_binding(
        program,
        uniform_block_index,
        uniform_block_binding,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix2x3fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix2x3fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix2x4fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix2x4fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix3x2fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix3x2fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix3x4fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix3x4fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix4x2fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix4x2fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_uniform_matrix4x3fv(
    location: i32,
    transpose: bool,
    data: *const f32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_uniform_matrix4x3fv(
        location,
        transpose,
        data,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_vertex_attrib_divisor(
    index: u32,
    divisor: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_divisor(
        index,
        divisor,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_vertex_attrib_i4i(
    index: u32,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4i(
        index,
        x,
        y,
        z,
        w,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_vertex_attrib_i4iv(
    index: u32,
    value: *const i32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4iv(
        index,
        value,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_vertex_attrib_i4ui(
    index: u32,
    x: u32,
    y: u32,
    z: u32,
    w: u32,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4ui(
        index,
        x,
        y,
        z,
        w,
        state.get_inner_mut(),
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_webgl2_vertex_attrib_i4uiv(
    index: u32,
    value: *const u32,
    size: usize,
    state: *mut WebGLState,
) {
    assert!(!state.is_null());
    let value = unsafe { std::slice::from_raw_parts(value, size) };
    let state = unsafe { &mut *state };
    canvas_webgl::webgl2::canvas_native_webgl2_vertex_attrib_i4uiv(
        index,
        value,
        state.get_inner_mut(),
    )
}

/* WebGL2 */

/* WEBGL2 IMPL */

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_destroy(asset: *mut ImageAsset) {
    if asset.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(asset) };
}

#[no_mangle]
pub extern "C" fn canvas_native_string_destroy(value: *mut c_char) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { CString::from_raw(value) };
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_destroy(value: *mut ImageData) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[no_mangle]
pub extern "C" fn canvas_native_matrix_destroy(value: *mut Matrix) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[no_mangle]
pub extern "C" fn canvas_native_path_destroy(value: *mut Path) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}
