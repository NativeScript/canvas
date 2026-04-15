use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::fmt::{Debug, Formatter};
use std::io::{BufRead, Read, Seek};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr::null;
use std::sync::Arc;

enum CanvasImage {
    #[cfg(not(feature = "2d"))]
    Stb(stb_image::image::Image<u8>),
    #[cfg(feature = "2d")]
    Skia(skia_safe::Bitmap, Vec<u8>),
}

#[cfg(test)]
mod simd_tests {
    use super::*;

    #[test]
    fn simd_rgba_to_bgra() {
        let mut data: Vec<u8> = vec![
            1, 2, 3, 4, // pixel0 r,g,b,a
            5, 6, 7, 8, // pixel1
            9, 10, 11, 12, // pixel2
            13, 14, 15, 16, // pixel3
            17, 18, 19, 20, // pixel4
            21, 22, 23, 24, // pixel5
            25, 26, 27, 28, // pixel6
            29, 30, 31, 32, // pixel7
        ];

        let expected = vec![
            3, 2, 1, 4, 7, 6, 5, 8, 11, 10, 9, 12, 15, 14, 13, 16, 19, 18, 17, 20, 23, 22, 21, 24,
            27, 26, 25, 28, 31, 30, 29, 32,
        ];

        ImageAsset::rgba_to_bgra_in_place(&mut data);
        assert_eq!(&data[..], &expected[..]);
    }
}

impl CanvasImage {
    pub fn with_bytes_dimension<F>(&self, f: F)
    where
        F: FnOnce(&[u8], (u32, u32)),
    {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => f(
                image.data.as_slice(),
                (image.width as u32, image.height as u32),
            ),
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, data) => {
                let dimensions = (image.width() as u32, image.height() as u32);
                f(data.as_slice(), dimensions)
            }
        }
    }

    pub fn with_bytes<F>(&self, f: F)
    where
        F: FnOnce(&[u8]),
    {
        self.with_bytes_dimension(|bytes, _| f(bytes))
    }

    pub fn with_bytes_mut_dimension<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [u8], (u32, u32)),
    {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => f(
                image.data.as_mut_slice(),
                (image.width as u32, image.height as u32),
            ),
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, data) => {
                let dimensions = (image.width() as u32, image.height() as u32);
                f(data.as_mut_slice(), dimensions)
            }
        }
    }

    pub fn with_bytes_mut<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [u8]),
    {
        self.with_bytes_mut_dimension(|bytes, _| f(bytes))
    }

    #[cfg(feature = "2d")]
    pub fn new(info: &skia_safe::ImageInfo, mut data: Vec<u8>) -> Self {
        let mut bitmap = skia_safe::Bitmap::new();
        let min_row_bytes = info.min_row_bytes();
        // SAFETY: `data` outlives the Bitmap because both are stored in the same
        // enum variant. We install the raw pointer so Skia can read pixels without
        // an extra copy. `data` must never be resized or dropped while `bitmap` lives.
        let data_ptr = data.as_mut_ptr() as *mut c_void;
        unsafe { bitmap.install_pixels(info, data_ptr, min_row_bytes) };
        Self::Skia(bitmap, data)
    }

    #[cfg(not(feature = "2d"))]
    pub fn new(width: usize, height: usize, depth: usize, data: Vec<u8>) -> Self {
        let image = stb_image::image::Image::new(width, height, depth, data);
        Self::Stb(image)
    }

    pub fn width(&self) -> u32 {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => image.width as u32,
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => image.width() as u32,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => image.height as u32,
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => image.height() as u32,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => (image.width as u32, image.height as u32),

            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let dimensions = image.dimensions();
                (dimensions.width as u32, dimensions.height as u32)
            }
        }
    }

    pub fn length(&self) -> usize {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => image.data.len(),
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => image.compute_byte_size(),
        }
    }
}

impl Clone for CanvasImage {
    fn clone(&self) -> Self {
        match self {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let image = stb_image::image::Image::new(
                    image.width,
                    image.height,
                    image.depth,
                    image.data.clone(),
                );
                Self::Stb(image)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, data) => {
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
            CanvasImage::Skia(image, _) => {
                let mut d = f.debug_struct("Image");
                d.field("width", &image.width());
                d.field("height", &image.height());
                d.finish()
            }
        }
    }
}

struct ImageAssetInner {
    image: Option<CanvasImage>,
    /// Lazily-created raster `Image` wrapping `self.image`.
    ///
    /// By caching the `Image` here and returning the same object each time, the
    /// unique-id stays stable and Skia's internal cache becomes effective:
    /// the first `drawImage` uploads pixels to the GPU; every subsequent call
    /// is a pure GPU→GPU blit at zero CPU cost.
    #[cfg(feature = "2d")]
    raster_image_cache: Option<skia_safe::Image>,
    error: Cow<'static, str>,
    has_alpha: bool,
}

impl Debug for ImageAssetInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("ImageAssetInner");
        s.field("image", &self.image);
        s.field("error", &self.error);
        s.field("has_alpha", &self.has_alpha);
        s.finish()
    }
}

impl Clone for ImageAssetInner {
    fn clone(&self) -> Self {
        Self {
            image: self.image.clone(),
            // Don't clone the cache – it will be rebuilt lazily on next access.
            #[cfg(feature = "2d")]
            raster_image_cache: None,
            error: self.error.clone(),
            has_alpha: self.has_alpha,
        }
    }
}

impl Default for ImageAssetInner {
    fn default() -> Self {
        Self {
            image: None,
            #[cfg(feature = "2d")]
            raster_image_cache: None,
            error: Cow::default(),
            has_alpha: false,
        }
    }
}

unsafe impl Send for ImageAssetInner {}

#[derive(Clone, Debug, Default)]
pub struct ImageAsset(Arc<parking_lot::Mutex<ImageAssetInner>>);

unsafe impl Send for ImageAsset {}

impl ImageAsset {
    pub fn with_bytes_dimension<F>(&self, f: F)
    where
        F: FnOnce(&[u8], (u32, u32)),
    {
        let lock = self.0.lock();
        match lock.image.as_ref() {
            None => f(&[], (0, 0)),
            Some(image) => image.with_bytes_dimension(f),
        }
    }

    pub fn with_bytes<F>(&self, f: F)
    where
        F: FnOnce(&[u8]),
    {
        self.with_bytes_dimension(|bytes, _| f(bytes))
    }

    pub fn with_bytes_mut_dimension<F>(&self, f: F)
    where
        F: FnOnce(&mut [u8], (u32, u32)),
    {
        let mut lock = self.0.lock();
        // Pixel data is about to be mutated; invalidate the cached Image so it is
        // rebuilt from the updated bytes on the next with_skia_image call.
        #[cfg(feature = "2d")]
        {
            lock.raster_image_cache = None;
        }
        match lock.image.as_mut() {
            None => f(&mut [], (0, 0)),
            Some(image) => image.with_bytes_mut_dimension(f),
        }
    }

    pub fn with_bytes_mut<F>(&self, f: F)
    where
        F: FnOnce(&mut [u8]),
    {
        self.with_bytes_mut_dimension(|bytes, _| f(bytes))
    }

    pub fn with_skia_bitmap<F>(&self, f: F)
    where
        F: FnOnce(Option<&skia_safe::Bitmap>),
    {
        let lock = self.0.lock();
        match lock.image.as_ref() {
            None => f(None),
            Some(image) => match image {
                #[cfg(not(feature = "2d"))]
                CanvasImage::Stb(image) => {
                    let info = skia_safe::ImageInfo::new(
                        (image.width as i32, image.height as i32),
                        skia_safe::ColorType::RGBA8888,
                        skia_safe::AlphaType::Unpremul,
                        None,
                    );
                    let mut bm = skia_safe::Bitmap::new();
                    let success = unsafe {
                        bm.install_pixels(
                            &info,
                            image.data.as_ptr() as *mut c_void,
                            info.min_row_bytes(),
                        )
                    };
                    if success {
                        f(Some(&bm))
                    } else {
                        f(None)
                    }
                }
                #[cfg(feature = "2d")]
                CanvasImage::Skia(image, _) => f(Some(image)),
            },
        }
    }

    pub fn with_skia_image<F>(&self, f: F)
    where
        F: FnOnce(Option<&skia_safe::Image>),
    {
        let mut lock = self.0.lock();

        #[cfg(feature = "2d")]
        {
            if lock.raster_image_cache.is_none() {
                let new_img = lock
                    .image
                    .as_ref()
                    .and_then(|ci| {
                        let CanvasImage::Skia(bitmap, _) = ci;
                        skia_safe::images::raster_from_bitmap(bitmap)
                    });
                lock.raster_image_cache = new_img;
            }
            f(lock.raster_image_cache.as_ref());
            return;
        }

        #[cfg(not(feature = "2d"))]
        match lock.image.as_ref() {
            None => f(None),
            Some(CanvasImage::Stb(image)) => {
                let info = skia_safe::ImageInfo::new(
                    (image.width as i32, image.height as i32),
                    skia_safe::ColorType::RGBA8888,
                    skia_safe::AlphaType::Unpremul,
                    None,
                );
                let data = unsafe { skia_safe::Data::new_bytes(image.data.as_slice()) };
                let img = skia_safe::images::raster_from_data(&info, data, info.min_row_bytes());
                f(img.as_ref())
            }
        }
    }

    pub fn with_skia_image_bytes<F>(&self, f: F)
    where
        F: FnOnce(Option<&skia_safe::Image>, Option<((u32, u32), &[u8])>),
    {
        let mut lock = self.0.lock();

        #[cfg(feature = "2d")]
        {
            if lock.raster_image_cache.is_none() {
                let new_img = lock
                    .image
                    .as_ref()
                    .and_then(|ci| {
                        let CanvasImage::Skia(bitmap, _) = ci;
                        skia_safe::images::raster_from_bitmap(bitmap)
                    });
                lock.raster_image_cache = new_img;
            }

            match lock.image.as_ref() {
                None => f(None, None),
                Some(CanvasImage::Skia(bitmap, bytes)) => {
                    let dims = bitmap.dimensions();
                    f(
                        lock.raster_image_cache.as_ref(),
                        Some(((dims.width as u32, dims.height as u32), bytes)),
                    )
                }
            }
            return;
        }

        #[cfg(not(feature = "2d"))]
        match lock.image.as_ref() {
            None => f(None, None),
            Some(CanvasImage::Stb(image)) => {
                let info = skia_safe::ImageInfo::new(
                    (image.width as i32, image.height as i32),
                    skia_safe::ColorType::RGBA8888,
                    skia_safe::AlphaType::Unpremul,
                    None,
                );
                let dimensions = (image.width as u32, image.height as u32);
                let slice = image.data.as_slice();
                let data = unsafe { skia_safe::Data::new_bytes(slice) };
                let img = skia_safe::images::raster_from_data(&info, data, info.min_row_bytes());
                f(img.as_ref(), Some((dimensions, slice)))
            }
        }
    }

    pub fn close(&self) {
        let mut lock = self.0.lock();
        lock.image = None;
        #[cfg(feature = "2d")]
        {
            lock.raster_image_cache = None;
        }
    }

    #[cfg(feature = "2d")]
    pub fn raster_image(&self) -> Option<skia_safe::Image> {
        let mut lock = self.0.lock();
        if lock.raster_image_cache.is_none() {
            let new_img = lock.image.as_ref().and_then(|ci| {
                let CanvasImage::Skia(bitmap, _) = ci;
                skia_safe::images::raster_from_bitmap(bitmap)
            });
            lock.raster_image_cache = new_img;
        }
        // Clone is cheap: skia_safe::Image is Arc-counted internally.
        lock.raster_image_cache.clone()
    }

    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }

    #[cfg(feature = "2d")]
    pub fn from_raw_bytes(
        width: usize,
        height: usize,
        color_type: skia_safe::ColorType,
        alpha_type: skia_safe::AlphaType,
        data: Vec<u8>,
    ) -> Self {
        let info = skia_safe::ImageInfo::new(
            skia_safe::ISize::new(width as i32, height as i32),
            color_type,
            alpha_type,
            None,
        );

        let has_alpha = info.is_opaque();
        let inner = ImageAssetInner {
            image: Some(CanvasImage::new(&info, data)),
            #[cfg(feature = "2d")]
            raster_image_cache: None,
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
            #[cfg(feature = "2d")]
            raster_image_cache: None,
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
        lock.raster_image_cache = None;
        let data = unsafe { skia_safe::Data::new_bytes(buf) };

        match skia_safe::images::deferred_from_encoded_data(data, None) {
            None => {
                lock.error = Cow::Borrowed("Failed to decode image");
                false
            }
            Some(image) => {
                /*

                                // todo

                                use core_foundation::base::TCFType;
                use core_foundation::dictionary::CFDictionary;
                use core_foundation::string::CFString;

                                  unsafe {
                                    let width = core_foundation::number::CFNumber::from(image.width());
                                    let height = core_foundation::number::CFNumber::from(image.height());
                                    let bytes_per_element = core_foundation::number::CFNumber::from(4);
                                    let is_global = core_foundation::number::CFNumber::from(1);

                                    let surface_width = CFString::wrap_under_create_rule(crate::io_surface::kIOSurfaceWidth);
                                    let surface_height = CFString::wrap_under_create_rule(crate::io_surface::kIOSurfaceHeight);
                                    let surface_bytes_per_element = CFString::wrap_under_create_rule(crate::io_surface::kIOSurfaceBytesPerElement);
                                    let surface_is_global = CFString::wrap_under_create_rule(crate::io_surface::kIOSurfaceIsGlobal);

                                    let dict = CFDictionary::from_CFType_pairs(&[
                                        (
                                            surface_width, width.as_CFType()
                                        ),
                                        (
                                            surface_height, height.as_CFType()
                                        ),
                                        (
                                            surface_bytes_per_element, bytes_per_element.as_CFType()
                                        ),
                                        (
                                            surface_is_global, is_global.as_CFType()
                                        )
                                    ]);

                                    let surface = crate::io_surface::new(&dict);
                                    surface.upload(buf);

                                }

                                 */
                let info = skia_safe::ImageInfo::new(
                    image.dimensions(),
                    skia_safe::ColorType::RGBA8888,
                    skia_safe::AlphaType::Unpremul,
                    None,
                );
                let row_bytes = info.min_row_bytes() as usize;
                let size = info.height() as usize * row_bytes;
                let mut buffer = vec![0u8; size];
                let success = image.read_pixels(
                    &info,
                    buffer.as_mut_slice(),
                    row_bytes,
                    (0i32, 0i32),
                    skia_safe::image::CachingHint::Allow,
                );
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
            stb_image::image::LoadResult::Error(e) => {
                let error = e.to_string();
                lock.error = Cow::Owned(error);
                false
            }
            stb_image::image::LoadResult::ImageU8(image) => {
                lock.image = Some(CanvasImage::Stb(image));
                true
            }
            stb_image::image::LoadResult::ImageF32(_) => {
                lock.image = None;
                false
            }
        }
    }

    pub fn load_from_bytes_int(&self, buf: &mut [i8]) -> bool {
        self.load_from_bytes(unsafe { std::mem::transmute::<&mut [i8], &[u8]>(buf) })
    }

    #[cfg(feature = "2d")]
    pub fn load_from_raw_bytes_rgba(&self, width: u32, height: u32, data: Vec<u8>) -> bool {
        let info = skia_safe::ImageInfo::new(
            (width as i32, height as i32),
            skia_safe::ColorType::RGBA8888,
            skia_safe::AlphaType::Unpremul,
            None,
        );
        self.load_from_raw_bytes(&info, data)
    }

    #[cfg(feature = "2d")]
    pub fn load_from_raw_bytes(&self, info: &skia_safe::ImageInfo, data: Vec<u8>) -> bool {
        let has_alpha = !info.is_opaque();
        let mut lock = self.0.lock();
        lock.error = Cow::default();
        lock.image = Some(CanvasImage::new(info, data));
        lock.raster_image_cache = None;
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
        // Normalize stored image data to 4 channels (RGBA) so later GPU
        // uploads can be zero-copy (no per-pixel format conversion).
        let normalized: Vec<u8> = match depth {
            4 => data,
            3 => {
                // RGB -> RGBA (append 0xFF alpha)
                let num_pixels = width * height;
                let mut out = Vec::with_capacity(num_pixels * 4);
                let mut i = 0usize;
                let src = data.as_slice();
                out.resize(num_pixels * 4, 0u8);
                let dst = out.as_mut_slice();
                while i + 7 < num_pixels {
                    let s0 = i * 3;
                    let d0 = i * 4;

                    dst[d0] = src[s0];
                    dst[d0 + 1] = src[s0 + 1];
                    dst[d0 + 2] = src[s0 + 2];
                    dst[d0 + 3] = 255u8;

                    let s1 = s0 + 3;
                    let d1 = d0 + 4;
                    dst[d1] = src[s1];
                    dst[d1 + 1] = src[s1 + 1];
                    dst[d1 + 2] = src[s1 + 2];
                    dst[d1 + 3] = 255u8;

                    let s2 = s1 + 3;
                    let d2 = d1 + 4;
                    dst[d2] = src[s2];
                    dst[d2 + 1] = src[s2 + 1];
                    dst[d2 + 2] = src[s2 + 2];
                    dst[d2 + 3] = 255u8;

                    let s3 = s2 + 3;
                    let d3 = d2 + 4;
                    dst[d3] = src[s3];
                    dst[d3 + 1] = src[s3 + 1];
                    dst[d3 + 2] = src[s3 + 2];
                    dst[d3 + 3] = 255u8;

                    let s4 = s3 + 3;
                    let d4 = d3 + 4;
                    dst[d4] = src[s4];
                    dst[d4 + 1] = src[s4 + 1];
                    dst[d4 + 2] = src[s4 + 2];
                    dst[d4 + 3] = 255u8;

                    let s5 = s4 + 3;
                    let d5 = d4 + 4;
                    dst[d5] = src[s5];
                    dst[d5 + 1] = src[s5 + 1];
                    dst[d5 + 2] = src[s5 + 2];
                    dst[d5 + 3] = 255u8;

                    let s6 = s5 + 3;
                    let d6 = d5 + 4;
                    dst[d6] = src[s6];
                    dst[d6 + 1] = src[s6 + 1];
                    dst[d6 + 2] = src[s6 + 2];
                    dst[d6 + 3] = 255u8;

                    let s7 = s6 + 3;
                    let d7 = d6 + 4;
                    dst[d7] = src[s7];
                    dst[d7 + 1] = src[s7 + 1];
                    dst[d7 + 2] = src[s7 + 2];
                    dst[d7 + 3] = 255u8;

                    i += 8;
                }

                while i < num_pixels {
                    let s = i * 3;
                    let d = i * 4;
                    dst[d] = src[s];
                    dst[d + 1] = src[s + 1];
                    dst[d + 2] = src[s + 2];
                    dst[d + 3] = 255u8;
                    i += 1;
                }

                out
            }
            2 => {
                // Luminance + Alpha -> replicate luminance to RGB, keep alpha
                let num_pixels = width * height;
                let mut out = Vec::with_capacity(num_pixels * 4);
                for chunk in data.chunks_exact(2) {
                    let l = chunk[0];
                    let a = chunk[1];
                    out.push(l);
                    out.push(l);
                    out.push(l);
                    out.push(a);
                }
                out
            }
            1 => {
                // Grayscale -> replicate to RGB, alpha = 0xFF
                let num_pixels = width * height;
                let mut out = Vec::with_capacity(num_pixels * 4);
                for &g in data.iter() {
                    out.push(g);
                    out.push(g);
                    out.push(g);
                    out.push(255u8);
                }
                out
            }
            _ => {
                // Unknown depth: attempt best-effort by treating as 4-channel blob
                data
            }
        };

        let image = stb_image::image::Image::new(width, height, 4usize, normalized);

        let mut lock = self.0.lock();
        lock.error = Cow::default();
        lock.image = Some(CanvasImage::Stb(image));
        // has_alpha is true if original depth included alpha (depth == 2 or 4)
        lock.has_alpha = depth == 2 || depth == 4;

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
        self.0.lock().image.as_mut().map(|image| match image {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let slice = image.data.as_slice();
                let width = image.width;
                let height = image.height;
                Self::rgba_to_rgb(slice, width, height)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let size = image.compute_byte_size();
                let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                let width = image.width() as usize;
                let height = image.height() as usize;
                Self::rgba_to_rgb(slice, width, height)
            }
        })
    }

    pub fn get_luminance_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_mut().map(|image| match image {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let slice = image.data.as_slice();
                let width = image.width;
                let height = image.height;
                Self::rgba_to_luminance(slice, width, height)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let size = image.compute_byte_size();
                let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                let width = image.width() as usize;
                let height = image.height() as usize;
                Self::rgba_to_luminance(slice, width, height)
            }
        })
    }

    pub fn get_luminance_alpha_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_mut().map(|image| match image {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let slice = image.data.as_slice();
                let width = image.width;
                let height = image.height;
                Self::rgba_to_luminance_alpha(slice, width, height)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let size = image.compute_byte_size();
                let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                let width = image.width() as usize;
                let height = image.height() as usize;
                Self::rgba_to_luminance_alpha(slice, width, height)
            }
        })
    }

    pub fn get_alpha_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_mut().map(|image| match image {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let slice = image.data.as_slice();
                let width = image.width;
                let height = image.height;
                Self::rgba_to_alpha(slice, width, height)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let size = image.compute_byte_size();
                let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                let width = image.width() as usize;
                let height = image.height() as usize;
                Self::rgba_to_alpha(slice, width, height)
            }
        })
    }

    pub fn get_red_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_mut().map(|image| match image {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let slice = image.data.as_slice();
                let width = image.width;
                let height = image.height;
                Self::rgba_to_red(slice, width, height)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let size = image.compute_byte_size();
                let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                let width = image.width() as usize;
                let height = image.height() as usize;
                Self::rgba_to_red(slice, width, height)
            }
        })
    }

    pub fn get_rg_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_mut().map(|image| match image {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let slice = image.data.as_slice();
                let width = image.width;
                let height = image.height;
                Self::rgba_to_rg(slice, width, height)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let size = image.compute_byte_size();
                let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                let width = image.width() as usize;
                let height = image.height() as usize;
                Self::rgba_to_rg(slice, width, height)
            }
        })
    }

    pub fn get_rgba_integers(&self) -> Option<Vec<u32>> {
        self.0.lock().image.as_mut().map(|image| match image {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let slice = image.data.as_slice();
                let width = image.width;
                let height = image.height;
                Self::rgba_to_rgba_integer(slice, width, height)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let size = image.compute_byte_size();
                let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                let width = image.width() as usize;
                let height = image.height() as usize;
                Self::rgba_to_rgba_integer(slice, width, height)
            }
        })
    }

    pub fn get_rgb_integers(&self) -> Option<Vec<u32>> {
        self.0.lock().image.as_mut().map(|image| match image {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let slice = image.data.as_slice();
                let width = image.width;
                let height = image.height;
                Self::rgba_to_rgb_integer(slice, width, height)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let size = image.compute_byte_size();
                let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                let width = image.width() as usize;
                let height = image.height() as usize;
                Self::rgba_to_rgb_integer(slice, width, height)
            }
        })
    }

    pub fn get_red_integers(&self) -> Option<Vec<u32>> {
        self.0.lock().image.as_mut().map(|image| match image {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let slice = image.data.as_slice();
                let width = image.width;
                let height = image.height;
                Self::rgba_to_red_integer(slice, width, height)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let size = image.compute_byte_size();
                let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                let width = image.width() as usize;
                let height = image.height() as usize;
                Self::rgba_to_red_integer(slice, width, height)
            }
        })
    }

    pub fn get_rg_integers(&self) -> Option<Vec<u32>> {
        self.0.lock().image.as_mut().map(|image| match image {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => {
                let slice = image.data.as_slice();
                let width = image.width;
                let height = image.height;
                Self::rgba_to_rg_integer(slice, width, height)
            }
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let size = image.compute_byte_size();
                let slice = unsafe { std::slice::from_raw_parts(image.pixels() as *mut u8, size) };
                let width = image.width() as usize;
                let height = image.height() as usize;
                Self::rgba_to_rg_integer(slice, width, height)
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

    pub fn is_empty(&self) -> bool {
        self.0.lock().image.is_none()
    }

    pub fn copy_bytes(&self) -> Option<Vec<u8>> {
        self.0.lock().image.as_ref().map(|d| match d {
            #[cfg(not(feature = "2d"))]
            CanvasImage::Stb(image) => image.data.clone(),
            #[cfg(feature = "2d")]
            CanvasImage::Skia(image, _) => {
                let size = image.compute_byte_size();
                let pixmap = image.pixmap();
                let data = unsafe { std::slice::from_raw_parts(pixmap.addr() as *const u8, size) };
                data.to_vec()
            }
        })
    }

    pub fn bgra_to_rgba_in_place(data: &mut [u8]) {
        for chunk in data.chunks_exact_mut(4) {
            chunk.swap(0, 2);
        }
    }

    pub fn rgba_to_bgra_in_place(data: &mut [u8]) {
        // Portable, stable accelerated swizzle using 32-bit word operations.
        // For each pixel interpret 4 bytes as a little-endian u32, then
        // compute: result = (v & 0xFF00FF00) | ((v & 0x000000FF) << 16) |
        // ((v & 0x00FF0000) >> 16), which swaps the low and mid bytes
        // corresponding to R and B in little-endian layout.
        let len = data.len() / 4;
        let mut i = 0usize;

        // Process per-pixel with u32 ops; LLVM can auto-vectorize this loop.
        while i < len {
            let base = i * 4;
            // read u32 in little-endian order
            let v =
                u32::from_le_bytes([data[base], data[base + 1], data[base + 2], data[base + 3]]);

            let res =
                (v & 0xFF00_FF00u32) | ((v & 0x0000_00FFu32) << 16) | ((v & 0x00FF_0000u32) >> 16);

            let out = res.to_le_bytes();
            data[base..base + 4].copy_from_slice(&out);

            i += 1;
        }
    }

    pub fn rgb565_to_rgba8888(data: &[u8]) -> Vec<u8> {
        let mut rgba_data = Vec::with_capacity(data.len() * 2);
        for chunk in data.chunks_exact(2) {
            // RGB565 little-endian: pixel = chunk[0] | (chunk[1] << 8)
            // Layout: RRRRRGGG GGGBBBBB -> pixel bits [15:11]=R, [10:5]=G, [4:0]=B
            let pixel = (chunk[0] as u16) | ((chunk[1] as u16) << 8);
            let r5 = ((pixel >> 11) & 0x1F) as u8;
            let g6 = ((pixel >> 5) & 0x3F) as u8;
            let b5 = (pixel & 0x1F) as u8;

            // Scale to 8-bit: replicate high bits into low bits for full range
            let r = (r5 << 3) | (r5 >> 2);
            let g = (g6 << 2) | (g6 >> 4);
            let b = (b5 << 3) | (b5 >> 2);

            rgba_data.extend_from_slice(&[r, g, b, 255]);
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
            let red = data[i * 4];
            let green = data[i * 4 + 1];

            buf[i * 2] = red;
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
            let red = data[i * 3];
            let green = data[i * 3 + 1];

            buf[i * 2] = red;
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
            let red = data[i * 4] as u32;
            let green = data[i * 4 + 1] as u32;

            let rg_packed = (red << 16) | green;
            buf.push(rg_packed);
        }

        buf
    }
}
