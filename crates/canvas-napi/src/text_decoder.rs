use napi::bindgen_prelude::{BufferSlice, ObjectFinalize};
use napi::*;
use std::ffi::{CStr, CString, NulError};

#[napi(custom_finalize)]
pub struct TextDecoder {
    pub(crate) decoder: *mut canvas_c::TextDecoder,
}

impl ObjectFinalize for TextDecoder {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_text_decoder_release(self.decoder);
        Ok(())
    }
}

#[napi]
impl TextDecoder {
    #[napi(constructor)]
    pub fn new(encoding: Option<JsString>) -> TextDecoder {
        if let Some(encoding) = encoding {
            let decoder = if let Some(encoding) = encoding.into_utf8().ok() {
                if let Ok(encoding) = encoding.as_str() {
                    match CString::new(encoding) {
                        Ok(encoding) => {
                            canvas_c::canvas_native_text_decoder_create(encoding.as_ptr())
                        }
                        Err(_) => {
                            let encoding = c"utf-8";
                            canvas_c::canvas_native_text_decoder_create(encoding.as_ptr())
                        }
                    }
                } else {
                    let encoding = c"utf-8";
                    canvas_c::canvas_native_text_decoder_create(encoding.as_ptr())
                }
            } else {
                let encoding = c"utf-8";
                canvas_c::canvas_native_text_decoder_create(encoding.as_ptr())
            };

            TextDecoder {
                decoder
            }
        } else {
            let encoding = c"utf-8";
            TextDecoder {
                decoder: canvas_c::canvas_native_text_decoder_create(encoding.as_ptr())
            }
        }
    }

    #[napi(getter)]
    pub fn encoding(&self) -> &str {
        let decoder = unsafe { &*self.decoder };
        decoder.encoding()
    }

    #[napi]
    pub fn decode(&self, data: &[u8], env: Env) -> Result<String> {
        let decoder = unsafe { &*self.decoder };
        Ok(decoder.decode(data))
    }
}
