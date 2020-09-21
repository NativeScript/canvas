#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

use std::os::raw::c_void;

use jni::{objects::JClass, JNIEnv};
use jni_sys::{jboolean, jint, jlong, JNI_TRUE};

use crate::common::{image_asset_get_bytes, image_asset_get_rgb_bytes, image_asset_get_rgba_bytes};

const RGBA: u32 = 0x1908;
const RGBA_INTEGER: u32 = 0x8D99;

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeTexImage3DAsset(
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
        crate::common::utils::flip_in_place_3d(
            data.array,
            data.length,
            crate::common::utils::bytes_per_pixel(image_type as u32, format as u32) as usize
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
        data.array as *const c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_WebGLRenderingContext_nativeTexSubImage3DAsset(
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
        crate::common::utils::flip_in_place_3d(
            data.array,
            data.length,
            crate::common::utils::bytes_per_pixel(image_type as u32, format as u32) as usize
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
        data.array as *const c_void,
    );
}
