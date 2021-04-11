use std::ffi::CString;
use std::os::raw::{c_char, c_longlong};

use crate::common::context::Context;
use crate::common::context::drawing_text::text_metrics::TextMetrics;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;

pub mod context;
pub mod gl;
pub mod image_asset;
pub mod text_decoder;
pub mod text_encoder;
pub mod path;
pub mod image_data;
pub mod utils;
pub mod matrix;
pub mod paint;
pub mod gradient;
pub mod pattern;
pub mod text_metrics;
pub mod svg;
pub mod image_bitmap;

#[no_mangle]
pub extern "C" fn destroy_string(string: *const c_char) {
    if string.is_null() {
        return;
    }
    unsafe { let _ = CString::from_raw(string as _); }
}


#[no_mangle]
pub extern "C" fn destroy_context(context: c_longlong) {
    if context == 0 {
        return;
    }
    unsafe {
        let context: *mut Context = context as _;
        let _ = Box::from_raw(context);
    }
}


#[no_mangle]
pub extern "C" fn destroy_paint_style(style: c_longlong) {
    if style == 0 {
        return;
    }
    unsafe {
        let style: *mut PaintStyle = style as _;
        let _ = Box::from_raw(style);
    }
}


#[no_mangle]
pub extern "C" fn destroy_text_metrics(metrics: c_longlong) {
    if metrics == 0 {
        return;
    }
    unsafe {
        let metrics: *mut TextMetrics = metrics as _;
        let _ = Box::from_raw(metrics);
    }
}


