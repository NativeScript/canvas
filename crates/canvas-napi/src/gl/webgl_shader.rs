use napi::*;

use napi_derive::napi;

#[napi(js_name = "WebGLShader")]
pub struct WebGLShader(pub(crate) u32);