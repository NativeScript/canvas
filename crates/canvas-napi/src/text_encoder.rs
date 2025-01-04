use napi::bindgen_prelude::{Buffer, ObjectFinalize};
use napi::*;
use std::ffi::{CStr, CString, NulError};

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

    #[napi(getter)]
    pub fn encoding(&self) -> &str {
        let encoder = unsafe { &*self.encoder };
        encoder.encoding()
    }

    #[napi]
    pub fn encode(&self, text: String) -> Buffer {
        let encoder = unsafe { &mut *self.encoder };
        Buffer::from(encoder.encode(&text))
    }
}
