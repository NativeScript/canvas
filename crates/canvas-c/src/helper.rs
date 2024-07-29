use crate::buffers::U8Buffer;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::c_char;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct FileHelper {
    data: Option<U8Buffer>,
    error: Option<String>,
}

#[no_mangle]
pub extern "C" fn canvas_native_helper_reference(value: *const FileHelper) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Arc::increment_strong_count(value) };
}

#[no_mangle]
pub extern "C" fn canvas_native_helper_release(value: *const FileHelper) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Arc::decrement_strong_count(value) };
}

#[no_mangle]
pub extern "C" fn canvas_native_helper_read_file(path: *const c_char) -> *const FileHelper {
    assert!(!path.is_null());
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_string_lossy();
    let mut ret = FileHelper::default();
    match std::fs::File::open(path.as_ref()) {
        Ok(mut file) => {
            let size = file.metadata().map(|meta| meta.len()).unwrap_or(0) as usize;
            let mut buf = Vec::with_capacity(size);
            let _ = file.read_to_end(&mut buf);
            let buf = U8Buffer::from(buf);
            ret.data = Some(buf);
        }
        Err(error) => ret.error = Some(error.to_string()),
    }
    Arc::into_raw(Arc::new(ret))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_read_file_has_error(file: *const FileHelper) -> bool {
    if file.is_null() {
        return false;
    };
    let file = &*file;
    return file.error.is_some();
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_read_file_get_data(
    file: *mut FileHelper,
) -> *const U8Buffer {
    assert!(!file.is_null());
    let file = &*file;
    let data = match &file.data {
        None => U8Buffer::default(),
        Some(data) => data.clone(),
    };
    Arc::into_raw(Arc::new(data))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_read_file_get_error(
    file: *const FileHelper,
) -> *const c_char {
    if file.is_null() {
        return std::ptr::null();
    };
    let file = &*file;
    match file.error.as_ref() {
        None => std::ptr::null(),
        Some(data) => CString::new(data.as_str()).unwrap().into_raw(),
    }
}
