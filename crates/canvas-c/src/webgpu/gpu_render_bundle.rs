use std::sync::Arc;

use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPURenderBundle {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) bundle: wgpu_core::id::RenderBundleId,
}