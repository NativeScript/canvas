#![allow(dead_code)]

use std::borrow::Cow;
use std::ffi::{c_char, CStr, CString};

use encoding_rs::UTF_8;

#[derive(Clone)]
pub struct TextDecoder(&'static encoding_rs::Encoding);

impl TextDecoder {
    pub fn new(decoding: Option<&str>) -> Self {
        let decoding = decoding.unwrap_or("utf-8");
        let decoder = encoding_rs::Encoding::for_label(decoding.as_bytes())
            .unwrap_or(UTF_8.output_encoding());
        Self(decoder)
    }

    pub fn decode_to_string(&self, data: &[u8]) -> String {
        let (res, _) = self.0.decode_with_bom_removal(data);

        match CStr::from_bytes_with_nul(res.as_bytes()) {
            Ok(res) => res.to_string_lossy().to_string(),
            Err(_) => res.chars().filter(|&c| c != '\0').collect(),
        }
    }

    pub fn decode(&self, data: *const u8, len: usize) -> CString {
        let txt = unsafe { std::slice::from_raw_parts(data, len) };
        let (res, _) = self.0.decode_with_bom_removal(txt);

        match CStr::from_bytes_with_nul(res.as_bytes()) {
            Ok(res) => CString::from(res),
            Err(_) => {
                //  let value: String =  res.chars().filter(|&c| c != '\0').collect();

                let utf8_src = res.as_bytes();
                let nul_range_end = utf8_src
                    .iter()
                    .position(|&c| c == b'\0')
                    .unwrap_or(utf8_src.len()); // default to length if no `\0` present*/
                let data = &utf8_src[..nul_range_end];

                unsafe { CString::from_vec_unchecked(data.to_vec()) }
            }
        }
    }

    pub fn decode_as_cow(&self, data: *const u8, len: usize) -> Cow<str> {
        let txt = unsafe { std::slice::from_raw_parts(data, len) };
        let (res, _) = self.0.decode_with_bom_removal(txt);
        res
    }

    pub fn decode_c_string(&self, data: *const c_char) -> CString {
        let txt = unsafe { CStr::from_ptr(data) };
        let txt = txt.to_bytes();
        let (res, _) = self.0.decode_with_bom_removal(txt);

        match CStr::from_bytes_with_nul(res.as_bytes()) {
            Ok(res) => CString::from(res),
            Err(_) => {
                let value: String = res.chars().filter(|&c| c != '\0').collect();
                CString::new(value).unwrap()
            }
        }
    }

    pub fn decode_as_bytes(&self, data: *const u8, len: usize) -> Vec<u8> {
        let txt = unsafe { std::slice::from_raw_parts(data, len) };

        let (res, _) = self.0.decode_with_bom_removal(txt);

        res.as_bytes().to_vec()
    }

    pub(crate) fn decode_to_bytes(&self, txt: &str) -> Vec<u8> {
        let (res, _) = self.0.decode_with_bom_removal(txt.as_bytes());

        res.as_bytes().to_vec()
    }

    pub fn encoding(&self) -> &str {
        self.0.name()
    }
}
