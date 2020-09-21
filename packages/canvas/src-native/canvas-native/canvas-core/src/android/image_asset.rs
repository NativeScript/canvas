#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

use std::ffi::CStr;

use jni::{
    objects::{JClass, JString},
    JNIEnv,
};
use jni_sys::{jboolean, jbyteArray, jint, jlong, jobject, jstring, JNI_FALSE, JNI_TRUE};

use crate::common::{
    create_image_asset, image_asset_flip_x, image_asset_flip_y, image_asset_free_bytes,
    image_asset_get_bytes, image_asset_get_error, image_asset_has_error, image_asset_height,
    image_asset_load_from_path, image_asset_load_from_slice_i8, image_asset_release,
    image_asset_save_path, image_asset_scale, image_asset_width,
};
use jni::objects::JByteBuffer;
use std::mem::transmute;

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeInit(
    _env: JNIEnv,
    _: JClass,
) -> jlong {
    create_image_asset()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeGetBytes(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jbyteArray {
    let array = image_asset_get_bytes(asset);
    let bytes = std::slice::from_raw_parts_mut(array.array, array.length);
    let result = env
        .new_byte_array(array.length as i32)
        .unwrap_or(env.new_byte_array(0).unwrap());
    let _ = env.get_byte_array_region(result, 0, transmute(bytes));
    image_asset_free_bytes(array);
    result
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeGetBuffer(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jobject {
    let array = image_asset_get_bytes(asset);
    let bytes = std::slice::from_raw_parts_mut(array.array, array.length);
    let result = env.new_direct_byte_buffer(bytes).unwrap();
    image_asset_free_bytes(array);
    result.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeGetWidth(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jint {
    image_asset_width(asset) as i32
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeGetHeight(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jint {
    image_asset_height(asset) as i32
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeScale(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
    x: jint,
    y: jint,
) -> jlong {
    image_asset_scale(asset, x as u32, y as u32)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeFlipX(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jlong {
    image_asset_flip_x(asset)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeSave(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    path: JString,
    format: jint,
) -> jboolean {
    let real_path = env.get_string(path).unwrap();
    image_asset_save_path(asset, real_path.get_raw(), format as u32) as u8
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeFlipY(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jlong {
    image_asset_flip_y(asset)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeGetError(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jstring {
    let error = image_asset_get_error(asset);
    let string = CStr::from_ptr(error).to_str();
    let string = string.unwrap_or("");
    env.new_string(string).unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeHasError(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jboolean {
    if image_asset_has_error(asset) == 0 {
        return JNI_FALSE;
    }
    return JNI_TRUE;
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeRelease(
    _env: JNIEnv,
    _: JClass,
    asset: jlong,
) {
    image_asset_release(asset)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeLoadAssetPath(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    path: JString,
) -> jboolean {
    let real_path = env.get_string(path).unwrap();
    image_asset_load_from_path(asset, real_path.get_raw()) as u8
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeLoadAssetBuffer(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    buffer: JByteBuffer,
) -> jboolean {
    let size = env.get_direct_buffer_address(buffer).unwrap();
    image_asset_load_from_slice_i8(asset, transmute(size)) as u8
}
#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_ImageAsset_nativeLoadAssetBytes(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    buffer: jbyteArray,
) -> jboolean {
    let size = env.get_array_length(buffer).unwrap_or(0);
    let mut buf = vec![0i8; size as usize];
    let _ = env.get_byte_array_region(buffer, 0, buf.as_mut_slice());
    image_asset_load_from_slice_i8(asset, buf.as_mut_slice()) as u8
}
