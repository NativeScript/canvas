use napi::*;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;
#[napi(js_name = "GPURenderBundle")]
pub struct g_p_u_render_bundle {
  pub(crate) render_bundle: Arc<canvas_c::webgpu::gpu_render_bundle::CanvasGPURenderBundle>,
}

#[napi]
impl g_p_u_render_bundle {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_render_bundle::canvas_native_webgpu_render_bundle_get_label(
        Arc::as_ptr(&self.render_bundle),
      )
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
