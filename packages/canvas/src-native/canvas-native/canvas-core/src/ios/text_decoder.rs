use std::os::raw::{c_char, c_longlong};

use libc::size_t;

use crate::common::{free_text_decoder, text_decoder_decode, text_decoder_get_encoding, TextDecoder};

#[no_mangle]
pub extern "C" fn native_create_text_decoder(decoding: *const c_char) -> c_longlong {
    Box::into_raw(Box::new(TextDecoder::new(decoding))) as i64
}

#[no_mangle]
pub extern "C" fn native_text_decoder_get_encoding(decoder: i64) -> *const c_char {
    text_decoder_get_encoding(decoder)
}

#[no_mangle]
pub extern "C" fn native_text_decoder_decode(
    decoder: i64,
    data: *const u8,
    len: size_t,
) -> *const c_char {
    text_decoder_decode(decoder, data, len)
}

#[no_mangle]
pub extern "C" fn native_text_decoder_decode_u16(
    decoder: i64,
    data: *const u16,
    len: size_t,
) -> *const c_char {
    text_decoder_decode(decoder, data as *const u8, len * 2)
}


#[no_mangle]
pub extern "C" fn native_text_decoder_decode_i16(
    decoder: i64,
    data: *const i16,
    len: size_t,
) -> *const c_char {
    text_decoder_decode(decoder, data as *const u8, len * 2)
}

#[no_mangle]
pub extern "C" fn native_text_decoder_decode_i32(
    decoder: i64,
    data: *const i32,
    len: size_t,
) -> *const c_char {
    text_decoder_decode(decoder, data as *const u8, len * 4)
}

#[no_mangle]
pub extern "C" fn native_text_decoder_free(decoder: i64) {
    free_text_decoder(decoder);
}
