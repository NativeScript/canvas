use std::ffi::{c_void, CString};
use std::os::raw::c_int;

use skia_safe::{AlphaType, Bitmap, ColorSpace, ColorType, Data, Image, ImageInfo, ISize, Pixmap};

use crate::context::image_asset::ImageAsset;

pub fn to_image(
    image_array: *const u8,
    image_size: usize,
    width: c_int,
    height: c_int,
) -> Option<Image> {
    let image_slice: &[u8] = unsafe { std::slice::from_raw_parts(image_array, image_size) };
    let info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    Image::from_raster_data(&info, Data::new_copy(image_slice), (width * 4) as usize)
}

pub fn to_image_encoded(image_array: *const u8, image_size: usize) -> Option<Image> {
    let image_slice: &[u8] = unsafe { std::slice::from_raw_parts(image_array, image_size) };
    Image::from_encoded(Data::new_copy(image_slice))
}

pub fn to_image_encoded_from_data(data: Data) -> Option<Image> {
    Image::from_encoded(data)
}

pub fn from_image_slice_no_copy(image_slice: &[u8], width: c_int, height: c_int) -> Option<Image> {
    let info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );

    unsafe { Image::from_raster_data(&info, Data::new_bytes(image_slice), (width * 4) as usize) }
}

pub fn from_bitmap_slice(image_slice: &[u8], width: c_int, height: c_int) -> Option<Image> {
    let info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );

    let mut bm = Bitmap::new();
    unsafe {
        bm.install_pixels(
            &info,
            image_slice.as_ptr() as *mut c_void,
            (width * 4) as usize,
        );
    }

    Image::from_bitmap(&bm)
}

pub fn from_image_slice(image_slice: &[u8], width: c_int, height: c_int) -> Option<Image> {
    let info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );
    Image::from_raster_data(&info, Data::new_copy(image_slice), (width * 4) as usize)
}

pub fn from_image_slice_encoded(image_slice: &[u8]) -> Option<Image> {
    Image::from_encoded(Data::new_copy(image_slice))
}

pub fn from_image_slice_encoded_no_copy(image_slice: &[u8]) -> Option<Image> {
    unsafe { Image::from_encoded(Data::new_bytes(image_slice)) }
}