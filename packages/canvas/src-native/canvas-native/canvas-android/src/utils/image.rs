use std::os::raw::c_void;

use jni::objects::{GlobalRef, JClass, JObject};
use jni::sys::jlong;
use jni::JNIEnv;
use ndk::bitmap::{AndroidBitmap, AndroidBitmapInfo};

#[allow(dead_code)]
pub(crate) struct BitmapBytes {
    pub(crate) bitmap: GlobalRef,
    native_bitmap: AndroidBitmap,
    is_locked: bool,
    pixels: *const c_void,
}

impl BitmapBytes {
    pub fn new(env: &JNIEnv, bitmap: JObject) -> Self {
        let bitmap_ref = env.new_global_ref(bitmap).unwrap();
        let native_interface = env.get_native_interface();
        let bitmap = bitmap_ref.as_obj().as_raw();
        let native_bitmap = unsafe { AndroidBitmap::from_jni(native_interface, bitmap) };

        Self {
            bitmap: bitmap_ref,
            native_bitmap,
            is_locked: false,
            pixels: std::ptr::null(),
        }
    }

    pub fn data_mut(&mut self) -> Option<&mut [u8]> {
        if self.pixels.is_null() {
            match self.native_bitmap.lock_pixels() {
                Ok(pixels) => self.pixels = pixels,
                _ => {
                    self.pixels = std::ptr::null();
                }
            }
            return None;
        }

        match self.native_bitmap.get_info() {
            Ok(info) => unsafe {
                Some(std::slice::from_raw_parts_mut(
                    self.pixels as _,
                    (info.width() * info.height() * 4) as usize,
                ))
            },
            _ => None,
        }
    }

    pub fn data(&mut self) -> Option<&[u8]> {
        if self.pixels.is_null() {
            match self.native_bitmap.lock_pixels() {
                Ok(pixels) => self.pixels = pixels,
                _ => {
                    self.pixels = std::ptr::null();
                }
            }
            return None;
        }

        match self.native_bitmap.get_info() {
            Ok(info) => unsafe {
                Some(std::slice::from_raw_parts(
                    self.pixels as _,
                    (info.width() * info.height() * 4) as usize,
                ))
            },
            _ => None,
        }
    }

    pub fn info(&self) -> Option<AndroidBitmapInfo> {
        match self.native_bitmap.get_info() {
            Ok(info) => Some(info),
            _ => None,
        }
    }
}

impl Drop for BitmapBytes {
    fn drop(&mut self) {
        let jvm = crate::JVM.get().unwrap();
        let _ = jvm.attach_current_thread().unwrap();
        if self.is_locked || !self.pixels.is_null() {
            let _ = self.native_bitmap.unlock_pixels();
        }
    }
}

pub fn get_bytes_from_bitmap(env: &JNIEnv, bitmap: JObject) -> Option<(Vec<u8>, AndroidBitmapInfo)> {
    let native_interface = env.get_native_interface();
    let bitmap = bitmap.as_raw();
    let native_bitmap = unsafe { AndroidBitmap::from_jni(native_interface, bitmap) };

    match (native_bitmap.lock_pixels(), native_bitmap.get_info()) {
        (Ok(pixels), Ok(info)) => {
            let buf = unsafe {
                std::slice::from_raw_parts(
                    pixels as *const u8,
                    (info.width() * info.height() * 4) as usize,
                )
            };
            Some((buf.to_vec(), info))
        }
        _ => None,
    }
}

pub fn bitmap_handler(
    env: JNIEnv,
    bitmap: JObject,
    handler: Box<dyn Fn(Option<(&mut [u8], &AndroidBitmapInfo)>)>,
) {
    let native_interface = env.get_native_interface();
    let bitmap = bitmap.as_raw();
    let native_bitmap = unsafe { AndroidBitmap::from_jni(native_interface, bitmap) };
    match (native_bitmap.lock_pixels(), native_bitmap.get_info()) {
        (Ok(pixels), Ok(info)) => {
            let buf = unsafe {
                std::slice::from_raw_parts_mut(
                    pixels as *mut u8,
                    (info.width() * info.height() * 4) as usize,
                )
            };
            handler(Some((buf, &info)));
        }
        _ => {
            handler(None);
        }
    }
    let _ = native_bitmap.unlock_pixels();
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_Bitmap_nativeLockBitmap(
    env: &JNIEnv,
    _: JClass,
    bitmap: JObject,
) -> jlong {
    let bb = BitmapBytes::new(env, bitmap);
    Box::into_raw(Box::new(bb)) as jlong
}
