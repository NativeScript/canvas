use napi_derive::napi;
use std::ffi::CString;

#[napi(js_name = "GPURenderBundle")]
pub struct GPURenderBundle {
  pub(crate) render_bundle: *const canvas_c::webgpu::gpu_render_bundle::CanvasGPURenderBundle,
}

#[napi]
impl GPURenderBundle {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_render_bundle::canvas_native_webgpu_render_bundle_get_label(
        self.render_bundle,
      )
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
