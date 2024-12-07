use napi_derive::napi;
use std::ffi::CString;

#[napi(js_name = "GPURenderBundleEncoder")]
pub struct GPURenderBundleEncoder {
  pub(crate) encoder:
    *const canvas_c::webgpu::gpu_render_bundle_encoder::CanvasGPURenderBundleEncoder,
}

#[napi]
impl GPURenderBundleEncoder {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_get_label(self.encoder)
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
