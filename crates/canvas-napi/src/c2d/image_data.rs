use std::cell::Cell;
use std::ptr;

use canvas_c::ImageData as CImageData;
use napi::bindgen_prelude::{
  Either, Object, ObjectFinalize, Uint8ClampedArray, Uint8ClampedSlice, Unknown,
};
use napi::*;
use napi_derive::napi;

#[napi(custom_finalize)]
pub struct ImageData {
  pub(crate) data: CImageData,
  /// The `data` view, created once so `imageData.data === imageData.data`.
  data_ref: Cell<sys::napi_ref>,
}

impl ObjectFinalize for ImageData {
  fn finalize(self, env: Env) -> Result<()> {
    let data_ref = self.data_ref.get();
    if !data_ref.is_null() {
      unsafe { sys::napi_delete_reference(env.raw(), data_ref) };
    }
    Ok(())
  }
}

impl ImageData {
  /// Takes ownership of an `ImageData` canvas-c returned (a boxed value).
  pub(crate) fn from_raw(data: *mut CImageData) -> Option<Self> {
    if data.is_null() {
      return None;
    }
    Some(Self::from_data(*unsafe { Box::from_raw(data) }))
  }

  pub(crate) fn from_data(data: CImageData) -> Self {
    Self {
      data,
      data_ref: Cell::new(ptr::null_mut()),
    }
  }

  pub(crate) fn as_ptr(&self) -> *const CImageData {
    &self.data
  }
}

#[napi]
impl ImageData {
  #[napi(constructor)]
  pub fn new(
    width_or_image_data: Either<f64, Uint8ClampedArray>,
    height: Option<f64>,
    settings_or_height: Option<Either<Object, f64>>,
  ) -> Result<ImageData> {
    match width_or_image_data {
      Either::A(width) => {
        if let Some(height) = height {
          return ImageData::from_raw(canvas_c::canvas_native_context_create_image_data(
            width as i32,
            height as i32,
          ))
          .ok_or_else(|| Error::from_reason("Failed to construct 'ImageData'"));
        };
        Err(Error::from_reason(
          "constructor: 1 is not a valid argument count for any overload.",
        ))
      }
      Either::B(value) => {
        let length = value.len();

        if let Some(width) = height {
          let row = (width * 4.) as usize;
          if row == 0 || (length % row) != 0 {
            return Err(Error::from_reason(format!(
              "Failed to construct 'ImageData': The input data length is not a multiple of (4 * {})",
              width
            )));
          }
          let mut new_height = (length / row) as i32;

          if let Some(height) = settings_or_height {
            match height {
              Either::A(_) => {
                // todo handle settings
              }
              Either::B(height) => {
                new_height = height as i32;
              }
            }
          }
          return ImageData::from_raw(canvas_c::canvas_native_context_create_image_data_with_data(
            width as i32,
            new_height,
            value.as_ptr(),
            value.len(),
          ))
          .ok_or_else(|| Error::from_reason("Failed to construct 'ImageData'"));
        }
        Err(Error::from_reason(
          "Failed to construct 'ImageData': 2 arguments required, but only 1 present.",
        ))
      }
    }
  }

  #[napi(getter)]
  pub fn width(&self) -> f64 {
    self.data.inner().width() as f64
  }

  pub(crate) fn width_inner(&self) -> i32 {
    self.data.inner().width()
  }

  #[napi(getter)]
  pub fn height(&self) -> f64 {
    self.data.inner().height() as f64
  }

  pub(crate) fn height_inner(&self) -> i32 {
    self.data.inner().height()
  }

  /// A `Uint8ClampedArray` over the pixels themselves (no copy); it holds its own reference to
  /// the buffer, so it stays valid if the `ImageData` is collected first.
  #[napi(getter, ts_return_type = "Uint8ClampedArray")]
  pub fn data<'env>(&self, env: &'env Env) -> Result<Unknown<'env>> {
    let cached = self.data_ref.get();
    if !cached.is_null() {
      let mut value = ptr::null_mut();
      check_status!(unsafe { sys::napi_get_reference_value(env.raw(), cached, &mut value) })?;
      if !value.is_null() {
        return Ok(unsafe { Unknown::from_raw_unchecked(env.raw(), value) });
      }
    }

    let bytes = self.data.inner().data();
    let (data, len) = (bytes.as_ptr() as *mut u8, bytes.len());
    let view = unsafe {
      Uint8ClampedSlice::from_external(env, data, len, self.data.clone(), |_, keep| drop(keep))
    }?;
    let value = unsafe { napi::bindgen_prelude::ToNapiValue::to_napi_value(env.raw(), view)? };
    let mut reference = ptr::null_mut();
    check_status!(unsafe { sys::napi_create_reference(env.raw(), value, 1, &mut reference) })?;
    self.data_ref.set(reference);
    Ok(unsafe { Unknown::from_raw_unchecked(env.raw(), value) })
  }

  pub(crate) fn data_inner(&self) -> &[u8] {
    self.data.inner().data()
  }
}
