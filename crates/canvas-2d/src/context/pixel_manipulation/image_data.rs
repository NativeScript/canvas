use bytes::BytesMut;
use parking_lot::Mutex;
use std::os::raw::c_int;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct ImageDataInner {
    pub(crate) data: Arc<Mutex<BytesMut>>,
    pub(crate) width: c_int,
    pub(crate) height: c_int,
    pub(crate) scale: f32,
}

#[derive(Debug, Clone)]
pub struct ImageData(ImageDataInner);

impl ImageData {
    pub fn new(width: c_int, height: c_int) -> Self {
        let data = BytesMut::zeroed((width * height * 4) as usize);
        Self(ImageDataInner {
            width,
            height,
            data: Arc::new(Mutex::new(data)),
            scale: 1.,
        })
    }

    pub fn new_with_data(width: c_int, height: c_int, data: &[u8]) -> Self {
        let data = BytesMut::from(data);
        Self(ImageDataInner {
            width,
            height,
            data: Arc::new(Mutex::new(data)),
            scale: 1.,
        })
    }

    pub fn new_with_buffer(width: c_int, height: c_int, data: BytesMut) -> Self {
        Self(ImageDataInner {
            width,
            height,
            data: Arc::new(Mutex::new(data)),
            scale: 1.,
        })
    }

    pub fn width(&self) -> i32 {
        (self.0.width as f32 / self.0.scale) as i32
    }

    pub fn height(&self) -> i32 {
        (self.0.height as f32 / self.0.scale) as i32
    }

    pub fn data(&self) -> &[u8] {
        // ..
        let (ptr, len) = {
            let lock = self.0.data.lock();
            (lock.as_ptr(), lock.len())
        };
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        // ..
        let (ptr, len) = {
            let mut lock = self.0.data.lock();
            (lock.as_mut_ptr(), lock.len())
        };
        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
    }

    pub fn data_len(&self) -> usize {
        self.0.data.lock().len()
    }

    pub unsafe fn data_raw(&mut self) -> *mut u8 {
        self.0.data.lock().as_mut_ptr()
    }

    pub fn bytes_mut(&self) -> Arc<Mutex<BytesMut>> {
        self.0.data.clone()
    }
}

impl From<&ImageData> for ImageData {
    fn from(data: &ImageData) -> Self {
        Self::new(data.0.width, data.0.height)
    }
}
