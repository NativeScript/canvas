#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]


use std::os::raw::c_void;
use jni::sys::jobject;

#[doc = " Bitmap info, see AndroidBitmap_getInfo()."]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AndroidBitmapInfo {
    #[doc = " The bitmap width in pixels."]
    pub width: u32,
    #[doc = " The bitmap height in pixels."]
    pub height: u32,
    #[doc = " The number of byte per row."]
    pub stride: u32,
    #[doc = " The bitmap pixel format. See {@link AndroidBitmapFormat}"]
    pub format: i32,
    #[doc = " Unused."]
    pub flags: u32,
}

impl Default for AndroidBitmapInfo {
    fn default() -> Self {
        AndroidBitmapInfo {
            width: 0,
            height: 0,
            stride: 0,
            format: 0,
            flags: 0,
        }
    }
}

extern "system" {
    #[doc = " Given a java bitmap object, fill out the AndroidBitmapInfo struct for it."]
    #[doc = " If the call fails, the info parameter will be ignored."]
    pub fn AndroidBitmap_getInfo(
        env: *mut jni::JNIEnv,
        jbitmap: jobject,
        info: *mut AndroidBitmapInfo,
    ) -> ::std::os::raw::c_int;
}

extern "system" {
    #[doc = " Given a java bitmap object, attempt to lock the pixel address."]
    #[doc = " Locking will ensure that the memory for the pixels will not move"]
    #[doc = " until the unlockPixels call, and ensure that, if the pixels had been"]
    #[doc = " previously purged, they will have been restored."]
    #[doc = ""]
    #[doc = " If this call succeeds, it must be balanced by a call to"]
    #[doc = " AndroidBitmap_unlockPixels, after which time the address of the pixels should"]
    #[doc = " no longer be used."]
    #[doc = ""]
    #[doc = " If this succeeds, *addrPtr will be set to the pixel address. If the call"]
    #[doc = " fails, addrPtr will be ignored."]
    pub fn AndroidBitmap_lockPixels(
        env: *mut jni::JNIEnv,
        jbitmap: jobject,
        addrPtr: *mut *mut c_void,
    ) -> ::std::os::raw::c_int;
}

extern "system" {
    #[doc = " Call this to balance a successful call to AndroidBitmap_lockPixels."]
    pub fn AndroidBitmap_unlockPixels(
        env: *mut jni::JNIEnv,
        jbitmap: jobject,
    ) -> ::std::os::raw::c_int;
}

#[doc = " Operation was successful."]
pub const ANDROID_BITMAP_RESULT_SUCCESS: _bindgen_ty_1 = 0;
#[doc = " Bad parameter."]
pub const ANDROID_BITMAP_RESULT_BAD_PARAMETER: _bindgen_ty_1 = -1;
#[doc = " JNI exception occured."]
pub const ANDROID_BITMAP_RESULT_JNI_EXCEPTION: _bindgen_ty_1 = -2;
#[doc = " Allocation failed."]
pub const ANDROID_BITMAP_RESULT_ALLOCATION_FAILED: _bindgen_ty_1 = -3;

#[doc = " AndroidBitmap functions result code."]
pub type _bindgen_ty_1 = i32;

#[doc = " No format."]
pub const AndroidBitmapFormat_ANDROID_BITMAP_FORMAT_NONE: AndroidBitmapFormat = 0;
#[doc = " Red: 8 bits, Green: 8 bits, Blue: 8 bits, Alpha: 8 bits."]
pub const AndroidBitmapFormat_ANDROID_BITMAP_FORMAT_RGBA_8888: AndroidBitmapFormat = 1;
#[doc = " Red: 5 bits, Green: 6 bits, Blue: 5 bits."]
pub const AndroidBitmapFormat_ANDROID_BITMAP_FORMAT_RGB_565: AndroidBitmapFormat = 4;
#[doc = " Deprecated in API level 13. Because of the poor quality of this configuration, it is advised to use ARGB_8888 instead."]
pub const AndroidBitmapFormat_ANDROID_BITMAP_FORMAT_RGBA_4444: AndroidBitmapFormat = 7;
#[doc = " Alpha: 8 bits."]
pub const AndroidBitmapFormat_ANDROID_BITMAP_FORMAT_A_8: AndroidBitmapFormat = 8;

#[doc = " Bitmap pixel format."]
pub type AndroidBitmapFormat = u32;
