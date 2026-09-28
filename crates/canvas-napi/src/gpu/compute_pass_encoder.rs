use std::ffi::CString;
use std::sync::Arc;

use canvas_c::webgpu::gpu_compute_pass_encoder::CanvasGPUComputePassEncoder;
use napi::bindgen_prelude::Unknown;
use napi::Result;
use napi_derive::napi;

use crate::gpu::bind_group::g_p_u_bind_group;
use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::compute_pipeline::g_p_u_compute_pipeline;
use crate::gpu::handle::Handle;
use crate::gpu::parse::{downcast, dynamic_offsets, take_string};

#[napi(js_name = "GPUComputePassEncoder")]
pub struct g_p_u_compute_pass_encoder {
  pub(crate) encoder: Handle<CanvasGPUComputePassEncoder>,
}

#[napi]
impl g_p_u_compute_pass_encoder {
  fn ptr(&self) -> *const CanvasGPUComputePassEncoder {
    self.encoder.ptr()
  }

  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(
        canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_get_label(
          self.ptr(),
        ),
      )
    }
    .unwrap_or_default()
  }

  #[napi]
  pub fn dispatch_workgroups(
    &self,
    workgroup_count_x: u32,
    workgroup_count_y: Option<u32>,
    workgroup_count_z: Option<u32>,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups(
        self.ptr(),
        workgroup_count_x,
        workgroup_count_y.unwrap_or(1),
        workgroup_count_z.unwrap_or(1),
      )
    }
  }

  #[napi]
  pub fn dispatch_workgroups_indirect(&self, indirect_buffer: &g_p_u_buffer, indirect_offset: f64) {
    unsafe {
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups_indirect(
        self.ptr(),
        Arc::as_ptr(&indirect_buffer.buffer),
        indirect_offset.max(0.) as usize,
      )
    }
  }

  #[napi]
  pub fn end(&self) {
    unsafe {
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_end(
        self.ptr(),
      )
    }
  }

  /// Releases the pass now (packages/canvas does right after `end`).
  #[napi]
  pub fn destroy(&self) {
    self.encoder.release();
  }

  #[napi]
  pub fn insert_debug_marker(&self, marker_label: String) {
    if let Ok(label) = CString::new(marker_label) {
      unsafe {
        canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_insert_debug_marker(
          self.ptr(),
          label.as_ptr(),
        )
      }
    }
  }

  #[napi]
  pub fn pop_debug_group(&self) {
    unsafe {
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_pop_debug_group(
        self.ptr(),
      )
    }
  }

  #[napi]
  pub fn push_debug_group(&self, group_label: String) {
    if let Ok(label) = CString::new(group_label) {
      unsafe {
        canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_push_debug_group(
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
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_set_bind_group(
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

  #[napi]
  pub fn set_pipeline(&self, pipeline: &g_p_u_compute_pipeline) {
    unsafe {
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_set_pipeline(
        self.ptr(),
        Arc::as_ptr(&pipeline.pipeline),
      )
    }
  }
}
