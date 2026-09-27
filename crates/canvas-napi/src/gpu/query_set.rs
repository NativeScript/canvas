use std::sync::Arc;

use canvas_c::webgpu::enums::CanvasQueryType;
use napi_derive::napi;

use crate::gpu::parse::take_string;

#[napi(js_name = "GPUQuerySet")]
pub struct g_p_u_query_set {
  pub(crate) query: Arc<canvas_c::webgpu::gpu_query_set::CanvasGPUQuerySet>,
}

#[napi]
impl g_p_u_query_set {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(
        canvas_c::webgpu::gpu_query_set::canvas_native_webgpu_query_set_get_label(Arc::as_ptr(
          &self.query,
        )),
      )
    }
    .unwrap_or_default()
  }

  #[napi(getter)]
  pub fn get_count(&self) -> u32 {
    unsafe {
      canvas_c::webgpu::gpu_query_set::canvas_native_webgpu_query_set_get_count(Arc::as_ptr(
        &self.query,
      ))
    }
  }

  #[napi(getter, js_name = "type")]
  pub fn get_type(&self) -> &'static str {
    match unsafe {
      canvas_c::webgpu::gpu_query_set::canvas_native_webgpu_query_set_get_type(Arc::as_ptr(
        &self.query,
      ))
    } {
      CanvasQueryType::Occlusion => "occlusion",
      CanvasQueryType::Timestamp => "timestamp",
    }
  }

  #[napi]
  pub fn destroy(&self) {
    unsafe {
      canvas_c::webgpu::gpu_query_set::canvas_native_webgpu_query_set_destroy(Arc::as_ptr(
        &self.query,
      ))
    }
  }
}
