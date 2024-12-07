use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi(js_name = "GPUComputePassEncoder")]
pub struct GPUComputePassEncoder {
  pub(crate) encoder: Arc<canvas_c::webgpu::gpu_compute_pass_encoder::CanvasGPUComputePassEncoder>,
}

#[napi]
impl GPUComputePassEncoder {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_get_label(Arc::as_ptr(&self.encoder))
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
