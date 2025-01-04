use crate::gpu::bind_group::g_p_u_bind_group;
use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::compute_pipeline::g_p_u_compute_pipeline;
use napi::bindgen_prelude::Uint32Array;
use napi::Either;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi(js_name = "GPUComputePassEncoder")]
pub struct g_p_u_compute_pass_encoder {
  pub(crate) encoder: Arc<canvas_c::webgpu::gpu_compute_pass_encoder::CanvasGPUComputePassEncoder>,
}

#[napi]
impl g_p_u_compute_pass_encoder {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_get_label(Arc::as_ptr(&self.encoder))
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
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
       Arc::as_ptr(&self.encoder), workgroup_count_x, workgroup_count_y.unwrap_or(1), workgroup_count_z.unwrap_or(1)
     )
    }
  }

  #[napi]
  pub fn dispatch_workgroups_indirect(&self, indirect_buffer: &g_p_u_buffer, indirect_offset: i64) {
    if let Some(indirect_offset) = indirect_offset.try_into().ok() {
      unsafe {
        canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups_indirect(
          Arc::as_ptr(&self.encoder), Arc::as_ptr(&indirect_buffer.buffer), indirect_offset
        )
      }
    }
  }

  #[napi]
  pub fn end(&self) {
    unsafe {
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_end(
        Arc::as_ptr(&self.encoder),
      )
    }
  }

  #[napi]
  pub fn insert_debug_marker(&self, marker_label: String) {
    if let Ok(label) = CString::new(marker_label) {
      unsafe {
        canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_insert_debug_marker(
          Arc::as_ptr(&self.encoder), label.as_ptr()
        )
      }
    }
  }

  #[napi]
  pub fn pop_debug_group(&self) {
    unsafe {
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_pop_debug_group(
        Arc::as_ptr(&self.encoder)
      )
    }
  }

  #[napi]
  pub fn push_debug_group(&self, group_label: String) {
    if let Ok(label) = CString::new(group_label) {
      unsafe {
        canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_push_debug_group(
          Arc::as_ptr(&self.encoder), label.as_ptr()
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
    let mut dynamic_offsets_size = 0_usize;

    if let Some(dynamic_offsets_data) = dynamic_offsets_data.as_ref() {
      match dynamic_offsets_data {
        Either::A(array) => {
          dynamic_offsets = array.as_ptr();
          dynamic_offsets_size = array.len();
        }
        Either::B(buffer) => {
          dynamic_offsets = buffer.as_ptr();
          dynamic_offsets_size = buffer.len();
        }
      }
    }

    let dynamic_offsets_data_start: Option<u64> = dynamic_offsets_data_start.and_then(|v| v.try_into().ok());
    let dynamic_offsets_data_length: Option<u64> = dynamic_offsets_data_length.and_then(|v| v.try_into().ok());
    unsafe {
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_set_bind_group(
        Arc::as_ptr(&self.encoder),index, Arc::as_ptr(&bind_group.group), dynamic_offsets,dynamic_offsets_size, dynamic_offsets_data_start.unwrap_or(0) as usize,dynamic_offsets_data_length.unwrap_or(0) as usize
      )
    }
  }

  #[napi]
  pub fn set_pipeline(&self, render_pipeline: &g_p_u_compute_pipeline) {
    unsafe {
      canvas_c::webgpu::gpu_compute_pass_encoder::canvas_native_webgpu_compute_pass_encoder_set_pipeline(
        Arc::as_ptr(&self.encoder), Arc::as_ptr(&render_pipeline.pipeline)
      )
    }
  }
}
