#[repr(C)]
pub struct U8Array {
    pub data: *mut u8,
    pub data_len: usize,
    pub data_cap: usize,
}

impl Into<Vec<u8>> for U8Array {
    fn into(self) -> Vec<u8> {
        unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) }
    }
}

impl From<Vec<u8>> for U8Array {
    fn from(mut vec: Vec<u8>) -> Self {
        let array = Self {
            data: vec.as_mut_ptr(),
            data_len: vec.len(),
            data_cap: vec.capacity(),
        };
        std::mem::forget(vec);
        array
    }
}

impl Drop for U8Array {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) };
    }
}
