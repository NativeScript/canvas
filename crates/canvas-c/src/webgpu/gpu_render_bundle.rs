use std::sync::Arc;

use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPURenderBundle {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) bundle: wgpu_core::id::RenderBundleId,
}

impl Drop for CanvasGPURenderBundle {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            gfx_select!(self.id => global.render_bundle_drop(self.bundle));
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_reference(
    bundle: *const CanvasGPURenderBundle
) {
    if bundle.is_null() {
        return;
    }

    Arc::increment_strong_count(bundle);
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_render_bundle_release(
    bundle: *const CanvasGPURenderBundle
) {
    if bundle.is_null() {
        return;
    }

    Arc::decrement_strong_count(bundle);
}