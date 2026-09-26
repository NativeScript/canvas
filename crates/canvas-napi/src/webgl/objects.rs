//! GL object handles handed to JS (`WebGLBuffer.h`, `WebGLTexture.h`, … in the V8 bindings).
//! Each wraps the GL name; `0`/`null` passed back means "unbind". Types used only by WebGL 2
//! (samplers, sync, transform feedback) live in `crate::webgl2`.

use napi::sys;

use crate::util::native::{Native, NativeType};

macro_rules! gl_handle {
    ($($name:ident: $ty:ty),* $(,)?) => {
        $(
            #[derive(Clone, Copy, Debug)]
            pub struct $name(pub $ty);

            impl Native for $name {
                const KIND: NativeType = NativeType::$name;
            }
        )*
    };
}

gl_handle! {
    WebGLBuffer: u32,
    WebGLFramebuffer: u32,
    WebGLProgram: u32,
    WebGLRenderbuffer: u32,
    WebGLShader: u32,
    WebGLTexture: u32,
    WebGLUniformLocation: i32,
    WebGLVertexArrayObject: u32,
    WebGLQuery: u32,
}

pub unsafe fn init(_env: sys::napi_env, _exports: sys::napi_value) {}
