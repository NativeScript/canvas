use std::ffi::{CStr, CString};

use encoding_rs::UTF_8;

pub struct TextDecoder {
    decoder: &'static encoding_rs::Encoding,
}

impl TextDecoder {
    pub fn new(decoding: Option<&str>) -> Self {
        let decoding = decoding.unwrap_or("utf-8");
        let decoder = encoding_rs::Encoding::for_label(decoding.as_bytes())
            .unwrap_or(UTF_8.output_encoding());
        Self { decoder }
    }

    // fn handle_slice(&self, res: Vec<u8>) -> &'static str {
    //     unsafe {
    //         let mut utf8_src = res.as_slice();
    //         let nul_range_end = utf8_src
    //             .iter()
    //             .position(|&c| c == b'\0')
    //             .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    //         let data = &utf8_src[0..nul_range_end];
    //         std::str::from_utf8_unchecked(data)
    //     }
    // }

    pub fn decode(&mut self, data: *const u8, len: usize) -> CString {
        let txt = unsafe { std::slice::from_raw_parts(data, len) };
        let (res, _) = self.decoder.decode_with_bom_removal(txt);

        let utf8_src = res.as_bytes();
        let nul_range_end = utf8_src
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(utf8_src.len()); // default to length if no `\0` present
        let data = &utf8_src[0..nul_range_end];
        // let c_str = unsafe { CStr::from_bytes_with_nul_unchecked(data) };
        let str = unsafe { std::str::from_utf8_unchecked(data) };
        CString::new(str).unwrap()
    }

    pub(crate) fn decode_to_bytes(&mut self, txt: &str) -> Vec<u8> {
        let (res, _) = self.decoder.decode_with_bom_removal(txt.as_bytes());

        let mut utf8_src = res.as_bytes();
        let nul_range_end = utf8_src
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(utf8_src.len()); // default to length if no `\0` present
        let data = &utf8_src[0..nul_range_end];
        data.to_vec()
    }


    pub fn encoding(&self) -> &str {
        self.decoder.name()
    }
}

pub fn destroy_text_decoder(decoder: *mut TextDecoder) {
    unsafe {
        if !decoder.is_null() {
            let _ = Box::from_raw(decoder);
        }
    }
}
