use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;

use crate::webgpu::prelude::label_to_ptr;

//use wgpu_core::gfx_select;
use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUBindGroupLayout {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) group_layout: wgpu_core::id::BindGroupLayoutId,
}

impl Drop for CanvasGPUBindGroupLayout {
    fn drop(&mut self) {
        let global = self.instance.global();
        let group_layout_id = self.group_layout;
        global.bind_group_layout_drop(group_layout_id);
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_bind_group_layout_get_label(
    bind_group_layout: *const CanvasGPUBindGroupLayout,
) -> *mut c_char {
    if bind_group_layout.is_null() {
        return std::ptr::null_mut();
    }
    let bind_group_layout = &*bind_group_layout;
    label_to_ptr(bind_group_layout.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_bind_group_layout_reference(
    bind_group_layout: *const CanvasGPUBindGroupLayout,
) {
    if bind_group_layout.is_null() {
        return;
    }

    Arc::increment_strong_count(bind_group_layout);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_bind_group_layout_release(
    bind_group_layout: *const CanvasGPUBindGroupLayout,
) {
    if bind_group_layout.is_null() {
        return;
    }

    Arc::decrement_strong_count(bind_group_layout);
}
