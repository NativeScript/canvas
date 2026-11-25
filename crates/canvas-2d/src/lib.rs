use std::ffi::c_uint;

use base64::Engine;
use skia_safe::gpu::BackendTexture;
use skia_safe::{images, AlphaType, ColorType, EncodedImageFormat, ImageInfo};

use context::Context;

pub mod context;
pub mod image_bitmap;
pub mod prelude;
pub mod utils;

const IMAGE_JPG: &'static str = "image/jpg";
const IMAGE_JPEG: &'static str = "image/jpeg";
const IMAGE_GIF: &'static str = "image/gif";
const IMAGE_WEBP: &'static str = "image/webp";
const IMAGE_HEIF: &'static str = "image/heif";
const IMAGE_HEIF_SEQUENCE: &'static str = "image/heif-sequence";
const IMAGE_HEIC: &'static str = "image/heic";
const IMAGE_HEIC_SEQUENCE: &'static str = "image/heic-sequence";

pub fn snapshot_to_backend_texture(context: &mut Context) -> Option<BackendTexture> {
    let snapshot = context.surface.image_snapshot();
    skia_safe::gpu::images::get_backend_texture_from_image(&snapshot, true)
        .map(|(texture, _)| texture)
}

pub fn bytes_to_data_url(
    width: i32,
    height: i32,
    bytes: &[u8],
    format: &str,
    quality: c_uint,
) -> String {
    let data = unsafe { skia_safe::Data::new_bytes(bytes) };
    let mut encoded_prefix = String::new();
    encoded_prefix.push_str("data:");
    encoded_prefix.push_str(format);
    encoded_prefix.push_str(";base64,");
    let image_info = ImageInfo::new((width, height), ColorType::N32, AlphaType::Unpremul, None);
    if let Some(image) = images::raster_from_data(&image_info, data, (width * 4) as usize) {
        let mut quality = quality;
        if quality > 100 {
            quality = 92;
        }
        let data_txt = "data:";
        let base_64_txt = ";base64,";
        let mut encoded_prefix =
            String::with_capacity(data_txt.len() + format.len() + base_64_txt.len());
        encoded_prefix.push_str("data:");
        encoded_prefix.push_str(format);
        encoded_prefix.push_str(";base64,");
        let data = image.encode(
            None,
            match format {
                IMAGE_JPG | IMAGE_JPEG => EncodedImageFormat::JPEG,
                IMAGE_WEBP => EncodedImageFormat::WEBP,
                IMAGE_GIF => EncodedImageFormat::GIF,
                IMAGE_HEIF | IMAGE_HEIC | IMAGE_HEIF_SEQUENCE | IMAGE_HEIC_SEQUENCE => {
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

/// `bytes` are RGBA, or BGRA when `bgra`: the source texture's format sets the order, not the
/// platform.
pub fn bytes_to_data_rgba8_url(
    width: i32,
    height: i32,
    bytes: &[u8],
    row_bytes: usize,
    bgra: bool,
    format: &str,
    quality: c_uint,
) -> String {
    let data = unsafe { skia_safe::Data::new_bytes(bytes) };
    let mut encoded_prefix = String::new();
    encoded_prefix.push_str("data:");
    encoded_prefix.push_str(format);
    encoded_prefix.push_str(";base64,");

    let fmt = if bgra { ColorType::BGRA8888 } else { ColorType::RGBA8888 };

    let image_info = ImageInfo::new((width, height), fmt, AlphaType::Unpremul, None);
    if let Some(image) = images::raster_from_data(&image_info, data, row_bytes) {
        let mut quality = quality;
        if quality > 100 {
            quality = 92;
        }
        let data_txt = "data:";
        let base_64_txt = ";base64,";
        let mut encoded_prefix =
            String::with_capacity(data_txt.len() + format.len() + base_64_txt.len());
        encoded_prefix.push_str("data:");
        encoded_prefix.push_str(format);
        encoded_prefix.push_str(";base64,");
        let data = image.encode(
            None,
            match format {
                IMAGE_JPG | IMAGE_JPEG => EncodedImageFormat::JPEG,
                IMAGE_WEBP => EncodedImageFormat::WEBP,
                IMAGE_GIF => EncodedImageFormat::GIF,
                IMAGE_HEIF | IMAGE_HEIC | IMAGE_HEIF_SEQUENCE | IMAGE_HEIC_SEQUENCE => {
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
                format!("{}{}", encoded_prefix, encoded_data)
            }
            _ => "data:,".to_string(),
        };
    }

    "data:,".to_string()
}
