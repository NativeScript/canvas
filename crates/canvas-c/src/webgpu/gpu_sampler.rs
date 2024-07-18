use std::borrow::Cow;
use std::sync::Arc;

use crate::webgpu::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUSampler {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) sampler: wgpu_core::id::SamplerId,
    pub(crate) label: Option<Cow<'static, str>>,
}


impl Drop for CanvasGPUSampler {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            gfx_select!(self.id => global.sampler_drop(self.sampler));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_sampler_reference(
    sampler: *const CanvasGPUSampler
) {
    if sampler.is_null() {
        return;
    }

    Arc::increment_strong_count(sampler);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_sampler_release(
    sampler: *const CanvasGPUSampler
) {
    if sampler.is_null() {
        return;
    }

    Arc::decrement_strong_count(sampler);
}