use super::{gpu::CanvasWebGPUInstance, gpu_bind_group_layout::CanvasGPUBindGroupLayout};

pub struct CanvasGPURenderPipeline {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) pipeline: wgpu_core::id::RenderPipelineId,
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pipeline_get_bind_group_layout(
    pipeline: *const CanvasGPURenderPipeline,
    index: u32,
)-> *mut CanvasGPUBindGroupLayout {
    if pipeline.is_null() {
        return std::ptr::null_mut();
    }

    let pipeline = &*pipeline;
    let pipeline_id = pipeline.pipeline;

    let global = &pipeline.instance.0;

    let (group_layout, error) = gfx_select!(pipeline_id => global.render_pipeline_get_bind_group_layout(pipeline_id, index, None));

    if let Some(error) = error {
        // todo handle error
        return std::ptr::null_mut();
    }

    Box::into_raw(Box::new(CanvasGPUBindGroupLayout {
        instance: pipeline.instance.clone(),
        group_layout: group_layout,
    }))
}
