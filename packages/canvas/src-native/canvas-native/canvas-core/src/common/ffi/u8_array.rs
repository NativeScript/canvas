#[repr(C)]
pub struct U8Array {
    pub data: *mut u8,
    pub data_len: usize,
}

impl Into<Vec<u8>> for U8Array {
    fn into(self) -> Vec<u8> {
        unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)).into_vec()
        }
    }
}

impl From<Vec<u8>> for U8Array {
    fn from(vec: Vec<u8>) -> Self {
        let mut box_slice = vec.into_boxed_slice();
        let mut array = Self {
            data: box_slice.as_mut_ptr(),
            data_len: box_slice.len(),
        };
        let _ = Box::into_raw(box_slice);
        array
    }
}

impl Drop for U8Array {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)) };
    }
}


#[no_mangle]
pub extern "C" fn destroy_u8_array(array: *mut U8Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}