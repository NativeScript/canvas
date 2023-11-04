use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_longlong};

use canvas_core::context::text_encoder::TextEncoder;
use canvas_core::ffi::u8_array::U8Array;

#[no_mangle]
pub extern "C" fn canvas_native_text_encoder_create(encoding: *const c_char) -> *mut TextEncoder {
    unsafe {
        if encoding.is_null() {
            Box::into_raw(Box::new(TextEncoder::new(None)))
        } else {
            let encoding = CStr::from_ptr(encoding).to_string_lossy();
            Box::into_raw(Box::new(TextEncoder::new(Some(encoding.as_ref()))))
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_encoder_get_encoding(
    encoder: *mut TextEncoder,
) -> *const c_char {
    assert!(!encoder.is_null());
    unsafe {
        let encoder = &mut *encoder;
        CString::new(encoder.encoding()).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_encoder_encode(
    encoder: *mut TextEncoder,
    text: *const c_char,
) -> *mut U8Array {
    assert!(!encoder.is_null());
    assert!(!text.is_null());
    unsafe {
        let encoder = &mut *encoder;
        let text = CStr::from_ptr(text).to_string_lossy();
        let array: U8Array = encoder.encode(text.as_ref()).into();
        Box::into_raw(Box::new(array))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_text_encoder(encoder: *mut TextEncoder) {
    if encoder.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(encoder);
    }
}
