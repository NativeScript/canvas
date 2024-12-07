use crate::gpu::command_buffer::g_p_u_command_buffer;
use napi::bindgen_prelude::ObjectFinalize;
use napi::*;
use std::ffi::CString;
use std::sync::Arc;

#[napi]
pub struct GPUQueue {
  queue: Arc<canvas_c::webgpu::gpu_queue::CanvasGPUQueue>,
}

#[napi]
impl GPUQueue {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_get_label(Arc::as_ptr(&self.queue))
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }

  #[napi]
  pub fn submit(&self, buffers: Vec<&g_p_u_command_buffer>) {
    let buffers = buffers
      .into_iter()
      .map(|buffer| Arc::as_ptr(&buffer.buffer))
      .collect::<Vec<_>>();
    unsafe {
      canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_submit(
        Arc::as_ptr(&self.queue),
        buffers.as_ptr(),
        buffers.len(),
      );
    }
  }
}
