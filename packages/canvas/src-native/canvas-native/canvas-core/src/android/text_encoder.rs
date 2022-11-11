#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jbyteArray, jlong, jobject, jstring};

use crate::common::context::text_encoder::TextEncoder;
use crate::common::prelude::ByteBufMut;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSTextEncoder_nativeInit(
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
pub extern "system" fn Java_org_nativescript_canvas_TNSTextEncoder_nativeDestroy(
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
pub extern "system" fn Java_org_nativescript_canvas_TNSTextEncoder_nativeGetEncoding(
    env: JNIEnv,
    _: JClass,
    encoder: jlong,
) -> jstring {
    unsafe {
        let encoder: *mut TextEncoder = encoder as _;
        let encoder = &mut *encoder;
        env.new_string(encoder.encoding()).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSTextEncoder_nativeEncode(
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


#[inline(always)]
#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSTextEncoder_nativeEncodeToBuffer(
    env: JNIEnv,
    _: JClass,
    encoder: jlong,
    text: JString,
) -> jobject {
    if encoder == 0 {
        return JObject::null().into_raw();
    }
    unsafe {
        if let Ok(text) = env.get_string(text) {
            let text = text.to_string_lossy();
            let encoder: *mut TextEncoder = encoder as _;
            let encoder = &mut *encoder;
            let array = encoder.encode(text.as_ref());
            let buffer = ByteBufMut::from(array);
            let db = env.new_direct_byte_buffer(buffer.data, buffer.len).unwrap();
            let buf = Box::into_raw(Box::new(buffer));
            let db: JValue = db.into();
            crate::android::watch_item(&env, buf as jlong, db);
            return db.l().unwrap().into_raw();
        }
    }
    JObject::null().into_raw()
}

