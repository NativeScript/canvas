use std::sync::Arc;

use crate::webgpu::error::handle_error;
use crate::webgpu::gpu::CanvasWebGPUInstance;
use crate::webgpu::gpu_bind_group_layout::CanvasGPUBindGroupLayout;

pub struct CanvasGPUComputePipeline {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) pipeline: wgpu_core::id::ComputePipelineId,
    pub(crate) error_sink: super::gpu_device::ErrorSink,
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pipeline_reference(pipeline: *const CanvasGPUComputePipeline) {
    if pipeline.is_null() {
        return;
    }
    Arc::increment_strong_count(pipeline);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pipeline_release(pipeline: *const CanvasGPUComputePipeline) {
    if pipeline.is_null() {
        return;
    }

    Arc::decrement_strong_count(pipeline);
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pipeline_get_bind_group_layout(
    pipeline: *const CanvasGPUComputePipeline,
    index: u32,
) -> *const CanvasGPUBindGroupLayout {
    if pipeline.is_null() {
        return std::ptr::null_mut();
    }

    let pipeline = &*pipeline;
    let pipeline_id = pipeline.pipeline;

    let global = pipeline.instance.global();
    let error_sink = pipeline.error_sink.as_ref();
    let (group_layout, error) = gfx_select!(pipeline_id => global.compute_pipeline_get_bind_group_layout(pipeline_id, index, None));

    if let Some(cause) = error {
        handle_error(
            global,
            error_sink,
            cause,
            "",
            None,
            "canvas_native_webgpu_compute_pipeline_get_bind_group_layout",
        );
    }

    Arc::into_raw(Arc::new(CanvasGPUBindGroupLayout {
        instance: pipeline.instance.clone(),
        group_layout,
    }))
}
