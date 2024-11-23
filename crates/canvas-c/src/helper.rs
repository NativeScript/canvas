use crate::buffers::U8Buffer;
use base64::Engine;
use bytes::{Buf, Bytes};
use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::c_char;
#[derive(Clone, Default)]
pub struct FileHelper {
    data: Option<U8Buffer>,
    error: Option<String>,
    ext: Option<String>,
    mime: Option<String>,
}


#[repr(C)]
pub struct FileHelperMime {
    mime_type: *const c_char,
    extension: *const c_char,
}

impl Drop for FileHelperMime {
    fn drop(&mut self) {
        if self.mime_type != std::ptr::null() {
            let _ = unsafe { CString::from_raw(self.mime_type as *mut _) };
        }

        if self.extension != std::ptr::null() {
            let _ = unsafe { CString::from_raw(self.extension as *mut _) };
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_base64_encode(data: *const u8, size: usize) -> *const c_char {
    if data.is_null() || size == 0 {
        return std::ptr::null();
    }
    let bytes = std::slice::from_raw_parts(data, size);
    let ret = base64::engine::general_purpose::STANDARD.encode(bytes);
    if ret.is_empty() {
        return std::ptr::null();
    }
    CString::new(ret).unwrap().into_raw()
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_base64_encode_c_str(data: *const c_char) -> *const c_char {
    if data.is_null() {
        return std::ptr::null();
    }
    let data = CStr::from_ptr(data);
    let ret = base64::engine::general_purpose::STANDARD.encode(data.to_bytes());
    if ret.is_empty() {
        return std::ptr::null();
    }
    CString::new(ret).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_base64_decode(data: *const u8, size: usize) -> *mut U8Buffer {
    if size == 0 {
        return std::ptr::null_mut();
    }
    let slice = std::slice::from_raw_parts(data, size);
    base64::engine::general_purpose::STANDARD.decode(slice).map(|buf| {
        match Bytes::from(buf).try_into_mut() {
            Ok(bytes) => {
                Box::into_raw(Box::new(U8Buffer::from(bytes)))
            }
            Err(_) => std::ptr::null_mut(),
        }
    })

        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_base64_decode_c_str(data: *const c_char) -> *mut U8Buffer {
    if data.is_null() {
        return std::ptr::null_mut();
    }
    let str = CStr::from_ptr(data);
    base64::engine::general_purpose::STANDARD.decode(str.to_bytes()).map(|buf| {
        match Bytes::from(buf).try_into_mut() {
            Ok(bytes) => {
                Box::into_raw(Box::new(U8Buffer::from(bytes)))
            }
            Err(_) => std::ptr::null_mut(),
        }
    })

        .unwrap_or(std::ptr::null_mut())
}


#[no_mangle]
pub extern "C" fn canvas_native_helper_get_mime(data: *const u8, size: usize) -> *mut FileHelperMime {
    let mut ret = FileHelperMime {
        mime_type: std::ptr::null_mut(),
        extension: std::ptr::null_mut(),
    };


    if data.is_null() || size == 0 {
        return Box::into_raw(Box::new(ret));
    }

    let data = unsafe { std::slice::from_raw_parts(data, size) };
    match infer::get(data) {
        None => std::ptr::null_mut(),
        Some(_type) => {
            ret.mime_type = CString::new(_type.mime_type().to_string()).unwrap().into_raw();
            ret.extension = CString::new(_type.extension().to_string()).unwrap().into_raw();
            Box::into_raw(Box::new(ret))
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_helper_release_mime(mime: *mut FileHelperMime) {
    if !mime.is_null() {
        let _ = unsafe { Box::from_raw(mime) };
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_helper_release(value: *mut FileHelper) {
    if value.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(value) };
}


#[no_mangle]
pub extern "C" fn canvas_native_helper_read_file(path: *const c_char) -> *mut FileHelper {
    assert!(!path.is_null());
    let path = unsafe { CStr::from_ptr(path).to_str() };
    let mut ret = FileHelper::default();
    if let Ok(path) = path {
        match std::fs::File::open(path) {
            Ok(mut file) => {
                let size = file.metadata().map(|meta| meta.len()).unwrap_or(0) as usize;
                if size != 0 {
                    let mut buf = Vec::with_capacity(size);
                    let _ = file.read_to_end(&mut buf);
                    if let Some(_type) = infer::get(&buf) {
                        ret.mime = Some(_type.mime_type().to_string());
                        ret.ext = Some(_type.extension().to_string());
                    }
                    let buf = U8Buffer::from(buf);
                    ret.data = Some(buf);
                }
            }
            Err(error) => ret.error = Some(error.to_string()),
        }
    }
    Box::into_raw(Box::new(ret))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_read_file_get_mime(
    file: *mut FileHelper,
) -> *mut c_char {
    assert!(!file.is_null());
    let file = &*file;
    file.mime.as_ref().map(|mime| CString::new(mime.to_string()).unwrap().into_raw()).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_read_file_get_extension(
    file: *mut FileHelper,
) -> *mut c_char {
    assert!(!file.is_null());
    let file = &*file;
    file.ext.as_ref().map(|ext| CString::new(ext.to_string()).unwrap().into_raw()).unwrap_or(std::ptr::null_mut())
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_read_file_has_error(file: *const FileHelper) -> bool {
    if file.is_null() {
        return false;
    };
    let file = &*file;
    file.error.is_some()
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_read_file_get_data(
    file: *mut FileHelper,
) -> *mut U8Buffer {
    assert!(!file.is_null());
    let file = &*file;
    let data = match &file.data {
        None => U8Buffer::default(),
        Some(data) => data.clone(),
    };
    Box::into_raw(Box::new(data))
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_helper_read_file_take_data(
    file: *mut FileHelper,
) -> *mut U8Buffer {
    assert!(!file.is_null());
    let file = &mut *file;
    let data = file.data.take().unwrap_or_else(|| U8Buffer::default());
    Box::into_raw(Box::new(data))
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
