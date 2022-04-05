use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_float, c_int, c_longlong, c_uchar, c_uint};
use std::str::FromStr;

use skia_safe::gpu::gl::Interface;
use skia_safe::image::CachingHint;
use skia_safe::{
    AlphaType, Color, ColorType, EncodedImageFormat, IPoint, ISize, ImageInfo, PixelGeometry, Rect,
    Surface, M44,
};

use canvas_core::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_core::context::drawing_paths::fill_rule::FillRule;
use canvas_core::context::fill_and_stroke_styles::paint::PaintStyle;
use canvas_core::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_core::context::image_asset::ImageAsset;
use canvas_core::context::image_smoothing::ImageSmoothingQuality;
use canvas_core::context::line_styles::line_cap::LineCap;
use canvas_core::context::line_styles::line_join::LineJoin;
use canvas_core::context::matrix::Matrix;
use canvas_core::context::paths::path::Path;
use canvas_core::context::pixel_manipulation::image_data::ImageData;
use canvas_core::context::text_styles::text_align::TextAlign;
use canvas_core::context::text_styles::text_baseline::TextBaseLine;
use canvas_core::context::text_styles::text_direction::TextDirection;
use canvas_core::context::ContextWrapper;
use canvas_core::context::{Context, Device, State};
use canvas_core::ffi::f32_array::F32Array;
use canvas_core::ffi::paint_style_value::PaintStyleValue;
use canvas_core::ffi::paint_style_value::PaintStyleValueType;
use canvas_core::ffi::u8_array::U8Array;
use canvas_core::to_data_url;
use canvas_core::utils::color::to_parsed_color;
use canvas_core::utils::image::{from_image_slice, to_image, to_image_encoded};

const GR_GL_RGB565: c_uint = 0x8D62;
const GR_GL_RGBA8: c_uint = 0x8058;

#[cfg(feature = "gl")]
#[no_mangle]
pub extern "C" fn canvas_native_context_init_gl(
    width: c_float,
    height: c_float,
    density: c_float,
    buffer_id: c_int,
    samples: usize,
    alpha: bool,
    font_color: c_uint,
    ppi: c_float,
    direction: TextDirection,
) -> *mut canvas_core::context::ContextWrapper {
    canvas_core::context::ContextWrapper::new(Context::new_gl(
        width,
        height,
        density,
        buffer_id,
        samples as i32,
        alpha,
        font_color as i32,
        ppi,
        direction,
    ))
    .into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_init(
    width: c_float,
    height: c_float,
    density: c_float,
    alpha: bool,
    font_color: c_int,
    ppi: c_float,
    direction: TextDirection,
) -> *mut canvas_core::context::ContextWrapper {
    canvas_core::context::ContextWrapper::new(Context::new(
        width, height, density, alpha, font_color, ppi, direction,
    ))
    .into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_resize(
    context: *mut canvas_core::context::ContextWrapper,
    width: c_float,
    height: c_float,
    density: c_float,
    alpha: bool,
    ppi: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        Context::resize(&mut context, width, height, density, alpha, ppi)
    }
}

#[cfg(feature = "gl")]
#[no_mangle]
pub extern "C" fn canvas_native_context_resize_gl(
    context: *mut canvas_core::context::ContextWrapper,
    width: c_float,
    height: c_float,
    density: c_float,
    buffer_id: c_int,
    samples: usize,
    alpha: bool,
    ppi: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        Context::resize_gl(
            &mut context,
            width,
            height,
            density,
            buffer_id,
            samples as i32,
            alpha,
            ppi,
        )
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_data_url(
    context: *mut canvas_core::context::ContextWrapper,
    format: *const c_char,
    quality: f32,
) -> *const c_char {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let format = CStr::from_ptr(format).to_string_lossy();
        CString::new(to_data_url(
            context,
            format.as_ref(),
            (quality * 100 as f32) as i32,
        ))
        .unwrap()
        .into_raw()
    }
}

#[inline]
pub(crate) fn canvas_native_context_to_data(
    context: *mut canvas_core::context::ContextWrapper,
) -> Vec<u8> {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.read_pixels()
    }
}

#[inline]
#[no_mangle]
pub extern "C" fn canvas_native_context_snapshot_canvas(
    context: *mut canvas_core::context::ContextWrapper,
) -> *mut U8Array {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let bytes = context.read_pixels();
        let mut ptr = bytes.into_boxed_slice();
        let raw = U8Array {
            data_len: ptr.len(),
            data: ptr.as_mut_ptr(),
        };
        Box::into_raw(ptr);
        Box::into_raw(Box::new(raw))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_flush(context: *mut canvas_core::context::ContextWrapper) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.flush()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_custom_with_buffer_flush(
    context: *mut canvas_core::context::ContextWrapper,
    buf: *mut u8,
    buf_size: usize,
    width: f32,
    height: f32,
) {
    unsafe {
        if buf.is_null() || buf_size == 0 {
            return;
        }
        let context = &mut *context;
        let mut context = context.get_context();
        let image_data = std::slice::from_raw_parts_mut(buf, buf_size);
        Context::flush_buffer(&mut context, width as i32, height as i32, image_data)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_direction(
    context: *mut canvas_core::context::ContextWrapper,
    direction: TextDirection,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_direction(direction);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_direction(
    context: *const canvas_core::context::ContextWrapper,
) -> TextDirection {
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.direction()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_fill_style(
    context: *mut canvas_core::context::ContextWrapper,
    style: *mut PaintStyle,
) {
    assert!(!context.is_null());
    assert!(!style.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let style = &*style;
        context.set_fill_style(style.clone())
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_fill_style(
    context: *mut canvas_core::context::ContextWrapper,
) -> *mut PaintStyleValue {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let context = context.get_context();
        let fill_style = context.fill_style().clone();
        let result = match fill_style {
            PaintStyle::Color(_) => {
                PaintStyleValue::new(fill_style, PaintStyleValueType::PaintStyleValueTypeColor)
            }
            PaintStyle::Gradient(_) => {
                PaintStyleValue::new(fill_style, PaintStyleValueType::PaintStyleValueTypeGradient)
            }
            PaintStyle::Pattern(_) => {
                PaintStyleValue::new(fill_style, PaintStyleValueType::PaintStyleValueTypePattern)
            }
        };
        Box::into_raw(Box::new(result))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_style_value(style: *mut PaintStyleValue) -> c_longlong {
    assert!(!style.is_null());
    unsafe {
        let style: *mut PaintStyleValue = style as _;
        let style = &mut *style;
        return style.value;
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_style_type(
    style: *mut PaintStyleValue,
) -> PaintStyleValueType {
    assert!(!style.is_null());
    unsafe {
        let style: *mut PaintStyleValue = style as _;
        let style = &mut *style;
        return style.value_type;
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_filter(
    context: *mut canvas_core::context::ContextWrapper,
    filter: *const c_char,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let filter = CStr::from_ptr(filter).to_string_lossy();
        context.set_filter(filter.as_ref())
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_filter(
    context: *mut canvas_core::context::ContextWrapper,
) -> *const c_char {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        let filter = context.get_filter();
        CString::new(filter).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_font(
    context: *mut canvas_core::context::ContextWrapper,
    filter: *const c_char,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let font = CStr::from_ptr(filter).to_string_lossy();
        context.set_font(font.as_ref());
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_font(
    context: *mut canvas_core::context::ContextWrapper,
) -> *const c_char {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        let font = context.font();
        CString::new(font).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_global_alpha(
    context: *mut canvas_core::context::ContextWrapper,
    alpha: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_global_alpha(alpha)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_global_alpha(
    context: *mut canvas_core::context::ContextWrapper,
) -> c_float {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.global_alpha()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_global_composite_operation(
    context: *mut canvas_core::context::ContextWrapper,
    operation: CompositeOperationType,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_global_composite_operation(operation);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_global_composite_operation(
    context: *mut canvas_core::context::ContextWrapper,
) -> CompositeOperationType {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.global_composite_operation()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_image_smoothing_enabled(
    context: *mut canvas_core::context::ContextWrapper,
    enabled: bool,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_image_smoothing_enabled(enabled);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_image_smoothing_enabled(
    context: *mut canvas_core::context::ContextWrapper,
) -> bool {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.get_image_smoothing_enabled()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_image_smoothing_quality(
    context: *mut canvas_core::context::ContextWrapper,
    quality: ImageSmoothingQuality,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_image_smoothing_quality(quality);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_image_smoothing_quality(
    context: *mut canvas_core::context::ContextWrapper,
) -> ImageSmoothingQuality {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.get_image_smoothing_quality()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_cap(
    context: *mut canvas_core::context::ContextWrapper,
    cap: LineCap,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_line_cap(cap)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_cap(
    context: *mut canvas_core::context::ContextWrapper,
) -> LineCap {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.line_cap()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_dash_offset(
    context: *mut canvas_core::context::ContextWrapper,
    offset: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_line_dash_offset(offset)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_dash_offset(
    context: *mut canvas_core::context::ContextWrapper,
) -> c_float {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.line_dash_offset()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_join(
    context: *mut canvas_core::context::ContextWrapper,
    join: LineJoin,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_line_join(join)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_join(
    context: *mut canvas_core::context::ContextWrapper,
) -> LineJoin {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.line_join()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_width(
    context: *mut canvas_core::context::ContextWrapper,
    width: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_line_width(width)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_width(
    context: *mut canvas_core::context::ContextWrapper,
) -> c_float {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.line_width()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_miter_limit(
    context: *mut canvas_core::context::ContextWrapper,
    limit: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_miter_limit(limit)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_miter_limit(
    context: *mut canvas_core::context::ContextWrapper,
) -> c_float {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.miter_limit()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_blur(
    context: *mut canvas_core::context::ContextWrapper,
    blur: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_shadow_blur(blur)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_blur(
    context: *mut canvas_core::context::ContextWrapper,
) -> c_float {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.shadow_blur()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_color(
    context: *mut canvas_core::context::ContextWrapper,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_shadow_color(skia_safe::Color::from_argb(a, r, g, b))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_color_string(
    context: *mut canvas_core::context::ContextWrapper,
    color: *const c_char,
) {
    assert!(!context.is_null());
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let color = CStr::from_ptr(color).to_string_lossy();
        if let Ok(color) = css_color_parser::Color::from_str(color.as_ref()) {
            context.set_shadow_color(skia_safe::Color::from_argb(
                (color.a * 255.0) as u8,
                color.r,
                color.g,
                color.b,
            ))
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_color(
    context: *mut canvas_core::context::ContextWrapper,
) -> *const c_char {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        CString::new(to_parsed_color(context.shadow_color()))
            .unwrap()
            .into_raw()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_offset_x(
    context: *mut canvas_core::context::ContextWrapper,
    x: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_shadow_offset_x(x)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_offset_x(
    context: *mut canvas_core::context::ContextWrapper,
) -> c_float {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.shadow_offset_x()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_offset_y(
    context: *mut canvas_core::context::ContextWrapper,
    y: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_shadow_offset_y(y)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_offset_y(
    context: *mut canvas_core::context::ContextWrapper,
) -> c_float {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.shadow_offset_y()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_stroke_style(
    context: *mut canvas_core::context::ContextWrapper,
    style: *mut PaintStyle,
) {
    assert!(!context.is_null());
    assert!(!style.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let style = &*style;
        context.set_stroke_style(style.clone())
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_stroke_style(
    context: *mut canvas_core::context::ContextWrapper,
) -> *mut PaintStyleValue {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let context = context.get_context();
        let stroke_style = context.stroke_style().clone();
        Box::into_raw(Box::new(match stroke_style {
            PaintStyle::Color(_) => {
                PaintStyleValue::new(stroke_style, PaintStyleValueType::PaintStyleValueTypeColor)
            }
            PaintStyle::Gradient(_) => PaintStyleValue::new(
                stroke_style,
                PaintStyleValueType::PaintStyleValueTypeGradient,
            ),
            PaintStyle::Pattern(_) => PaintStyleValue::new(
                stroke_style,
                PaintStyleValueType::PaintStyleValueTypePattern,
            ),
        }))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_text_align(
    context: *mut canvas_core::context::ContextWrapper,
    align: TextAlign,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_text_align(align)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_text_align(
    context: *mut canvas_core::context::ContextWrapper,
) -> TextAlign {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.text_align()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_text_baseline(
    context: *mut canvas_core::context::ContextWrapper,
    baseline: TextBaseLine,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_text_baseline(baseline);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_text_baseline(
    context: *mut canvas_core::context::ContextWrapper,
) -> TextBaseLine {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        context.text_baseline()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_arc(
    context: *mut canvas_core::context::ContextWrapper,
    x: c_float,
    y: c_float,
    radius: c_float,
    start_angle: c_float,
    end_angle: c_float,
    anti_clockwise: bool,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.arc(x, y, radius, start_angle, end_angle, anti_clockwise)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_arc_to(
    context: *mut canvas_core::context::ContextWrapper,
    x1: c_float,
    y1: c_float,
    x2: c_float,
    y2: c_float,
    radius: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.arc_to(x1, y1, x2, y2, radius)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_begin_path(
    context: *mut canvas_core::context::ContextWrapper,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.begin_path()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_bezier_curve_to(
    context: *mut canvas_core::context::ContextWrapper,
    cp1x: c_float,
    cp1y: c_float,
    cp2x: c_float,
    cp2y: c_float,
    x: c_float,
    y: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_clear_rect(
    context: *mut canvas_core::context::ContextWrapper,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.clear_rect(x, y, width, height)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_clip(
    context: *mut canvas_core::context::ContextWrapper,
    path: *mut Path,
    rule: FillRule,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.clip(Some(&mut *path), Some(rule))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_clip_rule(
    context: *mut canvas_core::context::ContextWrapper,
    rule: FillRule,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.clip(None, Some(rule))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_close_path(
    context: *mut canvas_core::context::ContextWrapper,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.close_path()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_image_data(
    width: c_int,
    height: c_int,
) -> *mut ImageData {
    Box::into_raw(Box::new(Context::create_image_data(width, height)))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_linear_gradient(
    context: *mut canvas_core::context::ContextWrapper,
    x0: c_float,
    y0: c_float,
    x1: c_float,
    y1: c_float,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        Box::into_raw(Box::new(PaintStyle::Gradient(
            context.create_linear_gradient(x0, y0, x1, y1),
        )))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern(
    context: *mut canvas_core::context::ContextWrapper,
    image_data: *const u8,
    image_len: usize,
    width: c_int,
    height: c_int,
    repetition: Repetition,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        if let Some(image) = to_image(image_data, image_len, width, height) {
            return Box::into_raw(Box::new(PaintStyle::Pattern(
                context.create_pattern(image, repetition),
            )));
        }
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern_asset(
    context: *mut canvas_core::context::ContextWrapper,
    asset: *mut ImageAsset,
    repetition: Repetition,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    assert!(!asset.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let asset = &mut *asset;
        let bytes = asset.rgba_internal_bytes();
        if let Some(image) = from_image_slice(
            bytes.as_slice(),
            asset.width() as i32,
            asset.height() as i32,
        ) {
            return Box::into_raw(Box::new(PaintStyle::Pattern(
                context.create_pattern(image, repetition),
            )));
        }
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern_encoded(
    context: *mut canvas_core::context::ContextWrapper,
    image_data: *const u8,
    image_len: usize,
    repetition: Repetition,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        if let Some(image) = to_image_encoded(image_data, image_len) {
            return Box::into_raw(Box::new(PaintStyle::Pattern(
                context.create_pattern(image, repetition),
            )));
        }
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_radial_gradient(
    context: *mut canvas_core::context::ContextWrapper,
    x0: c_float,
    y0: c_float,
    r0: c_float,
    x1: c_float,
    y1: c_float,
    r1: c_float,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        Box::into_raw(Box::new(PaintStyle::Gradient(
            context.create_radial_gradient(x0, y0, r0, x1, y1, r1),
        )))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy(
    context: *mut canvas_core::context::ContextWrapper,
    image_data: *const u8,
    image_len: usize,
    width: c_float,
    height: c_float,
    dx: c_float,
    dy: c_float,
) {
    canvas_native_context_draw_image(
        context, image_data, image_len, width, height, 0.0, 0.0, width, height, dx, dy, width,
        height,
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_dw_dh(
    context: *mut canvas_core::context::ContextWrapper,
    image_data: *const u8,
    image_len: usize,
    width: c_float,
    height: c_float,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) {
    canvas_native_context_draw_image(
        context, image_data, image_len, width, height, 0.0, 0.0, width, height, dx, dy, d_width,
        d_height,
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image(
    context: *mut canvas_core::context::ContextWrapper,
    image_data: *const u8,
    image_len: usize,
    width: c_float,
    height: c_float,
    sx: c_float,
    sy: c_float,
    s_width: c_float,
    s_height: c_float,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        if let Some(image) = to_image(image_data, image_len, width as i32, height as i32) {
            context.draw_image(
                &image,
                Rect::from_xywh(sx, sy, s_width, s_height),
                Rect::from_xywh(dx, dy, d_width, d_height),
            )
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_encoded_dx_dy(
    context: *mut canvas_core::context::ContextWrapper,
    image_data: *const u8,
    image_len: usize,
    dx: c_float,
    dy: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        if let Some(image) = to_image_encoded(image_data, image_len) {
            let width = image.width() as f32;
            let height = image.height() as f32;
            context.draw_image(
                &image,
                Rect::from_xywh(0.0, 0.0, width, height),
                Rect::from_xywh(dx, dy, width, height),
            )
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_encoded_dx_dy_dw_dh(
    context: *mut canvas_core::context::ContextWrapper,
    image_data: *const u8,
    image_len: usize,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        if let Some(image) = to_image_encoded(image_data, image_len) {
            let width = image.width() as f32;
            let height = image.height() as f32;
            context.draw_image(
                &image,
                Rect::from_xywh(0.0, 0.0, width, height),
                Rect::from_xywh(dx, dy, d_width, d_height),
            )
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_encoded(
    context: *mut canvas_core::context::ContextWrapper,
    image_data: *const u8,
    image_len: usize,
    sx: c_float,
    sy: c_float,
    s_width: c_float,
    s_height: c_float,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        if let Some(image) = to_image_encoded(image_data, image_len) {
            context.draw_image(
                &image,
                Rect::from_xywh(sx, sy, s_width, s_height),
                Rect::from_xywh(dx, dy, d_width, d_height),
            )
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_asset(
    context: *mut canvas_core::context::ContextWrapper,
    asset: *mut ImageAsset,
    dx: c_float,
    dy: c_float,
) {
    assert!(!context.is_null());
    assert!(!asset.is_null());
    unsafe {
        let image = &*asset;
        let width = image.width() as f32;
        let height = image.height() as f32;
        canvas_native_context_draw_image_asset(
            context, asset, 0.0, 0.0, width, height, dx, dy, width, height,
        );
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_dw_dh_asset(
    context: *mut canvas_core::context::ContextWrapper,
    asset: *mut ImageAsset,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) {
    assert!(!context.is_null());
    assert!(!asset.is_null());
    unsafe {
        let image = &*asset;
        let width = image.width() as f32;
        let height = image.height() as f32;
        canvas_native_context_draw_image_asset(
            context, asset, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
        );
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_asset(
    context: *mut canvas_core::context::ContextWrapper,
    asset: *mut ImageAsset,
    sx: c_float,
    sy: c_float,
    s_width: c_float,
    s_height: c_float,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) {
    assert!(!context.is_null());
    assert!(!asset.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let asset = &mut *asset;
        let bytes = asset.rgba_internal_bytes();
        if let Some(image) = from_image_slice(
            bytes.as_slice(),
            asset.width() as i32,
            asset.height() as i32,
        ) {
            context.draw_image(
                &image,
                Rect::from_xywh(sx, sy, s_width, s_height),
                Rect::from_xywh(dx, dy, d_width, d_height),
            )
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_ellipse(
    context: *mut canvas_core::context::ContextWrapper,
    x: c_float,
    y: c_float,
    radius_x: c_float,
    radius_y: c_float,
    rotation: c_float,
    start_angle: c_float,
    end_angle: c_float,
    anticlockwise: bool,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.ellipse(
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
}

#[no_mangle]
pub extern "C" fn canvas_native_context_fill(
    context: *mut canvas_core::context::ContextWrapper,
    path: *mut Path,
    rule: FillRule,
) {
    assert!(!context.is_null());
    assert!(!path.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        if path.is_null() {
            context.fill(None, rule)
        } else {
            context.fill(Some(&mut *path), rule)
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_fill_rect(
    context: *mut canvas_core::context::ContextWrapper,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let rect = Rect::from_xywh(x, y, width, height);
        context.fill_rect(&rect);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_fill_text(
    context: *mut canvas_core::context::ContextWrapper,
    text: *const c_char,
    x: c_float,
    y: c_float,
    width: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let txt = CStr::from_ptr(text).to_string_lossy();
        context.fill_text(txt.as_ref(), x, y, width)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_image_data(
    context: *mut canvas_core::context::ContextWrapper,
    sx: c_float,
    sy: c_float,
    sw: c_float,
    sh: c_float,
) -> *mut ImageData {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        Box::into_raw(Box::new(context.get_image_data(sx, sy, sw, sh)))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_dash(
    context: *mut canvas_core::context::ContextWrapper,
) -> *mut F32Array {
    assert!(!context.is_null());
    unsafe {
        let context = &*context;
        let context = context.get_context();
        let mut line_dash = context.line_dash().to_vec().into_boxed_slice();
        let array = F32Array {
            data: line_dash.as_mut_ptr(),
            data_len: line_dash.len(),
        };
        let _ = Box::into_raw(line_dash);
        Box::into_raw(Box::new(array))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_transform(
    context: *mut canvas_core::context::ContextWrapper,
) -> *mut Matrix {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let matrix = Matrix::from(context.get_transform());
        Box::into_raw(Box::new(matrix))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_path(
    context: *mut canvas_core::context::ContextWrapper,
    path: *mut Path,
    x: c_float,
    y: c_float,
    rule: FillRule,
) -> bool {
    assert!(!context.is_null());
    assert!(!path.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        if path.is_null() {
            context.is_point_in_path(None, x, y, rule)
        } else {
            context.is_point_in_path(Some(&*path), x, y, rule)
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_stroke(
    context: *mut canvas_core::context::ContextWrapper,
    path: *mut Path,
    x: c_float,
    y: c_float,
) -> bool {
    assert!(!context.is_null());
    assert!(!path.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        if path.is_null() {
            context.is_point_in_stroke(None, x, y)
        } else {
            context.is_point_in_stroke(Some(&*path), x, y)
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_line_to(
    context: *mut canvas_core::context::ContextWrapper,
    x: c_float,
    y: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.line_to(x, y)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_measure_text(
    context: *mut canvas_core::context::ContextWrapper,
    text: *const c_char,
) -> c_longlong {
    unsafe {
        let context = &*context;
        let context = context.get_context();
        let text = CStr::from_ptr(text).to_string_lossy();
        Box::into_raw(Box::new(context.measure_text(text.as_ref()))) as c_longlong
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_move_to(
    context: *mut canvas_core::context::ContextWrapper,
    x: c_float,
    y: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.move_to(x, y)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_put_image_data(
    context: *mut canvas_core::context::ContextWrapper,
    image_data: *mut ImageData,
    dx: c_float,
    dy: c_float,
    dirty_x: c_float,
    dirty_y: c_float,
    dirty_width: c_float,
    dirty_height: c_float,
) {
    assert!(!context.is_null());
    assert!(!image_data.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.put_image_data(
            &*image_data,
            dx,
            dy,
            dirty_x,
            dirty_y,
            dirty_width,
            dirty_height,
        )
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_quadratic_curve_to(
    context: *mut canvas_core::context::ContextWrapper,
    cpx: c_float,
    cpy: c_float,
    x: c_float,
    y: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.quadratic_curve_to(cpx, cpy, x, y)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_rect(
    context: *mut canvas_core::context::ContextWrapper,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.rect(x, y, width, height)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_reset_transform(
    context: *mut canvas_core::context::ContextWrapper,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.reset_transform()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_restore(
    context: *mut canvas_core::context::ContextWrapper,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.restore()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_rotate(
    context: *mut canvas_core::context::ContextWrapper,
    angle: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.rotate(angle)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_save(context: *mut canvas_core::context::ContextWrapper) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.save()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_scale(
    context: *mut canvas_core::context::ContextWrapper,
    x: c_float,
    y: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.scale(x, y)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_dash(
    context: *mut canvas_core::context::ContextWrapper,
    data: *const c_float,
    data_length: usize,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_line_dash(std::slice::from_raw_parts(data, data_length))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_transform(
    context: *mut canvas_core::context::ContextWrapper,
    a: c_float,
    b: c_float,
    c: c_float,
    d: c_float,
    e: c_float,
    f: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.set_transform(a, b, c, d, e, f)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_transform_matrix(
    context: *mut canvas_core::context::ContextWrapper,
    matrix: *mut Matrix,
) {
    assert!(!context.is_null());
    assert!(!matrix.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let matrix = &*matrix;
        let matrix = matrix.to_m33();
        context.set_transform_matrix(&matrix)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_stroke(
    context: *mut canvas_core::context::ContextWrapper,
    path: *mut Path,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        if path.is_null() {
            context.stroke(None)
        } else {
            context.stroke(Some(&mut *path))
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_stroke_rect(
    context: *mut canvas_core::context::ContextWrapper,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let rect = Rect::from_xywh(x, y, width, height);
        context.stroke_rect(&rect);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_stroke_text(
    context: *mut canvas_core::context::ContextWrapper,
    text: *const c_char,
    x: c_float,
    y: c_float,
    width: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        let txt = CStr::from_ptr(text).to_string_lossy();
        context.stroke_text(txt.as_ref(), x, y, width)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_transform(
    context: *mut canvas_core::context::ContextWrapper,
    a: c_float,
    b: c_float,
    c: c_float,
    d: c_float,
    e: c_float,
    f: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.transform(a, b, c, d, e, f)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_translate(
    context: *mut canvas_core::context::ContextWrapper,
    x: c_float,
    y: c_float,
) {
    assert!(!context.is_null());
    unsafe {
        let context = &mut *context;
        let mut context = context.get_context();
        context.translate(x, y)
    }
}
