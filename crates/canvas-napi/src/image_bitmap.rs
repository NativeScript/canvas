use crate::c2d::image_data::ImageData;
use crate::image_asset::ImageAsset;
use napi::bindgen_prelude::{AsyncTask, Either4};
use napi::*;
use napi_derive::napi;
use std::sync::Arc;

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "ImageBitmapOptionsImageOrientation", string_enum)]
#[derive(Debug, Clone, Copy)]
pub enum ImageBitmapOptionsImageOrientation {
  #[napi(value = "from-image")]
  fromImage,
  #[napi(value = "flipY")]
  flipY,
  none,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "ImageBitmapOptionsPremultiplyAlpha", string_enum)]
#[derive(Debug, Clone, Copy)]
pub enum ImageBitmapOptionsPremultiplyAlpha {
  premultiply,
  none,
  default,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "ImageBitmapOptionsColorSpaceConversion", string_enum)]
#[derive(Debug, Clone, Copy)]
pub enum ImageBitmapOptionsColorSpaceConversion {
  default,
  none,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "ImageBitmapOptionResizeQuality", string_enum)]
#[derive(Debug, Clone, Copy)]
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
  source: Either4<&ImageAsset, &ImageBitmap, &ImageData, &[u8]>,
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
      Some(data.data.clone()),
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
  image_data: Option<canvas_c::ImageData>,
  data: Option<Vec<u8>>,
  options: Option<ImageBitmapOptions>,
  source_rect: Option<(f64, f64, f64, f64)>,
}

impl AsyncImageBitmap {
  pub fn new(
    image_asset: Option<Arc<canvas_c::ImageAsset>>,
    image_data: Option<canvas_c::ImageData>,
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
    let o: CanvasImageBitmapOptions = self.options.take().unwrap_or_default().into();
    let rect = self
      .source_rect
      .map(|(x, y, w, h)| (x as f32, y as f32, w as f32, h as f32));
    let (flip, alpha, color, quality) = (
      o.image_orientation,
      o.premultiply_alpha,
      o.color_space_conversion,
      o.resize_quality,
    );
    let (rw, rh) = (o.resize_width as f32, o.resize_height as f32);

    let asset = if let Some(asset) = &self.image_asset {
      let asset = Arc::as_ptr(asset);
      match rect {
        Some((sx, sy, sw, sh)) => canvas_c::canvas_native_image_bitmap_create_from_asset_src_rect(
          asset, sx, sy, sw, sh, flip, alpha, color, quality, rw, rh,
        ),
        None => canvas_c::canvas_native_image_bitmap_create_from_asset(
          asset, flip, alpha, color, quality, rw, rh,
        ),
      }
    } else if let Some(data) = &self.image_data {
      // Pixels, not an encoded image: the ImageData entry points.
      let output = canvas_c::canvas_native_image_asset_create();
      let done = match rect {
        Some((sx, sy, sw, sh)) => {
          canvas_c::canvas_native_image_bitmap_create_from_image_data_src_rect_with_output(
            data, sx, sy, sw, sh, flip, alpha, color, quality, rw, rh, output,
          )
        }
        None => canvas_c::canvas_native_image_bitmap_create_from_image_data_with_output(
          data, flip, alpha, color, quality, rw, rh, output,
        ),
      };
      if !done {
        canvas_c::canvas_native_image_asset_release(output);
        return Err(napi::Error::from_reason(
          "Failed to execute 'createImageBitmap' : The provided source could not be read",
        ));
      }
      output
    } else if let Some(bytes) = &self.data {
      match rect {
        Some((sx, sy, sw, sh)) => {
          canvas_c::canvas_native_image_bitmap_create_from_encoded_bytes_src_rect(
            bytes.as_ptr(),
            bytes.len(),
            sx,
            sy,
            sw,
            sh,
            flip,
            alpha,
            color,
            quality,
            rw,
            rh,
          )
        }
        None => canvas_c::canvas_native_image_bitmap_create_from_encoded_bytes(
          bytes.as_ptr(),
          bytes.len(),
          flip,
          alpha,
          color,
          quality,
          rw,
          rh,
        ),
      }
    } else {
      std::ptr::null()
    };

    if asset.is_null() {
      return Err(napi::Error::from_reason(
        "Failed to execute 'createImageBitmap' : The provided source could not be decoded",
      ));
    }
    Ok(ImageBitmap {
      asset: unsafe { Arc::from_raw(asset) },
    })
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
