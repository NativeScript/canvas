use std::cell::RefCell;
use std::ffi::{c_longlong, c_void};
use std::ptr::NonNull;

use raw_window_handle::HasRawWindowHandle;

use canvas_core::context_attributes::ContextAttributes;
use canvas_core::gl::GLContext;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub(crate) struct iOSGLContext {
    pub(crate) context_attributes: ContextAttributes,
    pub(crate) gl_context: GLContext,
    ios_view: NonNull<c_void>,
}

#[cxx::bridge(namespace = "org::nativescript::canvas")]
pub mod ffi {
    extern "Rust" {
        pub fn canvas_native_init_ios_gl(
            view: i64,
            width: i32,
            height: i32,
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
            version: i32,
            is_canvas: bool,
        ) -> i64;

        pub fn canvas_native_create_2d_context(
            context: i64,
            width: i32,
            height: i32,
            alpha: bool,
            density: f32,
            samples: i32,
            font_color: i32,
            ppi: f32,
            direction: i32,
        ) -> i64;

        pub fn canvas_native_update_gl_surface(view: i64, width: i32, height: i32, context: i64);

        pub fn canvas_native_release_ios_gl(context: i64);

        pub fn canvas_native_get_gl_pointer(gl_context: i64) -> i64;

        pub fn canvas_native_release_gl_pointer(gl_context: i64);

        pub fn canvas_native_context_2d_test(context: i64);
    }
}

pub fn canvas_native_init_ios_gl(
    view: i64,
    width: i32,
    height: i32,
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
    version: i32,
    is_canvas: bool,
) -> c_longlong {
    if version == 1 && !GLContext::has_gl2support() {
        return 0;
    }

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
        let mut window_handle = raw_window_handle::UiKitWindowHandle::empty();
        window_handle.ui_view = ios_view.as_ptr();
        let window = raw_window_handle::RawWindowHandle::UiKit(window_handle);
        if let Some(gl_context) =
            GLContext::create_window_surface(&mut attrs, width, height, window)
        {
            return Box::into_raw(Box::new(iOSGLContext {
                ios_view,
                gl_context,
                context_attributes: attrs,
            })) as i64;
        }
    }

    0
}

pub fn canvas_native_create_2d_context(
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

    let mut ctx_2d = canvas_cxx::CanvasRenderingContext2D::new(
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

pub fn canvas_native_update_gl_surface(view: i64, width: i32, height: i32, context: i64) {
    if context == 0 {
        return;
    }

    if let Some(ios_view) = NonNull::new(view as *mut c_void) {
        let context = context as *mut iOSGLContext;
        let context = unsafe { &mut *context };

        let mut window_handle = raw_window_handle::UiKitWindowHandle::empty();
        window_handle.ui_view = ios_view.as_ptr();
        let window = raw_window_handle::RawWindowHandle::UiKit(window_handle);

        context.gl_context.set_window_surface(
            &mut context.context_attributes,
            width,
            height,
            window,
        );

        context.ios_view = ios_view;
    }
}

pub fn canvas_native_release_ios_gl(context: i64) {
    if context == 0 {
        return;
    }
    let context = context as *mut GLContext;
    let _ = unsafe { Box::from_raw(context) };
}

pub fn canvas_native_get_gl_pointer(gl_context: i64) -> i64 {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *mut iOSGLContext;
    let gl_context = unsafe { &*gl_context };
    gl_context.gl_context.as_raw_inner() as i64
}

pub fn canvas_native_release_gl_pointer(gl_context: i64) {
    if gl_context == 0 {
        return;
    }
    let gl_context = gl_context as *const RefCell<canvas_core::gl::GLContextInner>;
    let _ = GLContext::from_raw_inner(gl_context);
}

pub fn canvas_native_context_2d_test(context: i64) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_cxx::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    {
        let mut ctx = context.get_context_mut();
        ctx.set_fill_style_with_color("red");
        ctx.fill_rect_xywh(0., 0., 300., 300.);
    }
    context.render();
}

/*
pub(crate) struct AutoreleasePool(*mut objc::runtime::Object);

impl AutoreleasePool {
    fn new() -> Self {
        Self(unsafe { NSAutoreleasePool::new(cocoa::base::nil) })
    }
}

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        #[allow(clippy::let_unit_value)]
            unsafe {
            // the unit value here is needed  to type the return of msg_send().
            let () = msg_send![self.0, release];
        }
    }
}
 */
