use std::borrow::Cow;

use jni::objects::{JClass, JString};
use jni::strings::JavaStr;
use jni::sys::{jboolean, jint, jlong, jobject, jstring, JNI_TRUE};
use jni::JNIEnv;
use ndk::native_window::NativeWindow;
use parking_lot::RwLock;
use raw_window_handle::HasRawWindowHandle;

use canvas_core::context_attributes::ContextAttributes;
use canvas_core::gl::GLContext;

struct AndroidGLContext {
    contextAttributes: ContextAttributes,
    gl_context: GLContext,
    android_window: NativeWindow,
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
            if let Some(gl_context) = GLContext::create_window_surface(
                &mut attrs,
                window.width(),
                window.height(),
                window.raw_window_handle(),
            ) {
                gl_context.make_current();
                return Box::into_raw(Box::new(AndroidGLContext {
                    android_window: window,
                    gl_context,
                    contextAttributes: attrs,
                })) as jlong;
            }
        }
    }
    0
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCCanvas_nativeUpdateGLSurface(
    mut env: JNIEnv,
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
                &context.contextAttributes,
                window.width(),
                window.height(),
                window.raw_window_handle(),
            );
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
    let context: *mut GLContext = unsafe { context as _ };
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
    let gl_context: *mut AndroidGLContext = unsafe { gl_context as _ };
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
    let gl_context: *const RwLock<canvas_core::gl::GLContextInner> = unsafe { gl_context as _ };
    let _ = GLContext::from_raw_inner(gl_context);
}
