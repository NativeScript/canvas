use std::cell::RefCell;
use std::ffi::{c_longlong, c_void, CStr, CString};
use std::ops::DerefMut;
use std::os::raw::c_char;
use std::ptr::NonNull;
use parking_lot::RwLock;

use canvas_2d::context::fill_and_stroke_styles::pattern::Repetition;
use canvas_2d::utils::image::from_image_slice;
pub use canvas_c::*;
use canvas_c::CanvasRenderingContext2D;
use canvas_c::PaintStyle;
use canvas_core::context_attributes::ContextAttributes;
use canvas_core::gl::GLContext;
use canvas_core::image_asset::ImageAsset;
#[allow(non_camel_case_types)]
pub(crate) enum iOSView {
    OffScreen,
    OnScreen(NonNull<c_void>)
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub(crate) struct iOSGLContext {
    pub(crate) context_attributes: ContextAttributes,
    pub(crate) gl_context: GLContext,
    ios_view: iOSView,
}

#[no_mangle]
pub extern "C" fn canvas_native_init_ios_gl(
    view: i64,
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
    version: i32,
    is_canvas: bool,
) -> c_longlong {
    if version == 2 && !GLContext::has_gl2support() {
        return 0;
    }

    let power_preference = unsafe { CStr::from_ptr(power_preference).to_string_lossy() };

    if let Some(ios_view) = NonNull::new(view as *mut c_void) {
        let mut attrs = ContextAttributes::new(
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
        );

        if let Some(gl_context) = GLContext::create_window_context(&mut attrs, ios_view) {
            return Box::into_raw(Box::new(iOSGLContext {
                ios_view: iOSView::OnScreen(ios_view),
                gl_context,
                context_attributes: attrs,
            })) as i64;
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn canvas_native_init_ios_gl_with_shared_gl(
    view: i64,
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
    version: i32,
    is_canvas: bool,
    shared_context: i64,
) -> c_longlong {
    if version == 2 && !GLContext::has_gl2support() {
        return 0;
    }

    if shared_context == 0 {
        return 0;
    }

    let power_preference = unsafe { CStr::from_ptr(power_preference).to_string_lossy() };

    if let Some(ios_view) = NonNull::new(view as *mut c_void) {
        let mut attrs = ContextAttributes::new(
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
        );

        let shared_context = shared_context as *mut iOSGLContext;
        let shared_context = unsafe { &*shared_context };

        if let Some(gl_context) = GLContext::create_shared_window_context(
            &mut attrs,
            ios_view,
            &shared_context.gl_context,
        ) {
            return Box::into_raw(Box::new(iOSGLContext {
                ios_view: iOSView::OnScreen(ios_view),
                gl_context,
                context_attributes: attrs,
            })) as i64;
        }
    }

    0
}





#[no_mangle]
pub extern "C" fn canvas_native_init_offscreen_ios_gl(
    width: i32,
    height: i32,
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
    version: i32,
    is_canvas: bool,
) -> c_longlong {
    if version == 1 && !GLContext::has_gl2support() {
        return 0;
    }

    let power_preference = unsafe { CStr::from_ptr(power_preference).to_string_lossy() };

    let mut attrs = ContextAttributes::new(
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
    );

    if let Some(gl_context) = GLContext::create_offscreen_context(&mut attrs, width, height) {
        return Box::into_raw(Box::new(iOSGLContext {
            ios_view: iOSView::OffScreen,
            gl_context,
            context_attributes: attrs,
        })) as i64;
    }

    0
}

#[no_mangle]
pub extern "C" fn canvas_native_init_offscreen_ios_gl_with_shared_gl(
    width: i32,
    height: i32,
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
    version: i32,
    is_canvas: bool,
    shared_context: i64,
) -> c_longlong {
    if version == 1 && !GLContext::has_gl2support() {
        return 0;
    }

    if shared_context == 0 {
        return 0;
    }

    let power_preference = unsafe { CStr::from_ptr(power_preference).to_string_lossy() };

    let mut attrs = ContextAttributes::new(
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
    );

    let shared_context = shared_context as *mut iOSGLContext;
    let shared_context = unsafe { &*shared_context };

    if let Some(gl_context) = GLContext::create_shared_offscreen_context(
        &mut attrs,
        width,
        height,
        &shared_context.gl_context,
    ) {
        return Box::into_raw(Box::new(iOSGLContext {
            ios_view: iOSView::OffScreen,
            gl_context,
            context_attributes: attrs,
        })) as i64;
    }

    0
}


#[no_mangle]
pub extern "C" fn canvas_native_ios_flush_gl(context: i64) -> bool {
    if context == 0 {
        return false;
    }

    let context = context as *mut iOSGLContext;
    let context = unsafe { &mut *context };

    context.gl_context.make_current();
    context.gl_context.swap_buffers()
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
pub extern "C" fn canvas_native_resize_context_2d(context: i64, width: f32, height: f32) {
    if context == 0 {
        return;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.resize(width, height);
}

#[no_mangle]
pub extern "C" fn canvas_native_create_2d_context(
    context: i64,
    width: i32,
    height: i32,
    alpha: bool,
    density: f32,
    samples: i32,
    font_color: i32,
    ppi: f32,
    direction: i32,
) -> i64 {
    if context == 0 {
        return 0;
    }

    let context = context as *mut iOSGLContext;
    let context = unsafe { &mut *context };

    context.gl_context.make_current();
    let mut frame_buffers = [0];
    unsafe {
        gl_bindings::GetIntegerv(gl_bindings::FRAMEBUFFER_BINDING, frame_buffers.as_mut_ptr())
    };

    let ctx_2d = CanvasRenderingContext2D::new(
        canvas_2d::context::ContextWrapper::new(canvas_2d::context::Context::new_gl(
            width as f32,
            height as f32,
            density,
            frame_buffers[0],
            samples,
            alpha,
            font_color,
            ppi,
            canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
        )),
        context.gl_context.clone(),
        alpha,
    );

    Box::into_raw(Box::new(ctx_2d)) as i64
}

#[no_mangle]
pub extern "C" fn canvas_native_update_gl_surface(
    view: i64,
    width: i32,
    height: i32,
    context: i64,
) {
    if context == 0 {
        return;
    }

    if let Some(ios_view) = NonNull::new(view as *mut c_void) {
        let context = context as *mut iOSGLContext;
        let context = unsafe { &mut *context };

        context.gl_context.set_surface(ios_view);
        context.ios_view = iOSView::OnScreen(ios_view);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_release_ios_gl(context: i64) {
    if context == 0 {
        return;
    }
    let context = context as *mut iOSGLContext;
    let _ = unsafe { Box::from_raw(context) };
}

#[no_mangle]
pub extern "C" fn canvas_native_get_gl_pointer(gl_context: i64) -> i64 {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *mut iOSGLContext;
    let gl_context = unsafe { &*gl_context };
    gl_context.gl_context.as_raw_inner() as i64
}

#[no_mangle]
pub extern "C" fn canvas_native_release_gl_pointer(gl_context: i64) {
    if gl_context == 0 {
        return;
    }
    let gl_context = gl_context as *const RwLock<canvas_core::gl::GLContextInner>;
    let _ = GLContext::from_raw_inner(gl_context);
}

#[no_mangle]
pub extern "C" fn canvas_native_context_2d_test(context: i64) {
    if context == 0 {
        return;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    {
        let mut ctx = context.get_context_mut();
        ctx.set_fill_style_with_color("red");
        ctx.fill_rect_xywh(0., 0., 300., 300.);
    }
    context.render();
}

#[no_mangle]
pub extern "C" fn canvas_native_context_2d_test_to_data_url(context: i64) -> *mut c_char {
    if context == 0 {
        return std::ptr::null_mut();
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    let mut ctx = context.get_context_mut();
    ctx.flush_and_sync_cpu();
    let ret = canvas_2d::to_data_url_context(ctx.deref_mut(), "image/png", 92);
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_context_2d_destroy_string(string: *mut c_char) {
    if string.is_null() {
        return;
    }

    let _ = unsafe { CString::from_raw(string) };
}

#[no_mangle]
pub extern "C" fn canvas_native_imageasset_load_from_bytes(
    asset: i64,
    bytes: *mut u8,
    size: usize,
) -> bool {
    if asset == 0 {
        return false;
    }

    let asset = asset as *mut ImageAsset;
    let asset = unsafe { &mut *asset };

    let bytes = unsafe { std::slice::from_raw_parts(bytes as _, size) };
    asset.load_from_bytes(bytes)
}


#[no_mangle]
pub extern "C" fn canvas_native_context_create_pattern_raw(
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
        return Box::into_raw(Box::new(PaintStyle::new(Some(
            canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle::Pattern(
                context.get_context().create_pattern(
                    image,
                    Repetition::try_from(repetition.as_ref()).unwrap_or(Repetition::NoRepeat),
                ),
            ),
        )))) as i64;
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
    unsafe {
        if context == 0 {
            return false;
        }

        let context = context as *mut CanvasRenderingContext2D;

        let context = unsafe { &mut *context };

        if let Some(image) = from_image_slice(image_data, width as i32, height as i32) {
            context.make_current();
            let mut context = context.get_context_mut();
            context.draw_image_src_xywh_dst_xywh(
                &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
            );
            return true;
        }
        false
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_with_bytes(
    context: i64,
    bytes: *mut u8,
    size: usize,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
) -> bool {
    let bytes = unsafe { std::slice::from_raw_parts(bytes as _, size) };
    return draw_image(
        context, bytes, width, height, 0.0, 0.0, width, height, dx, dy, width, height,
    );
}

#[no_mangle]
pub extern "C" fn canvas_native_context_draw_image_dx_dy_dw_dh_with_bytes(
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
pub extern "C" fn canvas_native_context_draw_image_with_bytes(
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
