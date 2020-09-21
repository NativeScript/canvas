use std::os::raw::{c_char, c_longlong};

use crate::common::{free_text_encoder, NativeByteArray, text_encoder_encode, text_encoder_get_encoding, TextEncoder};

#[no_mangle]
pub extern "C" fn native_create_text_encoder(encoding: *const c_char) -> c_longlong {
    Box::into_raw(Box::new(TextEncoder::new(encoding))) as i64
}

#[no_mangle]
pub extern "C" fn native_text_encoder_get_encoding(encoder: i64) -> *const c_char {
    text_encoder_get_encoding(encoder)
}

#[no_mangle]
pub extern "C" fn native_text_encoder_encode(encoder: i64, text: *const c_char) -> *mut NativeByteArray {
    text_encoder_encode(encoder, text)
}

#[no_mangle]
pub extern "C" fn native_text_encoder_free(encoder: i64) {
    free_text_encoder(encoder);
}
