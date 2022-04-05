use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_longlong};

use canvas_core::context::text_decoder::TextDecoder;
use canvas_core::ffi::u8_array::U8Array;

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_create(decoding: *const c_char) -> *mut TextDecoder {
    unsafe {
        if decoding.is_null() {
            Box::into_raw(Box::new(TextDecoder::new(None)))
        } else {
            let decoding = CStr::from_ptr(decoding).to_string_lossy();
            Box::into_raw(Box::new(TextDecoder::new(Some(decoding.as_ref()))))
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_get_encoding(
    decoder: *mut TextDecoder,
) -> *const c_char {
    assert!(!decoder.is_null());
    unsafe {
        let decoder = &mut *decoder;
        CString::new(decoder.encoding()).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode(
    decoder: *mut TextDecoder,
    data: *const u8,
    len: usize,
) -> *const c_char {
    assert!(!decoder.is_null());
    unsafe {
        let decoder = &mut *decoder;
        let decoded = decoder.decode(data, len);
        decoded.into_raw()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_to_bytes(
    decoder: *mut TextDecoder,
    data: *const u8,
    len: usize,
) -> *const U8Array {
    assert!(!decoder.is_null());
    unsafe {
        let decoder = &mut *decoder;
        let decoded: U8Array = decoder.decode_as_bytes(data, len).into();
        Box::into_raw(Box::new(decoded))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_u16(
    decoder: *mut TextDecoder,
    data: *const u16,
    len: usize,
) -> *const c_char {
    canvas_native_text_decoder_decode(decoder, data as *const u8, len * 2)
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_i16(
    decoder: *mut TextDecoder,
    data: *const i16,
    len: usize,
) -> *const c_char {
    canvas_native_text_decoder_decode(decoder, data as *const u8, len * 2)
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_i32(
    decoder: *mut TextDecoder,
    data: *const i32,
    len: usize,
) -> *const c_char {
    canvas_native_text_decoder_decode(decoder, data as *const u8, len * 4)
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_text_decoder(decoder: *mut TextDecoder) {
    if decoder.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(decoder);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_bytes(
    decoder: *mut TextDecoder,
    data: *const u8,
    len: usize,
) -> *mut U8Array {
    assert!(!decoder.is_null());
    unsafe {
        let decoder = &mut *decoder;
        let decoded: U8Array = decoder.decode_as_bytes(data, len).into();
        Box::into_raw(Box::new(decoded))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_u16_bytes(
    decoder: *mut TextDecoder,
    data: *const u16,
    len: usize,
) -> *mut U8Array {
    canvas_native_text_decoder_decode_bytes(decoder, data as *const u8, len * 2)
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_i16_bytes(
    decoder: *mut TextDecoder,
    data: *const i16,
    len: usize,
) -> *mut U8Array {
    canvas_native_text_decoder_decode_bytes(decoder, data as *const u8, len * 2)
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_i32_bytes(
    decoder: *mut TextDecoder,
    data: *const i32,
    len: usize,
) -> *mut U8Array {
    canvas_native_text_decoder_decode_bytes(decoder, data as *const u8, len * 4)
}

#[no_mangle]
pub extern "C" fn canvas_native_text_decoder_decode_u32_bytes(
    decoder: *mut TextDecoder,
    data: *const u32,
    len: usize,
) -> *mut U8Array {
    canvas_native_text_decoder_decode_bytes(decoder, data as *const u8, len * 4)
}
