use std::sync::Arc;

use napi_derive::napi;

use crate::gpu::parse::take_string;

/// `device.importExternalTexture(...)`: a video frame sampled as a `texture_external`.
#[napi(js_name = "GPUExternalTexture")]
pub struct g_p_u_external_texture {
  pub(crate) texture: Arc<canvas_c::webgpu::gpu_external_texture::CanvasGPUExternalTexture>,
}

#[napi]
impl g_p_u_external_texture {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(
        canvas_c::webgpu::gpu_external_texture::canvas_native_webgpu_external_texture_get_label(
          Arc::as_ptr(&self.texture),
        ),
      )
    }
    .unwrap_or_default()
  }
}
