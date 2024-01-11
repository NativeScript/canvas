use encoding_rs::UTF_8;
use std::borrow::Cow;

#[derive(Clone)]
pub struct TextEncoder {
    inner: &'static encoding_rs::Encoding,
}

impl TextEncoder {
    pub fn new(encoding: Option<&str>) -> Self {
        let encoding = encoding.unwrap_or("utf-8");
        let encoder = encoding_rs::Encoding::for_label(encoding.as_bytes())
            .unwrap_or(UTF_8.output_encoding());
        Self { inner: encoder }
    }

    pub fn encode(&self, text: &str) -> Vec<u8> {
        let result = self.inner.encode(text);
        Vec::from(result.0)
    }

    pub fn encode_to_cow<'a>(&mut self, text: &'a str) -> Cow<'a, [u8]> {
        let result = self.inner.encode(text);
        result.0
    }

    pub fn encoding(&self) -> &str {
        self.inner.name()
    }
}

pub fn destroy_text_encoder(encoder: *mut TextEncoder) {
    unsafe {
        if !encoder.is_null() {
            let _ = Box::from_raw(encoder);
        }
    }
}
