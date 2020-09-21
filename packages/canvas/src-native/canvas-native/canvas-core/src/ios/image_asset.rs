use std::os::raw::{c_char, c_longlong, c_uint};

use libc::size_t;

use crate::common::{
    image_asset_flip_x, image_asset_flip_x_in_place_owned, image_asset_flip_y,
    image_asset_flip_y_in_place_owned, image_asset_free_bytes, image_asset_get_bytes,
    image_asset_get_error, image_asset_has_error, image_asset_height, image_asset_load_from_path,
    image_asset_load_from_raw, image_asset_release, image_asset_save_path, image_asset_scale,
    image_asset_width, NativeByteArray, NativeImageAsset,
};

#[no_mangle]
pub extern "C" fn native_image_asset_flip_y_in_place_owned(
    width: u32,
    height: u32,
    buf: *mut u8,
    length: usize,
) {
    image_asset_flip_y_in_place_owned(width, height, buf, length);
}

#[no_mangle]
pub extern "C" fn native_image_asset_flip_x_in_place_owned(
    width: u32,
    height: u32,
    buf: *mut u8,
    length: usize,
) {
    image_asset_flip_x_in_place_owned(width, height, buf, length)
}

#[no_mangle]
pub extern "C" fn native_create_image_asset() -> c_longlong {
    Box::into_raw(Box::new(NativeImageAsset::new())) as *mut _ as i64
}

#[no_mangle]
pub extern "C" fn native_image_asset_load_from_path(
    asset: c_longlong,
    path: *const c_char,
) -> c_uint {
    image_asset_load_from_path(asset, path)
}

#[no_mangle]
pub extern "C" fn native_image_asset_load_from_raw(
    asset: c_longlong,
    array: *const u8,
    size: size_t,
) -> c_uint {
    image_asset_load_from_raw(asset, array, size)
}

#[no_mangle]
pub extern "C" fn native_image_asset_get_bytes(asset: c_longlong) -> NativeByteArray {
    image_asset_get_bytes(asset)
}

#[no_mangle]
pub extern "C" fn native_image_asset_free_bytes(data: NativeByteArray) {
    image_asset_free_bytes(data)
}

#[no_mangle]
pub extern "C" fn native_image_asset_get_width(asset: c_longlong) -> c_uint {
    image_asset_width(asset)
}

#[no_mangle]
pub extern "C" fn native_image_asset_get_height(asset: c_longlong) -> c_uint {
    image_asset_height(asset)
}

#[no_mangle]
pub extern "C" fn native_image_asset_scale(asset: c_longlong, x: c_uint, y: c_uint) -> c_longlong {
    image_asset_scale(asset, x, y)
}

#[no_mangle]
pub extern "C" fn native_image_asset_flip_x(asset: c_longlong) -> c_longlong {
    image_asset_flip_x(asset)
}

#[no_mangle]
pub extern "C" fn native_image_asset_flip_y(asset: c_longlong) -> c_longlong {
    image_asset_flip_y(asset)
}

#[no_mangle]
pub extern "C" fn native_native_image_asset_save_path(
    asset: c_longlong,
    path: *const c_char,
    format: c_uint,
) -> c_uint {
    image_asset_save_path(asset, path, format)
}

#[no_mangle]
pub extern "C" fn native_image_asset_get_error(asset: c_longlong) -> *const c_char {
    image_asset_get_error(asset)
}

#[no_mangle]
pub extern "C" fn native_image_asset_has_error(asset: c_longlong) -> c_uint {
    image_asset_has_error(asset)
}

#[no_mangle]
pub extern "C" fn native_image_asset_release(asset: c_longlong) {
    image_asset_release(asset)
}
