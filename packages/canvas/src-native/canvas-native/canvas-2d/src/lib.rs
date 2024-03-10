extern crate core;

use std::ffi::c_uint;

use base64::Engine;
use skia_safe::gpu::BackendTexture;
use skia_safe::image::CachingHint;
use skia_safe::wrapper::NativeTransmutableWrapper;
use skia_safe::{
    images, surfaces, AlphaType, ColorType, EncodedImageFormat, IPoint, ISize, ImageInfo, Point,
};

use context::filter_quality::FilterQuality;
use context::{Context, ContextWrapper};

pub mod context;
pub mod ffi;
pub mod image_bitmap;
pub mod ios;
pub mod prelude;
pub mod svg;
pub mod utils;

const GR_GL_TEXTURE_2D: c_uint = 0x0DE1;

pub fn snapshot_to_backend_texture(context: &mut Context) -> Option<BackendTexture> {
    let snapshot = context.surface.image_snapshot();
    skia_safe::gpu::images::get_backend_texture_from_image(&snapshot, true)
        .map(|(texture, _)| texture)
}

pub fn to_data_url_context(context: &mut Context, format: &str, quality: c_uint) -> String {
    if context.device.is_np {
        return "data:,".to_string();
    }
    let mut ctx = context.surface.direct_context();
    let image = context
        .surface
        .image_snapshot()
        .make_raster_image(&mut ctx, Some(CachingHint::Allow));

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
        let data = image.encode(
            &mut ctx,
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

pub fn to_data_url(context: &mut ContextWrapper, format: &str, quality: c_uint) -> String {
    let mut context = context.get_context_mut();

    if context.device.is_np {
        return "data:,".to_string();
    }

    let mut ctx = context.surface.direct_context();
    let image = context
        .surface
        .image_snapshot()
        .make_raster_image(&mut ctx, Some(CachingHint::Disallow));

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
        let data = image.encode(
            &mut ctx,
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
        let data = image.encode(
            None,
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
    if context.device.is_np {
        return vec![];
    }

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
    if context.device.is_np {
        return vec![];
    }
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
        if context.device.is_np {
            return;
        }
        if let Some(mut context) = context.surface.direct_context() {
            context.flush_and_submit();
        }
        let info = ImageInfo::new(
            ISize::new(width, height),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        if let Some(mut dst_surface) = surfaces::wrap_pixels(&info, dst, None, None) {
            let dst_canvas = dst_surface.canvas();
            context
                .surface
                .draw(dst_canvas, Point::new(0., 0.), FilterQuality::High, None);

            if let Some(mut context) = context.surface.direct_context() {
                context.flush_and_submit();
            }

            if let Some(mut context) = dst_surface.direct_context() {
                context.flush_and_submit();
            }
        }
    }
}

pub(crate) fn snapshot_canvas(context: &mut ContextWrapper) -> Option<Vec<u8>> {
    unsafe {
        let mut context = context.get_context_mut();

        if context.device.is_np {
            return None;
        }

        if let Some(mut context) = context.surface.direct_context() {
            context.flush_and_submit();
        }

        let snapshot = context.surface.image_snapshot();
        let mut ctx = context.surface.direct_context();
        if let Some(data) = snapshot.encode(&mut ctx, EncodedImageFormat::PNG, None) {
            return Some(data.as_bytes().to_vec());
        }
        return None;
    }
}

pub(crate) fn snapshot_canvas_raw(context: &mut ContextWrapper) -> Vec<u8> {
    unsafe {
        let mut context = context.get_context_mut();

        if context.device.is_np {
            return vec![];
        }

        let info = ImageInfo::new(
            ISize::new(context.surface.width(), context.surface.height()),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        );
        let len: usize = info.min_row_bytes() * (info.height() as usize);
        let mut bytes = vec![0u8; len];
        let mut dst_surface =
            surfaces::wrap_pixels(&info, bytes.as_mut_slice(), None, None).unwrap();
        let dst_canvas = dst_surface.canvas();
        context
            .surface
            .draw(dst_canvas, Point::new(0., 0.), FilterQuality::High, None);

        if let Some(mut context) = context.surface.direct_context() {
            context.flush_and_submit();
        }

        if let Some(mut context) = dst_surface.direct_context() {
            context.flush_and_submit();
        }

        bytes
    }
}
