use std::os::raw::{c_char, c_longlong, c_uint};

use crate::common::context::image_asset::{ImageAsset, OutputFormat};
use crate::common::ffi::u8_array::U8Array;

#[no_mangle]
pub extern "C" fn image_asset_create() -> c_longlong {
    Box::into_raw(Box::new(ImageAsset::new())) as c_longlong
}

#[no_mangle]
pub extern "C" fn image_asset_load_from_path(asset: c_longlong, path: *const c_char) -> bool {
    if asset == 0 {
        return false;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        asset.load_from_path_raw(path)
    }
}

#[no_mangle]
pub extern "C" fn image_asset_load_from_raw(
    asset: c_longlong,
    array: *const u8,
    size: usize,
) -> bool {
    if asset == 0 {
        return false;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        asset.load_from_raw(array, size)
    }
}

#[no_mangle]
pub extern "C" fn image_asset_get_bytes(asset: c_longlong) -> *mut U8Array {
    if asset == 0 {
        return std::ptr::null_mut();
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        match asset.get_bytes() {
            None => 0 as _,
            Some(bytes) => U8Array::from(bytes.to_vec()).into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn image_asset_width(asset: c_longlong) -> c_uint {
    if asset == 0 {
        return 0;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        asset.width()
    }
}

#[no_mangle]
pub extern "C" fn image_asset_height(asset: c_longlong) -> c_uint {
    if asset == 0 {
        return 0;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        asset.height()
    }
}

#[no_mangle]
pub extern "C" fn image_asset_get_error(asset: c_longlong) -> *const c_char {
    if asset == 0 {
        return std::ptr::null();
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        asset.error_cstr()
    }
}

#[no_mangle]
pub extern "C" fn image_asset_has_error(asset: c_longlong) -> bool {
    if asset == 0 {
        return false;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        if asset.error().is_empty() {
            return false;
        }
        true
    }
}

#[no_mangle]
pub extern "C" fn image_asset_scale(asset: c_longlong, x: c_uint, y: c_uint) -> bool {
    if asset == 0 {
        return false;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        asset.scale(x, y)
    }
}

#[no_mangle]
pub extern "C" fn image_asset_save_path(
    asset: c_longlong,
    path: *const c_char,
    format: c_uint,
) -> bool {
    if asset == 0 {
        return false;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        asset.save_path_raw(path, OutputFormat::from(format))
    }
}

#[no_mangle]
pub extern "C" fn destroy_image_asset(asset: c_longlong) {
    if asset == 0 {
        return;
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let _ = Box::from_raw(asset);
    }
}
