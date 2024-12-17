use napi::*;
use napi_derive::napi;

#[napi(js_name = "WebGLSampler")]
pub struct WebGLSampler(pub(crate) u32);