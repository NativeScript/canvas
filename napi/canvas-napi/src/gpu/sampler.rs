use napi_derive::napi;
use std::ffi::CString;

#[napi]
pub struct g_p_u_sample {
  pub(crate) sampler: *const canvas_c::webgpu::gpu_sampler::CanvasGPUSampler,
}

#[napi]
impl g_p_u_sample {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_sampler::canvas_native_webgpu_sampler_get_label(self.sampler)
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
