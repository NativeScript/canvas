#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

use std::ffi::CStr;

use jni::objects::{JClass, JString};
use jni::strings::JavaStr;
use jni::JNIEnv;
use jni_sys::{jbyteArray, jlong, jstring};

use crate::common::{text_encoder_encode, text_encoder_get_encoding, NativeByteArray, TextEncoder};

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TextEncoder_nativeInit(
    env: JNIEnv,
    _: JClass,
    encoding: JString,
) -> jlong {
    let empty = env.new_string("").unwrap();
    let value = env
        .get_string(encoding)
        .unwrap_or(JavaStr::from_env(&env, empty).unwrap());
    TextEncoder::new(value.get_raw()).into_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TextEncoder_nativeGetEncoding(
    env: JNIEnv,
    _: JClass,
    encoder: i64,
) -> jstring {
    let encoding = text_encoder_get_encoding(encoder);
    let value = CStr::from_ptr(encoding).to_str().unwrap_or("");
    env.new_string(value).unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TextEncoder_nativeEncode(
    env: JNIEnv,
    _: JClass,
    encoder: i64,
    text: JString,
) -> jbyteArray {
    let empty = env.new_string("").unwrap();
    let string = env
        .get_string(text)
        .unwrap_or(JavaStr::from_env(&env, empty).unwrap());
    let rawArray = NativeByteArray::from_raw(text_encoder_encode(encoder, string.as_ptr()));
    let rawSlice = std::slice::from_raw_parts_mut(rawArray.array, rawArray.length);
    env.byte_array_from_slice(rawSlice).unwrap()
}
