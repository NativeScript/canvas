//! `CanvasRenderingContext2D`, mirroring `canvas2d/CanvasRenderingContext2DImpl.cpp`, and the
//! module functions that create one (`create2DContext`, `create2DContextWithPointer`).

use std::rc::Rc;

use canvas_c::CanvasRenderingContext2D;
use napi::sys;

use crate::util::frame::FrameSlot;
use crate::util::native::{Native, NativeType};

pub struct Context2D {
    pub(crate) context: *mut CanvasRenderingContext2D,
    /// Dirty tracking; flushing renders with `canvas_native_context_render`.
    pub(crate) frame: Rc<FrameSlot>,
}

impl Native for Context2D {
    const KIND: NativeType = NativeType::CanvasRenderingContext2D;
}

impl Context2D {
    /// Renders pending draw calls now; call before anything that reads the canvas's pixels.
    pub fn flush(&self) {
        self.frame.flush_now();
    }
}

pub unsafe fn init(_env: sys::napi_env, _exports: sys::napi_value) {}
