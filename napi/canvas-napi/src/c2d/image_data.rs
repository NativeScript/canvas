use std::sync::Arc;
use crate::c2d::path::Path2D;
use canvas_c::{canvas_native_context_create_image_data_with_data, ImageData as CImageData};
use napi::*;
use napi::bindgen_prelude::Uint8ClampedArray;
use napi_derive::napi;
#[napi]
pub struct ImageData {
    pub(crate) data: Arc<CImageData>,
}


#[napi]
impl ImageData {
    #[napi(constructor)]
    pub fn new(width_or_image_data: Either<f64, Uint8ClampedArray>, height: Option<f64>, settings_or_height: Option<Either<JsObject, f64>>) -> Result<ImageData> {
        match width_or_image_data {
            Either::A(width) => {
                if let Some(height) = height {
                    return Ok(
                        Self {
                            data: unsafe {
                                Arc::from_raw(canvas_c::canvas_native_context_create_image_data(width as i32, height as i32))
                            }
                        }
                    );
                };
                Err(Error::from_reason("constructor: 1 is not a valid argument count for any overload."))
            }
            Either::B(value) => {
                let length = value.len();

                if let Some(width) = height {
                    let row = (width * 4.) as usize;
                    if (length % row) != 0 {
                        return Err(Error::from_reason(format!("Failed to construct 'ImageData': The input data length is not a multiple of (4 * {}", width)));
                    }
                    let mut new_height = row as i32;

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
                    return Ok(
                        Self {
                            data: unsafe {
                               Arc::from_raw(canvas_c::canvas_native_context_create_image_data_with_data(
                                    width as i32, new_height, value.as_ptr(), value.len(),
                                ))
                            }
                        }
                    );
                }
                Err(Error::from_reason("Failed to construct 'ImageData': 2 arguments required, but only 1 present."))
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

    #[napi(getter)]
    pub fn data(&self) -> &[u8] {
        self.data.inner().data()
    }

    pub(crate) fn data_inner(&self) -> &[u8] {
       self.data.inner().data()
    }
}