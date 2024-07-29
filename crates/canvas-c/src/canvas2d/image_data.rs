use std::sync::Arc;

use crate::buffers::U8Buffer;

#[derive(Clone)]
pub struct ImageData(pub(crate) canvas_2d::context::pixel_manipulation::image_data::ImageData);

impl ImageData {
    pub(crate) fn new(data: canvas_2d::context::pixel_manipulation::image_data::ImageData) -> Self {
        ImageData(data)
    }

    pub fn inner(&self) -> &canvas_2d::context::pixel_manipulation::image_data::ImageData {
        &self.0
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_create(width: i32, height: i32) -> Arc<ImageData> {
    let data = canvas_2d::context::pixel_manipulation::ImageData::new(width, height);
    Arc::new(ImageData::new(data))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_width(image_data: &ImageData) -> i32 {
    image_data.0.width()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_height(image_data: &ImageData) -> i32 {
    image_data.0.height()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_data(image_data: &ImageData) -> *const U8Buffer {
    Arc::into_raw(Arc::new(U8Buffer::from(image_data.0.bytes_mut())))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_reference(value: *const ImageData) {
    if value.is_null() {
        return;
    }
    unsafe { Arc::increment_strong_count(value) };
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_release(value: *const ImageData) {
    if value.is_null() {
        return;
    }
    unsafe { Arc::decrement_strong_count(value) };
}
