use std::os::raw::c_void;

use jni::objects::{GlobalRef, JClass, JObject, JValue};
use jni::sys::{jbyteArray, jobject};
use jni::JNIEnv;

use canvas_core::ffi::u8_array::U8Array;
use canvas_core::image_bitmap::ImageBitmapPremultiplyAlpha::Default;
use log::debug;

use crate::prelude::find_class;
use crate::utils::bitmap::AndroidBitmapInfo;
use crate::TNSGCUTILS_CLASS;

pub(crate) struct BitmapBytes {
    pub(crate) bitmap: GlobalRef,
    info: AndroidBitmapInfo,
    data_ptr: *mut u8,
    data_len: usize,
}

impl BitmapBytes {
    pub fn new(env: JNIEnv, bitmap: JObject) -> Self {
        let bitmap_ref = env.new_global_ref(bitmap).unwrap();
        let native_interface = env.get_native_interface();
        let bitmap = bitmap_ref.as_obj().into_inner();
        let mut bitmap_info = std::mem::MaybeUninit::uninit();
        unsafe {
            if super::bitmap::AndroidBitmap_getInfo(
                native_interface as _,
                bitmap,
                bitmap_info.as_mut_ptr(),
            ) < super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
            {
                return Self {
                    bitmap: bitmap_ref,
                    info: AndroidBitmapInfo::default(),
                    data_ptr: 0 as *mut u8,
                    data_len: 0,
                };
            }
        }

        let bitmap_info = unsafe { bitmap_info.assume_init() };

        let mut pixels = std::ptr::null_mut() as *mut c_void;
        let pixels_ptr: *mut *mut c_void = &mut pixels;
        unsafe {
            if super::bitmap::AndroidBitmap_lockPixels(native_interface as _, bitmap, pixels_ptr)
                < super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
            {
                return Self {
                    bitmap: bitmap_ref,
                    info: bitmap_info,
                    data_ptr: 0 as *mut u8,
                    data_len: 0,
                };
            }
        }

        let length = (bitmap_info.height * bitmap_info.stride as u32) as usize;

        Self {
            bitmap: bitmap_ref,
            info: bitmap_info,
            data_ptr: pixels as *mut u8,
            data_len: length,
        }
    }

    pub fn data_mut(&mut self) -> Option<&mut [u8]> {
        if self.data_ptr.is_null() {
            return None;
        }
        unsafe { Some(std::slice::from_raw_parts_mut(self.data_ptr, self.data_len)) }
    }

    pub fn info(&self) -> &AndroidBitmapInfo {
        &self.info
    }
}

impl Drop for BitmapBytes {
    fn drop(&mut self) {
        let jvm = crate::JVM.get().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let native_interface = env.get_native_interface();
        unsafe {
            if super::bitmap::AndroidBitmap_unlockPixels(
                native_interface as _,
                self.bitmap.as_obj().into_inner(),
            ) < super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
            {
                debug!("Unlock Bitmap Failed");
            }
        }
    }
}

pub fn get_bytes_from_bitmap(env: JNIEnv, bitmap: JObject) -> (Vec<u8>, AndroidBitmapInfo) {
    let native_interface = env.get_native_interface();
    let bitmap = bitmap.into_inner();
    let mut bitmap_info = std::mem::MaybeUninit::uninit();

    unsafe {
        if super::bitmap::AndroidBitmap_getInfo(
            native_interface as _,
            bitmap,
            bitmap_info.as_mut_ptr(),
        ) < super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            debug!("Get Bitmap Info Failed");
            let info = AndroidBitmapInfo::default();
            let data = Vec::new();
            return (data, info);
        }
    }
    let bitmap_info = unsafe { bitmap_info.assume_init() };
    let mut pixels = std::ptr::null_mut() as *mut c_void;
    let pixels_ptr: *mut *mut c_void = &mut pixels;
    unsafe {
        if super::bitmap::AndroidBitmap_lockPixels(native_interface as _, bitmap, pixels_ptr)
            < super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            debug!("Get Bitmap Lock Failed");
            return (Vec::default(), bitmap_info);
        }
    }
    let length = (bitmap_info.height * bitmap_info.stride as u32) as usize;
    let slice: &mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, length as usize) };
    let slice = slice.to_vec();
    unsafe {
        if super::bitmap::AndroidBitmap_unlockPixels(native_interface as _, bitmap)
            < super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            debug!("Unlock Bitmap Failed");
        }
    }
    return (slice, bitmap_info);
}

pub fn bitmap_handler(
    env: JNIEnv,
    bitmap: JObject,
    handler: Box<dyn Fn(&mut [u8], &AndroidBitmapInfo)>,
) {
    let native_interface = env.get_native_interface();
    let bitmap = bitmap.into_inner();
    let mut bitmap_info = std::mem::MaybeUninit::uninit();
    let mut empty_vec = [0_u8; 0];
    unsafe {
        if super::bitmap::AndroidBitmap_getInfo(
            native_interface as _,
            bitmap,
            bitmap_info.as_mut_ptr(),
        ) != super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            debug!("Get Bitmap Info Failed");
            let info = AndroidBitmapInfo::default();
            handler(empty_vec.as_mut_slice(), &info)
        }
    }
    let bitmap_info = unsafe { bitmap_info.assume_init() };
    let mut pixels = std::ptr::null_mut() as *mut c_void;
    let pixels_ptr: *mut *mut c_void = &mut pixels;
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
        if super::bitmap::AndroidBitmap_unlockPixels(native_interface as _, bitmap)
            != super::bitmap::ANDROID_BITMAP_RESULT_SUCCESS
        {
            debug!("Unlock Bitmap Failed");
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_Utils_nativeGetBytesFromBitmap(
    env: JNIEnv,
    _: JClass,
    bitmap: JObject,
) -> jbyteArray {
    let bytes = get_bytes_from_bitmap(env, bitmap);
    env.byte_array_from_slice(bytes.0.as_slice())
        .unwrap_or(env.new_byte_array(0).unwrap())
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_Utils_nativeGetByteBufferFromBitmap(
    env: JNIEnv,
    _: JClass,
    bitmap: JObject,
) -> jobject {
    let (mut buf, _) = get_bytes_from_bitmap(env, bitmap);

    let db = env.new_direct_byte_buffer(buf.as_mut_slice()).unwrap();
    let u8 = U8Array::from(buf);
    let ptr = Box::into_raw(Box::new(u8));
    let clazz = find_class(TNSGCUTILS_CLASS).unwrap();
    let db: JValue = db.into();
    env.call_static_method(
        clazz,
        "watchItem",
        "(JLjava/nio/ByteBuffer;)V",
        &[(ptr as i64).into(), db],
    )
    .unwrap();

    db.l().unwrap().into_inner()
}
