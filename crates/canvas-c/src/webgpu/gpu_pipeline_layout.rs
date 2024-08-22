use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;
use crate::webgpu::prelude::label_to_ptr;
use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUPipelineLayout {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) layout: wgpu_core::id::PipelineLayoutId,
}

unsafe impl Send for CanvasGPUPipelineLayout {}

impl Drop for CanvasGPUPipelineLayout {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            gfx_select!(layout => global.pipeline_layout_drop(self.layout));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_pipeline_layout_get_label(
    pipeline_layout: *const CanvasGPUPipelineLayout,
) -> *mut c_char {
    if pipeline_layout.is_null() {
        return std::ptr::null_mut();
    }
    let pipeline_layout = &*pipeline_layout;
    label_to_ptr(pipeline_layout.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_pipeline_layout_reference(
    pipeline_layout: *const CanvasGPUPipelineLayout,
) {
    if pipeline_layout.is_null() {
        return;
    }

    Arc::increment_strong_count(pipeline_layout);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_pipeline_layout_release(
    pipeline_layout: *const CanvasGPUPipelineLayout,
) {
    if pipeline_layout.is_null() {
        return;
    }

    Arc::decrement_strong_count(pipeline_layout);
}
