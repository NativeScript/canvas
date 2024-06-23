use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUBindGroup {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) group: wgpu_core::id::BindGroupId,
}
