#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::str::Utf8Error;

use jni::objects::{JByteBuffer, JClass, JString, JValue, ReleaseMode};
use jni::sys::{jbyteArray, jlong, jstring};
use jni::JNIEnv;

use crate::common::context::text_decoder::TextDecoder;
use std::borrow::Borrow;

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextDecoder_nativeInit(
    env: JNIEnv,
    _: JClass,
    decoding: JString,
) -> jlong {
    if let Ok(decoding) = env.get_string(decoding) {
        let decoding = decoding.to_string_lossy();
        Box::into_raw(Box::new(TextDecoder::new(Some(decoding.as_ref())))) as jlong
    } else {
        Box::into_raw(Box::new(TextDecoder::new(None))) as jlong
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextDecoder_nativeDestroy(
    _: JNIEnv,
    _: JClass,
    decoder: jlong,
) {
    if decoder == 0 {
        return;
    }
    unsafe {
        let decoder: *mut TextDecoder = decoder as _;
        let _ = Box::from_raw(decoder);
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextDecoder_nativeGetEncoding(
    env: JNIEnv,
    _: JClass,
    decoder: jlong,
) -> jstring {
    unsafe {
        let decoder: *mut TextDecoder = decoder as _;
        let decoder = &mut *decoder;
        env.new_string(decoder.encoding()).unwrap().into_inner()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextDecoder_nativeDecodeBuffer(
    env: JNIEnv,
    _: JClass,
    decoder: jlong,
    data: JByteBuffer,
) -> jstring {
    return if let Ok(buf) = env.get_direct_buffer_address(data) {
        unsafe {
            let decoder: *mut TextDecoder = decoder as _;
            let decoder = &mut *decoder;
            let decoded = decoder.decode(buf.as_ptr(), buf.len());
            env.new_string(decoded.to_string_lossy())
                .unwrap_or(env.new_string("").unwrap())
                .into_inner()
        }
    } else {
        env.new_string("").unwrap().into_inner()
    };
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextDecoder_nativeDecodeBufferToBytes(
    env: JNIEnv,
    _: JClass,
    decoder: jlong,
    data: JByteBuffer,
) -> jbyteArray {
    return if let Ok(buf) = env.get_direct_buffer_address(data) {
        unsafe {
            let decoder: *mut TextDecoder = decoder as _;
            let decoder = &mut *decoder;
            let buf = decoder.decode_as_bytes(buf.as_ptr(), buf.len());
            env.byte_array_from_slice(buf.as_slice())
                .unwrap_or(env.new_byte_array(0).unwrap())
        }
    } else {
        env.new_byte_array(0).unwrap()
    };
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextDecoder_nativeDecode(
    env: JNIEnv,
    _: JClass,
    decoder: jlong,
    data: jbyteArray,
) -> jstring {
    return if let Ok(val) = env.get_byte_array_elements(data, ReleaseMode::NoCopyBack) {
        let length = val.size().unwrap_or(0) as usize;
        let buf = unsafe {
            std::slice::from_raw_parts(
                std::mem::transmute::<*mut i8, *mut u8>(val.as_ptr()),
                length,
            )
        };
        unsafe {
            let decoder: *mut TextDecoder = decoder as _;
            let decoder = &mut *decoder;
            let decoded = decoder.decode(buf.as_ptr(), buf.len());
            env.new_string(decoded.to_string_lossy())
                .unwrap_or(env.new_string("").unwrap())
                .into_inner()
        }
    } else {
        env.new_string("").unwrap().into_inner()
    };
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextDecoder_nativeDecodeToBytes(
    env: JNIEnv,
    _: JClass,
    decoder: jlong,
    data: jbyteArray,
) -> jbyteArray {
    return if let Ok(val) = env.get_byte_array_elements(data, ReleaseMode::NoCopyBack) {
        let length = val.size().unwrap_or(0) as usize;
        let buf = unsafe {
            std::slice::from_raw_parts(
                std::mem::transmute::<*mut i8, *mut u8>(val.as_ptr()),
                length,
            )
        };
        unsafe {
            let decoder: *mut TextDecoder = decoder as _;
            let decoder = &mut *decoder;
            let buf = decoder.decode_as_bytes(buf.as_ptr(), buf.len());
            env.byte_array_from_slice(buf.as_slice())
                .unwrap_or(env.new_byte_array(0).unwrap())
        }
    } else {
        env.new_byte_array(0).unwrap()
    };
}
