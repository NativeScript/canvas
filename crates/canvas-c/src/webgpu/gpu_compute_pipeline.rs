use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;

//use wgpu_core::gfx_select;
use crate::webgpu::error::handle_error;
use crate::webgpu::gpu::CanvasWebGPUInstance;
use crate::webgpu::gpu_bind_group_layout::CanvasGPUBindGroupLayout;
use crate::webgpu::prelude::label_to_ptr;

pub struct CanvasGPUComputePipeline {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) pipeline: wgpu_core::id::ComputePipelineId,
    pub(crate) error_sink: super::gpu_device::ErrorSink,
}

impl Drop for CanvasGPUComputePipeline {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            global.compute_pipeline_drop(self.pipeline);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pipeline_get_label(
    pipeline: *const CanvasGPUComputePipeline,
) -> *mut c_char {
    if pipeline.is_null() {
        return std::ptr::null_mut();
    }
    let pipeline = &*pipeline;
    label_to_ptr(pipeline.label.clone())
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pipeline_reference(
    pipeline: *const CanvasGPUComputePipeline,
) {
    if pipeline.is_null() {
        return;
    }

    Arc::increment_strong_count(pipeline);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pipeline_release(
    pipeline: *const CanvasGPUComputePipeline,
) {
    if pipeline.is_null() {
        return;
    }

    Arc::decrement_strong_count(pipeline);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compute_pipeline_get_bind_group_layout(
    pipeline: *const CanvasGPUComputePipeline,
    index: u32,
) -> *const CanvasGPUBindGroupLayout {
    if pipeline.is_null() {
        return std::ptr::null_mut();
    }

    let pipeline = &*pipeline;
    let pipeline_id = pipeline.pipeline;

    let global = pipeline.instance.global();
    let error_sink = pipeline.error_sink.as_ref();
    let (group_layout, error) =  global.compute_pipeline_get_bind_group_layout(pipeline_id, index, None);


    if let Some(cause) = error {
        handle_error(
            global,
            error_sink,
            cause,
            "",
            None,
            "canvas_native_webgpu_compute_pipeline_get_bind_group_layout",
        );
    }


    Arc::into_raw(Arc::new(CanvasGPUBindGroupLayout {
        label: None,
        instance: pipeline.instance.clone(),
        group_layout,
    }))
}
