//! `CanvasGradient`, mirroring `canvas2d/CanvasGradient.cpp`.

use canvas_c::PaintStyle;
use napi::sys;

use crate::util::native::{Native, NativeType};

pub struct CanvasGradient {
    pub(crate) style: *mut PaintStyle,
}

impl Native for CanvasGradient {
    const KIND: NativeType = NativeType::CanvasGradient;
}

impl Drop for CanvasGradient {
    fn drop(&mut self) {
        canvas_c::canvas_native_paint_style_release(self.style);
    }
}

pub unsafe fn init(_env: sys::napi_env, _exports: sys::napi_value) {}
