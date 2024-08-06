use std::sync::Arc;

use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUPipelineLayout {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) layout: wgpu_core::id::PipelineLayoutId,
}

impl Drop for CanvasGPUPipelineLayout {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            gfx_select!(layout => global.pipeline_layout_drop(self.layout));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_pipeline_layout_reference(
    pipeline_layout: *const CanvasGPUPipelineLayout,
) {
    if pipeline_layout.is_null() {
        return;
    }

    Arc::increment_strong_count(pipeline_layout);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_pipeline_layout_release(
    pipeline_layout: *const CanvasGPUPipelineLayout,
) {
    if pipeline_layout.is_null() {
        return;
    }

    Arc::decrement_strong_count(pipeline_layout);
}
