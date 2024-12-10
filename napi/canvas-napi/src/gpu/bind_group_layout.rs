use napi::*;
use napi_derive::*;
use std::ffi::CString;
use std::sync::Arc;

#[napi]
pub struct g_p_u_bind_group_layout {
  pub(crate) layout: Arc<canvas_c::webgpu::gpu_bind_group_layout::CanvasGPUBindGroupLayout>,
}

#[napi]
impl g_p_u_bind_group_layout {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_bind_group_layout::canvas_native_webgpu_bind_group_layout_get_label(
        Arc::as_ptr(&self.layout),
      )
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
