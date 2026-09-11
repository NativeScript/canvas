use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;

//use wgpu_core::gfx_select;
use crate::webgpu::error::handle_error;
use crate::webgpu::prelude::label_to_ptr;

use super::{gpu::CanvasWebGPUInstance, gpu_bind_group_layout::CanvasGPUBindGroupLayout};

pub struct CanvasGPURenderPipeline {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) pipeline: Arc<wgpu_core::pipeline::RenderPipeline>,
    pub(crate) error_sink: super::gpu_device::ErrorSink,
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pipeline_reference(
    pipeline: *const CanvasGPURenderPipeline,
) {
    if pipeline.is_null() {
        return;
    }
    Arc::increment_strong_count(pipeline);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pipeline_release(
    pipeline: *const CanvasGPURenderPipeline,
) {
    if pipeline.is_null() {
        return;
    }

    Arc::decrement_strong_count(pipeline);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pipeline_get_label(
    pipeline: *const CanvasGPURenderPipeline,
) -> *mut c_char {
    if pipeline.is_null() {
        return std::ptr::null_mut();
    }
    let pipeline = &*pipeline;
    label_to_ptr(pipeline.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_pipeline_get_bind_group_layout(
    pipeline: *const CanvasGPURenderPipeline,
    index: u32,
) -> *const CanvasGPUBindGroupLayout {
    if pipeline.is_null() {
        return std::ptr::null_mut();
    }

    let pipeline = &*pipeline;
    let pipeline_id = pipeline.pipeline;

    let (group_layout, error) = global.render_pipeline_get_bind_group_layout(pipeline_id, index, None);
Arc::into_raw(Arc::new(CanvasGPUBindGroupLayout {
        label: None,
        instance: pipeline.instance.clone(),
        group_layout,
    }))
}
