use crate::c2d::image_data::ImageData;
use crate::image_asset::ImageAsset;
use napi::bindgen_prelude::{AsyncTask, ClassInstance, Either4, ObjectFinalize};
use napi::*;
use napi_derive::napi;
use std::sync::Arc;

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "ImageBitmapOptionsImageOrientation", string_enum)]
#[derive(Debug)]
pub enum ImageBitmapOptionsImageOrientation {
  #[napi(value = "from-image")]
  fromImage,
  #[napi(value = "flipY")]
  flipY,
  none,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "ImageBitmapOptionsPremultiplyAlpha", string_enum)]
#[derive(Debug)]
pub enum ImageBitmapOptionsPremultiplyAlpha {
  premultiply,
  none,
  default,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "ImageBitmapOptionsColorSpaceConversion", string_enum)]
#[derive(Debug)]
pub enum ImageBitmapOptionsColorSpaceConversion {
  default,
  none,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "ImageBitmapOptionResizeQuality", string_enum)]
#[derive(Debug)]
pub enum ImageBitmapOptionResizeQuality {
  low,
  medium,
  high,
  pixelated,
}

#[napi(object)]
#[derive(Debug, Default, Clone)]
pub struct ImageBitmapOptions {
  pub image_orientation: Option<ImageBitmapOptionsImageOrientation>,
  pub premultiply_alpha: Option<ImageBitmapOptionsPremultiplyAlpha>,
  pub color_space_conversion: Option<ImageBitmapOptionsColorSpaceConversion>,
  pub resize_width: Option<f64>,
  pub resize_height: Option<f64>,
  pub resize_quality: Option<ImageBitmapOptionResizeQuality>,
}

struct CanvasImageBitmapOptions {
  pub image_orientation: bool,
  pub premultiply_alpha: canvas_c::ImageBitmapPremultiplyAlpha,
  pub color_space_conversion: canvas_c::ImageBitmapColorSpaceConversion,
  pub resize_width: f64,
  pub resize_height: f64,
  pub resize_quality: canvas_c::ImageBitmapResizeQuality,
}

impl From<ImageBitmapOptions> for CanvasImageBitmapOptions {
  fn from(value: ImageBitmapOptions) -> Self {
    Self {
      image_orientation: match value
        .image_orientation
        .unwrap_or(ImageBitmapOptionsImageOrientation::none)
      {
        ImageBitmapOptionsImageOrientation::fromImage => false,
        ImageBitmapOptionsImageOrientation::flipY => true,
        ImageBitmapOptionsImageOrientation::none => false,
      },
      premultiply_alpha: match value
        .premultiply_alpha
        .unwrap_or(ImageBitmapOptionsPremultiplyAlpha::none)
      {
        ImageBitmapOptionsPremultiplyAlpha::premultiply => {
          canvas_c::ImageBitmapPremultiplyAlpha::Premultiply
        }
        ImageBitmapOptionsPremultiplyAlpha::none => {
          canvas_c::ImageBitmapPremultiplyAlpha::AlphaNone
        }
        ImageBitmapOptionsPremultiplyAlpha::default => {
          canvas_c::ImageBitmapPremultiplyAlpha::Default
        }
      },
      color_space_conversion: match value
        .color_space_conversion
        .unwrap_or(ImageBitmapOptionsColorSpaceConversion::default)
      {
        ImageBitmapOptionsColorSpaceConversion::default => {
          canvas_c::ImageBitmapColorSpaceConversion::Default
        }
        ImageBitmapOptionsColorSpaceConversion::none => {
          canvas_c::ImageBitmapColorSpaceConversion::None
        }
      },
      resize_width: value.resize_width.unwrap_or(0.),
      resize_height: value.resize_height.unwrap_or(0.),
      resize_quality: match value
        .resize_quality
        .unwrap_or(ImageBitmapOptionResizeQuality::low)
      {
        ImageBitmapOptionResizeQuality::low => canvas_c::ImageBitmapResizeQuality::Low,
        ImageBitmapOptionResizeQuality::medium => canvas_c::ImageBitmapResizeQuality::Medium,
        ImageBitmapOptionResizeQuality::high => canvas_c::ImageBitmapResizeQuality::High,
        ImageBitmapOptionResizeQuality::pixelated => canvas_c::ImageBitmapResizeQuality::Pixelated,
      },
    }
  }
}

#[napi(ts_return_type = "Promise<ImageBitmap>")]
pub fn create_image_bitmap(
  source: Either4<
    ClassInstance<ImageAsset>,
    ClassInstance<ImageBitmap>,
    ClassInstance<ImageData>,
    &[u8],
  >,
  sx_or_options: Option<Either<ImageBitmapOptions, f64>>,
  sy: Option<f64>,
  sw: Option<f64>,
  sh: Option<f64>,
  options: Option<ImageBitmapOptions>,
) -> AsyncTask<AsyncImageBitmap> {
  let mut opts = None;
  let mut source_rect = None;
  match (sx_or_options, options) {
    (Some(sx_or_options), _) => match sx_or_options {
      Either::A(opt) => {
        opts = Some(opt);
      }
      Either::B(sx) => match (sy, sw, sh) {
        (Some(sy), Some(sw), Some(sh)) => {
          source_rect = Some((sx, sy, sw, sh));
        }
        _ => {}
      },
    },
    (_, Some(options)) => {
      opts = Some(options);
    }
    _ => {}
  }

  match source {
    Either4::A(asset) => AsyncTask::new(AsyncImageBitmap::new(
      Some(Arc::clone(&asset.asset)),
      None,
      None,
      opts,
      source_rect,
    )),
    Either4::B(bitmap) => AsyncTask::new(AsyncImageBitmap::new(
      Some(Arc::clone(&bitmap.asset)),
      None,
      None,
      opts,
      source_rect,
    )),
    Either4::C(data) => AsyncTask::new(AsyncImageBitmap::new(
      None,
      Some(Arc::clone(&data.data)),
      None,
      opts,
      source_rect,
    )),
    Either4::D(data) => AsyncTask::new(AsyncImageBitmap::new(
      None,
      None,
      Some(data.to_vec()),
      opts,
      source_rect,
    )),
  }
}

pub struct AsyncImageBitmap {
  image_asset: Option<Arc<canvas_c::ImageAsset>>,
  image_data: Option<Arc<canvas_c::ImageData>>,
  data: Option<Vec<u8>>,
  options: Option<ImageBitmapOptions>,
  source_rect: Option<(f64, f64, f64, f64)>,
}

impl AsyncImageBitmap {
  pub fn new(
    image_asset: Option<Arc<canvas_c::ImageAsset>>,
    image_data: Option<Arc<canvas_c::ImageData>>,
    data: Option<Vec<u8>>,
    options: Option<ImageBitmapOptions>,
    source_rect: Option<(f64, f64, f64, f64)>,
  ) -> Self {
    Self {
      image_asset,
      image_data,
      data,
      options,
      source_rect,
    }
  }
}

impl Task for AsyncImageBitmap {
  type Output = ImageBitmap;
  type JsValue = ImageBitmap;

  fn compute(&mut self) -> Result<Self::Output> {
    let options: CanvasImageBitmapOptions = self.options.take().unwrap_or_default().into();

    if let Some(asset) = &self.image_asset {
      unsafe {
        return Ok(ImageBitmap {
          asset: Arc::from_raw(canvas_c::canvas_native_image_bitmap_create_from_asset(
            Arc::as_ptr(&asset),
            options.image_orientation,
            options.premultiply_alpha,
            options.color_space_conversion,
            options.resize_quality,
            options.resize_width as f32,
            options.resize_height as f32,
          )),
        });
      }
    } else if let Some(data) = &self.image_data {
      let data = data.inner().as_ref();
      unsafe {
        return Ok(ImageBitmap {
          asset: Arc::from_raw(
            canvas_c::canvas_native_image_bitmap_create_from_encoded_bytes(
              data.as_ptr(),
              data.len(),
              options.image_orientation,
              options.premultiply_alpha,
              options.color_space_conversion,
              options.resize_quality,
              options.resize_width as f32,
              options.resize_height as f32,
            ),
          ),
        });
      }
    } else if let Some(data) = &self.data {
      unsafe {
        return Ok(ImageBitmap {
          asset: Arc::from_raw(
            canvas_c::canvas_native_image_bitmap_create_from_encoded_bytes(
              data.as_ptr(),
              data.len(),
              options.image_orientation,
              options.premultiply_alpha,
              options.color_space_conversion,
              options.resize_quality,
              options.resize_width as f32,
              options.resize_height as f32,
            ),
          ),
        });
      }
    }
    Err(napi::Error::from_status(napi::Status::Unknown))
  }

  fn resolve(&mut self, env: Env, output: ImageBitmap) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub struct ImageBitmap {
  pub(crate) asset: Arc<canvas_c::ImageAsset>,
}

#[napi]
impl ImageBitmap {
  #[napi(getter)]
  pub fn get_width(&self) -> u32 {
    self.asset.width()
  }
  #[napi(getter)]
  pub fn get_height(&self) -> u32 {
    self.asset.height()
  }

  #[napi]
  pub fn close(&self) {
    self.asset.close()
  }
}
