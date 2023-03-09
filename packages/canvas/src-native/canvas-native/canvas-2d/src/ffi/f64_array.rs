use std::os::raw::c_double;

#[repr(C)]
pub struct F64Array {
    pub data: *mut f64,
    pub data_len: usize,
    pub data_cap: usize,
}

impl Into<Vec<f64>> for F64Array {
    fn into(self) -> Vec<f64> {
        unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) }
    }
}

impl From<Vec<f64>> for F64Array {
    fn from(mut vec: Vec<f64>) -> Self {
        let array = Self {
            data: vec.as_mut_ptr(),
            data_len: vec.len(),
            data_cap: vec.capacity(),
        };
        std::mem::forget(vec);
        array
    }
}

impl Drop for F64Array {
    fn drop(&mut self) {
        let _ = unsafe { Vec::from_raw_parts(self.data, self.data_len, self.data_cap) };
    }
}
