#[repr(C)]
pub struct I16Array {
    pub data: *mut i16,
    pub data_len: usize,
}

impl Into<Vec<i16>> for I16Array {
    fn into(self) -> Vec<i16> {
        unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)).into_vec()
        }
    }
}

impl From<Vec<i16>> for I16Array {
    fn from(vec: Vec<i16>) -> Self {
        let mut box_slice = vec.into_boxed_slice();
        let array = Self {
            data: box_slice.as_mut_ptr(),
            data_len: box_slice.len(),
        };
        let _ = Box::into_raw(box_slice);
        array
    }
}

impl Drop for I16Array {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)) };
    }
}

#[no_mangle]
pub extern "C" fn destroy_i16_array(array: *mut I16Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}