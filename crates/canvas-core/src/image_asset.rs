use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::fmt::{Debug, Formatter};
use std::io::{BufRead, Read, Seek};
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr::null;
use std::sync::Arc;

enum CanvasImage {
    #[cfg(not(feature = "2d"))]
    Stb(stb_image::image::Image<u8>),
    #[cfg(feature = "2d")]
    Skia(skia_safe::Bitmap, Arc<skia_safe::Pixmap<'static>>, Vec<u8>),
}


impl CanvasImage {
    pub fn with_bytes_dimension<F>(&self, f: F)
    where
        F: FnOnce(&[u8], (u32, u32)),
    {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                f(image.data.as_slice(), (image.width as u32, image.height as u32))
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _, data) => {
                let dimensions = (image.width() as u32, image.height() as u32);;
                f(data.as_slice(), dimensions)
            }
        }
    }

    pub fn with_bytes<F>(&self, f: F)
    where
        F: FnOnce(&[u8]),
    {
        self.with_bytes_dimension(|bytes, _| {
            f(bytes)
        })
    }

    pub fn with_bytes_mut_dimension<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [u8], (u32, u32)),
    {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                f(image.data.as_mut_slice(), (image.width as u32, image.height as u32))
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _, data) => {
                let dimensions = (image.width() as u32, image.height() as u32);
                f(data.as_mut_slice(), dimensions)
            }
        }
    }


    pub fn with_bytes_mut<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [u8]),
    {
        self.with_bytes_mut_dimension(|bytes, _| {
            f(bytes)
        })
    }

    #[cfg(feature = "2d")]
    pub fn new(info: &skia_safe::ImageInfo, mut data: Vec<u8>) -> Self {
        let mut bitmap = skia_safe::Bitmap::new();
        let min_row_bytes = info.min_row_bytes();
        let size = data.len();
        let slice = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr(), size) };
        let pixmap = Arc::new(skia_safe::Pixmap::new(info, slice, min_row_bytes).unwrap());
        unsafe { bitmap.install_pixels(info, pixmap.writable_addr(), min_row_bytes) };
        Self::Skia(bitmap, pixmap, data)
    }

    #[cfg(not(feature = "2d"))]
    pub fn new(width: usize, height: usize, depth: usize, data: Vec<u8>) -> Self {
        let image = stb_image::image::Image::new(width, height, depth, data);
        Self::Stb(image)
    }

    pub fn width(&self) -> u32 {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                image.width as u32
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _, _) => {
                image.width() as u32
            }
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                image.height as u32
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _, _) => {
                image.height() as u32
            }
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                (image.width as u32, image.height as u32)
            }

            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _, _) => {
                (image.width() as u32, image.height() as u32)
            }
        }
    }

    pub fn length(&self) -> usize {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => image.data.len(),
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _, _) => image.compute_byte_size()
        }
    }
}

impl Clone for CanvasImage {
    fn clone(&self) -> Self {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let image = stb_image::image::Image::new(
                    image.width, image.height, image.depth, image.data.clone(),
                );
                Self::Stb(image)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _, data) => {
                let data = data.clone();
                CanvasImage::new(image.info(), data)
            }
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl Debug for CanvasImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let mut d = f.debug_struct("Image");
                d.field("width", &image.width);
                d.field("height", &image.height);
                d.finish()
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _, _) => {
                let mut d = f.debug_struct("Image");
                d.field("width", &image.width());
                d.field("height", &image.height());
                d.finish()
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct ImageAssetInner {
    image: Option<CanvasImage>,
    error: Cow<'static, str>,
    has_alpha: bool,
}

unsafe impl Send for ImageAssetInner {}

#[derive(Clone, Debug, Default)]
pub struct ImageAsset(Arc<parking_lot::Mutex<ImageAssetInner>>);

impl ImageAsset {
    pub fn with_bytes_dimension<F>(&self, f: F)
    where
        F: FnOnce(&[u8], (u32, u32)),
    {
        let lock = self.0.lock();
        match lock.image.as_ref() {
            None => {
                f(&[], (0, 0))
            }
            Some(image) => {
                image.with_bytes_dimension(f)
            }
        }
    }

    pub fn with_bytes<F>(&self, f: F)
    where
        F: FnOnce(&[u8]),
    {
        self.with_bytes_dimension(|bytes, _| {
            f(bytes)
        })
    }

    pub fn with_bytes_mut_dimension<F>(&self, f: F)
    where
        F: FnOnce(&mut [u8], (u32, u32)),
    {
        let mut lock = self.0.lock();
        match lock.image.as_mut() {
            None => {
                f(&mut [], (0, 0))
            }
            Some(image) => {
                image.with_bytes_mut_dimension(f)
            }
        }
    }

    pub fn with_bytes_mut<F>(&self, f: F)
    where
        F: FnOnce(&mut [u8]),
    {
        self.with_bytes_mut_dimension(|bytes, _| {
            f(bytes)
        })
    }

    pub fn with_skia_bitmap<F>(&self, f: F)
    where
        F: FnOnce(Option<&skia_safe::Bitmap>),
    {
        let mut lock = self.0.lock();
        match lock.image.as_mut() {
            None => {
                f(None)
            }
            Some(image) => {
                unsafe {
                    match image {
                        #[cfg(not(feature = "2d"))]
                        CanvasImage::Stb(image) => {
                            // should always be rgba
                            let info = skia_safe::ImageInfo::new(
                                (image.width as i32, image.height as i32),
                                skia_safe::ColorType::RGBA8888,
                                skia_safe::AlphaType::Unpremul,
                                None,
                            );
                            let mut bm = skia_safe::Bitmap::new();
                            let success = unsafe { bm.install_pixels(&info, image.data.as_mut_ptr() as *mut c_void, info.min_row_bytes()) };

                            if success {
                                f(Some(&bm))
                            } else {
                                f(None)
                            }
                        }
                        #[cfg(feature = "2d")]
                        CanvasImage::Skia(image, _, _) => {
                            f(Some(image))
                        }
                    }
                }
            }
        }
    }


    pub fn with_skia_image<F>(&self, f: F)
    where
        F: FnOnce(Option<&skia_safe::Image>),
    {
        let mut lock = self.0.lock();
        match lock.image.as_mut() {
            None => {
                f(None)
            }
            Some(image) => {
                unsafe {
                    match image {
                        #[cfg(not(feature = "2d"))]
                        CanvasImage::Stb(image) => {
                            // should always be rgba
                            let info = skia_safe::ImageInfo::new(
                                (image.width as i32, image.height as i32),
                                skia_safe::ColorType::RGBA8888,
                                skia_safe::AlphaType::Unpremul,
                                None,
                            );
                            let data = unsafe { skia_safe::Data::new_bytes(image.data.as_slice()) };
                            let image = skia_safe::images::raster_from_data(&info, data, info.min_row_bytes());
                            f(image.as_ref())
                        }
                        #[cfg(feature = "2d")]
                        CanvasImage::Skia(bitmap, _, _) => {
                            let image = skia_safe::images::raster_from_bitmap(bitmap);
                            f(image.as_ref())
                        }
                    }
                }
            }
        }
    }

    pub fn with_skia_image_bytes<F>(&self, f: F)
    where
        F: FnOnce(Option<&skia_safe::Image>, Option<((u32, u32), &[u8])>),
    {
        let lock = self.0.lock();
        match lock.image.as_ref() {
            None => {
                f(None, None)
            }
            Some(image) => {
                unsafe {
                    match image {
                        #[cfg(not(feature = "2d"))]
                        CanvasImage::Stb(image) => {
                            // should always be rgba
                            let info = skia_safe::ImageInfo::new(
                                (image.width as i32, image.height as i32),
                                skia_safe::ColorType::RGBA8888,
                                skia_safe::AlphaType::Unpremul,
                                None,
                            );
                            let dimensions = (image.width as u32, image.height as u32);
                            let slice = image.data.as_slice();
                            let data = unsafe { skia_safe::Data::new_bytes(slice) };
                            let image = skia_safe::images::raster_from_data(&info, data, info.min_row_bytes());
                            f(image.as_ref(), Some((dimensions, slice)))
                        }
                        #[cfg(feature = "2d")]
                        CanvasImage::Skia(bitmap, _, bytes) => {
                            let image = skia_safe::images::raster_from_bitmap(bitmap);
                            let dimensions = bitmap.dimensions();
                            // should not fail to cast
                            f(image.as_ref(), Some(((dimensions.width as u32, dimensions.height as u32), bytes)))
                        }
                    }
                }
            }
        }
    }


    pub fn close(&self) {
        let mut lock = self.0.lock();
        lock.image = None;
    }
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }

    #[cfg(feature = "2d")]
    pub fn from_raw_bytes(width: usize, height: usize, color_type: skia_safe::ColorType, alpha_type: skia_safe::AlphaType, data: Vec<u8>) -> Self {
        let info = skia_safe::ImageInfo::new(
            skia_safe::ISize::new(width as i32, height as i32),
            color_type,
            alpha_type,
            None,
        );

        let has_alpha = info.is_opaque();
        let inner = ImageAssetInner {
            image: Some(CanvasImage::new(&info, data)),
            error: Cow::default(),
            has_alpha,
        };

        Self(Arc::new(parking_lot::Mutex::new(inner)))
    }

    #[cfg(not(feature = "2d"))]
    pub fn from_raw_bytes(width: usize, height: usize, depth: usize, data: Vec<u8>) -> Self {
        let image = stb_image::image::Image::new(width, height, depth, data);
        let inner = ImageAssetInner {
            image: Some(CanvasImage::Stb(image)),
            error: Cow::default(),
            has_alpha: depth == 4,
        };

        Self(Arc::new(parking_lot::Mutex::new(inner)))
    }
    pub fn copy(asset: &ImageAsset) -> Option<ImageAsset> {
        let asset = asset.0.lock();

        let image = asset.image.clone();

        let inner = ImageAssetInner {
            image,
            error: Cow::default(),
            has_alpha: asset.has_alpha,
        };
        Some(Self(Arc::new(parking_lot::Mutex::new(inner))))
    }

    pub fn new() -> Self {
        Self(Arc::new(parking_lot::Mutex::default()))
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
            .map(|v| v.width() > 0 && v.height() > 0)
            .unwrap_or_default()
    }

    pub fn width(&self) -> c_uint {
        self.0
            .lock()
            .image
            .as_ref()
            .map(|v| v.width() as c_uint)
            .unwrap_or_default()
    }

    pub fn height(&self) -> c_uint {
        self.0
            .lock()
            .image
            .as_ref()
            .map(|v| v.height() as c_uint)
            .unwrap_or_default()
    }

    pub fn dimensions(&self) -> (c_uint, c_uint) {
        self.0
            .lock()
            .image
            .as_ref()
            .map(|v| v.dimensions())
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


    #[cfg(feature = "2d")]
    pub fn load_from_bytes(&self, buf: &[u8]) -> bool {
        let mut lock = self.0.lock();
        lock.error = Cow::default();
        lock.image = None;
        let data = unsafe { skia_safe::Data::new_bytes(buf) };

        match skia_safe::images::deferred_from_encoded_data(data, None) {
            None => {
                lock.error = Cow::Borrowed("Failed to decode image");
                false
            }
            Some(image) => {
                let info = skia_safe::ImageInfo::new(
                    image.dimensions(),
                    skia_safe::ColorType::RGBA8888,
                    skia_safe::AlphaType::Unpremul,
                    None,
                );
                let row_bytes = (info.width() * 4) as usize;
                let size = info.height() as usize * row_bytes;
                let mut buffer = vec![0u8; size];
                let success = image.read_pixels(image.image_info(), buffer.as_mut_slice(), row_bytes, (0i32, 0i32), skia_safe::image::CachingHint::Allow);
                if success {
                    lock.image = Some(CanvasImage::new(&info, buffer));
                } else {
                    lock.error = Cow::Borrowed("Failed to decode image");
                }
                success
            }
        }
    }


    #[cfg(not(feature = "2d"))]
    pub fn load_from_bytes(&self, buf: &[u8]) -> bool {
        let mut lock = self.0.lock();
        lock.error = Cow::default();
        lock.image = None;


        unsafe {
            stb_image::stb_image::stbi_set_unpremultiply_on_load(1);
            stb_image::stb_image::stbi_convert_iphone_png_to_rgb(1);
        }

        match stb_image::image::load_from_memory_with_depth(buf, 4, true) {
            LoadResult::Error(e) => {
                let error = e.to_string();
                lock.error = Cow::Owned(error);
                false
            }
            LoadResult::ImageU8(image) => {
                lock.image = Some(CanvasImage::Stb(image));
                true
            }
            LoadResult::ImageF32(_) => {
                lock.image = None;
                false
            }
        }
    }


    pub fn load_from_bytes_int(&self, buf: &mut [i8]) -> bool {
        self.load_from_bytes(unsafe { std::mem::transmute(buf) })
    }

    #[cfg(feature = "2d")]
    pub fn load_from_raw_bytes_rgba(
        &self,
        width: u32,
        height: u32,
        data: Vec<u8>,
    ) -> bool {
        let info = skia_safe::ImageInfo::new(
            (width as i32, height as i32),
            skia_safe::ColorType::RGBA8888,
            skia_safe::AlphaType::Unpremul,
            None,
        );
        self.load_from_raw_bytes(&info, data)
    }

    #[cfg(feature = "2d")]
    pub fn load_from_raw_bytes(
        &self,
        info: &skia_safe::ImageInfo,
        data: Vec<u8>,
    ) -> bool {
        let has_alpha = !info.is_opaque();
        let mut lock = self.0.lock();
        lock.error = Cow::default();
        lock.image = Some(CanvasImage::new(info, data));
        lock.has_alpha = has_alpha;
        true
    }


    #[cfg(not(feature = "2d"))]
    pub fn load_from_raw_bytes(
        &self,
        width: usize,
        height: usize,
        depth: usize,
        data: Vec<u8>,
    ) -> bool {
        let image = stb_image::image::Image::new(width, height, depth, data);

        let mut lock = self.0.lock();
        lock.error = Cow::default();
        lock.image = Some(CanvasImage::Stb(image));
        lock.has_alpha = depth > 3;

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
        self.0.lock().image.as_mut().map(|image| {
            match image {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    let slice = image.data.as_slice();
                    let width = image.width;
                    let height = image.height;
                    Self::rgba_to_rgb(slice, width, height)
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _, _) => {
                    let size = image.compute_byte_size();
                    let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                    let width = image.width() as usize;
                    let height = image.height() as usize;
                    Self::rgba_to_rgb(slice, width, height)
                }
            }
        })
    }

    pub fn get_luminance_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_mut().map(|image| {
            match image {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    let slice = image.data.as_slice();
                    let width = image.width;
                    let height = image.height;
                    Self::rgba_to_luminance(slice, width, height)
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _, _) => {
                    let size = image.compute_byte_size();
                    let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                    let width = image.width() as usize;
                    let height = image.height() as usize;
                    Self::rgba_to_luminance(slice, width, height)
                }
            }
        })
    }

    pub fn get_luminance_alpha_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_mut().map(|image| {
            match image {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    let slice = image.data.as_slice();
                    let width = image.width;
                    let height = image.height;
                    Self::rgba_to_luminance_alpha(slice, width, height)
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _, _) => {
                    let size = image.compute_byte_size();
                    let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                    let width = image.width() as usize;
                    let height = image.height() as usize;
                    Self::rgba_to_luminance_alpha(slice, width, height)
                }
            }
        })
    }

    pub fn get_alpha_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_mut().map(|image| {
            match image {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    let slice = image.data.as_slice();
                    let width = image.width;
                    let height = image.height;
                    Self::rgba_to_alpha(slice, width, height)
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _, _) => {
                    let size = image.compute_byte_size();
                    let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                    let width = image.width() as usize;
                    let height = image.height() as usize;
                    Self::rgba_to_alpha(slice, width, height)
                }
            }
        })
    }

    pub fn get_red_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_mut().map(|image| {
            match image {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    let slice = image.data.as_slice();
                    let width = image.width;
                    let height = image.height;
                    Self::rgba_to_red(slice, width, height)
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _, _) => {
                    let size = image.compute_byte_size();
                    let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                    let width = image.width() as usize;
                    let height = image.height() as usize;
                    Self::rgba_to_red(slice, width, height)
                }
            }
        })
    }

    pub fn get_rg_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_mut().map(|image| {
            match image {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    let slice = image.data.as_slice();
                    let width = image.width;
                    let height = image.height;
                    Self::rgba_to_rg(slice, width, height)
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _, _) => {
                    let size = image.compute_byte_size();
                    let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                    let width = image.width() as usize;
                    let height = image.height() as usize;
                    Self::rgba_to_rg(slice, width, height)
                }
            }
        })
    }

    pub fn get_rgba_integers(&self) -> Option<Vec<u32>> {
        self.0.lock().image.as_mut().map(|image| {
            match image {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    let slice = image.data.as_slice();
                    let width = image.width;
                    let height = image.height;
                    Self::rgba_to_rgba_integer(slice, width, height)
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _, _) => {
                    let size = image.compute_byte_size();
                    let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                    let width = image.width() as usize;
                    let height = image.height() as usize;
                    Self::rgba_to_rgba_integer(slice, width, height)
                }
            }
        })
    }

    pub fn get_rgb_integers(&self) -> Option<Vec<u32>> {
        self.0.lock().image.as_mut().map(|image| {
            match image {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    let slice = image.data.as_slice();
                    let width = image.width;
                    let height = image.height;
                    Self::rgba_to_rgb_integer(slice, width, height)
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _, _) => {
                    let size = image.compute_byte_size();
                    let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                    let width = image.width() as usize;
                    let height = image.height() as usize;
                    Self::rgba_to_rgb_integer(slice, width, height)
                }
            }
        })
    }

    pub fn get_red_integers(&self) -> Option<Vec<u32>> {
        self.0.lock().image.as_mut().map(|image| {
            match image {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    let slice = image.data.as_slice();
                    let width = image.width;
                    let height = image.height;
                    Self::rgba_to_red_integer(slice, width, height)
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _, _) => {
                    let size = image.compute_byte_size();
                    let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                    let width = image.width() as usize;
                    let height = image.height() as usize;
                    Self::rgba_to_red_integer(slice, width, height)
                }
            }
        })
    }

    pub fn get_rg_integers(&self) -> Option<Vec<u32>> {
        self.0.lock().image.as_mut().map(|image| {
            match image {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    let slice = image.data.as_slice();
                    let width = image.width;
                    let height = image.height;
                    Self::rgba_to_rg_integer(slice, width, height)
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _, _) => {
                    let size = image.compute_byte_size();
                    let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                    let width = image.width() as usize;
                    let height = image.height() as usize;
                    Self::rgba_to_rg_integer(slice, width, height)
                }
            }
        })
    }


    pub fn len(&self) -> usize {
        self.0
            .lock()
            .image
            .as_ref()
            .map(|d| d.length())
            .unwrap_or_default()
    }

    pub fn copy_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_ref().map(|d| {
            match d {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    image.data.clone()
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _, _) => {
                    let size = image.compute_byte_size();
                    let pixmap = image.pixmap();
                    let data = unsafe { std::slice::from_raw_parts(pixmap.addr() as *const u8, size) };
                    data.to_vec()
                }
            }
        })
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

    pub fn rgba_to_red(data: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0_u8; width * height];
        if data.len() < width * height * 4 {
            return buf;
        }
        for i in 0..(width * height) {
            let red = data[i * 4];
            buf[i] = red;
        }
        buf
    }

    pub fn rgb_to_red(data: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0_u8; width * height];
        if data.len() < width * height * 3 {
            return buf;
        }
        for i in 0..(width * height) {
            let red = data[i * 3];
            buf[i] = red;
        }
        buf
    }

    pub fn rgba_to_red_integer(data: &[u8], width: usize, height: usize) -> Vec<u32> {
        let mut buf: Vec<u32> = vec![0_u32; width * height];
        if data.len() < width * height * 4 {
            return buf;
        }
        for i in 0..(width * height) {
            let red = data[i * 4] as u32;
            buf[i] = red;
        }
        buf
    }

    pub fn rgb_to_red_integer(data: &[u8], width: usize, height: usize) -> Vec<u32> {
        let mut buf: Vec<u32> = vec![0_u32; width * height];
        if data.len() < width * height * 3 {
            return buf;
        }
        for i in 0..(width * height) {
            let red = data[i * 3] as u32;
            buf[i] = red;
        }
        buf
    }

    pub fn rgba_to_rgba_integer(data: &[u8], width: usize, height: usize) -> Vec<u32> {
        let mut buf: Vec<u32> = vec![0_u32; width * height];
        if data.len() < width * height * 4 {
            return buf;
        }
        for i in 0..(width * height) {
            let r = data[i * 4] as u32;
            let g = data[i * 4 + 1] as u32;
            let b = data[i * 4 + 2] as u32;
            let a = data[i * 4 + 3] as u32;

            buf[i] = (r << 24) | (g << 16) | (b << 8) | a;
        }
        buf
    }

    pub fn rgba_to_rgb_integer(data: &[u8], width: usize, height: usize) -> Vec<u32> {
        let mut buf: Vec<u32> = vec![0_u32; width * height];
        if data.len() < width * height * 4 {
            return buf;
        }
        for i in 0..(width * height) {
            let r = data[i * 4] as u32;
            let g = data[i * 4 + 1] as u32;
            let b = data[i * 4 + 2] as u32;

            buf[i] = (r << 16) | (g << 8) | b;
        }
        buf
    }

    pub fn rgba_to_rg(data: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0_u8; width * height * 2];

        if data.len() < width * height * 4 {
            return buf;
        }

        for i in 0..(width * height) {
            let red = data[i * 4 + 0];
            let green = data[i * 4 + 1];

            buf[i * 2 + 0] = red;
            buf[i * 2 + 1] = green;
        }

        buf
    }

    pub fn rgb_to_rg(data: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0_u8; width * height * 2];

        if data.len() < width * height * 3 {
            return buf;
        }

        for i in 0..(width * height) {
            let red = data[i * 3 + 0];
            let green = data[i * 3 + 1];

            buf[i * 2 + 0] = red;
            buf[i * 2 + 1] = green;
        }

        buf
    }

    pub fn rgba_to_rg_integer(data: &[u8], width: usize, height: usize) -> Vec<u32> {
        let num_pixels = width * height;
        let mut buf: Vec<u32> = Vec::with_capacity(num_pixels);

        if data.len() < width * height * 4 {
            return buf;
        }

        for i in 0..num_pixels {
            let red = data[i * 4 + 0] as u32;
            let green = data[i * 4 + 1] as u32;

            let rg_packed = (red << 16) | green;
            buf.push(rg_packed);
        }

        buf
    }
}
