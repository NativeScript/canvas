use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_longlong};

use crate::common::context::text_decoder::TextDecoder;

#[no_mangle]
pub extern "C" fn text_decoder_create(decoding: *const c_char) -> c_longlong {
    unsafe {
        if decoding.is_null() {
            Box::into_raw(Box::new(TextDecoder::new(None))) as c_longlong
        } else {
            let decoding = CStr::from_ptr(decoding).to_string_lossy();
            Box::into_raw(Box::new(TextDecoder::new(Some(decoding.as_ref())))) as c_longlong
        }
    }
}

#[no_mangle]
pub extern "C" fn text_decoder_get_encoding(decoder: c_longlong) -> *const c_char {
    unsafe {
        let decoder: *mut TextDecoder = decoder as _;
        let decoder = &mut *decoder;
        CString::new(decoder.encoding()).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn text_decoder_decode(
    decoder: c_longlong,
    data: *const u8,
    len: usize,
) -> *const c_char {
    unsafe {
        let decoder: *mut TextDecoder = decoder as _;
        let decoder = &mut *decoder;
        let decoded = decoder.decode(data, len);
        decoded.into_raw()
    }
}

#[no_mangle]
pub extern "C" fn text_decoder_decode_u16(
    decoder: c_longlong,
    data: *const u16,
    len: usize,
) -> *const c_char {
    text_decoder_decode(decoder, data as *const u8, len * 2)
}

#[no_mangle]
pub extern "C" fn text_decoder_decode_i16(
    decoder: c_longlong,
    data: *const i16,
    len: usize,
) -> *const c_char {
    text_decoder_decode(decoder, data as *const u8, len * 2)
}

#[no_mangle]
pub extern "C" fn text_decoder_decode_i32(
    decoder: c_longlong,
    data: *const i32,
    len: usize,
) -> *const c_char {
    text_decoder_decode(decoder, data as *const u8, len * 4)
}

#[no_mangle]
pub extern "C" fn destroy_text_decoder(
    decoder: c_longlong,
) {
    if decoder == 0 {
        return;
    }
    unsafe {
        let decoder: *mut TextDecoder = decoder as _;
        let _ = Box::from_raw(decoder);
    }
}
