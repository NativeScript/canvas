use napi::*;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;
#[napi(js_name = "GPUTextureView")]
pub struct g_p_u_texture_view {
  pub(crate) texture_view: Arc<canvas_c::webgpu::gpu_texture_view::CanvasGPUTextureView>,
}

#[napi]
impl g_p_u_texture_view {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_texture_view::canvas_native_webgpu_texture_view_get_label(Arc::as_ptr(
        &self.texture_view,
      ))
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
