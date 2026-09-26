//! `CanvasPattern`, mirroring `canvas2d/CanvasPattern.cpp`.

use canvas_c::PaintStyle;
use napi::sys;

use crate::util::native::{Native, NativeType};

pub struct CanvasPattern {
    pub(crate) style: *mut PaintStyle,
}

impl Native for CanvasPattern {
    const KIND: NativeType = NativeType::CanvasPattern;
}

impl Drop for CanvasPattern {
    fn drop(&mut self) {
        canvas_c::canvas_native_paint_style_release(self.style);
    }
}

pub unsafe fn init(_env: sys::napi_env, _exports: sys::napi_value) {}
