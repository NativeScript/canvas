#[repr(C)]
pub struct I32Array {
    pub data: *mut i32,
    pub data_len: usize,
    pub data_cap: usize,
}

impl Into<Vec<i32>> for I32Array {
    fn into(self) -> Vec<i32> {
        unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) }
    }
}

impl From<Vec<i32>> for I32Array {
    fn from(mut vec: Vec<i32>) -> Self {
        let array = Self {
            data: vec.as_mut_ptr(),
            data_len: vec.len(),
            data_cap: vec.capacity(),
        };
        std::mem::forget(vec);
        array
    }
}

impl Drop for I32Array {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) };
    }
}
