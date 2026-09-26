use napi::*;

use napi_derive::napi;

#[napi(js_name = "WebGLProgram")]
pub struct WebGLProgram(pub(crate) u32);