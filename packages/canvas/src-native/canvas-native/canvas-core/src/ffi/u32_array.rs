#[repr(C)]
pub struct U32Array {
    pub data: *mut u32,
    pub data_len: usize,
    pub data_cap: usize,
}

impl Into<Vec<u32>> for U32Array {
    fn into(self) -> Vec<u32> {
        unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) }
    }
}

impl From<Vec<u32>> for U32Array {
    fn from(mut vec: Vec<u32>) -> Self {
        let array = Self {
            data: vec.as_mut_ptr(),
            data_len: vec.len(),
            data_cap: vec.capacity(),
        };
        std::mem::forget(vec);
        array
    }
}

impl Drop for U32Array {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) };
    }
}
