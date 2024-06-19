use std::ffi::c_void;
use std::os::raw::c_int;

use skia_safe::{AlphaType, Bitmap, ColorType, Data, ISize, Image, ImageInfo};

pub fn from_backend_texture(
    context: &mut crate::context::Context,
    texture: &skia_safe::gpu::BackendTexture,
    origin: skia_safe::gpu::SurfaceOrigin,
    info: &ImageInfo,
) -> Option<Image> {
    if let Some(mut context) = context.surface.canvas().recording_context() {
        return Image::from_texture(
            &mut context,
            texture,
            origin,
            info.color_type(),
            info.alpha_type(),
            info.color_space(),
        );
    }
    None
}
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
        AlphaType::Unpremul,
        None,
    );
    skia_safe::images::raster_from_data(&info, Data::new_copy(image_slice), info.min_row_bytes())
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

    unsafe {
        skia_safe::images::raster_from_data(
            &info,
            Data::new_bytes(image_slice),
            info.min_row_bytes(),
        )
    }
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
            info.min_row_bytes(),
        );
    }

    skia_safe::images::raster_from_bitmap(&bm)
}

pub fn from_image_encoded_data(data: Data) -> Option<Image> {
    Image::from_encoded(data)
}

pub fn from_image_slice(image_slice: &[u8], width: c_int, height: c_int) -> Option<Image> {
    let info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );
    skia_safe::images::raster_from_data(&info, Data::new_copy(image_slice), info.min_row_bytes())
}

pub fn from_image_slice_encoded(image_slice: &[u8]) -> Option<Image> {
    Image::from_encoded(Data::new_copy(image_slice))
}

pub fn from_image_slice_encoded_no_copy(image_slice: &[u8]) -> Option<Image> {
    unsafe { Image::from_encoded(Data::new_bytes(image_slice)) }
}
