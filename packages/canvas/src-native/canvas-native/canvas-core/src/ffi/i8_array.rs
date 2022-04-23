#[repr(C)]
pub struct I8Array {
    pub data: *mut i8,
    pub data_len: usize,
    pub data_cap: usize,
}

impl Into<Vec<i8>> for I8Array {
    fn into(self) -> Vec<i8> {
        unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) }
    }
}

impl From<Vec<i8>> for I8Array {
    fn from(mut vec: Vec<i8>) -> Self {
        let array = Self {
            data: vec.as_mut_ptr(),
            data_len: vec.len(),
            data_cap: vec.capacity(),
        };
        std::mem::forget(vec);
        array
    }
}

impl Drop for I8Array {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) };
    }
}
