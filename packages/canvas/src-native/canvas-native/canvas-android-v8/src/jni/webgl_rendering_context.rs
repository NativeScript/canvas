#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_void;

use jni::objects::{JByteBuffer, JClass, JObject, ReleaseMode};
use jni::sys::{jboolean, jbyteArray, jfloatArray, jint, jintArray, jlong, jshortArray, JNI_TRUE};
use jni::JNIEnv;

use canvas_core::context::image_asset::ImageAsset;

const RGBA: u32 = 0x1908;
const RGBA_INTEGER: u32 = 0x8D99;

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
        canvas_core::utils::gl::flip_in_place(
            buf.as_mut_ptr(),
            buf.len(),
            bytesPerPixel as usize,
            height as usize,
        );
    }
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
            canvas_core::utils::gl::flip_in_place(
                buf.as_mut_ptr(),
                buf.len(),
                (canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
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
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DBufferEncoded(
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
            let mut image_asset = ImageAsset::new();
            image_asset.load_from_bytes(buf);
            texImage2DAsset(
                target,
                level,
                internalformat,
                border,
                format,
                image_type,
                &mut image_asset,
                flipY,
            );
        }
        Err(e) => {
            log::debug!("get_direct_buffer_address error {:?}", e);
        }
    }
}

fn texImage2DAsset(
    target: jint,
    level: jint,
    internalformat: jint,
    border: jint,
    format: jint,
    image_type: jint,
    asset: &mut ImageAsset,
    flipY: jboolean,
) {
    let mut data;
    match format as u32 {
        RGBA | RGBA_INTEGER => data = asset.rgba_internal_bytes(),
        _ => data = asset.rgb_internal_bytes(),
    }
    let data_array = data.as_mut_slice();
    if flipY == JNI_TRUE {
        canvas_core::utils::gl::flip_in_place(
            data_array.as_mut_ptr(),
            data_array.len(),
            (canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
                * asset.width() as u32) as i32 as usize,
            asset.height() as usize,
        );
    }
    unsafe {
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
}
#[no_mangle]
pub extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DAsset(
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
    let asset = unsafe { &mut *asset };
    texImage2DAsset(
        target,
        level,
        internalformat,
        border,
        format,
        image_type,
        asset,
        flipY,
    )
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
    if flipY == JNI_TRUE {
        let mut data = crate::utils::image::get_bytes_from_bitmap(env, bitmap);
        if !data.0.is_empty() {
            if flipY == JNI_TRUE {
                canvas_core::utils::gl::flip_in_place(
                    data.0.as_mut_ptr(),
                    data.0.len(),
                    (canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
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
    } else {
        let mut data = crate::utils::image::BitmapBytes::new(env, bitmap);
        if let Some(data) = data.data_mut() {
            gl_bindings::glTexImage2D(
                target as u32,
                level,
                internalformat,
                width,
                height,
                border,
                format as u32,
                image_type as u32,
                data.as_ptr() as *const c_void,
            );
        }
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
        canvas_core::utils::gl::flip_in_place(
            buf.as_mut_ptr(),
            buf.len(),
            (canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
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
    let mut data;
    match format as u32 {
        RGBA | RGBA_INTEGER => data = asset.rgba_internal_bytes(),
        _ => data = asset.rgb_internal_bytes(),
    }
    let data_array = data.as_mut_slice();
    if flip_y == JNI_TRUE {
        canvas_core::utils::gl::flip_in_place(
            data_array.as_mut_ptr(),
            data_array.len(),
            (canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
                * asset.width() as u32) as i32 as usize,
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
            canvas_core::utils::gl::flip_in_place(
                data.0.as_mut_ptr(),
                data.0.len(),
                (canvas_core::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
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