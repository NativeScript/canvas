use canvas_c::WebGLState;
use jni::sys::{jboolean, jlong, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;

pub mod st;
pub(crate) mod surface_texture;
pub mod texture_render;
pub mod webgl2_rendering_context;
pub mod webgl_rendering_context;

pub const TEXTURE_EXTERNAL_OES: u32 = 0x00008d65;

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_Utils_nativeMakeStateContextCurrent(
    _: JNIEnv,
    _: jni::objects::JClass,
    state: jlong,
) -> jboolean {
    if state == 0 {
        return JNI_FALSE;
    }

    let state =
        state as *mut WebGLState;

    if state.is_null() {
        return JNI_FALSE;
    }
    let state = &mut *state;

    if state.get_inner().make_current() {
        return JNI_TRUE;
    }
    JNI_FALSE
}
