use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::io::{Read, Seek};
use std::os::raw::{c_char, c_uint};
use std::ptr::{null};
use std::sync::Arc;

use parking_lot::lock_api::MutexGuard;
use parking_lot::RawMutex;
use stb::image::{Channels, Data, Info};
use crate::common::ffi::u8_array::U8Array;

struct ImageAssetInner {
    image: Option<Data<u8>>,
    info: Option<Info>,
    error: String,
    did_resize: bool,
}

unsafe impl Send for ImageAssetInner {}

pub struct ImageAsset(Arc<parking_lot::Mutex<ImageAssetInner>>);

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

impl Default for ImageAsset {
    fn default() -> Self {
        ImageAsset::new()
    }
}

#[repr(C)]
pub enum PixelType {
    RGB = 0,
    RGBA = 1,
}

#[repr(C)]
pub enum OutputFormat {
    JPG = 0,
    PNG = 1,
    ICO = 2,
    BMP = 3,
    TIFF = 4,
}

impl From<u32> for OutputFormat {
    fn from(format: u32) -> Self {
        match format {
            1 => OutputFormat::PNG,
            3 => OutputFormat::BMP,
            4 => OutputFormat::TIFF,
            _ => OutputFormat::JPG,
        }
    }
}

impl From<i32> for OutputFormat {
    fn from(format: i32) -> Self {
        match format {
            1 => OutputFormat::PNG,
            2 => OutputFormat::ICO,
            3 => OutputFormat::BMP,
            4 => OutputFormat::TIFF,
            _ => OutputFormat::JPG,
        }
    }
}

enum ByteType {
    Default,
    RGBA,
    RGB,
}

impl ImageAsset {
    pub fn copy(asset: &ImageAsset) -> Option<ImageAsset> {
        let asset = asset.0.lock();
        if let Some(data) = &asset.image {
            return match stb::image::stbi_load_from_memory(data.as_slice(), Channels::RgbAlpha) {
                None => None,
                Some((info, data)) => {
                    let inner = ImageAssetInner {
                        image: Some(data),
                        info: Some(info),
                        error: String::new(),
                        did_resize: false,
                    };
                    return Some(Self(Arc::new(parking_lot::Mutex::new(inner))));
                }
            }
        }
        None
    }
    pub fn new() -> Self {
        Self(Arc::new(parking_lot::Mutex::new(ImageAssetInner {
            image: None,
            info: None,
            error: String::new(),
            did_resize: false,
        })))
    }

    pub fn get_channels(&self) -> i32 {
        if let Some(info) = self.get_info() {
            return info.components;
        }
        return 0;
    }

    fn get_info(&self) -> Option<Info> {
        self.get_lock().info
    }

    fn get_lock(&self) -> MutexGuard<'_, RawMutex, ImageAssetInner> {
        self.0.lock()
    }

    pub fn error(&self) -> Cow<str> {
        self.0.lock().error.clone().into()
    }

    pub fn set_error(&self, error: &str) {
        let mut lock = self.0.lock();
        lock.error.clear();
        lock.error.push_str(error);
    }

    pub fn error_cstr(&self) -> *const c_char {
        let lock = self.get_lock();
        if lock.error.is_empty() {
            return null();
        }
        CString::new(lock.error.as_str()).unwrap().into_raw()
    }

    pub fn width(&self) -> c_uint {
        self.get_lock()
            .info
            .as_ref()
            .map(|v| v.width.try_into().unwrap_or_default())
            .unwrap_or_default()
    }

    pub fn height(&self) -> c_uint {
        self.get_lock()
            .info
            .as_ref()
            .map(|v| v.height.try_into().unwrap_or_default())
            .unwrap_or_default()
    }

    pub fn size(&self) -> usize {
        self.get_lock()
            .info
            .as_ref()
            .map(|v| (v.height * v.width * v.components).try_into().unwrap_or_default())
            .unwrap_or_default()
    }

    pub fn load_from_path(&mut self, path: &str) -> bool {
        {
            let mut lock = self.get_lock();
            if !lock.error.is_empty() {
                lock.error.clear()
            }
            lock.image = None;
        }
        match std::fs::File::open(path) {
            Ok(mut file) => return self.load_from_reader(&mut file),
            Err(e) => {
                let error = e.to_string();
                self.get_lock().error.push_str(error.as_str());
                false
            }
        }
    }

    pub fn load_from_reader<R>(&mut self, reader: &mut R) -> bool
        where
            R: Read + Seek,
    {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        lock.image = None;

        match stb::image::stbi_load_from_reader(reader, Channels::RgbAlpha) {
            None => {
                lock.error.push_str("Failed to decode image");
                false
            }
            Some((info, data)) => {
                lock.info = Some(info);
                lock.image = Some(data);
                return true;
            }
        }
    }

    pub unsafe fn load_from_path_raw(&mut self, path: *const c_char) -> bool {
        let real_path = CStr::from_ptr(path);
        self.load_from_path(real_path.to_string_lossy().as_ref())
    }

    pub unsafe fn load_from_raw(&mut self, buffer: *const u8, size: usize) -> bool {
        self.load_from_bytes(std::slice::from_raw_parts(buffer, size))
    }

    pub fn load_from_bytes(&mut self, buf: &[u8]) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        lock.image = None;

        match stb::image::stbi_load_from_memory(buf, Channels::RgbAlpha) {
            None => {
                lock.error.push_str("Failed to decode image");
                false
            }
            Some((info, data)) => {
                lock.info = Some(info);
                lock.image = Some(data);
                return true;
            }
        }
    }

    pub fn load_from_bytes_int(&mut self, buf: &mut [i8]) -> bool {
        self.load_from_bytes(unsafe { std::mem::transmute(buf) })
    }

    pub fn scale(&mut self, x: c_uint, y: c_uint) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        match &mut lock.image {
            Some(image) => {
                let info = image.info();
                let done = image.resize(x as i32, y as i32, Channels::RgbAlpha, info);
                if !done {
                    lock.error.push_str("Failed to scale Image");
                }
                return done;
            }
            _ => {
                lock.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn get_bytes(&self) -> Option<&[u8]> {
        self.get_lock().image.as_ref().map(|d| {
            let slice = d.as_slice();
            unsafe { std::slice::from_raw_parts(slice.as_ptr(), slice.len()) }
        })
    }

    pub fn get_bytes_mut(&self) -> Option<&mut [u8]> {
        self.get_lock().image.as_mut().map(|d| {
            let slice = d.as_mut_slice();
            unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr(), slice.len()) }
        })
    }

    pub fn save_path_raw(&mut self, path: *const c_char, format: OutputFormat) -> bool {
        let real_path = unsafe { CStr::from_ptr(path) };
        self.save_path(real_path.to_string_lossy().as_ref(), format)
    }

    pub fn save_path(&mut self, path: &str, format: OutputFormat) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        match lock.image.as_ref() {
            Some(image) => {
                let width = self.width() as i32;
                let height = self.height() as i32;
                let comp = self.get_info().unwrap_or_default();
                let path = CString::new(path).unwrap_or_default();
                return match format {
                    OutputFormat::PNG => stb::image_write::stbi_write_png(
                        path.as_c_str(),
                        width,
                        height,
                        comp.components,
                        image.as_slice(),
                        width * comp.components,
                    )
                        .is_some(),
                    OutputFormat::ICO => false, // todo
                    OutputFormat::BMP => stb::image_write::stbi_write_bmp(
                        path.as_c_str(),
                        width,
                        height,
                        comp.components,
                        image.as_slice(),
                    )
                        .is_some(),
                    OutputFormat::TIFF => false, // todo
                    _ => stb::image_write::stbi_write_jpg(
                        path.as_c_str(),
                        width,
                        height,
                        comp.components,
                        image.as_slice(),
                        100,
                    )
                        .is_some(),
                };
            }
            _ => {
                lock.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub unsafe fn free_image_data(data: *mut U8Array) {
        let _ = Box::from_raw(data);
    }
}
