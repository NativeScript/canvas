use napi::*;

use napi_derive::napi;

#[napi(js_name = "WebGLTexture")]
pub struct WebGLTexture(pub(crate) u32);