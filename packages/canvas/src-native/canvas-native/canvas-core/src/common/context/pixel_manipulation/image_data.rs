use std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageData {
    pub(crate) data: *mut u8,
    pub(crate) data_len: usize,
    width: c_int,
    height: c_int,
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
        let mut data = vec![255u8; (width * height * 4) as usize];
        let (data, data_len) = Self::to_raw(data);
        Self {
            width,
            height,
            data,
            data_len,
        }
    }

    pub(crate) fn set_data(&mut self, data: Vec<u8>) {
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len));
        }
        let (data, data_len) = Self::to_raw(data);
        self.data = data;
        self.data_len = data_len;
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.data_len) }
    }
}

impl From<&ImageData> for ImageData {
    fn from(data: &ImageData) -> Self {
        Self::new(data.width, data.height)
    }
}

impl Drop for ImageData {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)) };
    }
}
