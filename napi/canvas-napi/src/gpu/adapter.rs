use crate::gpu::adapter_info::AsyncAdapterInfo;
use crate::gpu::device::AsyncAdapterDevice;
use canvas_c::webgpu::gpu_supported_limits::CanvasGPUSupportedLimits;
use napi::bindgen_prelude::{AsyncTask, ObjectFinalize};
use napi::*;
use napi_derive::napi;
use std::sync::Arc;

#[napi]
pub struct g_p_u_adapter {
    pub(crate) adapter: Arc<canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter>,
}


#[napi]
impl g_p_u_adapter {
    fn ptr(&self) -> *const canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter {
        Arc::as_ptr(&self.adapter)
    }

    #[napi(getter)]
    pub fn get_features(&self) -> Vec<String> {
        let features = canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_get_features(self.ptr());
        if features.is_null() {
            return Vec::new();
        }
        unsafe { (*Box::from_raw(features)).into() }
    }

    #[napi(getter)]
    pub fn get_is_fallback_adapter(&self) -> bool {
        canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_is_fallback_adapter(self.ptr())
    }

    #[napi(getter)]
    pub fn get_limits(&self) -> GPUSupportedLimits {
        let limits = canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_get_limits(self.ptr());
        if limits.is_null() {
            return GPUSupportedLimits::default();
        }
        unsafe { GPUSupportedLimits::from(*Box::from_raw(limits)) }
    }

    #[napi(ts_return_type = "Promise<GPUAdapterInfo>")]
    pub fn request_adapter_info(&self) -> AsyncTask<AsyncAdapterInfo> {
        AsyncTask::new(AsyncAdapterInfo::new(Arc::clone(&self.adapter)))
    }

    #[napi(ts_return_type = "Promise<GPUDevice>")]
    pub fn request_device(&self, option: Option<crate::gpu::device::GPUDeviceDescriptor>) -> AsyncTask<AsyncAdapterDevice> {
        AsyncTask::new(AsyncAdapterDevice::new(Arc::clone(&self.adapter), option))
    }
}

#[napi(js_name = "GPUSupportedLimits", object)]
pub struct GPUSupportedLimits {
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
    pub max_buffer_size: i64,
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

impl From<CanvasGPUSupportedLimits> for GPUSupportedLimits {
    fn from(value: CanvasGPUSupportedLimits) -> Self {
        Self {
            max_texture_dimension_1d: value.max_texture_dimension_1d,
            max_texture_dimension_2d: value.max_texture_dimension_2d,
            max_texture_dimension_3d: value.max_texture_dimension_3d,
            max_texture_array_layers: value.max_texture_array_layers,
            max_bind_groups: value.max_bind_groups,
            max_bindings_per_bind_group: value.max_bindings_per_bind_group,
            max_dynamic_uniform_buffers_per_pipeline_layout: value.max_dynamic_uniform_buffers_per_pipeline_layout,
            max_dynamic_storage_buffers_per_pipeline_layout: value.max_dynamic_storage_buffers_per_pipeline_layout,
            max_sampled_textures_per_shader_stage: value.max_sampled_textures_per_shader_stage,
            max_samplers_per_shader_stage: value.max_samplers_per_shader_stage,
            max_storage_buffers_per_shader_stage: value.max_storage_buffers_per_shader_stage,
            max_storage_textures_per_shader_stage: value.max_storage_textures_per_shader_stage,
            max_uniform_buffers_per_shader_stage: value.max_uniform_buffers_per_shader_stage,
            max_uniform_buffer_binding_size: value.max_uniform_buffer_binding_size,
            max_storage_buffer_binding_size: value.max_storage_buffer_binding_size,
            max_vertex_buffers: value.max_vertex_buffers,
            max_buffer_size: value.max_buffer_size as i64,
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

impl Into<CanvasGPUSupportedLimits> for GPUSupportedLimits {
    fn into(self) -> CanvasGPUSupportedLimits {
        CanvasGPUSupportedLimits {
            max_texture_dimension_1d: self.max_texture_dimension_1d,
            max_texture_dimension_2d: self.max_texture_dimension_2d,
            max_texture_dimension_3d: self.max_texture_dimension_3d,
            max_texture_array_layers: self.max_texture_array_layers,
            max_bind_groups: self.max_bind_groups,
            max_bindings_per_bind_group: self.max_bindings_per_bind_group,
            max_dynamic_uniform_buffers_per_pipeline_layout: self.max_dynamic_uniform_buffers_per_pipeline_layout,
            max_dynamic_storage_buffers_per_pipeline_layout: self.max_dynamic_storage_buffers_per_pipeline_layout,
            max_sampled_textures_per_shader_stage: self.max_sampled_textures_per_shader_stage,
            max_samplers_per_shader_stage: self.max_samplers_per_shader_stage,
            max_storage_buffers_per_shader_stage: self.max_storage_buffers_per_shader_stage,
            max_storage_textures_per_shader_stage: self.max_storage_textures_per_shader_stage,
            max_uniform_buffers_per_shader_stage: self.max_uniform_buffers_per_shader_stage,
            max_uniform_buffer_binding_size: self.max_uniform_buffer_binding_size,
            max_storage_buffer_binding_size: self.max_storage_buffer_binding_size,
            max_vertex_buffers: self.max_vertex_buffers,
            max_buffer_size: self.max_buffer_size as u64,
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


impl Default for GPUSupportedLimits {
    fn default() -> Self {
        Self {
            max_texture_dimension_1d: 8192,
            max_texture_dimension_2d: 8192,
            max_texture_dimension_3d: 2048,
            max_texture_array_layers: 256,
            max_bind_groups: 4,
            max_bindings_per_bind_group: 1000,
            max_dynamic_uniform_buffers_per_pipeline_layout: 8,
            max_dynamic_storage_buffers_per_pipeline_layout: 4,
            max_sampled_textures_per_shader_stage: 16,
            max_samplers_per_shader_stage: 16,
            max_storage_buffers_per_shader_stage: 8,
            max_storage_textures_per_shader_stage: 4,
            max_uniform_buffers_per_shader_stage: 12,
            max_uniform_buffer_binding_size: 64 << 10, // (64 KiB)
            max_storage_buffer_binding_size: 128 << 20, // (128 MiB)
            max_vertex_buffers: 8,
            max_buffer_size: 256 << 20, // (256 MiB)
            max_vertex_attributes: 16,
            max_vertex_buffer_array_stride: 2048,
            min_uniform_buffer_offset_alignment: 256,
            min_storage_buffer_offset_alignment: 256,
            max_inter_stage_shader_components: 60,
            max_color_attachments: 8,
            max_color_attachment_bytes_per_sample: 32,
            max_compute_workgroup_storage_size: 16384,
            max_compute_invocations_per_workgroup: 256,
            max_compute_workgroup_size_x: 256,
            max_compute_workgroup_size_y: 256,
            max_compute_workgroup_size_z: 64,
            max_compute_workgroups_per_dimension: 65535,
            min_subgroup_size: 0,
            max_subgroup_size: 0,
            max_push_constant_size: 0,
            max_non_sampler_bindings: 1_000_000,
        }
    }
}