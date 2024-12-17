use crate::gpu::bind_group::g_p_u_bind_group;
use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::enums::GPUIndexFormat;
use crate::gpu::objects::GPURenderBundleDescriptor;
use crate::gpu::render_bundle::g_p_u_render_bundle;
use crate::gpu::render_pipeline::g_p_u_render_pipeline;
use napi::bindgen_prelude::Uint32Array;
use napi::Either;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi(js_name = "GPURenderBundleEncoder")]
pub struct g_p_u_render_bundle_encoder {
  pub(crate) encoder:
    Arc<canvas_c::webgpu::gpu_render_bundle_encoder::CanvasGPURenderBundleEncoder>,
}

#[napi]
impl g_p_u_render_bundle_encoder {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_get_label(Arc::as_ptr(&self.encoder))
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }

  #[napi]
  pub fn draw(
    &self,
    vertex_count: u32,
    instance_count: Option<u32>,
    first_vertex: Option<u32>,
    first_instance: Option<u32>,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_draw(
        Arc::as_ptr(&self.encoder),
        vertex_count,
        instance_count.unwrap_or(1),
        first_vertex.unwrap_or_default(),
        first_instance.unwrap_or_default(),
      )
    }
  }

  #[napi]
  pub fn draw_indexed(
    &self,
    index_count: u32,
    instance_count: Option<u32>,
    first_vertex: Option<u32>,
    base_vertex: Option<i32>,
    first_instance: Option<u32>,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_draw_indexed(
        Arc::as_ptr(&self.encoder),
        index_count,
        instance_count.unwrap_or(1),
        first_vertex.unwrap_or_default(),
        base_vertex.unwrap_or_default(),
        first_instance.unwrap_or_default(),
      )
    }
  }

  #[napi]
  pub fn draw_indexed_indirect(&self, indirect_buffer: &g_p_u_buffer, indirect_offset: i64) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_draw_indexed_indirect(
        Arc::as_ptr(&self.encoder), Arc::as_ptr(&indirect_buffer.buffer), indirect_offset as u64,
      )
    }
  }

  #[napi]
  pub fn draw_indirect(&self, indirect_buffer: &g_p_u_buffer, indirect_offset: i64) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_draw_indirect(
        Arc::as_ptr(&self.encoder), Arc::as_ptr(&indirect_buffer.buffer), indirect_offset as u64,
      )
    }
  }

  #[napi]
  pub fn finish(&self, descriptor: Option<GPURenderBundleDescriptor>) -> g_p_u_render_bundle {
    let label =
      descriptor.and_then(|descriptor| descriptor.label.map(|label| CString::new(label).unwrap()));
    let label_ptr = match label.as_ref() {
      None => std::ptr::null(),
      Some(l) => l.as_ptr(),
    };

    let bundle = unsafe {
      Arc::from_raw( canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_finish(
       Arc::as_ptr(&self.encoder),label_ptr
     ))
    };

    g_p_u_render_bundle {
      render_bundle: bundle,
    }
  }

  #[napi]
  pub fn insert_debug_marker(&self, marker_label: String) {
    if let Ok(label) = CString::new(marker_label) {
      unsafe {
        canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_insert_debug_marker(
          Arc::as_ptr(&self.encoder), label.as_ptr(),
        )
      }
    }
  }

  #[napi]
  pub fn pop_debug_group(&self) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_pop_debug_group(
        Arc::as_ptr(&self.encoder)
      )
    }
  }

  #[napi]
  pub fn push_debug_group(&self, group_label: String) {
    if let Ok(label) = CString::new(group_label) {
      unsafe {
        canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_push_debug_group(
          Arc::as_ptr(&self.encoder), label.as_ptr(),
        )
      }
    }
  }

  #[napi]
  pub fn set_bind_group(
    &self,
    index: u32,
    bind_group: &g_p_u_bind_group,
    dynamic_offsets_data: Option<Either<Vec<u32>, Uint32Array>>,
    dynamic_offsets_data_start: Option<i64>,
    dynamic_offsets_data_length: Option<i64>,
  ) {
    let mut dynamic_offsets = std::ptr::null();
    let mut dynamic_offsets_size: usize = 0;
    let mut dynamic_offsets_start: usize = 0;
    let mut dynamic_offsets_length: usize = 0;

    if let Some(dynamicOffsetsData) = dynamic_offsets_data {
      match dynamicOffsetsData {
        Either::A(array) => {
          dynamic_offsets = array.as_ptr();
          dynamic_offsets_size = array.len();
        }
        Either::B(buffer) => {
          dynamic_offsets = buffer.as_ptr();
          dynamic_offsets_size = buffer.len();

          dynamic_offsets_start = dynamic_offsets_data_start.unwrap_or(0) as usize;
          dynamic_offsets_length = dynamic_offsets_data_length.unwrap_or(0) as usize;
        }
      }
    }

    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_set_bind_group(
        Arc::as_ptr(&self.encoder), index, Arc::as_ptr(&bind_group.group), dynamic_offsets, dynamic_offsets_size, dynamic_offsets_start, dynamic_offsets_length,
      )
    }
  }

  #[napi]
  pub fn set_index_buffer(
    &self,
    buffer: &g_p_u_buffer,
    index_format: GPUIndexFormat,
    offset: Option<i64>,
    size: Option<i64>,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_set_index_buffer(
        Arc::as_ptr(&self.encoder), Arc::as_ptr(&buffer.buffer), index_format.into(), offset.unwrap_or(0), size.unwrap_or(
          buffer.size() - offset.unwrap_or(0)
        )
      )
    }
  }

  #[napi]
  pub fn set_pipeline(&self, render_pipeline: &g_p_u_render_pipeline) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_set_pipeline(
        Arc::as_ptr(&self.encoder), Arc::as_ptr(&render_pipeline.pipeline),
      )
    }
  }

  #[napi]
  pub fn set_vertex_buffer(
    &self,
    slot: u32,
    buffer: &g_p_u_buffer,
    offset: Option<i64>,
    size: Option<i64>,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_set_vertex_buffer(
        Arc::as_ptr(&self.encoder), slot, Arc::as_ptr(&buffer.buffer), offset.unwrap_or(0), size.unwrap_or(
          buffer.size() - offset.unwrap_or(0)
        )
      )
    }
  }
}
