use jni::JNIEnv;
use jni::sys::{jboolean, jlong, JNI_FALSE, JNI_TRUE};

pub(crate) mod surface_texture;
pub mod texture_render;
pub mod webgl2_rendering_context;
pub mod webgl_rendering_context;

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_Utils_nativeMakeStateContextCurrent(
    _: JNIEnv,
    _: jni::objects::JClass,
    state: jlong,
) -> jboolean {
    if state == 0 {
        return JNI_FALSE;
    }
    let state = unsafe { state as *mut crate::gl::prelude::WebGLState };
    if state.is_null() {
        return JNI_FALSE;
    }
    let mut state = &mut *state;
    if state.make_current() {
        return JNI_TRUE;
    }
    return JNI_FALSE;
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_Utils_nativeGetFlipYWebGL(
    _: JNIEnv,
    _: jni::objects::JClass,
    state: jlong,
) -> jboolean {
    if state == 0 {
        return JNI_FALSE;
    }
    let state = unsafe { state as *mut crate::gl::prelude::WebGLState };
    if state.is_null() {
        return JNI_FALSE;
    }
    let mut state = &mut *state;
    if state.get_flip_y() {
        return JNI_TRUE;
    }
    return JNI_FALSE;
}