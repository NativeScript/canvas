use std::os::raw::{c_float, c_int, c_longlong};

use crate::common::context::image_asset::ImageAsset;
use crate::common::image_bitmap::create_image_asset;

#[no_mangle]
pub extern "C" fn image_bitmap_create_from_bytes(
    image_bytes: *const u8,
    image_size: usize,
    image_width: c_float,
    image_height: c_float,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: c_float,
    resize_height: c_float,
) -> c_longlong {
    if image_bytes.is_null() || image_size == 0 {
        return Box::into_raw(Box::new(ImageAsset::new())) as c_longlong
    }
    let slice = unsafe { std::slice::from_raw_parts(image_bytes, image_size) };

    create_image_asset(
        slice,
        image_width,
        image_height,
        None,
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
    )
}

#[no_mangle]
pub extern "C" fn image_bitmap_create_from_bytes_src_rect(
    image_bytes: *const u8,
    image_size: usize,
    image_width: c_float,
    image_height: c_float,
    sx: c_float,
    sy: c_float,
    s_width: c_float,
    s_height: c_float,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: c_float,
    resize_height: c_float,
) -> c_longlong {
    if image_bytes.is_null() || image_size == 0 {
        return Box::into_raw(Box::new(ImageAsset::new())) as c_longlong
    }
    let slice = unsafe { std::slice::from_raw_parts(image_bytes, image_size) };
    create_image_asset(
        slice,
        image_width,
        image_height,
        Some(skia_safe::Rect::from_xywh(sx, sy, s_width, s_height)),
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
    )
}
