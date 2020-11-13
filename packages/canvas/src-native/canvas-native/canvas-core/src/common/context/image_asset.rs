use std::ffi::{CStr, CString};
use std::io::{Read, Seek, SeekFrom};
use std::os::raw::{c_char, c_uint};
use std::ptr::{null, null_mut};
use std::str::FromStr;

use image::{GenericImageView, ImageFormat};
use image::imageops::FilterType;

use crate::common::ffi::u8_array::U8Array;

pub struct ImageAsset {
    pub(crate) image: Option<image::DynamicImage>,
    pub(crate) error: String,
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

enum ByteType {
    Default,
    RGBA,
    RGB,
}

#[allow(unused)]
pub(crate) fn to_byte_slice(buf: &mut [i8]) -> &mut [u8] {
    unsafe { std::mem::transmute(buf) }
}


impl ImageAsset {
    pub fn new() -> Self {
        Self {
            image: None,
            error: String::new(),
        }
    }

    pub fn error(&self) -> *const c_char {
        if self.error.is_empty() {
            return null();
        }
        CString::new(self.error.as_str()).unwrap().into_raw()
    }

    pub fn width(&self) -> c_uint {
        match &self.image {
            Some(image) => image.width(),
            _ => 0,
        }
    }

    pub fn height(&self) -> c_uint {
        match &self.image {
            Some(image) => image.height(),
            _ => 0,
        }
    }

    pub fn load_from_url(image: std::sync::Arc<tokio::sync::Mutex<&'static mut ImageAsset>>, url: *const c_char, callback: extern fn(bool)) {
        unsafe {
            let url = CStr::from_ptr(url).to_string_lossy();
            let url = url.to_string();
            crate::common::utils::threads::THREAD_POOL.spawn(async move {
                let mut asset = image.lock().await;
                if !asset.error.is_empty() {
                    asset.error.clear()
                }
                asset.image = None;
                match reqwest::Url::from_str(&url) {
                    Ok(url) => {
                        match reqwest::get(url).await {
                            Ok(response) => match response.bytes().await {
                                Ok(bytes) => match image::load_from_memory(&bytes) {
                                    Ok(image) => {
                                        asset.image = Some(image);
                                        callback(true)
                                    }
                                    Err(err) => {
                                        asset.error.clear();
                                        let err = err.to_string();
                                        asset.error.push_str(err.as_str());
                                        callback(false);
                                    }
                                }
                                Err(err) => {
                                    asset.error.clear();
                                    let err = err.to_string();
                                    asset.error.push_str(err.as_str());
                                    callback(false);
                                }
                            }
                            Err(err) => {
                                asset.error.clear();
                                let err = err.to_string();
                                asset.error.push_str(err.as_str());
                                callback(false);
                            }
                        }
                    }
                    Err(err) => {
                        asset.error.clear();
                        let err = err.to_string();
                        asset.error.push_str(err.as_str());
                        callback(false);
                    }
                }
            });
        }
    }

    pub fn load_from_path(&mut self, path: *const c_char) -> bool {
        if !self.error.is_empty() {
            self.error.clear()
        }
        self.image = None;
        let real_path = unsafe { CStr::from_ptr(path) }.to_str().unwrap_or("");
        let file = std::fs::File::open(real_path);
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
                                    self.image = Some(result);
                                    true
                                }
                                Err(e) => {
                                    let error = e.to_string();
                                    self.error.clear();
                                    self.error.push_str(error.as_str());
                                    false
                                }
                            },
                            Err(e) => {
                                let error = e.to_string();
                                self.error.clear();
                                self.error.push_str(error.as_str());
                                false
                            }
                        }
                    }
                    Err(e) => {
                        let error = e.to_string();
                        self.error.clear();
                        self.error.push_str(error.as_str());
                        false
                    }
                }
            }
            Err(e) => {
                let error = e.to_string();
                self.error.clear();
                self.error.push_str(error.as_str());
                false
            }
        }
    }

    pub fn load_from_raw(&mut self, buffer: *const u8, size: usize) -> bool {
        if !self.error.is_empty() {
            self.error.clear()
        }
        self.image = None;
        match image::load_from_memory(unsafe { std::slice::from_raw_parts(buffer, size) }) {
            Ok(result) => {
                self.image = Some(result);
                true
            }
            Err(e) => {
                let error = e.to_string();
                self.error.push_str(error.as_str());
                false
            }
        }
    }

    pub fn load_from_bytes(&mut self, buf: &[u8]) -> bool {
        if !self.error.is_empty() {
            self.error.clear()
        }
        self.image = None;
        let result = image::load_from_memory(buf);
        match result {
            Ok(result) => {
                self.image = Some(result);
                true
            }
            Err(e) => {
                let error = e.to_string();
                self.error.push_str(error.as_str());
                false
            }
        }
    }

    pub fn load_from_bytes_int(&mut self, buf: &mut [i8]) -> bool {
        if !self.error.is_empty() {
            self.error.clear()
        }
        self.image = None;
        match image::load_from_memory(unsafe { std::mem::transmute(buf) }) {
            Ok(result) => {
                self.image = Some(result);
                true
            }
            Err(e) => {
                let error = e.to_string();
                self.error.push_str(error.as_str());
                false
            }
        }
    }

    pub fn scale(&mut self, x: c_uint, y: c_uint) -> bool {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &self.image {
            Some(image) => {
                image.resize(x, y, FilterType::Triangle);
                true
            }
            _ => {
                self.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn flip_x(&mut self) -> bool {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &self.image {
            Some(image) => {
                image.fliph();
                true
            }
            _ => {
                self.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn flip_x_in_place(&mut self) -> bool {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &mut self.image {
            Some(image) => {
                image::imageops::flip_horizontal_in_place(image);
                true
            }
            _ => {
                self.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn flip_y(&mut self) -> bool {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &self.image {
            Some(image) => {
                image.flipv();
                true
            }
            _ => {
                self.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn flip_y_in_place(&mut self) -> bool {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &mut self.image {
            Some(image) => {
                image::imageops::flip_vertical_in_place(image);
                true
            }
            _ => {
                self.error.push_str("No Image loaded");
                false
            }
        }
    }


    pub fn bytes_internal(&mut self) -> Vec<u8> {
        self.bytes_internal_with(ByteType::Default)
    }

    fn bytes_internal_with(&mut self, byte_type: ByteType) -> Vec<u8> {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &self.image {
            Some(image) => {
                let raw;
                match byte_type {
                    ByteType::RGB => {
                        let image_ref = image.to_rgb();
                        raw = image_ref.into_raw();
                    }
                    ByteType::RGBA => {
                        let image_ref = image.to_rgba();
                        raw = image_ref.into_raw();
                    }
                    ByteType::Default => {
                        raw = image.to_bytes();
                    }
                }
                raw
            }
            _ => {
                self.error.push_str("No Image loaded");
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
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &self.image {
            Some(image) => {
                let raw;
                match byte_type {
                    ByteType::RGB => {
                        let image_ref = image.to_rgb();
                        raw = image_ref.into_raw();
                    }
                    ByteType::RGBA => {
                        let image_ref = image.to_rgba();
                        raw = image_ref.into_raw();
                    }
                    ByteType::Default => {
                        raw = image.to_bytes();
                    }
                }
                let mut pixels = raw.to_vec().into_boxed_slice();
                let raw_pixels = pixels.as_mut_ptr();
                let size = pixels.len();
                let data = Box::into_raw(Box::new(U8Array {
                    data: raw_pixels,
                    data_len: size,
                }));
                Box::into_raw(pixels);
                data
            }
            _ => {
                self.error.push_str("No Image loaded");
                Box::into_raw(Box::new(U8Array {
                    data: null_mut(),
                    data_len: 0,
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

    pub fn save_path(&mut self, path: *const c_char, format: OutputFormat) -> bool {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &self.image {
            Some(image) => {
                let format = match format {
                    OutputFormat::PNG => ImageFormat::Png,
                    OutputFormat::ICO => ImageFormat::Ico,
                    OutputFormat::BMP => ImageFormat::Bmp,
                    OutputFormat::TIFF => ImageFormat::Tiff,
                    _ => ImageFormat::Jpeg,
                };
                let real_path = unsafe { CStr::from_ptr(path) }.to_str().unwrap_or("");
                let done = match image.save_with_format(real_path, format) {
                    Ok(_) => true,
                    _ => false,
                };
                done
            }
            _ => {
                self.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn free_image_data(data: *mut U8Array) {
        unsafe {
            let _ = Box::from_raw(data);
        }
    }
}
