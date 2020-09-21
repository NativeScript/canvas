use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_longlong};
use std::ptr::null;

use encoding_rs::UTF_8;

use crate::common::NativeByteArray;

#[repr(C)]
pub struct TextEncoder {
    encoder: &'static encoding_rs::Encoding,
}

impl TextEncoder {
    pub fn new(encoding: *const c_char) -> Self {
        let encoding = unsafe { CStr::from_ptr(encoding) }
            .to_str()
            .unwrap_or("utf-8");
        let encoder = encoding_rs::Encoding::for_label(encoding.as_bytes())
            .unwrap_or(UTF_8.output_encoding());
        Self { encoder }
    }

    pub fn encode(&mut self, text: *const c_char) -> *mut NativeByteArray {
        let txt = unsafe { CStr::from_ptr(text) };
        let value = String::from_utf8_lossy(txt.to_bytes());
        let val = &value[..];
        let result = self.encoder.encode(val);
        let mut data = Vec::from(result.0).into_boxed_slice();
        let array = data.as_mut_ptr();
        let length = data.len();
        std::mem::forget(data);
        NativeByteArray { array, length }.into_raw()
    }

    pub fn encoding(&self) -> *const c_char {
        CString::new(self.encoder.name()).unwrap().into_raw()
    }

    pub fn release(ptr: c_longlong) {
        if ptr != 0 {
            let _: Box<TextEncoder> = unsafe { Box::from_raw(ptr as *mut _) };
        }
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn into_ptr(self) -> c_longlong {
        self.into_raw() as i64
    }
}

#[inline]
pub(crate) fn text_encoder_get_encoding(encoder: c_longlong) -> *const c_char {
    if encoder != 0 {
        let encoder: Box<TextEncoder> = unsafe { Box::from_raw(encoder as *mut _) };
        return encoder.encoding();
    }
    null()
}

#[inline]
pub(crate) fn text_encoder_encode(
    encoder: c_longlong,
    text: *const c_char,
) -> *mut NativeByteArray {
    if encoder != 0 {
        let mut encoder: Box<TextEncoder> = unsafe { Box::from_raw(encoder as *mut _) };
        return encoder.encode(text);
    }
    NativeByteArray::empty().into_raw()
}

#[inline]
#[allow(unused)]
pub(crate) fn free_byte_array(array: *mut NativeByteArray) {
    if array.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(array);
    }
}

#[allow(unused)]
#[inline]
pub(crate) fn free_text_encoder(encoder: i64) {
    TextEncoder::release(encoder);
}
