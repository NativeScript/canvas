mod gl;
mod mtl;

pub use mtl::*;

use std::ffi::{c_int, c_longlong, c_void, CStr};
use std::os::raw::c_char;
use std::ptr::NonNull;

use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_2d::utils::image::from_image_slice;
pub use canvas_c::*;
use canvas_core::context_attributes::PowerPreference;
use canvas_core::gpu::gl::GLContext;
use canvas_webgl::prelude::WebGLVersion;


#[no_mangle]
pub extern "C" fn canvas_native_ios_create_webgl_context(
    view: *mut c_void,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: i32,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    version: u32,
) -> c_longlong {
    if version == 2 && !GLContext::has_gl2support() {
        return 0;
    }

    // let _ = env_logger::try_init();

    if let Some(power_preference) = PowerPreference::try_from(power_preference).ok() {
        return Box::into_raw(Box::new(WebGLState::new_with_view(
            view, WebGLVersion::try_from(version as i32).unwrap(),
            alpha,
            antialias, depth, fail_if_major_performance_caveat, power_preference.into(),
            premultiplied_alpha, preserve_drawing_buffer, stencil, desynchronized, xr_compatible, false,
        ))) as i64;
    }

    0
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_flush_webgl(context: i64) -> bool {
    if context == 0 {
        return false;
    }

    let context = context as *mut WebGLState;
    let context = unsafe { &mut *context };

    context.get_inner().make_current();
    context.get_inner().swap_buffers()
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_flush_2d_context(context: i64) {
    if context == 0 {
        return;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };


    context.get_context_mut().flush();
}


#[no_mangle]
pub extern "C" fn canvas_native_ios_present_drawable(context: i64) {
    if context == 0 {
        return;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    canvas_2d::context::Context::present(context.get_context_mut());
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_flush_2d_context_and_sync_cpu(context: i64) {
    if context == 0 {
        return;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };


    context.get_context_mut().flush_and_sync_cpu();
}


#[no_mangle]
pub extern "C" fn canvas_native_ios_resize_context_2d(context: i64, width: f32, height: f32) {
    if context == 0 {
        return;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.resize(width, height);
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_create_2d_context(
    view: *mut c_void,
    width: i32,
    height: i32,
    alpha: bool,
    density: f32,
    font_color: i32,
    ppi: f32,
    direction: i32,
) -> i64 {
    let ctx_2d = CanvasRenderingContext2D::new_gl(
        canvas_2d::context::Context::new_gl(
            view,
            width as f32,
            height as f32,
            density,
            alpha,
            font_color,
            ppi,
            canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
        ),
        alpha,
    );

    Box::into_raw(Box::new(ctx_2d)) as i64
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_update_2d_webgpu_surface(
    view: i64,
    width: f32,
    height: f32,
    context: i64,
) {
    if context == 0 {
        return;
    }

    if let Some(ios_view) = NonNull::new(view as *mut c_void) {
        let context = context as *mut CanvasRenderingContext2D;
        let context = unsafe { &mut *context };
        let context = context.get_context_mut();
        if let Some(context) = context.metal_context.as_mut() {
            unsafe {
                context.set_view(width, height, ios_view.as_ptr())
            }
        }
    }
}


#[no_mangle]
pub extern "C" fn canvas_native_ios_update_webgl_surface(
    view: i64,
    _width: i32,
    _height: i32,
    context: i64,
) {
    if context == 0 {
        return;
    }

    if let Some(ios_view) = NonNull::new(view as *mut c_void) {
        let context = context as *mut WebGLState;
        let context = unsafe { &mut *context };

        let context = context.get_inner_mut();
        context.set_surface(ios_view);
    }
}


#[no_mangle]
pub extern "C" fn canvas_native_ios_release_webgl(context: i64) {
    if context == 0 {
        return;
    }
    let context = context as *mut WebGLState;
    let _ = unsafe { Box::from_raw(context) };
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_gl_make_current(context: i64) {
    if context == 0 {
        return;
    }
    let gl_context = context as *mut WebGLState;
    let gl_context = unsafe { &*gl_context };
    gl_context.get_inner().make_current();
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_image_asset_load_from_bytes(
    asset: i64,
    bytes: *mut u8,
    size: usize,
) -> bool {
    if asset == 0 {
        return false;
    }

    let asset = asset as *const ImageAsset;
    let asset = unsafe { &*asset };

    let bytes = unsafe { std::slice::from_raw_parts(bytes as _, size) };
    asset.load_from_bytes(bytes)
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_context_create_pattern_raw(
    context: i64,
    width: i32,
    height: i32,
    bytes: *mut u8,
    size: usize,
    repetition: *const c_char,
) -> i64 {
    if context == 0 {
        return 0;
    }

    let context = context as *mut CanvasRenderingContext2D;

    let context = unsafe { &mut *context };

    let bytes = unsafe { std::slice::from_raw_parts(bytes as _, size) };

    if let Some(image) = from_image_slice(bytes, width, height) {
        let repetition = unsafe { CStr::from_ptr(repetition).to_string_lossy() };
        return Box::into_raw(Box::new(PaintStyle::new(
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                context.get_context().create_pattern(
                    image,
                    Repetition::try_from(repetition.as_ref()).unwrap_or(Repetition::NoRepeat),
                ),
            ),
        ))) as i64;
    }

    0
}

fn draw_image(
    context: i64,
    image_data: &[u8],
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
) -> bool {
    if context == 0 {
        return false;
    }

    let context = context as *mut CanvasRenderingContext2D;

    let context = unsafe { &mut *context };

    if let Some(image) = from_image_slice(image_data, width as i32, height as i32) {
        context.make_current();
        let context = context.get_context_mut();
        context.draw_image_src_xywh_dst_xywh(
            &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
        );
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_context_draw_image_dx_dy_with_bytes(
    context: i64,
    bytes: *mut u8,
    size: usize,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
) -> bool {
    let bytes = unsafe { std::slice::from_raw_parts(bytes as _, size) };
    draw_image(
        context, bytes, width, height, 0.0, 0.0, width, height, dx, dy, width, height,
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_context_draw_image_dx_dy_dw_dh_with_bytes(
    context: i64,
    bytes: *mut u8,
    size: usize,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) -> bool {
    let bytes = unsafe { std::slice::from_raw_parts(bytes as _, size) };
    draw_image(
        context, bytes, width, height, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_context_draw_image_with_bytes(
    context: i64,
    bytes: *mut u8,
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
) -> bool {
    let bytes = unsafe { std::slice::from_raw_parts(bytes as _, size) };
    draw_image(
        context, bytes, width, height, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
    )
}

/*
#[no_mangle]
pub extern "C" fn canvas_native_svg_draw_from_string(context: i64, svg: *const c_char) {
    if context == 0 || svg.is_null() {
        return;
    }

    let context = context as *mut CanvasRenderingContext2D;

    let context = unsafe { &mut *context };

    let svg = unsafe { CStr::from_ptr(svg) };

    let mut context = context.get_context_mut();
    let svg = svg.to_string_lossy();
    canvas_2d::svg::draw_svg(&mut context, svg.as_ref());
}

#[no_mangle]
pub extern "C" fn canvas_native_svg_draw_from_path(context: i64, path: *const c_char) {
    if context == 0 || path.is_null() {
        return;
    }

    let context = context as *mut CanvasRenderingContext2D;

    let context = unsafe { &mut *context };

    let path = unsafe { CStr::from_ptr(path) };

    let mut context = context.get_context_mut();
    let path = path.to_string_lossy();
    canvas_2d::svg::draw_svg_from_path(&mut context, path.as_ref());
}

*/

#[no_mangle]
pub extern "C" fn canvas_native_ios_context_custom_with_buffer_flush(
    context: i64,
    bytes: *mut u8,
    size: usize,
    width: f32,
    height: f32,
    alpha: bool,
) {
    unsafe {
        if context == 0 {
            return;
        }

        let ct = if alpha {
            skia_safe::ColorType::RGBA8888
        } else {
            skia_safe::ColorType::RGB565
        };

        let info = skia_safe::ImageInfo::new(
            skia_safe::ISize::new(width as i32, height as i32),
            ct,
            skia_safe::AlphaType::Premul,
            None,
        );
        let context = context as *mut CanvasRenderingContext2D;
        let context = &mut *context;
        let context = context.get_context_mut();

        let data = std::slice::from_raw_parts_mut(bytes, size);
        let mut surface = skia_safe::surfaces::wrap_pixels(&info, data, None, None).unwrap();
        let canvas = surface.canvas();
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::PaintStyle::Fill);
        paint.set_blend_mode(skia_safe::BlendMode::Clear);
        canvas.draw_rect(
            skia_safe::Rect::from_xywh(0f32, 0f32, width, height),
            &paint,
        );

        if let Some(image) = context.get_image() {
            surface.canvas().draw_image(image, (0, 0), None);
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_context_init_context_with_custom_surface(
    width: f32,
    height: f32,
    density: f32,
    alpha: bool,
    font_color: c_int,
    ppi: f32,
    direction: c_int,
) -> c_longlong {
    let mut ctx_2d = CanvasRenderingContext2D::new(
        canvas_2d::context::Context::new(
            width,
            height,
            density,
            alpha,
            font_color,
            ppi,
            canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
        ),
        alpha,
    );

    {
        let ctx = ctx_2d.get_context_mut();
        ctx.clear_canvas();
    }

    Box::into_raw(Box::new(ctx_2d)) as i64
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_webgl_tex_image_2d(
    context: i64,
    target: i32,
    level: i32,
    internalformat: i32,
    format: i32,
    type_: i32,
    bytes: *mut u8,
    size: usize,
    width: f32,
    height: f32,
    flip_y: bool,
) {
    if context == 0 {
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(bytes as _, size) };

    let gl_context = context as *mut WebGLState;
    let gl_context = unsafe { &*gl_context };
    gl_context.get_inner().make_current();

    unsafe {
        if flip_y {
            let mut buffer = bytes.to_vec();
            canvas_webgl::utils::gl::flip_in_place(
                buffer.as_mut_ptr(),
                buffer.len(),
                canvas_webgl::utils::gl::bytes_per_pixel(type_ as _, format as _) as _,
                height as usize,
            );

            gl_bindings::TexImage2D(
                target as u32,
                level,
                internalformat,
                width as i32,
                height as i32,
                0,
                format as u32,
                type_ as u32,
                buffer.as_ptr() as *const std::os::raw::c_void,
            );
        } else {
            gl_bindings::TexImage2D(
                target as u32,
                level,
                internalformat,
                width as i32,
                height as i32,
                0,
                format as u32,
                type_ as u32,
                bytes.as_ptr() as *const std::os::raw::c_void,
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_webgl_tex_sub_image_2d(
    context: i64,
    target: i32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    format: i32,
    type_: i32,
    bytes: *mut u8,
    size: usize,
    width: f32,
    height: f32,
    flip_y: bool,
) {
    if context == 0 {
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(bytes as _, size) };

    let gl_context = context as *mut WebGLState;
    let gl_context = unsafe { &*gl_context };
    gl_context.get_inner().make_current();

    unsafe {
        if flip_y {
            let mut buffer = bytes.to_vec();
            canvas_webgl::utils::gl::flip_in_place(
                buffer.as_mut_ptr(),
                buffer.len(),
                canvas_webgl::utils::gl::bytes_per_pixel(type_ as _, format as _) as _,
                height as usize,
            );

            gl_bindings::TexSubImage2D(
                target as u32,
                level,
                xoffset,
                yoffset,
                width as i32,
                height as i32,
                format as u32,
                type_ as u32,
                buffer.as_ptr() as *const std::os::raw::c_void,
            );
        } else {
            gl_bindings::TexSubImage2D(
                target as u32,
                level,
                xoffset,
                yoffset,
                width as i32,
                height as i32,
                format as u32,
                type_ as u32,
                bytes.as_ptr() as *const std::os::raw::c_void,
            );
        }
    }
}
