use std::ffi::CString;
use std::sync::Arc;

use canvas_c::webgpu::gpu_render_bundle_encoder::CanvasGPURenderBundleEncoder;
use napi::bindgen_prelude::Unknown;
use napi::Result;
use napi_derive::napi;

use crate::gpu::bind_group::g_p_u_bind_group;
use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::parse::{
  c_str, downcast, dynamic_offsets, index_format, label, range_arg, take_string,
};
use crate::gpu::render_bundle::g_p_u_render_bundle;
use crate::gpu::render_pipeline::g_p_u_render_pipeline;

#[napi(js_name = "GPURenderBundleEncoder")]
pub struct g_p_u_render_bundle_encoder {
  pub(crate) encoder: Arc<CanvasGPURenderBundleEncoder>,
}

#[napi]
impl g_p_u_render_bundle_encoder {
  fn ptr(&self) -> *const CanvasGPURenderBundleEncoder {
    Arc::as_ptr(&self.encoder)
  }

  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(
        canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_get_label(
          self.ptr(),
        ),
      )
    }
    .unwrap_or_default()
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
        self.ptr(),
        vertex_count,
        instance_count.unwrap_or(1),
        first_vertex.unwrap_or(0),
        first_instance.unwrap_or(0),
      )
    }
  }

  #[napi]
  pub fn draw_indexed(
    &self,
    index_count: u32,
    instance_count: Option<u32>,
    first_index: Option<u32>,
    base_vertex: Option<i32>,
    first_instance: Option<u32>,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_draw_indexed(
        self.ptr(),
        index_count,
        instance_count.unwrap_or(1),
        first_index.unwrap_or(0),
        base_vertex.unwrap_or(0),
        first_instance.unwrap_or(0),
      )
    }
  }

  #[napi]
  pub fn draw_indexed_indirect(&self, indirect_buffer: &g_p_u_buffer, indirect_offset: f64) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_draw_indexed_indirect(
        self.ptr(),
        Arc::as_ptr(&indirect_buffer.buffer),
        indirect_offset.max(0.) as u64,
      )
    }
  }

  #[napi]
  pub fn draw_indirect(&self, indirect_buffer: &g_p_u_buffer, indirect_offset: f64) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_draw_indirect(
        self.ptr(),
        Arc::as_ptr(&indirect_buffer.buffer),
        indirect_offset.max(0.) as u64,
      )
    }
  }

  #[napi(ts_args_type = "descriptor?: { label?: string }")]
  pub fn finish(&self, descriptor: Option<Unknown>) -> Option<g_p_u_render_bundle> {
    let label = descriptor.as_ref().and_then(label);
    let bundle = unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_finish(
        self.ptr(),
        c_str(&label),
      )
    };
    (!bundle.is_null()).then(|| g_p_u_render_bundle {
      render_bundle: unsafe { Arc::from_raw(bundle) },
    })
  }

  #[napi]
  pub fn insert_debug_marker(&self, marker_label: String) {
    if let Ok(label) = CString::new(marker_label) {
      unsafe {
        canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_insert_debug_marker(
          self.ptr(),
          label.as_ptr(),
        )
      }
    }
  }

  #[napi]
  pub fn pop_debug_group(&self) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_pop_debug_group(
        self.ptr(),
      )
    }
  }

  #[napi]
  pub fn push_debug_group(&self, group_label: String) {
    if let Ok(label) = CString::new(group_label) {
      unsafe {
        canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_push_debug_group(
          self.ptr(),
          label.as_ptr(),
        )
      }
    }
  }

  /// `setBindGroup(index, bindGroup, dynamicOffsets?, start?, length?)`.
  #[napi(
    ts_args_type = "index: number, bindGroup: GPUBindGroup | null, dynamicOffsetsData?: Uint32Array | number[], dynamicOffsetsDataStart?: number, dynamicOffsetsDataLength?: number"
  )]
  pub fn set_bind_group(
    &self,
    index: u32,
    bind_group: Option<Unknown>,
    dynamic_offsets_data: Option<Unknown>,
    dynamic_offsets_data_start: Option<f64>,
    dynamic_offsets_data_length: Option<f64>,
  ) -> Result<()> {
    let group = bind_group
      .as_ref()
      .and_then(downcast::<g_p_u_bind_group>)
      .map_or(std::ptr::null(), |group| Arc::as_ptr(&group.group));
    let offsets = dynamic_offsets(
      dynamic_offsets_data.as_ref(),
      dynamic_offsets_data_start,
      dynamic_offsets_data_length,
    )?;
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_set_bind_group(
        self.ptr(),
        index,
        group,
        offsets.as_ptr(),
        offsets.len(),
        0,
        offsets.len(),
      )
    }
    Ok(())
  }

  /// `setIndexBuffer(buffer, format, offset?, size?)`: format 0 / 1 or `"uint16"` / `"uint32"`.
  #[napi(
    ts_args_type = "buffer: GPUBuffer, indexFormat: number | 'uint16' | 'uint32', offset?: number, size?: number"
  )]
  pub fn set_index_buffer(
    &self,
    buffer: &g_p_u_buffer,
    index_format_value: Unknown,
    offset: Option<f64>,
    size: Option<f64>,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_set_index_buffer(
        self.ptr(),
        Arc::as_ptr(&buffer.buffer),
        index_format(&index_format_value),
        range_arg(offset),
        range_arg(size),
      )
    }
  }

  #[napi]
  pub fn set_pipeline(&self, pipeline: &g_p_u_render_pipeline) {
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_set_pipeline(
        self.ptr(),
        Arc::as_ptr(&pipeline.pipeline),
      )
    }
  }

  /// `setVertexBuffer(slot, buffer, offset?, size?)`; a null buffer is ignored.
  #[napi(ts_args_type = "slot: number, buffer: GPUBuffer | null, offset?: number, size?: number")]
  pub fn set_vertex_buffer(
    &self,
    slot: u32,
    buffer: Option<Unknown>,
    offset: Option<f64>,
    size: Option<f64>,
  ) {
    let Some(buffer) = buffer.as_ref().and_then(downcast::<g_p_u_buffer>) else {
      return;
    };
    unsafe {
      canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_set_vertex_buffer(
        self.ptr(),
        slot,
        Arc::as_ptr(&buffer.buffer),
        range_arg(offset),
        range_arg(size),
      )
    }
  }
}
