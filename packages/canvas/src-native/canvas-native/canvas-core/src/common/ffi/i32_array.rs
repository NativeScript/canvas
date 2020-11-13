#[repr(C)]
pub struct I32Array {
    pub data: *mut i32,
    pub data_len: usize,
}

impl Into<Vec<i32>> for I32Array {
    fn into(self) -> Vec<i32> {
        unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)).into_vec()
        }
    }
}

impl From<Vec<i32>> for I32Array {
    fn from(vec: Vec<i32>) -> Self {
        let mut box_slice = vec.into_boxed_slice();
        let mut array = Self {
            data: box_slice.as_mut_ptr(),
            data_len: box_slice.len(),
        };
        let _ = Box::into_raw(box_slice);
        array
    }
}

impl Drop for I32Array {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)) };
    }
}


#[no_mangle]
pub extern "C" fn destroy_i32_array(array: *mut I32Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}