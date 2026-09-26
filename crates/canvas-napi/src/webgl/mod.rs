//! WebGL 1 (`webgl/` in the V8 bindings): `WebGLRenderingContext`, the resource objects, and the
//! extensions. WebGL 2 (`crate::webgl2`) builds on [`base_methods`].

pub mod objects;

use std::rc::Rc;

use canvas_c::WebGLState;
use napi::sys;

use crate::util::class::ClassDef;
use crate::util::frame::FrameSlot;
use crate::util::native::{Native, NativeType};

/// Shared by `WebGLRenderingContext` and `WebGL2RenderingContext` (the V8 bindings'
/// `WebGLRenderingContextBase`), so either can be passed wherever a WebGL context is accepted
/// (`drawImage`, `texImage2D`, `createImageBitmap`).
pub struct WebGLContext {
    pub(crate) state: *mut WebGLState,
    /// 1 or 2.
    pub(crate) version: u8,
    /// Dirty tracking; flushing presents with `canvas_native_webgl_make_current_and_swap_buffers`.
    pub(crate) frame: Rc<FrameSlot>,
}

impl Native for WebGLContext {
    const KIND: NativeType = NativeType::WebGLRenderingContextBase;
}

impl WebGLContext {
    /// Presents pending rendering now; call before reading the drawing buffer from outside GL.
    pub fn flush(&self) {
        self.frame.flush_now();
    }
}

/// Adds every WebGL 1 method and accessor to `def`. `WebGLRenderingContext` is exactly this;
/// `WebGL2RenderingContext` is this plus the WebGL 2 additions.
pub fn base_methods(def: ClassDef) -> ClassDef {
    def
}

pub unsafe fn init(_env: sys::napi_env, _exports: sys::napi_value) {}
