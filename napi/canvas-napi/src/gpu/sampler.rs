use napi::*;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi(js_name = "GPUSampler")]
#[derive(Debug)]
pub struct g_p_u_sampler {
  pub(crate) sampler: Arc<canvas_c::webgpu::gpu_sampler::CanvasGPUSampler>,
}

#[napi]
impl g_p_u_sampler {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_sampler::canvas_native_webgpu_sampler_get_label(Arc::as_ptr(
        &self.sampler,
      ))
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}