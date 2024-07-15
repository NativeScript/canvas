use std::borrow::Cow;
use std::sync::Arc;

use crate::webgpu::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUSampler {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) sampler: wgpu_core::id::SamplerId,
    pub(crate) label: Option<Cow<'static, str>>,
}
