use std::sync::Arc;

use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUQuerySet {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) query: wgpu_core::id::QuerySetId,
}
