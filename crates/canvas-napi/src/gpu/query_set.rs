use napi::bindgen_prelude::ObjectFinalize;
use napi::*;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi(js_name = "GPUQuerySet")]
pub struct g_p_u_query_set {
  pub(crate) query: Arc<canvas_c::webgpu::gpu_query_set::CanvasGPUQuerySet>,
}

#[napi]
impl g_p_u_query_set {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_query_set::canvas_native_webgpu_query_set_get_label(Arc::as_ptr(
        &self.query,
      ))
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
