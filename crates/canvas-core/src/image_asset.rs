use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::io::{BufRead, Read, Seek};
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr::null;
use std::sync::Arc;

use stb_image::image::LoadResult;

struct ImageAssetInner {
    image: Option<stb_image::image::Image<u8>>,
    error: Cow<'static, str>,
    has_alpha: bool,
    #[cfg(feature = "2d")]
    skia_image: Option<skia_safe::Image>,
}

unsafe impl Send for ImageAssetInner {}

pub struct ImageAsset(Arc<parking_lot::Mutex<ImageAssetInner>>);

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

impl ImageAsset {
    pub fn from_raw_bytes(width: usize, height: usize, depth: usize, data: Vec<u8>) -> Self {
        let image = stb_image::image::Image::new(width, height, depth, data);

        let info = skia_safe::ImageInfo::new(
            skia_safe::ISize::new(width as i32, height as i32),
            skia_safe::ColorType::RGBA8888,
            skia_safe::AlphaType::Unpremul,
            None,
        );

        let skia_image = unsafe {
            skia_safe::images::raster_from_data(
                &info,
                skia_safe::Data::new_bytes(image.data.as_slice()),
                info.min_row_bytes(),
            )
        };
        let has_alpha = image.depth == 4;
        let inner = ImageAssetInner {
            image: Some(image),
            error: Cow::default(),
            has_alpha,
            #[cfg(feature = "2d")]
            skia_image,
        };

        Self(Arc::new(parking_lot::Mutex::new(inner)))
    }
    pub fn copy(asset: &ImageAsset) -> Option<ImageAsset> {
        let asset = asset.0.lock();
        let mut has_alpha = false;
        let image = asset.image.as_ref().map(|image| {
            has_alpha = image.depth == 4;
            stb_image::image::Image::new(image.width, image.height, image.depth, image.data.clone())
        });
        let inner = ImageAssetInner {
            image,
            error: Cow::default(),
            has_alpha,
            #[cfg(feature = "2d")]
            skia_image: asset.skia_image.as_ref().cloned(),
        };
        Some(Self(Arc::new(parking_lot::Mutex::new(inner))))
    }

    pub fn new() -> Self {
        Self(Arc::new(parking_lot::Mutex::new(ImageAssetInner {
            image: None,
            error: Cow::default(),
            has_alpha: false,
            #[cfg(feature = "2d")]
            skia_image: None,
        })))
    }

    #[cfg(feature = "2d")]
    pub fn skia_image(&self) -> Option<skia_safe::Image> {
        let lock = self.0.lock();
        lock.skia_image.as_ref().map(skia_safe::Image::from)
    }

    pub fn error(&self) -> Cow<'static, str> {
        self.0.lock().error.clone()
    }

    pub fn error_string(&self) -> String {
        self.0.lock().error.to_string()
    }

    pub fn set_error(&self, error: &str) {
        let mut lock = self.0.lock();
        lock.error = Cow::Owned(error.to_string());
    }

    pub fn error_cstr(&self) -> *const c_char {
        let lock = self.0.lock();
        if lock.error.is_empty() {
            return null();
        }
        CString::new(lock.error.as_ref()).unwrap().into_raw()
    }

    pub fn has_alpha(&self) -> bool {
        self.0.lock().has_alpha
    }

    pub fn is_valid(&self) -> bool {
        self.0
            .lock()
            .image
            .as_ref()
            .map(|v| v.width > 0 && v.height > 0)
            .unwrap_or_default()
    }

    pub fn width(&self) -> c_uint {
        self.0
            .lock()
            .image
            .as_ref()
            .map(|v| v.width as c_uint)
            .unwrap_or_default()
    }

    pub fn height(&self) -> c_uint {
        self.0
            .lock()
            .image
            .as_ref()
            .map(|v| v.height as c_uint)
            .unwrap_or_default()
    }

    pub fn dimensions(&self) -> (c_uint, c_uint) {
        self.0
            .lock()
            .image
            .as_ref()
            .map(|v| (v.width as c_uint, v.height as c_uint))
            .unwrap_or_default()
    }

    #[cfg(unix)]
    pub fn load_from_fd(&self, fd: c_int) -> bool {
        if fd == 0 {
            let mut lock = self.0.lock();

            if !lock.error.is_empty() {
                lock.error = Cow::default();
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
                let mut lock = self.0.lock();
                let error = e.to_string();
                lock.error = Cow::Owned(error);
                false
            }
        }
    }

    pub fn load_from_path(&self, path: &str) -> bool {
        let file = std::fs::File::open(path);
        match file {
            Ok(file) => {
                let mut reader = std::io::BufReader::new(file);
                self.load_from_reader(&mut reader)
            }
            Err(e) => {
                let mut lock = self.0.lock();
                lock.image = None;
                let error = e.to_string();
                lock.error = Cow::Owned(error);
                false
            }
        }
    }

    pub fn load_from_reader<R>(&self, reader: &mut R) -> bool
    where
        R: Read + Seek + BufRead,
    {
        let mut buf = Vec::new();
        match reader.read_to_end(&mut buf) {
            Ok(_) => self.load_from_bytes(buf.as_slice()),
            Err(e) => {
                let mut lock = self.0.lock();
                lock.image = None;
                let error = e.to_string();
                lock.error = Cow::Owned(error);
                false
            }
        }
    }

    pub unsafe fn load_from_path_raw(&self, path: *const c_char) -> bool {
        let real_path = CStr::from_ptr(path);
        self.load_from_path(real_path.to_string_lossy().as_ref())
    }

    pub unsafe fn load_from_raw(&self, buffer: *const u8, size: usize) -> bool {
        self.load_from_bytes(std::slice::from_raw_parts(buffer, size))
    }

    pub fn load_from_bytes(&self, buf: &[u8]) -> bool {
        {
            let mut lock = self.0.lock();
            if !lock.error.is_empty() {
                lock.error = Cow::default();
            }
            lock.image = None;
        }

        unsafe {
            stb_image::stb_image::stbi_set_unpremultiply_on_load(1);
            stb_image::stb_image::stbi_convert_iphone_png_to_rgb(1);
        }

        match stb_image::image::load_from_memory_with_depth(buf, 4, true) {
            LoadResult::Error(e) => {
                let mut lock = self.0.lock();
                let error = e.to_string();
                lock.error = Cow::Owned(error);
                false
            }
            LoadResult::ImageU8(image) => {
                let mut lock = self.0.lock();
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
                let mut lock = self.0.lock();
                lock.image = None;
                false
            }
        }
    }

    pub fn load_from_bytes_int(&mut self, buf: &mut [i8]) -> bool {
        self.load_from_bytes(unsafe { std::mem::transmute(buf) })
    }

    pub fn load_from_raw_bytes(
        &mut self,
        width: usize,
        height: usize,
        depth: usize,
        data: Vec<u8>,
    ) -> bool {
        let image = stb_image::image::Image::new(width, height, depth, data);

        let info = skia_safe::ImageInfo::new(
            skia_safe::ISize::new(width as i32, height as i32),
            skia_safe::ColorType::RGBA8888,
            skia_safe::AlphaType::Unpremul,
            None,
        );

        let skia_image = unsafe {
            skia_safe::images::raster_from_data(
                &info,
                skia_safe::Data::new_bytes(image.data.as_slice()),
                info.min_row_bytes(),
            )
        };

        let mut lock = self.0.lock();
        lock.error = Cow::default();
        lock.image = Some(image);
        lock.has_alpha = depth > 3;
        lock.skia_image = skia_image;

        true
    }

    #[allow(unused)]
    fn byte_swap(data: &mut [u8]) {
        let length = data.len();
        for i in (0..length).step_by(4) {
            data.swap(i + 2, i);
        }
    }

    #[allow(unused)]
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

    pub fn get_rgb_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_ref().map(|image| {
            let slice = image.data.as_slice();
            let width = image.width;
            let height = image.height;
            Self::rgba_to_rgb(slice, width, height)
        })
    }

    pub fn get_luminance_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_ref().map(|image| {
            let slice = image.data.as_slice();
            let width = image.width;
            let height = image.height;
            Self::rgba_to_luminance(slice, width, height)
        })
    }

    pub fn get_luminance_alpha_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_ref().map(|image| {
            let slice = image.data.as_slice();
            let width = image.width;
            let height = image.height;
            Self::rgba_to_luminance_alpha(slice, width, height)
        })
    }

    pub fn get_alpha_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_ref().map(|image| {
            let slice = image.data.as_slice();
            let width = image.width;
            let height = image.height;
            Self::rgba_to_alpha(slice, width, height)
        })
    }

    pub fn get_bytes(&self) -> Option<&[u8]> {
        self.0
            .lock()
            .image
            .as_ref()
            .map(|d| unsafe { std::slice::from_raw_parts(d.data.as_ptr(), d.data.len()) })
    }

    pub fn get_bytes_mut(&self) -> Option<&mut [u8]> {
        self.0
            .lock()
            .image
            .as_mut()
            .map(|d| unsafe { std::slice::from_raw_parts_mut(d.data.as_mut_ptr(), d.data.len()) })
    }

    pub fn len(&self) -> usize {
        self.0
            .lock()
            .image
            .as_ref()
            .map(|d| d.data.len())
            .unwrap_or_default()
    }

    pub fn copy_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_ref().map(|d| d.data.clone())
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

    pub fn rgba_to_alpha(data: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0_u8; width * height];
        if data.len() < width * height * 4 {
            return buf;
        }
        for i in 0..(width * height) {
            let alpha = data[i * 4 + 3];
            buf[i] = alpha;
        }
        buf
    }
    pub fn rgba_to_luminance_alpha(data: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0_u8; width * height * 2];
        if data.len() < width * height * 4 {
            return buf;
        }
        for i in 0..(width * height) {
            let red = data[i * 4] as f32;
            let green = data[i * 4 + 1] as f32;
            let blue = data[i * 4 + 2] as f32;
            let alpha = data[i * 4 + 3];

            buf[i * 2] = (0.299 * red + 0.587 * green + 0.114 * blue) as u8;
            buf[i * 2 + 1] = alpha;
        }
        buf
    }

    pub fn rgba_to_luminance(data: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![255_u8; width * height];
        if data.len() < width * height * 4 {
            return buf;
        }
        for i in 0..(width * height) {
            let red = data[i * 4] as f32;
            let green = data[i * 4 + 1] as f32;
            let blue = data[i * 4 + 2] as f32;

            buf[i * 2] = (0.299 * red + 0.587 * green + 0.114 * blue) as u8;
        }
        buf
    }

    pub fn rgb_to_luminance(data: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![255_u8; width * height];
        if data.len() < width * height * 3 {
            return buf;
        }
        for i in 0..(width * height) {
            let red = data[i * 3] as f32;
            let green = data[i * 3 + 1] as f32;
            let blue = data[i * 3 + 2] as f32;

            buf[i] = (0.299 * red + 0.587 * green + 0.114 * blue) as u8;
        }
        buf
    }

    pub fn rgba_to_rgb(data: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![255_u8; width * height * 3];
        if data.len() < width * height * 4 {
            return buf;
        }

        for i in 0..(width * height) {
            let red = data[i * 4];
            let green = data[i * 4 + 1];
            let blue = data[i * 4 + 2];

            buf[i * 3] = red;
            buf[i * 3 + 1] = green;
            buf[i * 3 + 2] = blue;
        }
        buf
    }
}
