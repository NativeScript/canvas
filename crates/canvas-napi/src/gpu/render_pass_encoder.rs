use crate::gpu::bind_group::g_p_u_bind_group;
use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::enums::GPUIndexFormat;
use crate::gpu::objects::GPUColorDict;
use crate::gpu::render_bundle::g_p_u_render_bundle;
use crate::gpu::render_pipeline::g_p_u_render_pipeline;
use canvas_c::webgpu::structs::CanvasColor;
use napi::bindgen_prelude::Uint32Array;
use napi::Either;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi(js_name = "GPURenderPassEncoder")]
pub struct g_p_u_render_pass_encoder {
  pub(crate) encoder: Arc<canvas_c::webgpu::gpu_render_pass_encoder::CanvasGPURenderPassEncoder>,
}

#[napi]
impl g_p_u_render_pass_encoder {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_get_label(
        Arc::as_ptr(&self.encoder),
      )
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }

  #[napi]
  pub fn begin_occlusion_query(&self, query_index: u32) {
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_begin_occlusion_query(
                Arc::as_ptr(&self.encoder), query_index,
            )
    }
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
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_draw(
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
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_draw_indexed(
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
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_draw_indexed_indirect(
                Arc::as_ptr(&self.encoder), Arc::as_ptr(&indirect_buffer.buffer), indirect_offset as u64,
            )
    }
  }

  #[napi]
  pub fn draw_indirect(&self, indirect_buffer: &g_p_u_buffer, indirect_offset: i64) {
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_draw_indirect(
                Arc::as_ptr(&self.encoder), Arc::as_ptr(&indirect_buffer.buffer), indirect_offset as u64,
            )
    }
  }

  // #[napi]
  // pub fn multi_draw_indexed(&self, indexCount: u32, instanceCount: Option<u32> = 1, firstVertex: Option<u32> = 0, firstInstance: Option<u32> = 0, Option<u32> = 0) {
  //     this[native_].multiDrawIndexed(indexCount, instanceCount?? 1, firstVertex?? 0, firstInstance?? 0, count?? 0);
  //     unsafe {
  //         canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_multi_draw_indexed(
  //             Arc::as_ptr(&self.encoder), indexCount,  instanceCount.unwrap_or(1), firstVertex.unwrap_or_default(), firstInstance.unwrap_or_default()
  //         )
  //     }
  // }

  #[napi]
  pub fn multi_draw_indexed_indirect(
    &self,
    indirect_buffer: &g_p_u_buffer,
    indirect_offset: i64,
    count: Option<u32>,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_multi_draw_indexed_indirect(
                Arc::as_ptr(&self.encoder), Arc::as_ptr(&indirect_buffer.buffer), indirect_offset as u64, count.unwrap_or(0),
            )
    }
  }

  #[napi]
  pub fn multi_draw_indirect(
    &self,
    indirect_buffer: &g_p_u_buffer,
    indirect_offset: i64,
    count: Option<u32>,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_multi_draw_indirect(
                Arc::as_ptr(&self.encoder), Arc::as_ptr(&indirect_buffer.buffer), indirect_offset as u64, count.unwrap_or(0),
            )
    }
  }

  #[napi]
  pub fn end(&self) {
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_end(
        Arc::as_ptr(&self.encoder),
      )
    }
  }

  #[napi]
  pub fn end_occlusion_query(&self) {
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_end_occlusion_query(
                Arc::as_ptr(&self.encoder),
            )
    }
  }

  #[napi]
  pub fn execute_bundles(&self, bundles: Vec<&g_p_u_render_bundle>) {
    let bundles = bundles
      .into_iter()
      .map(|bundle| Arc::as_ptr(&bundle.render_bundle))
      .collect::<Vec<_>>();
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_execute_bundles(
                Arc::as_ptr(&self.encoder), bundles.as_ptr(), bundles.len(),
            )
    }
  }

  #[napi]
  pub fn insert_debug_marker(&self, marker_label: String) {
    if let Ok(label) = CString::new(marker_label) {
      unsafe {
        canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_insert_debug_marker(
                    Arc::as_ptr(&self.encoder), label.as_ptr(),
                )
      }
    }
  }

  #[napi]
  pub fn pop_debug_group(&self) {
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_pop_debug_group(
                Arc::as_ptr(&self.encoder)
            )
    }
  }

  #[napi]
  pub fn push_debug_group(&self, group_label: String) {
    if let Ok(label) = CString::new(group_label) {
      unsafe {
        canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_push_debug_group(
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
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_set_bind_group(
                Arc::as_ptr(&self.encoder), index, Arc::as_ptr(&bind_group.group), dynamic_offsets, dynamic_offsets_size, dynamic_offsets_start, dynamic_offsets_length,
            )
    }
  }

  #[napi]
  pub fn set_blend_constant(&self, color: Either<GPUColorDict, Vec<f64>>) {
    match color {
      Either::A(dict) => {
        let color = CanvasColor {
          r: dict.r,
          g: dict.g,
          b: dict.b,
          a: dict.a,
        };
        unsafe {
          canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_set_blend_constant(
                        Arc::as_ptr(&self.encoder), &color
                    )
        }
      }
      Either::B(array) => {
        let color = CanvasColor {
          r: array[0],
          g: array[1],
          b: array[2],
          a: array[3],
        };
        unsafe {
          canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_set_blend_constant(
                       Arc::as_ptr(&self.encoder),  &color
                   )
        }
      }
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
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_set_index_buffer(
                Arc::as_ptr(&self.encoder), Arc::as_ptr(&buffer.buffer), index_format.into(), offset.unwrap_or(0), size.unwrap_or(
                    buffer.size() - offset.unwrap_or(0)
                )
            )
    }
  }

  #[napi]
  pub fn set_pipeline(&self, render_pipeline: &g_p_u_render_pipeline) {
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_set_pipeline(
                Arc::as_ptr(&self.encoder), Arc::as_ptr(&render_pipeline.pipeline),
            )
    }
  }

  #[napi]
  pub fn set_scissor_rect(&self, x: u32, y: u32, width: u32, height: u32) {
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_set_scissor_rect(
                Arc::as_ptr(&self.encoder), x,y,width, height
            )
    }
  }

  #[napi]
  pub fn set_stencil_reference(&self, reference: u32) {
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_set_stencil_reference(
                Arc::as_ptr(&self.encoder), reference
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
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_set_vertex_buffer(
                Arc::as_ptr(&self.encoder), slot, Arc::as_ptr(&buffer.buffer),  offset.unwrap_or(0), size.unwrap_or(
                    buffer.size() - offset.unwrap_or(0)
                )
            )
    }
  }

  #[napi]
  pub fn set_viewport(
    &self,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    min_depth: f64,
    max_depth: f64,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_set_viewport(
                Arc::as_ptr(&self.encoder), x as f32, y as f32, width as f32, height as f32, min_depth as f32, max_depth as f32
            )
    }
  }
}
