use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::{c_char, c_int, c_uint};
use std::sync::Arc;

use crate::buffers::U8Buffer;

#[derive(Clone)]
pub struct ImageAsset(pub(crate) canvas_core::image_asset::ImageAsset);

impl ImageAsset {
    pub fn new(asset: canvas_core::image_asset::ImageAsset) -> Self {
        Self(asset)
    }
}

impl Default for ImageAsset {
    fn default() -> Self {
        Self(canvas_core::image_asset::ImageAsset::new())
    }
}

impl ImageAsset {
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    pub fn dimensions(&self) -> (c_uint, c_uint) {
        self.0.dimensions()
    }

    pub fn width(&self) -> c_uint {
        self.0.width()
    }

    pub fn height(&self) -> c_uint {
        self.0.height()
    }

    // pub fn size(&self) -> usize {
    //     self.0.size()
    // }

    #[cfg(any(unix))]
    pub fn load_from_fd(&self, fd: c_int) -> bool {
        self.0.load_from_fd(fd)
    }

    pub fn load_from_raw_bytes(
        &mut self,
        width: u32,
        height: u32,
        depth: u32,
        data: Vec<u8>,
    ) -> bool {
        self.0
            .load_from_raw_bytes(width as usize, height as usize, depth as usize, data)
    }

    pub fn load_from_reader<R>(&self, reader: &mut R) -> bool
    where
        R: Read + std::io::Seek + std::io::BufRead,
    {
        self.0.load_from_reader(reader)
    }

    pub fn load_from_path(&self, path: &str) -> bool {
        self.0.load_from_path(path)
    }

    pub fn load_from_bytes(&self, buf: &[u8]) -> bool {
        self.0.load_from_bytes(buf)
    }

    pub fn get_bytes(&self) -> Option<&[u8]> {
        self.0.get_bytes()
    }

    pub fn copy_bytes(&self) -> Option<Vec<u8>> {
        self.0.copy_bytes()
    }

    pub fn get_channels(&self) -> i32 {
        //self.0.get_channels()
        // always rgba
        4
    }

    pub fn error(&self) -> Cow<'_, str> {
        self.0.error()
    }

    pub fn set_error(&self, error: &str) {
        self.0.set_error(error);
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_get_addr(asset: *mut ImageAsset) -> i64 {
    if asset.is_null() {
        return 0;
    }
    asset as i64
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_create() -> *const ImageAsset {
    Arc::into_raw(Arc::new(ImageAsset::default()))
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_reference(asset: *const ImageAsset) {
    assert!(!asset.is_null());
    unsafe { Arc::increment_strong_count(asset) }
}

// fn canvas_native_image_asset_get_size(asset: &ImageAsset) -> usize {
//     asset.size()
// }

#[cfg(any(unix))]
#[no_mangle]
pub extern "C" fn canvas_native_image_asset_load_from_fd(
    asset: *const ImageAsset,
    fd: c_int,
) -> bool {
    if asset.is_null() {
        return false;
    }
    let asset = unsafe { &*asset };
    asset.load_from_fd(fd)
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_load_from_path(
    asset: *mut ImageAsset,
    path: *const c_char,
) -> bool {
    if asset.is_null() {
        return false;
    }
    let asset = unsafe { &mut *asset };
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();
    asset.load_from_path(path.as_ref())
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_load_from_raw(
    asset: *mut ImageAsset,
    array: *const u8,
    size: usize,
) -> bool {
    let array = unsafe { std::slice::from_raw_parts(array, size) };
    let asset = unsafe { &mut *asset };
    asset.load_from_bytes(array)
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_load_from_url(
    asset: *mut ImageAsset,
    url: *const c_char,
) -> bool {
    if asset.is_null() || url.is_null() {
        return false;
    }
    let asset = unsafe { &mut *asset };
    let url = unsafe { CStr::from_ptr(url) };
    let url = url.to_string_lossy().to_string();
    canvas_native_image_asset_load_from_url_internal(&mut asset.0, url.as_str())
}

pub(crate) fn canvas_native_image_asset_load_from_url_internal(
    asset: &mut canvas_core::image_asset::ImageAsset,
    url: &str,
) -> bool {
    use std::io::prelude::*;

    let resp = ureq::AgentBuilder::new()
        .redirects(10)
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .get(url);

    if let Ok(res) = resp.call() {
        if res.status() != 200 {
            return false;
        }
        // assert!(!res.has("Content-Length"));
        let len: usize;

        if let Some(value) = res.header("Content-Length") {
            if let Ok(length) = value.parse::<usize>() {
                len = length;
            } else {
                return false;
            }
        } else {
            return false;
        }

        let mut bytes: Vec<u8> = Vec::with_capacity(len);
        if let Ok(_) = res.into_reader().read_to_end(&mut bytes) {
            //  assert_eq!(bytes.len(), len);
        } else {
            return false;
        }

        return asset.load_from_bytes(bytes.as_slice());
    }
    false
}

// #[no_mangle]
//pub extern "C" fn canvas_native_image_asset_get_bytes(asset: *mut ImageAsset) -> Option<*const U8Buffer> {
//     asset.get_bytes()
// }

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_addr(asset: *mut ImageAsset) -> i64 {
    asset as i64
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_width(asset: *mut ImageAsset) -> u32 {
    if asset.is_null() {
        return 0;
    }
    let asset = unsafe { &*asset };
    asset.width()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_height(asset: *mut ImageAsset) -> u32 {
    if asset.is_null() {
        return 0;
    }
    let asset = unsafe { &*asset };
    asset.height()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_get_error(asset: *mut ImageAsset) -> *const c_char {
    if asset.is_null() {
        return std::ptr::null_mut();
    }
    let asset = unsafe { &*asset };
    CString::new(asset.error().to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_has_error(asset: *mut ImageAsset) -> bool {
    if asset.is_null() {
        return false;
    }
    let asset = unsafe { &*asset };
    if asset.error().is_empty() {
        return false;
    }
    true
}

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_get_data(asset: *mut ImageAsset) -> *mut U8Buffer {
    if asset.is_null() {
        return std::ptr::null_mut();
    }
    let asset = unsafe { &*asset };
    Box::into_raw(Box::new(U8Buffer::from(asset.clone())))
}
