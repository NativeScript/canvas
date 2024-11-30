use napi::*;

use napi_derive::napi;

#[napi(js_name = "WebGLBuffer")]
pub struct WebGLBuffer(pub(crate) u32);