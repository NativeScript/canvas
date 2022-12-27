use std::ffi::{c_float, CString};
use std::os::raw::{c_char, c_longlong, c_void};

use crate::common::context::Context;
use crate::common::context::drawing_text::text_metrics::TextMetrics;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;

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
pub extern "C" fn destroy_string(string: *const c_char) {
    if string.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(string as _);
    }
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


#[no_mangle]
pub extern "C" fn gl_snapshot_current_gl_context(width: c_float, height: c_float, _alpha: bool) -> *mut crate::common::ffi::u8_array::U8Array {
    let mut buf = vec![0u8; (width * height * 4.) as usize];

    unsafe {
        gl_bindings::glFlush();
        gl_bindings::glReadPixels(
            0,
            0,
            width as i32,
            height as i32,
            gl_bindings::GL_RGBA as std::os::raw::c_uint,
            gl_bindings::GL_UNSIGNED_BYTE as std::os::raw::c_uint,
            buf.as_mut_ptr() as *mut c_void,
        );
        crate::common::ffi::u8_array::U8Array::from(buf).into_raw()
    }
}