use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Arc;

use crate::buffers::U8Buffer;
use crate::canvas2d::string::CCow;

#[derive(Clone)]
pub struct TextDecoder(canvas_2d::context::text_decoder::TextDecoder);

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_reference(value: *const TextDecoder) {
    if value.is_null() {
        return;
    }
    unsafe { Arc::increment_strong_count(value) };
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_release(value: *const TextDecoder) {
    if value.is_null() {
        return;
    }
    unsafe { Arc::decrement_strong_count(value) };
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_create(decoding: *const c_char) -> *const TextDecoder {
    assert!(!decoding.is_null());
    let decoding = unsafe { CStr::from_ptr(decoding) };
    let decoding = decoding.to_string_lossy();
    Arc::into_raw(Arc::new(TextDecoder(
        canvas_2d::context::text_decoder::TextDecoder::new(Some(decoding.as_ref())),
    )))
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode(
    decoder: *const TextDecoder,
    data: *const u8,
    size: usize,
) -> *const c_char {
    assert!(!decoder.is_null());
    assert!(!data.is_null());
    let decoder = unsafe { &*decoder };

    decoder.0.decode(data, size).into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_as_cow(
    decoder: *const TextDecoder,
    data: *const u8,
    size: usize,
) -> *mut CCow {
    assert!(!decoder.is_null());
    assert!(!data.is_null());
    let decoder = unsafe { &*decoder };

    let bytes = decoder.0.decode_as_cow(data, size);

    Box::into_raw(Box::new(CCow(bytes)))
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_as_bytes(
    decoder: *const TextDecoder,
    data: *const u8,
    size: usize,
) -> *const U8Buffer {
    assert!(!decoder.is_null());
    assert!(!data.is_null());
    let decoder = unsafe { &*decoder };

    let bytes = decoder.0.decode_as_bytes(data, size);

    Arc::into_raw(Arc::new(U8Buffer::new_with_vec(bytes)))
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_c_string(
    decoder: *const TextDecoder,
    data: *const c_char,
) -> *const c_char {
    assert!(!decoder.is_null());
    assert!(!data.is_null());
    let decoder = unsafe { &*decoder };

    decoder.0.decode_c_string(data).into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_get_encoding(
    decoder: *const TextDecoder,
) -> *const c_char {
    assert!(!decoder.is_null());
    let decoder = unsafe { &*decoder };
    CString::new(decoder.0.encoding().to_string().to_lowercase())
        .unwrap()
        .into_raw()
}
