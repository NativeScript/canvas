use std::os::raw::{c_int, c_longlong};

use crate::common::context::pixel_manipulation::image_data::ImageData;
use crate::common::ffi::u8_array::U8Array;

#[no_mangle]
pub extern "C" fn image_data_create(width: c_int, height: c_int) -> c_longlong {
    Box::into_raw(Box::new(ImageData::new(width, height))) as c_longlong
}

#[no_mangle]
pub extern "C" fn image_data_width(image_data: c_longlong) -> c_int {
    if image_data == 0 {
        return 0;
    }
    unsafe {
        let image_data: *mut ImageData = image_data as _;
        let image_data = &mut *image_data;
        image_data.width()
    }
}

#[no_mangle]
pub extern "C" fn image_data_height(image_data: c_longlong) -> c_int {
    if image_data == 0 {
        return 0;
    }
    unsafe {
        let image_data: *mut ImageData = image_data as _;
        let image_data = &mut *image_data;
        image_data.height()
    }
}

#[no_mangle]
pub extern "C" fn image_data_data(image_data: c_longlong) -> *mut u8 {
    if image_data == 0 {
        return std::ptr::null_mut();
    }
    unsafe {
        let image_data: *mut ImageData = image_data as _;
        let image_data = &mut *image_data;
        image_data.data
    }
}

#[no_mangle]
pub extern "C" fn image_data_data_length(image_data: c_longlong) -> usize {
    if image_data == 0 {
        return 0;
    }
    unsafe {
        let image_data: *mut ImageData = image_data as _;
        let image_data = &mut *image_data;
        image_data.data_len
    }
}

#[no_mangle]
pub extern "C" fn destroy_image_data(image_data: c_longlong) {
    if image_data == 0 {
        return;
    }
    unsafe {
        let image_data: *mut ImageData = image_data as _;
        let _ = Box::from_raw(image_data);
    }
}
