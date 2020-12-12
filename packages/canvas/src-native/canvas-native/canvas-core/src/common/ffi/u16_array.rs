#[repr(C)]
pub struct U16Array {
    pub data: *mut u16,
    pub data_len: usize,
}

impl Into<Vec<u16>> for U16Array {
    fn into(self) -> Vec<u16> {
        unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)).into_vec()
        }
    }
}

impl From<Vec<u16>> for U16Array {
    fn from(vec: Vec<u16>) -> Self {
        let mut box_slice = vec.into_boxed_slice();
        let mut array = Self {
            data: box_slice.as_mut_ptr(),
            data_len: box_slice.len(),
        };
        let _ = Box::into_raw(box_slice);
        array
    }
}

impl Drop for U16Array {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)) };
    }
}

#[no_mangle]
pub extern "C" fn destroy_u16_array(array: *mut U16Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}