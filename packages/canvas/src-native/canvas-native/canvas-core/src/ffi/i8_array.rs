#[repr(C)]
pub struct I8Array {
    pub data: *mut i8,
    pub data_len: usize,
}

impl Into<Vec<i8>> for I8Array {
    fn into(self) -> Vec<i8> {
        unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)).into_vec()
        }
    }
}

impl From<Vec<i8>> for I8Array {
    fn from(vec: Vec<i8>) -> Self {
        let mut box_slice = vec.into_boxed_slice();
        let array = Self {
            data: box_slice.as_mut_ptr(),
            data_len: box_slice.len(),
        };
        let _ = Box::into_raw(box_slice);
        array
    }
}

impl Drop for I8Array {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)) };
    }
}
