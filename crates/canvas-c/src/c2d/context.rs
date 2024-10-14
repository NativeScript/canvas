use std::cmp::PartialEq;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

use canvas_2d::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_2d::context::fill_and_stroke_styles::paint::paint_style_set_color_with_string;
use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_2d::context::image_smoothing::ImageSmoothingQuality;
use canvas_2d::context::line_styles::line_cap::LineCap;
use canvas_2d::context::line_styles::line_join::LineJoin;
use canvas_2d::context::text_styles::text_align::TextAlign;
use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::context::Context;
use canvas_2d::utils::color::{parse_color, to_parsed_color};
use canvas_2d::utils::image::{
    from_bitmap_slice, from_image_slice, from_image_slice_encoded,
};
use canvas_webgl::utils::gl::bytes_per_pixel;

use crate::buffers::{F32Buffer, U8Buffer};
use crate::c2d::enums::{CanvasFillRule, CanvasRepetition};
use crate::c2d::image_data::ImageData;
use crate::c2d::matrix::Matrix;
use crate::c2d::paint::{PaintStyle, PaintStyleType};
use crate::c2d::path::Path;
use crate::c2d::text_base_line::TextBaseLine;
use crate::c2d::text_metrics::TextMetrics;
use crate::image_asset::ImageAsset;
use crate::webgl::WebGLState;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Engine {
    CPU,
    GL,
    Vulkan,
    Metal,
}


#[allow(dead_code)]
pub struct CanvasRenderingContext2D {
    pub(crate) context: Context,
    alpha: bool,
    engine: Engine,
}

impl CanvasRenderingContext2D {
    pub fn get_context(&self) -> &Context {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_release(value: *mut CanvasRenderingContext2D) {
    if value.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(value)) };
}

fn to_data_url(context: &mut CanvasRenderingContext2D, format: &str, quality: u32) -> String {
    context.context.as_data_url(format, quality)
}

#[cfg(feature = "gl")]
pub fn resize_gl(context: &mut CanvasRenderingContext2D, width: f32, height: f32) {
    let alpha = context.alpha;
    context.make_current();
    let context = &mut context.context;
    let density = context.get_surface_data().scale();
    let ppi = context.get_surface_data().ppi();

    if width.floor() == context.get_surface_data().width().floor() && height.floor() == context.get_surface_data().height().floor() {
        return;
    }

    let mut fb = [0];
    context.clear_rect(0., 0., width, height);
    context.flush_and_render_to_surface();

    unsafe {
        gl_bindings::Viewport(0, 0, width as i32, height as i32);
        gl_bindings::GetIntegerv(gl_bindings::FRAMEBUFFER_BINDING, fb.as_mut_ptr());
    }

    Context::resize_gl(context, width, height, density, fb[0], 0, alpha, ppi)
}

#[cfg(feature = "vulkan")]
pub fn resize_vulkan(context: &mut CanvasRenderingContext2D, width: f32, height: f32) {
    let alpha = context.alpha;
    let context = &mut context.context;
    Context::resize_vulkan(context, width, height, alpha)
}

#[cfg(feature = "metal")]
pub fn resize_metal(context: &mut CanvasRenderingContext2D, width: f32, height: f32) {
    let context = &mut context.context;
    Context::resize_metal(context, width, height);
}

pub fn resize(context: &mut CanvasRenderingContext2D, width: f32, height: f32) {
    #[cfg(feature = "gl")]
    {
        if context.engine == Engine::GL {
            // #[cfg(target_os = "android")]
            // {
            //     context.make_current();
            //     context.gl_context.resize_window_surface(width.floor() as i32, height.floor() as i32);
            // }
            resize_gl(context, width, height);
            return;
        }
    }

    #[cfg(feature = "vulkan")]
    {
        if context.engine == Engine::Vulkan {
            resize_vulkan(context, width, height);
            return;
        }
    }

    #[cfg(feature = "metal")]
    {
        if context.engine == Engine::Metal {
            resize_metal(context, width, height);
            return;
        }
    }

    let alpha = context.alpha;
    let context = &mut context.context;
    let density = context.get_surface_data().scale();
    let ppi = context.get_surface_data().ppi();
    Context::resize(context, width, height, density, alpha, ppi);
}

impl CanvasRenderingContext2D {
    pub fn new(context: Context, alpha: bool) -> Self {
        Self {
            context,
            alpha,
            engine: Engine::CPU,
        }
    }

    #[cfg(feature = "gl")]
    pub fn new_gl(context: Context, alpha: bool) -> Self {
        Self {
            context,
            alpha,
            engine: Engine::GL,
        }
    }


    #[cfg(feature = "vulkan")]
    pub fn new_vulkan(context: Context, alpha: bool) -> Self {
        Self {
            context,
            alpha,
            engine: Engine::Vulkan,
        }
    }


    #[cfg(feature = "metal")]
    pub fn new_metal(context: Context, alpha: bool) -> Self {
        Self {
            context,
            alpha,
            engine: Engine::Metal,
        }
    }


    pub fn render(&mut self) {
        #[cfg(feature = "gl")]
        if let Some(context) = self.context.gl_context.as_ref() {
            context.make_current();
        }

        let mut flush = true;

        // metal will execute the flush_and_render_to_surface in the draw call
        #[cfg(feature = "metal")]{
            if self.engine == Engine::Metal {
                flush = false;
            }
        }

        #[cfg(feature = "vulkan")]
        if self.engine == Engine::Vulkan {
            flush = false;
        }

        {
            if flush {
                self.context.flush_and_render_to_surface();
            }
        }


        #[cfg(feature = "metal")]
        if let Some(context) = self.context.metal_context.as_mut() {
            context.present();
        }

        #[cfg(feature = "gl")]
        if let Some(context) = self.context.gl_context.as_ref() {
            context.swap_buffers();
        }

        #[cfg(feature = "vulkan")]
        if self.engine == Engine::Vulkan {
            self.context.flush()
        }

        #[cfg(feature = "vulkan")]
        if let Some(vulkan) = self.context.vulkan_context.as_mut() {
            vulkan.present();
        }

        #[cfg(feature = "vulkan")]
        if self.engine == Engine::Vulkan {
            self.context.replace_backend_texture()
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        resize(self, width, height);
    }

    #[cfg(feature = "gl")]
    pub fn make_current(&self) -> bool {
        if let Some(ref context) = self.context.gl_context {
            return context.make_current();
        }
        false
    }


    #[cfg(feature = "gl")]
    pub fn swap_buffers(&self) -> bool {
        if let Some(ref context) = self.context.gl_context {
            return context.swap_buffers();
        }
        false
    }


    pub fn remove_if_current(&self) {
        if let Some(ref context) = self.context.gl_context {
            context.remove_if_current();
        }
    }
}

/* Raf */
#[cfg(any(target_os = "android", target_os = "ios"))]
#[no_mangle]
pub extern "C" fn canvas_native_raf_create(
    callback: isize,
    on_frame_callback: Option<extern "C" fn(callback: isize, ts: i64)>,
) -> *mut crate::Raf {
    Box::into_raw(Box::new(crate::Raf(crate::raf::Raf::new(Some(Box::new(
        move |ts| {
            if let Some(on_frame_callback) = on_frame_callback {
                on_frame_callback(callback, ts)
            }
        },
    ))))))
}


#[cfg(any(target_os = "android", target_os = "ios"))]
#[no_mangle]
pub extern "C" fn canvas_native_raf_release(value: *mut crate::Raf) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { std::sync::Arc::decrement_strong_count(value) };
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[no_mangle]
pub extern "C" fn canvas_native_raf_start(raf: *mut crate::Raf) {
    if raf.is_null() {
        return;
    }
    let raf = unsafe { &*raf };
    raf.0.start();
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[no_mangle]
pub extern "C" fn canvas_native_raf_stop(raf: *mut crate::Raf) {
    if raf.is_null() {
        return;
    }
    let raf = unsafe { &*raf };
    raf.0.stop()
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[no_mangle]
pub extern "C" fn canvas_native_raf_get_started(raf: *const crate::Raf) -> bool {
    if raf.is_null() {
        return false;
    }
    let raf = unsafe { &*raf };
    raf.0.started()
}
/* Raf */

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
        context: Context::new(
            width,
            height,
            density,
            alpha,
            font_color,
            ppi,
            TextDirection::from(direction),
        ),
        alpha,
        engine: Engine::CPU,
    }))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_gl(
    view: *mut c_void,
    width: f32,
    height: f32,
    density: f32,
    alpha: bool,
    font_color: i32,
    ppi: f32,
    direction: u32,
) -> *mut CanvasRenderingContext2D {
    Box::into_raw(Box::new(CanvasRenderingContext2D {
        context: Context::new_gl(
            view,
            width,
            height,
            density,
            alpha,
            font_color,
            ppi,
            TextDirection::from(direction),
        ),
        alpha,
        engine: Engine::GL,
    }))
}

#[cfg(feature = "webgl")]
#[no_mangle]
pub extern "C" fn canvas_native_context_create_with_pointer(
    pointer: i64,
) -> *mut CanvasRenderingContext2D {
    if pointer == 0 {
        return std::ptr::null_mut();
    }
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
    let context = Context::new_gl(
        std::ptr::null_mut(),
        width,
        height,
        density,
        alpha,
        font_color,
        ppi,
        TextDirection::from(direction),
    );

    Box::into_raw(Box::new(CanvasRenderingContext2D {
        context,
        alpha,
        engine: Engine::GL,
    }))
}

/*
fn canvas_native_webgl_create_no_window_internal(
    width: i32,
    height: i32,
    version: WebGLVersion,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: WebGLPowerPreference,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    is_canvas: bool,
) -> WebGLState {
    let mut attrs = canvas_core::context_attributes::ContextAttributes::new(
        alpha,
        antialias,
        depth,
        fail_if_major_performance_caveat,
        power_preference.into(),
        premultiplied_alpha,
        preserve_drawing_buffer,
        stencil,
        desynchronized,
        xr_compatible,
        is_canvas,
    );

    let ctx = canvas_core::gl::GLContext::create_offscreen_context(&mut attrs, width, height)
        .unwrap_or_default();

    canvas_webgl::prelude::WebGLState::new_with_context(ctx, version)
}

*/

#[no_mangle]
pub extern "C" fn canvas_native_context_get_filter(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = context.context.get_filter().to_string();

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
        .context
        .set_filter(filter.to_string_lossy().as_ref())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_font(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = context.context.font().to_string();
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
    context.context.set_font(font.to_string_lossy().as_ref());
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_letter_spacing(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = context.context.get_letter_spacing().to_string();
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
        .context
        .set_letter_spacing(spacing.to_string_lossy().as_ref());
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_word_spacing(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = context.context.get_word_spacing().to_string();
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
        .context
        .set_word_spacing(spacing.to_string_lossy().as_ref());
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_global_alpha(
    context: *mut CanvasRenderingContext2D,
) -> f32 {
    let context = unsafe { &*context };
    context.context.global_alpha()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_global_alpha(
    context: *mut CanvasRenderingContext2D,
    alpha: f32,
) {
    let context = unsafe { &mut *context };
    context.context.set_global_alpha(alpha);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_image_smoothing_enabled(
    context: *const CanvasRenderingContext2D,
) -> bool {
    let context = unsafe { &*context };
    context.context.get_image_smoothing_enabled()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_image_smoothing_enabled(
    context: *mut CanvasRenderingContext2D,
    enabled: bool,
) {
    let context = unsafe { &mut *context };
    context.context.set_image_smoothing_enabled(enabled)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_image_smoothing_quality(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = match context.context.get_image_smoothing_quality() {
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
            .context
            .set_image_smoothing_quality(ImageSmoothingQuality::Low),
        "medium" => context
            .context
            .set_image_smoothing_quality(ImageSmoothingQuality::Medium),
        "high" => context
            .context
            .set_image_smoothing_quality(ImageSmoothingQuality::High),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_join(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = match context.context.line_join() {
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
        "bevel" => context.context.set_line_join(LineJoin::JoinBevel),
        "miter" => context.context.set_line_join(LineJoin::JoinMiter),
        "round" => context.context.set_line_join(LineJoin::JoinRound),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_cap(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = match context.context.line_cap() {
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
        "round" => context.context.set_line_cap(LineCap::CapRound),
        "butt" => context.context.set_line_cap(LineCap::CapButt),
        "square" => context.context.set_line_cap(LineCap::CapSquare),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_miter_limit(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    let context = unsafe { &*context };
    context.context.miter_limit()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_miter_limit(
    context: *mut CanvasRenderingContext2D,
    limit: f32,
) {
    let context = unsafe { &mut *context };
    context.context.set_miter_limit(limit);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_color(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let value = context.context.shadow_color();
    let ret = to_parsed_color(value);
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_color_buf(
    context: *const CanvasRenderingContext2D,
) -> *mut U8Buffer {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let value = context.context.shadow_color();
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
    context.context.shadow_color_rgba(r, g, b, a);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_color(
    context: *mut CanvasRenderingContext2D,
    color: *const c_char,
) {
    assert!(!context.is_null());
    assert!(!color.is_null());
    let context = unsafe { &mut *context };
    let color = unsafe { CStr::from_ptr(color) };
    if let Some(color) = parse_color(color.to_string_lossy().as_ref()) {
        context.context.set_shadow_color(color);
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
    context.context.set_shadow_color_rgba(r, g, b, a);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_blur(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.context.shadow_blur()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_blur(
    context: *mut CanvasRenderingContext2D,
    blur: f32,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.context.set_shadow_blur(blur)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_offset_x(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.context.shadow_offset_x()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_offset_x(
    context: *mut CanvasRenderingContext2D,
    x: f32,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.context.set_shadow_offset_x(x)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_shadow_offset_y(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.context.shadow_offset_y()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_shadow_offset_y(
    context: *mut CanvasRenderingContext2D,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context.context.set_shadow_offset_y(y)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_text_align(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = match context.context.text_align() {
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
        "start" => context.context.set_text_align(TextAlign::START),
        "left" => context.context.set_text_align(TextAlign::LEFT),
        "center" => context.context.set_text_align(TextAlign::CENTER),
        "right" => context.context.set_text_align(TextAlign::RIGHT),
        "end" => context.context.set_text_align(TextAlign::END),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_text_baseline_str(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = match context.context.text_baseline() {
        canvas_2d::context::text_styles::text_baseline::TextBaseLine::ALPHABETIC => "alphabetic",
        canvas_2d::context::text_styles::text_baseline::TextBaseLine::BOTTOM => "bottom",
        canvas_2d::context::text_styles::text_baseline::TextBaseLine::HANGING => "hanging",
        canvas_2d::context::text_styles::text_baseline::TextBaseLine::IDEOGRAPHIC => "ideographic",
        canvas_2d::context::text_styles::text_baseline::TextBaseLine::MIDDLE => "middle",
        canvas_2d::context::text_styles::text_baseline::TextBaseLine::TOP => "top",
    };
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_text_baseline_str(
    context: *mut CanvasRenderingContext2D,
    baseline: *const c_char,
) {
    if baseline.is_null() {
        return;
    }
    let context = unsafe { &mut *context };
    let baseline = unsafe { CStr::from_ptr(baseline) };
    match baseline.to_string_lossy().as_ref() {
        "alphabetic" => context.context.set_text_baseline(
            canvas_2d::context::text_styles::text_baseline::TextBaseLine::ALPHABETIC,
        ),
        "bottom" => context.context.set_text_baseline(
            canvas_2d::context::text_styles::text_baseline::TextBaseLine::BOTTOM,
        ),
        "hanging" => context.context.set_text_baseline(
            canvas_2d::context::text_styles::text_baseline::TextBaseLine::HANGING,
        ),
        "ideographic" => context.context.set_text_baseline(
            canvas_2d::context::text_styles::text_baseline::TextBaseLine::IDEOGRAPHIC,
        ),
        "middle" => context.context.set_text_baseline(
            canvas_2d::context::text_styles::text_baseline::TextBaseLine::MIDDLE,
        ),
        "top" => context
            .context
            .set_text_baseline(canvas_2d::context::text_styles::text_baseline::TextBaseLine::TOP),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_text_baseline(
    context: *mut CanvasRenderingContext2D,
    baseline: TextBaseLine,
) {
    let context = unsafe { &mut *context };
    context.context.set_text_baseline(baseline.into());
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_text_baseline(
    context: *const CanvasRenderingContext2D,
) -> TextBaseLine {
    let context = unsafe { &*context };
    context.context.text_baseline().into()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_global_composition(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    let context = unsafe { &*context };
    let ret = context.context.global_composite_operation().to_str();
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
        context.context.set_global_composite_operation(composition)
    }
}

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

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_get_color_string(
    color: *const PaintStyle,
) -> *const c_char {
    if color.is_null() {
        return std::ptr::null();
    }
    let color = unsafe { &*color };
    match &color.0 {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(color) => {
            CString::new(to_parsed_color(color.clone()))
                .unwrap()
                .into_raw()
        }
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_paint_style_get_current_stroke_color_string(
    context: *const CanvasRenderingContext2D,
) -> *const c_char {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    match context.context.stroke_style() {
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
    context: *mut CanvasRenderingContext2D,
) -> *mut U8Buffer {
    let context = unsafe { &*context };
    let ret = match context.context.stroke_style() {
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
    match context.context.stroke_style() {
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
    match context.context.fill_style() {
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
    match context.context.fill_style() {
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
    context: *const CanvasRenderingContext2D,
) -> *mut U8Buffer {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let ret = match context.context.fill_style() {
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
    context: *const CanvasRenderingContext2D,
) -> PaintStyleType {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    return match context.context.fill_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(_) => {
            PaintStyleType::Color
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            PaintStyleType::Gradient
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            PaintStyleType::Pattern
        }
    };
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_current_stroke_style_type(
    context: *const CanvasRenderingContext2D,
) -> PaintStyleType {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    return match context.context.fill_style() {
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Color(_) => {
            PaintStyleType::Color
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(_) => {
            PaintStyleType::Gradient
        }
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(_) => {
            PaintStyleType::Pattern
        }
    };
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_fill_style(
    context: *const CanvasRenderingContext2D,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    Box::into_raw(Box::new(PaintStyle(context.context.fill_style().clone())))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_fill_style(
    context: *mut CanvasRenderingContext2D,
    style: *const PaintStyle,
) {
    assert!(!context.is_null());
    assert!(!style.is_null());
    let context = unsafe { &mut *context };
    let style = unsafe { &*style };
    context.context.set_fill_style(style.0.clone())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_stroke_style(
    context: *const CanvasRenderingContext2D,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    Box::into_raw(Box::new(PaintStyle(context.context.stroke_style().clone())))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_stroke_style(
    context: *mut CanvasRenderingContext2D,
    style: *const PaintStyle,
) {
    assert!(!context.is_null());
    assert!(!style.is_null());
    let context = unsafe { &mut *context };
    let style = unsafe { &*style };
    context.context.set_stroke_style(style.0.clone())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_width(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.context.line_width()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_width(
    context: *mut CanvasRenderingContext2D,
    width: f32,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.context.set_line_width(width);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_dash_offset(
    context: *const CanvasRenderingContext2D,
) -> f32 {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    context.context.line_dash_offset()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_line_dash_offset(
    context: *mut CanvasRenderingContext2D,
    offset: f32,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.context.set_line_dash_offset(offset)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_line_dash(
    context: *const CanvasRenderingContext2D,
) -> *mut F32Buffer {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    Box::into_raw(Box::new(F32Buffer::from(
        context.context.line_dash().to_vec(),
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
    context.context.set_line_dash(dash)
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
        .context
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
    context.context.arc_to(x1, y1, x2, y2, radius)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_begin_path(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    context.context.begin_path()
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
        .context
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
    #[cfg(feature = "gl")]{
        context.make_current();
    }
    context.context.clear_rect(x, y, width, height);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_clip(
    context: *mut CanvasRenderingContext2D,
    path: *mut Path,
    rule: CanvasFillRule,
) {
    assert!(!context.is_null());
    let path = unsafe { &mut *path };
    let context = unsafe { &mut *context };
    context.context.clip(Some(&mut path.0), Some(rule.into()));
}

#[no_mangle]
pub extern "C" fn canvas_native_context_clip_rule(
    context: *mut CanvasRenderingContext2D,
    rule: CanvasFillRule,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.context.clip(None, Some(rule.into()));
}

#[no_mangle]
pub extern "C" fn canvas_native_context_close_path(context: *mut CanvasRenderingContext2D) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.context.close_path()
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
    context: *const CanvasRenderingContext2D,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    Box::into_raw(Box::new(PaintStyle(
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(
            context.context.create_linear_gradient(x0, y0, x1, y1),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_conic_gradient(
    context: *const CanvasRenderingContext2D,
    start_angle: f32,
    x: f32,
    y: f32,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    Box::into_raw(Box::new(PaintStyle(
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(
            context.context.create_conic_gradient(start_angle, x, y),
        ),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern(
    context: *const CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    width: i32,
    height: i32,
    repetition: CanvasRepetition,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let data = unsafe { std::slice::from_raw_parts(data, size) };
    let repetition: Repetition = repetition.into();

    from_image_slice(data, width, height).map(|image| {
        Box::into_raw(Box::new(PaintStyle(canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
            context.context.create_pattern(image, repetition),
        ))))
    }).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern_asset(
    context: *mut CanvasRenderingContext2D,
    asset: *const ImageAsset,
    repetition: CanvasRepetition,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    assert!(!asset.is_null());
    let context = unsafe { &*context };
    let asset = unsafe { &*asset };
    let repetition: Repetition = repetition.into();
    let has_alpha = asset.has_alpha();
    let mut ret = std::ptr::null_mut();
    // todo use bitmap directly ??
    asset.with_bytes_dimension(|bytes, (width, height)| {
        ret = if has_alpha {
            from_image_slice(bytes, width as i32, height as i32).map(
                |image| {
                    Box::into_raw(Box::new(PaintStyle(canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                        context.context.create_pattern(image, repetition),
                    ))))
                },
            ).unwrap_or(std::ptr::null_mut())
        } else {
            from_bitmap_slice(bytes, width as i32, height as i32).map(
                |image| {
                    Box::into_raw(Box::new(PaintStyle(canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                        context.context.create_pattern(image, repetition),
                    ))))
                },
            ).unwrap_or(std::ptr::null_mut())
        };
    });

    ret
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern_encoded(
    context: *mut CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    repetition: CanvasRepetition,
) -> *mut PaintStyle {
    assert!(!context.is_null());
    let context = unsafe { &*context };
    let repetition: Repetition = repetition.into();
    let data = unsafe { std::slice::from_raw_parts(data, size) };

    from_image_slice_encoded(data).map(|image| {
        Box::into_raw(Box::new(PaintStyle(canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
            context.context.create_pattern(image, repetition),
        ))))
    }).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern_canvas2d(
    source: *mut CanvasRenderingContext2D,
    context: *mut CanvasRenderingContext2D,
    repetition: CanvasRepetition,
) -> *mut PaintStyle {
    assert!(!source.is_null());
    assert!(!context.is_null());
    let source = unsafe { &mut *source };
    let context = unsafe { &*context };
    let repetition: Repetition = repetition.into();
    #[cfg(feature = "gl")]{
        source.make_current();
    }
    match source.context.get_image() {
        None => std::ptr::null_mut(),
        Some(image) => {
            Box::into_raw(Box::new(PaintStyle(canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                context.context.create_pattern(image, repetition),
            ))))
        }
    }
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
    Box::into_raw(Box::new(PaintStyle(
        canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Gradient(
            context.context.create_radial_gradient(x0, y0, r0, x1, y1, r1),
        ),
    )))
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
    context.context.draw_paint(color.as_ref());
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_point(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) {
    assert!(!context.is_null());
    let context = unsafe { &mut *context };
    context.context.draw_point(x, y);
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
    context
        .context
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

    let data = unsafe { std::slice::from_raw_parts(data, size) };
    if let Some(image) = from_image_slice(data, width as i32, height as i32) {
        context.context.draw_image_dx_dy(&image, dx, dy);
    }
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

    let data = unsafe { std::slice::from_raw_parts(data, size) };
    if let Some(image) = from_image_slice(data, width as i32, height as i32) {
        context
            .context
            .draw_image_dx_dy_dw_dh(&image, dx, dy, d_width, d_height);
    }
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
        context.context.draw_image_src_xywh_dst_xywh(
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
        let width = image.width() as f32;
        let height = image.height() as f32;
        context
            .context
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
        let width = image.width() as f32;
        let height = image.height() as f32;
        context.context.draw_image_src_xywh_dst_xywh(
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
        context.context.draw_image_src_xywh_dst_xywh(
            &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
        );
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_asset(
    context: *mut CanvasRenderingContext2D,
    asset: *const ImageAsset,
    dx: f32,
    dy: f32,
) {
    assert!(!context.is_null());
    assert!(!asset.is_null());
    let context = unsafe { &mut *context };
    let asset = unsafe { &*asset };
    context.context.draw_image_asset_dx_dy(&asset.0, dx, dy);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_dw_dh_asset(
    context: *mut CanvasRenderingContext2D,
    asset: *const ImageAsset,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    assert!(!context.is_null());
    assert!(!asset.is_null());
    let context = unsafe { &mut *context };
    let asset = unsafe { &*asset };
    context.context.draw_image_asset_dx_dy_dw_dh(&asset.0, dx, dy, d_width, d_height);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_asset(
    context: *mut CanvasRenderingContext2D,
    asset: *const ImageAsset,
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

    context.context.draw_image_asset_src_xywh_dst_xywh(
        &asset.0, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_context(
    context: *mut CanvasRenderingContext2D,
    source: *mut CanvasRenderingContext2D,
    dx: f32,
    dy: f32,
) {
    assert!(!context.is_null());
    assert!(!source.is_null());
    let context = unsafe { &mut *context };
    let source = unsafe { &mut *source };
    if let Some(image) = source.context.get_image() {
        context.context.draw_image_dx_dy(&image, dx, dy);
    }
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
    let context = unsafe { &mut *context };
    let source = unsafe { &mut *source };

    if let Some(image) = source.context.get_image() {
        context.context.draw_image_dx_dy_dw_dh(&image, dx, dy, d_width, d_height);
    }
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
    let source = unsafe { &mut *source };
    if let Some(image) = source.context.get_image() {
        context.context.draw_image_src_xywh_dst_xywh(
            &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
        );
    }
}

fn canvas_native_context_read_webgl_pixels(
    source: &mut canvas_webgl::prelude::WebGLState,
    internalformat: i32,
    format: i32,
) -> (i32, i32, Vec<u8>) {
    source.make_current();
    let width = source.get_drawing_buffer_width();
    let height = source.get_drawing_buffer_height();

    let row_size = bytes_per_pixel(internalformat as u32, format as u32) as i32;

    let mut buf = vec![255u8; (width * height * row_size) as usize];
    unsafe {
        gl_bindings::Flush();
        gl_bindings::ReadPixels(
            0,
            0,
            width,
            height,
            internalformat as u32,
            format as u32,
            buf.as_mut_ptr() as *mut c_void,
        );
    }

    (width, height, buf)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_webgl(
    context: *mut CanvasRenderingContext2D,
    source: *mut WebGLState,
    dx: f32,
    dy: f32,
) {
    assert!(!source.is_null());

    // let context = unsafe { &*context };
    let source = unsafe { &mut *source };
    let (width, height) = {
        let state = source.get_inner_mut();
        state.make_current();
        let (w, h) = state.get_dimensions();
        (w as f32, h as f32)
    };

    let pixels = canvas_native_context_read_webgl_pixels(
        &mut source.0,
        gl_bindings::RGBA as i32,
        gl_bindings::RGBA as i32,
    );

    let ptr = pixels.2.as_ptr();
    let size = pixels.2.len();
    canvas_native_context_draw_image_dx_dy(context, ptr, size, width, height, dx, dy);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_dw_dh_webgl(
    context: *mut CanvasRenderingContext2D,
    source: *mut WebGLState,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) {
    assert!(!context.is_null());
    assert!(!source.is_null());

    //  let context = unsafe { &*context };
    let source = unsafe { &mut *source };
    let (width, height) = {
        let state = source.get_inner_mut();
        state.make_current();
        let (w, h) = state.get_dimensions();
        (w as f32, h as f32)
    };

    let pixels = canvas_native_context_read_webgl_pixels(
        &mut source.0,
        gl_bindings::RGBA as i32,
        gl_bindings::RGBA as i32,
    );

    let ptr = pixels.2.as_ptr();
    let size = pixels.2.len();

    canvas_native_context_draw_image_dx_dy_dw_dh(
        context, ptr, size, width, height, dx, dy, d_width, d_height,
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_webgl(
    context: *mut CanvasRenderingContext2D,
    source: *mut WebGLState,
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

    // let context = unsafe { &*context };
    let source = unsafe { &mut *source };
    let (width, height) = {
        let state = source.get_inner_mut();
        state.make_current();
        let (w, h) = state.get_dimensions();
        (w as f32, h as f32)
    };

    let pixels = canvas_native_context_read_webgl_pixels(
        &mut source.0,
        gl_bindings::RGBA as i32,
        gl_bindings::RGBA as i32,
    );

    let ptr = pixels.2.as_ptr();
    let size = pixels.2.len();

    canvas_native_context_draw_image(
        context, ptr, size, width, height, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_atlas(
    context: *mut CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    width: f32,
    height: f32,
    xform: *const f32,
    xform_size: usize,
    tex: *const f32,
    tex_size: usize,
    colors: *const *const c_char,
    colors_size: usize,
    blend_mode: i32,
) {
    assert!(!context.is_null());

    let data = unsafe { std::slice::from_raw_parts(data, size) };
    if let Some(image) = from_image_slice(data, width as i32, height as i32) {
        let context = unsafe { &mut *context };


        let xform = unsafe { std::slice::from_raw_parts(xform, xform_size) };
        let tex = unsafe { std::slice::from_raw_parts(tex, tex_size) };
        let mut colors_value: Option<Vec<&CStr>> = None;

        if !colors.is_null() {
            let values = unsafe { std::slice::from_raw_parts(colors, colors_size) };
            let values: Vec<_> = values
                .iter()
                .map(|value| unsafe { CStr::from_ptr(*value) })
                .collect();

            colors_value = Some(values);
        }
        context.context.draw_atlas_color(
            &image,
            xform,
            tex,
            colors_value.as_deref(),
            CompositeOperationType::from(blend_mode),
        );
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_atlas_encoded(
    context: *mut CanvasRenderingContext2D,
    data: *const u8,
    size: usize,
    xform: *const f32,
    xform_size: usize,
    tex: *const f32,
    tex_size: usize,
    colors: *const *const c_char,
    colors_size: usize,
    blend_mode: i32,
) {
    assert!(!context.is_null());

    let data = unsafe { std::slice::from_raw_parts(data, size) };
    if let Some(image) = from_image_slice_encoded(data) {
        let context = unsafe { &mut *context };

        let xform = unsafe { std::slice::from_raw_parts(xform, xform_size) };
        let tex = unsafe { std::slice::from_raw_parts(tex, tex_size) };
        let mut colors_value: Option<Vec<&CStr>> = None;

        if !colors.is_null() {
            let values = unsafe { std::slice::from_raw_parts(colors, colors_size) };
            let values: Vec<_> = values
                .iter()
                .map(|value| unsafe { CStr::from_ptr(*value) })
                .collect();

            colors_value = Some(values);
        }
        context.context.draw_atlas_color(
            &image,
            xform,
            tex,
            colors_value.as_deref(),
            CompositeOperationType::from(blend_mode),
        );
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_atlas_asset(
    context: *mut CanvasRenderingContext2D,
    asset: *const ImageAsset,
    xform: *const f32,
    xform_size: usize,
    tex: *const f32,
    tex_size: usize,
    colors: *const *const c_char,
    colors_size: usize,
    blend_mode: i32,
) {
    assert!(!context.is_null());

    let asset = unsafe { &*asset };
    let context = unsafe { &mut *context };
    let xform = unsafe { std::slice::from_raw_parts(xform, xform_size) };
    let tex = unsafe { std::slice::from_raw_parts(tex, tex_size) };
    let mut colors_value: Option<Vec<&CStr>> = None;

    if !colors.is_null() {
        let values = unsafe { std::slice::from_raw_parts(colors, colors_size) };
        let values: Vec<_> = values
            .iter()
            .map(|value| unsafe { CStr::from_ptr(*value) })
            .collect();

        colors_value = Some(values);
    }
    context.context.draw_atlas_asset_color(
        &asset.0,
        xform,
        tex,
        colors_value.as_deref(),
        CompositeOperationType::from(blend_mode),
    );
}

/*
#[repr(C)]
pub enum VertexMode {}


#[no_mangle]
pub extern "C" fn canvas_native_context_draw_vertices(
    context: *mut CanvasRenderingContext2D,
    vertices: *const f32,
    vertices_size: usize,
    indices: *const c_ushort,
    indices_size: usize,
    textures: *const f32,
    textures_size: usize,
    colors: *const *const c_char,
    colors_size: usize,
    blend_mode: i32,
) {
    assert!(!context.is_null());

    let context = unsafe { &*context };
    context.make_current();
    let vertices = unsafe { std::slice::from_raw_parts(vertices, vertices_size) };
    let indices = unsafe { std::slice::from_raw_parts(indices, indices_size) };
    let textures = unsafe { std::slice::from_raw_parts(textures, textures_size) };
    let mut colors_value: Option<Vec<&CStr>> = None;

    if !colors.is_null() {
        let values = unsafe { std::slice::from_raw_parts(colors, colors_size) };
        let values: Vec<_> = values
            .iter()
            .map(|value| unsafe { CStr::from_ptr(*value) })
            .collect();

        colors_value = Some(values);
    }
    context.draw_vertices_color(
        &asset.0,
        xform,
        tex,
        colors_value.as_deref(),
        CompositeOperationType::from(blend_mode),
    );
} */

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
    context.context.ellipse(
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

#[no_mangle]
pub extern "C" fn canvas_native_context_fill(context: *mut CanvasRenderingContext2D, rule: CanvasFillRule) {
    let context = unsafe { &mut *context };
    context.context.fill_rule(None, rule.into());
}

#[no_mangle]
pub extern "C" fn canvas_native_context_fill_with_path(
    context: *mut CanvasRenderingContext2D,
    path: &mut Path,
    rule: CanvasFillRule,
) {
    let context = unsafe { &mut *context };
    context.context.fill_rule(Some(&mut path.0), rule.into());
}

#[no_mangle]
pub extern "C" fn canvas_native_context_fill_rect(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    let context = unsafe { &mut *context };
    context.context.fill_rect_xywh(x, y, width, height);
}

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
    context.context.fill_text(text.as_ref(), x, y, None);
}

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
    context.context.fill_text(text.as_ref(), x, y, Some(width));
}

#[no_mangle]
pub extern "C" fn canvas_native_context_fill_oval(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    let context = unsafe { &mut *context };
    context.context.fill_oval(x, y, width, height);
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
    Box::into_raw(Box::new(ImageData(
        context.context.get_image_data(sx, sy, sw, sh),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_get_transform(
    context: *mut CanvasRenderingContext2D,
) -> *mut Matrix {
    let context = unsafe { &mut *context };
    let matrix = context.context.get_transform();
    Box::into_raw(Box::new(Matrix(matrix)))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_path_str(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    rule: CanvasFillRule,
) -> bool {
    let context = unsafe { &*context };
    context.context.point_in_path(None, x, y, rule.into())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_path_with_path_str(
    context: *mut CanvasRenderingContext2D,
    path: &mut Path,
    x: f32,
    y: f32,
    rule: CanvasFillRule,
) -> bool {
    let context = unsafe { &*context };
    context
        .context
        .point_in_path(Some(&path.0), x, y, rule.into())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_path(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    rule: CanvasFillRule,
) -> bool {
    let context = unsafe { &*context };
    context.context.point_in_path(None, x, y, rule.into())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_path_with_path(
    context: *mut CanvasRenderingContext2D,
    path: &mut Path,
    x: f32,
    y: f32,
    rule: CanvasFillRule,
) -> bool {
    let context = unsafe { &*context };
    context
        .context
        .point_in_path(Some(&path.0), x, y, rule.into())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_stroke(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) -> bool {
    let context = unsafe { &*context };
    context.context.point_in_stroke(None, x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_is_point_in_stroke_with_path(
    context: *mut CanvasRenderingContext2D,
    path: &mut Path,
    x: f32,
    y: f32,
) -> bool {
    let context = unsafe { &*context };
    context.context.point_in_stroke(Some(&path.0), x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_line_to(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context.context.line_to(x, y)
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
    let context = unsafe { &*context };
    Box::into_raw(Box::new(TextMetrics(
        context.context.measure_text(text.as_ref()),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_context_move_to(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context.context.move_to(x, y)
}

#[no_mangle]
pub extern "C" fn canvas_native_context_put_image_data(
    context: *mut CanvasRenderingContext2D,
    image_data: *const ImageData,
    dx: f32,
    dy: f32,
    dirty_x: f32,
    dirty_y: f32,
    dirty_width: f32,
    dirty_height: f32,
) {
    let context = unsafe { &mut *context };
    let image_data = unsafe { &*image_data };
    context.context.put_image_data(
        &image_data.0,
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
    context.context.quadratic_curve_to(cpx, cpy, x, y)
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
    context.context.rect(x, y, width, height)
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
        context.context.round_rect(x, y, width, height, radii)
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
        .context
        .round_rect(x, y, width, height, radii.as_ref())
}

#[no_mangle]
pub extern "C" fn canvas_native_context_reset_transform(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    context.context.reset_transform();
}

#[no_mangle]
pub extern "C" fn canvas_native_context_restore(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    context.context.restore()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_rotate(context: *mut CanvasRenderingContext2D, angle: f32) {
    let context = unsafe { &mut *context };
    context.context.rotate(angle);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_save(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    context.context.save()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_scale(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context.context.scale(x, y);
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
    context.context.set_transform(a, b, c, d, e, f);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_set_transform_matrix(
    context: *mut CanvasRenderingContext2D,
    matrix: *const Matrix,
) {
    let context = unsafe { &mut *context };
    let matrix = unsafe { &*matrix };
    context.context.set_transform_matrix(&matrix.0);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_stroke(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    context.context.stroke(None);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_stroke_with_path(
    context: *mut CanvasRenderingContext2D,
    path: &mut Path,
) {
    let context = unsafe { &mut *context };
    context.context.stroke(Some(&mut path.0));
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
    context.context.stroke_rect_xywh(x, y, width, height);
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
    context.context.stroke_text(text.as_ref(), x, y, None);
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
    context
        .context
        .stroke_text(text.as_ref(), x, y, Some(width));
}

#[no_mangle]
pub extern "C" fn canvas_native_context_stroke_oval(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    let context = unsafe { &mut *context };
    context.context.stroke_oval(x, y, width, height);
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
    context.context.transform(a, b, c, d, e, f);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_translate(
    context: *mut CanvasRenderingContext2D,
    x: f32,
    y: f32,
) {
    let context = unsafe { &mut *context };
    context.context.translate(x, y);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_flush(context: *mut CanvasRenderingContext2D) {
    let context = unsafe { &mut *context };
    #[cfg(feature = "gl")]{
        if let Some(ref context) = context.context.gl_context {
            context.make_current();
        }
    }
    context.context.flush();
}

#[no_mangle]
pub extern "C" fn canvas_native_context_render(context: *mut CanvasRenderingContext2D) {
    if context.is_null() {
        return;
    }
    let context = unsafe { &mut *context };

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


    #[cfg(feature = "gl")]{
        if let Some(ref context) = context.context.gl_context {
            context.make_current();
        }
    }

    CString::new(to_data_url(context, format.as_ref(), quality))
        .unwrap()
        .into_raw()
}
