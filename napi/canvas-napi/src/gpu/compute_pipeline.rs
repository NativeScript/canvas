use crate::gpu::bind_group_layout::g_p_u_bind_group_layout;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi]
pub struct g_p_u_compute_pipeline {
  pub(crate) pipeline: *const canvas_c::webgpu::gpu_compute_pipeline::CanvasGPUComputePipeline,
}

#[napi]
impl g_p_u_compute_pipeline {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_compute_pipeline::canvas_native_webgpu_compute_pipeline_get_label(
        self.pipeline,
      )
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }

  #[napi]
  pub fn get_bind_group_layout(&self, index: u32) -> Option<g_p_u_bind_group_layout> {
    let layout = unsafe {
      canvas_c::webgpu::gpu_compute_pipeline::canvas_native_webgpu_compute_pipeline_get_bind_group_layout(self.pipeline, index)
    };
    if layout.is_null() {
      return None;
    }
    Some(g_p_u_bind_group_layout {
      layout: unsafe { Arc::from_raw(layout) },
    })
  }
}
