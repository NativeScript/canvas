use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_longlong};
use std::ptr::null;

use encoding_rs::UTF_8;
use libc::size_t;

#[repr(C)]
pub struct TextDecoder {
    decoder: &'static encoding_rs::Encoding,
}

impl TextDecoder {
    pub fn new(decoding: *const c_char) -> Self {
        let decoding = unsafe { CStr::from_ptr(decoding) }
            .to_str()
            .unwrap_or("utf-8");
        let decoder = encoding_rs::Encoding::for_label(decoding.as_bytes())
            .unwrap_or(UTF_8.output_encoding());
        Self { decoder }
    }

    unsafe fn str_from_u8_nul_utf8_unchecked(utf8_src: &[u8]) -> &str {
        let nul_range_end = utf8_src
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(utf8_src.len()); // default to length if no `\0` present
        ::std::str::from_utf8_unchecked(&utf8_src[0..nul_range_end])
    }

    pub fn decode(&mut self, data: *const u8, len: size_t) -> *const c_char {
        let txt = unsafe { std::slice::from_raw_parts(data, len) };
        let (res, _) = self.decoder.decode_with_bom_removal(txt);
        let ptr = unsafe { TextDecoder::str_from_u8_nul_utf8_unchecked(res.as_bytes()) };
        let result = CString::new(ptr);
        match result {
            Ok(result) => result.into_raw(),
            Err(err) => {
                dbg!("error {:?}", err.to_string());
                null()
            }
        }
    }

    /*

    pub fn decode(&mut self, data: *const u8, len: size_t) -> *const c_char {
        let txt = unsafe { std::slice::from_raw_parts(data, len) };
        let decoder = self.decoder.new_decoder_with_bom_removal();
        let result = self.decoder.decode_with_bom_removal(txt);
        let raw = result.0;
        let string = String::from(raw);
        let result = CString::new(string);
        match result {
            Ok(result) => result.into_raw(),
            Err(err) => {
                dbg!("error {:?}", err.to_string());
                null()
            }
        }
    }
     */

    pub fn encoding(&self) -> *const c_char {
        CString::new(self.decoder.name()).unwrap().into_raw()
    }

    pub fn release(ptr: c_longlong) {
        if ptr != 0 {
            let _: Box<TextDecoder> = unsafe { Box::from_raw(ptr as *mut _) };
        }
    }
}

pub(crate) fn text_decoder_get_encoding(decoder: c_longlong) -> *const c_char {
    if decoder != 0 {
        let decoder: Box<TextDecoder> = unsafe { Box::from_raw(decoder as *mut _) };
        let encoding = decoder.encoding();
        Box::into_raw(decoder);
        return encoding;
    }
    null()
}

pub(crate) fn text_decoder_decode(
    decoder: c_longlong,
    data: *const u8,
    len: size_t,
) -> *const c_char {
    if decoder != 0 {
        let mut decoder: Box<TextDecoder> = unsafe { Box::from_raw(decoder as *mut _) };
        let decoded = decoder.decode(data, len);
        Box::into_raw(decoder);
        return decoded;
    }
    null()
}

#[allow(unused)]
pub(crate) fn free_text_decoder(decoder: i64) {
    TextDecoder::release(decoder);
}
