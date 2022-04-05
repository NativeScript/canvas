use std::os::raw::{c_char, c_longlong, c_uint};

use canvas_core::context::image_asset::{ImageAsset, OutputFormat};
use canvas_core::ffi::u8_array::U8Array;

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_create() -> *mut ImageAsset {
    Box::into_raw(Box::new(ImageAsset::new()))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_load_from_path(
    asset: *mut ImageAsset,
    path: *const c_char,
) -> bool {
    assert!(!asset.is_null());
    unsafe {
        let mut asset = &mut *asset;
        asset.load_from_path_raw(path)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_load_from_raw(
    asset: *mut ImageAsset,
    array: *const u8,
    size: usize,
) -> bool {
    assert!(!asset.is_null());
    unsafe {
        let asset = &mut *asset;
        asset.load_from_raw(array, size)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_get_bytes(asset: *mut ImageAsset) -> *mut U8Array {
    if asset.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let asset = &mut *asset;
        asset.bytes()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_get_rgba_bytes(asset: *mut ImageAsset) -> *mut U8Array {
    if asset.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let asset: *mut ImageAsset = asset as _;
        let asset = &mut *asset;
        asset.rgba_bytes()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_get_rgb_bytes(asset: *mut ImageAsset) -> *mut U8Array {
    if asset.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let asset = &mut *asset;
        asset.rgb_bytes()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_width(asset: *mut ImageAsset) -> c_uint {
    if asset.is_null() {
        return 0;
    }
    unsafe {
        let asset = &mut *asset;
        asset.width()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_height(asset: *mut ImageAsset) -> c_uint {
    if asset.is_null() {
        return 0;
    }
    unsafe {
        let asset = &mut *asset;
        asset.height()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_get_error(asset: *mut ImageAsset) -> *const c_char {
    if asset.is_null() {
        return std::ptr::null();
    }
    unsafe {
        let asset = &mut *asset;
        asset.error_cstr()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_has_error(asset: *mut ImageAsset) -> bool {
    assert!(!asset.is_null());
    unsafe {
        let asset = &mut *asset;
        if asset.error().is_empty() {
            return false;
        }
        true
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_scale(
    asset: *mut ImageAsset,
    x: c_uint,
    y: c_uint,
) -> bool {
    assert!(!asset.is_null());
    unsafe {
        let asset = &mut *asset;
        asset.scale(x, y)
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_flip_x(asset: *mut ImageAsset) -> bool {
    assert!(!asset.is_null());
    unsafe {
        let asset = &mut *asset;
        asset.flip_x()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_flip_x_in_place(asset: *mut ImageAsset) -> bool {
    assert!(!asset.is_null());
    unsafe {
        let asset = &mut *asset;
        asset.flip_x_in_place()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_flip_y(asset: *mut ImageAsset) -> bool {
    assert!(!asset.is_null());
    unsafe {
        let asset = &mut *asset;
        asset.flip_y()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_flip_y_in_place_owned(buf: *mut u8, length: usize) {
    if let Ok(mut image) =
        image::load_from_memory(unsafe { std::slice::from_raw_parts_mut(buf, length) })
    {
        image::imageops::flip_vertical_in_place(&mut image);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_flip_x_in_place_owned(buf: *mut u8, length: usize) {
    if let Ok(mut image) =
        image::load_from_memory(unsafe { std::slice::from_raw_parts_mut(buf, length) })
    {
        image::imageops::flip_horizontal_in_place(&mut image);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_flip_y_in_place(asset: *mut ImageAsset) -> bool {
    assert!(!asset.is_null());
    unsafe {
        let asset = &mut *asset;
        asset.flip_y_in_place()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_save_path(
    asset: *mut ImageAsset,
    path: *const c_char,
    format: c_uint,
) -> bool {
    assert!(!asset.is_null());
    unsafe {
        let asset = &mut *asset;
        asset.save_path_raw(path, OutputFormat::from(format))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_image_asset(asset: *mut ImageAsset) {
    if asset.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(asset);
    }
}
