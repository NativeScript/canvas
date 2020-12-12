use std::os::raw::c_int;

use skia_safe::{AlphaType, ColorType, Data, Image, ImageInfo, ISize};

pub(crate) fn to_image(
    image_array: *const u8,
    image_size: usize,
    width: c_int,
    height: c_int,
) -> Option<Image> {
    let image_slice: &[u8] = unsafe { std::slice::from_raw_parts(image_array, image_size) };
    let info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );
    Image::from_raster_data(&info, Data::new_copy(image_slice), (width * 4) as usize)
}


pub(crate) fn to_image_encoded(image_array: *const u8, image_size: usize) -> Option<Image> {
    let image_slice: &[u8] = unsafe { std::slice::from_raw_parts(image_array, image_size) };
    Image::from_encoded(Data::new_copy(image_slice))
}


pub(crate) fn from_image_slice(
    image_slice: &[u8],
    width: c_int,
    height: c_int,
) -> Option<Image> {
    let info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );
    Image::from_raster_data(&info, Data::new_copy(image_slice), (width * 4) as usize)
}

pub(crate) fn from_image_slice_encoded(image_slice: &[u8]) -> Option<Image> {
    Image::from_encoded(Data::new_copy(image_slice))
}
