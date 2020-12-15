use std::os::raw::c_void;

use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass, JObject};
use jni::sys::{jbyteArray, jobject};
use log::{debug, info};

pub fn get_bytes_from_bitmap(env: JNIEnv,
                             bitmap: JObject) -> (Vec<u8>, super::bitmap::AndroidBitmapInfo) {
    let native_interface = env.get_native_interface();
    let bitmap = bitmap.into_inner();
    let bitmap_info: *mut super::bitmap::AndroidBitmapInfo = Box::into_raw(Box::new(super::bitmap::AndroidBitmapInfo::default()));

    unsafe {
        if super::bitmap::AndroidBitmap_getInfo(native_interface as _, bitmap, bitmap_info)
            < super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            debug!("Get Bitmap Info Failed");
            return (vec![], *Box::from_raw(bitmap_info));
        }
    }
    let bitmap_info = *unsafe { Box::from_raw(bitmap_info) };
    let mut pixels = std::ptr::null_mut() as *mut c_void;
    let mut pixels_ptr: *mut *mut c_void = &mut pixels;
    unsafe {
        if super::bitmap::AndroidBitmap_lockPixels(native_interface as _, bitmap, pixels_ptr)
            < super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            debug!("Get Bitmap Lock Failed");
            return (vec![], bitmap_info);
        }
    }
    let length = (bitmap_info.height * bitmap_info.stride as u32) as usize;
    let slice: &mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, length as usize) };
    let slice = slice.to_vec();
    unsafe {
        if super::bitmap::AndroidBitmap_unlockPixels(native_interface as _, bitmap) < super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            debug!("Unlock Bitmap Failed");
        }
    }
    return (slice, bitmap_info);
}

pub fn bitmap_handler(env: JNIEnv,
                      bitmap: JObject, handler: Box<dyn Fn(&mut [u8], &super::bitmap::AndroidBitmapInfo)>) {
    let native_interface = env.get_native_interface();
    let bitmap = bitmap.into_inner();
    let bitmap_info: *mut super::bitmap::AndroidBitmapInfo = Box::into_raw(Box::new(super::bitmap::AndroidBitmapInfo::default()));
    let mut empty_vec = vec![0u8; 0];
    unsafe {
        if super::bitmap::AndroidBitmap_getInfo(native_interface as _, bitmap, bitmap_info)
            != super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            debug!("Get Bitmap Info Failed");
            let info = *Box::from_raw(bitmap_info);
            handler(empty_vec.as_mut_slice(), &info)
        }
    }
    let bitmap_info = *unsafe { Box::from_raw(bitmap_info) };
    let mut pixels = std::ptr::null_mut() as *mut c_void;
    let mut pixels_ptr: *mut *mut c_void = &mut pixels;
    unsafe {
        if super::bitmap::AndroidBitmap_lockPixels(native_interface as _, bitmap, pixels_ptr)
            != super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            debug!("Get Bitmap Lock Failed");
            handler(empty_vec.as_mut_slice(), &bitmap_info);
            return;
        }
    }
    let length = (bitmap_info.height * bitmap_info.stride as u32) as usize;
    let slice: &mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, length as usize) };
    handler(slice, &bitmap_info);
    unsafe {
        if super::bitmap::AndroidBitmap_unlockPixels(native_interface as _, bitmap) != super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            debug!("Unlock Bitmap Failed");
        }
    }
}


#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_Utils_nativeGetByteBufferFromBitmap(
    env: JNIEnv,
    _: JClass,
    bitmap: JObject,
) -> jbyteArray {
    let mut bytes = get_bytes_from_bitmap(env, bitmap);
    env.byte_array_from_slice(bytes.0.as_slice()).unwrap_or(
        env.new_byte_array(0).unwrap()
    )
}