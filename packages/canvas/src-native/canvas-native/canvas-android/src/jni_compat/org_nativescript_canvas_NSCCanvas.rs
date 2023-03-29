use std::cell::RefCell;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jfloat, jint, jlong, jobject, JNI_TRUE};
use jni::JNIEnv;
use ndk::native_window::NativeWindow;
use parking_lot::RwLock;
use raw_window_handle::HasRawWindowHandle;

use canvas_core::context_attributes::ContextAttributes;
use canvas_core::gl::GLContext;

#[allow(dead_code)]
pub(crate) struct AndroidGLContext {
    pub(crate) contextAttributes: ContextAttributes,
    pub(crate) gl_context: GLContext,
    android_window: Option<NativeWindow>,
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCCanvas_nativeInitGL(
    mut env: JNIEnv,
    _: JClass,
    surface: jobject,
    alpha: jboolean,
    antialias: jboolean,
    depth: jboolean,
    fail_if_major_performance_caveat: jboolean,
    power_preference: JString,
    premultiplied_alpha: jboolean,
    preserve_drawing_buffer: jboolean,
    stencil: jboolean,
    desynchronized: jboolean,
    xr_compatible: jboolean,
    version: jint,
    is_canvas: jboolean,
) -> jlong {
    unsafe {
        if let Some(window) = NativeWindow::from_surface(env.get_native_interface(), surface) {
            if version == 1 && !GLContext::has_gl2support() {
                return 0;
            }
            let power_preference = match env.get_string(&power_preference) {
                Ok(value) => value.to_string_lossy().to_string(),
                Err(_) => "default".to_string(),
            };

            let mut attrs = ContextAttributes::new(
                alpha == JNI_TRUE,
                antialias == JNI_TRUE,
                depth == JNI_TRUE,
                fail_if_major_performance_caveat == JNI_TRUE,
                power_preference.as_ref(),
                premultiplied_alpha == JNI_TRUE,
                preserve_drawing_buffer == JNI_TRUE,
                stencil == JNI_TRUE,
                desynchronized == JNI_TRUE,
                xr_compatible == JNI_TRUE,
                is_canvas == JNI_TRUE,
            );
            if let Some(gl_context) = GLContext::create_window_context(
                &mut attrs,
                window.width(),
                window.height(),
                window.raw_window_handle(),
            ) {
                return Box::into_raw(Box::new(AndroidGLContext {
                    android_window: Some(window),
                    gl_context,
                    contextAttributes: attrs,
                })) as jlong;
            }
        }
    }
    0
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCCanvas_nativeInitGLNoSurface(
    mut env: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
    alpha: jboolean,
    antialias: jboolean,
    depth: jboolean,
    fail_if_major_performance_caveat: jboolean,
    power_preference: JString,
    premultiplied_alpha: jboolean,
    preserve_drawing_buffer: jboolean,
    stencil: jboolean,
    desynchronized: jboolean,
    xr_compatible: jboolean,
    version: jint,
    is_canvas: jboolean,
) -> jlong {
    if version == 1 && !GLContext::has_gl2support() {
        return 0;
    }
    let power_preference = match env.get_string(&power_preference) {
        Ok(value) => value.to_string_lossy().to_string(),
        Err(_) => "default".to_string(),
    };

    let mut attrs = ContextAttributes::new(
        alpha == JNI_TRUE,
        antialias == JNI_TRUE,
        depth == JNI_TRUE,
        fail_if_major_performance_caveat == JNI_TRUE,
        power_preference.as_ref(),
        premultiplied_alpha == JNI_TRUE,
        preserve_drawing_buffer == JNI_TRUE,
        stencil == JNI_TRUE,
        desynchronized == JNI_TRUE,
        xr_compatible == JNI_TRUE,
        is_canvas == JNI_TRUE,
    );

    if let Some(gl_context) = GLContext::create_pbuffer(&mut attrs, width, height) {
        return Box::into_raw(Box::new(AndroidGLContext {
            android_window: None,
            gl_context,
            contextAttributes: attrs,
        })) as jlong;
    }
    0
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCCanvas_nativeCreate2DContext(
    _env: JNIEnv,
    _: JClass,
    context: jlong,
    width: jint,
    height: jint,
    alpha: jboolean,
    density: jfloat,
    samples: jint,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    if context == 0 {
        return 0;
    }

    let context = context as *mut AndroidGLContext;
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
            alpha == JNI_TRUE,
            font_color,
            ppi,
            canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
        )),
        context.gl_context.clone(),
        alpha == JNI_TRUE,
    );

    //ctx_2d.get_context_mut().flush();
    //context.gl_context.swap_buffers();

    Box::into_raw(Box::new(ctx_2d)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCCanvas_nativeUpdateGLSurface(
    env: JNIEnv,
    _: JClass,
    surface: jobject,
    context: jlong,
) {
    if context == 0 {
        return;
    }
    let context = context as *mut AndroidGLContext;
    let context = unsafe { &mut *context };
    unsafe {
        if let Some(window) = NativeWindow::from_surface(env.get_native_interface(), surface) {
            context.gl_context.set_window_surface(
                &mut context.contextAttributes,
                window.width(),
                window.height(),
                window.raw_window_handle(),
            );
            context.android_window = Some(window);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCCanvas_nativeReleaseGL(
    _env: JNIEnv,
    _: JClass,
    context: jlong,
) {
    if context == 0 {
        return;
    }
    let context = context as *mut GLContext;
    let _ = unsafe { Box::from_raw(context) };
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCCanvas_nativeGetGLPointer(
    _env: JNIEnv,
    _: JClass,
    gl_context: jlong,
) -> jlong {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *mut AndroidGLContext;
    let gl_context = unsafe { &*gl_context };
    gl_context.gl_context.as_raw_inner() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCCanvas_nativeReleaseGLPointer(
    _env: JNIEnv,
    _: JClass,
    gl_context: jlong,
) {
    if gl_context == 0 {
        return;
    }
    let gl_context = gl_context as *const RefCell<canvas_core::gl::GLContextInner>;
    let _ = GLContext::from_raw_inner(gl_context);
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCCanvas_nativeContext2DTest(
    _env: JNIEnv,
    _: JClass,
    context: jlong,
) {
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
