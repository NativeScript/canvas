use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::command_buffer::g_p_u_command_buffer;
use crate::gpu::compute_pass_encoder::GPUComputePassEncoder;
use crate::gpu::objects::{
  GPUComputePassDescriptor, GPUExtent3DDict, GPUImageCopyBuffer, GPUImageCopyTexture,
  GPURenderPassDescriptor,
};
use crate::gpu::query_set::GPUQuerySet;
use crate::gpu::render_pass_encoder::GPURenderPassEncoder;
use canvas_c::webgpu::gpu_query_set::CanvasGPUQuerySet;
use canvas_c::webgpu::structs::{
  CanvasColor, CanvasPassChannelColor, CanvasRenderPassColorAttachment,
  CanvasRenderPassDepthStencilAttachment,
};
use napi::*;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi(js_name = "GPUCommandEncoder")]
pub struct GPUCommandEncoder {
  pub(crate) encoder: Arc<canvas_c::webgpu::gpu_command_encoder::CanvasGPUCommandEncoder>,
}

#[napi(object)]
pub struct GPUCommandEncoderFinishDescriptor {
  pub label: Option<String>,
}

#[napi]
impl GPUCommandEncoder {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_get_label(
        Arc::as_ptr(&self.encoder),
      )
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }

  #[napi]
  pub fn begin_compute_pass(
    &self,
    descriptor: Option<GPUComputePassDescriptor>,
  ) -> GPUComputePassEncoder {
    let mut query_set = std::ptr::null();
    let mut label_ptr = std::ptr::null();
    let mut label = None;
    let mut beginning_of_pass_write_index = -1;
    let mut end_of_pass_write_index = -1;
    if let Some(descriptor) = descriptor {
      label = descriptor.label.map(|l| CString::new(l).unwrap());
      if let Some(timestamp_writes) = descriptor.timestamp_writes {
        query_set = Arc::as_ptr(&timestamp_writes.query_qet.query);
        beginning_of_pass_write_index = timestamp_writes.beginning_of_pass_write_index;
        end_of_pass_write_index = timestamp_writes.end_of_pass_write_index;
      }
      if let Some(label) = &label {
        label_ptr = label.as_ptr();
      }
    }
    let pass = unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_begin_compute_pass(
        Arc::as_ptr(&self.encoder),
        query_set,
        label_ptr,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
      )
    };
    GPUComputePassEncoder {
      encoder: unsafe { Arc::from_raw(pass) },
    }
  }

  #[napi]
  pub fn begin_render_pass(&self, descriptor: GPURenderPassDescriptor) -> GPURenderPassEncoder {
    let mut label_ptr = std::ptr::null();
    let label = descriptor.label.map(|l| CString::new(l).unwrap());

    if let Some(label) = &label {
      label_ptr = label.as_ptr();
    }

    let color_attachments = descriptor
      .color_attachments
      .into_iter()
      .map(|attach| CanvasRenderPassColorAttachment {
        view: attach.view.texture_view,
        resolve_target: attach
          .resolve_target
          .map(|v| v.texture_view)
          .unwrap_or(std::ptr::null()),
        channel: CanvasPassChannelColor {
          load_op: attach.load_op.into(),
          store_op: attach.store_op.into(),
          clear_value: attach
            .clear_value
            .map(|c| match c {
              Either::A(array) => CanvasColor {
                r: *array.get(0).unwrap_or(&0.0),
                g: *array.get(1).unwrap_or(&0.0),
                b: *array.get(2).unwrap_or(&0.0),
                a: *array.get(3).unwrap_or(&0.0),
              },
              Either::B(dict) => CanvasColor {
                r: dict.r,
                g: dict.g,
                b: dict.b,
                a: dict.a,
              },
            })
            .unwrap_or(CanvasColor {
              r: 0.0,
              g: 0.0,
              b: 0.0,
              a: 0.0,
            }),
          read_only: false,
        },
      })
      .collect::<Vec<_>>();

    let mut depth_stencil_attachment: *const CanvasRenderPassDepthStencilAttachment =
      std::ptr::null();
    let mut occlusion_query_set: *const CanvasGPUQuerySet = std::ptr::null();
    let mut query_set: *const CanvasGPUQuerySet = std::ptr::null();
    let mut beginning_of_pass_write_index: i32 = -1;
    let mut end_of_pass_write_index: i32 = -1;

    let pass = unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_begin_render_pass(
        Arc::as_ptr(&self.encoder),
        label_ptr,
        color_attachments.as_ptr(),
        color_attachments.len(),
        depth_stencil_attachment,
        occlusion_query_set,
        query_set,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
      )
    };

    GPURenderPassEncoder { encoder: pass }
  }

  #[napi]
  pub fn clear_buffer(&self, buffer: &g_p_u_buffer, offset: Option<i64>, size: Option<i64>) {
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_clear_buffer(
        Arc::as_ptr(&self.encoder),
        Arc::as_ptr(&buffer.buffer),
        offset.unwrap_or(-1),
        size.unwrap_or(-1),
      )
    }
  }

  #[napi]
  pub fn copy_buffer_to_buffer(
    &self,
    source: &g_p_u_buffer,
    source_offset: i64,
    destination: &g_p_u_buffer,
    destination_offset: i64,
    size: i64,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_copy_buffer_to_buffer(
       Arc::as_ptr(&self.encoder), Arc::as_ptr(&source.buffer), source_offset, Arc::as_ptr(&destination.buffer), destination_offset, size.try_into().unwrap_or(0)
     )
    }
  }

  #[napi]
  pub fn copy_buffer_to_texture(
    &self,
    source: GPUImageCopyBuffer,
    destination: GPUImageCopyTexture,
    copy_size: Either<GPUExtent3DDict, Vec<u32>>,
  ) {
    let offset: Option<u64> = source.offset.map(|v| v.try_into().ok()).flatten();
    let src = canvas_c::webgpu::gpu_command_encoder::CanvasImageCopyBuffer {
      buffer: Arc::as_ptr(&source.buffer.buffer),
      offset: offset.unwrap_or(0),
      bytes_per_row: source.bytes_per_row,
      rows_per_image: source.rows_per_image.unwrap_or(-1),
    };

    let dst = canvas_c::webgpu::gpu_command_encoder::CanvasImageCopyTexture {
      texture: Arc::as_ptr(&destination.texture.texture),
      mip_level: destination.mip_level.unwrap_or(0),
      origin: match destination.origin {
        None => canvas_c::webgpu::structs::CanvasOrigin3d::default(),
        Some(value) => match value {
          Either::A(dict) => dict.into(),
          Either::B(array) => canvas_c::webgpu::structs::CanvasOrigin3d {
            x: *array.get(0).unwrap_or(&0),
            y: *array.get(1).unwrap_or(&0),
            z: *array.get(2).unwrap_or(&0),
          },
        },
      },
      aspect: destination
        .aspect
        .unwrap_or(crate::gpu::enums::GPUTextureAspect::all)
        .into(),
    };

    let size = match copy_size {
      Either::A(dict) => canvas_c::webgpu::structs::CanvasExtent3d {
        width: dict.width,
        height: dict.height.unwrap_or(1),
        depth_or_array_layers: dict.depth_or_array_layers.unwrap_or(1),
      },
      Either::B(array) => canvas_c::webgpu::structs::CanvasExtent3d {
        width: *array.get(0).unwrap_or(&0u32),
        height: *array.get(1).unwrap_or(&1u32),
        depth_or_array_layers: *array.get(1).unwrap_or(&1u32),
      },
    };
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_copy_buffer_to_texture(
        Arc::as_ptr(&self.encoder),&src, &dst, &size
      );
    }
  }

  #[napi]
  pub fn copy_texture_to_buffer(
    &self,
    source: GPUImageCopyTexture,
    destination: GPUImageCopyBuffer,
    copy_size: Either<GPUExtent3DDict, Vec<u32>>,
  ) {
    let src = canvas_c::webgpu::gpu_command_encoder::CanvasImageCopyTexture {
      texture: Arc::as_ptr(&source.texture.texture),
      mip_level: source.mip_level.unwrap_or(0),
      origin: match source.origin {
        None => canvas_c::webgpu::structs::CanvasOrigin3d::default(),
        Some(value) => match value {
          Either::A(dict) => dict.into(),
          Either::B(array) => canvas_c::webgpu::structs::CanvasOrigin3d {
            x: *array.get(0).unwrap_or(&0),
            y: *array.get(1).unwrap_or(&0),
            z: *array.get(2).unwrap_or(&0),
          },
        },
      },
      aspect: source
        .aspect
        .unwrap_or(crate::gpu::enums::GPUTextureAspect::all)
        .into(),
    };

    let offset: Option<u64> = destination.offset.map(|v| v.try_into().ok()).flatten();

    let dst = canvas_c::webgpu::gpu_command_encoder::CanvasImageCopyBuffer {
      buffer: Arc::as_ptr(&destination.buffer.buffer),
      offset: offset.unwrap_or(0),
      bytes_per_row: destination.bytes_per_row,
      rows_per_image: destination.rows_per_image.unwrap_or(-1),
    };

    let size = match copy_size {
      Either::A(dict) => canvas_c::webgpu::structs::CanvasExtent3d {
        width: dict.width,
        height: dict.height.unwrap_or(1),
        depth_or_array_layers: dict.depth_or_array_layers.unwrap_or(1),
      },
      Either::B(array) => canvas_c::webgpu::structs::CanvasExtent3d {
        width: *array.get(0).unwrap_or(&0),
        height: *array.get(1).unwrap_or(&1u32),
        depth_or_array_layers: *array.get(1).unwrap_or(&1u32),
      },
    };

    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_copy_texture_to_buffer(
        Arc::as_ptr(&self.encoder), &src, &dst, &size
      )
    }
  }

  #[napi]
  pub fn copy_texture_to_texture(
    &self,
    source: GPUImageCopyTexture,
    destination: GPUImageCopyTexture,
    copy_size: Either<GPUExtent3DDict, Vec<u32>>,
  ) {
    let src = canvas_c::webgpu::gpu_command_encoder::CanvasImageCopyTexture {
      texture: Arc::as_ptr(&source.texture.texture),
      mip_level: source.mip_level.unwrap_or(0),
      origin: match source.origin {
        None => canvas_c::webgpu::structs::CanvasOrigin3d::default(),
        Some(value) => match value {
          Either::A(dict) => dict.into(),
          Either::B(array) => canvas_c::webgpu::structs::CanvasOrigin3d {
            x: *array.get(0).unwrap_or(&0),
            y: *array.get(1).unwrap_or(&0),
            z: *array.get(2).unwrap_or(&0),
          },
        },
      },
      aspect: source
        .aspect
        .unwrap_or(crate::gpu::enums::GPUTextureAspect::all)
        .into(),
    };
    let dst = canvas_c::webgpu::gpu_command_encoder::CanvasImageCopyTexture {
      texture: Arc::as_ptr(&destination.texture.texture),
      mip_level: destination.mip_level.unwrap_or(0),
      origin: match destination.origin {
        None => canvas_c::webgpu::structs::CanvasOrigin3d::default(),
        Some(value) => match value {
          Either::A(dict) => dict.into(),
          Either::B(array) => canvas_c::webgpu::structs::CanvasOrigin3d {
            x: *array.get(0).unwrap_or(&0),
            y: *array.get(1).unwrap_or(&0),
            z: *array.get(2).unwrap_or(&0),
          },
        },
      },
      aspect: destination
        .aspect
        .unwrap_or(crate::gpu::enums::GPUTextureAspect::all)
        .into(),
    };

    let size = match copy_size {
      Either::A(dict) => canvas_c::webgpu::structs::CanvasExtent3d {
        width: dict.width,
        height: dict.height.unwrap_or(1),
        depth_or_array_layers: dict.depth_or_array_layers.unwrap_or(1),
      },
      Either::B(array) => canvas_c::webgpu::structs::CanvasExtent3d {
        width: *array.get(0).unwrap_or(&0u32),
        height: *array.get(1).unwrap_or(&1u32),
        depth_or_array_layers: *array.get(1).unwrap_or(&1u32),
      },
    };

    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_copy_texture_to_texture(
        Arc::as_ptr(&self.encoder),
        &src, &dst, &size
      )
    }
  }

  #[napi]
  pub fn finish(
    &self,
    descriptor: Option<GPUCommandEncoderFinishDescriptor>,
  ) -> g_p_u_command_buffer {
    let mut label_ptr = std::ptr::null();
    let label = descriptor.map(|d| CString::new(d.label.unwrap()).unwrap());
    if let Some(label) = &label {
      label_ptr = label.as_ptr();
    }
    let buffer = unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_finish(
        Arc::as_ptr(&self.encoder),
        label_ptr,
      )
    };
    g_p_u_command_buffer {
      buffer: unsafe { Arc::from_raw(buffer) },
    }
  }

  #[napi]
  pub fn insert_debug_marker(&self, marker_label: String) {
    if let Ok(label) = CString::new(marker_label) {
      unsafe {
        canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_insert_debug_marker(
         Arc::as_ptr(&self.encoder), label.as_ptr()
       )
      }
    }
  }

  #[napi]
  pub fn pop_debug_group(&self) {
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_pop_debug_group(
        Arc::as_ptr(&self.encoder),
      )
    }
  }

  #[napi]
  pub fn push_debug_group(&self, group_label: String) {
    if let Ok(label) = CString::new(group_label) {
      unsafe {
        canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_push_debug_group(
          Arc::as_ptr(&self.encoder),
          label.as_ptr(),
        )
      }
    }
  }

  #[napi]
  pub fn resolve_query_set(
    &self,
    query_set: &GPUQuerySet,
    first_query: u32,
    query_count: u32,
    destination: &g_p_u_buffer,
    destination_offset: i64,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_resolve_query_set(
        Arc::as_ptr(&self.encoder),
        Arc::as_ptr(&query_set.query),
        first_query,
        query_count,
        Arc::as_ptr(&destination.buffer),
        destination_offset.try_into().unwrap_or(0),
      )
    }
  }

  #[napi]
  pub fn write_timestamp(&self, query_set: &GPUQuerySet, query_index: u32) {
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_write_timestamp(
        Arc::as_ptr(&self.encoder),
        Arc::as_ptr(&query_set.query),
        query_index,
      )
    }
  }
}
