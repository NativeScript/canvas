#[repr(C)]
pub struct F64Array {
    pub data: *mut f64,
    pub data_len: usize,
}

impl Into<Vec<f64>> for F64Array {
    fn into(self) -> Vec<f64> {
        unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)).into_vec()
        }
    }
}

impl From<Vec<f64>> for F64Array {
    fn from(vec: Vec<f64>) -> Self {
        let mut box_slice = vec.into_boxed_slice();
        let mut array = Self {
            data: box_slice.as_mut_ptr(),
            data_len: box_slice.len(),
        };
        let _ = Box::into_raw(box_slice);
        array
    }
}

impl Drop for F64Array {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)) };
    }
}


#[no_mangle]
pub extern "C" fn destroy_f64_array(array: *mut F64Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}