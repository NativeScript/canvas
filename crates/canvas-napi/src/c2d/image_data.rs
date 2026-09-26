//! `ImageData`, mirroring `canvas2d/ImageDataImpl.cpp`.

use napi::sys;

use crate::util::native::{Native, NativeType};

pub struct ImageData {
    pub(crate) data: *mut canvas_c::ImageData,
}

impl Native for ImageData {
    const KIND: NativeType = NativeType::ImageData;
}

impl Drop for ImageData {
    fn drop(&mut self) {
        canvas_c::canvas_native_image_data_release(self.data);
    }
}

pub unsafe fn init(_env: sys::napi_env, _exports: sys::napi_value) {}
