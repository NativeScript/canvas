use napi_derive::napi;

use crate::gpu::handle::Handle;
use crate::gpu::parse::take_string;

#[napi(js_name = "GPUTextureView")]
pub struct g_p_u_texture_view {
  pub(crate) texture_view: Handle<canvas_c::webgpu::gpu_texture_view::CanvasGPUTextureView>,
}

#[napi]
impl g_p_u_texture_view {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(
        canvas_c::webgpu::gpu_texture_view::canvas_native_webgpu_texture_view_get_label(
          self.texture_view.ptr(),
        ),
      )
    }
    .unwrap_or_default()
  }

  /// Releases the view now (packages/canvas does once a swapchain frame is presented).
  #[napi]
  pub fn destroy(&self) {
    self.texture_view.release();
  }
}
