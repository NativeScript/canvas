use napi::*;
use napi_derive::napi;

#[napi(js_name = "WebGLUniformLocation")]
pub struct WebGLUniformLocation(pub(crate) i32);