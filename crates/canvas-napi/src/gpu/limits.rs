use canvas_c::webgpu::gpu_supported_limits::CanvasGPUSupportedLimits;
use napi::bindgen_prelude::Unknown;
use napi_derive::napi;

use crate::gpu::parse::{as_number, downcast, number};

/// `GPUSupportedLimits`: what adapter and device `limits` return, and what `requestDevice`
/// takes as `requiredLimits` (`new CanvasModule.GPUSupportedLimits()` starts from the WebGPU
/// defaults and `packages/canvas` assigns the limits it wants). Every limit is an accessor, as in
/// the V8 bindings; assigning a non-number is ignored.
#[napi(js_name = "GPUSupportedLimits")]
pub struct g_p_u_supported_limits {
  pub(crate) limits: CanvasGPUSupportedLimits,
}

impl From<CanvasGPUSupportedLimits> for g_p_u_supported_limits {
  fn from(limits: CanvasGPUSupportedLimits) -> Self {
    Self { limits }
  }
}

trait Limit: Copy {
  fn to_js(self) -> f64;
  fn from_js(value: f64) -> Self;
}

impl Limit for u32 {
  fn to_js(self) -> f64 {
    self as f64
  }

  fn from_js(value: f64) -> Self {
    value.clamp(0., u32::MAX as f64) as u32
  }
}

impl Limit for u64 {
  fn to_js(self) -> f64 {
    self as f64
  }

  fn from_js(value: f64) -> Self {
    // Beyond 2^53 a JS number cannot say which limit it meant.
    value.clamp(0., 9_007_199_254_740_991.) as u64
  }
}

macro_rules! limits {
  ($( $js:literal => $getter:ident, $setter:ident, $field:ident; )*) => {
    #[napi]
    impl g_p_u_supported_limits {
      #[napi(constructor)]
      pub fn new() -> Self {
        Self { limits: CanvasGPUSupportedLimits::default() }
      }

      $(
        #[napi(getter, js_name = $js)]
        pub fn $getter(&self) -> f64 {
          Limit::to_js(self.limits.$field)
        }

        #[napi(setter, js_name = $js)]
        pub fn $setter(&mut self, value: Unknown) {
          if let Some(value) = as_number(&value) {
            self.limits.$field = Limit::from_js(value);
          }
        }
      )*

      /// Legacy names the V8 bindings still answer to; subgroup sizes are not limits in wgpu.
      #[napi(getter, js_name = "maxInterStageShaderComponents")]
      pub fn get_max_inter_stage_shader_components(&self) -> f64 {
        self.limits.max_inter_stage_shader_variables as f64
      }

      #[napi(setter, js_name = "maxInterStageShaderComponents")]
      pub fn set_max_inter_stage_shader_components(&mut self, value: Unknown) {
        if let Some(value) = as_number(&value) {
          self.limits.max_inter_stage_shader_variables = Limit::from_js(value);
        }
      }

      #[napi(getter, js_name = "maxPushConstantSize")]
      pub fn get_max_push_constant_size(&self) -> f64 {
        self.limits.max_immediate_size as f64
      }

      #[napi(setter, js_name = "maxPushConstantSize")]
      pub fn set_max_push_constant_size(&mut self, value: Unknown) {
        if let Some(value) = as_number(&value) {
          self.limits.max_immediate_size = Limit::from_js(value);
        }
      }

      #[napi(getter, js_name = "minSubgroupSize")]
      pub fn get_min_subgroup_size(&self) -> u32 {
        0
      }

      #[napi(setter, js_name = "minSubgroupSize")]
      pub fn set_min_subgroup_size(&mut self, _value: Unknown) {}

      #[napi(getter, js_name = "maxSubgroupSize")]
      pub fn get_max_subgroup_size(&self) -> u32 {
        0
      }

      #[napi(setter, js_name = "maxSubgroupSize")]
      pub fn set_max_subgroup_size(&mut self, _value: Unknown) {}
    }

    impl g_p_u_supported_limits {
      /// `requiredLimits` given as a plain object: the defaults, with the named limits applied.
      fn from_object(object: &Unknown) -> CanvasGPUSupportedLimits {
        let mut limits = CanvasGPUSupportedLimits::default();
        $(
          if let Some(value) = number(object, unsafe {
            std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($js, "\0").as_bytes())
          }) {
            limits.$field = Limit::from_js(value);
          }
        )*
        limits
      }
    }
  };
}

limits! {
  "maxTextureDimension1D" => get_max_texture_dimension_1d, set_max_texture_dimension_1d, max_texture_dimension_1d;
  "maxTextureDimension2D" => get_max_texture_dimension_2d, set_max_texture_dimension_2d, max_texture_dimension_2d;
  "maxTextureDimension3D" => get_max_texture_dimension_3d, set_max_texture_dimension_3d, max_texture_dimension_3d;
  "maxTextureArrayLayers" => get_max_texture_array_layers, set_max_texture_array_layers, max_texture_array_layers;
  "maxBindGroups" => get_max_bind_groups, set_max_bind_groups, max_bind_groups;
  "maxBindGroupsPlusVertexBuffers" => get_max_bind_groups_plus_vertex_buffers, set_max_bind_groups_plus_vertex_buffers, max_bind_groups_plus_vertex_buffers;
  "maxBindingsPerBindGroup" => get_max_bindings_per_bind_group, set_max_bindings_per_bind_group, max_bindings_per_bind_group;
  "maxDynamicUniformBuffersPerPipelineLayout" => get_max_dynamic_uniform_buffers_per_pipeline_layout, set_max_dynamic_uniform_buffers_per_pipeline_layout, max_dynamic_uniform_buffers_per_pipeline_layout;
  "maxDynamicStorageBuffersPerPipelineLayout" => get_max_dynamic_storage_buffers_per_pipeline_layout, set_max_dynamic_storage_buffers_per_pipeline_layout, max_dynamic_storage_buffers_per_pipeline_layout;
  "maxSampledTexturesPerShaderStage" => get_max_sampled_textures_per_shader_stage, set_max_sampled_textures_per_shader_stage, max_sampled_textures_per_shader_stage;
  "maxSamplersPerShaderStage" => get_max_samplers_per_shader_stage, set_max_samplers_per_shader_stage, max_samplers_per_shader_stage;
  "maxStorageBuffersPerShaderStage" => get_max_storage_buffers_per_shader_stage, set_max_storage_buffers_per_shader_stage, max_storage_buffers_per_shader_stage;
  "maxStorageTexturesPerShaderStage" => get_max_storage_textures_per_shader_stage, set_max_storage_textures_per_shader_stage, max_storage_textures_per_shader_stage;
  "maxUniformBuffersPerShaderStage" => get_max_uniform_buffers_per_shader_stage, set_max_uniform_buffers_per_shader_stage, max_uniform_buffers_per_shader_stage;
  "maxBindingArrayElementsPerShaderStage" => get_max_binding_array_elements_per_shader_stage, set_max_binding_array_elements_per_shader_stage, max_binding_array_elements_per_shader_stage;
  "maxBindingArraySamplerElementsPerShaderStage" => get_max_binding_array_sampler_elements_per_shader_stage, set_max_binding_array_sampler_elements_per_shader_stage, max_binding_array_sampler_elements_per_shader_stage;
  "maxUniformBufferBindingSize" => get_max_uniform_buffer_binding_size, set_max_uniform_buffer_binding_size, max_uniform_buffer_binding_size;
  "maxStorageBufferBindingSize" => get_max_storage_buffer_binding_size, set_max_storage_buffer_binding_size, max_storage_buffer_binding_size;
  "maxVertexBuffers" => get_max_vertex_buffers, set_max_vertex_buffers, max_vertex_buffers;
  "maxBufferSize" => get_max_buffer_size, set_max_buffer_size, max_buffer_size;
  "maxVertexAttributes" => get_max_vertex_attributes, set_max_vertex_attributes, max_vertex_attributes;
  "maxVertexBufferArrayStride" => get_max_vertex_buffer_array_stride, set_max_vertex_buffer_array_stride, max_vertex_buffer_array_stride;
  "maxInterStageShaderVariables" => get_max_inter_stage_shader_variables, set_max_inter_stage_shader_variables, max_inter_stage_shader_variables;
  "minUniformBufferOffsetAlignment" => get_min_uniform_buffer_offset_alignment, set_min_uniform_buffer_offset_alignment, min_uniform_buffer_offset_alignment;
  "minStorageBufferOffsetAlignment" => get_min_storage_buffer_offset_alignment, set_min_storage_buffer_offset_alignment, min_storage_buffer_offset_alignment;
  "maxColorAttachments" => get_max_color_attachments, set_max_color_attachments, max_color_attachments;
  "maxColorAttachmentBytesPerSample" => get_max_color_attachment_bytes_per_sample, set_max_color_attachment_bytes_per_sample, max_color_attachment_bytes_per_sample;
  "maxComputeWorkgroupStorageSize" => get_max_compute_workgroup_storage_size, set_max_compute_workgroup_storage_size, max_compute_workgroup_storage_size;
  "maxComputeInvocationsPerWorkgroup" => get_max_compute_invocations_per_workgroup, set_max_compute_invocations_per_workgroup, max_compute_invocations_per_workgroup;
  "maxComputeWorkgroupSizeX" => get_max_compute_workgroup_size_x, set_max_compute_workgroup_size_x, max_compute_workgroup_size_x;
  "maxComputeWorkgroupSizeY" => get_max_compute_workgroup_size_y, set_max_compute_workgroup_size_y, max_compute_workgroup_size_y;
  "maxComputeWorkgroupSizeZ" => get_max_compute_workgroup_size_z, set_max_compute_workgroup_size_z, max_compute_workgroup_size_z;
  "maxComputeWorkgroupsPerDimension" => get_max_compute_workgroups_per_dimension, set_max_compute_workgroups_per_dimension, max_compute_workgroups_per_dimension;
  "maxImmediateSize" => get_max_immediate_size, set_max_immediate_size, max_immediate_size;
  "maxNonSamplerBindings" => get_max_non_sampler_bindings, set_max_non_sampler_bindings, max_non_sampler_bindings;
  "maxTaskInvocationsPerWorkgroup" => get_max_task_invocations_per_workgroup, set_max_task_invocations_per_workgroup, max_task_invocations_per_workgroup;
  "maxTaskInvocationsPerDimension" => get_max_task_invocations_per_dimension, set_max_task_invocations_per_dimension, max_task_invocations_per_dimension;
  "maxMeshInvocationsPerWorkgroup" => get_max_mesh_invocations_per_workgroup, set_max_mesh_invocations_per_workgroup, max_mesh_invocations_per_workgroup;
  "maxMeshInvocationsPerDimension" => get_max_mesh_invocations_per_dimension, set_max_mesh_invocations_per_dimension, max_mesh_invocations_per_dimension;
  "maxTaskPayloadSize" => get_max_task_payload_size, set_max_task_payload_size, max_task_payload_size;
  "maxMeshOutputVertices" => get_max_mesh_output_vertices, set_max_mesh_output_vertices, max_mesh_output_vertices;
  "maxMeshOutputPrimitives" => get_max_mesh_output_primitives, set_max_mesh_output_primitives, max_mesh_output_primitives;
  "maxMeshOutputLayers" => get_max_mesh_output_layers, set_max_mesh_output_layers, max_mesh_output_layers;
  "maxMeshMultiviewViewCount" => get_max_mesh_multiview_view_count, set_max_mesh_multiview_view_count, max_mesh_multiview_view_count;
  "maxBlasPrimitiveCount" => get_max_blas_primitive_count, set_max_blas_primitive_count, max_blas_primitive_count;
  "maxBlasGeometryCount" => get_max_blas_geometry_count, set_max_blas_geometry_count, max_blas_geometry_count;
  "maxTlasInstanceCount" => get_max_tlas_instance_count, set_max_tlas_instance_count, max_tlas_instance_count;
  "maxAccelerationStructuresPerShaderStage" => get_max_acceleration_structures_per_shader_stage, set_max_acceleration_structures_per_shader_stage, max_acceleration_structures_per_shader_stage;
  "maxMultiviewViewCount" => get_max_multiview_view_count, set_max_multiview_view_count, max_multiview_view_count;
}

/// `requiredLimits`: a `GPUSupportedLimits` (what `packages/canvas` builds) or, leniently, a
/// plain object of limits.
pub(crate) fn required_limits(value: &Unknown) -> Option<CanvasGPUSupportedLimits> {
  if let Some(limits) = downcast::<g_p_u_supported_limits>(value) {
    return Some(limits.limits);
  }
  crate::gpu::parse::is_object(value).then(|| g_p_u_supported_limits::from_object(value))
}
