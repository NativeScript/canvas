use std::os::raw::c_int;
use std::sync::Arc;

#[derive(Debug)]
struct ImageDataInner {
    pub(crate) data: *mut u8,
    pub(crate) data_len: usize,
    pub(crate) width: c_int,
    pub(crate) height: c_int,
    pub(crate) scale: f32,
}

#[derive(Debug)]
pub struct ImageData(Arc<ImageDataInner>);

impl Clone for ImageData {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = Arc::clone(&source.0)
    }
}

impl ImageData {
    fn to_raw(data: Vec<u8>) -> (*mut u8, usize) {
        let mut slice = data.into_boxed_slice();
        let ptr = slice.as_mut_ptr();
        let len = slice.len();
        Box::into_raw(slice);
        (ptr, len)
    }

    pub fn new(width: c_int, height: c_int) -> Self {
        let data = vec![0u8; (width * height * 4) as usize];
        let (data, data_len) = Self::to_raw(data);
        Self(Arc::new(ImageDataInner {
            width,
            height,
            data,
            data_len,
            scale: 1.,
        }))
    }

    pub fn new_with_data(width: c_int, height: c_int, data: Vec<u8>) -> Self {
        let (data, data_len) = Self::to_raw(data);
        Self(Arc::new(ImageDataInner {
            width,
            height,
            data,
            data_len,
            scale: 1.,
        }))
    }

    pub fn width(&self) -> i32 {
        (self.0.width as f32 / self.0.scale) as i32
    }

    pub fn height(&self) -> i32 {
        (self.0.height as f32 / self.0.scale) as i32
    }

    pub fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts_mut(self.0.data, self.0.data_len) }
    }

    pub fn data_mut(&self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.0.data, self.0.data_len) }
    }

    pub fn data_len(&self) -> usize {
        self.0.data_len
    }

    pub unsafe fn data_raw(&self) -> *mut u8 {
        self.0.data
    }
}

impl From<&ImageData> for ImageData {
    fn from(data: &ImageData) -> Self {
        Self::new(data.0.width, data.0.height)
    }
}

impl Drop for ImageDataInner {
    fn drop(&mut self) {
        // let _ =
        //     unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)) };
    }
}
