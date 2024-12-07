use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::enums::{
  GPUBlendFactor, GPUBlendOperation, GPUCompareFunction, GPUCullMode, GPUFrontFace, GPUIndexFormat,
  GPULoadOp, GPUPipelineLayoutAuto, GPUPrimitiveTopology, GPUStencilOperation, GPUStoreOp,
  GPUTextureAspect, GPUTextureFormat, GPUVertexFormat, GPUVertexStepMode,
};
use crate::gpu::pipeline_layout::g_p_u_pipeline_layout;
use crate::gpu::query_set::GPUQuerySet;
use crate::gpu::shader_module::g_p_u_shader_module;
use crate::gpu::texture::g_p_u_texture;
use crate::gpu::texture_view::GPUTextureView;
use canvas_c::webgpu::enums::{
  CanvasCullMode, CanvasFrontFace, CanvasGPUTextureFormat, CanvasOptionalIndexFormat,
  CanvasOptionalPrimitiveTopology,
};
use canvas_c::webgpu::gpu_device::{CanvasPrimitiveState, CanvasVertexBufferLayout};
use canvas_c::webgpu::structs::{
  CanvasBlendFactor, CanvasBlendOperation, CanvasBlendState, CanvasColor, CanvasColorTargetState,
  CanvasExtent3d, CanvasOrigin3d,
};
use napi::bindgen_prelude::ClassInstance;
use napi::Either;
use napi_derive::napi;
use std::collections::HashMap;

#[napi(object)]
pub struct GPUImageCopyBuffer {
  pub buffer: ClassInstance<g_p_u_buffer>,
  pub bytes_per_row: i32,
  pub offset: Option<i64>,
  pub rows_per_image: Option<i32>,
}

#[napi(object)]
pub struct GPUOrigin3DDict {
  pub x: u32,
  pub y: u32,
  pub z: u32,
}

impl From<CanvasOrigin3d> for GPUOrigin3DDict {
  fn from(value: CanvasOrigin3d) -> Self {
    Self {
      x: value.x,
      y: value.y,
      z: value.z,
    }
  }
}

impl From<GPUOrigin3DDict> for CanvasOrigin3d {
  fn from(value: GPUOrigin3DDict) -> Self {
    Self {
      x: value.x,
      y: value.y,
      z: value.z,
    }
  }
}

#[napi(object)]
pub struct GPUImageCopyTexture {
  pub aspect: Option<GPUTextureAspect>,
  pub mip_level: Option<u32>,
  pub origin: Option<Either<GPUOrigin3DDict, Vec<u32>>>,
  pub texture: ClassInstance<g_p_u_texture>,
}

#[napi(object)]
pub struct GPUComputePassTimestampWrites {
  pub beginning_of_pass_write_index: i32,
  pub end_of_pass_write_index: i32,
  pub query_qet: ClassInstance<GPUQuerySet>,
}

#[napi(object)]
pub struct GPUComputePassDescriptor {
  pub label: Option<String>,
  pub timestamp_writes: Option<GPUComputePassTimestampWrites>,
}

#[napi(object)]
pub struct GPUColorDict {
  pub a: f64,
  pub b: f64,
  pub g: f64,
  pub r: f64,
}

impl From<CanvasColor> for GPUColorDict {
  fn from(value: CanvasColor) -> Self {
    Self {
      r: value.r,
      g: value.g,
      b: value.b,
      a: value.a,
    }
  }
}

#[napi(object)]
pub struct GPURenderPassColorAttachment {
  pub clear_value: Option<Either<Vec<f64>, GPUColorDict>>,
  pub depth_slice: Option<u32>,
  pub load_op: GPULoadOp,
  pub resolve_target: Option<ClassInstance<GPUTextureView>>,
  pub store_op: GPUStoreOp,
  pub view: ClassInstance<GPUTextureView>,
}

#[napi(object)]
pub struct GPURenderPassDepthStencilAttachment {
  pub depth_clear_value: Option<u32>,
  pub depth_load_op: GPULoadOp,
  pub depth_read_only: Option<bool>,
  pub depth_store_op: GPUStoreOp,
  pub stencil_clear_value: Option<u32>,
  pub stencil_load_op: Option<GPULoadOp>,
  pub stencil_read_only: Option<bool>,
  pub stencil_store_op: Option<GPUStoreOp>,
  pub view: ClassInstance<GPUTextureView>,
}

#[napi(object)]
pub struct GPURenderPassTimestampWrites {
  pub beginning_of_pass_write_index: i32,
  pub end_of_pass_write_index: i32,
  pub query_set: ClassInstance<GPUQuerySet>,
}

#[napi(object)]
pub struct GPURenderPassDescriptor {
  pub color_attachments: Vec<GPURenderPassColorAttachment>,
  pub depth_stencil_attachment: Option<GPURenderPassDepthStencilAttachment>,
  pub label: Option<String>,
  pub max_draw_count: Option<i64>,
  pub occlusion_query_set: Option<ClassInstance<GPUQuerySet>>,
  pub timestamp_writes: Option<GPURenderPassTimestampWrites>,
}

#[napi(object)]
pub struct GPUCommandEncoderDescriptor {
  pub label: Option<String>,
}

#[napi(object)]
pub struct GPUShaderModuleDescriptor {
  pub code: String,
  pub label: Option<String>,
}

#[napi(object)]
pub struct GPUExtent3DDict {
  pub width: u32,
  pub height: Option<u32>,
  pub depth_or_array_layers: Option<u32>,
}

impl From<CanvasExtent3d> for GPUExtent3DDict {
  fn from(value: CanvasExtent3d) -> Self {
    Self {
      width: value.width,
      height: Some(value.height),
      depth_or_array_layers: Some(value.depth_or_array_layers),
    }
  }
}

impl Into<CanvasExtent3d> for GPUExtent3DDict {
  fn into(self) -> CanvasExtent3d {
    CanvasExtent3d {
      width: self.width,
      height: self.height.unwrap_or(1),
      depth_or_array_layers: self.depth_or_array_layers.unwrap_or(1),
    }
  }
}

impl Default for GPUExtent3DDict {
  fn default() -> Self {
    Self {
      width: 0,
      height: Some(1),
      depth_or_array_layers: Some(1),
    }
  }
}

#[napi(object)]
pub struct GPUStencilFaceState {
  pub compare: Option<GPUCompareFunction>,
  pub depth_fail_op: Option<GPUStencilOperation>,
  pub fail_op: Option<GPUStencilOperation>,
  pub pass_op: Option<GPUStencilOperation>,
}

#[napi(object)]
pub struct GPUDepthStencilState {
  pub depth_bias: Option<u32>,
  pub depth_bias_clamp: Option<u32>,
  pub depth_bias_slope_scale: Option<u32>,
  pub depth_compare: Option<GPUCompareFunction>,
  pub depth_write_enabled: Option<bool>,
  pub format: GPUTextureFormat,
  pub stencil_back: Option<GPUStencilFaceState>,
  pub stencil_front: Option<GPUStencilFaceState>,
  pub stencil_read_mask: Option<u32>,
  pub stencil_write_mask: Option<u32>,
}

#[napi(object)]
pub struct GPUBlendComponent {
  pub dst_factor: Option<GPUBlendFactor>,
  pub operation: Option<GPUBlendOperation>,
  pub src_factor: Option<GPUBlendFactor>,
}

impl From<canvas_c::webgpu::structs::CanvasBlendComponent> for GPUBlendComponent {
  fn from(value: canvas_c::webgpu::structs::CanvasBlendComponent) -> Self {
    Self {
      dst_factor: Some(value.dst_factor.into()),
      operation: Some(value.operation.into()),
      src_factor: Some(value.src_factor.into()),
    }
  }
}

impl Into<canvas_c::webgpu::structs::CanvasBlendComponent> for GPUBlendComponent {
  fn into(self) -> canvas_c::webgpu::structs::CanvasBlendComponent {
    canvas_c::webgpu::structs::CanvasBlendComponent {
      dst_factor: match self.dst_factor {
        None => CanvasBlendFactor::Zero,
        Some(value) => value.into(),
      },
      operation: match self.operation {
        None => CanvasBlendOperation::Add,
        Some(value) => value.into(),
      },
      src_factor: match self.src_factor {
        None => CanvasBlendFactor::One,
        Some(value) => value.into(),
      },
    }
  }
}

#[napi(object)]
pub struct GPUBlendState {
  pub alpha: GPUBlendComponent,
  pub color: GPUBlendComponent,
}

impl Into<CanvasBlendState> for GPUBlendState {
  fn into(self) -> CanvasBlendState {
    CanvasBlendState {
      color: self.color.into(),
      alpha: self.alpha.into(),
    }
  }
}

#[napi(object)]
pub struct GPUColorTargetState {
  pub blend: Option<GPUBlendState>,
  pub format: GPUTextureFormat,
  pub write_mask: Option<u32>,
}

impl Into<CanvasColorTargetState> for GPUColorTargetState {
  fn into(self) -> CanvasColorTargetState {
    let format: CanvasGPUTextureFormat = self.format.into();
    let mut state = CanvasColorTargetState::from(format);
    if let Some(blend) = self.blend {
      state.blend = canvas_c::webgpu::structs::CanvasOptionalBlendState::Some(blend.into())
    }

    if let Some(format) = self.write_mask {
      state.write_mask = format;
    }
    state
  }
}

#[napi(object)]
pub struct GPUFragmentState {
  pub constants: Option<HashMap<String, f64>>,
  pub entry_point: Option<String>,
  pub module: ClassInstance<g_p_u_shader_module>,
  pub targets: Vec<GPUColorTargetState>,
}

#[napi(object)]
pub struct GPUMultisampleState {
  pub alpha_to_coverage_enabled: Option<bool>,
  pub count: Option<u32>,
  pub mask: Option<u32>,
}

#[napi(object)]
pub struct GPUPrimitiveState {
  pub cull_mode: Option<GPUCullMode>,
  pub front_face: Option<GPUFrontFace>,
  pub strip_index_format: Option<GPUIndexFormat>,
  pub topology: Option<GPUPrimitiveTopology>,
  pub unclipped_depth: Option<bool>,
}

impl From<CanvasPrimitiveState> for GPUPrimitiveState {
  fn from(value: CanvasPrimitiveState) -> Self {
    Self {
      cull_mode: match value.cull_mode {
        CanvasCullMode::None => None,
        CanvasCullMode::Front => Some(GPUCullMode::front),
        CanvasCullMode::Back => Some(GPUCullMode::back),
      },
      front_face: match value.front_face {
        CanvasFrontFace::Ccw => Some(GPUFrontFace::cw),
        CanvasFrontFace::Cw => Some(GPUFrontFace::cw),
      },
      strip_index_format: match value.strip_index_format {
        CanvasOptionalIndexFormat::None => None,
        CanvasOptionalIndexFormat::Some(value) => Some(value.into()),
      },
      topology: match value.topology {
        CanvasOptionalPrimitiveTopology::None => None,
        CanvasOptionalPrimitiveTopology::Some(value) => Some(value.into()),
      },
      unclipped_depth: Some(value.unclipped_depth),
    }
  }
}

impl From<GPUPrimitiveState> for CanvasPrimitiveState {
  fn from(value: GPUPrimitiveState) -> Self {
    Self {
      topology: match value.topology {
        None => CanvasOptionalPrimitiveTopology::None,
        Some(value) => CanvasOptionalPrimitiveTopology::Some(value.into()),
      },
      strip_index_format: match value.strip_index_format {
        None => CanvasOptionalIndexFormat::None,
        Some(value) => CanvasOptionalIndexFormat::Some(value.into()),
      },
      front_face: match value.front_face {
        None => CanvasFrontFace::Ccw,
        Some(value) => match value {
          GPUFrontFace::ccw => canvas_c::webgpu::enums::CanvasFrontFace::Ccw,
          GPUFrontFace::cw => canvas_c::webgpu::enums::CanvasFrontFace::Cw,
        },
      },
      cull_mode: match value.cull_mode {
        None => canvas_c::webgpu::enums::CanvasCullMode::None,
        Some(value) => match value {
          GPUCullMode::none => canvas_c::webgpu::enums::CanvasCullMode::None,
          GPUCullMode::front => canvas_c::webgpu::enums::CanvasCullMode::Front,
          GPUCullMode::back => canvas_c::webgpu::enums::CanvasCullMode::Back,
        },
      },
      unclipped_depth: value.unclipped_depth.unwrap_or_default(),
    }
  }
}

#[napi(object)]
pub struct GPUVertexAttribute {
  pub format: GPUVertexFormat,
  pub offset: i64,
  pub shader_location: u32,
}

impl From<canvas_c::webgpu::structs::CanvasVertexAttribute> for GPUVertexAttribute {
  fn from(value: canvas_c::webgpu::structs::CanvasVertexAttribute) -> Self {
    Self {
      format: value.format.into(),
      offset: value.offset as _,
      shader_location: value.shader_location,
    }
  }
}

impl Into<canvas_c::webgpu::structs::CanvasVertexAttribute> for GPUVertexAttribute {
  fn into(self) -> canvas_c::webgpu::structs::CanvasVertexAttribute {
    canvas_c::webgpu::structs::CanvasVertexAttribute {
      format: self.format.into(),
      offset: self.offset as _,
      shader_location: self.shader_location,
    }
  }
}

#[napi(object)]
pub struct GPUVertexBufferLayout {
  pub array_stride: i64,
  pub attributes: Vec<GPUVertexAttribute>,
  pub step_mode: Option<GPUVertexStepMode>,
}

impl From<CanvasVertexBufferLayout> for GPUVertexBufferLayout {
  fn from(value: CanvasVertexBufferLayout) -> Self {
    Self {
      array_stride: value.array_stride as i64,
      attributes: {
        if value.attributes.is_null() || value.attributes_size == 0 {
          Vec::new()
        } else {
          unsafe {
            let values = std::slice::from_raw_parts(value.attributes, value.attributes_size);
            values.iter().map(|v| (*v).into()).collect()
          }
        }
      },
      step_mode: Some(value.step_mode.into()),
    }
  }
}

#[napi(object)]
pub struct GPUVertexState {
  pub buffers: Option<Vec<GPUVertexBufferLayout>>,
  pub constants: Option<HashMap<String, f64>>,
  pub entry_point: Option<String>,
  pub module: ClassInstance<g_p_u_shader_module>,
}

#[napi(object)]
pub struct GPURenderPipelineDescriptor {
  pub depth_stencil: Option<GPUDepthStencilState>,
  pub fragment: Option<GPUFragmentState>,
  pub label: Option<String>,
  pub layout: Option<Either<ClassInstance<g_p_u_pipeline_layout>, GPUPipelineLayoutAuto>>,
  pub multisample: Option<GPUMultisampleState>,
  pub primitive: Option<GPUPrimitiveState>,
  pub vertex: GPUVertexState,
}
