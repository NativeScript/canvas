use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUCommandBuffer {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) command_buffer: wgpu_core::id::CommandBufferId,
}
