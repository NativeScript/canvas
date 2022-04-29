#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::convert::TryInto;
use std::ffi::c_void;

use jni::objects::{JClass, JObject, JString, ReleaseMode};
use jni::sys::{jboolean, jbyteArray, jint, jlong, jstring, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;

use canvas_core::context::image_asset::{ImageAsset, OutputFormat};

use crate::utils::bitmap::AndroidBitmapInfo;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeInit(
    _: JNIEnv,
    _: JClass,
) -> jlong {
    Box::into_raw(Box::new(ImageAsset::new())) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeGetBytes(
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
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeGetWidth(
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
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeGetHeight(
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
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeScale(
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
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeFlipX(
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
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeFlipY(
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
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeSave(
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
            if asset.save_path_raw(path.as_ptr(), OutputFormat::from(format)) {
                return JNI_TRUE;
            }
            return JNI_FALSE;
        }
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeGetError(
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
        if let Ok(error) = env.new_string(asset.error()) {
            return error.into_inner();
        }
        env.new_string("").unwrap().into_inner()
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeHasError(
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
        if asset.error().is_empty() {
            return JNI_FALSE;
        }
        JNI_TRUE
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeDestroy(
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
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeLoadAssetPath(
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
            if asset.load_from_path_raw(path.as_ptr()) {
                return JNI_TRUE;
            }
            return JNI_FALSE;
        }
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeLoadAssetBytes(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    buffer: jbyteArray,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }
    unsafe {
        if let Ok(array) = env.get_primitive_array_critical(buffer, ReleaseMode::NoCopyBack) {
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                array.size().unwrap_or_default().try_into().unwrap(),
            );
            let asset: *mut ImageAsset = asset as _;
            let asset = &mut *asset;
            if asset.load_from_bytes(buf) {
                return JNI_TRUE;
            }
        }
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSImageAsset_nativeCopyToBitmap(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    bitmap: JObject,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let native_interface = env.get_native_interface();
    let bitmap = bitmap.into_inner();
    let mut bitmap_info = std::mem::MaybeUninit::uninit();

    unsafe {
        if crate::utils::bitmap::AndroidBitmap_getInfo(
            native_interface as _,
            bitmap,
            bitmap_info.as_mut_ptr(),
        ) < crate::utils::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            log::debug!("Get Bitmap Info Failed");
            let info = AndroidBitmapInfo::default();
            return JNI_FALSE;
        }
    }
    let bitmap_info = unsafe { bitmap_info.assume_init() };
    let mut pixels = std::ptr::null_mut() as *mut c_void;
    let pixels_ptr: *mut *mut c_void = &mut pixels;
    unsafe {
        if crate::utils::bitmap::AndroidBitmap_lockPixels(native_interface as _, bitmap, pixels_ptr)
            < crate::utils::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            log::debug!("Get Bitmap Lock Failed");
            return JNI_FALSE;
        }
    }
    let length = (bitmap_info.height * bitmap_info.stride as u32) as usize;
    let slice: &mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, length as usize) };

    let asset: *mut ImageAsset = asset as _;
    let asset = unsafe { &mut *asset };

    slice.copy_from_slice(asset.rgba_internal_bytes().as_mut_slice());

    unsafe {
        if crate::utils::bitmap::AndroidBitmap_unlockPixels(native_interface as _, bitmap)
            < crate::utils::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            log::debug!("Unlock Bitmap Failed");
        }
    }
    return JNI_TRUE;
}
