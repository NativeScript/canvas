use std::sync::Arc;

use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUPipelineLayout {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) layout: wgpu_core::id::PipelineLayoutId,
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_pipeline_layout_reference(
    pipeline_layout: *const CanvasGPUPipelineLayout
) {
    if pipeline_layout.is_null() {
        return;
    }

    Arc::increment_strong_count(pipeline_layout);
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_pipeline_layout_release(
    pipeline_layout: *const CanvasGPUPipelineLayout
) {
    if pipeline_layout.is_null() {
        return;
    }

    Arc::decrement_strong_count(pipeline_layout);
}
