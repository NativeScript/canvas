use napi::*;

use napi_derive::napi;

#[napi(js_name = "WebGLRenderbuffer")]
pub struct WebGLRenderbuffer(pub(crate) u32);