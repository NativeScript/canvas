use std::sync::Arc;

use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUPipelineLayout {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) layout: wgpu_core::id::PipelineLayoutId,
}
