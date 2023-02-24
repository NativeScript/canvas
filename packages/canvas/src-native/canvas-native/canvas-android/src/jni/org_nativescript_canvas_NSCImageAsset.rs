use std::ffi::CStr;

use jni::objects::{JByteArray, JByteBuffer, JClass, JObject, JString, ReleaseMode};
use jni::sys::{jboolean, jint, jlong, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCImageAsset_nativeSave(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    path: JString,
    format: jint,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }
    if let Ok(path) = env.get_string(&path) {
        unsafe {
            let asset: *mut canvas_core::context::image_asset::ImageAsset = asset as _;
            let asset = &mut *asset;
            let path = CStr::from_ptr(path.as_ptr());
            if asset.save_path(
                &path.to_string_lossy(),
                canvas_core::context::image_asset::OutputFormat::from(format),
            ) {
                return JNI_TRUE;
            }
            return JNI_FALSE;
        }
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCImageAsset_nativeLoadFromUrl(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    url: JString,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }
    if let Ok(url) = env.get_string(&url) {
        unsafe {
            let asset: *mut crate::ImageAsset = asset as _;
            let asset = &mut *asset;
            let url = CStr::from_ptr(url.as_ptr());
            if crate::canvas_native_image_asset_load_from_url(asset, &url.to_string_lossy()) {
                return JNI_TRUE;
            }
            return JNI_FALSE;
        }
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCImageAsset_nativeCloneRef(
    _: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jlong {
    if asset == 0 {
        return 0;
    }

    let asset: *mut crate::ImageAsset = asset as _;
    let asset = unsafe { &mut *asset };
    let clone = asset.clone();
    Box::into_raw(Box::new(clone)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCImageAsset_nativeDestroyRef(
    _: JNIEnv,
    _: JClass,
    asset: jlong,
) {
    if asset == 0 {
        return;
    }

    let asset: *mut crate::ImageAsset = asset as _;
    unsafe {
        let _ = Box::from_raw(asset);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCImageAsset_nativeLoadFromPath(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    path: JString,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }
    if let Ok(path) = env.get_string(&path) {
        unsafe {
            let asset: *mut crate::ImageAsset = asset as _;
            let asset = &mut *asset;
            let path = CStr::from_ptr(path.as_ptr());
            if asset.load_from_path(&path.to_string_lossy()) {
                return JNI_TRUE;
            }
            return JNI_FALSE;
        }
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCImageAsset_nativeLoadFromBytes(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    buffer: JByteArray,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    unsafe {
        if let Ok(array) = env.get_array_elements_critical(&buffer, ReleaseMode::NoCopyBack) {
            let size = array.len();
            let bytes = unsafe { std::slice::from_raw_parts_mut(array.as_ptr() as *mut u8, size) };

            let asset: *mut crate::ImageAsset = asset as _;
            let asset = unsafe { &mut *asset };
            if asset.load_from_bytes(bytes) {
                return JNI_TRUE;
            }
        }
    }

    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCImageAsset_nativeLoadFromBuffer(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    buffer: JByteBuffer,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    if let (Ok(buf), Ok(size)) = (
        env.get_direct_buffer_address(&buffer),
        env.get_direct_buffer_capacity(&buffer),
    ) {
        let asset: *mut crate::ImageAsset = asset as _;
        let asset = unsafe { &mut *asset };
        let buf = unsafe { std::slice::from_raw_parts_mut(buf, size) };
        if asset.load_from_bytes(buf) {
            return JNI_TRUE;
        }
    }

    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCImageAsset_nativeLoadFromBitmap(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    bitmap: JObject,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset: *mut crate::ImageAsset = asset as _;
    let asset = unsafe { &mut *asset };

    let mut bm = crate::utils::image::BitmapBytes::new(env, bitmap);

    if let Some(bytes) = bm.data() {
        if asset.load_from_bytes(bytes) {
            return JNI_TRUE;
        }
    }

    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_NSCImageAsset_nativeSetError(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    error: JString,
) {
    if asset == 0 {
        return;
    }
    if let Ok(error) = env.get_string(&error) {
        unsafe {
            let asset: *mut crate::ImageAsset = asset as _;
            let asset = &mut *asset;
            let error = CStr::from_ptr(error.as_ptr());
            asset.set_error(&error.to_string_lossy());
        }
    }
}
