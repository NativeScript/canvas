use std::cell::Cell;
use std::sync::Arc;

use canvas_c::{
  ImageAsset as CImageAsset, ImageBitmapColorSpaceConversion, ImageBitmapPremultiplyAlpha,
  ImageBitmapResizeQuality,
};
use napi::bindgen_prelude::Unknown;
use napi::Env;
use napi_derive::napi;

use crate::c2d::CanvasRenderingContext2D;
use crate::image_asset::ImageAsset;
use crate::module::{as_bool, as_number, as_string, downcast, property, PinnedBytes};

/// `createImageBitmap` options, read as the V8 bindings' `ImageBitmapImpl::HandleOptions` reads
/// them: unknown values keep the default instead of throwing.
#[derive(Debug, Clone, Copy)]
pub(crate) struct BitmapOptions {
  flip_y: bool,
  premultiply_alpha: ImageBitmapPremultiplyAlpha,
  color_space_conversion: ImageBitmapColorSpaceConversion,
  resize_quality: ImageBitmapResizeQuality,
  resize_width: f32,
  resize_height: f32,
}

impl Default for BitmapOptions {
  fn default() -> Self {
    Self {
      flip_y: false,
      premultiply_alpha: ImageBitmapPremultiplyAlpha::Default,
      color_space_conversion: ImageBitmapColorSpaceConversion::Default,
      resize_quality: ImageBitmapResizeQuality::Low,
      resize_width: 0.,
      resize_height: 0.,
    }
  }
}

/// `{ imageOrientation: 'flipY' | flipY: boolean, premultiplyAlpha, colorSpaceConversion,
/// resizeQuality, resizeWidth, resizeHeight }`; anything that is not an object gives the defaults.
pub(crate) fn parse_options(options: Option<&Unknown>) -> BitmapOptions {
  let mut ret = BitmapOptions::default();
  let Some(options) = options else {
    return ret;
  };
  let string = |name| property(options, name).as_ref().and_then(as_string);
  let number = |name| property(options, name).as_ref().and_then(as_number);

  if string(c"imageOrientation").as_deref() == Some("flipY") {
    ret.flip_y = true;
  }
  if let Some(flip_y) = property(options, c"flipY").as_ref().and_then(as_bool) {
    ret.flip_y = flip_y;
  }
  match string(c"premultiplyAlpha").as_deref() {
    Some("premultiply") => ret.premultiply_alpha = ImageBitmapPremultiplyAlpha::Premultiply,
    Some("none") => ret.premultiply_alpha = ImageBitmapPremultiplyAlpha::AlphaNone,
    _ => {}
  }
  if string(c"colorSpaceConversion").as_deref() == Some("none") {
    ret.color_space_conversion = ImageBitmapColorSpaceConversion::None;
  }
  match string(c"resizeQuality").as_deref() {
    Some("medium") => ret.resize_quality = ImageBitmapResizeQuality::Medium,
    Some("high") => ret.resize_quality = ImageBitmapResizeQuality::High,
    Some("pixelated") => ret.resize_quality = ImageBitmapResizeQuality::Pixelated,
    _ => {}
  }
  if let Some(width) = number(c"resizeWidth") {
    ret.resize_width = width as f32;
  }
  if let Some(height) = number(c"resizeHeight") {
    ret.resize_height = height as f32;
  }
  ret
}

/// A crop rect: `(sx, sy, sw, sh)`.
pub(crate) type Rect = (f32, f32, f32, f32);

/// What a worker decodes an ImageBitmap from.
pub(crate) enum BitmapSource {
  /// An ImageAsset's or ImageBitmap's image.
  Asset(Arc<CImageAsset>),
  /// An ImageData's pixels (shared, not copied).
  ImageData(canvas_c::ImageData),
  /// Encoded image bytes, read in place.
  Encoded(PinnedBytes),
}

impl BitmapSource {
  /// On the JS thread, once decoding is done.
  pub(crate) fn release(self, env: &Env) {
    if let BitmapSource::Encoded(bytes) = self {
      bytes.release(env);
    }
  }
}

/// Decodes `source` into a new image (the canvas-c `*_with_output` entry points); runs on a
/// worker thread.
pub(crate) fn decode(
  source: &BitmapSource,
  rect: Option<Rect>,
  o: BitmapOptions,
) -> Option<Arc<CImageAsset>> {
  let (flip, alpha, color, quality, rw, rh) = (
    o.flip_y,
    o.premultiply_alpha,
    o.color_space_conversion,
    o.resize_quality,
    o.resize_width,
    o.resize_height,
  );
  let output = canvas_c::canvas_native_image_asset_create();
  let done = match (source, rect) {
    (BitmapSource::Asset(asset), None) => {
      canvas_c::canvas_native_image_bitmap_create_from_asset_with_output(
        Arc::as_ptr(asset),
        flip,
        alpha,
        color,
        quality,
        rw,
        rh,
        output,
      )
    }
    (BitmapSource::Asset(asset), Some((sx, sy, sw, sh))) => {
      canvas_c::canvas_native_image_bitmap_create_from_asset_src_rect_with_output(
        Arc::as_ptr(asset),
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
        output,
      )
    }
    (BitmapSource::ImageData(data), None) => {
      canvas_c::canvas_native_image_bitmap_create_from_image_data_with_output(
        data, flip, alpha, color, quality, rw, rh, output,
      )
    }
    (BitmapSource::ImageData(data), Some((sx, sy, sw, sh))) => {
      canvas_c::canvas_native_image_bitmap_create_from_image_data_src_rect_with_output(
        data, sx, sy, sw, sh, flip, alpha, color, quality, rw, rh, output,
      )
    }
    (BitmapSource::Encoded(bytes), rect) => {
      let bytes = bytes.as_slice();
      if bytes.is_empty() {
        false
      } else if let Some((sx, sy, sw, sh)) = rect {
        canvas_c::canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(
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
          output,
        )
      } else {
        canvas_c::canvas_native_image_bitmap_create_from_encoded_bytes_with_output(
          bytes.as_ptr(),
          bytes.len(),
          flip,
          alpha,
          color,
          quality,
          rw,
          rh,
          output,
        )
      }
    }
  };
  let output = unsafe { Arc::from_raw(output) };
  done.then_some(output)
}

/// A snapshot of a 2D context (on the JS thread; the caller flushes pending drawing first).
pub(crate) fn from_context(
  context: &CanvasRenderingContext2D,
  rect: Option<Rect>,
  o: BitmapOptions,
) -> Option<Arc<CImageAsset>> {
  let output = canvas_c::canvas_native_image_asset_create();
  let (sx, sy, sw, sh) = rect.unwrap_or_default();
  let done = canvas_c::canvas_native_image_bitmap_create_from_context_with_output(
    context.context,
    sx,
    sy,
    sw,
    sh,
    rect.is_some(),
    o.flip_y,
    o.premultiply_alpha,
    o.color_space_conversion,
    o.resize_quality,
    o.resize_width,
    o.resize_height,
    output,
  );
  let output = unsafe { Arc::from_raw(output) };
  done.then_some(output)
}

/// Made by `CanvasModule.createImageBitmap` or `ImageBitmap.fromAsset`; not constructible from JS.
#[napi]
pub struct ImageBitmap {
  pub(crate) asset: Arc<CImageAsset>,
  closed: Cell<bool>,
}

impl ImageBitmap {
  pub(crate) fn new(asset: Arc<CImageAsset>) -> Self {
    Self {
      asset,
      closed: Cell::new(false),
    }
  }
}

#[napi]
impl ImageBitmap {
  /// `ImageBitmap.fromAsset(asset)`: a bitmap sharing the asset's image (Android loads encoded
  /// bytes into an asset on the Java side, then wraps it); `null` for anything else.
  #[napi(ts_args_type = "asset: ImageAsset")]
  pub fn from_asset(asset: Unknown) -> Option<ImageBitmap> {
    downcast::<ImageAsset>(&asset).map(|asset| ImageBitmap::new(Arc::clone(&asset.asset)))
  }

  /// 0 once closed.
  #[napi(getter)]
  pub fn get_width(&self) -> u32 {
    if self.closed.get() {
      0
    } else {
      self.asset.width()
    }
  }

  /// 0 once closed.
  #[napi(getter)]
  pub fn get_height(&self) -> u32 {
    if self.closed.get() {
      0
    } else {
      self.asset.height()
    }
  }

  #[napi]
  pub fn close(&self) {
    self.asset.close();
    self.closed.set(true);
  }

  /// The canvas-c image pointer, as a decimal string.
  #[napi(getter, js_name = "__addr")]
  pub fn addr(&self) -> String {
    (Arc::as_ptr(&self.asset) as usize).to_string()
  }

  /// Same as `__addr` (the host APIs take the pointer back without taking ownership).
  #[napi(js_name = "__getRef")]
  pub fn get_ref(&self) -> String {
    self.addr()
  }
}
