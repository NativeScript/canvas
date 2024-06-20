use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPURenderPassEncoder {
    instance: CanvasWebGPUInstance,
    pass: wgpu_core::id::RenderPassEncoderId,
}
