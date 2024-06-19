use std::os::raw::{c_double, c_float, c_int};

use canvas_core::ffi::f32_array::F32Array;
use canvas_core::ffi::f64_array::F64Array;
use canvas_core::ffi::i16_array::I16Array;
use canvas_core::ffi::i32_array::I32Array;
use canvas_core::ffi::i8_array::I8Array;
use canvas_core::ffi::u16_array::U16Array;
use canvas_core::ffi::u32_array::U32Array;
use canvas_core::ffi::u8_array::U8Array;

#[no_mangle]
pub extern "C" fn canvas_native_get_i8_array_data(array: *mut I8Array) -> *mut i8 {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data;
        }
    }
    return std::ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn canvas_native_get_i8_array_data_len(array: *mut I8Array) -> usize {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data_len;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_i8_array(array: *mut I8Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_get_i16_array_data(array: *mut I16Array) -> *mut i16 {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data;
        }
    }
    return std::ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn canvas_native_get_i16_array_data_len(array: *mut I16Array) -> usize {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data_len;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_i16_array(array: *mut I16Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_get_I32_array_data(array: *mut I32Array) -> *mut c_int {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data;
        }
    }
    return std::ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn canvas_native_get_I32_array_data_len(array: *mut I32Array) -> usize {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data_len;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_i32_array(array: *mut I32Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_get_u8_array_data(array: *mut U8Array) -> *mut u8 {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data;
        }
    }
    return std::ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn canvas_native_get_u8_array_data_len(array: *mut U8Array) -> usize {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data_len;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_u8_array(array: *mut U8Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_get_u16_array_data(array: *mut U16Array) -> *mut u16 {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data;
        }
    }
    return std::ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn canvas_native_get_u16_array_data_len(array: *mut U16Array) -> usize {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data_len;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_u16_array(array: *mut U16Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_get_u32_array_data(array: *mut U32Array) -> *mut u32 {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data;
        }
    }
    return std::ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn canvas_native_get_u32_array_data_len(array: *mut U32Array) -> usize {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data_len;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_u32_array(array: *mut U32Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_get_f32_array_data(array: *mut F32Array) -> *mut c_float {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data;
        }
    }
    return std::ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn canvas_native_get_f32_array_data_len(array: *mut F32Array) -> usize {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data_len;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_f32_array(array: *mut F32Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_get_f64_array_data(array: *mut F64Array) -> *mut c_double {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data;
        }
    }
    return std::ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn canvas_native_get_f64_array_data_len(array: *mut F64Array) -> usize {
    unsafe {
        if !array.is_null() {
            let array = &*array;
            return array.data_len;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn canvas_native_destroy_f64_array(array: *mut F64Array) {
    unsafe {
        if !array.is_null() {
            let _ = Box::from_raw(array);
        }
    }
}
