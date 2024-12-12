use crate::c2d::image_data::ImageData;
use crate::c2d::CanvasRenderingContext2D;
use crate::gl::web_g_l_rendering_context;
use crate::gl2::web_g_l_2_rendering_context;
use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::command_buffer::g_p_u_command_buffer;
use crate::gpu::context::g_p_u_canvas_context;
use crate::gpu::enums::GPUTextureAspect;
use crate::gpu::objects::{
  GPUExtent3DDict, GPUImageCopyExternalImage, GPUImageCopyTextureTagged, GPUOrigin2DDict,
  GPUOrigin3DDict,
};
use crate::image_asset::ImageAsset;
use canvas_c::webgpu::gpu_command_encoder::CanvasImageCopyTexture;
use canvas_c::webgpu::structs::{
  CanvasExtent3d, CanvasImageCopyCanvasRenderingContext2D, CanvasImageCopyExternalImage,
  CanvasImageCopyGPUContext, CanvasImageCopyImageAsset, CanvasImageCopyWebGL, CanvasOrigin2d,
  CanvasOrigin3d,
};
use napi::bindgen_prelude::{
  Buffer, ClassInstance, Either3, Either4, Either5, Either6, Float32Array, ObjectFinalize,
};
use napi::*;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi]
pub struct g_p_u_queue {
  pub(crate) queue: Arc<canvas_c::webgpu::gpu_queue::CanvasGPUQueue>,
}

#[napi]
impl g_p_u_queue {
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
  pub fn copy_external_image_to_texture(
    &self,
    source: GPUImageCopyExternalImage,
    destination: GPUImageCopyTextureTagged,
    copy_size: Either<Vec<u32>, GPUExtent3DDict>,
  ) {
    let size = match copy_size {
      Either::A(array) => CanvasExtent3d {
        width: *array.get(0).unwrap_or(&0),
        height: *array.get(1).unwrap_or(&1),
        depth_or_array_layers: *array.get(2).unwrap_or(&1),
      },
      Either::B(dict) => CanvasExtent3d {
        width: dict.width,
        height: dict.height.unwrap_or(1),
        depth_or_array_layers: dict.depth_or_array_layers.unwrap_or(1),
      },
    };
    let origin = source
      .origin
      .map(|origin| match origin {
        Either::A(array) => CanvasOrigin2d {
          x: *array.get(0).unwrap_or(&0),
          y: *array.get(1).unwrap_or(&0),
        },
        Either::B(dict) => CanvasOrigin2d {
          x: dict.x.unwrap_or_default(),
          y: dict.y.unwrap_or_default(),
        },
      })
      .unwrap_or(CanvasOrigin2d { x: 0, y: 0 });
    let dst = CanvasImageCopyTexture {
      texture: Arc::as_ptr(&destination.texture.texture),
      mip_level: destination.mip_level.unwrap_or(0),
      origin: destination
        .origin
        .map(|origin| match origin {
          Either::A(array) => CanvasOrigin3d {
            x: *array.get(0).unwrap_or(&0),
            y: *array.get(1).unwrap_or(&0),
            z: *array.get(2).unwrap_or(&0),
          },
          Either::B(dict) => CanvasOrigin3d {
            x: dict.x,
            y: dict.y,
            z: dict.z,
          },
        })
        .unwrap_or_default(),
      aspect: destination.aspect.unwrap_or(GPUTextureAspect::all).into(),
    };
    match source.source {
      Either6::A(image_data) => {
        let data = unsafe { &*image_data.data };
        let width = data.inner().width() as u32;
        let height = data.inner().height() as u32;
        let data = data.inner().data();
        let src = CanvasImageCopyExternalImage {
          source: data.as_ptr(),
          source_size: data.len(),
          origin,
          flip_y: source.flip_y.unwrap_or(false),
          width,
          height,
        };
        unsafe {
          canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_external_image_to_texture(
            Arc::as_ptr(&self.queue),
            &src,
            &dst,
            &size,
          )
        }
      }
      Either6::B(image_asset) => {
        let src = CanvasImageCopyImageAsset {
          source: Arc::as_ptr(&image_asset.asset),
          origin,
          flip_y: source.flip_y.unwrap_or(false),
        };
        unsafe {
          canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_image_asset_to_texture(
            Arc::as_ptr(&self.queue),
            &src,
            &dst,
            &size,
          )
        }
      }
      Either6::C(c2d) => {
        let src = CanvasImageCopyCanvasRenderingContext2D {
          source: c2d.context,
          origin,
          flip_y: source.flip_y.unwrap_or(false),
        };
        unsafe {
          canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_context_to_texture(
            Arc::as_ptr(&self.queue),
            &src,
            &dst,
            &size,
          )
        }
      }
      Either6::D(gl) => {
        let src = CanvasImageCopyWebGL {
          source: gl.state,
          origin,
          flip_y: source.flip_y.unwrap_or(false),
        };
        unsafe {
          canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_webgl_to_texture(
            Arc::as_ptr(&self.queue),
            &src,
            &dst,
            &size,
          )
        }
      }
      Either6::E(gl2) => {
        let src = CanvasImageCopyWebGL {
          source: gl2.state,
          origin,
          flip_y: source.flip_y.unwrap_or(false),
        };
        unsafe {
          canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_webgl_to_texture(
            Arc::as_ptr(&self.queue),
            &src,
            &dst,
            &size,
          )
        }
      }
      Either6::F(gpu) => {
        let src = CanvasImageCopyGPUContext {
          source: Arc::as_ptr(&gpu.context),
          origin,
          flip_y: source.flip_y.unwrap_or(false),
        };
        unsafe {
          canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_gpu_context_to_texture(
            Arc::as_ptr(&self.queue),
            &src,
            &dst,
            &size,
          )
        }
      }
    }
  }

  #[napi]
  pub fn write_buffer(
    &self,
    buffer: &g_p_u_buffer,
    buffer_offset: i64,
    data: Either4<JsArrayBuffer, &[u8], &[f32], &[u32]>,
    data_offset: Option<i64>,
    size: Option<i64>,
  ) -> Result<()> {
    match data {
      Either4::A(buffer_data) => unsafe {
        let buffer_data = buffer_data.into_value()?;
        canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_write_buffer(
          Arc::as_ptr(&self.queue),
          Arc::as_ptr(&buffer.buffer),
          buffer_offset as u64,
          buffer_data.as_ptr(),
          buffer_data.len(),
          data_offset.unwrap_or(0) as usize,
          size.unwrap_or(-1) as isize,
        );
        Ok(())
      },
      Either4::B(view) => {
        unsafe {
          canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_write_buffer(
            Arc::as_ptr(&self.queue),
            Arc::as_ptr(&buffer.buffer),
            buffer_offset as u64,
            view.as_ptr(),
            view.len(),
            data_offset.unwrap_or(0) as usize,
            size.unwrap_or(-1) as isize,
          );
        }
        Ok(())
      }
      Either4::C(view) => {
        unsafe {
          canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_write_buffer(
            Arc::as_ptr(&self.queue),
            Arc::as_ptr(&buffer.buffer),
            buffer_offset as u64,
            view.as_ptr() as *const u8,
            view.len() * size_of::<f32>(),
            data_offset.unwrap_or(0) as usize,
            size.unwrap_or(-1) as isize,
          );
        }
        Ok(())
      }
      Either4::D(view) => {
        unsafe {
          canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_write_buffer(
            Arc::as_ptr(&self.queue),
            Arc::as_ptr(&buffer.buffer),
            buffer_offset as u64,
            view.as_ptr() as *const u8,
            view.len() * size_of::<u32>(),
            data_offset.unwrap_or(0) as usize,
            size.unwrap_or(-1) as isize,
          );
        }
        Ok(())
      }
    }
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
