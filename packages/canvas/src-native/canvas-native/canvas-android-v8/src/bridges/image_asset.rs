use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use cxx::{CxxString, SharedPtr};
use parking_lot::{Mutex, MutexGuard, RawMutex};

use crate::bridges::utils::console_log;

pub struct ImageAsset(Arc<Mutex<canvas_core::context::image_asset::ImageAsset>>);

impl Default for ImageAsset {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Default::default())))
    }
}

impl Clone for ImageAsset {
    fn clone(&self) -> Self {
        Self {
            0: Arc::clone(&self.0),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = Arc::clone(&source.0)
    }
}

impl ImageAsset {
    pub fn lock(&self) -> MutexGuard<canvas_core::context::image_asset::ImageAsset> {
        self.0.lock()
    }

    pub fn width(&self) -> c_uint {
        self.0.lock().width()
    }

    pub fn height(&self) -> c_uint {
        self.0.lock().height()
    }
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("canvas-android-v8/src/OnImageAssetLoadCallbackHolder.h");
        pub(crate) type OnImageAssetLoadCallbackHolder;
        fn complete(&self, complete: bool);
    }

    extern "Rust" {
        type ImageAsset;
        fn canvas_native_image_asset_create() -> Box<ImageAsset>;

        fn canvas_native_image_asset_load_from_path(asset: &mut ImageAsset, path: String) -> bool;

        fn canvas_native_image_asset_load_from_path_async(
            asset: &mut ImageAsset,
            path: String,
            callback: SharedPtr<OnImageAssetLoadCallbackHolder>,
        );

        fn canvas_native_image_asset_load_from_url(asset: &mut ImageAsset, url: String) -> bool;

        fn canvas_native_image_asset_load_from_url_async(
            asset: &mut ImageAsset,
            url: String,
            callback: SharedPtr<OnImageAssetLoadCallbackHolder>,
        );

        fn canvas_native_image_asset_load_from_raw(asset: &mut ImageAsset, array: &[u8]) -> bool;

        fn canvas_native_image_asset_load_from_raw_async(
            asset: &mut ImageAsset,
            array: &[u8],
            callback: SharedPtr<OnImageAssetLoadCallbackHolder>,
        );

        fn canvas_native_image_asset_get_bytes(asset: &mut ImageAsset) -> Vec<u8>;

        fn canvas_native_image_asset_get_rgba_bytes(asset: &mut ImageAsset) -> Vec<u8>;

        fn canvas_native_image_asset_get_rgb_bytes(asset: &mut ImageAsset) -> Vec<u8>;

        fn canvas_native_image_asset_width(asset: &mut ImageAsset) -> u32;

        fn canvas_native_image_asset_height(asset: &mut ImageAsset) -> u32;

        fn canvas_native_image_asset_get_error(asset: &mut ImageAsset) -> String;

        fn canvas_native_image_asset_has_error(asset: &mut ImageAsset) -> bool;

        fn canvas_native_image_asset_scale(asset: &mut ImageAsset, x: u32, y: u32) -> bool;

        fn canvas_native_image_asset_flip_x(asset: &mut ImageAsset) -> bool;

        fn canvas_native_image_asset_flip_x_in_place(asset: &mut ImageAsset) -> bool;

        fn canvas_native_image_asset_flip_y(asset: &mut ImageAsset) -> bool;

        fn canvas_native_image_asset_flip_y_in_place_owned(buf: &mut [u8]);

        fn canvas_native_image_asset_flip_x_in_place_owned(buf: &mut [u8]);

        fn canvas_native_image_asset_flip_y_in_place(asset: &mut ImageAsset) -> bool;

        fn canvas_native_image_asset_save_path(
            asset: &mut ImageAsset,
            path: &str,
            format: u32,
        ) -> bool;

        fn canvas_native_image_asset_save_path_async(
            asset: &mut ImageAsset,
            path: &str,
            format: u32,
            callback: SharedPtr<OnImageAssetLoadCallbackHolder>,
        );
    }
}

unsafe impl Sync for ffi::OnImageAssetLoadCallbackHolder {}

unsafe impl Send for ffi::OnImageAssetLoadCallbackHolder {}

pub fn canvas_native_image_asset_create() -> Box<ImageAsset> {
    Box::new(ImageAsset::default())
}

pub fn canvas_native_image_asset_load_from_path(asset: &mut ImageAsset, path: String) -> bool {
    asset.lock().load_from_path(&path)
}

pub fn canvas_native_image_asset_load_from_path_async(
    asset: &mut ImageAsset,
    path: String,
    callback: SharedPtr<ffi::OnImageAssetLoadCallbackHolder>,
) {
    let mut asset = asset.clone();
    std::thread::spawn(move || {
        let done = asset.lock().load_from_path(path.as_ref());
        callback.complete(done);
    });
}

pub fn canvas_native_image_asset_load_from_raw(asset: &mut ImageAsset, array: &[u8]) -> bool {
    asset.lock().load_from_bytes(array)
}

pub fn canvas_native_image_asset_load_from_raw_async(
    asset: &mut ImageAsset,
    array: &[u8],
    callback: SharedPtr<ffi::OnImageAssetLoadCallbackHolder>,
) {
    let mut asset = asset.clone();
    let array = array.to_vec();
    std::thread::spawn(move || {
        let done = asset.lock().load_from_bytes(array.as_slice());
        callback.complete(done);
    });
}

pub fn canvas_native_image_asset_load_from_url_str(asset: &mut ImageAsset, url: &str) -> bool {
    use std::fs::File;
    use std::io::prelude::*;
    if let Ok(res) = ureq::get(url).call() {
        let mut reader = res.into_reader();
        let mut data = Vec::new();
        return match reader.read_to_end(&mut data) {
            Ok(_) => asset.lock().load_from_bytes(data.as_slice()),
            Err(_) => false,
        };
    }
    false
}

pub fn canvas_native_image_asset_load_from_url(asset: &mut ImageAsset, url: String) -> bool {
    canvas_native_image_asset_load_from_url_str(asset, url.as_str())
}

pub fn canvas_native_image_asset_load_from_url_async(
    asset: &mut ImageAsset,
    url: String,
    callback: SharedPtr<ffi::OnImageAssetLoadCallbackHolder>,
) {
    let mut asset = asset.clone();
    let callback = callback.clone();
    std::thread::spawn(move || {
        let done = canvas_native_image_asset_load_from_url_str(&mut asset, url.as_str());
        callback.complete(done);
    });
}

pub fn canvas_native_image_asset_get_bytes(asset: &mut ImageAsset) -> Vec<u8> {
    asset.lock().bytes_internal()
}

pub fn canvas_native_image_asset_get_rgba_bytes(asset: &mut ImageAsset) -> Vec<u8> {
    asset.lock().rgba_internal_bytes()
}

pub fn canvas_native_image_asset_get_rgb_bytes(asset: &mut ImageAsset) -> Vec<u8> {
    asset.lock().rgb_internal_bytes()
}

pub fn canvas_native_image_asset_width(asset: &mut ImageAsset) -> u32 {
    asset.lock().width()
}

pub fn canvas_native_image_asset_height(asset: &mut ImageAsset) -> u32 {
    asset.lock().height()
}

pub fn canvas_native_image_asset_get_error(asset: &mut ImageAsset) -> String {
    asset.lock().error().to_string()
}

pub fn canvas_native_image_asset_has_error(asset: &mut ImageAsset) -> bool {
    if asset.lock().error().is_empty() {
        return false;
    }
    true
}

pub fn canvas_native_image_asset_scale(asset: &mut ImageAsset, x: u32, y: u32) -> bool {
    asset.lock().scale(x, y)
}

pub fn canvas_native_image_asset_flip_x(asset: &mut ImageAsset) -> bool {
    asset.lock().flip_x()
}

pub fn canvas_native_image_asset_flip_x_in_place(asset: &mut ImageAsset) -> bool {
    asset.lock().flip_x_in_place()
}

pub fn canvas_native_image_asset_flip_y(asset: &mut ImageAsset) -> bool {
    asset.lock().flip_y()
}

pub fn canvas_native_image_asset_flip_y_in_place_owned(buf: &mut [u8]) {
    if let Ok(mut image) = image::load_from_memory(buf) {
        image::imageops::flip_vertical_in_place(&mut image);
    }
}

pub fn canvas_native_image_asset_flip_x_in_place_owned(buf: &mut [u8]) {
    if let Ok(mut image) = image::load_from_memory(buf) {
        image::imageops::flip_horizontal_in_place(&mut image);
    }
}

pub fn canvas_native_image_asset_flip_y_in_place(asset: &mut ImageAsset) -> bool {
    asset.lock().flip_y_in_place()
}

pub fn canvas_native_image_asset_save_path(
    asset: &mut ImageAsset,
    path: &str,
    format: u32,
) -> bool {
    asset.lock().save_path(
        path,
        canvas_core::context::image_asset::OutputFormat::from(format),
    )
}

pub fn canvas_native_image_asset_save_path_async(
    asset: &mut ImageAsset,
    path: &str,
    format: u32,
    callback: SharedPtr<ffi::OnImageAssetLoadCallbackHolder>,
) {
    let mut asset = asset.clone();
    let path = path.to_string();
    std::thread::spawn(move || {
        let done = asset.lock().save_path(
            path.as_ref(),
            canvas_core::context::image_asset::OutputFormat::from(format),
        );

        callback.complete(done);
    });
}
