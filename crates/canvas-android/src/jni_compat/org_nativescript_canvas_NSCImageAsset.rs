use jni::objects::{JByteArray, JByteBuffer, JClass, JIntArray, JObject, JString, ReleaseMode};
use jni::sys::{jboolean, jlong, jobject, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use ndk::bitmap::BitmapFormat;

use canvas_c::{canvas_native_image_asset_release, ImageAsset};

#[no_mangle]
pub extern "system" fn nativeCreateImageAsset(_env: JNIEnv, _: JClass) -> jlong {
    Box::into_raw(Box::new(ImageAsset::default())) as jlong
}

#[no_mangle]
pub extern "system" fn nativeDestroyImageAsset(_env: JNIEnv, _: JClass, asset: jlong) {
    if asset == 0 {
        return;
    }

    let asset = asset as *const ImageAsset;

    canvas_native_image_asset_release(asset);
}

#[no_mangle]
pub extern "system" fn nativeLoadFromBitmap(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    bitmap: JObject,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *const ImageAsset;
    let asset = unsafe { &*asset };

    let bytes = crate::utils::image::get_bytes_from_bitmap(&env, bitmap);

    if let Some((image_data, info)) = bytes {
        if match info.format() {
            BitmapFormat::NONE => false,
            BitmapFormat::RGBA_8888 => {
                asset.load_from_raw_bytes(info.width(), info.height(), 4, image_data)
            }
            BitmapFormat::RGB_565 => {
                let image =
                    canvas_core::image_asset::ImageAsset::rgb565_to_rgba8888(image_data.as_slice());
                asset.load_from_raw_bytes(info.width(), info.height(), 4, image)
            }
            #[allow(deprecated)]
            BitmapFormat::RGBA_4444 => false,
            BitmapFormat::A_8 => false,
            BitmapFormat::RGBA_F16 => false,
        } {
            return JNI_TRUE;
        }
    }

    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn nativeLoadFromPath(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    path: JString,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *const ImageAsset;
    let asset = unsafe { &*asset };

    match env.get_string(&path) {
        Ok(path) => {
            let path = path.to_string_lossy();
            if asset.load_from_path(path.as_ref()) {
                return JNI_TRUE;
            }
            JNI_FALSE
        }
        Err(error) => {
            let error = error.to_string();
            asset.set_error(error.as_str());
            JNI_FALSE
        }
    }
}

#[no_mangle]
pub extern "system" fn nativeLoadFromUrl(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    url: JString,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *const ImageAsset;

    match env.get_string(&url) {
        Ok(path) => {
            if canvas_c::canvas_native_image_asset_load_from_url(asset, path.as_ptr()) {
                return JNI_TRUE;
            }
            JNI_FALSE
        }
        Err(error) => {
            let asset = unsafe {  &*asset };
            let error = error.to_string();
            asset.set_error(error.as_str());
            JNI_FALSE
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn nativeLoadFromBytes(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    byteArray: JByteArray,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *const ImageAsset;

    match env.get_array_elements_critical(&byteArray, ReleaseMode::NoCopyBack) {
        Ok(bytes) => {
            let size = bytes.len();
            let slice = std::slice::from_raw_parts_mut(bytes.as_ptr() as *mut u8, size);
            let asset = unsafe { &*asset };
            if asset.load_from_bytes(slice) {
                return JNI_TRUE;
            }
            JNI_FALSE
        }
        Err(_) => JNI_FALSE,
    }
}

#[no_mangle]
pub extern "system" fn nativeLoadFromBuffer(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    buffer: JByteBuffer,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *const ImageAsset;

    if let (Ok(buf), Ok(size)) = (
        env.get_direct_buffer_address(&buffer),
        env.get_direct_buffer_capacity(&buffer),
    ) {
        let slice = unsafe { std::slice::from_raw_parts(buf, size) };
        let asset = unsafe { &*asset };
        if asset.load_from_bytes(slice) {
            return JNI_TRUE;
        }
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn nativeGetDimensions(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    dimensions: JIntArray,
) {
    if asset == 0 {
        return;
    }

    let asset = asset as *const ImageAsset;
    let asset = unsafe { &*asset };

    let (width, height) = asset.dimensions();
    let dim = [width as i32, height as i32];
    let _ = env.set_int_array_region(&dimensions, 0, dim.as_slice());
}

#[no_mangle]
pub extern "system" fn nativeGetError(env: JNIEnv, _: JClass, asset: jlong) -> jobject {
    if asset == 0 {
        return JObject::null().into_raw();
    }

    let asset = asset as *const ImageAsset;
    let asset = unsafe { &*asset };

    let error = asset.error();
    if error.is_empty() {
        return JObject::null().into_raw();
    }

    env.new_string(error.to_string()).unwrap().into_raw()
}
