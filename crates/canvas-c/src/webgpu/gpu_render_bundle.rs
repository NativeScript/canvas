use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;
use crate::webgpu::prelude::label_to_ptr;
//use wgpu_core::gfx_select;
use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPURenderBundle {
    pub(super) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) bundle: wgpu_core::id::RenderBundleId,
}

impl Drop for CanvasGPURenderBundle {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            gfx_select!(self.bundle => global.render_bundle_drop(self.bundle));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_get_label(
    bundle: *const CanvasGPURenderBundle,
) -> *mut c_char {
    if bundle.is_null() {
        return std::ptr::null_mut();
    }

    let bundle = &*bundle;
    label_to_ptr(bundle.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_reference(
    bundle: *const CanvasGPURenderBundle,
) {
    if bundle.is_null() {
        return;
    }

    Arc::increment_strong_count(bundle);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_release(
    bundle: *const CanvasGPURenderBundle,
) {
    if bundle.is_null() {
        return;
    }

    Arc::decrement_strong_count(bundle);
}
