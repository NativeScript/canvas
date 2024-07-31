use std::os::raw::c_int;
use std::sync::Arc;

use bytes::BytesMut;

#[derive(Debug, Clone)]
struct ImageDataInner {
    pub(crate) data: *mut BytesMut,
    pub(crate) width: c_int,
    pub(crate) height: c_int,
    pub(crate) scale: f32,
    len: usize,
    ref_count: Arc<()>,
}

unsafe impl Send for ImageDataInner {}

#[derive(Debug, Clone)]
pub struct ImageData(ImageDataInner);

impl Drop for ImageDataInner {
    fn drop(&mut self) {
        let ref_count = Arc::strong_count(&self.ref_count);

        if ref_count == 1 {
            let _ = unsafe { Box::from_raw(self.data) };
        }
    }
}

impl ImageData {
    pub fn new(width: c_int, height: c_int) -> Self {
        let len = (width * height * 4) as usize;
        let data = BytesMut::zeroed(len);
        Self(ImageDataInner {
            width,
            height,
            data: Box::into_raw(Box::new(data)),
            scale: 1.,
            ref_count: Arc::default(),
            len,
        })
    }

    pub fn new_with_data(width: c_int, height: c_int, data: &[u8]) -> Self {
        let len = data.len();
        let data = BytesMut::from(data);
        Self(ImageDataInner {
            width,
            height,
            data: Box::into_raw(Box::new(data)),
            scale: 1.,
            ref_count: Arc::default(),
            len,
        })
    }

    pub fn new_with_buffer(width: c_int, height: c_int, data: BytesMut) -> Self {
        let len = data.len();
        Self(ImageDataInner {
            width,
            height,
            data: Box::into_raw(Box::new(data)),
            scale: 1.,
            ref_count: Arc::default(),
            len,
        })
    }

    pub fn width(&self) -> i32 {
        (self.0.width as f32 / self.0.scale) as i32
    }

    pub fn height(&self) -> i32 {
        (self.0.height as f32 / self.0.scale) as i32
    }

    pub fn data(&self) -> &[u8] {
        unsafe { (&*self.0.data).as_ref() }
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        unsafe { (&mut *self.0.data).as_mut() }
    }

    pub fn data_len(&self) -> usize {
        self.0.len
    }

    pub fn data_raw(&mut self) -> *mut u8 {
        unsafe { (&mut *self.0.data).as_mut_ptr() }
    }
}

impl AsRef<[u8]> for ImageData {
    fn as_ref(&self) -> &[u8] {
        self.data()
    }
}

impl AsMut<[u8]> for ImageData {
    fn as_mut(&mut self) -> &mut [u8] {
        self.data_mut()
    }
}


impl From<&ImageData> for ImageData {
    fn from(data: &ImageData) -> Self {
        Self::new(data.0.width, data.0.height)
    }
}
