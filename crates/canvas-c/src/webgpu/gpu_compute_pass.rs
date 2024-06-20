use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUComputePass {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) pass: wgpu_core::command::ComputePass,
}
