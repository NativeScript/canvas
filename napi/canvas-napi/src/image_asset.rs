use canvas_c::{canvas_native_image_asset_load_from_url, ImageAsset as CImageAsset};
use napi::bindgen_prelude::{AsyncTask, FromNapiValue, ObjectFinalize, ToNapiValue};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::{Env, Error, JsBoolean, JsFunction, JsObject, JsString, Result, Task};
use napi_derive::napi;
use std::ffi::{CStr, CString, FromBytesUntilNulError, NulError};
use std::sync::Arc;

#[napi]
pub struct ImageAsset {
    pub(crate) asset: Arc<CImageAsset>,
}

pub struct AsyncUrlTask {
    url: String,
    asset: Arc<CImageAsset>,
}

impl Task for AsyncUrlTask {
    type Output = bool;
    type JsValue = bool;

    fn compute(&mut self) -> Result<Self::Output> {
        match CString::new(self.url.as_str()) {
            Ok(url) => {
                let done = canvas_native_image_asset_load_from_url(Arc::as_ptr(&self.asset), url.as_ptr());
                Ok(done)
            }
            Err(e) => {
                Err(Error::from_reason(e.to_string()))?
            }
        }
    }

    fn resolve(&mut self, env: Env, done: bool) -> Result<Self::JsValue> {
        Ok(done)
    }
}


#[napi]
impl ImageAsset {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            asset: unsafe {
                Arc::from_raw(canvas_c::canvas_native_image_asset_create())
            }
        }
    }

    #[napi(getter)]
    pub fn width(&self) -> u32 {
        self.asset.width()
    }

    #[napi(getter)]
    pub fn height(&self) -> u32 {
        self.asset.height()
    }

    #[napi(getter)]
    pub fn error(&self) -> String {
        self.asset.error().to_string()
    }


    #[napi(js_name = "fromUrlSync")]
    pub fn load_url_sync(&self, url: JsString) -> bool {
        let asset = unsafe { &*self.asset };

        if let Some(url) = url.into_utf8().ok() {
            if let Ok(url) = url.as_str() {
                return match CString::new(url) {
                    Ok(url) => {
                        canvas_c::canvas_native_image_asset_load_from_url(Arc::as_ptr(&self.asset), url.as_ptr())
                    }
                    Err(_) => false
                };
            }
        }
        false
    }

    #[napi(js_name = "fromUrl", ts_return_type = "Promise<boolean>")]
    pub fn load_url(&self, url: String) -> AsyncTask<AsyncUrlTask> {
        AsyncTask::new(
            AsyncUrlTask {
                url,
                asset: Arc::clone(&self.asset),
            })
    }


    #[napi]
    pub fn load_url_callback(&self, url: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<bool, ErrorStrategy::CalleeHandled> = callback
            .create_threadsafe_function(0, |ctx| {
                ctx.env.get_boolean(ctx.value).map(|v| vec![v])
            })?;
        let tsfn = tsfn.clone();
        let asset = Arc::clone(&self.asset);
        std::thread::spawn(move || {
            let done = match CString::new(url) {
                Ok(url) => {
                    canvas_c::canvas_native_image_asset_load_from_url(Arc::as_ptr(&asset), url.as_ptr())
                }
                Err(_) => false
            };
            tsfn.call(Ok(done), ThreadsafeFunctionCallMode::NonBlocking);
        });
        Ok(())
    }
}