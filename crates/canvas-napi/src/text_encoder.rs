use napi::bindgen_prelude::{ObjectFinalize, Uint8Array};
use napi::*;
use std::ffi::CString;

#[napi(custom_finalize)]
pub struct TextEncoder {
    pub(crate) encoder: *mut canvas_c::TextEncoder,
}

impl ObjectFinalize for TextEncoder {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_text_encoder_release(self.encoder);
        Ok(())
    }
}

#[napi]
impl TextEncoder {
    #[napi(constructor)]
    pub fn new(encoding: Option<JsString>) -> TextEncoder {
        if let Some(encoding) = encoding {
            let encoder = if let Some(encoding) = encoding.into_utf8().ok() {
                if let Ok(encoding) = encoding.as_str() {
                    match CString::new(encoding) {
                        Ok(encoding) => {
                            canvas_c::canvas_native_text_encoder_create(encoding.as_ptr())
                        }
                        Err(_) => {
                            let encoding = c"utf-8";
                            canvas_c::canvas_native_text_encoder_create(encoding.as_ptr())
                        }
                    }
                } else {
                    let encoding = c"utf-8";
                    canvas_c::canvas_native_text_encoder_create(encoding.as_ptr())
                }
            } else {
                let encoding = c"utf-8";
                canvas_c::canvas_native_text_encoder_create(encoding.as_ptr())
            };

            TextEncoder {
                encoder
            }
        } else {
            let encoding = c"utf-8";
            TextEncoder {
                encoder: canvas_c::canvas_native_text_encoder_create(encoding.as_ptr())
            }
        }
    }

    /// Lower case, as the V8 bindings (and the web) report it: `"utf-8"`.
    #[napi(getter)]
    pub fn encoding(&self) -> String {
        let encoder = unsafe { &*self.encoder };
        encoder.encoding().to_lowercase()
    }

    /// A `Uint8Array` over the encoded bytes. UTF-8 (every encoder, per the spec) costs one copy,
    /// out of the JS string; the array takes the bytes as they are.
    #[napi]
    pub fn encode(&self, text: Option<String>) -> Uint8Array {
        let text = text.unwrap_or_default();
        let encoder = unsafe { &*self.encoder };
        if encoder.encoding() == "UTF-8" {
            Uint8Array::new(text.into_bytes())
        } else {
            Uint8Array::new(encoder.encode(&text))
        }
    }
}
