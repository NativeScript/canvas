use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;

//use wgpu_core::gfx_select;
use crate::webgpu::gpu::CanvasWebGPUInstance;
use crate::webgpu::prelude::label_to_ptr;

pub struct CanvasGPUSampler {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) sampler: wgpu_core::id::SamplerId,
    pub(crate) label: Option<Cow<'static, str>>,
}

impl Drop for CanvasGPUSampler {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            global.sampler_drop(self.sampler);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_sampler_get_label(sampler: *const CanvasGPUSampler) -> *mut c_char {
    if sampler.is_null() {
        return std::ptr::null_mut();
    }

    let sampler = &*sampler;

    let sampler = &*sampler;
    label_to_ptr(sampler.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_sampler_reference(sampler: *const CanvasGPUSampler) {
    if sampler.is_null() {
        return;
    }

    Arc::increment_strong_count(sampler);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_sampler_release(sampler: *const CanvasGPUSampler) {
    if sampler.is_null() {
        return;
    }

    Arc::decrement_strong_count(sampler);
}
