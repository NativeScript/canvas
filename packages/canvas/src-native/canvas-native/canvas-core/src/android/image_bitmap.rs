use jni::errors::Error;
use jni::objects::{AutoPrimitiveArray, JByteBuffer, JClass, ReleaseMode};
use jni::sys::{jboolean, jbyteArray, jfloat, jint, jlong, JNI_TRUE};
use jni::JNIEnv;
use skia_safe::{RCHandle, Rect};

use crate::common::context::image_asset::ImageAsset;
use crate::common::image_bitmap;
use crate::common::image_bitmap::create_image_bitmap;
use crate::common::utils::image::{from_image_slice, from_image_slice_encoded};

/*
ImageBitmap backed by ImageAsset
*/

fn create_from_bytes_src_rect(
    env: JNIEnv,
    image_bytes: jbyteArray,
    image_width: jfloat,
    image_height: jfloat,
    rect: Option<skia_safe::Rect>,
    flip_y: bool,
    premultiply_alpha: jint,
    color_space_conversion: jint,
    resize_quality: jint,
    resize_width: jfloat,
    resize_height: jfloat,
) -> jlong {
    let default_result = || Box::into_raw(Box::new(ImageAsset::new())) as jlong;
    return match env.get_primitive_array_critical(image_bytes, ReleaseMode::NoCopyBack) {
        Ok(buf) => {
            let size = buf.size().unwrap_or(0) as usize;
            if size > 0 {
                let slice = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, size) };
                return image_bitmap::create_image_asset(
                    slice,
                    image_width,
                    image_height,
                    rect,
                    flip_y,
                    premultiply_alpha,
                    color_space_conversion,
                    resize_quality,
                    resize_width,
                    resize_height,
                )
            }
            default_result()
        }
        Err(_) => default_result(),
    }
}

#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSImageBitmap_nativeCreateFromBuffer(
    env: JNIEnv,
    _: JClass,
    image_buffer: JByteBuffer,
    image_width: jfloat,
    image_height: jfloat,
    flip_y: jboolean,
    premultiply_alpha: jint,
    color_space_conversion: jint,
    resize_quality: jint,
    resize_width: jfloat,
    resize_height: jfloat,
) -> jlong {
    match env.get_direct_buffer_address(image_buffer) {
        Ok(buf) => image_bitmap::create_image_asset(
            buf,
            image_width,
            image_height,
            None,
            flip_y == JNI_TRUE,
            premultiply_alpha,
            color_space_conversion,
            resize_quality,
            resize_width,
            resize_height,
        ),
        Err(_) => Box::into_raw(Box::new(ImageAsset::new())) as jlong,
    }
}

#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSImageBitmap_nativeCreateFromBufferSrcRect(
    env: JNIEnv,
    _: JClass,
    image_buffer: JByteBuffer,
    image_width: jfloat,
    image_height: jfloat,
    sx: jfloat,
    sy: jfloat,
    s_width: jfloat,
    s_height: jfloat,
    flip_y: jboolean,
    premultiply_alpha: jint,
    color_space_conversion: jint,
    resize_quality: jint,
    resize_width: jfloat,
    resize_height: jfloat,
) -> jlong {
    match env.get_direct_buffer_address(image_buffer) {
        Ok(buf) => image_bitmap::create_image_asset(
            buf,
            image_width,
            image_height,
            Some(skia_safe::Rect::from_xywh(sx, sy, s_width, s_height)),
            flip_y == JNI_TRUE,
            premultiply_alpha,
            color_space_conversion,
            resize_quality,
            resize_width,
            resize_height,
        ),
        Err(_) => Box::into_raw(Box::new(ImageAsset::new())) as jlong,
    }
}

#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSImageBitmap_nativeCreateFromBytes(
    env: JNIEnv,
    _: JClass,
    image_bytes: jbyteArray,
    image_width: jfloat,
    image_height: jfloat,
    flip_y: jboolean,
    premultiply_alpha: jint,
    color_space_conversion: jint,
    resize_quality: jint,
    resize_width: jfloat,
    resize_height: jfloat,
) -> jlong {
    create_from_bytes_src_rect(
        env,
        image_bytes,
        image_width,
        image_height,
        None,
        flip_y == JNI_TRUE,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
    )
}

#[no_mangle]
pub extern "C" fn Java_com_github_triniwiz_canvas_TNSImageBitmap_nativeCreateFromBytesSrcRect(
    env: JNIEnv,
    _: JClass,
    image_bytes: jbyteArray,
    image_width: jfloat,
    image_height: jfloat,
    sx: jfloat,
    sy: jfloat,
    s_width: jfloat,
    s_height: jfloat,
    flip_y: jboolean,
    premultiply_alpha: jint,
    color_space_conversion: jint,
    resize_quality: jint,
    resize_width: jfloat,
    resize_height: jfloat,
) -> jlong {
    create_from_bytes_src_rect(
        env,
        image_bytes,
        image_width,
        image_height,
        Some(skia_safe::Rect::from_xywh(sx, sy, s_width, s_height)),
        flip_y == JNI_TRUE,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
    )
}
