use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPURenderPipeline {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) pipeline: wgpu_core::id::RenderPipelineId,
}
