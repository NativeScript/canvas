#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jbyteArray, jint, jlong, JNI_FALSE, JNI_TRUE, jobject, jstring};

use crate::common::context::image_asset::{ImageAsset, OutputFormat};

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeInit(
    _: JNIEnv,
    _: JClass,
) -> jlong {
    Box::into_raw(Box::new(ImageAsset::new())) as jlong
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeGetBytes(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jbyteArray {
    if asset == 0 {
        return env.new_byte_array(0).unwrap();
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        let bytes = asset.bytes_internal();
        if let Ok(array) = env.byte_array_from_slice(bytes.as_slice()) {
            array
        } else {
            env.new_byte_array(0).unwrap()
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeGetWidth(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jint {
    if asset == 0 {
        return 0;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        asset.width() as i32
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeGetHeight(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jint {
    if asset == 0 {
        return 0;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        asset.height() as i32
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeScale(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
    x: jint,
    y: jint,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        if asset.scale(x as u32, y as u32) {
            return JNI_TRUE;
        }
        JNI_FALSE
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeFlipX(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        if asset.flip_x() {
            return JNI_TRUE;
        }
        JNI_FALSE
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeFlipY(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        if asset.flip_y() {
            return JNI_TRUE;
        }
        JNI_FALSE
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeSave(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    path: JString,
    format: jint,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }
    if let Ok(path) = env.get_string(path) {
        unsafe {
            let asset: *mut ImageAsset = asset as _;
            let asset = &mut *asset;
            if asset.save_path(path.as_ptr(), OutputFormat::from(format)) {
                return JNI_TRUE;
            }
            return JNI_FALSE;
        }
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeGetError(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jstring {
    if asset == 0 {
        return env.new_string("").unwrap().into_inner();
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        if let Ok(error) = env.new_string(&asset.error) {
            return error.into_inner();
        }
        env.new_string("").unwrap().into_inner()
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeHasError(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        if asset.error.is_empty() {
            return JNI_FALSE;
        }
        JNI_TRUE
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeDestroy(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) {
    if asset == 0 {
        return;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let _ = Box::from_raw(asset);
    }
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeLoadAssetPath(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    path: JString,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }
    if let Ok(path) = env.get_string(path) {
        unsafe {
            let asset: *mut ImageAsset = asset as _;
            let asset = &mut *asset;
            if asset.load_from_path(path.as_ptr()) {
                return JNI_TRUE;
            }
            return JNI_FALSE;
        }
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "C" fn Java_org_nativescript_canvas_TNSImageAsset_nativeLoadAssetBytes(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    buffer: jbyteArray,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }
    if let Ok(size) = env.get_array_length(buffer) {
        let mut buf = vec![0u8; size as usize];
        unsafe {
            if let Ok(_) =
            env.get_byte_array_region(buffer, 0, std::mem::transmute(buf.as_mut_slice()))
            {
                let asset: *mut ImageAsset = asset as _;
                let asset = &mut *asset;
                if asset.load_from_bytes(buf.as_slice()) {
                    return JNI_TRUE;
                }
            }
        }
    }
    JNI_FALSE
}
