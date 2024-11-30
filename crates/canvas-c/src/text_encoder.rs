use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::buffers::U8Buffer;

#[derive(Clone)]
pub struct TextEncoder(canvas_2d::context::text_encoder::TextEncoder);
impl TextEncoder {
    pub fn encoding(&self) -> &str {
        self.0.encoding()
    }

    pub fn encode(&self, value: &str) -> Vec<u8> {
        self.0.encode(value)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_encoder_release(value: *mut TextEncoder) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}

#[no_mangle]
pub extern "C" fn canvas_native_text_encoder_create(encoding: *const c_char) -> *mut TextEncoder {
    assert!(!encoding.is_null());
    let encoding = unsafe { CStr::from_ptr(encoding) };
    let encoding = encoding.to_string_lossy();

    Box::into_raw(Box::new(TextEncoder(
        canvas_2d::context::text_encoder::TextEncoder::new(Some(encoding.as_ref())),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_text_encoder_encode(
    encoder: *const TextEncoder,
    text: *const c_char,
) -> *mut U8Buffer {
    assert!(!encoder.is_null());
    assert!(!text.is_null());
    let encoder = unsafe { &*encoder };
    let text = unsafe { CStr::from_ptr(text) };
    let text = text.to_string_lossy();
    let encoded = encoder.0.encode(text.as_ref());
    Box::into_raw(Box::new(U8Buffer::from(encoded)))
}

#[no_mangle]
pub extern "C" fn canvas_native_text_encoder_get_encoding(
    encoder: *const TextEncoder,
) -> *const c_char {
    assert!(!encoder.is_null());
    let encoder = unsafe { &*encoder };
    CString::new(encoder.0.encoding().to_string().to_lowercase())
        .unwrap()
        .into_raw()
}
