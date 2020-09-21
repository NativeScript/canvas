use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_longlong, c_uint};
use std::ptr::{null, null_mut};

use image::imageops::FilterType;
use image::{GenericImageView, ImageFormat};
use libc::size_t;

use crate::common::NativeByteArray;

#[repr(C)]
pub struct NativeImageAsset {
    pub(crate) image: Option<image::DynamicImage>,
    pub(crate) error: String,
}

#[repr(C)]
pub enum PixelType {
    RGB,
    RGBA,
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

#[allow(unused)]
pub(crate) fn to_byte_slice(buf: &mut [i8]) -> &mut [u8] {
    unsafe { &mut *(buf as *mut [i8] as *mut [u8]) }
}

impl NativeImageAsset {
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

    pub fn load_from_path(&mut self, path: *const c_char) -> c_uint {
        if !self.error.is_empty() {
            self.error.clear()
        }
        self.image = None;
        let real_path = unsafe { CStr::from_ptr(path) }.to_str().unwrap_or("");
        let result = image::open(real_path);
        match result {
            Ok(result) => {
                self.image = Some(result);
                1
            }
            Err(e) => {
                let error = e.to_string();
                self.error.push_str(error.as_str());
                0
            }
        }
    }

    pub fn load_from_raw(&mut self, buffer: *const u8, size: size_t) -> c_uint {
        if !self.error.is_empty() {
            self.error.clear()
        }
        self.image = None;
        let buf = unsafe { std::slice::from_raw_parts(buffer, size) };
        let result = image::load_from_memory(buf);
        match result {
            Ok(result) => {
                self.image = Some(result);
                1
            }
            Err(e) => {
                let error = e.to_string();
                self.error.push_str(error.as_str());
                0
            }
        }
    }

    pub fn load_from_bytes(&mut self, buf: &[u8]) -> c_uint {
        if !self.error.is_empty() {
            self.error.clear()
        }
        self.image = None;
        let result = image::load_from_memory(buf);
        match result {
            Ok(result) => {
                self.image = Some(result);
                1
            }
            Err(e) => {
                let error = e.to_string();
                self.error.push_str(error.as_str());
                0
            }
        }
    }

    pub fn load_from_bytes_int(&mut self, buf: &mut [i8]) -> c_uint {
        if !self.error.is_empty() {
            self.error.clear()
        }
        self.image = None;
        let buf = unsafe { &*(buf as *mut [i8] as *mut [u8]) };
        let result = image::load_from_memory(buf);
        match result {
            Ok(result) => {
                self.image = Some(result);
                1
            }
            Err(e) => {
                let error = e.to_string();
                self.error.push_str(error.as_str());
                0
            }
        }
    }

    pub fn scale(&mut self, x: c_uint, y: c_uint) {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &self.image {
            Some(image) => {
                image.resize(x, y, FilterType::Triangle);
            }
            _ => {
                self.error.push_str("No Image loaded");
            }
        }
    }

    pub fn flip_x(&mut self) {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &self.image {
            Some(image) => {
                image.fliph();
            }
            _ => {
                self.error.push_str("No Image loaded");
            }
        }
    }

    pub fn flip_x_in_place(&mut self) -> c_longlong {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &mut self.image {
            Some(image) => {
                image::imageops::flip_horizontal_in_place(image);
                1
            }
            _ => {
                self.error.push_str("No Image loaded");
                0
            }
        }
    }

    pub fn flip_y(&mut self) {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &self.image {
            Some(image) => {
                image.flipv();
            }
            _ => {
                self.error.push_str("No Image loaded");
            }
        }
    }

    pub fn flip_y_in_place(&mut self) -> c_longlong {
        if !self.error.is_empty() {
            self.error.clear()
        }
        match &mut self.image {
            Some(image) => {
                image::imageops::flip_vertical_in_place(image);
                1
            }
            _ => {
                self.error.push_str("No Image loaded");
                0
            }
        }
    }

    pub fn bytes(&mut self) -> NativeByteArray {
        self.bytes_with(ByteType::Default)
    }

    fn bytes_with(&mut self, byte_type: ByteType) -> NativeByteArray {
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
                //let mut raw = image_ref.into_raw();
                let mut pixels = raw.into_boxed_slice();
                let raw_pixels = pixels.as_mut_ptr();
                let size = pixels.len();
                mem::forget(pixels);
                NativeByteArray {
                    array: raw_pixels,
                    length: size,
                }
            }
            _ => {
                self.error.push_str("No Image loaded");
                NativeByteArray {
                    array: null_mut(),
                    length: 0,
                }
            }
        }
    }

    pub fn rgba_bytes(&mut self) -> NativeByteArray {
        self.bytes_with(ByteType::RGBA)
    }

    pub fn rgb_bytes(&mut self) -> NativeByteArray {
        self.bytes_with(ByteType::RGB)
    }

    pub fn save_path(&mut self, path: *const c_char, format: OutputFormat) -> c_uint {
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
                    Ok(_) => 1,
                    _ => 0,
                };
                done
            }
            _ => {
                self.error.push_str("No Image loaded");
                0
            }
        }
    }

    pub fn free_image_data(data: NativeByteArray) {
        unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(data.array, data.length));
        }
    }
}

enum ByteType {
    Default,
    RGBA,
    RGB,
}

#[allow(unused)]
pub(crate) fn create_image_asset() -> c_longlong {
    Box::into_raw(Box::new(NativeImageAsset::new())) as *mut _ as i64
}

pub(crate) fn image_asset_load_from_path(asset: c_longlong, path: *const c_char) -> c_uint {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    let result = native_asset.load_from_path(path);
    Box::into_raw(native_asset);
    result
}

#[allow(unused)]
pub(crate) fn image_asset_load_from_raw(
    asset: c_longlong,
    array: *const u8,
    size: size_t,
) -> c_uint {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    let result = native_asset.load_from_raw(array, size);
    Box::into_raw(native_asset);
    result
}

#[allow(unused)]
pub(crate) fn image_asset_load_from_slice_i8(asset: c_longlong, array: &mut [i8]) -> c_uint {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    let result = native_asset.load_from_bytes_int(array);
    Box::into_raw(native_asset);
    result
}

pub(crate) fn image_asset_get_bytes(asset: c_longlong) -> NativeByteArray {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    let result = native_asset.bytes();
    Box::into_raw(native_asset);
    result
}

pub(crate) fn image_asset_get_rgba_bytes(asset: c_longlong) -> NativeByteArray {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    let result = native_asset.rgba_bytes();
    Box::into_raw(native_asset);
    result
}

pub(crate) fn image_asset_get_rgb_bytes(asset: c_longlong) -> NativeByteArray {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    let result = native_asset.rgb_bytes();
    Box::into_raw(native_asset);
    result
}

pub(crate) fn image_asset_free_bytes(data: NativeByteArray) {
    NativeImageAsset::free_image_data(data);
}

pub(crate) fn image_asset_width(asset: c_longlong) -> c_uint {
    let native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    let result = native_asset.width();
    Box::into_raw(native_asset);
    result
}

pub(crate) fn image_asset_height(asset: c_longlong) -> c_uint {
    let native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    let result = native_asset.height();
    Box::into_raw(native_asset);
    result
}

pub(crate) fn image_asset_get_error(asset: c_longlong) -> *const c_char {
    if asset == 0 {
        return null();
    }
    let native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    let result = native_asset.error();
    Box::into_raw(native_asset);
    result
}

pub(crate) fn image_asset_has_error(asset: c_longlong) -> c_uint {
    if asset == 0 {
        return 0;
    }
    let native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    let result = native_asset.error();
    Box::into_raw(native_asset);
    if !result.is_null() {
        return 0;
    }
    return 1;
}

pub(crate) fn image_asset_scale(asset: c_longlong, x: c_uint, y: c_uint) -> c_longlong {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    native_asset.scale(x, y);
    Box::into_raw(native_asset) as *mut _ as i64
}

pub(crate) fn image_asset_flip_x(asset: c_longlong) -> c_longlong {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    native_asset.flip_x();
    Box::into_raw(native_asset) as *mut _ as i64
}

#[allow(unused)]
pub(crate) fn image_asset_flip_x_in_place(asset: c_longlong) -> c_longlong {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    native_asset.flip_x_in_place();
    Box::into_raw(native_asset) as *mut _ as i64
}

pub(crate) fn image_asset_flip_y(asset: c_longlong) -> c_longlong {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    native_asset.flip_y();
    Box::into_raw(native_asset) as *mut _ as i64
}

#[allow(unused)]
pub(crate) fn image_asset_flip_y_in_place_owned(
    _width: u32,
    _height: u32,
    buf: *mut u8,
    length: usize,
) {
    let mut buffer = unsafe { std::slice::from_raw_parts_mut(buf, length) };
    let mut image = image::load_from_memory(buffer).unwrap();
    image::imageops::flip_vertical_in_place(&mut image);
}

#[allow(unused)]
pub(crate) fn image_asset_flip_x_in_place_owned(
    _width: u32,
    _height: u32,
    buf: *mut u8,
    length: usize,
) {
    let mut buffer = unsafe { std::slice::from_raw_parts_mut(buf, length) };
    let mut image = image::load_from_memory(buffer).unwrap();
    image::imageops::flip_horizontal_in_place(&mut image);
}

#[allow(unused)]
pub(crate) fn image_asset_flip_y_in_place(asset: c_longlong) -> c_longlong {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    native_asset.flip_y_in_place();
    Box::into_raw(native_asset) as *mut _ as i64
}

pub(crate) fn image_asset_save_path(
    asset: c_longlong,
    path: *const c_char,
    format: c_uint,
) -> c_uint {
    let mut native_asset: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
    let result = native_asset.save_path(path, OutputFormat::from(format));
    Box::into_raw(native_asset) as *mut _ as i64;
    result
}

pub(crate) fn image_asset_release(asset: c_longlong) {
    let _: Box<NativeImageAsset> = unsafe { Box::from_raw(asset as *mut _) };
}
