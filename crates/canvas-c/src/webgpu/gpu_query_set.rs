use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUQuerySet {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) query: wgpu_core::id::QuerySetId,
}
