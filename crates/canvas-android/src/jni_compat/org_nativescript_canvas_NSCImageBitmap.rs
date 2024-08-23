use jni::JNIEnv;
use jni::objects::{JByteArray, JByteBuffer, JClass, ReleaseMode};
use jni::sys::{jboolean, jfloat, jint, jlong, JNI_FALSE, JNI_TRUE};

use canvas_c::{ImageAsset, ImageBitmapColorSpaceConversion, ImageBitmapPremultiplyAlpha, ImageBitmapResizeQuality};

fn loadFromPtr(
    asset: *const ImageAsset,
    data: *const u8,
    size: usize,
) -> bool {
    canvas_c::canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
        data, size, false,
        ImageBitmapPremultiplyAlpha::Default,
        ImageBitmapColorSpaceConversion::Default,
        ImageBitmapResizeQuality::Low,
        0., 0.,
        asset,
    )
}

fn loadFromPtrOptions(
    asset: *const ImageAsset,
    data: *const u8,
    size: usize,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: jfloat,
    resize_height: jfloat,
) -> bool {
    canvas_c::canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
        data, size, flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width, resize_height,
        asset,
    )
}

fn loadFromPtrRectOptions(
    asset: *const ImageAsset,
    data: *const u8,
    size: usize,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
    flip_y: bool,
    premultiply_alpha: ImageBitmapPremultiplyAlpha,
    color_space_conversion: ImageBitmapColorSpaceConversion,
    resize_quality: ImageBitmapResizeQuality,
    resize_width: jfloat,
    resize_height: jfloat,
) -> bool {
    canvas_c::canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
        data, size,
        x, y, width, height,
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width, resize_height,
        asset,
    )
}
#[no_mangle]
pub unsafe extern "system" fn nativeLoadBitmapFromBytes(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    byteArray: JByteArray,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *const ImageAsset;

    match env.get_array_elements_critical(&byteArray, ReleaseMode::NoCopyBack) {
        Ok(bytes) => {
            let size = bytes.len();
            let slice = std::slice::from_raw_parts_mut(bytes.as_ptr() as *mut u8, size);
            let valid = loadFromPtr(asset, slice.as_ptr(), size);
            if valid {
                return JNI_TRUE;
            }
            JNI_FALSE
        }
        Err(_) => JNI_FALSE,
    }
}

#[no_mangle]
pub extern "system" fn nativeLoadBitmapFromBuffer(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    buffer: JByteBuffer,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *const ImageAsset;

    if let (Ok(buf), Ok(size)) = (
        env.get_direct_buffer_address(&buffer),
        env.get_direct_buffer_capacity(&buffer),
    ) {
        let valid = loadFromPtr(asset, buf, size);
        if valid {
            return JNI_TRUE;
        }
    }
    JNI_FALSE
}


#[no_mangle]
pub unsafe extern "system" fn nativeLoadBitmapFromBytesOptions(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    byteArray: JByteArray,
    flip_y: bool,
    premultiply_alpha: jint,
    color_space_conversion: jint,
    resize_quality: jint,
    resize_width: jfloat,
    resize_height: jfloat,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *const ImageAsset;

    match env.get_array_elements_critical(&byteArray, ReleaseMode::NoCopyBack) {
        Ok(bytes) => {
            let size = bytes.len();
            let ptr = bytes.as_ptr() as *mut u8;

            let premultiply_alpha = match premultiply_alpha {
                0 => ImageBitmapPremultiplyAlpha::Default,
                1 => ImageBitmapPremultiplyAlpha::Premultiply,
                2 => ImageBitmapPremultiplyAlpha::AlphaNone,
                _ => return {
                    JNI_FALSE
                }
            };
            let color_space_conversion = match color_space_conversion {
                0 => ImageBitmapColorSpaceConversion::Default,
                1 => ImageBitmapColorSpaceConversion::None,
                _ => return {
                    JNI_FALSE
                }
            };
            let resize_quality = match resize_quality {
                0 => ImageBitmapResizeQuality::Low,
                1 => ImageBitmapResizeQuality::Medium,
                2 => ImageBitmapResizeQuality::High,
                3 => ImageBitmapResizeQuality::Pixelated,
                _ => return {
                    JNI_FALSE
                }
            };

            let valid = loadFromPtrOptions(asset, ptr, size, flip_y, premultiply_alpha, color_space_conversion, resize_quality, resize_width, resize_height);
            if valid {
                return JNI_TRUE;
            }
            JNI_FALSE
        }
        Err(_) => JNI_FALSE,
    }
}


#[no_mangle]
pub extern "system" fn nativeLoadBitmapFromBufferOptions(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    buffer: JByteBuffer,
    flip_y: bool,
    premultiply_alpha: jint,
    color_space_conversion: jint,
    resize_quality: jint,
    resize_width: jfloat,
    resize_height: jfloat,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *const ImageAsset;

    if let (Ok(buf), Ok(size)) = (
        env.get_direct_buffer_address(&buffer),
        env.get_direct_buffer_capacity(&buffer),
    ) {
        let premultiply_alpha = match premultiply_alpha {
            0 => ImageBitmapPremultiplyAlpha::Default,
            1 => ImageBitmapPremultiplyAlpha::Premultiply,
            2 => ImageBitmapPremultiplyAlpha::AlphaNone,
            _ => return {
                JNI_FALSE
            }
        };
        let color_space_conversion = match color_space_conversion {
            0 => ImageBitmapColorSpaceConversion::Default,
            1 => ImageBitmapColorSpaceConversion::None,
            _ => return {
                JNI_FALSE
            }
        };
        let resize_quality = match resize_quality {
            0 => ImageBitmapResizeQuality::Low,
            1 => ImageBitmapResizeQuality::Medium,
            2 => ImageBitmapResizeQuality::High,
            3 => ImageBitmapResizeQuality::Pixelated,
            _ => return {
                JNI_FALSE
            }
        };
        let valid = loadFromPtrOptions(asset, buf, size, flip_y, premultiply_alpha, color_space_conversion, resize_quality, resize_width, resize_height);
        if valid {
            return JNI_TRUE;
        }
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn nativeLoadBitmapFromBufferRectOptions(
    env: JNIEnv,
    _: JClass,
    asset: jlong,
    buffer: JByteBuffer,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
    flip_y: bool,
    premultiply_alpha: jint,
    color_space_conversion: jint,
    resize_quality: jint,
    resize_width: jfloat,
    resize_height: jfloat,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *const ImageAsset;

    if let (Ok(buf), Ok(size)) = (
        env.get_direct_buffer_address(&buffer),
        env.get_direct_buffer_capacity(&buffer),
    ) {
        let premultiply_alpha = match premultiply_alpha {
            0 => ImageBitmapPremultiplyAlpha::Default,
            1 => ImageBitmapPremultiplyAlpha::Premultiply,
            2 => ImageBitmapPremultiplyAlpha::AlphaNone,
            _ => return {
                JNI_FALSE
            }
        };
        let color_space_conversion = match color_space_conversion {
            0 => ImageBitmapColorSpaceConversion::Default,
            1 => ImageBitmapColorSpaceConversion::None,
            _ => return {
                JNI_FALSE
            }
        };
        let resize_quality = match resize_quality {
            0 => ImageBitmapResizeQuality::Low,
            1 => ImageBitmapResizeQuality::Medium,
            2 => ImageBitmapResizeQuality::High,
            3 => ImageBitmapResizeQuality::Pixelated,
            _ => return {
                JNI_FALSE
            }
        };
        let valid = loadFromPtrRectOptions(asset, buf, size, x, y, width, height, flip_y, premultiply_alpha, color_space_conversion, resize_quality, resize_width, resize_height);
        if valid {
            return JNI_TRUE;
        }
    }
    JNI_FALSE
}


#[no_mangle]
pub unsafe extern "system" fn nativeLoadBitmapFromBytesRectOptions(
    mut env: JNIEnv,
    _: JClass,
    asset: jlong,
    byteArray: JByteArray,
    x: jfloat,
    y: jfloat,
    width: jfloat,
    height: jfloat,
    flip_y: bool,
    premultiply_alpha: jint,
    color_space_conversion: jint,
    resize_quality: jint,
    resize_width: jfloat,
    resize_height: jfloat,
) -> jboolean {
    if asset == 0 {
        return JNI_FALSE;
    }

    let asset = asset as *const ImageAsset;

    match env.get_array_elements_critical(&byteArray, ReleaseMode::NoCopyBack) {
        Ok(bytes) => {
            let size = bytes.len();
            let ptr = bytes.as_ptr() as *mut u8;

            let premultiply_alpha = match premultiply_alpha {
                0 => ImageBitmapPremultiplyAlpha::Default,
                1 => ImageBitmapPremultiplyAlpha::Premultiply,
                2 => ImageBitmapPremultiplyAlpha::AlphaNone,
                _ => return {
                    JNI_FALSE
                }
            };
            let color_space_conversion = match color_space_conversion {
                0 => ImageBitmapColorSpaceConversion::Default,
                1 => ImageBitmapColorSpaceConversion::None,
                _ => return {
                    JNI_FALSE
                }
            };
            let resize_quality = match resize_quality {
                0 => ImageBitmapResizeQuality::Low,
                1 => ImageBitmapResizeQuality::Medium,
                2 => ImageBitmapResizeQuality::High,
                3 => ImageBitmapResizeQuality::Pixelated,
                _ => return {
                    JNI_FALSE
                }
            };

            let valid = loadFromPtrRectOptions(asset, ptr, size, x, y, width, height, flip_y, premultiply_alpha, color_space_conversion, resize_quality, resize_width, resize_height);
            if valid {
                return JNI_TRUE;
            }
            JNI_FALSE
        }
        Err(_) => JNI_FALSE,
    }
}


//
// #[no_mangle]
// pub extern "system" fn nativeLoadFromAsset(
//    _env: JNIEnv,
//     _: JClass,
//     asset: jlong,
// ) -> jboolean {
//     if asset == 0 {
//         return JNI_FALSE;
//     }
//
//     let asset = asset as *const ImageAsset;
//
//     canvas_c::canvas_native_image_bitmap_create_from_asset(
//         asset, false,
//         ImageBitmapPremultiplyAlpha::Default,
//         ImageBitmapColorSpaceConversion::Default,
//         ImageBitmapResizeQuality::Low,
//         0., 0.,
//         asset,
//     )
//
// }
