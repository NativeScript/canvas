use napi::*;
use napi_derive::napi;

#[napi(js_name = "WebGLQuery")]
pub struct WebGLQuery(pub(crate) u32);