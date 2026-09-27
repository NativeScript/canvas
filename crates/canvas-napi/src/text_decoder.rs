use napi::bindgen_prelude::{AsyncTask, ObjectFinalize, Unknown};
use napi::*;
use std::ffi::CString;

use crate::module::{JsBytes, JsRaw, PinnedBytes};

const NOT_A_BUFFER: &str = "Failed to execute 'decode' on 'TextDecoder': The provided value is not of type '(ArrayBuffer or ArrayBufferView)'";

/// Decodes `bytes` straight into a JS string: canvas-c decodes as a `Cow` (borrowing `bytes` when
/// they are already valid UTF-8), and the only copy is the JS string's own.
fn decode_to_js(env: sys::napi_env, decoder: *const canvas_c::TextDecoder, bytes: &[u8]) -> Result<sys::napi_value> {
    let mut value = std::ptr::null_mut();
    if bytes.is_empty() {
        check_status!(unsafe { sys::napi_create_string_utf8(env, c"".as_ptr(), 0, &mut value) })?;
        return Ok(value);
    }
    let cow = canvas_c::canvas_native_text_decoder_decode_as_cow(decoder, bytes.as_ptr(), bytes.len());
    let status = unsafe {
        sys::napi_create_string_utf8(
            env,
            canvas_c::canvas_native_ccow_get_bytes(cow).cast(),
            canvas_c::canvas_native_ccow_get_length(cow) as isize,
            &mut value,
        )
    };
    canvas_c::canvas_native_ccow_release(cow);
    check_status!(status)?;
    Ok(value)
}

/// `decodeAsync`: decodes on a libuv worker, reading the buffer in place.
pub struct DecodeTask {
    decoder: canvas_c::TextDecoder,
    bytes: Option<PinnedBytes>,
}

impl DecodeTask {
    fn release(&mut self, env: &Env) {
        if let Some(bytes) = self.bytes.take() {
            bytes.release(env);
        }
    }
}

impl Task for DecodeTask {
    /// The decoded text (a canvas-c `CCow`, which may borrow the buffer), as an address.
    type Output = usize;
    type JsValue = JsRaw;

    fn compute(&mut self) -> Result<Self::Output> {
        let bytes = self.bytes.as_ref().ok_or_else(|| Error::from_reason(NOT_A_BUFFER))?.as_slice();
        if bytes.is_empty() {
            return Ok(0);
        }
        Ok(canvas_c::canvas_native_text_decoder_decode_as_cow(&self.decoder, bytes.as_ptr(), bytes.len()) as usize)
    }

    fn resolve(&mut self, env: Env, cow: usize) -> Result<Self::JsValue> {
        let cow = cow as *mut canvas_c::CCow;
        let mut value = std::ptr::null_mut();
        let status = unsafe {
            if cow.is_null() {
                sys::napi_create_string_utf8(env.raw(), c"".as_ptr(), 0, &mut value)
            } else {
                sys::napi_create_string_utf8(
                    env.raw(),
                    canvas_c::canvas_native_ccow_get_bytes(cow).cast(),
                    canvas_c::canvas_native_ccow_get_length(cow) as isize,
                    &mut value,
                )
            }
        };
        if !cow.is_null() {
            canvas_c::canvas_native_ccow_release(cow);
        }
        self.release(&env);
        check_status!(status)?;
        Ok(JsRaw(value))
    }

    fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
        self.release(&env);
        Err(err)
    }
}

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

    /// Lower case, as the V8 bindings (and the web) report it: `"utf-8"`.
    #[napi(getter)]
    pub fn encoding(&self) -> String {
        let decoder = unsafe { &*self.decoder };
        decoder.encoding().to_lowercase()
    }

    /// `decode(buffer)`: any `ArrayBuffer` or view (typed array, `DataView`), read in place;
    /// `""` without one.
    #[napi(ts_args_type = "buffer?: ArrayBuffer | ArrayBufferView", ts_return_type = "string")]
    pub fn decode(&self, env: &Env, buffer: Option<Unknown>) -> Result<JsRaw> {
        let bytes = match buffer {
            None => None,
            Some(buffer) => Some(JsBytes::from_unknown(&buffer).ok_or_else(|| Error::from_reason(NOT_A_BUFFER))?),
        };
        let bytes = bytes.as_ref().map_or(&[][..], JsBytes::as_slice);
        decode_to_js(env.raw(), self.decoder, bytes).map(JsRaw)
    }

    /// `decodeAsync(buffer)`: `decode` off the JS thread, as a Promise; rejects for a non-buffer.
    #[napi(ts_args_type = "buffer: ArrayBuffer | ArrayBufferView", ts_return_type = "Promise<string>")]
    pub fn decode_async(&self, buffer: Option<Unknown>) -> Result<AsyncTask<DecodeTask>> {
        let bytes = match buffer.as_ref().and_then(JsBytes::from_unknown) {
            Some(bytes) => Some(bytes.pin()?),
            None => None,
        };
        Ok(AsyncTask::new(DecodeTask {
            decoder: unsafe { (*self.decoder).clone() },
            bytes,
        }))
    }
}
