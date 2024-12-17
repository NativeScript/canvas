use canvas_c::webgpu::enums::{
  CanvasAddressMode, CanvasAstcBlock, CanvasAstcChannel, CanvasBufferBindingType,
  CanvasCompareFunction, CanvasCullMode, CanvasFilterMode, CanvasFrontFace, CanvasGPUTextureFormat,
  CanvasIndexFormat, CanvasPrimitiveTopology, CanvasQueryType, CanvasSamplerBindingType,
  CanvasStencilOperation, CanvasStorageTextureAccess, CanvasTextureAspect, CanvasTextureDimension,
  CanvasTextureSampleType, CanvasTextureViewDimension, CanvasVertexFormat, CanvasVertexStepMode,
};
use canvas_c::webgpu::structs::{
  CanvasBlendFactor, CanvasBlendOperation, CanvasLoadOp, CanvasStoreOp,
};
use napi::*;
use napi_derive::napi;

use canvas_c::webgpu::gpu_buffer::GPUMapMode as CGPUMapMode;
use canvas_c::webgpu::gpu_device::CanvasGPUErrorFilter;
use canvas_c::webgpu::wgt::TextureAspect;

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUShaderStage")]
pub enum GPUShaderStage {
  VERTEX = 0x1,
  FRAGMENT = 0x2,
  COMPUTE = 0x4,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUQueryType", string_enum)]
pub enum GPUQueryType {
  occlusion,
  timestamp,
}

impl Into<CanvasQueryType> for GPUQueryType {
  fn into(self) -> CanvasQueryType {
    match self {
      GPUQueryType::occlusion => CanvasQueryType::Occlusion,
      GPUQueryType::timestamp => CanvasQueryType::Timestamp,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUMipmapFilterMode", string_enum)]
pub enum GPUMipmapFilterMode {
  nearest,
  linear,
}

impl Into<CanvasFilterMode> for GPUMipmapFilterMode {
  fn into(self) -> CanvasFilterMode {
    match self {
      GPUMipmapFilterMode::nearest => CanvasFilterMode::Nearest,
      GPUMipmapFilterMode::linear => CanvasFilterMode::Linear,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUFilterMode", string_enum)]
pub enum GPUFilterMode {
  nearest,
  linear,
}

impl Into<CanvasFilterMode> for GPUFilterMode {
  fn into(self) -> CanvasFilterMode {
    match self {
      GPUFilterMode::nearest => CanvasFilterMode::Nearest,
      GPUFilterMode::linear => CanvasFilterMode::Linear,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUAddressMode", string_enum = "kebab-case")]
pub enum GPUAddressMode {
  clampToEdge,
  repeat,
  mirrorRepeat,
}

impl Into<CanvasAddressMode> for GPUAddressMode {
  fn into(self) -> CanvasAddressMode {
    match self {
      GPUAddressMode::clampToEdge => CanvasAddressMode::ClampToEdge,
      GPUAddressMode::repeat => CanvasAddressMode::Repeat,
      GPUAddressMode::mirrorRepeat => CanvasAddressMode::MirrorRepeat,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUBufferUsage")]
pub enum GPUBufferUsage {
  MAP_READ = 0x0001,
  MAP_WRITE = 0x0002,
  COPY_SRC = 0x0004,
  COPY_DST = 0x0008,
  INDEX = 0x0010,
  VERTEX = 0x0020,
  UNIFORM = 0x0040,
  STORAGE = 0x0080,
  INDIRECT = 0x0100,
  QUERY_RESOLVE = 0x0200,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUTextureUsage")]
pub enum GPUTextureUsage {
  COPY_SRC = 0x01,
  COPY_DST = 0x02,
  RENDER_ATTACHMENT = 0x10,
  STORAGE_BINDING = 0x08,
  TEXTURE_BINDING = 0x04,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUErrorFilter", string_enum = "kebab-case")]
pub enum GPUErrorFilter {
  validation,
  outOfMemory,
  internal,
}

impl Into<CanvasGPUErrorFilter> for GPUErrorFilter {
  fn into(self) -> CanvasGPUErrorFilter {
    match self {
      GPUErrorFilter::validation => CanvasGPUErrorFilter::Validation,
      GPUErrorFilter::outOfMemory => CanvasGPUErrorFilter::OutOfMemory,
      GPUErrorFilter::internal => CanvasGPUErrorFilter::Internal,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUTextureSampleType", string_enum = "kebab-case")]
#[derive(Debug)]
pub enum GPUTextureSampleType {
  float,
  unfilterableFloat,
  depth,
  sint,
  uint,
}

impl Into<CanvasTextureSampleType> for GPUTextureSampleType {
  fn into(self) -> CanvasTextureSampleType {
    match self {
      GPUTextureSampleType::float => CanvasTextureSampleType::Float,
      GPUTextureSampleType::unfilterableFloat => CanvasTextureSampleType::UnfilterableFloat,
      GPUTextureSampleType::depth => CanvasTextureSampleType::Depth,
      GPUTextureSampleType::sint => CanvasTextureSampleType::Sint,
      GPUTextureSampleType::uint => CanvasTextureSampleType::Uint,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUTextureViewDimension", string_enum = "kebab-case")]
#[derive(Debug)]
pub enum GPUTextureViewDimension {
  #[napi(value = "1d")]
  d1,
  #[napi(value = "2d")]
  d2,
  #[napi(value = "2d-array")]
  array2d,
  cube,
  cubeArray,
  #[napi(value = "3d")]
  d3,
}

impl Into<CanvasTextureViewDimension> for GPUTextureViewDimension {
  fn into(self) -> CanvasTextureViewDimension {
    match self {
      GPUTextureViewDimension::d1 => CanvasTextureViewDimension::D1,
      GPUTextureViewDimension::d2 => CanvasTextureViewDimension::D2,
      GPUTextureViewDimension::array2d => CanvasTextureViewDimension::D2Array,
      GPUTextureViewDimension::cube => CanvasTextureViewDimension::Cube,
      GPUTextureViewDimension::cubeArray => CanvasTextureViewDimension::CubeArray,
      GPUTextureViewDimension::d3 => CanvasTextureViewDimension::D3,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUStorageTextureAccess", string_enum = "kebab-case")]
#[derive(Debug)]
pub enum GPUStorageTextureAccess {
  writeOnly,
  readOnly,
  readWrite,
}

impl Into<CanvasStorageTextureAccess> for GPUStorageTextureAccess {
  fn into(self) -> CanvasStorageTextureAccess {
    match self {
      GPUStorageTextureAccess::writeOnly => CanvasStorageTextureAccess::WriteOnly,
      GPUStorageTextureAccess::readOnly => CanvasStorageTextureAccess::ReadOnly,
      GPUStorageTextureAccess::readWrite => CanvasStorageTextureAccess::ReadWrite,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUSamplerBindingType", string_enum = "kebab-case")]
#[derive(Debug)]
pub enum GPUSamplerBindingType {
  filtering,
  nonFiltering,
  comparison,
}

impl Into<CanvasSamplerBindingType> for GPUSamplerBindingType {
  fn into(self) -> CanvasSamplerBindingType {
    match self {
      GPUSamplerBindingType::filtering => CanvasSamplerBindingType::Filtering,
      GPUSamplerBindingType::nonFiltering => CanvasSamplerBindingType::NonFiltering,
      GPUSamplerBindingType::comparison => CanvasSamplerBindingType::Comparison,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUBufferBindingType", string_enum = "kebab-case")]
#[derive(Debug)]
pub enum GPUBufferBindingType {
  uniform,
  storage,
  readOnlyStorage,
}

impl Into<CanvasBufferBindingType> for GPUBufferBindingType {
  fn into(self) -> CanvasBufferBindingType {
    match self {
      GPUBufferBindingType::uniform => CanvasBufferBindingType::Uniform,
      GPUBufferBindingType::storage => CanvasBufferBindingType::Storage,
      GPUBufferBindingType::readOnlyStorage => CanvasBufferBindingType::ReadOnlyStorage,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUTextureDimension", string_enum)]
#[derive(Debug)]
pub enum GPUTextureDimension {
  #[napi(value = "1d")]
  d1,
  #[napi(value = "2d")]
  d2,
  #[napi(value = "3d")]
  d3,
}

impl Into<CanvasTextureDimension> for GPUTextureDimension {
  fn into(self) -> CanvasTextureDimension {
    match self {
      GPUTextureDimension::d1 => CanvasTextureDimension::D1,
      GPUTextureDimension::d2 => CanvasTextureDimension::D2,
      GPUTextureDimension::d3 => CanvasTextureDimension::D3,
    }
  }
}

impl From<CanvasTextureDimension> for GPUTextureDimension {
  fn from(value: CanvasTextureDimension) -> Self {
    match value {
      CanvasTextureDimension::D1 => Self::d1,
      CanvasTextureDimension::D2 => Self::d2,
      CanvasTextureDimension::D3 => Self::d3,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUTextureAspect", string_enum = "kebab-case")]
pub enum GPUTextureAspect {
  all,
  stencilOnly,
  depthOnly,
}

impl Into<TextureAspect> for GPUTextureAspect {
  fn into(self) -> TextureAspect {
    match self {
      GPUTextureAspect::all => TextureAspect::All,
      GPUTextureAspect::stencilOnly => TextureAspect::StencilOnly,
      GPUTextureAspect::depthOnly => TextureAspect::DepthOnly,
    }
  }
}

impl Into<CanvasTextureAspect> for GPUTextureAspect {
  fn into(self) -> CanvasTextureAspect {
    match self {
      GPUTextureAspect::all => CanvasTextureAspect::All,
      GPUTextureAspect::stencilOnly => CanvasTextureAspect::StencilOnly,
      GPUTextureAspect::depthOnly => CanvasTextureAspect::DepthOnly,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUMapMode")]
pub enum GPUMapMode {
  READ = 0x0001,
  WRITE = 0x0002,
}

impl From<GPUMapMode> for CGPUMapMode {
  fn from(value: GPUMapMode) -> Self {
    match value {
      GPUMapMode::READ => Self::Read,
      GPUMapMode::WRITE => Self::Write,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPULoadOp", string_enum)]
pub enum GPULoadOp {
  load,
  clear,
}

impl From<GPULoadOp> for CanvasLoadOp {
  fn from(value: GPULoadOp) -> Self {
    match value {
      GPULoadOp::load => Self::Load,
      GPULoadOp::clear => Self::Clear,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUStoreOp", string_enum)]
pub enum GPUStoreOp {
  #[napi(value = "store")]
  Store,
  #[napi(value = "discard")]
  Discard,
}

impl From<GPUStoreOp> for CanvasStoreOp {
  fn from(value: GPUStoreOp) -> Self {
    match value {
      GPUStoreOp::Store => Self::Store,
      GPUStoreOp::Discard => Self::Discard,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUVertexFormat", string_enum = "lowercase")]
pub enum GPUVertexFormat {
  uint8x2,
  uint8x4,
  sint8x2,
  sint8x4,
  unorm8x2,
  unorm8x4,
  snorm8x2,
  snorm8x4,
  uint16x2,
  uint16x4,
  sint16x2,
  sint16x4,
  unorm16x2,
  unorm16x4,
  snorm16x2,
  snorm16x4,
  float16x2,
  float16x4,
  float32,
  float32x2,
  float32x3,
  float32x4,
  uint32,
  uint32x2,
  uint32x3,
  uint32x4,
  sint32,
  sint32x2,
  sint32x3,
  sint32x4,
  float64,
  float64x2,
  float64x3,
  float64x4,
  #[napi(value = "unorm10-10-10-2")]
  unorm1010102,
}

impl From<CanvasVertexFormat> for GPUVertexFormat {
  fn from(value: CanvasVertexFormat) -> Self {
    match value {
      CanvasVertexFormat::Uint8x2 => GPUVertexFormat::uint8x2,
      CanvasVertexFormat::Uint8x4 => GPUVertexFormat::uint8x4,
      CanvasVertexFormat::Sint8x2 => GPUVertexFormat::sint8x2,
      CanvasVertexFormat::Sint8x4 => GPUVertexFormat::sint8x4,
      CanvasVertexFormat::Unorm8x2 => GPUVertexFormat::unorm8x2,
      CanvasVertexFormat::Unorm8x4 => GPUVertexFormat::unorm8x4,
      CanvasVertexFormat::Snorm8x2 => GPUVertexFormat::snorm8x2,
      CanvasVertexFormat::Snorm8x4 => GPUVertexFormat::snorm8x4,
      CanvasVertexFormat::Uint16x2 => GPUVertexFormat::uint16x2,
      CanvasVertexFormat::Uint16x4 => GPUVertexFormat::uint16x4,
      CanvasVertexFormat::Sint16x2 => GPUVertexFormat::sint16x2,
      CanvasVertexFormat::Sint16x4 => GPUVertexFormat::sint16x4,
      CanvasVertexFormat::Unorm16x2 => GPUVertexFormat::unorm16x2,
      CanvasVertexFormat::Unorm16x4 => GPUVertexFormat::unorm16x4,
      CanvasVertexFormat::Snorm16x2 => GPUVertexFormat::snorm16x2,
      CanvasVertexFormat::Snorm16x4 => GPUVertexFormat::snorm16x4,
      CanvasVertexFormat::Float16x2 => GPUVertexFormat::float16x2,
      CanvasVertexFormat::Float16x4 => GPUVertexFormat::float16x4,
      CanvasVertexFormat::Float32 => GPUVertexFormat::float32,
      CanvasVertexFormat::Float32x2 => GPUVertexFormat::float32x2,
      CanvasVertexFormat::Float32x3 => GPUVertexFormat::float32x3,
      CanvasVertexFormat::Float32x4 => GPUVertexFormat::float32x4,
      CanvasVertexFormat::Uint32 => GPUVertexFormat::uint32,
      CanvasVertexFormat::Uint32x2 => GPUVertexFormat::uint32x2,
      CanvasVertexFormat::Uint32x3 => GPUVertexFormat::uint32x3,
      CanvasVertexFormat::Uint32x4 => GPUVertexFormat::uint32x4,
      CanvasVertexFormat::Sint32 => GPUVertexFormat::sint32,
      CanvasVertexFormat::Sint32x2 => GPUVertexFormat::sint32x2,
      CanvasVertexFormat::Sint32x3 => GPUVertexFormat::sint32x3,
      CanvasVertexFormat::Sint32x4 => GPUVertexFormat::sint32x4,
      CanvasVertexFormat::Float64 => GPUVertexFormat::float64,
      CanvasVertexFormat::Float64x2 => GPUVertexFormat::float64x2,
      CanvasVertexFormat::Float64x3 => GPUVertexFormat::float64x3,
      CanvasVertexFormat::Float64x4 => GPUVertexFormat::float64x4,
      CanvasVertexFormat::Unorm10_10_10_2 => GPUVertexFormat::unorm1010102,
    }
  }
}

impl Into<CanvasVertexFormat> for GPUVertexFormat {
  fn into(self) -> CanvasVertexFormat {
    match self {
      GPUVertexFormat::uint8x2 => CanvasVertexFormat::Uint8x2,
      GPUVertexFormat::uint8x4 => CanvasVertexFormat::Uint8x4,
      GPUVertexFormat::sint8x2 => CanvasVertexFormat::Sint8x2,
      GPUVertexFormat::sint8x4 => CanvasVertexFormat::Sint8x4,
      GPUVertexFormat::unorm8x2 => CanvasVertexFormat::Unorm8x2,
      GPUVertexFormat::unorm8x4 => CanvasVertexFormat::Unorm8x4,
      GPUVertexFormat::snorm8x2 => CanvasVertexFormat::Snorm8x2,
      GPUVertexFormat::snorm8x4 => CanvasVertexFormat::Snorm8x4,
      GPUVertexFormat::uint16x2 => CanvasVertexFormat::Uint16x2,
      GPUVertexFormat::uint16x4 => CanvasVertexFormat::Uint16x4,
      GPUVertexFormat::sint16x2 => CanvasVertexFormat::Sint16x2,
      GPUVertexFormat::sint16x4 => CanvasVertexFormat::Sint16x4,
      GPUVertexFormat::unorm16x2 => CanvasVertexFormat::Unorm16x2,
      GPUVertexFormat::unorm16x4 => CanvasVertexFormat::Unorm16x4,
      GPUVertexFormat::snorm16x2 => CanvasVertexFormat::Snorm16x2,
      GPUVertexFormat::snorm16x4 => CanvasVertexFormat::Snorm16x4,
      GPUVertexFormat::float16x2 => CanvasVertexFormat::Float16x2,
      GPUVertexFormat::float16x4 => CanvasVertexFormat::Float16x4,
      GPUVertexFormat::float32 => CanvasVertexFormat::Float32,
      GPUVertexFormat::float32x2 => CanvasVertexFormat::Float32x2,
      GPUVertexFormat::float32x3 => CanvasVertexFormat::Float32x3,
      GPUVertexFormat::float32x4 => CanvasVertexFormat::Float32x4,
      GPUVertexFormat::uint32 => CanvasVertexFormat::Uint32,
      GPUVertexFormat::uint32x2 => CanvasVertexFormat::Uint32x2,
      GPUVertexFormat::uint32x3 => CanvasVertexFormat::Uint32x3,
      GPUVertexFormat::uint32x4 => CanvasVertexFormat::Uint32x4,
      GPUVertexFormat::sint32 => CanvasVertexFormat::Sint32,
      GPUVertexFormat::sint32x2 => CanvasVertexFormat::Sint32x2,
      GPUVertexFormat::sint32x3 => CanvasVertexFormat::Sint32x3,
      GPUVertexFormat::sint32x4 => CanvasVertexFormat::Sint32x4,
      GPUVertexFormat::float64 => CanvasVertexFormat::Float64,
      GPUVertexFormat::float64x2 => CanvasVertexFormat::Float64x2,
      GPUVertexFormat::float64x3 => CanvasVertexFormat::Float64x3,
      GPUVertexFormat::float64x4 => CanvasVertexFormat::Float64x4,
      GPUVertexFormat::unorm1010102 => CanvasVertexFormat::Unorm10_10_10_2,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUVertexStepMode", string_enum)]
pub enum GPUVertexStepMode {
  vertex,
  instance,
}

impl From<CanvasVertexStepMode> for GPUVertexStepMode {
  fn from(value: CanvasVertexStepMode) -> Self {
    match value {
      CanvasVertexStepMode::Vertex => GPUVertexStepMode::vertex,
      CanvasVertexStepMode::Instance => GPUVertexStepMode::instance,
    }
  }
}

impl Into<CanvasVertexStepMode> for GPUVertexStepMode {
  fn into(self) -> CanvasVertexStepMode {
    match self {
      GPUVertexStepMode::vertex => CanvasVertexStepMode::Vertex,
      GPUVertexStepMode::instance => CanvasVertexStepMode::Instance,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUPrimitiveTopology", string_enum = "kebab-case")]
pub enum GPUPrimitiveTopology {
  pointList,
  lineList,
  lineStrip,
  triangleList,
  triangleStrip,
}

impl From<CanvasPrimitiveTopology> for GPUPrimitiveTopology {
  fn from(value: CanvasPrimitiveTopology) -> Self {
    match value {
      CanvasPrimitiveTopology::PointList => GPUPrimitiveTopology::pointList,
      CanvasPrimitiveTopology::LineList => GPUPrimitiveTopology::lineList,
      CanvasPrimitiveTopology::LineStrip => GPUPrimitiveTopology::lineStrip,
      CanvasPrimitiveTopology::TriangleList => GPUPrimitiveTopology::triangleList,
      CanvasPrimitiveTopology::TriangleStrip => GPUPrimitiveTopology::triangleStrip,
    }
  }
}

impl Into<CanvasPrimitiveTopology> for GPUPrimitiveTopology {
  fn into(self) -> CanvasPrimitiveTopology {
    match self {
      GPUPrimitiveTopology::pointList => CanvasPrimitiveTopology::PointList,
      GPUPrimitiveTopology::lineList => CanvasPrimitiveTopology::LineList,
      GPUPrimitiveTopology::lineStrip => CanvasPrimitiveTopology::LineStrip,
      GPUPrimitiveTopology::triangleList => CanvasPrimitiveTopology::TriangleList,
      GPUPrimitiveTopology::triangleStrip => CanvasPrimitiveTopology::TriangleStrip,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUIndexFormat", string_enum)]
pub enum GPUIndexFormat {
  uint16,
  uint32,
}

impl From<CanvasIndexFormat> for GPUIndexFormat {
  fn from(value: CanvasIndexFormat) -> Self {
    match value {
      CanvasIndexFormat::Uint16 => Self::uint16,
      CanvasIndexFormat::Uint32 => Self::uint32,
    }
  }
}

impl Into<CanvasIndexFormat> for GPUIndexFormat {
  fn into(self) -> CanvasIndexFormat {
    match self {
      GPUIndexFormat::uint16 => CanvasIndexFormat::Uint16,
      GPUIndexFormat::uint32 => CanvasIndexFormat::Uint32,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUFrontFace", string_enum)]
pub enum GPUFrontFace {
  ccw,
  cw,
}

impl From<CanvasFrontFace> for GPUFrontFace {
  fn from(value: CanvasFrontFace) -> Self {
    match value {
      CanvasFrontFace::Ccw => Self::ccw,
      CanvasFrontFace::Cw => GPUFrontFace::cw,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUCullMode", string_enum)]
pub enum GPUCullMode {
  none,
  front,
  back,
}

impl From<CanvasCullMode> for GPUCullMode {
  fn from(value: CanvasCullMode) -> Self {
    match value {
      CanvasCullMode::None => GPUCullMode::none,
      CanvasCullMode::Front => GPUCullMode::front,
      CanvasCullMode::Back => GPUCullMode::back,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUBlendOperation", string_enum = "kebab-case")]
pub enum GPUBlendOperation {
  add,
  subtract,
  reverseSubtract,
  min,
  max,
}

impl From<CanvasBlendOperation> for GPUBlendOperation {
  fn from(value: CanvasBlendOperation) -> Self {
    match value {
      CanvasBlendOperation::Add => Self::add,
      CanvasBlendOperation::Subtract => Self::subtract,
      CanvasBlendOperation::ReverseSubtract => Self::reverseSubtract,
      CanvasBlendOperation::Min => Self::min,
      CanvasBlendOperation::Max => Self::max,
    }
  }
}

impl Into<CanvasBlendOperation> for GPUBlendOperation {
  fn into(self) -> CanvasBlendOperation {
    match self {
      GPUBlendOperation::add => CanvasBlendOperation::Add,
      GPUBlendOperation::subtract => CanvasBlendOperation::Subtract,
      GPUBlendOperation::reverseSubtract => CanvasBlendOperation::ReverseSubtract,
      GPUBlendOperation::min => CanvasBlendOperation::Min,
      GPUBlendOperation::max => CanvasBlendOperation::Max,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUBlendFactor", string_enum = "kebab-case")]
pub enum GPUBlendFactor {
  zero,
  one,
  src,
  oneMinusSrc,
  srcAlpha,
  oneMinusSrcAlpha,
  dst,
  oneMinusDst,
  dstAlpha,
  oneMinusDstAlpha,
  srcAlphaSaturated,
  constant,
  oneMinusConstant,
  #[napi(value = "src1")]
  src1,
  #[napi(value = "oneMinusSrc1")]
  oneMinusSrc1,
  #[napi(value = "src1Alpha")]
  src1Alpha,
  #[napi(value = "oneMinusSrc1Alpha")]
  oneMinusSrc1Alpha,
}

impl From<CanvasBlendFactor> for GPUBlendFactor {
  fn from(value: CanvasBlendFactor) -> Self {
    match value {
      CanvasBlendFactor::Zero => Self::zero,
      CanvasBlendFactor::One => Self::one,
      CanvasBlendFactor::Src => Self::src,
      CanvasBlendFactor::OneMinusSrc => Self::oneMinusSrc,
      CanvasBlendFactor::SrcAlpha => Self::srcAlpha,
      CanvasBlendFactor::OneMinusSrcAlpha => Self::oneMinusSrcAlpha,
      CanvasBlendFactor::Dst => Self::dst,
      CanvasBlendFactor::OneMinusDst => Self::oneMinusDst,
      CanvasBlendFactor::DstAlpha => Self::dstAlpha,
      CanvasBlendFactor::OneMinusDstAlpha => Self::oneMinusDstAlpha,
      CanvasBlendFactor::SrcAlphaSaturated => Self::srcAlphaSaturated,
      CanvasBlendFactor::Constant => Self::constant,
      CanvasBlendFactor::OneMinusConstant => Self::oneMinusConstant,
      CanvasBlendFactor::Src1 => Self::src1,
      CanvasBlendFactor::OneMinusSrc1 => Self::oneMinusSrc1,
      CanvasBlendFactor::Src1Alpha => Self::src1Alpha,
      CanvasBlendFactor::OneMinusSrc1Alpha => Self::oneMinusSrc1Alpha,
    }
  }
}

impl Into<CanvasBlendFactor> for GPUBlendFactor {
  fn into(self) -> CanvasBlendFactor {
    match self {
      GPUBlendFactor::zero => CanvasBlendFactor::Zero,
      GPUBlendFactor::one => CanvasBlendFactor::One,
      GPUBlendFactor::src => CanvasBlendFactor::Src,
      GPUBlendFactor::oneMinusSrc => CanvasBlendFactor::OneMinusSrc,
      GPUBlendFactor::srcAlpha => CanvasBlendFactor::SrcAlpha,
      GPUBlendFactor::oneMinusSrcAlpha => CanvasBlendFactor::OneMinusSrcAlpha,
      GPUBlendFactor::dst => CanvasBlendFactor::Dst,
      GPUBlendFactor::oneMinusDst => CanvasBlendFactor::OneMinusDst,
      GPUBlendFactor::dstAlpha => CanvasBlendFactor::DstAlpha,
      GPUBlendFactor::oneMinusDstAlpha => CanvasBlendFactor::OneMinusDstAlpha,
      GPUBlendFactor::srcAlphaSaturated => CanvasBlendFactor::SrcAlphaSaturated,
      GPUBlendFactor::constant => CanvasBlendFactor::Constant,
      GPUBlendFactor::oneMinusConstant => CanvasBlendFactor::OneMinusConstant,
      GPUBlendFactor::src1 => CanvasBlendFactor::Src1,
      GPUBlendFactor::oneMinusSrc1 => CanvasBlendFactor::OneMinusSrc1,
      GPUBlendFactor::src1Alpha => CanvasBlendFactor::Src1Alpha,
      GPUBlendFactor::oneMinusSrc1Alpha => CanvasBlendFactor::OneMinusSrc1Alpha,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUPipelineLayoutAuto", string_enum)]
pub enum GPUPipelineLayoutAuto {
  auto,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUStencilOperation", string_enum = "kebab-case")]
pub enum GPUStencilOperation {
  keep,
  zero,
  replace,
  invert,
  incrementClamp,
  decrementClamp,
  incrementWrap,
  decrementWrap,
}

impl Into<CanvasStencilOperation> for GPUStencilOperation {
  fn into(self) -> CanvasStencilOperation {
    match self {
      GPUStencilOperation::keep => CanvasStencilOperation::Keep,
      GPUStencilOperation::zero => CanvasStencilOperation::Zero,
      GPUStencilOperation::replace => CanvasStencilOperation::Replace,
      GPUStencilOperation::invert => CanvasStencilOperation::Invert,
      GPUStencilOperation::incrementClamp => CanvasStencilOperation::IncrementClamp,
      GPUStencilOperation::decrementClamp => CanvasStencilOperation::DecrementClamp,
      GPUStencilOperation::incrementWrap => CanvasStencilOperation::IncrementWrap,
      GPUStencilOperation::decrementWrap => CanvasStencilOperation::DecrementWrap,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUCompareFunction", string_enum = "kebab-case")]
pub enum GPUCompareFunction {
  never,
  less,
  equal,
  lessEqual,
  greater,
  notEqual,
  greaterEqual,
  always,
}

impl Into<CanvasCompareFunction> for GPUCompareFunction {
  fn into(self) -> CanvasCompareFunction {
    match self {
      GPUCompareFunction::never => CanvasCompareFunction::Never,
      GPUCompareFunction::less => CanvasCompareFunction::Less,
      GPUCompareFunction::equal => CanvasCompareFunction::Equal,
      GPUCompareFunction::lessEqual => CanvasCompareFunction::LessEqual,
      GPUCompareFunction::greater => CanvasCompareFunction::Greater,
      GPUCompareFunction::notEqual => CanvasCompareFunction::NotEqual,
      GPUCompareFunction::greaterEqual => CanvasCompareFunction::GreaterEqual,
      GPUCompareFunction::always => CanvasCompareFunction::Always,
    }
  }
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "PredefinedColorSpaceEnum", string_enum = "kebab-case")]
pub enum PredefinedColorSpaceEnum {
  #[napi(value = "display-p3")]
  displayP3,
  srgb,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUCanvasAlphaMode", string_enum)]
pub enum GPUCanvasAlphaMode {
  opaque,
  premultiplied,
  postmultiplied,
  inherit,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUCanvasPresentMode", string_enum = "camelCase")]
pub enum GPUCanvasPresentMode {
  autoVsync,
  autoNoVsync,
  fifo,
  fifoRelaxed,
  immediate,
  mailbox,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUTextureFormat", string_enum = "lowercase")]
#[derive(Debug)]
pub enum GPUTextureFormat {
  r8unorm,
  r8snorm,
  r8uint,
  r8sint,
  r16uint,
  r16sint,
  r16unorm,
  r16snorm,
  r16float,
  rg8unorm,
  rg8snorm,
  rg8uint,
  rg8sint,
  rg16unorm,
  rg16snorm,
  r32uint,
  r32sint,
  r32float,
  rg16uint,
  rg16sint,
  rg16float,
  rgba8unorm,
  #[napi(value = "rgba8unorm-srgb")]
  rgba8unorm_srgb,
  rgba8snorm,
  rgba8uint,
  rgba8sint,
  bgra8unorm,
  #[napi(value = "bgra8unorm-srgb")]
  bgra8unorm_srgb,
  rgb9e5ufloat,
  rgb10a2uint,
  rgb10a2unorm,
  rg11b10ufloat,
  rg32uint,
  rg32sint,
  rg32float,
  rgba16uint,
  rgba16sint,
  rgba16unorm,
  rgba16snorm,
  rgba16float,
  rgba32uint,
  rgba32sint,
  rgba32float,
  stencil8,
  nv12,
  depth16unorm,
  depth24plus,
  #[napi(value = "depth24plus_stencil8")]
  depth24plus_stencil8,
  depth32float,
  #[napi(value = "depth32float-stencil8")]
  depth32float_stencil8,
  #[napi(value = "bc1-rgba-unorm")]
  bc1_rgba_unorm,
  #[napi(value = "bc1-rgba-unorm-srgb")]
  bc1_rgba_unorm_srgb,
  #[napi(value = "bc2-rgba-unorm")]
  bc2_rgba_unorm,
  #[napi(value = "bc2-rgba-unorm-srgb")]
  bc2_rgba_unorm_srgb,
  #[napi(value = "bc3-rgba-unorm")]
  bc3_rgba_unorm,
  #[napi(value = "bc3-rgba-unorm-srgb")]
  bc3_rgba_unorm_srgb,
  #[napi(value = "bc4-r-unorm")]
  bc4_r_unorm,
  #[napi(value = "bc4-r-snorm")]
  bc4_r_snorm,
  #[napi(value = "bc5-rg-unorm")]
  bc5_rg_unorm,
  #[napi(value = "bc5-rg-snorm")]
  bc5_rg_snorm,
  #[napi(value = "bc6h-rgb-ufloat")]
  bc6h_rgb_ufloat,
  #[napi(value = "bc6h-rgb-float")]
  bc6h_rgb_float,
  #[napi(value = "bc7-rgba-unorm")]
  bc7_rgba_unorm,
  #[napi(value = "bc7-rgba-unorm-srgb")]
  bc7_rgba_unorm_srgb,
  #[napi(value = "etc2-rgb8unorm")]
  etc2_rgb8unorm,
  #[napi(value = "etc2-rgb8unorm-srgb")]
  etc2_rgb8unorm_srgb,
  #[napi(value = "etc2-rgb8a1unorm")]
  etc2_rgb8a1unorm,
  #[napi(value = "etc2-rgb8a1unorm-srgb")]
  etc2_rgb8a1unorm_srgb,
  #[napi(value = "etc2-rgba8unorm")]
  etc2_rgba8unorm,
  #[napi(value = "etc2-rgba8unorm-srgb")]
  etc2_rgba8unorm_srgb,
  #[napi(value = "eac-r11unorm")]
  eac_r11unorm,
  #[napi(value = "eac-r11snorm")]
  eac_r11snorm,
  #[napi(value = "eac-rg11unorm")]
  eac_rg11unorm,
  #[napi(value = "eac-rg11snorm")]
  eac_rg11snorm,
  #[napi(value = "astc-4x4-unorm")]
  astc_4x4_unorm,
  #[napi(value = "astc-4x4-unorm-srgb")]
  astc_4x4_unorm_srgb,
  #[napi(value = "astc-4x4-hdr")]
  astc_4x4_hdr,
  #[napi(value = "astc-5x4-unorm")]
  astc_5x4_unorm,
  #[napi(value = "astc-5x4-unorm-srgb")]
  astc_5x4_unorm_srgb,
  #[napi(value = "astc-5x4-hdr")]
  astc_5x4_hdr,
  #[napi(value = "astc-5x5-unorm")]
  astc_5x5_unorm,
  #[napi(value = "astc-5x5-unorm-srgb")]
  astc_5x5_unorm_srgb,
  #[napi(value = "astc-5x5-hdr")]
  astc_5x5_hdr,
  #[napi(value = "astc-6x5-unorm")]
  astc_6x5_unorm,
  #[napi(value = "astc-6x5-unorm-srgb")]
  astc_6x5_unorm_srgb,
  #[napi(value = "astc-6x5-hdr")]
  astc_6x5_hdr,
  #[napi(value = "astc-6x6-unorm")]
  astc_6x6_unorm,
  #[napi(value = "astc-6x6-unorm-srgb")]
  astc_6x6_unorm_srgb,
  #[napi(value = "astc-6x6-hdr")]
  astc_6x6_hdr,
  #[napi(value = "astc-8x5-unorm")]
  astc_8x5_unorm,
  #[napi(value = "astc-8x5-unorm-srgb")]
  astc_8x5_unorm_srgb,
  #[napi(value = "astc-8x5-hdr")]
  astc_8x5_hdr,
  #[napi(value = "astc-8x6-unorm")]
  astc_8x6_unorm,
  #[napi(value = "astc-8x6-unorm-srgb")]
  astc_8x6_unorm_srgb,
  #[napi(value = "astc-8x6-hdr")]
  astc_8x6_hdr,
  #[napi(value = "astc-8x8-unorm")]
  astc_8x8_unorm,
  #[napi(value = "astc-8x8-unorm-srgb")]
  astc_8x8_unorm_srgb,
  #[napi(value = "astc-8x8-hdr")]
  astc_8x8_hdr,
  #[napi(value = "astc-10x5-unorm")]
  astc_10x5_unorm,
  #[napi(value = "astc-10x5-unorm-srgb")]
  astc_10x5_unorm_srgb,
  #[napi(value = "astc-10x5-hdr")]
  astc_10x5_hdr,
  #[napi(value = "astc-10x6-unorm")]
  astc_10x6_unorm,
  #[napi(value = "astc-10x6-unorm-srgb")]
  astc_10x6_unorm_srgb,
  #[napi(value = "astc-10x6-hdr")]
  astc_10x6_hdr,
  #[napi(value = "astc-10x8-unorm")]
  astc_10x8_unorm,
  #[napi(value = "astc-10x8-unorm-srgb")]
  astc_10x8_unorm_srgb,
  #[napi(value = "astc-10x8-hdr")]
  astc_10x8_hdr,
  #[napi(value = "astc-10x10-unorm")]
  astc_10x10_unorm,
  #[napi(value = "astc-10x10-unorm-srgb")]
  astc_10x10_unorm_srgb,
  #[napi(value = "astc-10x10-hdr")]
  astc_10x10_hdr,
  #[napi(value = "astc-12x10-unorm")]
  astc_12x10_unorm,
  #[napi(value = "astc-12x10-unorm-srgb")]
  astc_12x10_unorm_srgb,
  #[napi(value = "astc-12x10-hdr")]
  astc_12x10_hdr,
  #[napi(value = "astc-12x12-unorm")]
  astc_12x12_unorm,
  #[napi(value = "astc-12x12-unorm-srgb")]
  astc_12x12_unorm_srgb,
  #[napi(value = "astc-12x12-hdr")]
  astc_12x12_hdr,
}

impl Into<CanvasGPUTextureFormat> for GPUTextureFormat {
  fn into(self) -> CanvasGPUTextureFormat {
    match self {
      GPUTextureFormat::r8unorm => CanvasGPUTextureFormat::R8Unorm,
      GPUTextureFormat::r8snorm => CanvasGPUTextureFormat::R8Snorm,
      GPUTextureFormat::r8uint => CanvasGPUTextureFormat::R8Uint,
      GPUTextureFormat::r8sint => CanvasGPUTextureFormat::R8Sint,
      GPUTextureFormat::r16uint => CanvasGPUTextureFormat::R16Uint,
      GPUTextureFormat::r16sint => CanvasGPUTextureFormat::R16Sint,
      GPUTextureFormat::r16float => CanvasGPUTextureFormat::R16Float,
      GPUTextureFormat::rg8unorm => CanvasGPUTextureFormat::Rg8Unorm,
      GPUTextureFormat::rg8snorm => CanvasGPUTextureFormat::Rg8Snorm,
      GPUTextureFormat::rg8uint => CanvasGPUTextureFormat::Rg8Uint,
      GPUTextureFormat::rg8sint => CanvasGPUTextureFormat::Rg8Sint,
      GPUTextureFormat::r32uint => CanvasGPUTextureFormat::R32Uint,
      GPUTextureFormat::r32sint => CanvasGPUTextureFormat::R32Sint,
      GPUTextureFormat::r32float => CanvasGPUTextureFormat::R32Float,
      GPUTextureFormat::rg16uint => CanvasGPUTextureFormat::Rg16Uint,
      GPUTextureFormat::rg16sint => CanvasGPUTextureFormat::Rg16Sint,
      GPUTextureFormat::r16snorm => CanvasGPUTextureFormat::R16Snorm,
      GPUTextureFormat::rg16float => CanvasGPUTextureFormat::Rg16Float,
      GPUTextureFormat::rgba8unorm => CanvasGPUTextureFormat::Rgba8Unorm,
      GPUTextureFormat::rgba8unorm_srgb => CanvasGPUTextureFormat::Rgba8UnormSrgb,
      GPUTextureFormat::rgba8snorm => CanvasGPUTextureFormat::Rgba8Snorm,
      GPUTextureFormat::rgba8uint => CanvasGPUTextureFormat::Rgba8Uint,
      GPUTextureFormat::rgba8sint => CanvasGPUTextureFormat::Rgba8Sint,
      GPUTextureFormat::bgra8unorm => CanvasGPUTextureFormat::Bgra8Unorm,
      GPUTextureFormat::bgra8unorm_srgb => CanvasGPUTextureFormat::Bgra8UnormSrgb,
      GPUTextureFormat::rgb9e5ufloat => CanvasGPUTextureFormat::Rgb9e5Ufloat,
      GPUTextureFormat::rgb10a2uint => CanvasGPUTextureFormat::Rgb10a2Uint,
      GPUTextureFormat::rgb10a2unorm => CanvasGPUTextureFormat::Rgb10a2Unorm,
      GPUTextureFormat::rg11b10ufloat => CanvasGPUTextureFormat::Rg11b10UFloat,
      GPUTextureFormat::rg32uint => CanvasGPUTextureFormat::Rg32Uint,
      GPUTextureFormat::rg32sint => CanvasGPUTextureFormat::Rg32Sint,
      GPUTextureFormat::rg32float => CanvasGPUTextureFormat::Rg32Float,
      GPUTextureFormat::rgba16uint => CanvasGPUTextureFormat::Rgba16Uint,
      GPUTextureFormat::rgba16sint => CanvasGPUTextureFormat::Rgba16Sint,
      GPUTextureFormat::rgba16float => CanvasGPUTextureFormat::Rgba16Float,
      GPUTextureFormat::rgba32uint => CanvasGPUTextureFormat::Rgba32Uint,
      GPUTextureFormat::rgba32sint => CanvasGPUTextureFormat::Rgba32Sint,
      GPUTextureFormat::rgba32float => CanvasGPUTextureFormat::Rgba32Float,
      GPUTextureFormat::r16unorm => CanvasGPUTextureFormat::R16Unorm,
      GPUTextureFormat::rg16snorm => CanvasGPUTextureFormat::Rg16Snorm,
      GPUTextureFormat::rg16unorm => CanvasGPUTextureFormat::Rg16Unorm,
      GPUTextureFormat::rgba16unorm => CanvasGPUTextureFormat::Rgba16Unorm,
      GPUTextureFormat::rgba16snorm => CanvasGPUTextureFormat::Rgba16Snorm,
      GPUTextureFormat::nv12 => CanvasGPUTextureFormat::NV12,
      GPUTextureFormat::stencil8 => CanvasGPUTextureFormat::Stencil8,
      GPUTextureFormat::depth16unorm => CanvasGPUTextureFormat::Depth16Unorm,
      GPUTextureFormat::depth24plus => CanvasGPUTextureFormat::Depth24Plus,
      GPUTextureFormat::depth24plus_stencil8 => CanvasGPUTextureFormat::Depth24PlusStencil8,
      GPUTextureFormat::depth32float => CanvasGPUTextureFormat::Depth32Float,
      GPUTextureFormat::depth32float_stencil8 => CanvasGPUTextureFormat::Depth32FloatStencil8,
      GPUTextureFormat::bc1_rgba_unorm => CanvasGPUTextureFormat::Bc1RgbaUnorm,
      GPUTextureFormat::bc1_rgba_unorm_srgb => CanvasGPUTextureFormat::Bc1RgbaUnormSrgb,
      GPUTextureFormat::bc2_rgba_unorm => CanvasGPUTextureFormat::Bc2RgbaUnorm,
      GPUTextureFormat::bc2_rgba_unorm_srgb => CanvasGPUTextureFormat::Bc2RgbaUnormSrgb,
      GPUTextureFormat::bc3_rgba_unorm => CanvasGPUTextureFormat::Bc3RgbaUnorm,
      GPUTextureFormat::bc3_rgba_unorm_srgb => CanvasGPUTextureFormat::Bc3RgbaUnormSrgb,
      GPUTextureFormat::bc4_r_unorm => CanvasGPUTextureFormat::Bc4RUnorm,
      GPUTextureFormat::bc4_r_snorm => CanvasGPUTextureFormat::Bc4RSnorm,
      GPUTextureFormat::bc5_rg_unorm => CanvasGPUTextureFormat::Bc5RgUnorm,
      GPUTextureFormat::bc5_rg_snorm => CanvasGPUTextureFormat::Bc5RgSnorm,
      GPUTextureFormat::bc6h_rgb_ufloat => CanvasGPUTextureFormat::Bc6hRgbUfloat,
      GPUTextureFormat::bc6h_rgb_float => CanvasGPUTextureFormat::Bc6hRgbFloat,
      GPUTextureFormat::bc7_rgba_unorm => CanvasGPUTextureFormat::Bc7RgbaUnorm,
      GPUTextureFormat::bc7_rgba_unorm_srgb => CanvasGPUTextureFormat::Bc7RgbaUnormSrgb,
      GPUTextureFormat::etc2_rgb8unorm => CanvasGPUTextureFormat::Etc2Rgb8Unorm,
      GPUTextureFormat::etc2_rgb8unorm_srgb => CanvasGPUTextureFormat::Etc2Rgb8UnormSrgb,
      GPUTextureFormat::etc2_rgb8a1unorm => CanvasGPUTextureFormat::Etc2Rgb8A1Unorm,
      GPUTextureFormat::etc2_rgb8a1unorm_srgb => CanvasGPUTextureFormat::Etc2Rgb8A1UnormSrgb,
      GPUTextureFormat::etc2_rgba8unorm => CanvasGPUTextureFormat::Etc2Rgba8Unorm,
      GPUTextureFormat::etc2_rgba8unorm_srgb => CanvasGPUTextureFormat::Etc2Rgba8UnormSrgb,
      GPUTextureFormat::eac_r11unorm => CanvasGPUTextureFormat::EacR11Unorm,
      GPUTextureFormat::eac_r11snorm => CanvasGPUTextureFormat::EacR11Snorm,
      GPUTextureFormat::eac_rg11unorm => CanvasGPUTextureFormat::EacRg11Unorm,
      GPUTextureFormat::eac_rg11snorm => CanvasGPUTextureFormat::EacRg11Snorm,
      GPUTextureFormat::astc_4x4_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B4x4,
      },
      GPUTextureFormat::astc_4x4_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B4x4,
      },
      GPUTextureFormat::astc_4x4_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B4x4,
      },
      GPUTextureFormat::astc_5x4_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B5x4,
      },
      GPUTextureFormat::astc_5x4_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B5x4,
      },
      GPUTextureFormat::astc_5x4_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B5x4,
      },
      GPUTextureFormat::astc_5x5_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B5x5,
      },
      GPUTextureFormat::astc_5x5_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B5x5,
      },
      GPUTextureFormat::astc_5x5_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B5x5,
      },
      GPUTextureFormat::astc_6x5_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B6x5,
      },
      GPUTextureFormat::astc_6x5_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B6x5,
      },
      GPUTextureFormat::astc_6x5_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B6x5,
      },
      GPUTextureFormat::astc_6x6_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B6x6,
      },
      GPUTextureFormat::astc_6x6_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B6x6,
      },
      GPUTextureFormat::astc_6x6_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B6x6,
      },
      GPUTextureFormat::astc_8x5_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B8x5,
      },
      GPUTextureFormat::astc_8x5_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B8x5,
      },
      GPUTextureFormat::astc_8x5_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B8x5,
      },
      GPUTextureFormat::astc_8x6_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B8x6,
      },
      GPUTextureFormat::astc_8x6_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B8x6,
      },
      GPUTextureFormat::astc_8x6_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B8x6,
      },
      GPUTextureFormat::astc_8x8_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B8x8,
      },
      GPUTextureFormat::astc_8x8_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B8x8,
      },
      GPUTextureFormat::astc_8x8_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B8x8,
      },
      GPUTextureFormat::astc_10x5_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B10x5,
      },
      GPUTextureFormat::astc_10x5_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B10x5,
      },
      GPUTextureFormat::astc_10x5_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B10x5,
      },
      GPUTextureFormat::astc_10x6_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B10x6,
      },
      GPUTextureFormat::astc_10x6_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B10x6,
      },
      GPUTextureFormat::astc_10x6_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B10x6,
      },
      GPUTextureFormat::astc_10x8_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B10x8,
      },
      GPUTextureFormat::astc_10x8_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B10x8,
      },
      GPUTextureFormat::astc_10x8_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B10x8,
      },
      GPUTextureFormat::astc_10x10_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B10x10,
      },
      GPUTextureFormat::astc_10x10_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B10x10,
      },
      GPUTextureFormat::astc_10x10_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B10x10,
      },
      GPUTextureFormat::astc_12x10_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B12x10,
      },
      GPUTextureFormat::astc_12x10_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B12x10,
      },
      GPUTextureFormat::astc_12x10_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B12x10,
      },
      GPUTextureFormat::astc_12x12_unorm => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Unorm,
        block: CanvasAstcBlock::B12x12,
      },
      GPUTextureFormat::astc_12x12_unorm_srgb => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::UnormSrgb,
        block: CanvasAstcBlock::B12x12,
      },
      GPUTextureFormat::astc_12x12_hdr => CanvasGPUTextureFormat::Astc {
        channel: CanvasAstcChannel::Hdr,
        block: CanvasAstcBlock::B12x12,
      },
    }
  }
}

impl From<CanvasGPUTextureFormat> for GPUTextureFormat {
  fn from(value: CanvasGPUTextureFormat) -> GPUTextureFormat {
    match value {
      CanvasGPUTextureFormat::R8Unorm => GPUTextureFormat::r8unorm,
      CanvasGPUTextureFormat::R8Snorm => GPUTextureFormat::r8snorm,
      CanvasGPUTextureFormat::R8Uint => GPUTextureFormat::r8uint,
      CanvasGPUTextureFormat::R8Sint => GPUTextureFormat::r8sint,
      CanvasGPUTextureFormat::R16Uint => GPUTextureFormat::r16uint,
      CanvasGPUTextureFormat::R16Sint => GPUTextureFormat::r16sint,
      CanvasGPUTextureFormat::R16Float => GPUTextureFormat::r16float,
      CanvasGPUTextureFormat::Rg8Unorm => GPUTextureFormat::rg8unorm,
      CanvasGPUTextureFormat::Rg8Snorm => GPUTextureFormat::rg8snorm,
      CanvasGPUTextureFormat::Rg8Uint => GPUTextureFormat::rg8uint,
      CanvasGPUTextureFormat::Rg8Sint => GPUTextureFormat::rg8sint,
      CanvasGPUTextureFormat::R32Uint => GPUTextureFormat::r32uint,
      CanvasGPUTextureFormat::R32Sint => GPUTextureFormat::r32sint,
      CanvasGPUTextureFormat::R32Float => GPUTextureFormat::r32float,
      CanvasGPUTextureFormat::Rg16Uint => GPUTextureFormat::rg16uint,
      CanvasGPUTextureFormat::Rg16Sint => GPUTextureFormat::rg16sint,
      CanvasGPUTextureFormat::Rg16Float => GPUTextureFormat::rg16float,
      CanvasGPUTextureFormat::Rgba8Unorm => GPUTextureFormat::rgba8unorm,
      CanvasGPUTextureFormat::Rgba8UnormSrgb => GPUTextureFormat::rgba8unorm_srgb,
      CanvasGPUTextureFormat::Rgba8Snorm => GPUTextureFormat::rgba8snorm,
      CanvasGPUTextureFormat::Rgba8Uint => GPUTextureFormat::rgba8uint,
      CanvasGPUTextureFormat::Rgba8Sint => GPUTextureFormat::rgba8sint,
      CanvasGPUTextureFormat::Bgra8Unorm => GPUTextureFormat::bgra8unorm,
      CanvasGPUTextureFormat::Bgra8UnormSrgb => GPUTextureFormat::bgra8unorm_srgb,
      CanvasGPUTextureFormat::Rgb9e5Ufloat => GPUTextureFormat::rgb9e5ufloat,
      CanvasGPUTextureFormat::Rgb10a2Uint => GPUTextureFormat::rgb10a2uint,
      CanvasGPUTextureFormat::Rgb10a2Unorm => GPUTextureFormat::rgb10a2unorm,
      CanvasGPUTextureFormat::Rg11b10UFloat => GPUTextureFormat::rg11b10ufloat,
      CanvasGPUTextureFormat::Rg32Uint => GPUTextureFormat::rg32uint,
      CanvasGPUTextureFormat::Rg32Sint => GPUTextureFormat::rg32sint,
      CanvasGPUTextureFormat::Rg32Float => GPUTextureFormat::rg32float,
      CanvasGPUTextureFormat::Rgba16Uint => GPUTextureFormat::rgba16uint,
      CanvasGPUTextureFormat::Rgba16Sint => GPUTextureFormat::rgba16sint,
      CanvasGPUTextureFormat::Rgba16Float => GPUTextureFormat::rgba16float,
      CanvasGPUTextureFormat::Rgba32Uint => GPUTextureFormat::rgba32uint,
      CanvasGPUTextureFormat::Rgba32Sint => GPUTextureFormat::rgba32sint,
      CanvasGPUTextureFormat::Rgba32Float => GPUTextureFormat::rgba32float,
      CanvasGPUTextureFormat::R16Unorm => GPUTextureFormat::r16unorm,
      CanvasGPUTextureFormat::R16Snorm => GPUTextureFormat::r16snorm,
      CanvasGPUTextureFormat::Rg16Unorm => GPUTextureFormat::rg16unorm,
      CanvasGPUTextureFormat::Rg16Snorm => GPUTextureFormat::rg16snorm,
      CanvasGPUTextureFormat::Rgba16Unorm => GPUTextureFormat::rgba16unorm,
      CanvasGPUTextureFormat::Rgba16Snorm => GPUTextureFormat::rgba16snorm,
      CanvasGPUTextureFormat::NV12 => GPUTextureFormat::nv12,
      CanvasGPUTextureFormat::Stencil8 => GPUTextureFormat::stencil8,
      CanvasGPUTextureFormat::Depth16Unorm => GPUTextureFormat::depth16unorm,
      CanvasGPUTextureFormat::Depth24Plus => GPUTextureFormat::depth24plus,
      CanvasGPUTextureFormat::Depth24PlusStencil8 => GPUTextureFormat::depth24plus_stencil8,
      CanvasGPUTextureFormat::Depth32Float => GPUTextureFormat::depth32float,
      CanvasGPUTextureFormat::Depth32FloatStencil8 => GPUTextureFormat::depth32float_stencil8,
      CanvasGPUTextureFormat::Bc1RgbaUnorm => GPUTextureFormat::bc1_rgba_unorm,
      CanvasGPUTextureFormat::Bc1RgbaUnormSrgb => GPUTextureFormat::bc1_rgba_unorm_srgb,
      CanvasGPUTextureFormat::Bc2RgbaUnorm => GPUTextureFormat::bc2_rgba_unorm,
      CanvasGPUTextureFormat::Bc2RgbaUnormSrgb => GPUTextureFormat::bc2_rgba_unorm_srgb,
      CanvasGPUTextureFormat::Bc3RgbaUnorm => GPUTextureFormat::bc3_rgba_unorm,
      CanvasGPUTextureFormat::Bc3RgbaUnormSrgb => GPUTextureFormat::bc3_rgba_unorm_srgb,
      CanvasGPUTextureFormat::Bc4RUnorm => GPUTextureFormat::bc4_r_unorm,
      CanvasGPUTextureFormat::Bc4RSnorm => GPUTextureFormat::bc4_r_snorm,
      CanvasGPUTextureFormat::Bc5RgUnorm => GPUTextureFormat::bc5_rg_unorm,
      CanvasGPUTextureFormat::Bc5RgSnorm => GPUTextureFormat::bc5_rg_snorm,
      CanvasGPUTextureFormat::Bc6hRgbUfloat => GPUTextureFormat::bc6h_rgb_ufloat,
      CanvasGPUTextureFormat::Bc6hRgbFloat => GPUTextureFormat::bc6h_rgb_float,
      CanvasGPUTextureFormat::Bc7RgbaUnorm => GPUTextureFormat::bc7_rgba_unorm,
      CanvasGPUTextureFormat::Bc7RgbaUnormSrgb => GPUTextureFormat::bc7_rgba_unorm_srgb,
      CanvasGPUTextureFormat::Etc2Rgb8Unorm => GPUTextureFormat::etc2_rgb8unorm,
      CanvasGPUTextureFormat::Etc2Rgb8UnormSrgb => GPUTextureFormat::etc2_rgb8unorm_srgb,
      CanvasGPUTextureFormat::Etc2Rgb8A1Unorm => GPUTextureFormat::etc2_rgb8a1unorm,
      CanvasGPUTextureFormat::Etc2Rgb8A1UnormSrgb => GPUTextureFormat::etc2_rgb8a1unorm_srgb,
      CanvasGPUTextureFormat::Etc2Rgba8Unorm => GPUTextureFormat::etc2_rgba8unorm,
      CanvasGPUTextureFormat::Etc2Rgba8UnormSrgb => GPUTextureFormat::etc2_rgba8unorm_srgb,
      CanvasGPUTextureFormat::EacR11Unorm => GPUTextureFormat::eac_r11unorm,
      CanvasGPUTextureFormat::EacR11Snorm => GPUTextureFormat::eac_r11snorm,
      CanvasGPUTextureFormat::EacRg11Unorm => GPUTextureFormat::eac_rg11unorm,
      CanvasGPUTextureFormat::EacRg11Snorm => GPUTextureFormat::eac_rg11snorm,
      CanvasGPUTextureFormat::Astc { channel, block } => match (channel, block) {
        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B4x4) => GPUTextureFormat::astc_4x4_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B4x4) => {
          GPUTextureFormat::astc_4x4_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B4x4) => GPUTextureFormat::astc_4x4_hdr,
        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B5x4) => GPUTextureFormat::astc_5x4_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B5x4) => {
          GPUTextureFormat::astc_5x4_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B5x4) => GPUTextureFormat::astc_5x4_hdr,
        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B5x5) => GPUTextureFormat::astc_5x5_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B5x5) => {
          GPUTextureFormat::astc_5x5_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B5x5) => GPUTextureFormat::astc_5x5_hdr,

        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B6x5) => GPUTextureFormat::astc_6x5_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B6x5) => {
          GPUTextureFormat::astc_6x5_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B6x5) => GPUTextureFormat::astc_6x5_hdr,

        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B6x6) => GPUTextureFormat::astc_6x6_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B6x6) => {
          GPUTextureFormat::astc_6x6_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B6x6) => GPUTextureFormat::astc_6x6_hdr,

        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B8x5) => GPUTextureFormat::astc_8x5_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B8x5) => {
          GPUTextureFormat::astc_8x5_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B8x5) => GPUTextureFormat::astc_8x5_hdr,

        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B8x6) => GPUTextureFormat::astc_8x6_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B8x6) => {
          GPUTextureFormat::astc_8x6_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B8x6) => GPUTextureFormat::astc_8x6_hdr,

        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B8x8) => GPUTextureFormat::astc_8x8_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B8x8) => {
          GPUTextureFormat::astc_8x8_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B8x8) => GPUTextureFormat::astc_8x8_hdr,

        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B10x5) => GPUTextureFormat::astc_10x5_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B10x5) => {
          GPUTextureFormat::astc_10x5_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B10x5) => GPUTextureFormat::astc_10x5_hdr,

        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B10x6) => GPUTextureFormat::astc_10x6_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B10x6) => {
          GPUTextureFormat::astc_10x6_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B10x6) => GPUTextureFormat::astc_10x6_hdr,

        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B10x8) => GPUTextureFormat::astc_10x8_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B10x8) => {
          GPUTextureFormat::astc_10x8_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B10x8) => GPUTextureFormat::astc_10x8_hdr,

        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B10x10) => GPUTextureFormat::astc_10x10_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B10x10) => {
          GPUTextureFormat::astc_10x10_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B10x10) => GPUTextureFormat::astc_10x10_hdr,

        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B12x10) => GPUTextureFormat::astc_12x10_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B12x10) => {
          GPUTextureFormat::astc_12x10_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B12x10) => GPUTextureFormat::astc_12x10_hdr,

        (CanvasAstcChannel::Unorm, CanvasAstcBlock::B12x12) => GPUTextureFormat::astc_12x12_unorm,
        (CanvasAstcChannel::UnormSrgb, CanvasAstcBlock::B12x12) => {
          GPUTextureFormat::astc_12x12_unorm_srgb
        }
        (CanvasAstcChannel::Hdr, CanvasAstcBlock::B12x12) => GPUTextureFormat::astc_12x12_hdr,
      },
    }
  }
}