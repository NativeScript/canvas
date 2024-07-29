use std::sync::Arc;

use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUBindGroupLayout {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) group_layout: wgpu_core::id::BindGroupLayoutId,
}

impl Drop for CanvasGPUBindGroupLayout {
    fn drop(&mut self) {
        let global = self.instance.global();
        let group_layout_id = self.group_layout;
        gfx_select!(group_id => global.bind_group_layout_drop(group_layout_id));
    }
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
