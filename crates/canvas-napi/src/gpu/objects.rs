use crate::c2d::image_data::ImageData;
use crate::c2d::CanvasRenderingContext2D;
use crate::gl::{web_g_l_rendering_context, HTMLCanvasSource, HTMLImageSource};
use crate::gl2::web_g_l_2_rendering_context;
use crate::gpu::bind_group_layout::g_p_u_bind_group_layout;
use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::context::g_p_u_canvas_context;
use crate::gpu::enums::{
  GPUAddressMode, GPUBlendFactor, GPUBlendOperation, GPUBufferBindingType, GPUCompareFunction,
  GPUCullMode, GPUFilterMode, GPUFrontFace, GPUIndexFormat, GPULoadOp, GPUPipelineLayoutAuto,
  GPUPrimitiveTopology, GPUQueryType, GPUSamplerBindingType, GPUStencilOperation,
  GPUStorageTextureAccess, GPUStoreOp, GPUTextureAspect, GPUTextureDimension, GPUTextureFormat,
  GPUTextureSampleType, GPUTextureViewDimension, GPUVertexFormat, GPUVertexStepMode,
  PredefinedColorSpaceEnum,
};
use crate::gpu::pipeline_layout::g_p_u_pipeline_layout;
use crate::gpu::query_set::g_p_u_query_set;
use crate::gpu::sampler::g_p_u_sampler;
use crate::gpu::shader_module::g_p_u_shader_module;
use crate::gpu::texture::g_p_u_texture;
use crate::gpu::texture_view::g_p_u_texture_view;
use crate::image_asset::ImageAsset;
use canvas_c::webgpu::enums::{
  CanvasCullMode, CanvasFrontFace, CanvasGPUTextureFormat, CanvasOptionalIndexFormat,
  CanvasOptionalPrimitiveTopology, CanvasStencilFaceState,
};
use canvas_c::webgpu::gpu_device::{CanvasPrimitiveState, CanvasVertexBufferLayout};
use canvas_c::webgpu::gpu_supported_limits::CanvasGPUSupportedLimits;
use canvas_c::webgpu::structs::{
  CanvasBlendFactor, CanvasBlendOperation, CanvasBlendState, CanvasColor, CanvasColorTargetState,
  CanvasExtent3d, CanvasOrigin3d,
};
use napi::bindgen_prelude::{ClassInstance, Either9, Object};
use napi::Either;
use napi_derive::napi;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::image_bitmap::ImageBitmap;

#[napi(js_name = "GPUImageDataLayout", object)]
pub struct GPUImageDataLayout {
  pub bytes_per_row: i32,
  pub offset: Option<i64>,
  pub rows_per_image: Option<i32>,
}

#[napi(js_name = "GPURenderBundleDescriptor", object)]
pub struct GPURenderBundleDescriptor {
  pub label: Option<String>,
}

#[napi(js_name = "GPURenderBundleEncoderDescriptor", object)]
pub struct GPURenderBundleEncoderDescriptor {
  pub label: Option<String>,
  pub color_formats: Vec<GPUTextureFormat>,
  pub depth_read_only: Option<bool>,
  pub depth_stencil_format: Option<GPUTextureFormat>,
  pub sample_count: Option<u32>,
  pub stencil_read_only: Option<bool>,
}

#[napi(js_name = "GPUPipelineLayoutDescriptor", object)]
pub struct GPUPipelineLayoutDescriptor<'a> {
  pub label: Option<String>,
  pub bind_group_layouts: Vec<ClassInstance<'a, g_p_u_bind_group_layout>>,
}

#[napi(js_name = "GPUQuerySetDescriptor", object)]
pub struct GPUQuerySetDescriptor {
  pub label: Option<String>,
  pub count: u32,
  #[napi(js_name = "type")]
  pub type_: GPUQueryType,
}

#[napi(js_name = "GPUSamplerDescriptor", object)]
pub struct GPUSamplerDescriptor {
  pub label: Option<String>,
  pub address_mode_u: Option<GPUAddressMode>,
  pub address_mode_v: Option<GPUAddressMode>,
  pub address_mode_w: Option<GPUAddressMode>,
  pub compare: Option<GPUCompareFunction>,
  pub lod_max_clamp: Option<f64>,
  pub lod_min_clamp: Option<f64>,
  pub mag_filter: Option<GPUFilterMode>,
  pub max_anisotropy: Option<u16>,
  pub min_filter: Option<GPUFilterMode>,
  pub mipmap_filter: Option<GPUFilterMode>,
}

#[napi(js_name = "GPUOrigin2DDict", object)]
pub struct GPUOrigin2DDict {
  pub x: Option<u32>,
  pub y: Option<u32>,
}

#[napi(js_name = "GPUImageCopyTextureTagged", object)]
pub struct GPUImageCopyTextureTagged<'a> {
  pub aspect: Option<GPUTextureAspect>,
  pub color_space: Option<PredefinedColorSpaceEnum>,
  pub mip_level: Option<u32>,
  pub origin: Option<Either<Vec<u32>, GPUOrigin3DDict>>,
  pub premultiplied_alpha: Option<bool>,
  pub texture: ClassInstance<'a, g_p_u_texture>,
}

#[napi(js_name = "GPUImageCopyExternalImage", object)]
pub struct GPUImageCopyExternalImage<'a> {
  pub flip_y: Option<bool>,
  pub origin: Option<Either<Vec<u32>, GPUOrigin2DDict>>,
  pub source: Either9<
    ClassInstance<'a, ImageData>,
    ClassInstance<'a, ImageAsset>,
    ClassInstance<'a, CanvasRenderingContext2D>,
    ClassInstance<'a, web_g_l_rendering_context>,
    ClassInstance<'a, web_g_l_2_rendering_context>,
    ClassInstance<'a, g_p_u_canvas_context>,
    ClassInstance<'a, ImageBitmap>,
    HTMLImageSource<'a>,
    HTMLCanvasSource<'a>,
  >,
}

#[napi(js_name = "GPUTextureDescriptor", object)]
#[derive(Debug)]
pub struct GPUTextureDescriptor {
  pub label: Option<String>,
  pub dimension: Option<GPUTextureDimension>,
  pub format: GPUTextureFormat,
  pub mip_level_count: Option<u32>,
  pub sample_count: Option<u32>,
  pub size: Either<Vec<u32>, GPUExtent3DDict>,
  pub usage: u32,
  pub view_formats: Option<Vec<GPUTextureFormat>>,
}

#[napi(js_name = "GPUTextureViewDescriptor", object)]
pub struct GPUTextureViewDescriptor {
  pub label: Option<String>,
  pub format: Option<GPUTextureFormat>,
  pub dimension: Option<GPUTextureViewDimension>,
  pub aspect: Option<GPUTextureAspect>,
  pub base_mip_level: Option<u32>,
  pub mip_level_count: Option<u32>,
  pub base_array_layer: Option<u32>,
  pub array_layer_count: Option<u32>,
}

#[napi(js_name = "GPUProgrammableStage", object)]
pub struct GPUProgrammableStage<'a> {
  pub constants: Option<HashMap<String, f64>>,
  pub entry_point: Option<String>,
  pub module: ClassInstance<'a, g_p_u_shader_module>,
}

#[napi(js_name = "GPUComputePipelineDescriptor", object)]
pub struct GPUComputePipelineDescriptor<'a> {
  pub label: Option<String>,
  pub compute: GPUProgrammableStage<'a>,
  pub layout: Either<ClassInstance<'a, g_p_u_pipeline_layout>, GPUPipelineLayoutAuto>,
}

#[napi(js_name = "GPUBufferDescriptor", object)]
pub struct GPUBufferDescriptor {
  pub label: Option<String>,
  pub mapped_at_creation: Option<bool>,
  pub size: i64,
  pub usage: u32,
}

#[napi(object)]
pub struct GPUBufferBinding<'a> {
  pub buffer: ClassInstance<'a, g_p_u_buffer>,
  pub offset: Option<i64>,
  pub size: Option<i64>,
}

impl Debug for GPUBufferBinding<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("GPUBufferBinding")
      .field("buffer", &self.buffer.buffer)
      .field("offset", &self.offset)
      .field("size", &self.size)
      .finish()
  }
}

// pub type GPUBindingResource = Either4<
//   ClassInstance<g_p_u_sampler>,
//   ClassInstance<g_p_u_texture_view>,
//   ClassInstance<GPUBufferBinding>,
//   ClassInstance<g_p_u_external_texture>,
// >;

#[napi(js_name = "GPUTextureBindingLayout", object)]
#[derive(Debug)]
pub struct GPUTextureBindingLayout {
  pub multisampled: Option<bool>,
  pub sample_type: Option<GPUTextureSampleType>,
  pub view_dimension: Option<GPUTextureViewDimension>,
}

#[napi(js_name = "GPUStorageTextureBindingLayout", object)]
#[derive(Debug)]
pub struct GPUStorageTextureBindingLayout {
  pub access: Option<GPUStorageTextureAccess>,
  pub format: GPUTextureFormat,
  pub view_dimension: Option<GPUTextureViewDimension>,
}

#[napi(js_name = "GPUSamplerBindingLayout", object)]
#[derive(Debug)]
pub struct GPUSamplerBindingLayout {
  #[napi(js_name = "type")]
  pub type_: Option<GPUSamplerBindingType>,
}

#[napi(js_name = "GPUExternalTextureBindingLayout", object)]
#[derive(Debug)]
pub struct GPUExternalTextureBindingLayout {}

#[napi(js_name = "GPUBufferBindingLayout", object)]
#[derive(Debug)]
pub struct GPUBufferBindingLayout {
  pub has_dynamic_offset: Option<bool>,
  pub min_binding_size: Option<i64>,
  #[napi(js_name = "type")]
  pub type_: Option<GPUBufferBindingType>,
}

#[napi(js_name = "GPUBindGroupLayoutEntry", object)]
#[derive(Debug)]
pub struct GPUBindGroupLayoutEntry {
  pub label: Option<String>,
  pub binding: u32,
  pub buffer: Option<GPUBufferBindingLayout>,
  pub external_texture: Option<GPUExternalTextureBindingLayout>,
  pub sampler: Option<GPUSamplerBindingLayout>,
  pub storage_texture: Option<GPUStorageTextureBindingLayout>,
  pub texture: Option<GPUTextureBindingLayout>,
  pub visibility: u32,
}

#[napi(js_name = "GPUBindGroupLayoutDescriptor", object)]

pub struct GPUBindGroupLayoutDescriptor {
  pub label: Option<String>,
  pub entries: Vec<GPUBindGroupLayoutEntry>,
}

#[napi(js_name = "GPUBindGroupSampleEntry", object)]
pub struct GPUBindGroupSampleEntry<'a> {
  pub binding: u32,
  pub resource: ClassInstance<'a, g_p_u_sampler>,
}

impl Debug for GPUBindGroupSampleEntry<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("GPUBindGroupSampleEntry")
      .field("binding", &self.binding)
      .field("resource", &self.resource.sampler)
      .finish()
  }
}

#[napi(js_name = "GPUBindGroupTextureViewEntry", object)]
pub struct GPUBindGroupTextureViewEntry<'a> {
  pub binding: u32,
  pub resource: ClassInstance<'a, g_p_u_texture_view>,
}

impl Debug for GPUBindGroupTextureViewEntry<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("GPUBindGroupTextureViewEntry")
      .field("binding", &self.binding)
      .field("resource", &self.resource.texture_view)
      .finish()
  }
}

#[napi(js_name = "GPUBindGroupBufferEntry", object)]
#[derive(Debug)]
pub struct GPUBindGroupBufferEntry<'a> {
  pub binding: u32,
  pub resource: GPUBufferBinding<'a>,
}

#[napi(js_name = "GPUBindGroupEntry", object)]
pub struct GPUBindGroupEntry<'a> {
  pub binding: u32,
  pub resource: Object<'a>,
}

impl Debug for GPUBindGroupEntry<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("GPUBindGroupEntry")
      .field("binding", &self.binding)
      .finish()
  }
}

#[napi(js_name = "GPUBindGroupDescriptor", object)]
pub struct GPUBindGroupDescriptor<'a> {
  pub entries: Vec<GPUBindGroupEntry<'a>>,
  pub label: Option<String>,
  pub layout: ClassInstance<'a, g_p_u_bind_group_layout>,
}

#[napi(js_name = "GPUSupportedLimits", object)]
#[derive(Debug)]
/// Mirrors canvas-c's `CanvasGPUSupportedLimits` (wgpu 30 `Limits`). The 64-bit
/// limits cross the JS boundary as `i64` (a JS number), saturating at `i64::MAX`.
pub struct GPUSupportedLimits {
  #[napi(js_name = "maxTextureDimension1D")]
  pub max_texture_dimension_1d: Option<u32>,
  #[napi(js_name = "maxTextureDimension2D")]
  pub max_texture_dimension_2d: Option<u32>,
  #[napi(js_name = "maxTextureDimension3D")]
  pub max_texture_dimension_3d: Option<u32>,
  pub max_texture_array_layers: Option<u32>,
  pub max_bind_groups: Option<u32>,
  pub max_bindings_per_bind_group: Option<u32>,
  pub max_dynamic_uniform_buffers_per_pipeline_layout: Option<u32>,
  pub max_dynamic_storage_buffers_per_pipeline_layout: Option<u32>,
  pub max_sampled_textures_per_shader_stage: Option<u32>,
  pub max_samplers_per_shader_stage: Option<u32>,
  pub max_storage_buffers_per_shader_stage: Option<u32>,
  pub max_storage_textures_per_shader_stage: Option<u32>,
  pub max_uniform_buffers_per_shader_stage: Option<u32>,
  pub max_binding_array_elements_per_shader_stage: Option<u32>,
  pub max_binding_array_sampler_elements_per_shader_stage: Option<u32>,
  pub max_uniform_buffer_binding_size: Option<i64>,
  pub max_storage_buffer_binding_size: Option<i64>,
  pub max_vertex_buffers: Option<u32>,
  pub max_buffer_size: Option<i64>,
  pub max_vertex_attributes: Option<u32>,
  pub max_vertex_buffer_array_stride: Option<u32>,
  pub max_inter_stage_shader_variables: Option<u32>,
  pub min_uniform_buffer_offset_alignment: Option<u32>,
  pub min_storage_buffer_offset_alignment: Option<u32>,
  pub max_color_attachments: Option<u32>,
  pub max_color_attachment_bytes_per_sample: Option<u32>,
  pub max_compute_workgroup_storage_size: Option<u32>,
  pub max_compute_invocations_per_workgroup: Option<u32>,
  pub max_compute_workgroup_size_x: Option<u32>,
  pub max_compute_workgroup_size_y: Option<u32>,
  pub max_compute_workgroup_size_z: Option<u32>,
  pub max_compute_workgroups_per_dimension: Option<u32>,
  pub max_immediate_size: Option<u32>,
  pub max_non_sampler_bindings: Option<u32>,
  pub max_task_invocations_per_workgroup: Option<u32>,
  pub max_task_invocations_per_dimension: Option<u32>,
  pub max_mesh_invocations_per_workgroup: Option<u32>,
  pub max_mesh_invocations_per_dimension: Option<u32>,
  pub max_task_payload_size: Option<u32>,
  pub max_mesh_output_vertices: Option<u32>,
  pub max_mesh_output_primitives: Option<u32>,
  pub max_mesh_output_layers: Option<u32>,
  pub max_mesh_multiview_view_count: Option<u32>,
  pub max_blas_primitive_count: Option<u32>,
  pub max_blas_geometry_count: Option<u32>,
  pub max_tlas_instance_count: Option<u32>,
  pub max_acceleration_structures_per_shader_stage: Option<u32>,
  pub max_multiview_view_count: Option<u32>,
  pub max_bind_groups_plus_vertex_buffers: Option<u32>,
}

fn limit_to_js(value: u64) -> i64 {
  i64::try_from(value).unwrap_or(i64::MAX)
}

fn limit_from_js(value: i64) -> u64 {
  value.max(0) as u64
}

impl From<CanvasGPUSupportedLimits> for GPUSupportedLimits {
  fn from(value: CanvasGPUSupportedLimits) -> Self {
    Self {
      max_texture_dimension_1d: Some(value.max_texture_dimension_1d),
      max_texture_dimension_2d: Some(value.max_texture_dimension_2d),
      max_texture_dimension_3d: Some(value.max_texture_dimension_3d),
      max_texture_array_layers: Some(value.max_texture_array_layers),
      max_bind_groups: Some(value.max_bind_groups),
      max_bindings_per_bind_group: Some(value.max_bindings_per_bind_group),
      max_dynamic_uniform_buffers_per_pipeline_layout: Some(value.max_dynamic_uniform_buffers_per_pipeline_layout),
      max_dynamic_storage_buffers_per_pipeline_layout: Some(value.max_dynamic_storage_buffers_per_pipeline_layout),
      max_sampled_textures_per_shader_stage: Some(value.max_sampled_textures_per_shader_stage),
      max_samplers_per_shader_stage: Some(value.max_samplers_per_shader_stage),
      max_storage_buffers_per_shader_stage: Some(value.max_storage_buffers_per_shader_stage),
      max_storage_textures_per_shader_stage: Some(value.max_storage_textures_per_shader_stage),
      max_uniform_buffers_per_shader_stage: Some(value.max_uniform_buffers_per_shader_stage),
      max_binding_array_elements_per_shader_stage: Some(value.max_binding_array_elements_per_shader_stage),
      max_binding_array_sampler_elements_per_shader_stage: Some(value.max_binding_array_sampler_elements_per_shader_stage),
      max_uniform_buffer_binding_size: Some(limit_to_js(value.max_uniform_buffer_binding_size)),
      max_storage_buffer_binding_size: Some(limit_to_js(value.max_storage_buffer_binding_size)),
      max_vertex_buffers: Some(value.max_vertex_buffers),
      max_buffer_size: Some(limit_to_js(value.max_buffer_size)),
      max_vertex_attributes: Some(value.max_vertex_attributes),
      max_vertex_buffer_array_stride: Some(value.max_vertex_buffer_array_stride),
      max_inter_stage_shader_variables: Some(value.max_inter_stage_shader_variables),
      min_uniform_buffer_offset_alignment: Some(value.min_uniform_buffer_offset_alignment),
      min_storage_buffer_offset_alignment: Some(value.min_storage_buffer_offset_alignment),
      max_color_attachments: Some(value.max_color_attachments),
      max_color_attachment_bytes_per_sample: Some(value.max_color_attachment_bytes_per_sample),
      max_compute_workgroup_storage_size: Some(value.max_compute_workgroup_storage_size),
      max_compute_invocations_per_workgroup: Some(value.max_compute_invocations_per_workgroup),
      max_compute_workgroup_size_x: Some(value.max_compute_workgroup_size_x),
      max_compute_workgroup_size_y: Some(value.max_compute_workgroup_size_y),
      max_compute_workgroup_size_z: Some(value.max_compute_workgroup_size_z),
      max_compute_workgroups_per_dimension: Some(value.max_compute_workgroups_per_dimension),
      max_immediate_size: Some(value.max_immediate_size),
      max_non_sampler_bindings: Some(value.max_non_sampler_bindings),
      max_task_invocations_per_workgroup: Some(value.max_task_invocations_per_workgroup),
      max_task_invocations_per_dimension: Some(value.max_task_invocations_per_dimension),
      max_mesh_invocations_per_workgroup: Some(value.max_mesh_invocations_per_workgroup),
      max_mesh_invocations_per_dimension: Some(value.max_mesh_invocations_per_dimension),
      max_task_payload_size: Some(value.max_task_payload_size),
      max_mesh_output_vertices: Some(value.max_mesh_output_vertices),
      max_mesh_output_primitives: Some(value.max_mesh_output_primitives),
      max_mesh_output_layers: Some(value.max_mesh_output_layers),
      max_mesh_multiview_view_count: Some(value.max_mesh_multiview_view_count),
      max_blas_primitive_count: Some(value.max_blas_primitive_count),
      max_blas_geometry_count: Some(value.max_blas_geometry_count),
      max_tlas_instance_count: Some(value.max_tlas_instance_count),
      max_acceleration_structures_per_shader_stage: Some(value.max_acceleration_structures_per_shader_stage),
      max_multiview_view_count: Some(value.max_multiview_view_count),
      max_bind_groups_plus_vertex_buffers: Some(value.max_bind_groups_plus_vertex_buffers),
    }
  }
}

impl Into<CanvasGPUSupportedLimits> for GPUSupportedLimits {
  fn into(self) -> CanvasGPUSupportedLimits {
    // Limits left out of `requiredLimits` keep the WebGPU defaults (wgpu `Limits::default()`).
    let defaults = CanvasGPUSupportedLimits::default();
    CanvasGPUSupportedLimits {
      max_texture_dimension_1d: self.max_texture_dimension_1d.unwrap_or(defaults.max_texture_dimension_1d),
      max_texture_dimension_2d: self.max_texture_dimension_2d.unwrap_or(defaults.max_texture_dimension_2d),
      max_texture_dimension_3d: self.max_texture_dimension_3d.unwrap_or(defaults.max_texture_dimension_3d),
      max_texture_array_layers: self.max_texture_array_layers.unwrap_or(defaults.max_texture_array_layers),
      max_bind_groups: self.max_bind_groups.unwrap_or(defaults.max_bind_groups),
      max_bindings_per_bind_group: self.max_bindings_per_bind_group.unwrap_or(defaults.max_bindings_per_bind_group),
      max_dynamic_uniform_buffers_per_pipeline_layout: self.max_dynamic_uniform_buffers_per_pipeline_layout.unwrap_or(defaults.max_dynamic_uniform_buffers_per_pipeline_layout),
      max_dynamic_storage_buffers_per_pipeline_layout: self.max_dynamic_storage_buffers_per_pipeline_layout.unwrap_or(defaults.max_dynamic_storage_buffers_per_pipeline_layout),
      max_sampled_textures_per_shader_stage: self.max_sampled_textures_per_shader_stage.unwrap_or(defaults.max_sampled_textures_per_shader_stage),
      max_samplers_per_shader_stage: self.max_samplers_per_shader_stage.unwrap_or(defaults.max_samplers_per_shader_stage),
      max_storage_buffers_per_shader_stage: self.max_storage_buffers_per_shader_stage.unwrap_or(defaults.max_storage_buffers_per_shader_stage),
      max_storage_textures_per_shader_stage: self.max_storage_textures_per_shader_stage.unwrap_or(defaults.max_storage_textures_per_shader_stage),
      max_uniform_buffers_per_shader_stage: self.max_uniform_buffers_per_shader_stage.unwrap_or(defaults.max_uniform_buffers_per_shader_stage),
      max_binding_array_elements_per_shader_stage: self.max_binding_array_elements_per_shader_stage.unwrap_or(defaults.max_binding_array_elements_per_shader_stage),
      max_binding_array_sampler_elements_per_shader_stage: self.max_binding_array_sampler_elements_per_shader_stage.unwrap_or(defaults.max_binding_array_sampler_elements_per_shader_stage),
      max_uniform_buffer_binding_size: self
        .max_uniform_buffer_binding_size
        .map(limit_from_js)
        .unwrap_or(defaults.max_uniform_buffer_binding_size),
      max_storage_buffer_binding_size: self
        .max_storage_buffer_binding_size
        .map(limit_from_js)
        .unwrap_or(defaults.max_storage_buffer_binding_size),
      max_vertex_buffers: self.max_vertex_buffers.unwrap_or(defaults.max_vertex_buffers),
      max_buffer_size: self
        .max_buffer_size
        .map(limit_from_js)
        .unwrap_or(defaults.max_buffer_size),
      max_vertex_attributes: self.max_vertex_attributes.unwrap_or(defaults.max_vertex_attributes),
      max_vertex_buffer_array_stride: self.max_vertex_buffer_array_stride.unwrap_or(defaults.max_vertex_buffer_array_stride),
      max_inter_stage_shader_variables: self.max_inter_stage_shader_variables.unwrap_or(defaults.max_inter_stage_shader_variables),
      min_uniform_buffer_offset_alignment: self.min_uniform_buffer_offset_alignment.unwrap_or(defaults.min_uniform_buffer_offset_alignment),
      min_storage_buffer_offset_alignment: self.min_storage_buffer_offset_alignment.unwrap_or(defaults.min_storage_buffer_offset_alignment),
      max_color_attachments: self.max_color_attachments.unwrap_or(defaults.max_color_attachments),
      max_color_attachment_bytes_per_sample: self.max_color_attachment_bytes_per_sample.unwrap_or(defaults.max_color_attachment_bytes_per_sample),
      max_compute_workgroup_storage_size: self.max_compute_workgroup_storage_size.unwrap_or(defaults.max_compute_workgroup_storage_size),
      max_compute_invocations_per_workgroup: self.max_compute_invocations_per_workgroup.unwrap_or(defaults.max_compute_invocations_per_workgroup),
      max_compute_workgroup_size_x: self.max_compute_workgroup_size_x.unwrap_or(defaults.max_compute_workgroup_size_x),
      max_compute_workgroup_size_y: self.max_compute_workgroup_size_y.unwrap_or(defaults.max_compute_workgroup_size_y),
      max_compute_workgroup_size_z: self.max_compute_workgroup_size_z.unwrap_or(defaults.max_compute_workgroup_size_z),
      max_compute_workgroups_per_dimension: self.max_compute_workgroups_per_dimension.unwrap_or(defaults.max_compute_workgroups_per_dimension),
      max_immediate_size: self.max_immediate_size.unwrap_or(defaults.max_immediate_size),
      max_non_sampler_bindings: self.max_non_sampler_bindings.unwrap_or(defaults.max_non_sampler_bindings),
      max_task_invocations_per_workgroup: self.max_task_invocations_per_workgroup.unwrap_or(defaults.max_task_invocations_per_workgroup),
      max_task_invocations_per_dimension: self.max_task_invocations_per_dimension.unwrap_or(defaults.max_task_invocations_per_dimension),
      max_mesh_invocations_per_workgroup: self.max_mesh_invocations_per_workgroup.unwrap_or(defaults.max_mesh_invocations_per_workgroup),
      max_mesh_invocations_per_dimension: self.max_mesh_invocations_per_dimension.unwrap_or(defaults.max_mesh_invocations_per_dimension),
      max_task_payload_size: self.max_task_payload_size.unwrap_or(defaults.max_task_payload_size),
      max_mesh_output_vertices: self.max_mesh_output_vertices.unwrap_or(defaults.max_mesh_output_vertices),
      max_mesh_output_primitives: self.max_mesh_output_primitives.unwrap_or(defaults.max_mesh_output_primitives),
      max_mesh_output_layers: self.max_mesh_output_layers.unwrap_or(defaults.max_mesh_output_layers),
      max_mesh_multiview_view_count: self.max_mesh_multiview_view_count.unwrap_or(defaults.max_mesh_multiview_view_count),
      max_blas_primitive_count: self.max_blas_primitive_count.unwrap_or(defaults.max_blas_primitive_count),
      max_blas_geometry_count: self.max_blas_geometry_count.unwrap_or(defaults.max_blas_geometry_count),
      max_tlas_instance_count: self.max_tlas_instance_count.unwrap_or(defaults.max_tlas_instance_count),
      max_acceleration_structures_per_shader_stage: self.max_acceleration_structures_per_shader_stage.unwrap_or(defaults.max_acceleration_structures_per_shader_stage),
      max_multiview_view_count: self.max_multiview_view_count.unwrap_or(defaults.max_multiview_view_count),
      max_bind_groups_plus_vertex_buffers: self.max_bind_groups_plus_vertex_buffers.unwrap_or(defaults.max_bind_groups_plus_vertex_buffers),
    }
  }
}

impl Default for GPUSupportedLimits {
  fn default() -> Self {
    Self::from(CanvasGPUSupportedLimits::default())
  }
}

#[napi(object)]
pub struct GPUImageCopyBuffer<'a> {
  pub buffer: ClassInstance<'a, g_p_u_buffer>,
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
pub struct GPUImageCopyTexture<'a> {
  pub aspect: Option<GPUTextureAspect>,
  pub mip_level: Option<u32>,
  pub origin: Option<Either<GPUOrigin3DDict, Vec<u32>>>,
  pub texture: ClassInstance<'a, g_p_u_texture>,
}

#[napi(object)]
pub struct GPUComputePassTimestampWrites<'a> {
  pub beginning_of_pass_write_index: i32,
  pub end_of_pass_write_index: i32,
  pub query_set: ClassInstance<'a, g_p_u_query_set>,
}

#[napi(object)]
pub struct GPUComputePassDescriptor<'a> {
  pub label: Option<String>,
  pub timestamp_writes: Option<GPUComputePassTimestampWrites<'a>>,
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

#[napi(js_name = "GPURenderPassColorAttachment", object)]
pub struct GPURenderPassColorAttachment<'a> {
  pub clear_value: Option<Either<Vec<f64>, GPUColorDict>>,
  pub depth_slice: Option<u32>,
  #[napi(js_name = "loadOp")]
  pub load_op: GPULoadOp,
  pub resolve_target: Option<ClassInstance<'a, g_p_u_texture_view>>,
  #[napi(js_name = "storeOp")]
  pub store_op: GPUStoreOp,
  pub view: ClassInstance<'a, g_p_u_texture_view>,
}

#[napi(object)]
pub struct GPURenderPassDepthStencilAttachment<'a> {
  /// f64 on the JS boundary (napi-rs 3 has no `f32` from-JS conversion); narrowed at use.
  pub depth_clear_value: Option<f64>,
  pub depth_load_op: Option<GPULoadOp>,
  pub depth_read_only: Option<bool>,
  pub depth_store_op: Option<GPUStoreOp>,
  pub stencil_clear_value: Option<u32>,
  pub stencil_load_op: Option<GPULoadOp>,
  pub stencil_read_only: Option<bool>,
  pub stencil_store_op: Option<GPUStoreOp>,
  pub view: ClassInstance<'a, g_p_u_texture_view>,
}

#[napi(object)]
pub struct GPURenderPassTimestampWrites<'a> {
  pub beginning_of_pass_write_index: Option<i32>,
  pub end_of_pass_write_index: Option<i32>,
  pub query_set: ClassInstance<'a, g_p_u_query_set>,
}

#[napi(object)]
pub struct GPURenderPassDescriptor<'a> {
  pub color_attachments: Vec<GPURenderPassColorAttachment<'a>>,
  pub depth_stencil_attachment: Option<GPURenderPassDepthStencilAttachment<'a>>,
  pub label: Option<String>,
  pub max_draw_count: Option<i64>,
  pub occlusion_query_set: Option<ClassInstance<'a, g_p_u_query_set>>,
  pub timestamp_writes: Option<GPURenderPassTimestampWrites<'a>>,
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
#[derive(Debug)]
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

impl Into<CanvasStencilFaceState> for GPUStencilFaceState {
  fn into(self) -> CanvasStencilFaceState {
    let mut ret = CanvasStencilFaceState::IGNORE;
    if let Some(compare) = self.compare {
      ret.compare = compare.into();
    }

    if let Some(depth_fail_op) = self.depth_fail_op {
      ret.depth_fail_op = depth_fail_op.into();
    }

    if let Some(fail_op) = self.fail_op {
      ret.fail_op = fail_op.into();
    }

    if let Some(pass_op) = self.pass_op {
      ret.pass_op = pass_op.into();
    }

    ret
  }
}

#[napi(object)]
pub struct GPUDepthStencilState {
  pub depth_bias: Option<i32>,
  pub depth_bias_clamp: Option<f64>,
  pub depth_bias_slope_scale: Option<f64>,
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
pub struct GPUFragmentState<'a> {
  pub constants: Option<HashMap<String, f64>>,
  pub entry_point: Option<String>,
  pub module: ClassInstance<'a, g_p_u_shader_module>,
  pub targets: Vec<GPUColorTargetState>,
}

#[napi(object)]
pub struct GPUMultisampleState {
  pub alpha_to_coverage_enabled: Option<bool>,
  pub count: Option<u32>,
  pub mask: Option<i64>,
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
pub struct GPUVertexState<'a> {
  pub buffers: Option<Vec<GPUVertexBufferLayout>>,
  pub constants: Option<HashMap<String, f64>>,
  pub entry_point: Option<String>,
  pub module: ClassInstance<'a, g_p_u_shader_module>,
}

#[napi(object)]
pub struct GPURenderPipelineDescriptor<'a> {
  pub depth_stencil: Option<GPUDepthStencilState>,
  pub fragment: Option<GPUFragmentState<'a>>,
  pub label: Option<String>,
  pub layout: Option<Either<Object<'a>, GPUPipelineLayoutAuto>>,
  pub multisample: Option<GPUMultisampleState>,
  pub primitive: Option<GPUPrimitiveState>,
  pub vertex: GPUVertexState<'a>,
}
