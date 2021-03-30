#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


use std::os::raw::c_void;

use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass, JObject};
use jni::sys::{jboolean, jint, jlong, JNI_TRUE, jobject};

use crate::common::context::image_asset::ImageAsset;
use jni::errors::Error;

const RGBA: u32 = 0x1908;
const RGBA_INTEGER: u32 = 0x8D99;

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeFlipBufferInPlace(
    env: JNIEnv,
    _: JClass,
    pixels: JByteBuffer,
    bytesPerPixel: jint,
    height: jint,
) {
    if let Ok(buf) = env.get_direct_buffer_address(pixels) {
        crate::common::utils::gl::flip_in_place(
            buf.as_mut_ptr(),
            buf.len(),
            bytesPerPixel as usize,
            height as usize,
        );
    }
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeGetVertexAttribOffset(
    env: JNIEnv,
    _: JClass,
    index: jint,
    pname: jint,
    buffer: JByteBuffer,
) {
    if let Ok(buf) = env.get_direct_buffer_address(buffer) {
        let mut ptr = buf.as_mut_ptr() as *mut c_void;
        let ptr_ptr: *mut *mut c_void = &mut ptr;
        gl_bindings::glGetVertexAttribPointerv(
            index as std::os::raw::c_uint,
            pname as std::os::raw::c_uint,
            ptr_ptr,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeBindBuffer(
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
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexImage2DBuffer(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    border: jint,
    format: jint,
    image_type: jint,
    buffer: JByteBuffer,
    flipY: jboolean,
) {
    match env.get_direct_buffer_address(buffer) {
        Ok(buf) => {
            if flipY == JNI_TRUE {
                crate::common::utils::gl::flip_in_place(
                    buf.as_mut_ptr(),
                    buf.len(),
                    (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32 * width)
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
                buf.as_ptr() as *const c_void,
            );
        }
        Err(e) => {
            log::debug!("get_direct_buffer_address error {:?}",  e);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexImage2DAsset(
    _env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
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
        crate::common::utils::gl::flip_in_place(
            data_array.as_mut_ptr(),
            data_array.len(),
            (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) * asset.width() as u32) as i32
                as usize,
            asset.height() as usize,
        );
    }
    gl_bindings::glTexImage2D(
        target as u32,
        level,
        internalformat,
        asset.width() as i32,
        asset.height() as i32,
        border,
        format as u32,
        image_type as u32,
        data_array.as_ptr() as *const c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexImage2DBitmap(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    border: jint,
    format: jint,
    image_type: jint,
    bitmap: JObject,
    flipY: jboolean,
) {
    let mut data = super::super::utils::image::get_bytes_from_bitmap(env, bitmap);
    if !data.0.is_empty() {
        if flipY == JNI_TRUE {
            crate::common::utils::gl::flip_in_place(
                data.0.as_mut_ptr(),
                data.0.len(),
                (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) * data.1.width as u32) as i32
                    as usize,
                data.1.height as usize,
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
            data.0.as_ptr() as *const c_void,
        );
    }
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DBuffer(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    width: jint,
    height: jint,
    format: jint,
    image_type: jint,
    buffer: JByteBuffer,
    flip_y: jboolean,
) {
    if let Ok(data_array) = env.get_direct_buffer_address(buffer) {
        if flip_y == JNI_TRUE {
            crate::common::utils::gl::flip_in_place(
                data_array.as_mut_ptr(),
                data_array.len(),
                (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32 * width as i32)
                    as usize,
                height as usize,
            );
        }
        gl_bindings::glTexSubImage2D(
            target as u32,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format as u32,
            image_type as u32,
            data_array.as_ptr() as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DAsset(
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
    if flip_y == JNI_TRUE {
        crate::common::utils::gl::flip_in_place(
            data_array.as_mut_ptr(),
            data_array.len(),
            (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) * asset.width() as u32) as i32
                as usize,
            asset.height() as usize,
        );
    }
    gl_bindings::glTexSubImage2D(
        target as u32,
        level,
        xoffset,
        yoffset,
        width,
        height,
        format as u32,
        image_type as u32,
        data_array.as_ptr() as *const c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DBitmap(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    width: jint,
    height: jint,
    format: jint,
    image_type: jint,
    bitmap: JObject,
    flip_y: jboolean,
) {
    let mut data = super::super::utils::image::get_bytes_from_bitmap(env, bitmap);
    if !data.0.is_empty() {
        if flip_y == JNI_TRUE {
            crate::common::utils::gl::flip_in_place(
                data.0.as_mut_ptr(),
                data.0.len(),
                (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32 * data.1.width as i32) as i32
                    as usize,
                data.1.height as usize,
            );
        }
        gl_bindings::glTexSubImage2D(
            target as u32,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format as u32,
            image_type as u32,
            data.0.as_ptr() as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_github_triniwiz_canvas_TNSWebGLRenderingContext_nativeVertexAttribPointer(
    _env: JNIEnv,
    _: JClass,
    index: jint,
    size: jint,
    pointer_type: jint,
    normalized: jboolean,
    stride: jint,
    offset: jlong,
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
