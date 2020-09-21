#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

use std::ffi::CStr;

use jni::objects::{JClass, JString};
use jni::strings::JavaStr;
use jni::JNIEnv;
use jni_sys::{jbyteArray, jintArray, jlong, jshortArray, jstring};

use crate::common::{text_decoder_decode, text_decoder_get_encoding, TextDecoder};

#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TextDecoder_nativeInit(
    env: JNIEnv,
    _: JClass,
    decoding: JString,
) -> jlong {
    let empty = env.new_string("").unwrap();
    let value = env
        .get_string(decoding)
        .unwrap_or(JavaStr::from_env(&env, empty).unwrap());
    Box::into_raw(Box::new(TextDecoder::new(value.get_raw()))) as i64
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TextDecoder_nativeGetEncoding(
    env: JNIEnv,
    _: JClass,
    decoder: i64,
) -> jstring {
    let encoding = text_decoder_get_encoding(decoder);
    let value = CStr::from_ptr(encoding).to_str().unwrap_or("");
    env.new_string(value).unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TextDecoder_nativeDecode(
    env: JNIEnv,
    _: JClass,
    decoder: i64,
    data: jbyteArray,
) -> jstring {
    let len = env.get_array_length(data).unwrap_or(0);
    let mut rawData = vec![0i8; len as usize];
    let _ = env.get_byte_array_region(data, 0, rawData.as_mut_slice());
    let decoded = text_decoder_decode(decoder, rawData.as_ptr() as *const u8, rawData.len());
    let value = CStr::from_ptr(decoded).to_str().unwrap_or("");
    env.new_string(value).unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TextDecoder_nativeDecodeShort(
    env: JNIEnv,
    _: JClass,
    decoder: i64,
    data: jshortArray,
) -> jstring {
    let len = env.get_array_length(data).unwrap_or(0);
    let mut rawData = vec![0i16; len as usize];
    let _ = env.get_short_array_region(data, 0, rawData.as_mut_slice());
    let decoded = text_decoder_decode(decoder, rawData.as_ptr() as *const u8, rawData.len() * 2);
    let value = CStr::from_ptr(decoded).to_str().unwrap_or("");
    env.new_string(value).unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TextDecoder_nativeDecodeInt(
    env: JNIEnv,
    _: JClass,
    decoder: i64,
    data: jintArray,
) -> jstring {
    let len = env.get_array_length(data).unwrap_or(0);
    let mut rawData = vec![0i32; len as usize];
    let _ = env.get_int_array_region(data, 0, rawData.as_mut_slice());
    let decoded = text_decoder_decode(decoder, rawData.as_ptr() as *const u8, rawData.len() * 4);
    let value = CStr::from_ptr(decoded).to_str().unwrap_or("");
    env.new_string(value).unwrap().into_inner()
}
