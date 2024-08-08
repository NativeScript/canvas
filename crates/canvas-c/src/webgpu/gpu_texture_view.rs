use std::sync::Arc;
//use wgpu_core::gfx_select;
use super::gpu::CanvasWebGPUInstance;
#[derive(Clone)]
pub struct CanvasGPUTextureView {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) texture_view: wgpu_core::id::TextureViewId,
}

impl Drop for CanvasGPUTextureView {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            let _ = gfx_select!(self.texture_view => global.texture_view_drop(self.texture_view, false));
        }
    }
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
