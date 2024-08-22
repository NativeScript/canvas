use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;

use crate::webgpu::prelude::label_to_ptr;

//use wgpu_core::gfx_select;
use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUBindGroup {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) group: wgpu_core::id::BindGroupId,
}

impl Drop for CanvasGPUBindGroup {
    fn drop(&mut self) {
        let global = self.instance.global();
        let group_id = self.group;
        gfx_select!(group_id => global.bind_group_drop(group_id));
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_bind_group_get_label(
    bind_group: *const CanvasGPUBindGroup,
) -> *mut c_char {
    if bind_group.is_null() {
        return std::ptr::null_mut();
    }
    let bind_group = &*bind_group;
    label_to_ptr(bind_group.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_bind_group_reference(
    bind_group: *const CanvasGPUBindGroup,
) {
    if bind_group.is_null() {
        return;
    }

    Arc::increment_strong_count(bind_group);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_bind_group_release(
    bind_group: *const CanvasGPUBindGroup,
) {
    if bind_group.is_null() {
        return;
    }

    Arc::decrement_strong_count(bind_group);
}
