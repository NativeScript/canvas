use core::convert::{From, Into};
use core::default::Default;
use core::option::Option;
use core::option::Option::{None, Some};

use skia_safe::{Point, RCHandle, Rect};

use crate::common::context::image_asset::ImageAsset;
use crate::common::utils::image::from_image_slice;

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
            ImageBitmapResizeQuality::Low => skia_safe::SamplingOptions::from_filter_quality(
                skia_safe::FilterQuality::Low,
                skia_safe::sampling_options::MediumBehavior::AsMipmapLinear,
            ),
            ImageBitmapResizeQuality::Medium => skia_safe::SamplingOptions::from_filter_quality(
                skia_safe::FilterQuality::Medium,
                skia_safe::sampling_options::MediumBehavior::AsMipmapLinear,
            ),
            ImageBitmapResizeQuality::High => skia_safe::SamplingOptions::from_filter_quality(
                skia_safe::FilterQuality::High,
                skia_safe::sampling_options::MediumBehavior::AsMipmapLinear,
            ),
            ImageBitmapResizeQuality::Pixelated => skia_safe::SamplingOptions::from_filter_quality(
                skia_safe::FilterQuality::None,
                skia_safe::sampling_options::MediumBehavior::AsMipmapLinear,
            ),
        };
    }
}

pub(crate) fn create_image_bitmap(
    image: skia_safe::Image,
    rect: Option<skia_safe::Rect>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> i64 {
    let mut output = ImageAsset::new();
    let mut original_surface_buffer = vec![0_u8; (image.width() * image.height() * 4) as usize];
    let image_info = skia_safe::ImageInfo::new(
        image.dimensions(),
        skia_safe::ColorType::RGBA8888,
        ImageBitmapPremultiplyAlpha::from(premultiply_alpha).into(),
        ImageBitmapColorSpaceConversion::from(color_space_conversion).to_color_space(),
    );
    match skia_safe::Surface::new_raster_direct(
        &image_info,
        original_surface_buffer.as_mut_slice(),
        Some((image.width() * 4) as usize),
        None,
    ) {
        None => {}
        Some(mut surface) => {
            let canvas = surface.canvas();
            if flip_y {
                canvas.translate(skia_safe::Vector::new(0., image.height() as f32));
                canvas.scale((1., -1.));
            }
            let mut paint = skia_safe::Paint::default();
            paint.set_anti_alias(true);
            match rect {
                None => {
                    surface.canvas().draw_image_with_sampling_options(
                        &image,
                        (0, 0),
                        skia_safe::SamplingOptions::default(),
                        Some(&paint),
                    );
                }
                Some(rect) => {
                    surface.canvas().draw_image_rect_with_sampling_options(
                        &image,
                        None,
                        &rect,
                        skia_safe::SamplingOptions::default(),
                        &paint,
                        skia_safe::canvas::SrcRectConstraint::Fast,
                    );
                }
            }

            let mut out_width = image.width() as f32;
            let mut out_height = image.height() as f32;
            if resize_width <= 0. && resize_height > 0. {
                out_width = image.width() as f32 + resize_height / image.height() as f32;
            }

            if resize_height <= 0. || resize_width > 0. {
                out_height = image.height() as f32 + resize_width / image.width() as f32;
            }

            if resize_width > 0. && resize_height > 0. {
                out_width = resize_width;
                out_height = resize_height;
            }

            if image.width() as f32 > resize_width || image.height() as f32 > resize_height {
                let mut resized_surface_buffer =
                    vec![0_u8; (resize_width * resize_height * 4.) as usize];
                let new_image_info = image_info.with_dimensions((out_width.round() as i32, out_height.round() as i32));
                let mut resized_surface = skia_safe::Surface::new_raster_direct(
                    &new_image_info,
                    resized_surface_buffer.as_mut_slice(),
                    Some((out_width.round() * 4.) as usize),
                    None,
                );
                match resized_surface {
                    None => {}
                    Some(mut resized_surface) => {
                        surface.draw(
                            resized_surface.canvas(),
                            (0, 0),
                            ImageBitmapResizeQuality::from(resize_quality).to_quality(),
                            None,
                        );
                        output.load_from_bytes(resized_surface_buffer.as_slice());
                    }
                }
            } else {
                output.load_from_bytes(original_surface_buffer.as_slice());
            }
        }
    }
    Box::into_raw(Box::new(output)) as i64
}

pub fn create_image_asset(
    buf: &[u8],
    image_width: f32,
    image_height: f32,
    rect: Option<skia_safe::Rect>,
    flip_y: bool,
    premultiply_alpha: i32,
    color_space_conversion: i32,
    resize_quality: i32,
    resize_width: f32,
    resize_height: f32,
) -> i64 {
    match from_image_slice(buf, image_width as i32, image_height as i32) {
        None => Box::into_raw(Box::new(ImageAsset::new())) as i64,
        Some(image) => match rect {
            None => create_image_bitmap(
                image,
                None,
                flip_y,
                premultiply_alpha,
                color_space_conversion,
                resize_quality,
                resize_width,
                resize_height,
            ),
            Some(rect) => create_image_bitmap(
                image,
                Some(rect),
                flip_y,
                premultiply_alpha,
                color_space_conversion,
                resize_quality,
                resize_width,
                resize_height,
            ),
        },
    }
}

fn create_from_image_asset_src_rect(
    image_asset: i64,
    rect: Option<skia_safe::Rect>,
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
        let bytes = asset.rgba_internal_bytes();
        let width = asset.width() as f32;
        let height = asset.height() as f32;
        create_image_asset(
            bytes.as_slice(),
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
}
