use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_longlong};

use crate::common::context::text_encoder::TextEncoder;
use crate::common::ffi::u8_array::U8Array;

#[no_mangle]
pub extern "C" fn text_encoder_create(encoding: *const c_char) -> c_longlong {
    unsafe {
        if encoding.is_null() {
            Box::into_raw(Box::new(TextEncoder::new(None))) as c_longlong
        } else {
            let encoding = CStr::from_ptr(encoding).to_string_lossy();
            Box::into_raw(Box::new(TextEncoder::new(Some(encoding.as_ref())))) as c_longlong
        }
    }
}

#[no_mangle]
pub extern "C" fn text_encoder_get_encoding(encoder: c_longlong) -> *const c_char {
    if encoder == 0 {
        return std::ptr::null();
    }
    unsafe {
        let encoder: *mut TextEncoder = encoder as _;
        let encoder = &mut *encoder;
        CString::new(encoder.encoding()).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn text_encoder_encode(encoder: c_longlong, text: *const c_char) -> *mut U8Array {
    if encoder == 0 {
        return std::ptr::null_mut();
    }
    unsafe {
        let encoder: *mut TextEncoder = encoder as _;
        let encoder = &mut *encoder;
        let text = CStr::from_ptr(text).to_string_lossy();
        let array: U8Array = encoder.encode(text.as_ref()).into();
        Box::into_raw(Box::new(array))
    }
}

#[no_mangle]
pub extern "C" fn destroy_text_encoder(encoder: c_longlong) {
    if encoder == 0 {
        return;
    }
    unsafe {
        let encoder: *mut TextEncoder = encoder as _;
        let _ = Box::from_raw(encoder);
    }
}
