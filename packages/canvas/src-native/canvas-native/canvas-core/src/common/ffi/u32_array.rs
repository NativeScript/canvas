#[repr(C)]
pub struct U32Array {
    pub data: *mut u32,
    pub data_len: usize,
}

impl Into<Vec<u32>> for U32Array {
    fn into(self) -> Vec<u32> {
        unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)).into_vec()
        }
    }
}

impl From<Vec<u32>> for U32Array {
    fn from(vec: Vec<u32>) -> Self {
        let mut box_slice = vec.into_boxed_slice();
        let array = Self {
            data: box_slice.as_mut_ptr(),
            data_len: box_slice.len(),
        };
        let _ = Box::into_raw(box_slice);
        array
    }
}

impl Drop for U32Array {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)) };
    }
}


#[no_mangle]
pub extern "C" fn destroy_u32_array(array: *mut U32Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}