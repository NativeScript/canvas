use std::os::raw::c_float;

#[repr(C)]
pub struct F32Array {
    pub data: *mut c_float,
    pub data_len: usize,
    pub data_cap: usize,
}

impl Into<Vec<f32>> for F32Array {
    fn into(self) -> Vec<f32> {
        unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) }
    }
}

impl From<Vec<f32>> for F32Array {
    fn from(mut vec: Vec<f32>) -> Self {
        let array = Self {
            data: vec.as_mut_ptr(),
            data_len: vec.len(),
            data_cap: vec.capacity(),
        };
        std::mem::forget(vec);
        array
    }
}

impl Drop for F32Array {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) };
    }
}
