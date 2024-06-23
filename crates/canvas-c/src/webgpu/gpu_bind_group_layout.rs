use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUBindGroupLayout {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) group_layout: wgpu_core::id::BindGroupLayoutId,
}
