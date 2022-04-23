use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::io::{Read, Seek, SeekFrom};
use std::os::raw::{c_char, c_uint};
use std::ptr::{null, null_mut};
use std::sync::Arc;

use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat};
use parking_lot::lock_api::MutexGuard;
use parking_lot::RawMutex;

use crate::ffi::u8_array::U8Array;

struct ImageAssetInner {
    image: Option<image::DynamicImage>,
    error: String,
}

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
            2 => OutputFormat::ICO,
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
    pub fn new() -> Self {
        Self(Arc::new(parking_lot::Mutex::new(ImageAssetInner {
            image: None,
            error: String::new(),
        })))
    }
    fn get_lock(&self) -> MutexGuard<'_, RawMutex, ImageAssetInner> {
        self.0.lock()
    }

    pub fn error(&self) -> Cow<str> {
        self.0.lock().error.clone().into()
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
            .image
            .as_ref()
            .map(|v| v.width())
            .unwrap_or_default()
    }

    pub fn height(&self) -> c_uint {
        self.get_lock()
            .image
            .as_ref()
            .map(|v| v.height())
            .unwrap_or_default()
    }

    pub fn load_from_path(&mut self, path: &str) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        lock.image = None;
        let file = std::fs::File::open(path);
        match file {
            Ok(file) => {
                let mut reader = std::io::BufReader::new(file);
                let mut bytes = [0; 16];
                let result = reader.read(&mut bytes);
                match result {
                    Ok(_) => {
                        let _ = reader.seek(SeekFrom::Start(0));
                        let mime = image::guess_format(&bytes);
                        match mime {
                            Ok(mime) => match image::load(reader, mime) {
                                Ok(result) => {
                                    lock.image = Some(result);
                                    true
                                }
                                Err(e) => {
                                    let error = e.to_string();
                                    lock.error.clear();
                                    lock.error.push_str(error.as_str());
                                    false
                                }
                            },
                            Err(e) => {
                                let error = e.to_string();
                                lock.error.clear();
                                lock.error.push_str(error.as_str());
                                false
                            }
                        }
                    }
                    Err(e) => {
                        let error = e.to_string();
                        lock.error.clear();
                        lock.error.push_str(error.as_str());
                        false
                    }
                }
            }
            Err(e) => {
                let error = e.to_string();
                lock.error.clear();
                lock.error.push_str(error.as_str());
                false
            }
        }
    }

    pub unsafe fn load_from_path_raw(&mut self, path: *const c_char) -> bool {
        let real_path = CStr::from_ptr(path);
        self.load_from_path(real_path.to_string_lossy().as_ref())
    }

    pub unsafe fn load_from_raw(&mut self, buffer: *const u8, size: usize) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        lock.image = None;
        match image::load_from_memory(std::slice::from_raw_parts(buffer, size)) {
            Ok(result) => {
                lock.image = Some(result);
                true
            }
            Err(e) => {
                let error = e.to_string();
                lock.error.push_str(error.as_str());
                false
            }
        }
    }

    pub fn load_from_bytes(&mut self, buf: &[u8]) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        lock.image = None;
        let result = image::load_from_memory(buf);
        match result {
            Ok(result) => {
                lock.image = Some(result);
                true
            }
            Err(e) => {
                let error = e.to_string();
                lock.error.push_str(error.as_str());
                false
            }
        }
    }

    pub fn load_from_bytes_int(&mut self, buf: &mut [i8]) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        lock.image = None;
        match image::load_from_memory(unsafe { std::mem::transmute(buf) }) {
            Ok(result) => {
                lock.image = Some(result);
                true
            }
            Err(e) => {
                let error = e.to_string();
                lock.error.push_str(error.as_str());
                false
            }
        }
    }

    pub fn scale(&mut self, x: c_uint, y: c_uint) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        match &lock.image {
            Some(image) => {
                image.resize(x, y, FilterType::Triangle);
                true
            }
            _ => {
                lock.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn flip_x(&mut self) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        match &lock.image {
            Some(image) => {
                image.fliph();
                true
            }
            _ => {
                lock.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn flip_x_in_place(&mut self) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        match &mut lock.image {
            Some(image) => {
                image::imageops::flip_horizontal_in_place(image);
                true
            }
            _ => {
                lock.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn flip_y(&mut self) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        match &lock.image {
            Some(image) => {
                image.flipv();
                true
            }
            _ => {
                lock.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn flip_y_in_place(&mut self) -> bool {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        match &mut lock.image {
            Some(image) => {
                image::imageops::flip_vertical_in_place(image);
                true
            }
            _ => {
                lock.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn bytes_internal(&mut self) -> Vec<u8> {
        self.bytes_internal_with(ByteType::Default)
    }

    fn bytes_internal_with(&mut self, byte_type: ByteType) -> Vec<u8> {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        match &lock.image {
            Some(image) => {
                let raw;
                match byte_type {
                    ByteType::RGB => {
                        let image_ref = image.to_rgb8();
                        raw = image_ref.into_raw();
                    }
                    ByteType::RGBA => {
                        let image_ref = image.to_rgba8();
                        raw = image_ref.into_raw();
                    }
                    ByteType::Default => {
                        raw = image.to_bytes();
                    }
                }
                raw
            }
            _ => {
                lock.error.push_str("No Image loaded");
                vec![]
            }
        }
    }

    pub fn rgba_internal_bytes(&mut self) -> Vec<u8> {
        self.bytes_internal_with(ByteType::RGBA)
    }

    pub fn rgb_internal_bytes(&mut self) -> Vec<u8> {
        self.bytes_internal_with(ByteType::RGB)
    }

    pub fn bytes(&mut self) -> *mut U8Array {
        self.bytes_with(ByteType::Default)
    }

    fn bytes_with(&mut self, byte_type: ByteType) -> *mut U8Array {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        match &lock.image {
            Some(image) => {
                let mut raw;
                match byte_type {
                    ByteType::RGB => {
                        let image_ref = image.to_rgb8();
                        raw = image_ref.into_raw();
                    }
                    ByteType::RGBA => {
                        let image_ref = image.to_rgba8();
                        raw = image_ref.into_raw();
                    }
                    ByteType::Default => {
                        raw = image.to_bytes();
                    }
                }

                let data = Box::into_raw(Box::new(U8Array {
                    data: raw.as_mut_ptr(),
                    data_len: raw.len(),
                    data_cap: raw.capacity(),
                }));
                std::mem::forget(raw);
                data
            }
            _ => {
                lock.error.push_str("No Image loaded");
                Box::into_raw(Box::new(U8Array {
                    data: null_mut(),
                    data_len: 0,
                    data_cap: 0,
                }))
            }
        }
    }

    pub fn rgba_bytes(&mut self) -> *mut U8Array {
        self.bytes_with(ByteType::RGBA)
    }

    pub fn rgb_bytes(&mut self) -> *mut U8Array {
        self.bytes_with(ByteType::RGB)
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
        match &lock.image {
            Some(image) => {
                let format = match format {
                    OutputFormat::PNG => ImageFormat::Png,
                    OutputFormat::ICO => ImageFormat::Ico,
                    OutputFormat::BMP => ImageFormat::Bmp,
                    OutputFormat::TIFF => ImageFormat::Tiff,
                    _ => ImageFormat::Jpeg,
                };
                let done = match image.save_with_format(path, format) {
                    Ok(_) => true,
                    _ => false,
                };
                done
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
