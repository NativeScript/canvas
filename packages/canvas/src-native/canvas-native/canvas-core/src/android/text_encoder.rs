#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jbyteArray, jlong, jobject, jstring};

use crate::common::context::text_encoder::TextEncoder;

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextEncoder_nativeInit(
    env: JNIEnv,
    _: JClass,
    encoding: JString,
) -> jlong {
    if let Ok(encoding) = env.get_string(encoding) {
        let encoding = encoding.to_string_lossy();
        Box::into_raw(Box::new(TextEncoder::new(Some(encoding.as_ref())))) as jlong
    } else {
        Box::into_raw(Box::new(TextEncoder::new(None))) as jlong
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextEncoder_nativeDestroy(
    _: JNIEnv,
    _: JClass,
    encoder: jlong,
) {
    if encoder == 0 {
        return;
    }
    unsafe {
        let encoder: *mut TextEncoder = encoder as _;
        let _ = Box::from_raw(encoder);
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextEncoder_nativeGetEncoding(
    env: JNIEnv,
    _: JClass,
    encoder: jlong,
) -> jstring {
    unsafe {
        let encoder: *mut TextEncoder = encoder as _;
        let encoder = &mut *encoder;
        env.new_string(encoder.encoding()).unwrap().into_inner()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSTextEncoder_nativeEncode(
    env: JNIEnv,
    _: JClass,
    encoder: jlong,
    text: JString,
) -> jbyteArray {
    if encoder == 0 {
        return env.new_byte_array(0).unwrap();
    }
    unsafe {
        if let Ok(text) = env.get_string(text) {
            let text = text.to_string_lossy();
            let encoder: *mut TextEncoder = encoder as _;
            let encoder = &mut *encoder;
            let array = encoder.encode(text.as_ref());
            return env
                .byte_array_from_slice(array.as_slice())
                .unwrap_or(env.new_byte_array(0).unwrap());
        }
    }
    env.new_byte_array(0).unwrap()
}
