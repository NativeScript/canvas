//! `TextMetrics`, mirroring `canvas2d/TextMetricsImpl.cpp`.

use napi::sys;

use crate::util::native::{Native, NativeType};

pub struct TextMetrics {
    pub(crate) metrics: *mut canvas_c::TextMetrics,
}

impl Native for TextMetrics {
    const KIND: NativeType = NativeType::TextMetrics;
}

impl Drop for TextMetrics {
    fn drop(&mut self) {
        canvas_c::canvas_native_text_metrics_release(self.metrics);
    }
}

pub unsafe fn init(_env: sys::napi_env, _exports: sys::napi_value) {}
