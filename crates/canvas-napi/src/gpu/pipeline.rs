//! Render and compute pipeline descriptors, parsed the way the V8 bindings'
//! `GPUDeviceImpl::CreateRenderPipeline` / `CreateComputePipeline` read them (vertex formats as
//! `packages/canvas`'s `parseVertexFormat` ints or as strings, enums as ints or strings), into the
//! canvas-c structs, which point into storage that lives for the duration of one closure.

use std::ffi::CString;
use std::ptr;
use std::sync::Arc;

use canvas_c::webgpu::enums::{
  CanvasCullMode, CanvasFrontFace, CanvasIndexFormat, CanvasOptionalBool,
  CanvasOptionalCompareFunction, CanvasOptionalIndexFormat, CanvasOptionalPrimitiveTopology,
  CanvasPrimitiveTopology, CanvasStencilFaceState, CanvasVertexFormat, CanvasVertexStepMode,
};
use canvas_c::webgpu::gpu_device::{
  CanvasConstants, CanvasCreateRenderPipelineDescriptor, CanvasDepthStencilState,
  CanvasFragmentState, CanvasGPUAutoLayoutMode, CanvasGPUPipelineLayoutOrGPUAutoLayoutMode,
  CanvasPrimitiveState, CanvasProgrammableStage, CanvasVertexBufferLayout, CanvasVertexState,
};
use canvas_c::webgpu::structs::{
  CanvasBlendComponent, CanvasBlendFactor, CanvasBlendOperation, CanvasBlendState,
  CanvasColorTargetState, CanvasMultisampleState, CanvasOptionalBlendState, CanvasVertexAttribute,
};
use napi::bindgen_prelude::Unknown;
use napi::Result;

use crate::gpu::parse::{
  array_field, as_string, blend_factor, blend_operation, boolean, c_str, class, compare_function,
  constants, field, int32, is_nullish, is_object, label, number, stencil_operation, string,
  texture_format_field, type_error, uint32, uint32_value,
};
use crate::gpu::pipeline_layout::g_p_u_pipeline_layout;
use crate::gpu::shader_module::g_p_u_shader_module;

/// `layout`: a `GPUPipelineLayout`, else `"auto"`.
pub(crate) fn layout(descriptor: &Unknown) -> CanvasGPUPipelineLayoutOrGPUAutoLayoutMode {
  match class::<g_p_u_pipeline_layout>(descriptor, c"layout") {
    Some(layout) => CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Layout(Arc::as_ptr(&layout.layout)),
    None => CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Auto(CanvasGPUAutoLayoutMode::Auto),
  }
}

/// A programmable stage's `module`, `entryPoint` and `constants`.
struct Stage {
  module: *const canvas_c::webgpu::gpu_shader_module::CanvasGPUShaderModule,
  entry_point: Option<CString>,
  constants: Option<CanvasConstants>,
}

impl Stage {
  fn parse(stage: &Unknown, what: &str) -> Result<Self> {
    let module = class::<g_p_u_shader_module>(stage, c"module")
      .map(|module| Arc::as_ptr(&module.module))
      .ok_or_else(|| type_error(format!("{what}.module is not a GPUShaderModule")))?;
    Ok(Self {
      module,
      entry_point: string(stage, c"entryPoint").and_then(|entry| CString::new(entry).ok()),
      constants: constants(field(stage, c"constants").as_ref()),
    })
  }

  fn constants_ptr(&self) -> *const CanvasConstants {
    self
      .constants
      .as_ref()
      .map_or(ptr::null(), |constants| constants as *const _)
  }
}

/// `createComputePipeline(descriptor)`: calls `f(label, layout, stage)`.
pub(crate) fn with_compute_pipeline<R>(
  descriptor: &Unknown,
  f: impl FnOnce(
    *const std::ffi::c_char,
    CanvasGPUPipelineLayoutOrGPUAutoLayoutMode,
    &CanvasProgrammableStage,
  ) -> R,
) -> Result<R> {
  if !is_object(descriptor) {
    return Err(type_error(
      "The compute pipeline descriptor is not an object",
    ));
  }
  let label = label(descriptor);
  let compute = field(descriptor, c"compute")
    .filter(is_object)
    .ok_or_else(|| type_error("descriptor.compute is not an object"))?;
  let stage = Stage::parse(&compute, "compute")?;
  let programmable = CanvasProgrammableStage {
    module: stage.module,
    entry_point: c_str(&stage.entry_point),
    constants: stage.constants_ptr(),
  };
  Ok(f(c_str(&label), layout(descriptor), &programmable))
}

const VERTEX_FORMATS: [(&str, CanvasVertexFormat); 45] = [
  ("uint8", CanvasVertexFormat::Uint8),
  ("uint8x2", CanvasVertexFormat::Uint8x2),
  ("uint8x4", CanvasVertexFormat::Uint8x4),
  ("sint8", CanvasVertexFormat::Sint8),
  ("sint8x2", CanvasVertexFormat::Sint8x2),
  ("sint8x4", CanvasVertexFormat::Sint8x4),
  ("unorm8", CanvasVertexFormat::Unorm8),
  ("unorm8x2", CanvasVertexFormat::Unorm8x2),
  ("unorm8x4", CanvasVertexFormat::Unorm8x4),
  ("snorm8", CanvasVertexFormat::Snorm8),
  ("snorm8x2", CanvasVertexFormat::Snorm8x2),
  ("snorm8x4", CanvasVertexFormat::Snorm8x4),
  ("uint16", CanvasVertexFormat::Uint16),
  ("uint16x2", CanvasVertexFormat::Uint16x2),
  ("uint16x4", CanvasVertexFormat::Uint16x4),
  ("sint16", CanvasVertexFormat::Sint16),
  ("sint16x2", CanvasVertexFormat::Sint16x2),
  ("sint16x4", CanvasVertexFormat::Sint16x4),
  ("unorm16", CanvasVertexFormat::Unorm16),
  ("unorm16x2", CanvasVertexFormat::Unorm16x2),
  ("unorm16x4", CanvasVertexFormat::Unorm16x4),
  ("snorm16", CanvasVertexFormat::Snorm16),
  ("snorm16x2", CanvasVertexFormat::Snorm16x2),
  ("snorm16x4", CanvasVertexFormat::Snorm16x4),
  ("float16", CanvasVertexFormat::Float16),
  ("float16x2", CanvasVertexFormat::Float16x2),
  ("float16x4", CanvasVertexFormat::Float16x4),
  ("float32", CanvasVertexFormat::Float32),
  ("float32x2", CanvasVertexFormat::Float32x2),
  ("float32x3", CanvasVertexFormat::Float32x3),
  ("float32x4", CanvasVertexFormat::Float32x4),
  ("uint32", CanvasVertexFormat::Uint32),
  ("uint32x2", CanvasVertexFormat::Uint32x2),
  ("uint32x3", CanvasVertexFormat::Uint32x3),
  ("uint32x4", CanvasVertexFormat::Uint32x4),
  ("sint32", CanvasVertexFormat::Sint32),
  ("sint32x2", CanvasVertexFormat::Sint32x2),
  ("sint32x3", CanvasVertexFormat::Sint32x3),
  ("sint32x4", CanvasVertexFormat::Sint32x4),
  ("float64", CanvasVertexFormat::Float64),
  ("float64x2", CanvasVertexFormat::Float64x2),
  ("float64x3", CanvasVertexFormat::Float64x3),
  ("float64x4", CanvasVertexFormat::Float64x4),
  ("unorm10-10-10-2", CanvasVertexFormat::Unorm10_10_10_2),
  ("unorm8x4-bgra", CanvasVertexFormat::Unorm8x4Bgra),
];

/// A vertex attribute `format`: `parseVertexFormat`'s index (its order is canvas-c's
/// `CanvasVertexFormat`) or the WebGPU name.
fn vertex_format(value: Option<Unknown>) -> Option<CanvasVertexFormat> {
  let value = value?;
  if let Some(index) = uint32_value(&value) {
    return VERTEX_FORMATS
      .get(index as usize)
      .map(|(_, format)| *format);
  }
  let name = as_string(&value)?;
  VERTEX_FORMATS
    .iter()
    .find(|(known, _)| *known == name)
    .map(|(_, format)| *format)
}

fn step_mode(value: Option<Unknown>) -> CanvasVertexStepMode {
  let Some(value) = value else {
    return CanvasVertexStepMode::Vertex;
  };
  match (uint32_value(&value), as_string(&value).as_deref()) {
    (Some(1), _) | (_, Some("instance")) => CanvasVertexStepMode::Instance,
    _ => CanvasVertexStepMode::Vertex,
  }
}

fn primitive(value: &Unknown) -> CanvasPrimitiveState {
  let mut state = CanvasPrimitiveState {
    topology: CanvasOptionalPrimitiveTopology::None,
    strip_index_format: CanvasOptionalIndexFormat::None,
    front_face: CanvasFrontFace::Ccw,
    cull_mode: CanvasCullMode::None,
    unclipped_depth: false,
  };
  let int_or_str = |name: &std::ffi::CStr| {
    let value = field(value, name);
    (
      value.as_ref().and_then(uint32_value),
      value.as_ref().and_then(as_string),
    )
  };
  match int_or_str(c"cullMode") {
    (Some(1), _) => state.cull_mode = CanvasCullMode::Front,
    (Some(2), _) => state.cull_mode = CanvasCullMode::Back,
    (_, Some(mode)) if mode == "front" => state.cull_mode = CanvasCullMode::Front,
    (_, Some(mode)) if mode == "back" => state.cull_mode = CanvasCullMode::Back,
    _ => {}
  }
  match int_or_str(c"frontFace") {
    (Some(1), _) => state.front_face = CanvasFrontFace::Cw,
    (_, Some(face)) if face == "cw" => state.front_face = CanvasFrontFace::Cw,
    _ => {}
  }
  match int_or_str(c"stripIndexFormat") {
    (Some(0), _) => {
      state.strip_index_format = CanvasOptionalIndexFormat::Some(CanvasIndexFormat::Uint16)
    }
    (Some(1), _) => {
      state.strip_index_format = CanvasOptionalIndexFormat::Some(CanvasIndexFormat::Uint32)
    }
    (_, Some(format)) if format == "uint16" => {
      state.strip_index_format = CanvasOptionalIndexFormat::Some(CanvasIndexFormat::Uint16)
    }
    (_, Some(format)) if format == "uint32" => {
      state.strip_index_format = CanvasOptionalIndexFormat::Some(CanvasIndexFormat::Uint32)
    }
    _ => {}
  }
  let topology = match int_or_str(c"topology") {
    (Some(0), _) => Some(CanvasPrimitiveTopology::PointList),
    (Some(1), _) => Some(CanvasPrimitiveTopology::LineList),
    (Some(2), _) => Some(CanvasPrimitiveTopology::LineStrip),
    (Some(3), _) => Some(CanvasPrimitiveTopology::TriangleList),
    (Some(4), _) => Some(CanvasPrimitiveTopology::TriangleStrip),
    (_, Some(topology)) => match topology.as_str() {
      "point-list" => Some(CanvasPrimitiveTopology::PointList),
      "line-list" => Some(CanvasPrimitiveTopology::LineList),
      "line-strip" => Some(CanvasPrimitiveTopology::LineStrip),
      "triangle-list" => Some(CanvasPrimitiveTopology::TriangleList),
      "triangle-strip" => Some(CanvasPrimitiveTopology::TriangleStrip),
      _ => None,
    },
    _ => None,
  };
  if let Some(topology) = topology {
    state.topology = CanvasOptionalPrimitiveTopology::Some(topology);
  }
  if let Some(unclipped) = boolean(value, c"unclippedDepth") {
    state.unclipped_depth = unclipped;
  }
  state
}

fn stencil_face(value: Option<Unknown>) -> CanvasStencilFaceState {
  let mut face = CanvasStencilFaceState::IGNORE;
  if let Some(value) = value.filter(is_object) {
    if let Some(compare) = compare_function(string(&value, c"compare")) {
      face.compare = compare;
    }
    if let Some(op) = stencil_operation(string(&value, c"failOp")) {
      face.fail_op = op;
    }
    if let Some(op) = stencil_operation(string(&value, c"depthFailOp")) {
      face.depth_fail_op = op;
    }
    if let Some(op) = stencil_operation(string(&value, c"passOp")) {
      face.pass_op = op;
    }
  }
  face
}

fn depth_stencil(value: &Unknown) -> Result<CanvasDepthStencilState> {
  let format = texture_format_field(value, c"format")
    .ok_or_else(|| type_error("depthStencil.format is not a valid GPUTextureFormat"))?;
  Ok(CanvasDepthStencilState {
    format,
    // As in the V8 bindings: an absent `depthWriteEnabled` reads as false.
    depth_write_enabled: CanvasOptionalBool::Some(
      field(value, c"depthWriteEnabled")
        .is_some_and(|v| crate::gpu::parse::as_bool(&v) == Some(true)),
    ),
    depth_compare: match compare_function(string(value, c"depthCompare")) {
      Some(compare) => CanvasOptionalCompareFunction::Some(compare),
      None => CanvasOptionalCompareFunction::None,
    },
    stencil_front: stencil_face(field(value, c"stencilFront")),
    stencil_back: stencil_face(field(value, c"stencilBack")),
    stencil_read_mask: uint32(value, c"stencilReadMask").unwrap_or(0xFFFF_FFFF),
    stencil_write_mask: uint32(value, c"stencilWriteMask").unwrap_or(0xFFFF_FFFF),
    depth_bias: int32(value, c"depthBias").unwrap_or(0),
    depth_bias_slope_scale: number(value, c"depthBiasSlopeScale").unwrap_or(0.) as f32,
    depth_bias_clamp: number(value, c"depthBiasClamp").unwrap_or(0.) as f32,
  })
}

fn multisample(value: &Unknown) -> CanvasMultisampleState {
  CanvasMultisampleState {
    count: uint32(value, c"count").unwrap_or(1),
    mask: number(value, c"mask").map_or(0xFFFF_FFFF, |mask| mask.max(0.) as u64),
    alpha_to_coverage_enabled: boolean(value, c"alphaToCoverageEnabled").unwrap_or(false),
  }
}

/// A blend component, with the WebGPU defaults (`one`, `zero`, `add`) for what is left out.
fn blend_component(value: Option<Unknown>) -> CanvasBlendComponent {
  let value = value.filter(is_object);
  let read = |name: &std::ffi::CStr| value.as_ref().and_then(|value| string(value, name));
  CanvasBlendComponent {
    src_factor: blend_factor(read(c"srcFactor")).unwrap_or(CanvasBlendFactor::One),
    dst_factor: blend_factor(read(c"dstFactor")).unwrap_or(CanvasBlendFactor::Zero),
    operation: blend_operation(read(c"operation")).unwrap_or(CanvasBlendOperation::Add),
  }
}

fn color_target(value: &Unknown) -> Result<CanvasColorTargetState> {
  let format = texture_format_field(value, c"format")
    .ok_or_else(|| type_error("A fragment target's format is not a valid GPUTextureFormat"))?;
  let blend = match field(value, c"blend").filter(is_object) {
    Some(blend) => CanvasOptionalBlendState::Some(CanvasBlendState {
      color: blend_component(field(&blend, c"color")),
      alpha: blend_component(field(&blend, c"alpha")),
    }),
    None => CanvasOptionalBlendState::None,
  };
  Ok(CanvasColorTargetState {
    format,
    blend,
    write_mask: uint32(value, c"writeMask").unwrap_or(0xF),
  })
}

/// `createRenderPipeline(descriptor)` / `createRenderPipelineAsync`: calls `f` with the canvas-c
/// descriptor while everything it points at is alive.
pub(crate) fn with_render_pipeline<R>(
  descriptor: &Unknown,
  f: impl FnOnce(&CanvasCreateRenderPipelineDescriptor) -> R,
) -> Result<R> {
  if !is_object(descriptor) {
    return Err(type_error(
      "The render pipeline descriptor is not an object",
    ));
  }
  let label = label(descriptor);

  // vertex
  let vertex_value = field(descriptor, c"vertex")
    .filter(is_object)
    .ok_or_else(|| type_error("descriptor.vertex is not an object"))?;
  let vertex_stage = Stage::parse(&vertex_value, "vertex")?;
  let buffer_values = array_field(&vertex_value, c"buffers").unwrap_or_default();
  let mut attributes: Vec<Vec<CanvasVertexAttribute>> = Vec::with_capacity(buffer_values.len());
  let mut strides_and_modes = Vec::with_capacity(buffer_values.len());
  for buffer in &buffer_values {
    // A `null` slot (a hole in the layout list): no attributes, nothing read.
    if is_nullish(buffer) || !is_object(buffer) {
      attributes.push(Vec::new());
      strides_and_modes.push((0u64, CanvasVertexStepMode::Vertex));
      continue;
    }
    let mut list = Vec::new();
    for attribute in array_field(buffer, c"attributes").unwrap_or_default() {
      if !is_object(&attribute) {
        continue;
      }
      let format = vertex_format(field(&attribute, c"format"))
        .ok_or_else(|| type_error("A vertex attribute's format is not a valid GPUVertexFormat"))?;
      list.push(CanvasVertexAttribute {
        format,
        offset: number(&attribute, c"offset").unwrap_or(0.).max(0.) as u64,
        shader_location: uint32(&attribute, c"shaderLocation").unwrap_or(0),
      });
    }
    attributes.push(list);
    strides_and_modes.push((
      number(buffer, c"arrayStride").unwrap_or(0.).max(0.) as u64,
      step_mode(field(buffer, c"stepMode")),
    ));
  }
  let buffers: Vec<CanvasVertexBufferLayout> = attributes
    .iter()
    .zip(&strides_and_modes)
    .map(
      |(attributes, (array_stride, step_mode))| CanvasVertexBufferLayout {
        array_stride: *array_stride,
        step_mode: *step_mode,
        attributes: if attributes.is_empty() {
          ptr::null()
        } else {
          attributes.as_ptr()
        },
        attributes_size: attributes.len(),
      },
    )
    .collect();
  let vertex = CanvasVertexState {
    module: vertex_stage.module,
    entry_point: c_str(&vertex_stage.entry_point),
    constants: vertex_stage.constants_ptr(),
    buffers: if buffers.is_empty() {
      ptr::null()
    } else {
      buffers.as_ptr()
    },
    buffers_size: buffers.len(),
  };

  // fragment
  let fragment_value = field(descriptor, c"fragment").filter(is_object);
  let fragment_stage = match fragment_value.as_ref() {
    Some(fragment) => Some(Stage::parse(fragment, "fragment")?),
    None => None,
  };
  let mut targets = Vec::new();
  if let Some(fragment) = fragment_value.as_ref() {
    for target in array_field(fragment, c"targets").unwrap_or_default() {
      // canvas-c has no sparse targets: holes are skipped.
      if !is_object(&target) {
        continue;
      }
      targets.push(color_target(&target)?);
    }
  }
  let fragment = fragment_stage.as_ref().map(|stage| CanvasFragmentState {
    targets: if targets.is_empty() {
      ptr::null()
    } else {
      targets.as_ptr()
    },
    targets_size: targets.len(),
    module: stage.module,
    entry_point: c_str(&stage.entry_point),
    constants: stage.constants_ptr(),
  });

  let primitive = field(descriptor, c"primitive")
    .filter(is_object)
    .map(|v| primitive(&v));
  let depth_stencil = match field(descriptor, c"depthStencil").filter(is_object) {
    Some(value) => Some(depth_stencil(&value)?),
    None => None,
  };
  let multisample = field(descriptor, c"multisample")
    .filter(is_object)
    .map(|v| multisample(&v));

  let desc = CanvasCreateRenderPipelineDescriptor {
    label: c_str(&label),
    layout: layout(descriptor),
    vertex: &vertex,
    primitive: primitive.as_ref().map_or(ptr::null(), |v| v as *const _),
    depth_stencil: depth_stencil
      .as_ref()
      .map_or(ptr::null(), |v| v as *const _),
    multisample: multisample.as_ref().map_or(ptr::null(), |v| v as *const _),
    fragment: fragment.as_ref().map_or(ptr::null(), |v| v as *const _),
  };
  Ok(f(&desc))
}
