use std::sync::Arc;
use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUCommandBuffer {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) command_buffer: wgpu_core::id::CommandBufferId,
}
