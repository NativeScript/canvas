use napi::*;
use napi_derive::napi;

#[napi(js_name = "WebGLVertexArrayObject")]
pub struct WebGLVertexArrayObject(pub(crate) u32);