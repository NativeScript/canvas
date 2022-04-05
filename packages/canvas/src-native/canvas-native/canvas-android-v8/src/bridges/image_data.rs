#[derive(Clone)]
pub struct ImageData(canvas_core::context::pixel_manipulation::image_data::ImageData);

impl ImageData {
    pub(crate) fn new(
        data: canvas_core::context::pixel_manipulation::image_data::ImageData,
    ) -> Self {
        ImageData(data)
    }

    pub fn inner(&self) -> &canvas_core::context::pixel_manipulation::image_data::ImageData {
        &self.0
    }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type ImageData;
        fn canvas_native_image_data_create(width: i32, height: i32) -> Box<ImageData>;
        fn canvas_native_image_data_width(image_data: &ImageData) -> i32;
        fn canvas_native_image_data_height(image_data: &ImageData) -> i32;
        fn canvas_native_image_data(image_data: &mut ImageData) -> &mut [u8];
        fn canvas_native_image_data_get_shared_instance(image_data: &mut ImageData) -> Box<ImageData>;
    }
}

pub fn canvas_native_image_data_create(width: i32, height: i32) -> Box<ImageData> {
    Box::new(ImageData(
        canvas_core::context::pixel_manipulation::ImageData::new(width, height),
    ))
}

pub fn canvas_native_image_data_width(image_data: &ImageData) -> i32 {
    image_data.0.width()
}

pub fn canvas_native_image_data_height(image_data: &ImageData) -> i32 {
    image_data.0.height()
}

pub fn canvas_native_image_data(image_data: &mut ImageData) -> &mut [u8] {
    image_data.0.data_mut()
}

pub fn canvas_native_image_data_get_shared_instance(image_data: &mut ImageData) -> Box<ImageData> {
    Box::new(image_data.clone())
}
