use crate::buffers::U8Buffer;

#[derive(Debug, Clone)]
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
pub extern "C" fn canvas_native_image_data_create(width: i32, height: i32) -> *mut ImageData {
    let data = canvas_2d::context::pixel_manipulation::ImageData::new(width, height);
    Box::into_raw(Box::new(ImageData::new(data)))
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
pub extern "C" fn canvas_native_image_data_get_length(image_data: &ImageData) -> usize {
    image_data.0.data_len()
}


#[no_mangle]
pub extern "C" fn canvas_native_image_data_get_data(image_data: &ImageData) -> *mut U8Buffer {
    Box::into_raw(Box::new(U8Buffer::from(image_data.clone())))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_data_release(value: *mut ImageData) {
    if value.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(value)) };
}
