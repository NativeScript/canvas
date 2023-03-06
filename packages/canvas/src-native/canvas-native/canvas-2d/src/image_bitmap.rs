#![allow(dead_code)]

use core::convert::{From, Into};

use skia_safe::EncodedImageFormat;

use canvas_core::image_asset::ImageAsset;

use crate::context::filter_quality::FilterQuality;
use crate::context::pixel_manipulation::image_data::ImageData;
use crate::utils::image::{from_image_slice, from_image_slice_encoded};

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
        return match self {
            ImageBitmapColorSpaceConversion::Default => Some(skia_safe::ColorSpace::new_srgb()),
            ImageBitmapColorSpaceConversion::None => None,
        };
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
        return match self {
            ImageBitmapResizeQuality::Low => FilterQuality::Low.into(),
            ImageBitmapResizeQuality::Medium => FilterQuality::Medium.into(),
            ImageBitmapResizeQuality::High => FilterQuality::High.into(),
            ImageBitmapResizeQuality::Pixelated => FilterQuality::None.into(),
        };
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
    unsafe {
        let data: *mut ImageData = image_data as _;
        let data = &mut *data;

        Box::into_raw(Box::new(create_from_image_data(
            data,
            rect,
            flip_y,
            premultiply_alpha,
            color_space_conversion,
            resize_quality,
            resize_width,
            resize_height,
        ))) as i64
    }
}

pub fn create_from_image_data(
    image_data: &mut ImageData,
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
    Box::into_raw(Box::new(create_image_bitmap(
        image,
        rect,
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_width,
        resize_height,
    ))) as i64
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
    output: &mut ImageAsset,
) {
    let mut out_width = image.width() as f32;
    let mut out_height = image.height() as f32;

    if resize_width <= 0. && resize_height > 0. {
        out_width = image.width() as f32 + resize_height / image.height() as f32;
    }

    if resize_height <= 0. && resize_width > 0. {
        out_height = image.height() as f32 + resize_width / image.width() as f32;
    }

    if resize_width > 0. && resize_height > 0. {
        out_width = resize_width;
        out_height = resize_height;
    }

    let source_rect;

    match rect {
        None => {
            source_rect =
                skia_safe::Rect::from_xywh(0., 0., image.width() as f32, image.height() as f32);
        }
        Some(rect) => {
            source_rect = skia_safe::Rect::from_xywh(rect.0, rect.1, rect.2, rect.3);
            if resize_width == 0. && resize_height == 0. {
                out_width = rect.1;
                out_height = rect.2;
            }
        }
    }

    let image_info = skia_safe::ImageInfo::new(
        (source_rect.width() as i32, source_rect.height() as i32),
        skia_safe::ColorType::N32,
        ImageBitmapPremultiplyAlpha::from(premultiply_alpha).into(),
        ImageBitmapColorSpaceConversion::from(color_space_conversion).to_color_space(),
    );
    match skia_safe::Surface::new_raster(
        &image_info,
        Some((source_rect.width() * 4.) as usize),
        None,
    ) {
        None => {}
        Some(mut surface) => {
            let canvas = surface.canvas();
            if flip_y {
                canvas.translate(skia_safe::Vector::new(0., source_rect.height() as f32));
                canvas.scale((1., -1.));
            }

            let mut paint = skia_safe::Paint::default();
            paint.set_anti_alias(true);

            surface
                .canvas()
                .draw_image(&image, (source_rect.x(), source_rect.y()), Some(&paint));

            let image = surface.image_snapshot();
            let data;
            if image.width() != out_width as i32 && image.height() != out_height as i32 {
                let resize_info = image_info.with_dimensions((out_width as i32, out_height as i32));

                let mut bytes = vec![0_u8; (out_width * out_height * 4.) as usize];
                let pixel_map = skia_safe::Pixmap::new(
                    &resize_info,
                    bytes.as_mut_slice(),
                    (out_width * 4.) as usize,
                );
                let _ = image.scale_pixels(
                    &pixel_map,
                    ImageBitmapResizeQuality::from(resize_quality).to_quality(),
                    None,
                );

                data = pixel_map.encode(EncodedImageFormat::PNG, 100);
            } else {
                data = image.encode_to_data(EncodedImageFormat::PNG);
            }

            if let Some(data) = data {
                output.load_from_bytes(data.as_bytes());
            };
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
    Box::into_raw(Box::new(create_image_asset(
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
    ))) as i64
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
    output: &mut ImageAsset,
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
    unsafe {
        let asset: *mut ImageAsset = image_asset as _;
        let asset = &mut *asset;
        return Box::into_raw(Box::new(create_from_image_asset_src_rect(
            asset,
            rect,
            flip_y,
            premultiply_alpha,
            color_space_conversion,
            resize_quality,
            resize_width,
            resize_height,
        ))) as i64;
    }
}

pub fn create_from_image_asset_src_rect(
    image_asset: &mut ImageAsset,
    rect: Option<(f32, f32, f32, f32)>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> ImageAsset {
    unsafe {
        if let Some(bytes) = image_asset.get_bytes() {
            let width = image_asset.width() as f32;
            let height = image_asset.height() as f32;
            return create_image_asset(
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
            );
        }
        ImageAsset::new()
    }
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
    Box::into_raw(Box::new(create_image_asset_encoded(
        buf,
        rect,
        flip_y,
        premultiply_alpha,
        color_space_conversion,
        resize_quality,
        resize_height,
        resize_width,
    ))) as i64
}
