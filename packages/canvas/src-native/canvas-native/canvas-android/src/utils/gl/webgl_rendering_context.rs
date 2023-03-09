#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::c_void;

use jni::objects::{
    JByteArray, JByteBuffer, JClass, JFloatArray, JIntArray, JObject, JShortArray, ReleaseMode,
};
use jni::sys::{jboolean, jint, jlong, JNI_TRUE};
use jni::JNIEnv;

use canvas_core::image_asset::ImageAsset;

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
    if let (Ok(buf), Ok(size)) = (
        env.get_direct_buffer_address(&pixels),
        env.get_direct_buffer_capacity(&pixels),
    ) {
        canvas_2d::utils::gl::flip_in_place(buf, size, bytesPerPixel as usize, height as usize);
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
    if let Ok(buf) = env.get_direct_buffer_address(&buffer) {
        let mut ptr = buf as *mut c_void;
        let ptr_ptr: *mut *mut c_void = &mut ptr;
        gl_bindings::GetVertexAttribPointerv(
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
    gl_bindings::BindBuffer(
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
    gl_bindings::ReadPixels(
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
            let mut buf = buf.to_vec();
            canvas_2d::utils::gl::flip_in_place(
                buf.as_mut_ptr(),
                buf.len(),
                (canvas_2d::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
                    * width) as usize,
                height as usize,
            );
            gl_bindings::TexImage2D(
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
        } else {
            gl_bindings::TexImage2D(
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
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DByteArray(
    mut env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    border: jint,
    format: jint,
    image_type: jint,
    byteArray: JByteArray,
    flipY: jboolean,
) {
    match env.get_array_elements_critical(&byteArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.len();
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
        Err(_) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DShortArray(
    mut env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    border: jint,
    format: jint,
    image_type: jint,
    shortArray: JShortArray,
    flipY: jboolean,
) {
    match env.get_array_elements_critical(&shortArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.len();
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
        Err(_) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DIntArray(
    mut env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    border: jint,
    format: jint,
    image_type: jint,
    intArray: JIntArray,
    flipY: jboolean,
) {
    match env.get_array_elements_critical(&intArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.len();
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
        Err(_) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DFloatArray(
    mut env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    border: jint,
    format: jint,
    image_type: jint,
    floatArray: JFloatArray,
    flipY: jboolean,
) {
    match env.get_array_elements_critical(&floatArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.len();
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
        Err(_) => {}
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
    match (
        env.get_direct_buffer_address(&buffer),
        env.get_direct_buffer_capacity(&buffer),
    ) {
        (Ok(buf), Ok(size)) => {
            let buf = unsafe { std::slice::from_raw_parts_mut(buf, size) };
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
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage2DBufferEncoded(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    _width: jint,
    _height: jint,
    border: jint,
    format: jint,
    image_type: jint,
    buffer: JByteBuffer,
    flipY: jboolean,
) {
    match (
        env.get_direct_buffer_address(&buffer),
        env.get_direct_buffer_capacity(&buffer),
    ) {
        (Ok(buf), Ok(size)) => {
            let mut image_asset = ImageAsset::new();
            let buf = unsafe { std::slice::from_raw_parts_mut(buf, size) };
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
        _ => {}
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
    if let Some(data_array) = asset.get_bytes() {
        if flipY == JNI_TRUE {
            let mut data_array = data_array.to_vec();
            canvas_2d::utils::gl::flip_in_place(
                data_array.as_mut_ptr(),
                data_array.len(),
                (canvas_2d::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
                    * asset.width() as u32) as i32 as usize,
                asset.height() as usize,
            );

            unsafe {
                gl_bindings::TexImage2D(
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
            };
        } else {
            unsafe {
                gl_bindings::TexImage2D(
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
        let data = crate::utils::image::get_bytes_from_bitmap(env, bitmap);
        if let Some(mut data) = data {
            if !data.0.is_empty() {
                if flipY == JNI_TRUE {
                    canvas_2d::utils::gl::flip_in_place(
                        data.0.as_mut_ptr(),
                        data.0.len(),
                        (canvas_2d::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
                            * data.1.width() as u32) as i32 as usize,
                        data.1.height() as usize,
                    );
                }
                gl_bindings::TexImage2D(
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
    } else {
        let mut data = crate::utils::image::BitmapBytes::new(env, bitmap);
        if let Some(data) = data.data_mut() {
            gl_bindings::TexImage2D(
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
        let mut buf = buf.to_vec();
        canvas_2d::utils::gl::flip_in_place(
            buf.as_mut_ptr(),
            buf.len(),
            (canvas_2d::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
                * width as i32) as usize,
            height as usize,
        );

        unsafe {
            gl_bindings::TexSubImage2D(
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
    } else {
        unsafe {
            gl_bindings::TexSubImage2D(
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
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DByteArray(
    mut env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    width: jint,
    height: jint,
    format: jint,
    image_type: jint,
    byteArray: JByteArray,
    flip_y: jboolean,
) {
    match env.get_array_elements_critical(&byteArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.len();
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
        Err(_) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DShortArray(
    mut env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    width: jint,
    height: jint,
    format: jint,
    image_type: jint,
    shortArray: JShortArray,
    flip_y: jboolean,
) {
    match env.get_array_elements_critical(&shortArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.len();
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
        Err(_) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DIntArray(
    mut env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    width: jint,
    height: jint,
    format: jint,
    image_type: jint,
    intArray: JIntArray,
    flip_y: jboolean,
) {
    match env.get_array_elements_critical(&intArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.len();
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
        Err(_) => {}
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage2DFloatArray(
    mut env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    width: jint,
    height: jint,
    format: jint,
    image_type: jint,
    floatArray: JFloatArray,
    flip_y: jboolean,
) {
    match env.get_array_elements_critical(&floatArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.len();
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
        Err(_) => {}
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
    if let (Ok(data_array), Ok(size)) = (
        env.get_direct_buffer_address(&buffer),
        env.get_direct_buffer_capacity(&buffer),
    ) {
        let data_array = unsafe { std::slice::from_raw_parts_mut(data_array, size) };
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
    if let Some(data_array) = asset.get_bytes() {
        if flip_y == JNI_TRUE {
            let mut data_array = data_array.to_vec();
            canvas_2d::utils::gl::flip_in_place(
                data_array.as_mut_ptr(),
                data_array.len(),
                (canvas_2d::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
                    * asset.width() as u32) as i32 as usize,
                asset.height() as usize,
            );

            gl_bindings::TexSubImage2D(
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
        } else {
            gl_bindings::TexSubImage2D(
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
    let data = super::super::image::get_bytes_from_bitmap(env, bitmap);
    if let Some(mut data) = data {
        if !data.0.is_empty() {
            if flip_y == JNI_TRUE {
                canvas_2d::utils::gl::flip_in_place(
                    data.0.as_mut_ptr(),
                    data.0.len(),
                    (canvas_2d::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as i32
                        * data.1.width() as i32) as i32 as usize,
                    data.1.height() as usize,
                );
            }
            gl_bindings::TexSubImage2D(
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
    gl_bindings::VertexAttribPointer(
        index as u32,
        size,
        pointer_type as u32,
        normalized,
        stride,
        offset as *const c_void,
    )
}
