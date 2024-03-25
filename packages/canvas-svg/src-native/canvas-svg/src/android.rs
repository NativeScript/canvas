use std::ffi::c_void;

use jni::objects::{JClass, JObject, JString};
use jni::JNIEnv;
use ndk::bitmap::AndroidBitmap;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeDrawSVG(
    mut env: JNIEnv,
    _: JClass,
    bitmap: JObject,
    svg: JString,
) {
    if bitmap.is_null() {
        return;
    }
    if let Ok(svg) = env.get_string(&svg) {
        let svg = svg.to_string_lossy();
        let native_bitmap =
            unsafe { AndroidBitmap::from_jni(env.get_native_interface(), bitmap.as_raw()) };
        if let (Ok(ptr), Ok(info)) = (native_bitmap.lock_pixels(), native_bitmap.get_info()) {
            if ptr.is_null() {
                return;
            }
            let width = info.width();
            let height = info.height();
            let size = height * info.stride();
            let slice = unsafe {
                std::slice::from_raw_parts_mut(
                    std::mem::transmute::<*mut c_void, *mut u8>(ptr),
                    size as usize,
                )
            };

            let info = skia_safe::ImageInfo::new_n32_premul(
                skia_safe::ISize::new(width as i32, height as i32),
                None,
            );
            if let Some(mut surface) =
                skia_safe::surface::surfaces::wrap_pixels(&info, slice, None, None)
            {
                crate::draw_svg(&mut surface, svg.as_ref())
            }
        }
        let _ = native_bitmap.unlock_pixels();
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeDrawSVGFromPath(
    mut env: JNIEnv,
    _: JClass,
    bitmap: JObject,
    path: JString,
) {
    if bitmap.is_null() {
        return;
    }

    if let Ok(path) = env.get_string(&path) {
        let path = path.to_string_lossy();
        let native_bitmap =
            unsafe { AndroidBitmap::from_jni(env.get_native_interface(), bitmap.as_raw()) };
        if let (Ok(ptr), Ok(info)) = (native_bitmap.lock_pixels(), native_bitmap.get_info()) {
            if ptr.is_null() {
                return;
            }
            let width = info.width();
            let height = info.height();
            let size = height * info.stride();
            let slice = unsafe {
                std::slice::from_raw_parts_mut(
                    std::mem::transmute::<*mut c_void, *mut u8>(ptr),
                    size as usize,
                )
            };

            let info = skia_safe::ImageInfo::new_n32_premul(
                skia_safe::ISize::new(width as i32, height as i32),
                None,
            );
            if let Some(mut surface) =
                skia_safe::surface::surfaces::wrap_pixels(&info, slice, None, None)
            {
                crate::draw_svg_from_path(&mut surface, path.as_ref())
            }
        }
        let _ = native_bitmap.unlock_pixels();
    }
}