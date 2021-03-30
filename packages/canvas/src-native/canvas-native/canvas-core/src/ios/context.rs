use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_float, c_int, c_longlong, c_uint};
use std::panic::catch_unwind;
use std::ptr::null;
use std::str::Utf8Error;
use std::sync::{Arc, Mutex};

use skia_safe::{AlphaType, Color, ColorType, EncodedImageFormat, ImageInfo, IPoint, ISize, M44, PixelGeometry, Rect, Surface};
use skia_safe::gpu::gl::Interface;
use skia_safe::image::CachingHint;

use crate::common::context::{Context, Device, State};
use crate::common::context::compositing::composite_operation_type::CompositeOperationType;
use crate::common::context::drawing_paths::fill_rule::FillRule;
use crate::common::context::drawing_text::text_metrics::TextMetrics;
use crate::common::context::fill_and_stroke_styles::gradient::Gradient;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::context::fill_and_stroke_styles::pattern::{Pattern, Repetition};
use crate::common::context::image_asset::ImageAsset;
use crate::common::context::image_smoothing::ImageSmoothingQuality;
use crate::common::context::line_styles::line_cap::LineCap;
use crate::common::context::line_styles::line_join::LineJoin;
use crate::common::context::matrix::Matrix;
use crate::common::context::paths::path::Path;
use crate::common::context::pixel_manipulation::image_data::ImageData;
use crate::common::context::text_styles::text_align::TextAlign;
use crate::common::context::text_styles::text_baseline::TextBaseLine;
use crate::common::context::text_styles::text_direction::TextDirection;
use crate::common::ffi::f32_array::F32Array;
use crate::common::ffi::paint_style_value::{PaintStyleValue, PaintStyleValueType};
use crate::common::ffi::u8_array::U8Array;
use crate::common::to_data_url;
use crate::common::utils::color::to_parsed_color;
use crate::common::utils::image::{from_image_slice, to_image, to_image_encoded};

/*
#[inline]
pub extern "C" fn context_init_context_mtl(
    device: *mut c_void,
    queue: *mut c_void,
    view: *mut c_void,
    density: c_float,
    scale: c_float,
    font_color
    direction: *const c_char,
) -> *mut Context {
    let mut context = unsafe { gpu::Context::new_metal(device, queue) }.unwrap();
    let surface_props = SurfaceProps::new(SurfacePropsFlags::default(), PixelGeometry::Unknown);
    let surface_holder = Surface::from_ca_mtk_view(
        &mut context,
        view,
        gpu::SurfaceOrigin::TopLeft,
        Some(1),
        ColorType::BGRA8888,
        None,
        Some(&surface_props),
    );
    }

 */

const GR_GL_RGB565: c_uint = 0x8D62;
const GR_GL_RGBA8: c_uint = 0x8058;

#[no_mangle]
pub extern "C" fn context_init_context(
    width: c_float,
    height: c_float,
    density: c_float,
    buffer_id: c_int,
    samples: usize,
    alpha: bool,
    font_color: c_uint,
    ppi: c_float,
    direction: TextDirection,
) -> c_longlong {
    let density = 1.0;
    let mut device = Device {
        width,
        height,
        density,
        non_gpu: false,
        samples,
        alpha,
        ppi,
    };
    let interface = Interface::new_native();
    let mut ctx = skia_safe::gpu::DirectContext::new_gl(interface, None).unwrap();
    let mut frame_buffer = skia_safe::gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);
    if alpha {
        frame_buffer.format = GR_GL_RGBA8;
    } else {
        frame_buffer.format = GR_GL_RGB565;
    }

    let target = skia_safe::gpu::BackendRenderTarget::new_gl(
        (width as i32, height as i32),
        Some(samples),
        8,
        frame_buffer,
    );
    let surface_props =
        skia_safe::SurfaceProps::new(skia_safe::SurfacePropsFlags::default(), PixelGeometry::Unknown);
    let mut color_type = ColorType::RGBA8888;
    if !alpha {
        color_type = ColorType::RGB565;
    }
    let mut surface_holder = Surface::from_backend_render_target(
        &mut ctx,
        &target,
        skia_safe::gpu::SurfaceOrigin::BottomLeft,
        color_type,
        None,
        Some(&surface_props),
    );

    Box::into_raw(Box::new(Context {
        surface: surface_holder.unwrap(),
        path: Path::default(),
        state: State::from_device(device, direction),
        state_stack: vec![],
        font_color: Color::new(font_color),
        device,
    })) as c_longlong
}


#[no_mangle]
pub extern "C" fn context_init_context_with_custom_surface(
    width: c_float,
    height: c_float,
    density: c_float,
    alpha: bool,
    font_color: c_int,
    ppi: c_float,
    direction: TextDirection,
) -> c_longlong {
    // let density = 1.0;
    let mut device = Device {
        width,
        height,
        density,
        non_gpu: true,
        samples: 0,
        alpha,
        ppi,
    };
    let info = ImageInfo::new(
        ISize::new(width as i32, height as i32),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );

    Box::into_raw(Box::new(Context {
        surface: Surface::new_raster(
            &info,
            None,
            None,
        ).unwrap(),
        path: Path::default(),
        state: State::from_device(device, direction),
        state_stack: vec![],
        font_color: Color::new(font_color as u32),
        device,
    })) as c_longlong
}

#[no_mangle]
pub extern "C" fn context_resize_custom_surface(
    context: c_longlong,
    width: c_float,
    height: c_float,
    density: c_float,
    alpha: bool,
    ppi: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let mut device = Device {
            width,
            height,
            density,
            non_gpu: true,
            samples: 0,
            alpha,
            ppi,
        };

        let info = ImageInfo::new(
            ISize::new(width as i32, height as i32),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        if let Some(surface) = Surface::new_raster(
            &info,
            None,
            None,
        ) {
            context.surface = surface;
            context.device = device;
            context.path = Path::default();
            context.reset_state();
        }
    }
}


#[no_mangle]
pub extern "C" fn context_resize_surface(
    context: c_longlong,
    width: c_float,
    height: c_float,
    density: c_float,
    buffer_id: c_int,
    samples: usize,
    alpha: bool,
    ppi: c_float,
) {
    let density = 1.0;
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let interface = skia_safe::gpu::gl::Interface::new_native();
        let ctx = skia_safe::gpu::DirectContext::new_gl(interface, None);
        if ctx.is_none() {
            return;
        }
        let mut ctx = ctx.unwrap();
        ctx.reset(None);
        let mut device = Device {
            width,
            height,
            density,
            non_gpu: false,
            samples,
            alpha: false,
            ppi,
        };
        let mut frame_buffer = skia_safe::gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);

        if alpha {
            frame_buffer.format = GR_GL_RGBA8;
        } else {
            frame_buffer.format = GR_GL_RGB565;
        }

        let target = skia_safe::gpu::BackendRenderTarget::new_gl(
            (width as i32, height as i32),
            Some(samples),
            8,
            frame_buffer,
        );
        let surface_props =
            skia_safe::SurfaceProps::new(skia_safe::SurfacePropsFlags::default(), PixelGeometry::Unknown);
        let mut color_type = ColorType::RGBA8888;

        if !alpha {
            color_type = ColorType::RGB565;
        }
        if let Some(surface) = Surface::from_backend_render_target(
            &mut ctx,
            &target,
            skia_safe::gpu::SurfaceOrigin::BottomLeft,
            color_type,
            None,
            Some(&surface_props),
        ) {
            context.surface = surface;
            context.device = device;
            context.path = Path::default();
            context.reset_state();
        }
    }
}

#[no_mangle]
pub extern "C" fn context_data_url(
    context: c_longlong,
    format: *const c_char,
    quality: f32,
) -> *const c_char {
    unsafe {
        if context == 0 {
            return std::ptr::null();
        }
        let context: *mut Context = context as _;
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
pub(crate) fn context_to_data(context: *mut Context) -> Vec<u8> {
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        let surface = &mut context.surface;
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
        let _ = image.read_pixels(
            &mut info,
            pixels.as_mut_slice(),
            row_bytes as usize,
            IPoint::new(0, 0),
            CachingHint::Allow,
        );
        pixels
    }
}

#[inline]
#[no_mangle]
pub extern "C" fn context_snapshot_canvas(context: c_longlong) -> *mut U8Array {
    if context == 0 {
        return std::ptr::null_mut();
    }
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.surface.canvas().flush();
        let ss = context.surface.image_snapshot();
        let data = ss.encode_to_data(EncodedImageFormat::PNG).unwrap();
        let len = data.len();
        let bytes = data.as_bytes().to_vec();
        let mut ptr = bytes.into_boxed_slice();
        let raw = U8Array {
            data_len: len,
            data: ptr.as_mut_ptr(),
        };
        Box::into_raw(ptr);
        Box::into_raw(
            Box::new(raw)
        )
    }
}

#[no_mangle]
pub extern "C" fn context_flush(context: c_longlong) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.flush()
    }
}


#[no_mangle]
pub extern "C" fn context_custom_with_buffer_flush(context: c_longlong, buf: *mut u8, buf_size: usize, width: f32, height: f32) {
    unsafe {
        if context == 0 || buf.is_null() || buf_size == 0 {
            return;
        }
        let info = ImageInfo::new(
            ISize::new(width as i32, height as i32),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );
        let context: *mut Context = context as _;
        let context = &mut *context;
        let image_data = std::slice::from_raw_parts_mut(buf, buf_size);
        let mut surface = Surface::new_raster_direct(
            &info, image_data, None, None,
        ).unwrap();
        let canvas = surface.canvas();
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::PaintStyle::Fill);
        paint.set_blend_mode(skia_safe::BlendMode::Clear);
        canvas.draw_rect(
            Rect::from_xywh(0f32, 0f32, width as f32, height as f32),
            &paint,
        );
        context.draw_on_surface(&mut surface);
    }
}

#[no_mangle]
pub extern "C" fn context_set_direction(context: c_longlong, direction: TextDirection) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_direction(direction);
    }
}

#[no_mangle]
pub extern "C" fn context_get_direction(context: *const Context) -> TextDirection {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.direction()
    }
}

#[no_mangle]
pub extern "C" fn context_set_fill_style(context: c_longlong, style: c_longlong) {
    unsafe {
        if context == 0 || style == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let style: *const PaintStyle = style as _;
        let style = &*style;
        context.set_fill_style(style.clone())
    }
}

#[no_mangle]
pub extern "C" fn context_get_fill_style(context: c_longlong) -> *mut PaintStyleValue {
    if context == 0 {
        return std::ptr::null_mut();
    }
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        let fill_style = context.fill_style().clone();
        let result = match fill_style {
            PaintStyle::Color(_) => PaintStyleValue::new(fill_style, PaintStyleValueType::PaintStyleValueTypeColor),
            PaintStyle::Gradient(_) => PaintStyleValue::new(fill_style, PaintStyleValueType::PaintStyleValueTypeGradient),
            PaintStyle::Pattern(_) => PaintStyleValue::new(fill_style, PaintStyleValueType::PaintStyleValueTypePattern)
        };
        Box::into_raw(
            Box::new(
                result
            )
        )
    }
}

#[no_mangle]
pub extern "C" fn context_set_filter(context: c_longlong, filter: *const c_char) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let filter = CStr::from_ptr(filter).to_string_lossy();
        context.set_filter(filter.as_ref())
    }
}

#[no_mangle]
pub extern "C" fn context_get_filter(context: c_longlong) -> *const c_char {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        let filter = context.get_filter();
        CString::new(filter).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn context_set_font(context: c_longlong, filter: *const c_char) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let font = CStr::from_ptr(filter).to_string_lossy();
        context.set_font(font.as_ref());
    }
}

#[no_mangle]
pub extern "C" fn context_get_font(context: c_longlong) -> *const c_char {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        let font = context.font();
        CString::new(font).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn context_set_global_alpha(context: c_longlong, alpha: c_float) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_global_alpha(alpha)
    }
}

#[no_mangle]
pub extern "C" fn context_get_global_alpha(context: c_longlong) -> c_float {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.global_alpha()
    }
}

#[no_mangle]
pub extern "C" fn context_set_global_composite_operation(
    context: c_longlong,
    operation: CompositeOperationType,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_global_composite_operation(operation);
    }
}

#[no_mangle]
pub extern "C" fn context_get_global_composite_operation(
    context: c_longlong,
) -> CompositeOperationType {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.global_composite_operation()
    }
}

#[no_mangle]
pub extern "C" fn context_set_image_smoothing_enabled(
    context: c_longlong,
    enabled: bool,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_image_smoothing_enabled(enabled);
    }
}

#[no_mangle]
pub extern "C" fn context_get_image_smoothing_enabled(context: c_longlong) -> bool {
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.get_image_smoothing_enabled()
    }
}

#[no_mangle]
pub extern "C" fn context_set_image_smoothing_quality(
    context: c_longlong,
    quality: ImageSmoothingQuality,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_image_smoothing_quality(quality);
    }
}

#[no_mangle]
pub extern "C" fn context_get_image_smoothing_quality(context: c_longlong) -> ImageSmoothingQuality {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.get_image_smoothing_quality()
    }
}

#[no_mangle]
pub extern "C" fn context_set_line_cap(context: c_longlong, cap: LineCap) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_line_cap(cap)
    }
}

#[no_mangle]
pub extern "C" fn context_get_line_cap(context: c_longlong) -> LineCap {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.line_cap()
    }
}

#[no_mangle]
pub extern "C" fn context_set_line_dash_offset(context: c_longlong, offset: c_float) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_line_dash_offset(offset)
    }
}

#[no_mangle]
pub extern "C" fn context_get_line_dash_offset(context: c_longlong) -> c_float {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.line_dash_offset()
    }
}

#[no_mangle]
pub extern "C" fn context_set_line_join(context: c_longlong, join: LineJoin) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_line_join(join)
    }
}

#[no_mangle]
pub extern "C" fn context_get_line_join(context: c_longlong) -> LineJoin {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.line_join()
    }
}

#[no_mangle]
pub extern "C" fn context_set_line_width(context: c_longlong, width: c_float) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_line_width(width)
    }
}

#[no_mangle]
pub extern "C" fn context_get_line_width(context: c_longlong) -> c_float {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.line_width()
    }
}

#[no_mangle]
pub extern "C" fn context_set_miter_limit(context: c_longlong, limit: c_float) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_miter_limit(limit)
    }
}

#[no_mangle]
pub extern "C" fn context_get_miter_limit(context: c_longlong) -> c_float {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.miter_limit()
    }
}

#[no_mangle]
pub extern "C" fn context_set_shadow_blur(context: c_longlong, blur: c_float) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_shadow_blur(blur)
    }
}

#[no_mangle]
pub extern "C" fn context_get_shadow_blur(context: c_longlong) -> c_float {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.shadow_blur()
    }
}

#[no_mangle]
pub extern "C" fn context_set_shadow_color(context: c_longlong, r: u8, g: u8, b: u8, a: u8) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_shadow_color(skia_safe::Color::from_argb(a, r, g, b))
    }
}

#[no_mangle]
pub extern "C" fn context_set_shadow_color_string(context: c_longlong, color: *const c_char) {
    unsafe {
        if context == 0 || color.is_null() {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let color = CStr::from_ptr(color).to_string_lossy();
        if let Ok(color) = color.as_ref().parse::<css_color_parser::Color>() {
            context.set_shadow_color(skia_safe::Color::from_argb((color.a * 255.0) as u8, color.r, color.g, color.b))
        }
    }
}

#[no_mangle]
pub extern "C" fn context_get_shadow_color(context: c_longlong) -> *const c_char {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        CString::new(to_parsed_color(context.shadow_color()))
            .unwrap()
            .into_raw()
    }
}

#[no_mangle]
pub extern "C" fn context_set_shadow_offset_x(context: c_longlong, x: c_float) {
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_shadow_offset_x(x)
    }
}

#[no_mangle]
pub extern "C" fn context_get_shadow_offset_x(context: c_longlong) -> c_float {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.shadow_offset_x()
    }
}

#[no_mangle]
pub extern "C" fn context_set_shadow_offset_y(context: c_longlong, y: c_float) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_shadow_offset_y(y)
    }
}

#[no_mangle]
pub extern "C" fn context_get_shadow_offset_y(context: c_longlong) -> c_float {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.shadow_offset_y()
    }
}

#[no_mangle]
pub extern "C" fn context_set_stroke_style(context: c_longlong, style: c_longlong) {
    unsafe {
        if context == 0 || style == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let style: *const PaintStyle = style as _;
        let style = &*style;
        context.set_stroke_style(style.clone())
    }
}

#[no_mangle]
pub extern "C" fn context_get_stroke_style(context: c_longlong) -> *mut PaintStyleValue {
    if context == 0 {
        return std::ptr::null_mut();
    }
    unsafe {
        let context: *mut Context = context as _;
        let context = &mut *context;
        let stroke_style = context.stroke_style().clone();
        Box::into_raw(
            Box::new(
                match stroke_style {
                    PaintStyle::Color(_) => PaintStyleValue::new(stroke_style, PaintStyleValueType::PaintStyleValueTypeColor),
                    PaintStyle::Gradient(_) => PaintStyleValue::new(stroke_style, PaintStyleValueType::PaintStyleValueTypeGradient),
                    PaintStyle::Pattern(_) => PaintStyleValue::new(stroke_style, PaintStyleValueType::PaintStyleValueTypePattern)
                }
            )
        )
    }
}

#[no_mangle]
pub extern "C" fn context_set_text_align(context: c_longlong, align: TextAlign) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_text_align(align)
    }
}

#[no_mangle]
pub extern "C" fn context_get_text_align(context: c_longlong) -> TextAlign {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.text_align()
    }
}

#[no_mangle]
pub extern "C" fn context_set_text_baseline(context: c_longlong, baseline: TextBaseLine) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_text_baseline(baseline);
    }
}

#[no_mangle]
pub extern "C" fn context_get_text_baseline(context: c_longlong) -> TextBaseLine {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        context.text_baseline()
    }
}

#[no_mangle]
pub extern "C" fn context_arc(
    context: c_longlong,
    x: c_float,
    y: c_float,
    radius: c_float,
    start_angle: c_float,
    end_angle: c_float,
    anti_clockwise: bool,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.arc(x, y, radius, start_angle, end_angle, anti_clockwise)
    }
}

#[no_mangle]
pub extern "C" fn context_arc_to(
    context: c_longlong,
    x1: c_float,
    y1: c_float,
    x2: c_float,
    y2: c_float,
    radius: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.arc_to(x1, y1, x2, y2, radius)
    }
}

#[no_mangle]
pub extern "C" fn context_begin_path(context: c_longlong) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.begin_path()
    }
}

#[no_mangle]
pub extern "C" fn context_bezier_curve_to(
    context: c_longlong,
    cp1x: c_float,
    cp1y: c_float,
    cp2x: c_float,
    cp2y: c_float,
    x: c_float,
    y: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
    }
}

#[no_mangle]
pub extern "C" fn context_clear_rect(
    context: c_longlong,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.clear_rect(x, y, width, height)
    }
}

#[no_mangle]
pub extern "C" fn context_clip(context: c_longlong, path: c_longlong, rule: FillRule) {
    unsafe {
        if context == 0 || path == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let path: *mut Path = path as _;
        context.clip(Some(&mut *path), Some(rule))
    }
}

#[no_mangle]
pub extern "C" fn context_clip_rule(context: c_longlong, rule: FillRule) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.clip(None, Some(rule))
    }
}

#[no_mangle]
pub extern "C" fn context_close_path(context: c_longlong) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.close_path()
    }
}


#[no_mangle]
pub extern "C" fn context_create_image_data(width: c_int, height: c_int) -> c_longlong {
    unsafe { Box::into_raw(Box::new(Context::create_image_data(width, height))) as c_longlong }
}

#[no_mangle]
pub extern "C" fn context_create_linear_gradient(
    context: c_longlong,
    x0: c_float,
    y0: c_float,
    x1: c_float,
    y1: c_float,
) -> c_longlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        Box::into_raw(Box::new(
            PaintStyle::Gradient(context.create_linear_gradient(x0, y0, x1, y1))
        )) as c_longlong
    }
}

#[no_mangle]
pub extern "C" fn context_create_pattern(
    context: c_longlong,
    image_data: *const u8,
    image_len: usize,
    width: c_int,
    height: c_int,
    repetition: Repetition,
) -> c_longlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Some(image) = to_image(image_data, image_len, width, height) {
            return Box::into_raw(Box::new(
                PaintStyle::Pattern(context.create_pattern(image, repetition))
            )) as c_longlong;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn context_create_pattern_asset(
    context: c_longlong,
    asset: c_longlong,
    repetition: Repetition,
) -> c_longlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let bytes = asset.rgba_internal_bytes();
        if let Some(image) = from_image_slice(bytes.as_slice(), asset.width() as i32, asset.height() as i32) {
            return Box::into_raw(Box::new(
                PaintStyle::Pattern(context.create_pattern(image, repetition))
            )) as c_longlong;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn context_create_pattern_encoded(
    context: c_longlong,
    image_data: *const u8,
    image_len: usize,
    repetition: Repetition,
) -> c_longlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if let Some(image) = to_image_encoded(image_data, image_len) {
            return Box::into_raw(Box::new(PaintStyle::Pattern(context.create_pattern(image, repetition)))) as c_longlong;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn context_create_radial_gradient(
    context: c_longlong,
    x0: c_float,
    y0: c_float,
    r0: c_float,
    x1: c_float,
    y1: c_float,
    r1: c_float,
) -> c_longlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        Box::into_raw(Box::new(
            PaintStyle::Gradient(context.create_radial_gradient(x0, y0, r0, x1, y1, r1))
        )) as c_longlong
    }
}

#[no_mangle]
pub extern "C" fn context_draw_image_dx_dy(
    context: c_longlong,
    image_data: *const u8,
    image_len: usize,
    width: c_float,
    height: c_float,
    dx: c_float,
    dy: c_float,
) {
    context_draw_image(
        context, image_data, image_len, width,
        height, 0.0, 0.0, width, height,
        dx, dy, width, height,
    );
}


#[no_mangle]
pub extern "C" fn context_draw_image_dx_dy_dw_dh(
    context: c_longlong,
    image_data: *const u8,
    image_len: usize,
    width: c_float,
    height: c_float,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) {
    context_draw_image(
        context, image_data, image_len,
        width, height,
        0.0, 0.0, width, height,
        dx, dy, d_width, d_height,
    )
}


#[no_mangle]
pub extern "C" fn context_draw_image(
    context: c_longlong,
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
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
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
pub extern "C" fn context_draw_image_encoded_dx_dy(
    context: c_longlong,
    image_data: *const u8,
    image_len: usize,
    dx: c_float,
    dy: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
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
pub extern "C" fn context_draw_image_encoded_dx_dy_dw_dh(
    context: c_longlong,
    image_data: *const u8,
    image_len: usize,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
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
pub extern "C" fn context_draw_image_encoded(
    context: c_longlong,
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
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
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
pub extern "C" fn context_draw_image_dx_dy_asset(
    context: c_longlong,
    asset: c_longlong,
    dx: c_float,
    dy: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let ctx_ptr = context;
        let asset_ptr = asset;
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let width = asset.width() as f32;
        let height = asset.height() as f32;
        context_draw_image_asset(
            ctx_ptr, asset_ptr, 0.0, 0.0, width, height,
            dx, dy, width, height,
        );
    }
}


#[no_mangle]
pub extern "C" fn context_draw_image_dx_dy_dw_dh_asset(
    context: c_longlong,
    asset: c_longlong,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let ctx_ptr = context;
        let asset_ptr = asset;
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let width = asset.width() as f32;
        let height = asset.height() as f32;
        context_draw_image_asset(
            ctx_ptr, asset_ptr, 0.0, 0.0, width, height,
            dx, dy, d_width, d_height,
        );
    }
}

#[no_mangle]
pub extern "C" fn context_draw_image_asset(
    context: c_longlong,
    asset: c_longlong,
    sx: c_float,
    sy: c_float,
    s_width: c_float,
    s_height: c_float,
    dx: c_float,
    dy: c_float,
    d_width: c_float,
    d_height: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let bytes = asset.rgba_internal_bytes();
        if let Some(image) = from_image_slice(bytes.as_slice(), asset.width() as i32, asset.height() as i32) {
            context.draw_image(
                &image,
                Rect::from_xywh(sx, sy, s_width, s_height),
                Rect::from_xywh(dx, dy, d_width, d_height),
            )
        }
    }
}

#[no_mangle]
pub extern "C" fn context_ellipse(
    context: c_longlong,
    x: c_float,
    y: c_float,
    radius_x: c_float,
    radius_y: c_float,
    rotation: c_float,
    start_angle: c_float,
    end_angle: c_float,
    anticlockwise: bool,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
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
pub extern "C" fn context_fill(context: c_longlong, path: c_longlong, rule: FillRule) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if path == 0 {
            context.fill(None, rule)
        } else {
            let path: *mut Path = path as _;
            context.fill(Some(&mut *path), rule)
        }
    }
}

#[no_mangle]
pub extern "C" fn context_fill_rect(
    context: c_longlong,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let rect = Rect::from_xywh(x, y, width, height);
        context.fill_rect(&rect);
    }
}

#[no_mangle]
pub extern "C" fn context_fill_text(
    context: c_longlong,
    text: *const c_char,
    x: c_float,
    y: c_float,
    width: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let txt = CStr::from_ptr(text).to_string_lossy();
        context.fill_text(txt.as_ref(), x, y, width)
    }
}

#[no_mangle]
pub extern "C" fn context_get_image_data(
    context: c_longlong,
    sx: c_float,
    sy: c_float,
    sw: c_float,
    sh: c_float,
) -> c_longlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        Box::into_raw(Box::new(context.get_image_data(sx, sy, sw, sh))) as c_longlong
    }
}

#[no_mangle]
pub extern "C" fn context_get_line_dash(context: c_longlong) -> *mut F32Array {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
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
pub extern "C" fn context_get_transform(context: c_longlong) -> c_longlong {
    unsafe {
        if context == 0 {
            return 0;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let matrix = Matrix {
            matrix: M44::from(context.get_transform().clone()),
        };
        Box::into_raw(Box::new(matrix)) as c_longlong
    }
}

#[no_mangle]
pub extern "C" fn context_is_point_in_path(
    context: c_longlong,
    path: c_longlong,
    x: c_float,
    y: c_float,
    rule: FillRule,
) -> bool {
    unsafe {
        if context == 0 {
            return false;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if path == 0 {
            context.is_point_in_path(None, x, y, rule)
        } else {
            let path: *mut Path = path as _;
            context.is_point_in_path(Some(&*path), x, y, rule)
        }
    }
}

#[no_mangle]
pub extern "C" fn context_is_point_in_stroke(
    context: c_longlong,
    path: c_longlong,
    x: c_float,
    y: c_float,
) -> bool {
    unsafe {
        if context == 0 {
            return false;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if path == 0 {
            context.is_point_in_stroke(None, x, y)
        } else {
            let path: *mut Path = path as _;
            context.is_point_in_stroke(Some(&*path), x, y)
        }
    }
}

#[no_mangle]
pub extern "C" fn context_line_to(context: c_longlong, x: c_float, y: c_float) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.line_to(x, y)
    }
}

#[no_mangle]
pub extern "C" fn context_measure_text(context: c_longlong, text: *const c_char) -> c_longlong {
    unsafe {
        let context: *const Context = context as _;
        let context = &*context;
        let text = CStr::from_ptr(text).to_string_lossy();
        Box::into_raw(Box::new(context.measure_text(text.as_ref()))) as c_longlong
    }
}

#[no_mangle]
pub extern "C" fn context_move_to(context: c_longlong, x: c_float, y: c_float) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.move_to(x, y)
    }
}

#[no_mangle]
pub extern "C" fn context_put_image_data(
    context: c_longlong,
    image_data: c_longlong,
    dx: c_float,
    dy: c_float,
    dirty_x: c_float,
    dirty_y: c_float,
    dirty_width: c_float,
    dirty_height: c_float,
) {
    unsafe {
        if context == 0 || image_data == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let image_data: *const ImageData = image_data as _;
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
pub extern "C" fn context_quadratic_curve_to(
    context: c_longlong,
    cpx: c_float,
    cpy: c_float,
    x: c_float,
    y: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.quadratic_curve_to(cpx, cpy, x, y)
    }
}

#[no_mangle]
pub extern "C" fn context_rect(
    context: c_longlong,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.rect(x, y, width, height)
    }
}

#[no_mangle]
pub extern "C" fn context_reset_transform(context: c_longlong) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.reset_transform()
    }
}

#[no_mangle]
pub extern "C" fn context_restore(context: c_longlong) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.restore()
    }
}

#[no_mangle]
pub extern "C" fn context_rotate(context: c_longlong, angle: c_float) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.rotate(angle)
    }
}

#[no_mangle]
pub extern "C" fn context_save(context: c_longlong) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.save()
    }
}

#[no_mangle]
pub extern "C" fn context_scale(context: c_longlong, x: c_float, y: c_float) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.scale(x, y)
    }
}

#[no_mangle]
pub extern "C" fn context_set_line_dash(context: c_longlong, data: *const c_float, data_length: usize) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_line_dash(std::slice::from_raw_parts(data, data_length))
    }
}

#[no_mangle]
pub extern "C" fn context_set_transform(
    context: c_longlong,
    a: c_float,
    b: c_float,
    c: c_float,
    d: c_float,
    e: c_float,
    f: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.set_transform(a, b, c, d, e, f)
    }
}

#[no_mangle]
pub extern "C" fn context_set_transform_matrix(context: c_longlong, matrix: c_longlong) {
    unsafe {
        if context == 0 || matrix == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let matrix: *const Matrix = matrix as _;
        let matrix = &*matrix;
        context.set_transform_matrix(&matrix.matrix.to_m33())
    }
}

#[no_mangle]
pub extern "C" fn context_stroke(context: c_longlong, path: c_longlong) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        if path == 0 {
            context.stroke(None)
        } else {
            let path: *mut Path = path as _;
            context.stroke(Some(&mut *path))
        }
    }
}

#[no_mangle]
pub extern "C" fn context_stroke_rect(
    context: c_longlong,
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let rect = Rect::from_xywh(x, y, width, height);
        context.stroke_rect(&rect);
    }
}

#[no_mangle]
pub extern "C" fn context_stroke_text(
    context: c_longlong,
    text: *const c_char,
    x: c_float,
    y: c_float,
    width: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        let txt = CStr::from_ptr(text).to_string_lossy();
        context.stroke_text(txt.as_ref(), x, y, width)
    }
}

#[no_mangle]
pub extern "C" fn context_transform(
    context: c_longlong,
    a: c_float,
    b: c_float,
    c: c_float,
    d: c_float,
    e: c_float,
    f: c_float,
) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.transform(a, b, c, d, e, f)
    }
}

#[no_mangle]
pub extern "C" fn context_translate(context: c_longlong, x: c_float, y: c_float) {
    unsafe {
        if context == 0 {
            return;
        }
        let context: *mut Context = context as _;
        let context = &mut *context;
        context.translate(x, y)
    }
}
