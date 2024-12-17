use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;

use crate::webgpu::prelude::label_to_ptr;

//use wgpu_core::gfx_select;
use super::gpu::CanvasWebGPUInstance;

#[derive(Clone, Debug)]
pub struct CanvasGPUTextureView {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) texture_view: wgpu_core::id::TextureViewId,
}

impl Drop for CanvasGPUTextureView {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            let _ = global.texture_view_drop(self.texture_view);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_texture_view_get_label(
    texture_view: *const CanvasGPUTextureView,
) -> *mut c_char {
    if texture_view.is_null() {
        return std::ptr::null_mut();
    }

    let texture_view = &*texture_view;
    label_to_ptr(texture_view.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_texture_view_reference(
    texture_view: *const CanvasGPUTextureView,
) {
    if texture_view.is_null() {
        return;
    }

    Arc::increment_strong_count(texture_view);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_texture_view_release(
    texture_view: *const CanvasGPUTextureView,
) {
    if texture_view.is_null() {
        return;
    }

    Arc::decrement_strong_count(texture_view);
}
