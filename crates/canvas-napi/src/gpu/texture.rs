use canvas_c::webgpu::enums::{
  CanvasOptionalGPUTextureFormat, CanvasOptionalTextureViewDimension, CanvasTextureDimension,
  CanvasTextureViewDimension,
};
use canvas_c::webgpu::gpu_texture::{CanvasCreateTextureViewDescriptor, CanvasGPUTexture};
use canvas_c::webgpu::structs::CanvasImageSubresourceRange;
use napi::bindgen_prelude::Unknown;
use napi_derive::napi;

use crate::gpu::handle::Handle;
use crate::gpu::parse::{
  aspect, c_str, int32, is_object, label, string, take_string, texture_format, texture_format_name,
  uint32, view_dimension,
};
use crate::gpu::texture_view::g_p_u_texture_view;

#[napi(js_name = "GPUTexture")]
pub struct g_p_u_texture {
  pub(crate) texture: Handle<CanvasGPUTexture>,
}

impl g_p_u_texture {
  pub(crate) unsafe fn from_raw(texture: *const CanvasGPUTexture) -> Option<Self> {
    unsafe { Handle::from_raw(texture) }.map(|texture| Self { texture })
  }

  /// A view of the texture (`descriptor` as `createView` reads it, or the default view).
  pub(crate) fn create_view_raw(
    texture: *const CanvasGPUTexture,
    descriptor: Option<&Unknown>,
  ) -> Option<g_p_u_texture_view> {
    if texture.is_null() {
      return None;
    }
    let descriptor = descriptor.filter(|d| is_object(d));
    let label = descriptor.and_then(label);
    let range = descriptor.map(|d| CanvasImageSubresourceRange {
      aspect: aspect(string(d, c"aspect")),
      base_mip_level: uint32(d, c"baseMipLevel").unwrap_or(0),
      mip_level_count: int32(d, c"mipLevelCount").unwrap_or(-1),
      base_array_layer: uint32(d, c"baseArrayLayer").unwrap_or(0),
      array_layer_count: int32(d, c"arrayLayerCount").unwrap_or(-1),
    });
    let desc = descriptor
      .zip(range.as_ref())
      .map(|(d, range)| CanvasCreateTextureViewDescriptor {
        label: c_str(&label),
        format: match string(d, c"format").and_then(|f| texture_format(&f)) {
          Some(format) => CanvasOptionalGPUTextureFormat::Some(format),
          None => CanvasOptionalGPUTextureFormat::None,
        },
        dimension: match string(d, c"dimension").and_then(|v| view_dimension(&v)) {
          Some(CanvasTextureViewDimension::D1) => CanvasOptionalTextureViewDimension::D1,
          Some(CanvasTextureViewDimension::D2) => CanvasOptionalTextureViewDimension::D2,
          Some(CanvasTextureViewDimension::D2Array) => CanvasOptionalTextureViewDimension::D2Array,
          Some(CanvasTextureViewDimension::Cube) => CanvasOptionalTextureViewDimension::Cube,
          Some(CanvasTextureViewDimension::CubeArray) => {
            CanvasOptionalTextureViewDimension::CubeArray
          }
          Some(CanvasTextureViewDimension::D3) => CanvasOptionalTextureViewDimension::D3,
          None => CanvasOptionalTextureViewDimension::None,
        },
        range,
        // 0: the texture's usage (the WebGPU default).
        usage: uint32(d, c"usage").unwrap_or(0),
      });
    let view = unsafe {
      canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_create_texture_view(
        texture,
        desc
          .as_ref()
          .map_or(std::ptr::null(), |desc| desc as *const _),
      )
    };
    unsafe { Handle::from_raw(view) }.map(|texture_view| g_p_u_texture_view { texture_view })
  }
}

#[napi]
impl g_p_u_texture {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(
        canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_label(self.texture.ptr()),
      )
    }
    .unwrap_or_default()
  }

  #[napi(getter)]
  pub fn get_depth_or_array_layers(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_depth_or_array_layers(
      self.texture.ptr(),
    )
  }

  #[napi(getter)]
  pub fn get_dimension(&self) -> &'static str {
    match canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_dimension(
      self.texture.ptr(),
    ) {
      CanvasTextureDimension::D1 => "1d",
      CanvasTextureDimension::D2 => "2d",
      CanvasTextureDimension::D3 => "3d",
    }
  }

  #[napi(getter)]
  pub fn get_format(&self) -> String {
    texture_format_name(
      canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_format(self.texture.ptr()),
    )
  }

  #[napi(getter)]
  pub fn get_mip_level_count(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_mip_level_count(
      self.texture.ptr(),
    )
  }

  #[napi(getter)]
  pub fn get_sample_count(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_sample_count(self.texture.ptr())
  }

  #[napi(getter)]
  pub fn get_width(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_width(self.texture.ptr())
  }

  #[napi(getter)]
  pub fn get_height(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_height(self.texture.ptr())
  }

  #[napi(getter)]
  pub fn get_usage(&self) -> u32 {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_usage(self.texture.ptr())
  }

  #[napi(ts_args_type = "descriptor?: object")]
  pub fn create_view(&self, descriptor: Option<Unknown>) -> Option<g_p_u_texture_view> {
    Self::create_view_raw(self.texture.ptr(), descriptor.as_ref())
  }

  #[napi]
  pub fn destroy(&self) {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_destroy(self.texture.ptr())
  }

  /// Drops this wrapper's reference now (packages/canvas does for swapchain textures once the
  /// frame is presented) without destroying the texture.
  #[napi(js_name = "__releaseHandle")]
  pub fn release_handle(&self) {
    self.texture.release();
  }
}
