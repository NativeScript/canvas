use super::gpu::CanvasWebGPUInstance;

#[derive(Clone)]
pub struct CanvasGPUTextureView {
    instance: CanvasWebGPUInstance,
    texture: wgpu_core::id::TextureViewId,
}
