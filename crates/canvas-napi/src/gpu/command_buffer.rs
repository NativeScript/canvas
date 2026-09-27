use napi_derive::napi;

use crate::gpu::handle::Handle;
use crate::gpu::parse::take_string;

#[napi(js_name = "GPUCommandBuffer")]
pub struct g_p_u_command_buffer {
  pub(crate) buffer: Handle<canvas_c::webgpu::gpu_command_buffer::CanvasGPUCommandBuffer>,
}

#[napi]
impl g_p_u_command_buffer {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(
        canvas_c::webgpu::gpu_command_buffer::canvas_native_webgpu_command_buffer_get_label(
          self.buffer.ptr(),
        ),
      )
    }
    .unwrap_or_default()
  }

  /// Releases the command buffer now (packages/canvas does right after `submit`).
  #[napi]
  pub fn destroy(&self) {
    self.buffer.release();
  }
}
