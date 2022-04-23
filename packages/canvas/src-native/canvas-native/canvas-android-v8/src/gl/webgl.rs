use std::ffi::{CStr, CString};

use crate::gl::prelude::WebGLState;

pub fn canvas_native_webgl_active_texture(texture: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::glActiveTexture(texture);
    }
}

pub fn canvas_native_webgl_attach_shader(program: u32, shader: u32, state: &mut WebGLState) {
    state.make_current();
    unsafe {
        gl_bindings::glAttachShader(program, shader);
    }
}

pub fn canvas_native_webgl_bind_attrib_location(
    program: u32,
    index: u32,
    name: &str,
    state: &mut WebGLState,
) {
    state.make_current();
    let name = CString::new(name).unwrap();
    unsafe {
        gl_bindings::glBindAttribLocation(program, index, name.as_ptr());
    }
}
