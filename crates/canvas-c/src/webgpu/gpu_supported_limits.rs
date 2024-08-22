#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CanvasGPUSupportedLimits {
    pub max_texture_dimension_1d: u32,
    pub max_texture_dimension_2d: u32,
    pub max_texture_dimension_3d: u32,
    pub max_texture_array_layers: u32,
    pub max_bind_groups: u32,
    pub max_bindings_per_bind_group: u32,
    pub max_dynamic_uniform_buffers_per_pipeline_layout: u32,
    pub max_dynamic_storage_buffers_per_pipeline_layout: u32,
    pub max_sampled_textures_per_shader_stage: u32,
    pub max_samplers_per_shader_stage: u32,
    pub max_storage_buffers_per_shader_stage: u32,
    pub max_storage_textures_per_shader_stage: u32,
    pub max_uniform_buffers_per_shader_stage: u32,
    pub max_uniform_buffer_binding_size: u32,
    pub max_storage_buffer_binding_size: u32,
    pub max_vertex_buffers: u32,
    pub max_buffer_size: u64,
    pub max_vertex_attributes: u32,
    pub max_vertex_buffer_array_stride: u32,
    pub min_uniform_buffer_offset_alignment: u32,
    pub min_storage_buffer_offset_alignment: u32,
    pub max_inter_stage_shader_components: u32,
    pub max_color_attachments: u32,
    pub max_color_attachment_bytes_per_sample: u32,
    pub max_compute_workgroup_storage_size: u32,
    pub max_compute_invocations_per_workgroup: u32,
    pub max_compute_workgroup_size_x: u32,
    pub max_compute_workgroup_size_y: u32,
    pub max_compute_workgroup_size_z: u32,
    pub max_compute_workgroups_per_dimension: u32,
    pub min_subgroup_size: u32,
    pub max_subgroup_size: u32,
    pub max_push_constant_size: u32,
    pub max_non_sampler_bindings: u32,
}

impl Into<wgt::Limits> for CanvasGPUSupportedLimits {
    fn into(self) -> wgt::Limits {
        wgt::Limits {
            max_texture_dimension_1d: self.max_texture_dimension_1d,
            max_texture_dimension_2d: self.max_texture_dimension_2d,
            max_texture_dimension_3d: self.max_texture_dimension_3d,
            max_texture_array_layers: self.max_texture_array_layers,
            max_bind_groups: self.max_bind_groups,
            max_bindings_per_bind_group: self.max_bindings_per_bind_group,
            max_dynamic_uniform_buffers_per_pipeline_layout: self
                .max_dynamic_uniform_buffers_per_pipeline_layout,
            max_dynamic_storage_buffers_per_pipeline_layout: self
                .max_dynamic_storage_buffers_per_pipeline_layout,
            max_sampled_textures_per_shader_stage: self.max_sampled_textures_per_shader_stage,
            max_samplers_per_shader_stage: self.max_samplers_per_shader_stage,
            max_storage_buffers_per_shader_stage: self.max_storage_buffers_per_shader_stage,
            max_storage_textures_per_shader_stage: self.max_storage_textures_per_shader_stage,
            max_uniform_buffers_per_shader_stage: self.max_uniform_buffers_per_shader_stage,
            max_uniform_buffer_binding_size: self.max_uniform_buffer_binding_size,
            max_storage_buffer_binding_size: self.max_storage_buffer_binding_size,
            max_vertex_buffers: self.max_vertex_buffers,
            max_buffer_size: self.max_buffer_size,
            max_vertex_attributes: self.max_vertex_attributes,
            max_vertex_buffer_array_stride: self.max_vertex_buffer_array_stride,
            min_uniform_buffer_offset_alignment: self.min_uniform_buffer_offset_alignment,
            min_storage_buffer_offset_alignment: self.min_storage_buffer_offset_alignment,
            max_inter_stage_shader_components: self.max_inter_stage_shader_components,
            max_color_attachments: self.max_color_attachments,
            max_color_attachment_bytes_per_sample: self.max_color_attachment_bytes_per_sample,
            max_compute_workgroup_storage_size: self.max_compute_workgroup_storage_size,
            max_compute_invocations_per_workgroup: self.max_compute_invocations_per_workgroup,
            max_compute_workgroup_size_x: self.max_compute_workgroup_size_x,
            max_compute_workgroup_size_y: self.max_compute_workgroup_size_y,
            max_compute_workgroup_size_z: self.max_compute_workgroup_size_z,
            max_compute_workgroups_per_dimension: self.max_compute_workgroups_per_dimension,
            min_subgroup_size: self.min_subgroup_size,
            max_subgroup_size: self.max_subgroup_size,
            max_push_constant_size: self.max_push_constant_size,
            max_non_sampler_bindings: self.max_non_sampler_bindings,
        }
    }
}

impl From<wgt::Limits> for CanvasGPUSupportedLimits {
    fn from(value: wgt::Limits) -> Self {
        CanvasGPUSupportedLimits {
            max_texture_dimension_1d: value.max_texture_dimension_1d,
            max_texture_dimension_2d: value.max_texture_dimension_2d,
            max_texture_dimension_3d: value.max_texture_dimension_3d,
            max_texture_array_layers: value.max_texture_array_layers,
            max_bind_groups: value.max_bind_groups,
            max_bindings_per_bind_group: value.max_bindings_per_bind_group,
            max_dynamic_uniform_buffers_per_pipeline_layout: value
                .max_dynamic_uniform_buffers_per_pipeline_layout,
            max_dynamic_storage_buffers_per_pipeline_layout: value
                .max_dynamic_storage_buffers_per_pipeline_layout,
            max_sampled_textures_per_shader_stage: value.max_sampled_textures_per_shader_stage,
            max_samplers_per_shader_stage: value.max_samplers_per_shader_stage,
            max_storage_buffers_per_shader_stage: value.max_storage_buffers_per_shader_stage,
            max_storage_textures_per_shader_stage: value.max_storage_textures_per_shader_stage,
            max_uniform_buffers_per_shader_stage: value.max_uniform_buffers_per_shader_stage,
            max_uniform_buffer_binding_size: value.max_uniform_buffer_binding_size,
            max_storage_buffer_binding_size: value.max_storage_buffer_binding_size,
            max_vertex_buffers: value.max_vertex_buffers,
            max_buffer_size: value.max_buffer_size,
            max_vertex_attributes: value.max_vertex_attributes,
            max_vertex_buffer_array_stride: value.max_vertex_buffer_array_stride,
            min_uniform_buffer_offset_alignment: value.min_uniform_buffer_offset_alignment,
            min_storage_buffer_offset_alignment: value.min_storage_buffer_offset_alignment,
            max_inter_stage_shader_components: value.max_inter_stage_shader_components,
            max_color_attachments: value.max_color_attachments,
            max_color_attachment_bytes_per_sample: value.max_color_attachment_bytes_per_sample,
            max_compute_workgroup_storage_size: value.max_compute_workgroup_storage_size,
            max_compute_invocations_per_workgroup: value.max_compute_invocations_per_workgroup,
            max_compute_workgroup_size_x: value.max_compute_workgroup_size_x,
            max_compute_workgroup_size_y: value.max_compute_workgroup_size_y,
            max_compute_workgroup_size_z: value.max_compute_workgroup_size_z,
            max_compute_workgroups_per_dimension: value.max_compute_workgroups_per_dimension,
            min_subgroup_size: value.min_subgroup_size,
            max_subgroup_size: value.max_subgroup_size,
            max_push_constant_size: value.max_push_constant_size,
            max_non_sampler_bindings: value.max_non_sampler_bindings,
        }
    }
}

impl Default for CanvasGPUSupportedLimits {
    fn default() -> Self {
        wgt::Limits::default().into()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_create_limits() -> *mut CanvasGPUSupportedLimits {
    Box::into_raw(Box::new(wgt::Limits::default().into()))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_limits_release(
    limits: *mut CanvasGPUSupportedLimits,
) {
    if limits.is_null() {
        return;
    }

    // Arc::decrement_strong_count(limits);
    let _ = Box::from_raw(limits);
}
