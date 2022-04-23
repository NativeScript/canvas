#[repr(C)]
pub struct I16Array {
    pub data: *mut i16,
    pub data_len: usize,
    pub data_cap: usize,
}

impl Into<Vec<i16>> for I16Array {
    fn into(self) -> Vec<i16> {
        unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) }
    }
}

impl From<Vec<i16>> for I16Array {
    fn from(mut vec: Vec<i16>) -> Self {
        let array = Self {
            data: vec.as_mut_ptr(),
            data_len: vec.len(),
            data_cap: vec.capacity(),
        };
        std::mem::forget(vec);
        array
    }
}

impl Drop for I16Array {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) };
    }
}
