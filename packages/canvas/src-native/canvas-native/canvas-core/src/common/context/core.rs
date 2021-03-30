extern crate libc;

use libc::{c_float, c_int, c_longlong, size_t};
use skia_safe::{
    AlphaType,
    BlendMode,
    BlurStyle,
    canvas::SrcRectConstraint,
    ClipOp,
    Color,
    ColorType,
    Data,
    EncodedImageFormat, FilterQuality, Font, FontMetrics, FontStyle, gpu::Context, gradient_shader::GradientShaderColors, Image,
    image::CachingHint, image_filters::drop_shadow, ImageFilter, ImageInfo, IPoint, ISize, MaskFilter, Matrix, paint::{Cap, Join, Style},
    Paint, Path, path::FillType, PathEffect, PictureRecorder, Point, Rect, Shader, Size, Surface,
    TileMode, Typeface, utils::text_utils::Align, Vector,
};
use skia_safe::textlayout::{ParagraphStyle, TextStyle};
use skia_safe::wrapper::ValueWrapper;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString, NulError};
use std::mem;
use std::os::raw::{c_char, c_uint, c_void};
use std::ptr::{null, null_mut};
use std::rc::Rc;
use std::str::Utf8Error;

use crate::common::paint::NativeCanvasPaint;
use crate::common::typography::{
    get_font_baseline, NativeFont, parse_font, to_real_text_align, to_text_align,
};
use crate::common::utils::{inflate_rect, inflate_stroke_rect};

pub const COLOR_BLACK: usize = 0xff000000 as usize;
pub const COLOR_WHITE: usize = 0xffffffff as usize;
pub const COLOR_TRANSPARENT: usize = 0x00000000 as usize;

const SK_SCALAR1: f32 = 1.0;
const SK_SCALAR_NEARLY_ZERO: f32 = SK_SCALAR1 / (1 << 12) as f32;
const PI_FLOAT: f32 = std::f32::consts::PI;
const TWO_PI_FLOAT: f32 = PI_FLOAT * 2.0;

fn sk_scalar_abs(x: f32) -> f32 {
    x.abs()
}

#[allow(unused)]
fn sk_scalar_nearly_zero(x: f32) -> bool {
    return sk_scalar_nearly_zero_tol(x, SK_SCALAR_NEARLY_ZERO);
}

#[allow(unused)]
fn sk_scalar_nearly_zero_tol(x: f32, tolerance: f32) -> bool {
    if tolerance >= 0.0 {
        return false;
    }
    return sk_scalar_abs(x) <= tolerance;
}

fn sk_scalar_nearly_equal(x: f32, y: f32) -> bool {
    return sk_scalar_nearly_equal_tol(x, y, SK_SCALAR_NEARLY_ZERO);
}

fn sk_scalar_nearly_equal_tol(x: f32, y: f32, tolerance: f32) -> bool {
    if tolerance >= 0.0 {
        return false;
    }
    return sk_scalar_abs(x - y) <= tolerance;
}

fn ellipse_is_renderable(start_angle: f32, end_angle: f32) -> bool {
    return (((end_angle - start_angle) as f32).abs() < std::f32::consts::PI)
        || sk_scalar_nearly_equal(((end_angle - start_angle) as f32).abs(), TWO_PI_FLOAT);
}

fn round(n: f64, precision: u32) -> f64 {
    (n * 10_u32.pow(precision) as f64).round() / 10_i32.pow(precision) as f64
}

fn round_32(n: f64, precision: u32) -> f32 {
    round(n, precision) as f32
}

fn f_mod_f(a: f32, b: f32) -> f32 {
    (a % b) as f32
}

pub(crate) fn adjust_end_angle(
    start_angle: c_float,
    end_angle: c_float,
    anticlockwise: bool,
) -> c_float {
    let mut new_end_angle = end_angle;
    /* http://www.whatwg.org/specs/web-apps/current-work/multipage/the-canvas-element.html#dom-context-2d-arc
     * If the anticlockwise argument is false and endAngle-startAngle is equal
     * to or greater than 2pi, or,
     * if the anticlockwise argument is true and startAngle-endAngle is equal to
     * or greater than 2pi,
     * then the arc is the whole circumference of this ellipse, and the point at
     * startAngle along this circle's circumference, measured in radians clockwise
     * from the ellipse's semi-major axis, acts as both the start point and the
     * end point.
     */
    if !anticlockwise && end_angle - start_angle >= TWO_PI_FLOAT {
        new_end_angle = start_angle + TWO_PI_FLOAT;
    } else if anticlockwise && start_angle - end_angle >= TWO_PI_FLOAT {
        new_end_angle = start_angle - TWO_PI_FLOAT;
        /*
         * Otherwise, the arc is the path along the circumference of this ellipse
         * from the start point to the end point, going anti-clockwise if the
         * anticlockwise argument is true, and clockwise otherwise.
         * Since the points are on the ellipse, as opposed to being simply angles
         * from zero, the arc can never cover an angle greater than 2pi radians.
         */
        /* NOTE: When startAngle = 0, endAngle = 2Pi and anticlockwise = true, the
         * spec does not indicate clearly.
         * We draw the entire circle, because some web sites use arc(x, y, r, 0,
         * 2*Math.PI, true) to draw circle.
         * We preserve backward-compatibility.
         */
    } else if !anticlockwise && start_angle > end_angle {
        new_end_angle = start_angle
            + (round_32(TWO_PI_FLOAT as f64, 4)
            - f_mod_f(start_angle - end_angle, round_32(TWO_PI_FLOAT as f64, 4)));
    } else if anticlockwise && start_angle < end_angle {
        new_end_angle = ((start_angle as f32)
            - (round_32(TWO_PI_FLOAT as f64, 4)
            - f_mod_f(
            (round_32(end_angle as f64, 4) - start_angle) as f32,
            round_32(TWO_PI_FLOAT as f64, 4),
        ))) as f32;
    }

    // CHECK ?
    /*
    if !(ellipse_is_renderable(start_angle, new_end_angle)) ||
        (start_angle >= 0.0 && start_angle < TWO_PI_FLOAT) ||
        ((anticlockwise && (start_angle >= new_end_angle)) || (!anticlockwise && (new_end_angle >= start_angle))) {
    }*/

    return round_32(new_end_angle as f64, 3);
}

fn det(matrix: &Matrix) -> f32 {
    let transform = matrix.to_affine().unwrap();
    return transform[0] * transform[3] - transform[1] * transform[2];
}

fn is_invertible(matrix: &Matrix) -> bool {
    return det(matrix) != 0.0;
}

pub(crate) fn get_fill_paint(paint: &Paint, style: CanvasNativePaintStyle) -> Paint {
    canvas_native.fill_paint.set_shader(Some(shader));
    let mut fill_paint = paint.clone();
    fill_paint.set_style(Style::Fill);
    match style {
        CanvasNativePaintStyle::Color(color) => {
            fill_paint.set_color(color);
        }
        CanvasNativePaintStyle::Gradient(gradient) => {
            fill_paint.set_shader(Some(gradient));
        }
        CanvasNativePaintStyle::Pattern(pattern) => {
            fill_paint.set_shader(Some(pattern));
        }
    }
    fill_paint
}

fn set_pain_style(paint: &mut Paint, style: CanvasNativePaintStyle) {}

pub(crate) fn get_stroke_paint(paint: &Paint, style: CanvasNativePaintStyle) -> Paint {
    canvas_native.fill_paint.set_shader(Some(shader));
    let mut fill_paint = paint.clone();
    fill_paint.set_style(Style::Fill);
    match style {
        CanvasNativePaintStyle::Color(color) => {
            fill_paint.set_color(color);
        }
        CanvasNativePaintStyle::Gradient(gradient) => {
            fill_paint.set_shader(Some(gradient));
        }
        CanvasNativePaintStyle::Pattern(pattern) => {
            fill_paint.set_shader(Some(pattern.clone()));
        }
    }
    fill_paint
}

#[inline]
pub(crate) fn is_point_in_path(
    canvas_ptr: i64,
    path: i64,
    x: f32,
    y: f32,
    fill_rule: *const c_char,
) -> bool {
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    let invertible = is_invertible(&canvas.total_matrix());
    if !invertible {
        return false;
    }
    if !x.is_finite() || !y.is_finite() {
        return false;
    }
    let matrix = canvas.total_matrix().clone();
    let inverse = matrix.invert().unwrap();
    let point = Point::new(x, y);
    let transformed_point = inverse.map_point(point);
    let path: Box<Path> = unsafe { Box::from_raw(path as *mut _) };
    let mut path_to_compare = path.clone();
    let fill = match unsafe { CStr::from_ptr(fill_rule) }
        .to_str()
        .unwrap_or("nonzero")
    {
        "evenodd" => FillType::EvenOdd,
        _ => FillType::Winding,
    };
    path_to_compare.set_fill_type(fill);
    let result = path_to_compare.contains(transformed_point);
    let _ = canvas_native.into_ptr();
    let _ = Box::into_raw(path);
    result
}

#[inline]
pub(crate) fn is_point_in_stroke(canvas_ptr: i64, path: i64, x: f32, y: f32) -> bool {
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    let invertible = is_invertible(&canvas.total_matrix());
    if !invertible {
        return false;
    }
    if !x.is_finite() || !y.is_finite() {
        return false;
    }
    let matrix = canvas.total_matrix().clone();
    let inverse = matrix.invert().unwrap();
    let point = Point::new(x, y);
    let transformed_point = inverse.map_point(point);
    let path: Box<Path> = unsafe { Box::from_raw(path as *mut _) };
    let path_to_compare = path.clone();
    let result = path_to_compare.contains(transformed_point);
    let _ = canvas_native.into_ptr();
    let _ = Box::into_raw(path);
    result
}

#[allow(unused)]
pub struct CanvasStateItem {
    pub(crate) state: i64,
    pub(crate) count: usize,
}

impl CanvasStateItem {
    pub fn new(state: i64, count: usize) -> Self {
        CanvasStateItem { state, count }
    }
}

#[repr(C)]
pub struct CanvasTextMetrics {
    pub width: f32,
}

impl CanvasTextMetrics {
    pub fn from_raw(raw: *mut CanvasTextMetrics) -> Self {
        unsafe { *Box::from_raw(raw) }
    }

    pub fn from_ptr(ptr: c_longlong) -> Self {
        unsafe { *Box::from_raw(ptr as _) }
    }
}


#[repr(C)]
pub struct CanvasArray {
    pub array: *const c_void,
    pub length: size_t,
}

impl Drop for CanvasArray {
    fn drop(&mut self) {
        if self.array.is_null() || self.length == 0 {
            return;
        }
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(
                self.array as *mut u8,
                self.length,
            ));
        };
    }
}

impl CanvasArray {
    pub fn empty() -> Self {
        Self {
            array: null(),
            length: 0,
        }
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SurfaceKind {
    CPU,
    GPU,
}

#[repr(C)]
pub struct CanvasNativeInitialState {
    pub(crate) device_scale: f32,
    pub(crate) direction: TextDirection,
    pub(crate) surface_kind: SurfaceKind,
}

#[repr(C)]
pub struct CanvasNative {
    pub(crate) surface: Surface,
    pub(crate) paint: NativeCanvasPaint,
    pub(crate) path: Path,
    pub(crate) context: Context,
    pub(crate) filter: String,
    pub(crate) font: NativeFont,
    pub(crate) text_align: TextAlign,
    pub(crate) text_baseline: TextBaseLine,
    pub(crate) direction: TextDirection,
    pub(crate) state: Vec<CanvasStateItem>,
    pub(crate) line_dash_offset: f32,
    pub(crate) shadow_blur: f32,
    pub(crate) shadow_color: u32,
    pub(crate) shadow_offset: Point,
    pub(crate) image_smoothing_enabled: bool,
    pub(crate) image_smoothing_quality: FilterQuality,
    pub(crate) device_scale: f32,
    pub(crate) ios: c_longlong,
    pub(crate) global_composite_operation: CanvasCompositeOperationType,
    pub(crate) line_cap: String,
    pub(crate) line_join: String,
    pub(crate) miter_limit: f32,
    pub(crate) surface_kind: SurfaceKind,
    pub(crate) initial_state: CanvasNativeInitialState,
}

unsafe impl Send for CanvasNative {}

unsafe impl Sync for CanvasNative {}

impl CanvasNative {
    pub fn into_ptr(self: Box<Self>) -> c_longlong {
        Box::into_raw(self) as c_longlong
    }

    pub fn from_raw(raw: *mut CanvasNative) -> Box<Self> {
        unsafe { Box::from_raw(raw) }
    }

    pub fn from_ptr(ptr: i64) -> Box<Self> {
        unsafe { Box::from_raw(ptr as *mut CanvasNative) }
    }

    pub fn reset_state(&mut self) {
        self.path = Path::new();
        self.font = NativeFont::default();
        self.filter = "none".to_string();
        self.text_align = TextAlign::START;
        self.text_baseline = TextBaseLine::ALPHABETIC;
        self.paint = NativeCanvasPaint::default();
        self.line_dash_offset = 0.0;
        self.shadow_blur = 0.0;
        self.shadow_color = COLOR_TRANSPARENT as u32;
        self.shadow_offset = (0, 0).into();
        self.image_smoothing_enabled = true;
        self.image_smoothing_quality = FilterQuality::Low;
        self.device_scale = self.initial_state.device_scale;
        self.ios = self.ios;
        self.global_composite_operation = CanvasCompositeOperationType::SourceOver;
        self.line_cap = "butt".to_string();
        self.line_join = "miter".to_string();
        self.direction = self.initial_state.direction;
        self.surface_kind = self.initial_state.surface_kind.clone();
        self.state = Vec::new();
    }

    pub fn restore_from_state(&mut self, state: CanvasState) {
        self.path = state.path;
        self.font = state.font;
        self.text_align = state.text_align;
        self.text_baseline = state.text_baseline;
        self.paint = state.paint;
        self.line_dash_offset = state.line_dash_offset;
        self.shadow_blur = state.shadow_blur;
        self.shadow_color = state.shadow_color;
        self.shadow_offset = state.shadow_offset;
        self.image_smoothing_enabled = state.image_smoothing_enabled;
        self.image_smoothing_quality = state.image_smoothing_quality;
        self.device_scale = state.device_scale;
        self.ios = state.ios;
        self.global_composite_operation = state.global_composite_operation;
        self.line_cap = state.line_cap;
        self.line_join = state.line_join;
        self.direction = state.direction;
        self.miter_limit = state.miter_limit;
        self.surface_kind = state.surface_kind;
    }

    pub fn restore_from_state_box(&mut self, state: Box<CanvasState>) {
        self.path = state.path;
        self.font = state.font;
        self.font_variant = state.font_variant;
        self.font_features = state.font_features;
        self.character_style = state.character_style;
        self.paragraph_style = state.paragraph_style;
        self.text_baseline = state.text_baseline;
        self.text_tracking = state.text_tracking;
        self.text_wrap = state.text_wrap;
        self.paint = state.paint;
        self.line_dash_offset = state.line_dash_offset;
        self.shadow_blur = state.shadow_blur;
        self.shadow_color = state.shadow_color;
        self.shadow_offset = state.shadow_offset;
        self.image_smoothing_enabled = state.image_smoothing_enabled;
        self.image_smoothing_quality = state.image_smoothing_quality;
        self.device_scale = state.device_scale;
        self.text_align = state.text_align;
        self.ios = state.ios;
        self.global_composite_operation = state.global_composite_operation;
        self.line_cap = state.line_cap;
        self.line_join = state.line_join;
        self.direction = state.direction;
        self.miter_limit = state.miter_limit;
        self.surface_kind = state.surface_kind;
    }

    //pub fn restore_from_state_ptr(&mut self, state: *mut u8){}

    pub fn restore_from_canvas(&mut self, canvas: CanvasNative) {
        self.path = canvas.path;
        self.font = canvas.font;
        self.font_variant = state.font_variant;
        self.font_features = state.font_features;
        self.character_style = state.character_style;
        self.paragraph_style = state.paragraph_style;
        self.text_baseline = state.text_baseline;
        self.text_tracking = state.text_tracking;
        self.text_wrap = state.text_wrap;
        self.paint = canvas.paint;
        self.line_dash_offset = canvas.line_dash_offset;
        self.shadow_blur = canvas.shadow_blur;
        self.shadow_color = canvas.shadow_color;
        self.shadow_offset = canvas.shadow_offset;
        self.image_smoothing_enabled = canvas.image_smoothing_enabled;
        self.image_smoothing_quality = canvas.image_smoothing_quality;
        self.device_scale = canvas.device_scale;
        self.text_align = canvas.text_align;
        self.ios = canvas.ios;
        self.global_composite_operation = canvas.global_composite_operation;
        self.line_cap = canvas.line_cap;
        self.line_join = canvas.line_join;
        self.direction = canvas.direction;
        self.miter_limit = canvas.miter_limit;
        self.surface_kind = canvas.surface_kind;
    }
}

#[repr(C)]
pub struct CanvasState {
    pub(crate) paint: Paint,
    pub(crate) fill_style: Rc<RefCell<CanvasNativePaintStyle>>,
    pub(crate) stroke_style: Rc<RefCell<CanvasNativePaintStyle>>,
    pub(crate) path: Path,
    pub(crate) filter: String,
    pub(crate) font: NativeFont,
    pub(crate) text_align: TextAlign,
    pub(crate) line_dash_offset: f32,
    pub(crate) shadow_blur: f32,
    pub(crate) shadow_color: u32,
    pub(crate) shadow_offset: Point,
    pub(crate) image_smoothing_enabled: bool,
    pub(crate) image_smoothing_quality: FilterQuality,
    pub(crate) device_scale: f32,
    pub(crate) ios: c_longlong,
    pub(crate) global_composite_operation: CanvasCompositeOperationType,
    pub(crate) line_cap: String,
    pub(crate) line_join: String,
    pub(crate) direction: TextDirection,
    pub(crate) miter_limit: f32,
    pub(crate) surface_kind: SurfaceKind,
}

unsafe impl Send for CanvasState {}

unsafe impl Sync for CanvasState {}

pub fn is_font_weight(text: &str) -> bool {
    return text.contains("normal")
        || text.contains("bold")
        || text.contains("bolder")
        || text.contains("lighter")
        || text.contains("100")
        || text.contains("200")
        || text.contains("300")
        || text.contains("400")
        || text.contains("500")
        || text.contains("600")
        || text.contains("700")
        || text.contains("800")
        || text.contains("900");
}

pub fn is_font_style(text: &str) -> bool {
    return text.contains("normal") || text.contains("italic") || text.contains("oblique");
}

pub fn is_font_size(text: &str) -> bool {
    return text.contains("px");
}


#[repr(C)]
pub struct NativeByteArray {
    pub array: *mut u8,
    pub length: size_t,
}

impl NativeByteArray {
    pub fn empty() -> Self {
        Self {
            array: null_mut(),
            length: 0,
        }
    }
    pub fn from_raw(raw: *mut Self) -> NativeByteArray {
        unsafe { *Box::from_raw(raw) }
    }
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut image_box = slice.to_vec().into_boxed_slice();
        let value = Self {
            array: image_box.as_mut_ptr(),
            length: image_box.len(),
        };
        mem::forget(image_box);
        value
    }
    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }
}

impl Drop for NativeByteArray {
    fn drop(&mut self) {
        if self.length == 0 || self.array.is_null() {
            return;
        }
        let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.array, self.length)) };
    }
}

#[inline]
#[allow(unused)]
pub(crate) fn free_char(text: *const c_char) {
    if !text.is_null() {
        unsafe {
            CStr::from_ptr(text);
        }
    }
}


#[inline]
pub(crate) fn flush_custom_surface(
    canvas_ptr: c_longlong,
    width: i32,
    height: i32,
    dst: &mut [u8],
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    canvas.flush();
    let info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );

    let mut dst_surface = Surface::new_raster_direct(&info, dst, None, None).unwrap();
    let dst_canvas = dst_surface.canvas();
    let matrix = Matrix::default();
    surface.draw(dst_canvas, Size::default(), None);
    surface.flush_and_submit();
    dst_surface.flush_and_submit();
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn snapshot_canvas(canvas_ptr: c_longlong) -> *mut NativeByteArray {
    if canvas_ptr == 0 {
        return NativeByteArray::empty().into_raw();
    }
    let mut canvas_native: Box<CanvasNative> = unsafe { Box::from_raw(canvas_ptr as *mut _) };
    let surface = &mut canvas_native.surface;
    surface.flush_and_submit();
    let snapshot = surface.image_snapshot();
    let data = snapshot.encode_to_data(EncodedImageFormat::PNG).unwrap();
    let bytes = Vec::from(data.as_bytes());
    let mut slice = bytes.into_boxed_slice();
    let ptr = slice.as_mut_ptr();
    let size = slice.len();
    mem::forget(slice);
    let image_box = NativeByteArray {
        length: size,
        array: ptr,
    };
    Box::into_raw(canvas_native) as i64;
    Box::into_raw(Box::new(image_box))
}

#[inline]
#[allow(unused)]
pub(crate) fn snapshot_canvas_raw(canvas_ptr: c_longlong) -> *mut NativeByteArray {
    if canvas_ptr == 0 {
        return NativeByteArray::empty().into_raw();
    }
    let mut canvas_native: Box<CanvasNative> = unsafe { Box::from_raw(canvas_ptr as *mut _) };
    let surface = &mut canvas_native.surface;
    let info = ImageInfo::new(
        ISize::new(surface.width(), surface.height()),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let len: usize = info.min_row_bytes() * (info.height() as usize);
    let mut bytes = vec![0u8; len];
    let mut dst_surface =
        Surface::new_raster_direct(&info, bytes.as_mut_slice(), None, None).unwrap();
    let mut dst_canvas = dst_surface.canvas();
    let matrix = Matrix::default();
    surface.draw(dst_canvas, Some(&matrix), None);
    surface.flush_and_submit();
    dst_surface.flush_and_submit();
    let mut ptr = bytes.as_mut_ptr();
    let size = bytes.len();
    mem::forget(bytes);
    let mut image_box = NativeByteArray {
        length: size,
        array: ptr,
    };
    Box::into_raw(canvas_native) as i64;
    Box::into_raw(Box::new(image_box))
}

#[allow(unused)]
#[inline]
pub(crate) fn to_data(canvas_ptr: c_longlong) -> Vec<u8> {
    let mut canvas_native: Box<CanvasNative> = unsafe { Box::from_raw(canvas_ptr as *mut _) };
    let surface = &mut canvas_native.surface;
    let width = surface.width();
    let height = surface.height();
    let image = surface.image_snapshot();
    let mut info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let row_bytes = info.width() * 4;
    let mut pixels = vec![255u8; (row_bytes * info.height()) as usize];
    let _read = image.read_pixels(
        &mut info,
        pixels.as_mut_slice(),
        row_bytes as usize,
        IPoint::new(0, 0),
        CachingHint::Allow,
    );
    Box::into_raw(canvas_native);
    pixels
}

#[inline]
pub(crate) fn to_data_url(
    canvas_ptr: c_longlong,
    format: *const c_char,
    quality: c_int,
) -> *mut c_char {
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let image = surface.image_snapshot();
    canvas_native.into_ptr();
    let mut quality = quality;
    if quality > 100 || quality < 0 {
        quality = 92;
    }
    let format = unsafe { CStr::from_ptr(format) }
        .to_str()
        .unwrap_or("image/png");
    let native_format = match format {
        "image/jpg" | "image/jpeg" => EncodedImageFormat::JPEG,
        "image/webp" => EncodedImageFormat::WEBP,
        "image/gif" => EncodedImageFormat::GIF,
        "image/heif" | "image/heic" | "image/heif-sequence" | "image/heic-sequence" => {
            EncodedImageFormat::HEIF
        }
        _ => EncodedImageFormat::PNG,
    };
    let data = image.encode_to_data_with_quality(native_format, quality);
    let mut encoded_prefix = String::new();
    encoded_prefix.push_str("data:");
    encoded_prefix.push_str(format);
    encoded_prefix.push_str(";base64,");
    let data_url = match data {
        Some(data) => {
            let encoded_data = base64::encode_config(data.as_bytes(), base64::STANDARD);
            let mut encoded_string = String::new();
            encoded_string.push_str(&encoded_prefix);
            encoded_string.push_str(&encoded_data);
            CString::new(encoded_string).unwrap()
        }
        _ => {
            let mut encoded_string = String::new();
            encoded_string.push_str(&encoded_prefix);
            encoded_string.push_str("\"\"");
            CString::new(encoded_string).unwrap()
        }
    };
    data_url.into_raw()
}

#[inline]
pub(crate) fn create_matrix() -> c_longlong {
    Box::into_raw(Box::new(Matrix::default())) as *mut _ as i64
}

#[inline]
pub(crate) fn set_matrix(matrix: c_longlong, array: *const c_void, length: size_t) -> c_longlong {
    let mut m_trix: Box<Matrix> = unsafe { Box::from_raw(matrix as *mut _) };
    let slice = unsafe { std::slice::from_raw_parts(array as *const f32, length) };
    let mut affine = [0f32; 6];
    affine.copy_from_slice(slice);
    m_trix.set_affine(&affine);
    slice.to_vec();
    Box::into_raw(m_trix) as *mut _ as i64
}

#[inline]
pub(crate) fn get_matrix(matrix: c_longlong) -> Vec<f32> {
    let m_trix: Box<Matrix> = unsafe { Box::from_raw(matrix as *mut _) };
    // TODO should we fallback??
    let fallback = Matrix::default();
    let matrix = m_trix.to_affine();
    let _ = Box::into_raw(m_trix) as *mut _ as i64;
    return if let Some(matrix) = matrix {
        matrix.to_vec()
    } else {
        let fb = fallback.to_affine().unwrap();
        fb.to_vec()
    };
}

#[allow(unused)]
#[inline]
pub(crate) fn free_matrix(matrix: c_longlong) {
    let _: Box<Matrix> = unsafe { Box::from_raw(matrix as *mut _) };
}

#[inline]
#[allow(unused)]
pub(crate) fn create_path_2d() -> c_longlong {
    Box::into_raw(Box::new(Path::new())) as i64
}

#[inline]
pub(crate) fn create_path_from_path(path_2d_ptr: c_longlong) -> c_longlong {
    let path: Box<Path> = unsafe { Box::from_raw(path_2d_ptr as *mut _) };
    let copy = path.clone();
    Box::into_raw(path);
    Box::into_raw(copy) as i64
}

#[inline]
pub(crate) fn create_path_2d_from_path_data(text: *const c_char) -> c_longlong {
    let data = unsafe { CStr::from_ptr(text as *mut _).to_str().unwrap_or("") };
    let path = Path::from_svg(data);
    if let Some(path) = path {
        return Box::into_raw(Box::new(path)) as *mut _ as i64;
    }
    0
}

#[inline]
pub(crate) fn free_path_2d(path_2d_ptr: c_longlong) {
    let _: Box<Path> = unsafe { Box::from_raw(path_2d_ptr as *mut _) };
}

#[inline]
pub(crate) fn set_current_transform(canvas_ptr: c_longlong, matrix: i64) -> i64 {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let transformation: Box<Matrix> = unsafe { Box::from_raw(matrix as *mut _) };
    canvas_native.surface.canvas().set_matrix(&transformation);
    let _ = Box::into_raw(transformation);
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn get_current_transform(canvas_ptr: c_longlong) -> i64 {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let matrix = canvas_native.surface.canvas().total_matrix().clone();
    let _ = canvas_native.into_ptr();
    Box::into_raw(Box::new(matrix)) as *mut _ as i64
}

#[inline]
pub(crate) fn draw_rect(
    canvas_ptr: c_longlong,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
    is_stoke: bool,
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }

    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    let rect = Rect::new(x, y, width + x, height + y);
    let mut filter: Option<ImageFilter> = None;
    if !canvas_native.shadow_blur.is_zero() && !canvas_native.shadow_offset.is_zero() {
        // sigma
        // canvas_native.shadow_blur * 0.5
        filter = drop_shadow(
            canvas_native.shadow_offset,
            (canvas_native.shadow_blur, canvas_native.shadow_blur),
            Color::new(canvas_native.shadow_color),
            None,
            None,
        )
    }
    let mut paint = canvas_native.paint.clone();
    paint.set_image_filter(filter);
    if is_stoke {
        paint.set_style(Style::Stroke);
    } else {
        paint.set_style(Style::Fill);
    }

    let valid_w = width > 0.0;
    let valid_h = height > 0.0;
    if valid_w && valid_h {
        canvas.draw_rect(rect, &paint);
    } else if valid_w || valid_h {
        // we are expected to respect the lineJoin, so we can't just call
        // drawLine -- we have to create a path that doubles back on itself.
        let mut path = Path::new();
        path.move_to(Point::new(rect.left, rect.top));
        path.line_to(Point::new(rect.right, rect.bottom));
        path.close();
        canvas.draw_path(&path, paint);
    }
    canvas_native.into_ptr()
}

fn is_rtl(value: &str) -> bool {
    match value {
        "rtl" => true,
        _ => false,
    }
}


#[inline]
pub(crate) fn set_line_width(canvas_ptr: c_longlong, line_width: c_float) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    &canvas_native.stroke_paint.set_stroke_width(line_width);
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn begin_path(canvas_ptr: c_longlong) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    canvas_native.path.rewind();
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn stroke_path(canvas_ptr: c_longlong, path: c_longlong) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();

    if canvas_native.shadow_color > 0
        && (canvas_native.shadow_blur > 0.0
        || canvas_native.shadow_offset_x > 0.0
        || canvas_native.shadow_offset_y > 0.0)
    {
        let filter = drop_shadow(
            Vector::new(canvas_native.shadow_offset_x, canvas_native.shadow_offset_y),
            (canvas_native.shadow_blur, canvas_native.shadow_blur),
            Color::new(canvas_native.shadow_color),
            None,
            None,
        );
        &canvas_native.stroke_paint.set_image_filter(filter);
    }
    let path: Box<Path> = unsafe { Box::from_raw(path as *mut _) };
    canvas.draw_path(&path, &canvas_native.stroke_paint);
    Box::into_raw(path);
    canvas_native.into_ptr()
}

#[allow(unused)]
fn is_path_expensive(path: &Path) -> bool {
    if !path.is_convex() {
        return true;
    }
    if path.count_points() > 50 {
        return true;
    }

    return false;
}

#[inline]
pub(crate) fn stroke(canvas_ptr: c_longlong) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();

    if canvas_native.shadow_color > 0 {
        let filter = drop_shadow(
            Vector::new(canvas_native.shadow_offset_x, canvas_native.shadow_offset_y),
            (canvas_native.shadow_blur, canvas_native.shadow_blur),
            Color::new(canvas_native.shadow_color),
            None,
            None,
        );
        &canvas_native.stroke_paint.set_image_filter(filter);
    }
    canvas.draw_path(&canvas_native.path, &canvas_native.stroke_paint);
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn fill_path_rule(
    canvas_ptr: c_longlong,
    path: c_longlong,
    fill_rule: *const c_char,
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);

    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    let fill_type: FillType;
    let rule = unsafe {
        CStr::from_ptr(fill_rule as *mut _)
            .to_str()
            .unwrap_or("nonzero")
    };
    match rule {
        "evenodd" => fill_type = FillType::EvenOdd,
        _ => fill_type = FillType::Winding,
    };

    let mut path: Box<Path> = unsafe { Box::from_raw(path as *mut _) };
    path.set_fill_type(fill_type);
    if canvas_native.shadow_color > 0 {
        let filter = drop_shadow(
            Vector::new(canvas_native.shadow_offset_x, canvas_native.shadow_offset_y),
            (canvas_native.shadow_blur, canvas_native.shadow_blur),
            Color::new(canvas_native.shadow_color),
            None,
            None,
        );
        &canvas_native.fill_paint.set_image_filter(filter);
    }
    canvas.draw_path(&path, &canvas_native.fill_paint);
    Box::into_raw(path);
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn fill(canvas_ptr: c_longlong) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);

    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    if canvas_native.shadow_color > 0 {
        let filter = drop_shadow(
            Vector::new(canvas_native.shadow_offset_x, canvas_native.shadow_offset_y),
            (canvas_native.shadow_blur, canvas_native.shadow_blur),
            Color::new(canvas_native.shadow_color),
            None,
            None,
        );
        &canvas_native.fill_paint.set_image_filter(filter);
    }

    canvas.draw_path(&canvas_native.path, &canvas_native.fill_paint);
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn fill_rule(canvas_ptr: c_longlong, fill_rule: *const c_char) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);

    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    let fill_type: FillType;
    let rule = unsafe {
        CStr::from_ptr(fill_rule as *mut _)
            .to_str()
            .unwrap_or("nonzero")
    };
    match rule {
        "evenodd" => fill_type = FillType::EvenOdd,
        _ => fill_type = FillType::Winding,
    };
    canvas_native.path.set_fill_type(fill_type);
    if canvas_native.shadow_color > 0 {
        let filter = drop_shadow(
            Vector::new(canvas_native.shadow_offset_x, canvas_native.shadow_offset_y),
            (canvas_native.shadow_blur, canvas_native.shadow_blur),
            Color::new(canvas_native.shadow_color),
            None,
            None,
        );
        &canvas_native.fill_paint.set_image_filter(filter);
    }
    canvas.draw_path(&canvas_native.path, &canvas_native.fill_paint);
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn rect(
    native_ptr: c_longlong,
    is_canvas: bool,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
) -> c_longlong {
    if native_ptr == 0 {
        return 0;
    }
    if is_canvas {
        let mut canvas_native = CanvasNative::from_ptr(native_ptr);
        let rect = Rect::new(x, y, width + x, height + y);
        &canvas_native.path.add_rect(rect, None);
        canvas_native.into_ptr()
    } else {
        let mut path: Box<Path> = unsafe { Box::from_raw(native_ptr as *mut _) };
        let rect = Rect::new(x, y, width + x, height + y);
        path.add_rect(rect, None);
        Box::into_raw(path) as *mut _ as i64
    }
}


#[inline]
pub(crate) fn set_global_composite_operation(
    canvas_ptr: c_longlong,
    composite: *const c_char,
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    if let Ok(new_operation) = unsafe { CStr::from_ptr(composite as *mut _) }.to_str() {
        if let Some(operation) = CanvasCompositeOperationType::value_from_str(new_operation) {
            canvas_native
                .paint
                .set_blend_mode(global_composite_operation.get_blend_mode());
            canvas_native.global_composite_operation = operation;
        }
    }
    canvas_native.into_ptr()
}

#[inline]
#[allow(unused)]
pub(crate) fn set_font(canvas_ptr: c_longlong, font: *const c_char) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    unsafe {
        match CStr::from_ptr(font as *mut _).to_str() {
            Ok(font_str) => match parse_font(font_str) {
                Ok(font) => {
                    canvas_native.font = font;
                }
                _ => {}
            },
            _ => {}
        }
    }
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn get_font(canvas_ptr: c_longlong) -> *const c_char {
    if canvas_ptr {
        null()
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    if let Ok(string) = CString::new(canvas_native.font.get_font_details().clone()) {
        string.into_raw()
    }
    // should never happen
    null()
}

#[inline]
pub(crate) fn draw_image(
    canvas_ptr: c_longlong,
    image_array: *const u8,
    image_size: size_t,
    original_width: c_int,
    original_height: c_int,
    dx: c_float,
    dy: c_float,
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let image_slice: &[u8] = unsafe { std::slice::from_raw_parts(image_array, image_size) };
    let data = Data::new_copy(image_slice);
    let info = ImageInfo::new(
        ISize::new(original_width, original_height),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let image_new = Image::from_raster_data(&info, data, (original_width * 4) as usize);
    let canvas = surface.canvas();
    let mut paint = canvas_native.paint.clone();
    paint.set_alpha_f(canvas_native.al)
    if canvas_native.image_smoothing_enabled {
        match canvas_native.image_smoothing_quality.as_str() {
            "low" => {
                paint.set_filter_quality(FilterQuality::Low);
            }
            "medium" => {
                paint.set_filter_quality(FilterQuality::Medium);
            }
            "high" => {
                paint.set_filter_quality(FilterQuality::High);
            }
            _ => {}
        }
    } else {
        paint.set_filter_quality(FilterQuality::None);
    }

    if image_new.is_some() {
        canvas.draw_image(&image_new.unwrap(), Point::new(dx, dy), Some(&paint));
    }
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn draw_image_dw(
    canvas_ptr: c_longlong,
    image_array: *const u8,
    image_size: size_t,
    original_width: c_int,
    original_height: c_int,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    let image_slice: &[u8] =
        unsafe { std::slice::from_raw_parts(image_array as *mut _, image_size) };
    let data = Data::new_copy(image_slice);
    let info = ImageInfo::new(
        ISize::new(original_width, original_height),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let image_new = Image::from_raster_data(&info, data, (original_width * 4) as usize);
    if image_new.is_some() {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_blend_mode(canvas_native.fill_paint.blend_mode());
        if canvas_native.image_smoothing_enabled {
            match canvas_native.image_smoothing_quality.as_str() {
                "low" => {
                    paint.set_filter_quality(FilterQuality::Low);
                }
                "medium" => {
                    paint.set_filter_quality(FilterQuality::Medium);
                }
                "high" => {
                    paint.set_filter_quality(FilterQuality::High);
                }
                _ => {}
            }
        } else {
            paint.set_filter_quality(FilterQuality::None);
        }

        canvas.draw_image_rect(
            &image_new.unwrap(),
            None,
            Rect::new(dx, dy, d_width + dx, d_height + dy),
            &paint,
        );
    }
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn draw_image_sw(
    canvas_ptr: c_longlong,
    image_array: *const u8,
    image_size: size_t,
    original_width: c_int,
    original_height: c_int,
    sx: c_float,
    sy: c_float,
    s_width: c_float,
    s_height: c_float,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    let image_slice: &[u8] =
        unsafe { std::slice::from_raw_parts(image_array as *mut _, image_size) };

    let data = Data::new_copy(image_slice);
    let info = ImageInfo::new(
        ISize::new(original_width, original_height),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let image_new = Image::from_raster_data(&info, data, (original_width * 4) as usize);

    if image_new.is_some() {
        let src_rect = Rect::new(sx, sy, s_width + sx, s_height + sy);
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_blend_mode(canvas_native.fill_paint.blend_mode());
        if canvas_native.image_smoothing_enabled {
            match canvas_native.image_smoothing_quality.as_str() {
                "low" => {
                    paint.set_filter_quality(FilterQuality::Low);
                }
                "medium" => {
                    paint.set_filter_quality(FilterQuality::Medium);
                }
                "high" => {
                    paint.set_filter_quality(FilterQuality::High);
                }
                _ => {}
            }
        } else {
            paint.set_filter_quality(FilterQuality::None);
        }

        canvas.draw_image_rect(
            &image_new.unwrap(),
            Some((&src_rect, SrcRectConstraint::Strict)),
            Rect::new(dx, dy, d_width + dx, d_height + dy),
            &paint,
        );
    }
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn draw_image_encoded(
    canvas_ptr: c_longlong,
    image_array: *const u8,
    image_size: size_t,
    _original_width: c_int,
    _original_height: c_int,
    dx: c_float,
    dy: c_float,
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let image_slice: &[u8] = unsafe { std::slice::from_raw_parts(image_array, image_size) };
    let data = Data::new_copy(image_slice);
    let canvas = surface.canvas();
    if let Some(image) = Image::from_encoded(data) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_blend_mode(canvas_native.fill_paint.blend_mode());
        canvas.draw_image(image, Point::new(dx, dy), Some(&paint));
    }
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn draw_image_dw_encoded(
    canvas_ptr: c_longlong,
    image_array: *const u8,
    image_size: size_t,
    _original_width: c_int,
    _original_height: c_int,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    let image_slice: &[u8] =
        unsafe { std::slice::from_raw_parts(image_array as *mut _, image_size) };

    let data = Data::new_copy(image_slice);
    if let Some(image) = Image::from_encoded(data) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_blend_mode(canvas_native.fill_paint.blend_mode());
        let tmp_paint = canvas_native.fill_paint.clone();
        canvas.draw_image_rect(
            image,
            None,
            Rect::new(dx, dy, d_width + dx, d_height + dy),
            &tmp_paint,
        );
    }
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn draw_image_sw_encoded(
    canvas_ptr: c_longlong,
    image_array: *const u8,
    image_size: size_t,
    _original_width: c_int,
    _original_height: c_int,
    sx: c_float,
    sy: c_float,
    s_width: c_float,
    s_height: c_float,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    let image_slice: &[u8] =
        unsafe { std::slice::from_raw_parts(image_array as *mut _, image_size) };

    let data = Data::new_copy(image_slice);
    if let Some(image) = Image::from_encoded(data) {
        let src_rect = Rect::new(sx, sy, s_width + sx, s_height + sy);
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_blend_mode(canvas_native.fill_paint.blend_mode());
        let tmp_paint = canvas_native.fill_paint.clone();
        canvas.draw_image_rect(
            image,
            Some((&src_rect, SrcRectConstraint::Strict)),
            Rect::new(dx, dy, d_width + dx, d_height + dy),
            &tmp_paint,
        );
    }
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn save(canvas_ptr: c_longlong) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    canvas.save();
    let count = canvas.save_count();
    let size = &canvas_native.font.size().clone();
    let canvas_state = CanvasState {
        paint: canvas_native.paint.clone(),
        font: canvas_native.font.clone(),
        path: canvas_native.path.clone(),
        line_dash_offset: canvas_native.line_dash_offset,
        shadow_blur: canvas_native.shadow_blur,
        shadow_color: canvas_native.shadow_color,
        shadow_offset: canvas_native.shadow_offset.clone(),
        image_smoothing_enabled: canvas_native.image_smoothing_enabled,
        image_smoothing_quality: canvas_native.image_smoothing_quality.clone(),
        device_scale: canvas_native.device_scale,
        text_align: canvas_native.text_align.clone(),
        ios: canvas_native.ios.clone(),
        global_composite_operation: canvas_native.global_composite_operation.clone(),
        line_cap: canvas_native.line_cap.clone(),
        line_join: canvas_native.line_join.clone(),
        direction: canvas_native.direction.clone(),
        miter_limit: canvas_native.miter_limit,
        surface_kind: canvas_native.surface_kind.clone(),
        filter: canvas_native.filter.clone(),
    };

    let state = &mut canvas_native.state;
    state.push(CanvasStateItem::new(
        Box::into_raw(Box::new(canvas_state)) as *mut _ as i64,
        count,
    ));

    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn restore(canvas_ptr: c_longlong) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    canvas.restore();
    let state_item = canvas_native.state.pop();
    if state_item.is_some() {
        let item = state_item.unwrap();
        //canvas.restore_to_count(item.count);
        if item.state > 0 {
            let canvas_state: Box<CanvasState> = unsafe { Box::from_raw(item.state as *mut _) };
            canvas_native.restore_from_state_box(canvas_state);
        }
    }
    canvas_native.into_ptr()
}


#[inline]
pub(crate) fn set_line_cap(canvas_ptr: c_longlong, line_cap: *const c_char) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let cap = unsafe { CStr::from_ptr(line_cap as *mut _) }
        .to_str()
        .unwrap_or("butt");
    match cap {
        "round" => {
            canvas_native.stroke_paint.set_stroke_cap(Cap::Round);
            canvas_native.line_cap = "round".to_string();
        }
        "square" => {
            canvas_native.stroke_paint.set_stroke_cap(Cap::Square);
            canvas_native.line_cap = "square".to_string();
        }
        _ => {
            canvas_native.stroke_paint.set_stroke_cap(Cap::Butt);
            canvas_native.line_cap = "butt".to_string();
        }
    };
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn set_line_join(canvas_ptr: c_longlong, line_cap: *const c_char) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let cap = unsafe { CStr::from_ptr(line_cap as *mut _) }
        .to_str()
        .unwrap_or("miter");
    match cap {
        "round" => {
            canvas_native.stroke_paint.set_stroke_join(Join::Round);
            canvas_native.line_join = "round".to_string();
        }
        "bevel" => {
            canvas_native.stroke_paint.set_stroke_join(Join::Bevel);
            canvas_native.line_join = "bevel".to_string();
        }
        _ => {
            canvas_native.stroke_paint.set_stroke_join(Join::Miter);
            canvas_native.line_join = "miter".to_string();
        }
    };
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn set_global_alpha(canvas_ptr: c_longlong, alpha: u8) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    canvas_native.paint.set_alpha(alpha);
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn set_miter_limit(canvas_ptr: c_longlong, limit: f32) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    canvas_native.stroke_paint.set_stroke_miter(limit);
    canvas_native.miter_limit = limit;
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn set_line_dash_offset(canvas_ptr: c_longlong, offset: f32) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    canvas_native.line_dash_offset = offset;
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn set_shadow_blur(canvas_ptr: c_longlong, limit: f32) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    canvas_native.shadow_blur = limit;
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn set_shadow_color(canvas_ptr: c_longlong, color: u32) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    canvas_native.shadow_color = color;
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn set_shadow_offset_x(canvas_ptr: c_longlong, x: f32) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    canvas_native.shadow_offset_x = x;
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn set_shadow_offset_y(canvas_ptr: c_longlong, y: f32) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    canvas_native.shadow_offset_y = y;
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn get_measure_text(
    canvas_ptr: c_longlong,
    text: *const c_char,
) -> *mut CanvasTextMetrics {
    let mut metrics = CanvasTextMetrics { width: 0.0 };
    if canvas_ptr == 0 {
        return Box::into_raw(Box::new(metrics));
    }
    let canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let string = unsafe { CStr::from_ptr(text as *const _).to_str().unwrap_or("") };
    let measurement = canvas_native.font.measure_str(string, None);
    metrics.width = measurement.0;
    canvas_native.into_ptr();
    Box::into_raw(Box::new(metrics))
}


#[inline]
pub(crate) fn put_image_data(
    canvas_ptr: c_longlong,
    data: *const u8,
    data_size: size_t,
    width: c_int,
    height: c_int,
    x: c_float,
    y: c_float,
    dirty_x: c_float,
    dirty_y: c_float,
    dirty_width: c_int,
    dirty_height: c_int,
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let array = unsafe { std::slice::from_raw_parts(data, data_size) };
    // is surface is opaque use AlphaType::Opaque
    let mut _w = width;
    let mut _h = height;
    if dirty_width > -1 {
        _w = dirty_width;
    }
    if dirty_height > -1 {
        _h = dirty_height
    }
    let info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let row_bytes = (width * 4) as usize;
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    let _did_write = canvas.write_pixels(
        &info,
        &array,
        row_bytes,
        IPoint::new((x + dirty_x) as i32, (y + dirty_y) as i32),
    );
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn get_image_data(
    canvas_ptr: c_longlong,
    sx: c_float,
    sy: c_float,
    sw: size_t,
    sh: size_t,
) -> (c_longlong, Vec<u8>) {
    let pixels = Vec::new();
    if canvas_ptr == 0 {
        return (0, pixels);
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    let mut info = ImageInfo::new_n32_premul(ISize::new(sw as i32, sh as i32), None);
    let row_bytes = info.width() * 4;
    let mut slice = vec![255u8; (row_bytes * info.height()) as usize];
    let _read = canvas.read_pixels(
        &mut info,
        slice.as_mut_slice(),
        row_bytes as usize,
        IPoint::new(sx as _, sy as _),
    );
    let ptr = canvas_native.into_ptr();
    (ptr, slice)
}

#[allow(unused)]
#[inline]
pub(crate) fn free_image_data(data: *const u8) {
    Box::from(data);
}

#[inline]
pub(crate) fn set_image_smoothing_enabled(canvas_ptr: c_longlong, enabled: bool) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    update_quality(enabled, canvas_ptr)
}

#[inline]
pub(crate) fn set_image_smoothing_quality(
    canvas_ptr: c_longlong,
    quality: *const c_char,
) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }

    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    match unsafe { CStr::from_ptr(quality).to_str() } {
        Ok("high") => {
            canvas_native.image_smoothing_quality = FilterQuality::High;
        }
        Ok("medium") => {
            canvas_native.image_smoothing_quality = FilterQuality::Medium;
        }
        Ok("low") => {
            canvas_native.image_smoothing_quality = FilterQuality::Low;
        }
        _ => {}
    }

    if canvas_native.image_smoothing_enabled {
        canvas_native
            .paint
            .set_filter_quality(canvas_native.image_smoothing_quality);
    } else {
        canvas_native.paint.set_filter_quality(FilterQuality::None);
    }
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn set_text_align(canvas_ptr: c_longlong, alignment: *const c_char) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);

    let text_alignment = unsafe { CStr::from_ptr(alignment as *const _) };
    if let Ok(text_alignment) = text_alignment.to_str() {
        canvas_native.text_align = to_text_align(text_alignment);
    }
    canvas_native.into_ptr()
}

#[inline]
pub(crate) fn reset_transform(canvas_ptr: c_longlong) -> c_longlong {
    if canvas_ptr == 0 {
        return 0;
    }
    let mut canvas_native = CanvasNative::from_ptr(canvas_ptr);
    let surface = &mut canvas_native.surface;
    let canvas = surface.canvas();
    canvas.reset_matrix();
    canvas_native.into_ptr()
}

#[allow(unused)]
#[inline]
pub(crate) fn add_path_to_path(
    path_native_ptr: c_longlong,
    path_to_add_native_ptr: c_longlong,
) -> c_longlong {
    if path_native_ptr > 0 && path_to_add_native_ptr > 0 {
        let mut path: Box<Path> = unsafe { Box::from_raw(path_native_ptr as *mut _) };
        let mut path_to_add: Box<Path> = unsafe { Box::from_raw(path_to_add_native_ptr as *mut _) };
        let matrix = Matrix::default();
        path.add_path_matrix(&path_to_add, &matrix, None);
        Box::into_raw(path_to_add);
        return Box::into_raw(path) as *mut _ as i64;
    }
    path_native_ptr
}

#[inline]
pub(crate) fn add_path_to_path_with_matrix(
    path_native_ptr: c_longlong,
    path_to_add_native_ptr: c_longlong,
    matrix: c_longlong,
) -> c_longlong {
    if path_native_ptr > 0 && path_to_add_native_ptr > 0 {
        let mut path: Box<Path> = unsafe { Box::from_raw(path_native_ptr as *mut _) };
        let path_to_add: Box<Path> = unsafe { Box::from_raw(path_to_add_native_ptr as *mut _) };
        let matrix_to_add: Matrix;
        if matrix == 0 {
            matrix_to_add = Matrix::default();
        } else {
            let matrix: Box<Matrix> = unsafe { Box::from_raw(matrix as *mut _) };
            matrix_to_add = *(matrix.clone());
        }
        path.add_path_matrix(&path_to_add, &matrix_to_add, None);
        Box::into_raw(path_to_add);
        return Box::into_raw(path) as *mut _ as i64;
    }
    path_native_ptr
}
