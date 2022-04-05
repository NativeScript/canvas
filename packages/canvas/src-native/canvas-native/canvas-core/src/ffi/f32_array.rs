use std::os::raw::c_float;

#[repr(C)]
pub struct F32Array {
    pub data: *mut std::os::raw::c_float,
    pub data_len: usize,
}
impl Into<Vec<f32>> for F32Array {
    fn into(self) -> Vec<f32> {
        unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)).into_vec()
        }
    }
}

impl From<Vec<f32>> for F32Array {
    fn from(vec: Vec<f32>) -> Self {
        let mut box_slice = vec.into_boxed_slice();
        let array = Self {
            data: box_slice.as_mut_ptr(),
            data_len: box_slice.len(),
        };
        let _ = Box::into_raw(box_slice);
        array
    }
}

impl Drop for F32Array {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.data_len)) };
    }
}
