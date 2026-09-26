//! `ImageAsset` and `ImageBitmap`, mirroring `ImageAssetImpl.cpp` / `ImageBitmapImpl.cpp`. Both
//! hold a reference-counted canvas-c `ImageAsset`.

use canvas_c::ImageAsset as Asset;
use napi::sys;

use crate::util::native::{Native, NativeType};

pub struct ImageAsset {
    pub(crate) asset: *const Asset,
}

impl Native for ImageAsset {
    const KIND: NativeType = NativeType::ImageAsset;
}

impl Drop for ImageAsset {
    fn drop(&mut self) {
        canvas_c::canvas_native_image_asset_release(self.asset);
    }
}

pub struct ImageBitmap {
    pub(crate) asset: *const Asset,
}

impl Native for ImageBitmap {
    const KIND: NativeType = NativeType::ImageBitmap;
}

impl Drop for ImageBitmap {
    fn drop(&mut self) {
        canvas_c::canvas_native_image_asset_release(self.asset);
    }
}

pub unsafe fn init(_env: sys::napi_env, _exports: sys::napi_value) {}
