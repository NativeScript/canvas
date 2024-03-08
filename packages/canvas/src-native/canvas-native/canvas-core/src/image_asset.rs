use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::io::{BufRead, Read, Seek};
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr::null;
use std::sync::Arc;

use parking_lot::lock_api::{RwLockReadGuard, RwLockWriteGuard};
use parking_lot::RawRwLock;
use stb_image::image::LoadResult;

struct ImageAssetInner {
    image: Option<stb_image::image::Image<u8>>,
    error: String,
    has_alpha: bool,
    #[cfg(feature = "2d")]
    skia_image: Option<skia_safe::Image>
}

unsafe impl Send for ImageAssetInner {}

pub struct ImageAsset(Arc<parking_lot::RwLock<ImageAssetInner>>);

impl Clone for ImageAsset {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
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

enum ByteType {
    Default,
    RGBA,
    RGB,
}

impl ImageAsset {
    pub fn from_raw_bytes(width: usize, height: usize, depth: usize, data: Vec<u8>) -> Self {
        let image = stb_image::image::Image::new(
            width , height, depth , data
        );

        let info = skia_safe::ImageInfo::new(
            skia_safe::ISize::new(width as i32, height as i32),
            skia_safe::ColorType::RGBA8888,
            skia_safe::AlphaType::Unpremul,
            None,
        );


        let skia_image = unsafe {
            skia_safe::images::raster_from_data(&info, skia_safe::Data::new_bytes(image.data.as_slice()), info.min_row_bytes())
        };
        let has_alpha = image.depth == 4;
        let inner = ImageAssetInner {
            image: Some(image),
            error: String::new(),
            has_alpha,
            #[cfg(feature = "2d")]
            skia_image
        };

        Self(Arc::new(parking_lot::RwLock::new(inner)))
    }
    pub fn copy(asset: &ImageAsset) -> Option<ImageAsset> {
        let asset = asset.0.write();
        let mut has_alpha = false;
        let image = asset.image.as_ref().map(|image| {
            has_alpha = image.depth == 4;
            stb_image::image::Image::new(image.width, image.height, image.depth, image.data.clone())
        });
        let inner = ImageAssetInner {
            image,
            error: String::new(),
            has_alpha,
            #[cfg(feature = "2d")]
            skia_image: asset.skia_image.as_ref().cloned(),
        };
        Some(Self(Arc::new(parking_lot::RwLock::new(inner))))
    }

    pub fn new() -> Self {
        Self(Arc::new(parking_lot::RwLock::new(ImageAssetInner {
            image: None,
            error: String::new(),
            has_alpha: false,
            #[cfg(feature = "2d")]
            skia_image: None
        })))
    }

    #[cfg(feature = "2d")]
    pub fn skia_image(&self) -> Option<skia_safe::Image> {
        self.read().skia_image.as_ref().map(skia_safe::Image::from)
    }

    fn read(&self) -> RwLockReadGuard<'_, RawRwLock, ImageAssetInner> {
        self.0.read()
    }

    fn write(&self) -> RwLockWriteGuard<'_, RawRwLock, ImageAssetInner> {
        self.0.write()
    }

    fn get_lock(&self) -> RwLockWriteGuard<'_, RawRwLock, ImageAssetInner> {
        self.0.write()
    }

    pub fn error(&self) -> Cow<str> {
        self.0.read().error.clone().into()
    }

    pub fn error_string(&self) -> String {
        self.0.read().error.clone()
    }

    pub fn set_error(&self, error: &str) {
        let mut lock = self.0.write();
        lock.error.clear();
        lock.error.push_str(error);
    }

    pub fn error_cstr(&self) -> *const c_char {
        let lock = self.read();
        if lock.error.is_empty() {
            return null();
        }
        CString::new(lock.error.as_str()).unwrap().into_raw()
    }

    pub fn has_alpha(&self) -> bool {
        self.read().has_alpha
    }

    pub fn is_valid(&self) -> bool {
        self.read()
            .image
            .as_ref()
            .map(|v| v.width > 0 && v.height > 0)
            .unwrap_or_default()
    }

    pub fn width(&self) -> c_uint {
        self.read()
            .image
            .as_ref()
            .map(|v| v.width as c_uint)
            .unwrap_or_default()
    }

    pub fn height(&self) -> c_uint {
        self.read()
            .image
            .as_ref()
            .map(|v| v.height as c_uint)
            .unwrap_or_default()
    }

    pub fn dimensions(&self) -> (c_uint, c_uint) {
        self.read()
            .image
            .as_ref()
            .map(|v| (v.width as c_uint, v.height as c_uint))
            .unwrap_or_default()
    }

    #[cfg(any(unix))]
    pub fn load_from_fd(&mut self, fd: c_int) -> bool {
        if fd == 0 {
            let mut lock = self.get_lock();

            if !lock.error.is_empty() {
                lock.error.clear()
            }
            lock.image = None;
            return false;
        }
        use std::os::fd::FromRawFd;
        let file = unsafe { std::fs::File::from_raw_fd(fd) };
        let mut reader = std::io::BufReader::new(file);
        let mut buf: Vec<u8> = Vec::new();
        match reader.read_to_end(&mut buf) {
            Ok(_) => self.load_from_bytes(buf.as_slice()),
            Err(e) => {
                let mut lock = self.get_lock();
                let error = e.to_string();
                lock.error.clear();
                lock.error.push_str(error.as_str());
                false
            }
        }
    }

    pub fn load_from_path(&mut self, path: &str) -> bool {
        let file = std::fs::File::open(path);
        match file {
            Ok(file) => {
                let mut reader = std::io::BufReader::new(file);
                self.load_from_reader(&mut reader)
            }
            Err(e) => {
                let mut lock = self.get_lock();
                if !lock.error.is_empty() {
                    lock.error.clear()
                }
                lock.image = None;
                let error = e.to_string();
                lock.error.push_str(error.as_str());
                false
            }
        }
    }

    pub fn load_from_reader<R>(&mut self, reader: &mut R) -> bool
    where
        R: Read + Seek + BufRead,
    {
        let mut buf = Vec::new();
        match reader.read_to_end(&mut buf) {
            Ok(_) => self.load_from_bytes(buf.as_slice()),
            Err(e) => {
                let mut lock = self.get_lock();
                if !lock.error.is_empty() {
                    lock.error.clear()
                }
                lock.image = None;
                let error = e.to_string();
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
        self.load_from_bytes(std::slice::from_raw_parts(buffer, size))
    }

    pub fn load_from_bytes(&mut self, buf: &[u8]) -> bool {
        {
            let mut lock = self.get_lock();
            if !lock.error.is_empty() {
                lock.error.clear()
            }
            lock.image = None;
        }

        unsafe {
            stb_image::stb_image::stbi_set_unpremultiply_on_load(1);
            stb_image::stb_image::stbi_convert_iphone_png_to_rgb(1);
        }

        match stb_image::image::load_from_memory_with_depth(buf, 4, true) {
            LoadResult::Error(e) => {
                let mut lock = self.get_lock();
                let error = e.to_string();
                lock.error.push_str(error.as_str());
                false
            }
            LoadResult::ImageU8(image) => {
                let mut lock = self.get_lock();
                let width = image.width as i32;
                let height = image.height as i32;

                #[cfg(feature = "2d")]
                {
                    let color_type = skia_safe::ColorType::RGBA8888;
                    let alpha_type = skia_safe::AlphaType::Unpremul;
                    let info = skia_safe::ImageInfo::new(
                        skia_safe::ISize::new(width, height),
                        color_type,
                        alpha_type,
                        None,
                    );

                    lock.skia_image = skia_safe::images::raster_from_data(
                        &info,
                        unsafe { skia_safe::Data::new_bytes(image.data.as_slice()) },
                        info.min_row_bytes(),
                    );
                }
                lock.image = Some(image);
                true
            }
            LoadResult::ImageF32(_) => {
                let mut lock = self.get_lock();
                lock.image = None;
                false
            }
        }
    }

    pub fn load_from_bytes_int(&mut self, buf: &mut [i8]) -> bool {
        self.load_from_bytes(unsafe { std::mem::transmute(buf) })
    }

    pub fn load_from_raw_bytes(&mut self, width: usize, height: usize, depth: usize, data: Vec<u8>) -> bool  {
        let image = stb_image::image::Image::new(
            width , height, depth , data
        );

        let info = skia_safe::ImageInfo::new(
            skia_safe::ISize::new(width as i32, height as i32),
            skia_safe::ColorType::RGBA8888,
            skia_safe::AlphaType::Unpremul,
            None,
        );


        let skia_image = unsafe {
            skia_safe::images::raster_from_data(&info, skia_safe::Data::new_bytes(image.data.as_slice()), info.min_row_bytes())
        };

        let mut lock = self.get_lock();
        lock.error.clear();
        lock.image = Some(image);
        lock.has_alpha = depth > 3;
        lock.skia_image = skia_image;

        true
    }

    fn byte_swap(data: &mut [u8]) {
        let length = data.len();
        for i in (0..length).step_by(4) {
            data.swap(i + 2, i);
        }
    }

    fn byte_swap_and_premultiply(data: &mut [u8]) {
        let length = data.len();
        for i in (0..length).step_by(4) {
            let r = data[i + 2];
            let g = data[i + 1];
            let b = data[i];
            let a = data[i + 3];
            data[i] = ((r as u32) * (a as u32) / 255) as u8;
            data[i + 1] = ((g as u32) * (a as u32) / 255) as u8;
            data[i + 2] = ((b as u32) * (a as u32) / 255) as u8;
        }
    }

    pub fn get_luminance_bytes(&self) -> Option<Vec<u8>> {
        self.read().image.as_ref().map(|image| {
            let slice = image.data.as_slice();
            let mut buf: Vec<u8> = Vec::with_capacity(slice.len() / 4);
            for (index, chunk) in slice.chunks(4).enumerate() {
                let red = chunk[0] as f32;
                let green = chunk[1] as f32;
                let blue = chunk[2] as f32;

                buf[index] = (0.299 * red + 0.587 * green + 0.114 * blue) as u8;
            }
            buf
        })
    }

    pub fn get_luminance_alpha_bytes(&self) -> Option<Vec<u8>> {
        self.read().image.as_ref().map(|image| {
            let slice = image.data.as_slice();
            let mut buf: Vec<u8> = Vec::with_capacity(slice.len() / 4 * 2);
            for (rgba, luma) in slice.chunks(4).zip(buf.chunks_mut(2)) {
                let red = rgba[0] as f32;
                let green = rgba[1] as f32;
                let blue = rgba[2] as f32;
                let alpha = rgba[3];

                luma[0] = (0.299 * red + 0.587 * green + 0.114 * blue) as u8;
                luma[1] = alpha;
            }
            buf
        })
    }

    pub fn get_bytes(&self) -> Option<&[u8]> {
        self.read()
            .image
            .as_ref()
            .map(|d| unsafe { std::slice::from_raw_parts(d.data.as_ptr(), d.data.len()) })
    }

    pub fn copy_bytes(&self) -> Option<Vec<u8>> {
        self.read().image.as_ref().map(|d| d.data.clone())
    }

    pub fn rgb565_to_rgba8888(data: &[u8]) -> Vec<u8> {
        let mut rgba_data = Vec::with_capacity(data.len() * 2);
        for chunk in data.chunks(2) {
            let r = (chunk[0] & 0xF8) >> 3;
            let g = ((chunk[0] & 0x07) << 5) | ((chunk[1] & 0xE0) >> 3);
            let b = (chunk[1] & 0x1F) << 3;

            let a = 255;

            rgba_data.extend_from_slice(&[r, g, b, a]);
        }
        rgba_data
    }
}
