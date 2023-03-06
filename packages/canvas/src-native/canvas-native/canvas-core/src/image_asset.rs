use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::io::{BufRead, Read, Seek, SeekFrom};
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr::{null, null_mut};
use std::sync::Arc;

use image::imageops::FilterType;
use image::{DynamicImage, EncodableLayout, ImageFormat, RgbImage, RgbaImage};
use parking_lot::lock_api::{RwLockReadGuard, RwLockWriteGuard};
use parking_lot::RawRwLock;

struct ImageAssetInner {
    image: Option<DynamicImage>,
    error: String,

    #[cfg(feature = "2d")]
    skia_image: Option<skia_safe::Image>,
}

unsafe impl Send for ImageAssetInner {}

pub struct ImageAsset(Arc<parking_lot::RwLock<ImageAssetInner>>);

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
        let asset = asset.0.write();
        if let Some(data) = &asset.image {
            let mut inner = ImageAssetInner {
                image: Some(data.clone()),
                error: String::new(),

                #[cfg(feature = "2d")]
                skia_image: asset.skia_image.as_ref().cloned(),
            };
            return Some(Self(Arc::new(parking_lot::RwLock::new(inner))));
        }
        None
    }

    pub fn new() -> Self {
        Self(Arc::new(parking_lot::RwLock::new(ImageAssetInner {
            image: None,
            error: String::new(),
            #[cfg(feature = "2d")]
            skia_image: None,
        })))
    }

    #[cfg(feature = "2d")]
    pub fn skia_image(&self) -> Option<skia_safe::Image> {
        self.read()
            .skia_image
            .as_ref()
            .map(skia_safe::Image::from)
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

    pub fn set_error(&self, error: &str) {
        let mut lock = self.0.write();
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
        {
            let mut lock = self.get_lock();
            if !lock.error.is_empty() {
                lock.error.clear()
            }
            lock.image = None;
        }
        let file = std::fs::File::open(path);
        match file {
            Ok(file) => {
                let mut reader = std::io::BufReader::new(file);
                return self.load_from_reader(&mut reader);
            }
            Err(e) => {
                let error = e.to_string();
                self.get_lock().error.push_str(error.as_str());
                false
            }
        }
    }

    pub fn load_from_reader<R>(&mut self, reader: &mut R) -> bool
    where
        R: Read + Seek + BufRead,
    {
        let mut lock = self.get_lock();
        if !lock.error.is_empty() {
            lock.error.clear()
        }
        lock.image = None;

        let mut bytes = [0; 16];
        let position = reader.stream_position();

        let result = reader.read(&mut bytes);
        match result {
            Ok(_) => {
                if let Ok(position) = position {
                    let _ = reader.seek(SeekFrom::Start(position));
                }

                match image::guess_format(&bytes) {
                    Ok(mime) => match image::load(reader, mime) {
                        Ok(result) => {
                            let mut lock = self.get_lock();

                            let width = result.width() as i32;
                            let height = result.height() as i32;

                            if let Some(image) = result.as_rgba8() {
                                let info = skia_safe::ImageInfo::new(
                                    skia_safe::ISize::new(width, height),
                                    skia_safe::ColorType::RGBA8888,
                                    skia_safe::AlphaType::Unpremul,
                                    None,
                                );

                                let skia_image = unsafe {
                                    skia_safe::Image::from_raster_data(
                                        &info,
                                        skia_safe::Data::new_bytes(image.as_ref()),
                                        (width * 4) as usize,
                                    )
                                };
                                lock.skia_image = skia_image;
                            } else if let Some(image) = result.as_rgb8() {
                                let info = skia_safe::ImageInfo::new(
                                    skia_safe::ISize::new(width, height),
                                    skia_safe::ColorType::RGB565,
                                    skia_safe::AlphaType::Unpremul,
                                    None,
                                );

                                let skia_image = unsafe {
                                    skia_safe::Image::from_raster_data(
                                        &info,
                                        skia_safe::Data::new_bytes(image.as_ref()),
                                        (width * 4) as usize,
                                    )
                                };
                                lock.skia_image = skia_image;
                            }

                            lock.image = Some(result);

                            true
                        }
                        Err(e) => {
                            let error = e.to_string();
                            let mut lock = self.get_lock();
                            lock.error.clear();
                            lock.error.push_str(error.as_str());
                            false
                        }
                    },
                    Err(e) => {
                        let error = e.to_string();
                        let mut lock = self.get_lock();
                        lock.error.clear();
                        lock.error.push_str(error.as_str());
                        false
                    }
                }
            }
            Err(e) => {
                let error = e.to_string();
                let mut lock = self.get_lock();
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

        match image::load_from_memory(buf) {
            Err(e) => {
                let error = e.to_string();
                let mut lock = self.get_lock();
                lock.error.clear();
                lock.error.push_str(error.as_str());
                false
            }
            Ok(data) => {
                let mut lock = self.get_lock();
                lock.image = Some(match data {
                    DynamicImage::ImageRgba8(_) => data,
                    _ => DynamicImage::ImageRgba8(data.into_rgba8()),
                });

                #[cfg(feature = "2d")]
                {
                    if let Some(image) = lock.image.as_ref() {
                        let width = image.width() as i32;
                        let height = image.height() as i32;

                        let info = skia_safe::ImageInfo::new(
                            skia_safe::ISize::new(width, height),
                            skia_safe::ColorType::RGBA8888,
                            skia_safe::AlphaType::Unpremul,
                            None,
                        );
                        let skia_image = unsafe {
                            skia_safe::Image::from_raster_data(
                                &info,
                                skia_safe::Data::new_bytes(image.as_bytes()),
                                (width * 4) as usize,
                            )
                        };
                        lock.skia_image = skia_image;
                    }
                }

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
                let data = image.resize(x, y, FilterType::Nearest);

                let width = data.width() as i32;
                let height = data.height() as i32;

                #[cfg(feature = "2d")]{
                    if let Some(image) = data.as_rgba8() {
                        let info = skia_safe::ImageInfo::new(
                            skia_safe::ISize::new(width, height),
                            skia_safe::ColorType::RGBA8888,
                            skia_safe::AlphaType::Unpremul,
                            None,
                        );

                        let skia_image = unsafe {
                            skia_safe::Image::from_raster_data(
                                &info,
                                skia_safe::Data::new_bytes(image.as_ref()),
                                (width * 4) as usize,
                            )
                        };
                        lock.skia_image = skia_image;
                    } else if let Some(image) = data.as_rgb8() {
                        let info = skia_safe::ImageInfo::new(
                            skia_safe::ISize::new(width, height),
                            skia_safe::ColorType::RGB565,
                            skia_safe::AlphaType::Unpremul,
                            None,
                        );

                        let skia_image = unsafe {
                            skia_safe::Image::from_raster_data(
                                &info,
                                skia_safe::Data::new_bytes(image.as_ref()),
                                (width * 4) as usize,
                            )
                        };
                        lock.skia_image = skia_image;
                    }
                }


                lock.image = Some(data);

                true
            }
            _ => {
                lock.error.push_str("No Image loaded");
                false
            }
        }
    }

    pub fn get_rgb_bytes(&self, block: fn(Option<&[u8]>)) {
        let lock = self.get_lock();
        match lock.image.as_ref() {
            Some(image) => match image.as_rgb8() {
                Some(image) => block(Some(image.as_ref())),
                None => block(None),
            },
            None => block(None),
        }
    }

    pub fn get_rgba_bytes(&self, block: fn(Option<&[u8]>)) {
        let lock = self.get_lock();
        match lock.image.as_ref() {
            Some(image) => match image.as_rgba8() {
                Some(image) => block(Some(image.as_ref())),
                None => block(None),
            },
            None => block(None),
        }
    }

    pub fn get_bytes(&self) -> Option<&[u8]> {
        self.get_lock().image.as_ref().map(|d| {
            let slice = d.as_bytes();
            unsafe { std::slice::from_raw_parts(slice.as_ptr(), slice.len()) }
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
                return match format {
                    OutputFormat::PNG => image.save_with_format(path, ImageFormat::Png).is_ok(),
                    OutputFormat::ICO => image.save_with_format(path, ImageFormat::Ico).is_ok(),
                    OutputFormat::BMP => image.save_with_format(path, ImageFormat::Bmp).is_ok(),
                    OutputFormat::TIFF => image.save_with_format(path, ImageFormat::Tiff).is_ok(),
                    _ => image.save_with_format(path, ImageFormat::Jpeg).is_ok(),
                };
            }
            _ => {
                lock.error.push_str("No Image loaded");
                false
            }
        }
    }
}
