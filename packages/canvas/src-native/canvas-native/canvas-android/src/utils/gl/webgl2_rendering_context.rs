#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_void;

use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass, JObject, ReleaseMode};
use jni::sys::{
    jboolean, jbyteArray, jdoubleArray, jfloatArray, jint, jintArray, jlong, jlongArray,
    JNI_TRUE, jshortArray,
};

use canvas_core::context::image_asset::ImageAsset;

use crate::{__log, LogPriority};

const RGBA: u32 = 0x1908;
const RGBA_INTEGER: u32 = 0x8D99;

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeFlipInPlace3D(
    env: JNIEnv,
    _: JClass,
    pixels: JByteBuffer,
    bytesPerPixel: jint,
    height: jint,
    depth: jint,
) {
    if let Ok(buf) = env.get_direct_buffer_address(pixels) {
        canvas_core::utils::gl::flip_in_place_3d(
            buf.as_mut_ptr(),
            buf.len(),
            bytesPerPixel as usize,
            height as usize,
            depth as usize,
        );
    }
}

fn texImage3D(
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    flipY: bool,
    buf: &mut [u8],
) {
    if flipY {
        canvas_core::utils::gl::flip_in_place_3d(
            buf.as_mut_ptr(),
            buf.len(),
            canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                * width as usize,
            height as usize,
            depth as usize,
        );
    }
    unsafe {
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
            buf.as_ptr() as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexImage3DBuffer(
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
        texImage3D(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format,
            image_type,
            flipY == JNI_TRUE,
            data_array,
        );
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexImage3DByteArray(
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
    byteArray: jbyteArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(byteArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(array.as_ptr() as *mut u8, size);
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexImage3DShortArray(
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
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexImage3DIntArray(
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
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexImage3DFloatArray(
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
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexImage3DDoubleArray(
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
    doubleArray: jdoubleArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(doubleArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<f64>(),
            );
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexImage3DLongArray(
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
    longArray: jlongArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(longArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i64>(),
            );
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexImage3DAsset(
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
    let mut data = asset.get_bytes_mut();
    if data.is_some() {
        return;
    }
    let data_array = data.unwrap();
    if flipY == JNI_TRUE {
        canvas_core::utils::gl::flip_in_place_3d(
            data_array.as_mut_ptr(),
            data_array.len(),
            canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
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
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexImage3DBitmap(
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
   if let Some(mut data) = super::super::image::get_bytes_from_bitmap(env, bitmap) {
       if !data.0.is_empty() {
           if flipY == JNI_TRUE {
               canvas_core::utils::gl::flip_in_place_3d(
                   data.0.as_mut_ptr(),
                   data.0.len(),
                   canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                       * data.1.width() as usize,
                   data.1.height() as usize,
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
}

fn texSubImage3D(
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
    flipY: bool,
    buf: &mut [u8],
) {
    if flipY {
        canvas_core::utils::gl::flip_in_place_3d(
            buf.as_mut_ptr(),
            buf.len(),
            canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                * width as usize,
            height as usize,
            depth as usize,
        );
    }
    unsafe {
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
            buf.as_ptr() as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexSubImage3DBuffer(
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
        texSubImage3D(
            target,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format,
            image_type,
            flipY == JNI_TRUE,
            data_array,
        );
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexSubImage3DByteArray(
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
    byteArray: jbyteArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(byteArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(array.as_ptr() as *mut u8, size);
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexSubImage3DShortArray(
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
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexSubImage3DIntArray(
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
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexSubImage3DLongArray(
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
    longArray: jlongArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(longArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i64>(),
            );
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexSubImage3DFloatArray(
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
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexSubImage3DDoubleArray(
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
    doubleArray: jdoubleArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(doubleArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<f64>(),
            );
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexSubImage3DAsset(
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
    let mut data = asset.get_bytes_mut();
    if data.is_some() {
        return;
    }
    let data_array = data.unwrap();
    if flipY == JNI_TRUE {
        canvas_core::utils::gl::flip_in_place_3d(
            data_array.as_mut_ptr(),
            data_array.len(),
            canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
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
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGL2RenderingContext_nativeTexSubImage3DBitmap(
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
    let mut data = super::super::image::get_bytes_from_bitmap(env, bitmap);
    if let Some(mut data) = data {
        if !data.0.is_empty() {
            if flipY == JNI_TRUE {
                canvas_core::utils::gl::flip_in_place_3d(
                    data.0.as_mut_ptr(),
                    data.0.len(),
                    canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                        * data.1.width() as usize,
                    data.1.height() as usize,
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
}
