//! Linux: a transferred canvas's view takes frames through a hook the host supplies, called on the
//! thread that renders the context (render thread, WebGL thread or owner) at frame end.

use std::sync::Arc;

use canvas_core::gpu::gl::PresentHook;

use super::CanvasOffscreenSurface;
use crate::{CanvasColorSpace, CanvasRenderingContext2D, WebGLState};

pub(super) struct View {
    pub(super) hook: PresentHook,
}

pub(super) fn hook_2d(context: &CanvasRenderingContext2D, hook: PresentHook) {
    match context.render_target() {
        Some(target) => target.post(move |real| {
            if let Some(gl) = real.context.gl_context.as_ref() {
                gl.set_present_hook(Some(hook));
            }
        }),
        None => {
            if let Some(gl) = context.context.gl_context.as_ref() {
                gl.set_present_hook(Some(hook));
            }
        }
    }
}

pub(super) fn hook_webgl(state: &WebGLState, hook: PresentHook) {
    if state.is_threaded() {
        state.post(move |inner| inner.gl_context().set_present_hook(Some(hook)));
    } else {
        state.get_inner().gl_context().set_present_hook(Some(hook));
    }
}

/// UI thread. `hook(width, height)` gets every frame of the context made on the surface.
pub fn canvas_native_offscreen_surface_create_linux(
    width: u32,
    height: u32,
    density: f32,
    ppi: f32,
    direction: u32,
    color_space: CanvasColorSpace,
    hook: PresentHook,
) -> *const CanvasOffscreenSurface {
    let surface = CanvasOffscreenSurface::new(
        width.max(1),
        height.max(1),
        density,
        ppi,
        direction,
        color_space,
    );
    surface.inner().view = Some(View { hook });
    Arc::into_raw(Arc::new(surface))
}
