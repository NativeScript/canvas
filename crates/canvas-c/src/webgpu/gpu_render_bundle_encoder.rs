use std::sync::Arc;

use crate::webgpu::gpu::CanvasWebGPUInstance;

pub struct CanvasGPURenderBundleEncoder {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) encoder: wgpu_core::command::RenderBundleEncoder,
}
