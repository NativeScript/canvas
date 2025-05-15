use crate::gpu::enums::{
  GPUTextureAspect, GPUTextureDimension, GPUTextureFormat, GPUTextureViewDimension,
};
use crate::gpu::objects::GPUTextureViewDescriptor;
use crate::gpu::texture_view::g_p_u_texture_view;
use canvas_c::webgpu::enums::{CanvasOptionalGPUTextureFormat, CanvasOptionalTextureViewDimension};
use canvas_c::webgpu::gpu_texture::CanvasCreateTextureViewDescriptor;
use canvas_c::webgpu::structs::CanvasImageSubresourceRange;
use napi::*;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;
use napi::bindgen_prelude::ClassInstance;

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
  pub fn create_view(&self, env: Env, descriptor: Option<GPUTextureViewDescriptor>) -> Result<ClassInstance<g_p_u_texture_view>> {
    let mut descriptor_ptr = std::ptr::null();
    let mut descriptor_value = None;
    let mut label: Option<CString> = None;
    let mut label_ptr = std::ptr::null();

    let mut range: Option<CanvasImageSubresourceRange> = None;
    let mut range_ptr: *const CanvasImageSubresourceRange = std::ptr::null();

    if let Some(descriptor) = descriptor {
      label = descriptor.label.map(|label| CString::new(label).unwrap());
      label_ptr = label
        .as_ref()
        .map_or(std::ptr::null(), |label| label.as_ptr());

      range = Some(CanvasImageSubresourceRange {
        aspect: descriptor.aspect.unwrap_or(GPUTextureAspect::all).into(),
        base_mip_level: descriptor.base_mip_level.unwrap_or(0),
        mip_level_count: descriptor
          .mip_level_count
          .and_then(|v| Some(v as i32))
          .unwrap_or(-1),
        base_array_layer: descriptor.base_array_layer.unwrap_or(0),
        array_layer_count: descriptor
          .array_layer_count
          .and_then(|v| Some(v as i32))
          .unwrap_or(-1),
      });

      if let Some(range) = range.as_ref() {
        range_ptr = range;
      }

      descriptor_value = Some(CanvasCreateTextureViewDescriptor {
        label: label_ptr,
        format: descriptor
          .format
          .map(|format| CanvasOptionalGPUTextureFormat::Some(format.into()))
          .unwrap_or(CanvasOptionalGPUTextureFormat::None),
        dimension: descriptor
          .dimension
          .map(|dimension| match dimension {
            GPUTextureViewDimension::d1 => CanvasOptionalTextureViewDimension::D1,
            GPUTextureViewDimension::d2 => CanvasOptionalTextureViewDimension::D2,
            GPUTextureViewDimension::array2d => CanvasOptionalTextureViewDimension::D2Array,
            GPUTextureViewDimension::cube => CanvasOptionalTextureViewDimension::Cube,
            GPUTextureViewDimension::cubeArray => CanvasOptionalTextureViewDimension::CubeArray,
            GPUTextureViewDimension::d3 => CanvasOptionalTextureViewDimension::D3,
          })
          .unwrap_or(CanvasOptionalTextureViewDimension::None),
        range: range_ptr,
      });
    }

    if let Some(descriptor_value) = descriptor_value.as_ref() {
      descriptor_ptr = descriptor_value;
    }

    let view = unsafe {
      Arc::from_raw(
        canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_create_texture_view(
          Arc::as_ptr(&self.texture),
          descriptor_ptr,
        ),
      )
    };

    g_p_u_texture_view { texture_view: view }.into_instance(env)
  }

  #[napi]
  pub fn destroy(&self) {
    canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_destroy(Arc::as_ptr(&self.texture))
  }
}
