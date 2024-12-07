use crate::gpu::enums::{GPUTextureDimension, GPUTextureFormat};
use napi::*;
use napi_derive::napi;
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

  #[napi(getter)]
  pub fn get_depth_or_array_layers(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_depth_or_array_layers(
      Arc::as_ptr(&self.texture),
    )
  }
  #[napi(getter)]
  pub fn get_dimension(&self) -> GPUTextureDimension {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_dimension(Arc::as_ptr(
      &self.texture,
    ))
    .into()
  }
  #[napi(getter)]
  pub fn get_format(&self) -> GPUTextureFormat {
    let format = canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_format(
      Arc::as_ptr(&self.texture),
    );
    format.into()
  }
  #[napi(getter)]
  pub fn get_mip_level_count(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_mip_level_count(Arc::as_ptr(
      &self.texture,
    ))
  }
  #[napi(getter)]
  pub fn get_sample_count(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_sample_count(Arc::as_ptr(
      &self.texture,
    ))
  }
  #[napi(getter)]
  pub fn get_width(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_width(Arc::as_ptr(
      &self.texture,
    ))
  }
  #[napi(getter)]
  pub fn get_height(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_height(Arc::as_ptr(
      &self.texture,
    ))
  }
  #[napi(getter)]
  pub fn get_usage(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_usage(Arc::as_ptr(
      &self.texture,
    ))
  }

  #[napi]
  pub fn destroy(&self) {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_destroy(Arc::as_ptr(&self.texture))
  }
}
