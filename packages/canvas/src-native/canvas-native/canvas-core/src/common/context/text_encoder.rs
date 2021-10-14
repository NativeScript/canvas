use encoding_rs::UTF_8;

pub struct TextEncoder {
    encoder: &'static encoding_rs::Encoding,
}

impl TextEncoder {
    pub fn new(encoding: Option<&str>) -> Self {
        let encoding = encoding.unwrap_or("utf-8");
        let encoder = encoding_rs::Encoding::for_label(encoding.as_bytes())
            .unwrap_or(UTF_8.output_encoding());
        Self { encoder }
    }

    pub fn encode(&mut self, text: &str) -> Vec<u8> {
        let result = self.encoder.encode(text);
        Vec::from(result.0)
    }

    pub fn encoding(&self) -> &str {
        self.encoder.name()
    }
}

pub fn destroy_text_encoder(encoder: *mut TextEncoder) {
    unsafe {
        if !encoder.is_null() {
            let _ = Box::from_raw(encoder);
        }
    }
}
