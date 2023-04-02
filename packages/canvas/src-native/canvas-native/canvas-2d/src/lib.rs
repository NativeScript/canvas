extern crate core;

use std::os::raw::c_int;

use base64::Engine;
use image::EncodableLayout;
use skia_safe::image::CachingHint;
use skia_safe::wrapper::NativeTransmutableWrapper;
use skia_safe::{
    AlphaType, ColorType, EncodedImageFormat, IPoint, ISize, ImageInfo, Point, Surface,
};

use context::filter_quality::FilterQuality;
use context::{Context, ContextWrapper};

pub mod context;
pub mod ffi;
pub mod image_bitmap;
pub mod prelude;
pub mod svg;
pub mod utils;

pub mod ios;

pub fn to_data_url_context(context: &mut Context, format: &str, quality: c_int) -> String {
    let image = context
        .surface
        .image_snapshot()
        .to_raster_image(Some(CachingHint::Allow));

    if let Some(image) = image {
        let mut quality = quality;
        if quality > 100 || quality < 0 {
            quality = 92;
        }
        let data_txt = "data:";
        let base_64_txt = ";base64,";
        let mut encoded_prefix =
            String::with_capacity(data_txt.len() + format.len() + base_64_txt.len());
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
        return match data {
            Some(data) => {
                let encoded_data =
                    base64::engine::general_purpose::STANDARD.encode(data.as_bytes());
                if encoded_data.is_empty() {
                    return "data:,".to_string();
                }
                let mut encoded = String::with_capacity(encoded_prefix.len() + encoded_data.len());
                encoded.push_str(&encoded_prefix);
                encoded.push_str(&encoded_data);
                encoded
            }
            _ => "data:,".to_string(),
        };
    }

    "data:,".to_string()
}

pub fn to_data_url(context: &mut ContextWrapper, format: &str, quality: c_int) -> String {
    let mut context = context.get_context_mut();
    let image = context
        .surface
        .image_snapshot()
        .to_raster_image(Some(CachingHint::Allow));

    if let Some(image) = image {
        let mut quality = quality;
        if quality > 100 || quality < 0 {
            quality = 92;
        }
        let data_txt = "data:";
        let base_64_txt = ";base64,";
        let mut encoded_prefix =
            String::with_capacity(data_txt.len() + format.len() + base_64_txt.len());
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
        return match data {
            Some(data) => {
                let encoded_data =
                    base64::engine::general_purpose::STANDARD.encode(data.as_bytes());
                if encoded_data.is_empty() {
                    return "data:,".to_string();
                }
                let mut encoded = String::with_capacity(encoded_prefix.len() + encoded_data.len());
                encoded.push_str(&encoded_prefix);
                encoded.push_str(&encoded_data);
                encoded
            }
            _ => "data:,".to_string(),
        };
    }

    "data:,".to_string()
}

pub fn bytes_to_data_url(
    width: i32,
    height: i32,
    bytes: &[u8],
    format: &str,
    quality: c_int,
) -> String {
    let data = unsafe { skia_safe::Data::new_bytes(bytes) };
    let mut encoded_prefix = String::new();
    encoded_prefix.push_str("data:");
    encoded_prefix.push_str(format);
    encoded_prefix.push_str(";base64,");
    let image_info = ImageInfo::new(
        (width, height),
        ColorType::N32,
        skia_bindings::SkAlphaType::Unpremul,
        None,
    );
    if let Some(image) = skia_safe::Image::from_raster_data(&image_info, data, (width * 4) as usize)
    {
        let mut quality = quality;
        if quality > 100 || quality < 0 {
            quality = 92;
        }
        let data_txt = "data:";
        let base_64_txt = ";base64,";
        let mut encoded_prefix =
            String::with_capacity(data_txt.len() + format.len() + base_64_txt.len());
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
        return match data {
            Some(data) => {
                let encoded_data =
                    base64::engine::general_purpose::STANDARD.encode(data.as_bytes());
                if encoded_data.is_empty() {
                    return "data:,".to_string();
                }
                let mut encoded = String::with_capacity(encoded_prefix.len() + encoded_data.len());
                encoded.push_str(&encoded_prefix);
                encoded.push_str(&encoded_data);
                encoded
            }
            _ => "data:,".to_string(),
        };
    }

    return "data:,".to_string();
}

pub(crate) fn to_data_with_context(context: &mut Context) -> Vec<u8> {
    let width = context.surface.width();
    let height = context.surface.height();
    let image = context.surface.image_snapshot();
    let mut info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
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

pub(crate) fn to_data(context: &mut ContextWrapper) -> Vec<u8> {
    let mut context = context.get_context_mut();
    let width = context.surface.width();
    let height = context.surface.height();
    let image = context.surface.image_snapshot();
    let mut info = ImageInfo::new(
        ISize::new(width, height),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
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

pub(crate) fn flush_custom_surface(
    context: &mut ContextWrapper,
    width: i32,
    height: i32,
    dst: &mut [u8],
) {
    unsafe {
        let mut context = context.get_context_mut();
        context.surface.flush();
        let info = ImageInfo::new(
            ISize::new(width, height),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        if let Some(mut dst_surface) = Surface::new_raster_direct(&info, dst, None, None) {
            let dst_canvas = dst_surface.canvas();
            context
                .surface
                .draw(dst_canvas, Point::new(0., 0.), FilterQuality::High, None);
            context.surface.flush_and_submit();
            dst_surface.flush_and_submit();
        }
    }
}

pub(crate) fn snapshot_canvas(context: &mut ContextWrapper) -> Option<Vec<u8>> {
    unsafe {
        let mut context = context.get_context_mut();
        context.surface.flush_and_submit();
        let snapshot = context.surface.image_snapshot();
        if let Some(data) = snapshot.encode_to_data(EncodedImageFormat::PNG) {
            return Some(data.as_bytes().to_vec());
        }
        return None;
    }
}

pub(crate) fn snapshot_canvas_raw(context: &mut ContextWrapper) -> Vec<u8> {
    unsafe {
        let mut context = context.get_context_mut();
        let info = ImageInfo::new(
            ISize::new(context.surface.width(), context.surface.height()),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        );
        let len: usize = info.min_row_bytes() * (info.height() as usize);
        let mut bytes = vec![0u8; len];
        let mut dst_surface =
            Surface::new_raster_direct(&info, bytes.as_mut_slice(), None, None).unwrap();
        let dst_canvas = dst_surface.canvas();
        context
            .surface
            .draw(dst_canvas, Point::new(0., 0.), FilterQuality::High, None);
        context.surface.flush_and_submit();
        dst_surface.flush_and_submit();
        bytes
    }
}
