#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

use std::mem;
use std::mem::transmute;
use std::os::raw::{c_longlong, c_void};
use std::ptr::null_mut;

use jni::objects::JByteBuffer;
use jni::{
    objects::{JClass, JObject},
    JNIEnv,
};
use jni_sys::{jboolean, jbyteArray, jint, jlong, jobject, JNI_TRUE};
use log::debug;

use crate::android::bitmap::{
    AndroidBitmapInfo, AndroidBitmap_getInfo, AndroidBitmap_lockPixels, AndroidBitmap_unlockPixels,
    ANDROID_BITMAP_RESULT_SUCCESS,
};
use crate::common::{
    image_asset_flip_y, image_asset_flip_y_in_place, image_asset_get_rgb_bytes,
    image_asset_get_rgba_bytes,
};

const RGBA: u32 = 0x1908;
const RGBA_INTEGER: u32 = 0x8D99;

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeFlipInPlace3D(
    env: JNIEnv,
    _: JClass,
    pixels: jbyteArray,
    bytesPerPixel: jint,
    height: jint,
    depth: jint,
) {
    let size = env.get_array_length(pixels).unwrap_or(0);
    let mut buf = vec![0i8; size as usize];
    let array = env.get_byte_array_region(pixels, 0, buf.as_mut_slice());
    if array.is_ok() {
        crate::common::utils::flip_in_place_3d(
            transmute(buf.as_mut_ptr()),
            buf.len(),
            bytesPerPixel as usize,
            height as usize,
            depth as usize,
        );
        let _ = env.set_byte_array_region(pixels, 0, buf.as_mut_slice());
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeFlipInPlace(
    env: JNIEnv,
    _: JClass,
    pixels: jbyteArray,
    bytesPerPixel: jint,
    height: jint,
) {
    let size = env.get_array_length(pixels).unwrap_or(0);
    let mut buf = vec![0i8; size as usize];
    let array = env.get_byte_array_region(pixels, 0, buf.as_mut_slice());
    if array.is_ok() {
        crate::common::utils::flip_in_place(
            transmute(buf.as_mut_ptr()),
            buf.len(),
            bytesPerPixel as usize,
            height as usize,
        );
        let _ = env.set_byte_array_region(pixels, 0, buf.as_mut_slice());
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeFlipBufferInPlace(
    env: JNIEnv,
    _: JClass,
    pixels: JByteBuffer,
    bytesPerPixel: jint,
    height: jint,
) {
    let buf = env.get_direct_buffer_address(pixels);
    if buf.is_ok() {
        let buf = buf.unwrap();
        crate::common::utils::flip_in_place(
            buf.as_mut_ptr(),
            buf.len(),
            bytesPerPixel as usize,
            height as usize,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeBufferFromBitmap(
    env: JNIEnv,
    _: JClass,
    bitmap: JObject,
    bytesPerPixel: jint,
    flipY: jboolean,
) -> jobject {
    let native_interface = env.get_native_interface();
    let bitmap_raw = bitmap.into_inner();
    let bitmap_info = Box::into_raw(Box::new(AndroidBitmapInfo::default()));
    let mut empty_slice = [0u8; 0];
    if AndroidBitmap_getInfo(native_interface, bitmap_raw, bitmap_info)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("bytesFromBitmap get bitmap info failed");
        return env
            .new_direct_byte_buffer(&mut empty_slice)
            .unwrap()
            .into_inner();
    }
    let info_to_draw: Box<AndroidBitmapInfo> = Box::from_raw(bitmap_info);

    let mut _dstPixelsToDraw = null_mut() as *mut c_void;
    let dstPixelsToDraw: *mut *mut c_void = &mut _dstPixelsToDraw;
    if AndroidBitmap_lockPixels(native_interface, bitmap_raw, dstPixelsToDraw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("bytesFromBitmap get bitmap lock failed");
        return env
            .new_direct_byte_buffer(&mut empty_slice)
            .unwrap()
            .into_inner();
    }
    let ratio_to_draw = mem::size_of_val(&dstPixelsToDraw) / mem::size_of::<u8>();
    let length_to_draw =
        ((info_to_draw.width * info_to_draw.height) * ratio_to_draw as u32) as usize;

    let ptr_to_draw = _dstPixelsToDraw as *mut _;
    let pixels_to_draw: &mut [i8] =
        std::slice::from_raw_parts_mut(ptr_to_draw as *mut _, length_to_draw as usize);
    let storage;
    if flipY == JNI_TRUE {
        let height = info_to_draw.height as i32;
        crate::common::utils::flip_in_place(
            transmute(pixels_to_draw.as_mut_ptr()),
            pixels_to_draw.len(),
            bytesPerPixel as usize,
            height as usize,
        );
        let storage_slice = { &mut *(pixels_to_draw as *mut [i8] as *mut [u8]) };
        storage = env.new_direct_byte_buffer(storage_slice).unwrap()
    } else {
        let storage_slice = { &mut *(pixels_to_draw as *mut [i8] as *mut [u8]) };
        storage = env.new_direct_byte_buffer(storage_slice).unwrap()
    }

    if AndroidBitmap_unlockPixels(native_interface, bitmap_raw) < ANDROID_BITMAP_RESULT_SUCCESS {
        debug!("bytesFromBitmap unlock bitmap failed");
    }

    storage.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeBytesFromBitmap(
    env: JNIEnv,
    _: JClass,
    bitmap: JObject,
    bytesPerPixel: jint,
    flipY: jboolean,
) -> jbyteArray {
    let native_interface = env.get_native_interface();
    let bitmap_raw = bitmap.into_inner();
    let bitmap_info = Box::into_raw(Box::new(AndroidBitmapInfo::default()));

    if AndroidBitmap_getInfo(native_interface, bitmap_raw, bitmap_info)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("bytesFromBitmap get bitmap info failed");
        return env.new_byte_array(0).unwrap();
    }
    let info_to_draw: Box<AndroidBitmapInfo> = Box::from_raw(bitmap_info);

    let mut _dstPixelsToDraw = null_mut() as *mut c_void;
    let dstPixelsToDraw: *mut *mut c_void = &mut _dstPixelsToDraw;
    if AndroidBitmap_lockPixels(native_interface, bitmap_raw, dstPixelsToDraw)
        < ANDROID_BITMAP_RESULT_SUCCESS
    {
        debug!("bytesFromBitmap get bitmap lock failed");
        return env.new_byte_array(0).unwrap();
    }
    let ratio_to_draw = mem::size_of_val(&dstPixelsToDraw) / mem::size_of::<u8>();
    let length_to_draw =
        ((info_to_draw.width * info_to_draw.height) * ratio_to_draw as u32) as usize;

    let ptr_to_draw = _dstPixelsToDraw as *mut _;
    let pixels_to_draw: &mut [i8] =
        std::slice::from_raw_parts_mut(ptr_to_draw as *mut _, length_to_draw as usize);
    let storage;
    if flipY == JNI_TRUE {
        let height = info_to_draw.height as i32;

        crate::common::utils::flip_in_place(
            transmute(pixels_to_draw.as_mut_ptr()),
            pixels_to_draw.len(),
            bytesPerPixel as usize,
            height as usize,
        );
        let storage_slice = { &*(pixels_to_draw as *mut [i8] as *mut [u8]) };
        storage = env.byte_array_from_slice(storage_slice).unwrap();
    } else {
        let storage_slice = { &*(pixels_to_draw as *mut [i8] as *mut [u8]) };
        storage = env.byte_array_from_slice(storage_slice).unwrap();
    }

    if AndroidBitmap_unlockPixels(native_interface, bitmap_raw) < ANDROID_BITMAP_RESULT_SUCCESS {
        debug!("bytesFromBitmap unlock bitmap failed");
    }

    storage
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeGetVertexAttribOffset(
    env: JNIEnv,
    _: JClass,
    index: jint,
    pname: jint,
    buffer: JByteBuffer,
) {
    let buf = env.get_direct_buffer_address(buffer).unwrap();
    let mut ptr = buf.as_ptr() as *mut c_void;
    let ptr_ptr: *mut *mut c_void = &mut ptr;
    gl_bindings::glGetVertexAttribPointerv(
        index as std::os::raw::c_uint,
        pname as std::os::raw::c_uint,
        ptr_ptr,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeBindBuffer(
    _env: JNIEnv,
    _: JClass,
    target: jint,
    buffer: jint,
) {
    gl_bindings::glBindBuffer(
        target as std::os::raw::c_uint,
        buffer as std::os::raw::c_uint,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeTexImage2DAsset(
    _env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    border: jint,
    format: jint,
    image_type: jint,
    asset: jlong,
    flipY: jboolean,
) {
    let data;
    match format as u32 {
        RGBA | RGBA_INTEGER => {
            data = image_asset_get_rgba_bytes(asset);
        }
        _ => {
            data = image_asset_get_rgb_bytes(asset);
        }
    }
    let flip = flipY == JNI_TRUE;
    if flip {
        image_asset_flip_y_in_place(asset);
        crate::common::utils::flip_in_place(
            data.array,
            data.length,
            (crate::common::utils::bytes_per_pixel(image_type as u32, format as u32) as i32 * width)
                as usize,
            height as usize,
        );
    }
    gl_bindings::glTexImage2D(
        target as u32,
        level,
        internalformat,
        width,
        height,
        border,
        format as u32,
        image_type as u32,
        data.array as *const c_void,
    );
}

pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeTexSubImage2DAsset(
    _env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    width: jint,
    height: jint,
    format: jint,
    image_type: jint,
    asset: jlong,
    flip_y: jboolean,
) {
    let data;
    match format as u32 {
        RGBA | RGBA_INTEGER => {
            data = image_asset_get_rgba_bytes(asset);
        }
        _ => {
            data = image_asset_get_rgb_bytes(asset);
        }
    }
    let flip = flip_y == JNI_TRUE;
    if flip {
        crate::common::utils::flip_in_place(
            data.array,
            data.length,
            (crate::common::utils::bytes_per_pixel(image_type as u32, format as u32) as i32 * width)
                as usize,
            height as usize,
        );
    }
    unsafe {
        gl_bindings::glTexSubImage2D(
            target as u32,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format as u32,
            image_type as u32,
            data.array as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeVertexAttribPointer(
    _env: JNIEnv,
    _: JClass,
    index: jint,
    size: jint,
    pointer_type: jint,
    normalized: jboolean,
    stride: jint,
    offset: c_longlong,
) {
    gl_bindings::glVertexAttribPointer(
        index as u32,
        size,
        pointer_type as u32,
        normalized,
        stride,
        offset as *const c_void,
    )
}
