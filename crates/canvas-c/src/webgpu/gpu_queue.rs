use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CanvasGPUQueue(pub(crate) Arc<parking_lot::RwLock<wgpu::Queue>>);

unsafe impl Send for CanvasGPUQueue {}
