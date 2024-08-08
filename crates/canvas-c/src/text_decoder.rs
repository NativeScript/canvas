use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::buffers::U8Buffer;
use crate::c2d::CCow;

#[derive(Clone)]
pub struct TextDecoder(canvas_2d::context::text_decoder::TextDecoder);

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_release(value: *mut TextDecoder) {
    if value.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(value)) };
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_create(decoding: *const c_char) -> *mut TextDecoder {
    assert!(!decoding.is_null());
    let decoding = unsafe { CStr::from_ptr(decoding) };
    let decoding = decoding.to_string_lossy();
    Box::into_raw(Box::new(TextDecoder(
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
) -> *mut U8Buffer {
    assert!(!decoder.is_null());
    assert!(!data.is_null());
    let decoder = unsafe { &*decoder };

    let bytes = decoder.0.decode_as_bytes(data, size);

    Box::into_raw(Box::new(U8Buffer::new_with_vec(bytes)))
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
