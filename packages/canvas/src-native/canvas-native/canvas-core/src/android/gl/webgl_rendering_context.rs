#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_void;


use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass, JObject, ReleaseMode};
use jni::sys::{
    jboolean, jbyteArray, jfloatArray, jint, jintArray, jlong, JNI_TRUE, jshortArray,
};


use crate::common::context::image_asset::ImageAsset;


#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DTexture(
    _env: JNIEnv,
    _: JClass,
    _width: jint,
    _height: jint,
    _src_texture: jint,
) {
    let _previous_view_port = [-1_i32; 4];
    let _previous_active_texture = [-1_i32; 1];
    let _previous_texture = [-1_i32; 1];
    let _previous_program = [-1_i32; 1];
    let _previous_frame_buffer = [-1_i32; 1];
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeFlipBufferInPlace(
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
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeGetVertexAttribOffset(
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
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeBindBuffer(
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
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeReadPixels(
    _env: JNIEnv,
    _: JClass,
    x: jint,
    y: jint,
    width: jint,
    height: jint,
    format: jint,
    pixel_type: jint,
) {
    gl_bindings::glReadPixels(
        x,
        y,
        width,
        height,
        format as std::os::raw::c_uint,
        pixel_type as std::os::raw::c_uint,
        0 as *mut c_void,
    );
}

fn texImage2D(
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    border: jint,
    format: jint,
    image_type: jint,
    flipY: bool,
    buf: &mut [u8],
) {
    unsafe {
        if flipY {
            crate::common::utils::gl::flip_in_place(
                buf.as_mut_ptr(),
                buf.len(),
                (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
                    * width) as usize,
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
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DByteArray(
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
    byteArray: jbyteArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(byteArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(array.as_ptr() as *mut u8, size);
            texImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!("byte get_primitive_array_critical error {:?}", e);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DShortArray(
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
    shortArray: jshortArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(shortArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i16>(),
            );
            texImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!("short get_primitive_array_critical error {:?}", e);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DIntArray(
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
    intArray: jintArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(intArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i32>(),
            );
            texImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!("short get_primitive_array_critical error {:?}", e);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DFloatArray(
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
    floatArray: jfloatArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(floatArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<f32>(),
            );
            texImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!("short get_primitive_array_critical error {:?}", e);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DBuffer(
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
            texImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!("get_direct_buffer_address error {:?}", e);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DAsset(
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

    if let Some(bytes) = asset.get_bytes() {
        if flipY == JNI_TRUE {
            let mut bytes = bytes.to_vec();
            crate::common::utils::gl::flip_in_place(
                bytes.as_mut_ptr(),
                bytes.len(),
                (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
                    * asset.width() as u32) as i32 as usize,
                asset.height() as usize,
            );
            gl_bindings::glTexImage2D(
                target as u32,
                level,
                internalformat,
                asset.width() as i32,
                asset.height() as i32,
                border,
                format as u32,
                image_type as u32,
                bytes.as_ptr() as *const c_void,
            );
        } else {
            gl_bindings::glTexImage2D(
                target as u32,
                level,
                internalformat,
                asset.width() as i32,
                asset.height() as i32,
                border,
                format as u32,
                image_type as u32,
                bytes.as_ptr() as *const c_void,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DBitmap(
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
                (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
                    * data.1.width as u32) as i32 as usize,
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

fn texSubImage2D(
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    width: jint,
    height: jint,
    format: jint,
    image_type: jint,
    flip_y: bool,
    buf: &mut [u8],
) {
    if flip_y {
        crate::common::utils::gl::flip_in_place(
            buf.as_mut_ptr(),
            buf.len(),
            (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
                * width as i32) as usize,
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
            buf.as_ptr() as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DByteArray(
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
    byteArray: jbyteArray,
    flip_y: jboolean,
) {
    match env.get_primitive_array_critical(byteArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(array.as_ptr() as *mut u8, size);
            texSubImage2D(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                image_type,
                flip_y == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texSubImage2D: byte get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DShortArray(
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
    shortArray: jshortArray,
    flip_y: jboolean,
) {
    match env.get_primitive_array_critical(shortArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i16>(),
            );
            texSubImage2D(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                image_type,
                flip_y == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texSubImage2D: short get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DIntArray(
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
    intArray: jintArray,
    flip_y: jboolean,
) {
    match env.get_primitive_array_critical(intArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i32>(),
            );
            texSubImage2D(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                image_type,
                flip_y == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texSubImage2D: int get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DFloatArray(
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
    floatArray: jfloatArray,
    flip_y: jboolean,
) {
    match env.get_primitive_array_critical(floatArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<f32>(),
            );
            texSubImage2D(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                image_type,
                flip_y == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texSubImage2D: int get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DBuffer(
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
        texSubImage2D(
            target,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            image_type,
            flip_y == JNI_TRUE,
            data_array,
        );
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DAsset(
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
    if let Some(bytes) = asset.get_bytes() {
        if flip_y == JNI_TRUE {
            let mut bytes = bytes.to_vec();
            crate::common::utils::gl::flip_in_place(
                bytes.as_mut_ptr(),
                bytes.len(),
                (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
                    * asset.width() as u32) as i32 as usize,
                asset.height() as usize,
            );

            gl_bindings::glTexSubImage2D(
                target as u32,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format as u32,
                image_type as u32,
                bytes.as_ptr() as *const c_void,
            );
        } else {
            gl_bindings::glTexSubImage2D(
                target as u32,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format as u32,
                image_type as u32,
                bytes.as_ptr() as *const c_void,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DBitmap(
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
                (crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
                    * data.1.width as i32) as i32 as usize,
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
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeVertexAttribPointer(
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
