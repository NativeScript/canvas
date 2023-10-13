#![allow(dead_code)]

use std::ffi::CString;

use encoding_rs::UTF_8;

#[derive(Clone)]
pub struct TextDecoder {
    inner: &'static encoding_rs::Encoding,
}

impl TextDecoder {
    pub fn new(decoding: Option<&str>) -> Self {
        let decoding = decoding.unwrap_or("utf-8");
        let decoder = encoding_rs::Encoding::for_label(decoding.as_bytes())
            .unwrap_or(UTF_8.output_encoding());
        Self { inner: decoder }
    }

    pub fn decode_to_string(&self, data: &[u8]) -> String {
        let (res, _) = self.inner.decode_with_bom_removal(data);
        res.to_string()
    }

    pub fn decode(&self, data: *const u8, len: usize) -> CString {
        let txt = unsafe { std::slice::from_raw_parts(data, len) };
        let (res, _) = self.inner.decode_with_bom_removal(txt);
        // let utf8_src = res.as_bytes();
        /*let nul_range_end = utf8_src
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present*/
        // let data = &utf8_src[0..];
        // let str = unsafe { std::str::from_utf8_unchecked(data) };
        CString::new(res.to_string()).unwrap()
    }

    pub fn decode_as_bytes(&self, data: *const u8, len: usize) -> Vec<u8> {
        let txt = unsafe { std::slice::from_raw_parts(data, len) };
        let (res, _) = self.inner.decode_with_bom_removal(txt);
        let utf8_src = res.as_bytes();
        /*let nul_range_end = utf8_src
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present*/
        // let data = &utf8_src[0..];
        // let str = unsafe { std::str::from_utf8_unchecked(data) };
        utf8_src.to_vec()
    }

    pub(crate) fn decode_to_bytes(&self, txt: &str) -> Vec<u8> {
        let (res, _) = self.inner.decode_with_bom_removal(txt.as_bytes());

        // let mut utf8_src = res.as_bytes();
        /* let nul_range_end = utf8_src
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present*/
        // let data = &utf8_src[0..];
        // data.to_vec()
        res.as_bytes().to_vec()
    }

    pub fn encoding(&self) -> &str {
        self.inner.name()
    }
}

pub fn destroy_text_decoder(decoder: *mut TextDecoder) {
    unsafe {
        if !decoder.is_null() {
            let _ = Box::from_raw(decoder);
        }
    }
}
