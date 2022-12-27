use jni::{
    JNIEnv,
    objects::{JClass, JObject},
    sys::{jbyteArray},
};

use ndk::{
    bitmap::AndroidBitmap,
    bitmap::AndroidBitmapInfo,
};


pub fn get_bytes_from_bitmap(env: JNIEnv, bitmap: JObject) -> Option<(Vec<u8>, AndroidBitmapInfo)> {
    let bm = unsafe { AndroidBitmap::from_jni(env.get_native_interface(), bitmap.into_raw()) };
    match (bm.get_info(), bm.lock_pixels()) {
        (Ok(bitmap_info), Ok(pixels)) => {
            let length = (bitmap_info.height() * bitmap_info.stride()) as usize;
            let slice: &mut [u8] =
                unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, length as usize) };

            let ret = slice.to_vec();
            let _ = bm.unlock_pixels();
            Some((ret, bitmap_info))
        }
        _ => {
            None
        }
    }
}

pub fn bitmap_handler(
    env: JNIEnv,
    bitmap: JObject,
    handler: Box<dyn Fn(Option<(&mut [u8], &AndroidBitmapInfo)>)>,
) {
    let bm = unsafe { AndroidBitmap::from_jni(env.get_native_interface(), bitmap.into_raw()) };

    match (bm.get_info(), bm.lock_pixels()) {
        (Ok(bitmap_info), Ok(pixels)) => {
            let length = (bitmap_info.height() * bitmap_info.stride()) as usize;
            let slice: &mut [u8] =
                unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, length as usize) };
            handler(Some((slice, &bitmap_info)));
            let _ = bm.unlock_pixels();
        }
        (_, _) => {
            handler(None)
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_Utils_nativeGetByteBufferFromBitmap(
    env: JNIEnv,
    _: JClass,
    bitmap: JObject,
) -> jbyteArray {
    if let Some((bytes, _)) = get_bytes_from_bitmap(env, bitmap) {
        return env.byte_array_from_slice(bytes.as_slice())
            .unwrap_or(env.new_byte_array(0).unwrap());
    }
    env.new_byte_array(0).unwrap()
}
