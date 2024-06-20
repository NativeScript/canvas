use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUShaderModule {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) module: wgpu_core::id::ShaderModuleId,
}
