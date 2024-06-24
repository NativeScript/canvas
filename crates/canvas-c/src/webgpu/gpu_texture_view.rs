use super::gpu::CanvasWebGPUInstance;

#[derive(Clone)]
pub struct CanvasGPUTextureView {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) texture_view: wgpu_core::id::TextureViewId,
}
