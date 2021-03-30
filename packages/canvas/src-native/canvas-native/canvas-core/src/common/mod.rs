use std::os::raw::c_int;

use skia_safe::{
    AlphaType, ColorType, EncodedImageFormat, ImageInfo, IPoint, ISize, SamplingOptions, Size,
    Surface,
};
use skia_safe::image::CachingHint;

use crate::common::context::Context;

pub mod context;
pub mod ffi;
pub(crate) mod svg;
pub(crate) mod utils;

pub(crate) fn to_data_url(context: &mut Context, format: &str, quality: c_int) -> String {
    let surface = &mut context.surface;
    let image = surface.image_snapshot();
    let mut quality = quality;
    if quality > 100 || quality < 0 {
        quality = 92;
    }
    let mut encoded_prefix = String::new();
    encoded_prefix.push_str("data:");
    encoded_prefix.push_str(format);
    encoded_prefix.push_str(";base64,");
    let data = image.encode_to_data_with_quality(
        match format {
            "image/jpg" | "image/jpeg" => EncodedImageFormat::JPEG,
            "image/webp" => EncodedImageFormat::WEBP,
            "image/gif" => EncodedImageFormat::GIF,
            "image/heif" | "image/heic" | "image/heif-sequence" | "image/heic-sequence" => {
                EncodedImageFormat::HEIF
            }
            _ => EncodedImageFormat::PNG,
        },
        quality,
    );
    match data {
        Some(data) => {
            let encoded_data = base64::encode_config(data.as_bytes(), base64::STANDARD);
            let mut encoded = String::new();
            encoded.push_str(&encoded_prefix);
            encoded.push_str(&encoded_data);
            encoded
        }
        _ => {
            let mut encoded = String::new();
            encoded.push_str(&encoded_prefix);
            encoded.push_str("\"\"");
            encoded
        }
    }
}

pub(crate) fn to_data(context: &mut Context) -> Vec<u8> {
    let mut surface = &mut context.surface;
    let width = surface.width();
    let height = surface.height();
    let image = surface.image_snapshot();
    let mut info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let row_bytes = info.width() * 4;
    let mut pixels = vec![255u8; (row_bytes * info.height()) as usize];
    let _read = image.read_pixels(
        &mut info,
        pixels.as_mut_slice(),
        row_bytes as usize,
        IPoint::new(0, 0),
        CachingHint::Allow,
    );
    pixels
}

pub(crate) fn flush_custom_surface(context: *mut Context, width: i32, height: i32, dst: &mut [u8]) {
    unsafe {
        let context = &mut *context;
        context.surface.flush();
        let info = ImageInfo::new(
            ISize::new(width, height),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        if let Some(mut dst_surface) = Surface::new_raster_direct(&info, dst, None, None) {
            let dst_canvas = dst_surface.canvas();
            context.surface.draw(
                dst_canvas,
                Size::new(0.0, 0.0),
                SamplingOptions::from_filter_quality(skia_safe::FilterQuality::High, None),
                None,
            );
            context.surface.flush_and_submit();
            dst_surface.flush_and_submit();
        }
    }
}

pub(crate) fn snapshot_canvas(context: *mut Context) -> Option<Vec<u8>> {
    unsafe {
        if context.is_null() {
            return None;
        }
        let context = &mut *context;
        let surface = &mut context.surface;
        surface.flush_and_submit();
        let snapshot = surface.image_snapshot();
        if let Some(data) = snapshot.encode_to_data(EncodedImageFormat::PNG) {
            return Some(data.as_bytes().to_vec());
        }
        return None;
    }
}

pub(crate) fn snapshot_canvas_raw(context: *mut Context) -> Vec<u8> {
    unsafe {
        let context = &mut *context;
        let surface = &mut context.surface;
        let info = ImageInfo::new(
            ISize::new(surface.width(), surface.height()),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        );
        let len: usize = info.min_row_bytes() * (info.height() as usize);
        let mut bytes = vec![0u8; len];
        let mut dst_surface =
            Surface::new_raster_direct(&info, bytes.as_mut_slice(), None, None).unwrap();
        let mut dst_canvas = dst_surface.canvas();
        surface.draw(
            dst_canvas,
            Size::new(0.0, 0.0),
            SamplingOptions::from_filter_quality(skia_safe::FilterQuality::High, None),
            None,
        );
        surface.flush_and_submit();
        dst_surface.flush_and_submit();
        bytes
    }
}
