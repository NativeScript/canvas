use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPURenderBundle {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) bundle: wgpu_core::id::RenderBundleId
}