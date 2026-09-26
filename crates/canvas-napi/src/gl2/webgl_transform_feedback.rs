use napi::*;
use napi_derive::napi;

#[napi(js_name = "WebGLTransformFeedback")]
pub struct WebGLTransformFeedback(pub(crate) u32);