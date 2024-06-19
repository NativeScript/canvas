use std::os::raw::{c_int, c_longlong};

use canvas_core::context::pixel_manipulation::image_data::ImageData;
use canvas_core::ffi::u8_array::U8Array;

#[no_mangle]
pub extern "C" fn canvas_native_image_data_create(width: c_int, height: c_int) -> *mut ImageData {
    Box::into_raw(Box::new(ImageData::new(width, height)))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_width(image_data: *mut ImageData) -> c_int {
    assert!(!image_data.is_null());
    unsafe {
        let image_data = &mut *image_data;
        image_data.width()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_height(image_data: *mut ImageData) -> c_int {
    assert!(!image_data.is_null());
    unsafe {
        let image_data = &mut *image_data;
        image_data.height()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_data(image_data: *mut ImageData) -> *mut u8 {
    assert!(!image_data.is_null());
    unsafe {
        let image_data = &mut *image_data;
        image_data.data_raw()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_data_length(image_data: *mut ImageData) -> usize {
    assert!(!image_data.is_null());
    unsafe {
        let image_data = &mut *image_data;
        image_data.data_len()
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_shared_instance(
    image_data: *mut ImageData,
) -> *mut ImageData {
    assert!(!image_data.is_null());
    unsafe { Box::into_raw(Box::new((&mut *image_data).clone())) }
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_image_data(image_data: *mut ImageData) {
    if image_data.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(image_data);
    }
}
