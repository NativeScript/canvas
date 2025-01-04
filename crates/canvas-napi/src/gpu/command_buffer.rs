use std::ffi::CString;
use std::sync::Arc;
use napi::*;
use napi_derive::napi;

#[napi]
pub struct g_p_u_command_buffer {
  pub(crate) buffer: Arc<canvas_c::webgpu::gpu_command_buffer::CanvasGPUCommandBuffer>,
}

#[napi]
impl g_p_u_command_buffer {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_command_buffer::canvas_native_webgpu_command_buffer_get_label(
        Arc::as_ptr(&self.buffer),
      )
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
