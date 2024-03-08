use jni::JNIEnv;
use jni::objects::{JClass, JIntArray, JObject, JString};
use jni::sys::{jboolean, jlong, JNI_FALSE, JNI_TRUE, jobject};
use ndk::bitmap::BitmapFormat;

use canvas_c::ImageAsset;

#[no_mangle]
pub extern "system" fn nativeCreateImageAsset(_env: JNIEnv, _: JClass) -> jlong {
    Box::into_raw(Box::new(ImageAsset::default())) as jlong
}

#[no_mangle]
pub extern "system" fn nativeDestroyImageAsset(_env: JNIEnv, _: JClass, asset: jlong) {
    if asset == 0 {
        return;
    }

    let asset = asset as *mut ImageAsset;
    let _ = unsafe { Box::from_raw(asset) };
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

    let asset = asset as *mut ImageAsset;
    let asset = unsafe { &mut *asset };

    let bytes = crate::utils::image::get_bytes_from_bitmap(&env, bitmap);

    if let Some((image_data, info)) = bytes {
        if match info.format() {
            BitmapFormat::NONE => {
                false
            }
            BitmapFormat::RGBA_8888 => {
                asset.load_from_raw_bytes(info.width(), info.height(), 4, image_data)
            }
            BitmapFormat::RGB_565 => {
                let image = canvas_core::image_asset::ImageAsset::rgb565_to_rgba8888(image_data.as_slice());
                asset.load_from_raw_bytes(info.width(), info.height(), 4, image)
            }
            BitmapFormat::RGBA_4444 => {
               false
            }
            BitmapFormat::A_8 => {
                false
            }
            BitmapFormat::RGBA_F16 => {
                false
            }
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

    let asset = asset as *mut ImageAsset;
    let asset = unsafe { &mut *asset };

    match env.get_string(&path) {
        Ok(path) => {
            let path: String =  path.into();
            if asset.load_from_path(path.as_str()) {
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
pub extern "system" fn nativeGetDimensions(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    dimensions: JIntArray,
) {
    if asset == 0 {
        return;
    }

    let asset = asset as *mut ImageAsset;
    let asset = unsafe { &mut *asset };

    let (width, height) = asset.dimensions();
    let dim = [width as i32, height as i32];
    let _ = env.set_int_array_region(&dimensions, 0, dim.as_slice());
}

#[no_mangle]
pub extern "system" fn nativeGetError(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
) -> jobject {
    if asset == 0 {
        return JObject::null().into_raw()
    }

    let asset = asset as *mut ImageAsset;
    let asset = unsafe { &mut *asset };

    let error = asset.error();
    if error.is_empty() {
        return JObject::null().into_raw();
    }

    env.new_string(error.to_string()).unwrap().into_raw()
}
