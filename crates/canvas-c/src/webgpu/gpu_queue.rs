use std::sync::Arc;

use super::gpu::CanvasWebGPUInstance;

#[derive(Clone)]
pub struct CanvasGPUQueue {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) queue: wgpu_core::id::QueueId,
}

unsafe impl Send for CanvasGPUQueue {}
