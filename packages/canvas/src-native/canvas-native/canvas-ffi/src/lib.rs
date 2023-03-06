use std::ffi::CString;
use std::os::raw::{c_char, c_longlong};

use canvas_core::context::drawing_text::text_metrics::TextMetrics;
use canvas_core::context::fill_and_stroke_styles::paint::PaintStyle;
use canvas_core::context::{Context, ContextWrapper};

pub mod arrays;
pub mod context;
pub mod gl;
pub mod gradient;
pub mod image_asset;
pub mod image_bitmap;
pub mod image_data;
pub mod matrix;
pub mod paint;
pub mod path;
pub mod pattern;
pub mod svg;
pub mod text_decoder;
pub mod text_encoder;
pub mod text_metrics;
pub mod utils;

#[no_mangle]
pub extern "C" fn canvas_native_destroy_string(string: *const c_char) {
    if string.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(string as _);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_context(context: *mut ContextWrapper) {
    unsafe {
        let _ = Box::from_raw(context);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_paint_style(style: *mut PaintStyle) {
    unsafe {
        let _ = Box::from_raw(style);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_text_metrics(metrics: *mut TextMetrics) {
    unsafe {
        let _ = Box::from_raw(metrics);
    }
}
