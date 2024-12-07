use std::ffi::CString;
use std::sync::Arc;

#[napi]
pub struct g_p_u_texture {
  pub(crate) texture: Arc<canvas_c::webgpu::gpu_texture::CanvasGPUTexture>,
}

#[napi]
impl g_p_u_texture {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_label(Arc::as_ptr(
        &self.texture,
      ))
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }
}
