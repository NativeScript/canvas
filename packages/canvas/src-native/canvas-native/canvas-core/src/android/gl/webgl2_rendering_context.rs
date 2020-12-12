#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_void;

use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass, JObject};
use jni::sys::{jboolean, jint, jlong, JNI_TRUE};

use crate::common::context::image_asset::ImageAsset;

const RGBA: u32 = 0x1908;
const RGBA_INTEGER: u32 = 0x8D99;


#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeFlipInPlace3D(
    env: JNIEnv,
    _: JClass,
    pixels: JByteBuffer,
    bytesPerPixel: jint,
    height: jint,
    depth: jint,
) {
    if let Ok(buf) = env.get_direct_buffer_address(pixels) {
        crate::common::utils::gl::flip_in_place_3d(
            buf.as_mut_ptr(),
            buf.len(),
            bytesPerPixel as usize,
            height as usize,
            depth as usize,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexImage3DBuffer(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    buffer: JByteBuffer,
    flipY: jboolean,
) {
    if let Ok(data_array) = env.get_direct_buffer_address(buffer) {
        if flipY == JNI_TRUE {
            crate::common::utils::gl::flip_in_place_3d(
                data_array.as_mut_ptr(),
                data_array.len(),
                crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                    * width as usize,
                height as usize,
                depth as usize,
            );
        }
        gl_bindings::glTexImage3D(
            target as u32,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format as u32,
            image_type as u32,
            data_array.as_ptr() as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexImage3DAsset(
    _env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    asset: jlong,
    flipY: jboolean,
) {
    let asset: *mut ImageAsset = asset as _;
    let asset = &mut *asset;
    let mut data;
    match format as u32 {
        RGBA | RGBA_INTEGER => {
            data = asset.rgba_internal_bytes()
        }
        _ => {
            data = asset.rgb_internal_bytes()
        }
    }
    let data_array = data.as_mut_slice();
    if flipY == JNI_TRUE {
        crate::common::utils::gl::flip_in_place_3d(
            data_array.as_mut_ptr(),
            data_array.len(),
            crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                * asset.width() as usize,
            asset.height() as usize,
            depth as usize,
        );
    }
    gl_bindings::glTexImage3D(
        target as u32,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format as u32,
        image_type as u32,
        data_array.as_ptr() as *const c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexImage3DBitmap(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    bitmap: JObject,
    flipY: jboolean,
) {
    let mut data = super::super::utils::image::get_bytes_from_bitmap(env, bitmap);
    if !data.0.is_empty() {
        if flipY == JNI_TRUE {
            crate::common::utils::gl::flip_in_place_3d(
                data.0.as_mut_ptr(),
                data.0.len(),
                crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                    * data.1.width as usize,
                data.1.height as usize,
                depth as usize,
            );
        }
        gl_bindings::glTexImage3D(
            target as u32,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format as u32,
            image_type as u32,
            data.0.as_ptr() as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DBuffer(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    buffer: JByteBuffer,
    flipY: jboolean,
) {
    if let Ok(data_array) = env.get_direct_buffer_address(buffer) {
        if flipY == JNI_TRUE {
            crate::common::utils::gl::flip_in_place_3d(
                data_array.as_mut_ptr(),
                data_array.len(),
                crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                    * width as usize,
                height as usize,
                depth as usize,
            );
        }
        gl_bindings::glTexSubImage3D(
            target as u32,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format as u32,
            image_type as u32,
            data_array.as_ptr() as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DAsset(
    _env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    asset: jlong,
    flipY: jboolean,
) {
    let asset: *mut ImageAsset = asset as _;
    let asset = &mut *asset;
    let mut data;
    match format as u32 {
        RGBA | RGBA_INTEGER => {
            data = asset.rgba_internal_bytes()
        }
        _ => {
            data = asset.rgb_internal_bytes()
        }
    }
    let data_array = data.as_mut_slice();
    if flipY == JNI_TRUE {
        crate::common::utils::gl::flip_in_place_3d(
            data_array.as_mut_ptr(),
            data_array.len(),
            crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                * asset.width() as usize,
            asset.height() as usize,
            depth as usize,
        );
    }
    gl_bindings::glTexSubImage3D(
        target as u32,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format as u32,
        image_type as u32,
        data_array.as_ptr() as *const c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DBitmap(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    bitmap: JObject,
    flipY: jboolean,
) {
    let mut data = super::super::utils::image::get_bytes_from_bitmap(env, bitmap);
    if !data.0.is_empty() {
        if flipY == JNI_TRUE {
            crate::common::utils::gl::flip_in_place_3d(
                data.0.as_mut_ptr(),
                data.0.len(),
                crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                    * data.1.width as usize,
                data.1.height as usize,
                depth as usize,
            );
        }
        gl_bindings::glTexSubImage3D(
            target as u32,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format as u32,
            image_type as u32,
            data.0.as_ptr() as *const c_void,
        );
    }
}
