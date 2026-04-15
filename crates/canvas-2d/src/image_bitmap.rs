#![allow(dead_code)]
use std::sync::Arc;

use skia_safe::surfaces;

use canvas_core::image_asset::ImageAsset;

use crate::context::filter_quality::FilterQuality;
use crate::context::pixel_manipulation::image_data::ImageData;
use crate::utils::image::{from_image_slice, from_image_slice_encoded};

/// Wrap an owned value in an `Arc`, leak it as a raw pointer, and cast to `i64`.
macro_rules! into_raw_i64 {
    ($expr:expr) => {
        Arc::into_raw(Arc::new($expr)) as i64
    };
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ImageBitmapPremultiplyAlpha {
    Default,
    Premultiply,
    None,
}

impl Default for ImageBitmapPremultiplyAlpha {
    fn default() -> Self {
        Self::Default
    }
}

impl From<i32> for ImageBitmapPremultiplyAlpha {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Premultiply,
            2 => Self::None,
            _ => Self::Default,
        }
    }
}

impl Into<skia_safe::AlphaType> for ImageBitmapPremultiplyAlpha {
    fn into(self) -> skia_safe::AlphaType {
        match self {
            ImageBitmapPremultiplyAlpha::Default => skia_safe::AlphaType::Unpremul,
            ImageBitmapPremultiplyAlpha::Premultiply => skia_safe::AlphaType::Premul,
            ImageBitmapPremultiplyAlpha::None => skia_safe::AlphaType::Unpremul,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ImageBitmapColorSpaceConversion {
    Default,
    None,
}

impl Default for ImageBitmapColorSpaceConversion {
    fn default() -> Self {
        Self::Default
    }
}

impl From<i32> for ImageBitmapColorSpaceConversion {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::None,
            _ => Self::Default,
        }
    }
}

impl ImageBitmapColorSpaceConversion {
    pub fn to_color_space(&self) -> Option<skia_safe::ColorSpace> {
        match self {
            ImageBitmapColorSpaceConversion::Default => Some(skia_safe::ColorSpace::new_srgb()),
            ImageBitmapColorSpaceConversion::None => None,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ImageBitmapResizeQuality {
    Low,
    Medium,
    High,
    Pixelated,
}

impl Default for ImageBitmapResizeQuality {
    fn default() -> Self {
        Self::Low
    }
}

impl From<i32> for ImageBitmapResizeQuality {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Low,
            1 => Self::Medium,
            2 => Self::High,
            3 => Self::Pixelated,
            _ => Self::Low,
        }
    }
}

impl ImageBitmapResizeQuality {
    pub fn to_quality(&self) -> skia_safe::SamplingOptions {
        match self {
            ImageBitmapResizeQuality::Low => FilterQuality::Low.into(),
            ImageBitmapResizeQuality::Medium => FilterQuality::Medium.into(),
            ImageBitmapResizeQuality::High => FilterQuality::High.into(),
            ImageBitmapResizeQuality::Pixelated => FilterQuality::None.into(),
        }
    }
}

pub fn create_from_image_data_raw(
    image_data: i64,
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> i64 {
    let data = unsafe { &*(image_data as *const ImageData) };
    into_raw_i64!(create_from_image_data(
        data,
        rect,
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
    ))
}

pub fn create_from_image_data(
    image_data: &ImageData,
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> ImageAsset {
    let bytes = image_data.data();
    let width = image_data.width() as f32;
    let height = image_data.height() as f32;
    create_image_asset(
        bytes,
        width,
        height,
        rect,
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
    )
}

pub(crate) fn create_image_bitmap_raw(
    image: skia_safe::Image,
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> i64 {
    into_raw_i64!(create_image_bitmap(
        image,
        rect,
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
    ))
}

pub(crate) fn create_image_bitmap_internal(
    image: skia_safe::Image,
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
    output: &ImageAsset,
) {
    let img_w = image.width() as f32;
    let img_h = image.height() as f32;

    let is_identity = !flip_y && rect.is_none() && resize_width <= 0. && resize_height <= 0.;

    if is_identity {
        let alpha_type = ImageBitmapPremultiplyAlpha::from(premultiply_alpha).into();
        let row_bytes = (img_w as usize) * 4;
        let mut pixels = vec![0_u8; img_h as usize * row_bytes];
        let read_info = skia_safe::ImageInfo::new(
            image.dimensions(),
            skia_safe::ColorType::RGBA8888,
            alpha_type,
            ImageBitmapColorSpaceConversion::from(color_space_conversion).to_color_space(),
        );
        if image.read_pixels(
            &read_info,
            pixels.as_mut_slice(),
            row_bytes,
            (0, 0),
            skia_safe::image::CachingHint::Allow,
        ) {
            output.load_from_raw_bytes_rgba(img_w as u32, img_h as u32, pixels);
        }
        return;
    }

    // Only one of resize_width / resize_height being set means proportional scale.
    let (mut out_width, mut out_height) = (img_w, img_h);

    if resize_width > 0. && resize_height > 0. {
        out_width = resize_width;
        out_height = resize_height;
    } else if resize_width > 0. {
        let ratio = resize_width / img_w;
        out_width = resize_width;
        out_height = img_h * ratio;
    } else if resize_height > 0. {
        let ratio = resize_height / img_h;
        out_width = img_w * ratio;
        out_height = resize_height;
    }

    let source_rect = match rect {
        None => skia_safe::Rect::from_xywh(0., 0., img_w, img_h),
        Some(r) => {
            let sr = skia_safe::Rect::from_xywh(r.0, r.1, r.2, r.3);
            if resize_width == 0. && resize_height == 0. {
                out_width = sr.width();
                out_height = sr.height();
            }
            sr
        }
    };

    let alpha_type = ImageBitmapPremultiplyAlpha::from(premultiply_alpha).into();
    let color_space =
        ImageBitmapColorSpaceConversion::from(color_space_conversion).to_color_space();

    let image_info = skia_safe::ImageInfo::new(
        (source_rect.width() as i32, source_rect.height() as i32),
        skia_safe::ColorType::RGBA8888,
        alpha_type,
        color_space,
    );

    let mut surface =
        match surfaces::raster(&image_info, Some((source_rect.width() * 4.) as usize), None) {
            None => return,
            Some(s) => s,
        };

    {
        let canvas = surface.canvas();
        if flip_y {
            canvas.translate(skia_safe::Vector::new(0., source_rect.height()));
            canvas.scale((1., -1.));
        }
        let mut paint = skia_safe::Paint::default();
        paint.set_anti_alias(true);
        canvas.draw_image(&image, (-source_rect.x(), -source_rect.y()), Some(&paint));
    }

    let snapshot = surface.image_snapshot();
    let needs_resize =
        snapshot.width() != out_width as i32 || snapshot.height() != out_height as i32;

    if needs_resize {
        let resize_info = image_info.with_dimensions((out_width as i32, out_height as i32));
        let row_bytes = (out_width * 4.) as usize;
        let mut pixels = vec![0_u8; out_height as usize * row_bytes];

        if let Some(pixel_map) =
            skia_safe::Pixmap::new(&resize_info, pixels.as_mut_slice(), row_bytes)
        {
            let _ = snapshot.scale_pixels(
                &pixel_map,
                ImageBitmapResizeQuality::from(resize_quality).to_quality(),
                None,
            );
        }
        output.load_from_raw_bytes_rgba(out_width as u32, out_height as u32, pixels);
    } else {
        let row_bytes = (snapshot.width() * 4) as usize;
        let mut pixels = vec![0_u8; snapshot.height() as usize * row_bytes];
        let read_info = skia_safe::ImageInfo::new(
            snapshot.dimensions(),
            skia_safe::ColorType::RGBA8888,
            alpha_type,
            None,
        );
        if snapshot.read_pixels(
            &read_info,
            pixels.as_mut_slice(),
            row_bytes,
            (0, 0),
            skia_safe::image::CachingHint::Allow,
        ) {
            output.load_from_raw_bytes_rgba(
                snapshot.width() as u32,
                snapshot.height() as u32,
                pixels,
            );
        }
    }
}

pub(crate) fn create_image_bitmap(
    image: skia_safe::Image,
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> ImageAsset {
    let mut output = ImageAsset::new();
    create_image_bitmap_internal(
        image,
        rect,
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
        &mut output,
    );
    output
}

pub fn create_image_asset_raw(
    buf: &[u8],
    image_width: f32,
    image_height: f32,
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> i64 {
    into_raw_i64!(create_image_asset(
        buf,
        image_width,
        image_height,
        rect,
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
    ))
}

pub fn create_image_asset(
    buf: &[u8],
    image_width: f32,
    image_height: f32,
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> ImageAsset {
    match from_image_slice(buf, image_width as i32, image_height as i32) {
        None => ImageAsset::new(),
        Some(image) => create_image_bitmap(
            image,
            rect,
            flip_y,
            premultiply_alpha,
            color_space_conversion,
            resize_quality,
            resize_width,
            resize_height,
        ),
    }
}

pub fn create_image_asset_with_output(
    buf: &[u8],
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
    output: &ImageAsset,
) {
    if let Some(image) = from_image_slice_encoded(buf) {
        create_image_bitmap_internal(
            image,
            rect,
            flip_y,
            premultiply_alpha,
            color_space_conversion,
            resize_quality,
            resize_width,
            resize_height,
            output,
        );
    }
}

pub fn create_from_image_asset_src_rect_raw(
    image_asset: i64,
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> i64 {
    let asset = unsafe { &*(image_asset as *const ImageAsset) };
    into_raw_i64!(create_from_image_asset_src_rect(
        asset,
        rect,
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
    ))
}

pub fn create_from_image_asset_src_rect(
    image_asset: &ImageAsset,
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> ImageAsset {
    let mut ret = ImageAsset::new();
    image_asset.with_bytes_dimension(|bytes, (width, height)| {
        if width != 0 && height != 0 {
            ret = create_image_asset(
                bytes,
                width as f32,
                height as f32,
                rect,
                flip_y,
                premultiply_alpha,
                color_space_conversion,
                resize_quality,
                resize_width,
                resize_height,
            );
        }
    });
    ret
}

pub fn create_image_asset_encoded(
    buf: &[u8],
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> ImageAsset {
    match from_image_slice_encoded(buf) {
        None => ImageAsset::new(),
        Some(image) => create_image_bitmap(
            image,
            rect,
            flip_y,
            premultiply_alpha,
            color_space_conversion,
            resize_quality,
            resize_width,
            resize_height,
        ),
    }
}

pub fn create_image_asset_encoded_raw(
    buf: &[u8],
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> i64 {
    into_raw_i64!(create_image_asset_encoded(
        buf,
        rect,
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
    ))
}
