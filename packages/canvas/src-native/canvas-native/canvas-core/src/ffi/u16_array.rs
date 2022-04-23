#[repr(C)]
pub struct U16Array {
    pub data: *mut u16,
    pub data_len: usize,
    pub data_cap: usize
}

impl Into<Vec<u16>> for U16Array {
    fn into(self) -> Vec<u16> {
        unsafe {
            Vec::from_raw_parts(self.data, self.data_len, self.data_cap)
        }
    }
}

impl From<Vec<u16>> for U16Array {
    fn from(mut vec: Vec<u16>) -> Self {
        let array = Self {
            data: vec.as_mut_ptr(),
            data_len: vec.len(),
            data_cap: vec.capacity()
        };
        std::mem::forget(vec);
        array
    }
}

impl Drop for U16Array {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) };
    }
}