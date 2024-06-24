pub struct CanvasGPUPipelineLayout {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) layout: wgpu_core::id::PipelineLayoutId,
}
