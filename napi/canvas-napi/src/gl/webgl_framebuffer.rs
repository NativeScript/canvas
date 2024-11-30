use napi::*;

use napi_derive::napi;

#[napi(js_name = "WebGLFramebuffer")]
pub struct WebGLFramebuffer(pub(crate) u32);