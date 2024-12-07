use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi]
pub struct g_p_u_pipeline_layout {
  pub(crate) layout: Arc<canvas_c::webgpu::gpu_pipeline_layout::CanvasGPUPipelineLayout>,
}

#[napi]
impl g_p_u_pipeline_layout {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_pipeline_layout::canvas_native_webgpu_pipeline_layout_get_label(
        Arc::as_ptr(&self.layout),
      )
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
