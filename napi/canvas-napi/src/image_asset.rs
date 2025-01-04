use canvas_c::{
  canvas_native_image_asset_load_from_path, canvas_native_image_asset_load_from_url,
  ImageAsset as CImageAsset,
};
use napi::bindgen_prelude::{AsyncTask, Buffer, FromNapiValue, ObjectFinalize, ToNapiValue};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::{
  Either, Env, Error, JsArrayBuffer, JsBoolean, JsBuffer, JsFunction, JsObject, JsString, Result,
  Task,
};
use napi_derive::napi;
use std::ffi::{CStr, CString, FromBytesUntilNulError, NulError};
use std::sync::Arc;

#[napi]
#[derive(Clone, Debug)]
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
      Err(e) => Err(Error::from_reason(e.to_string()))?,
    }
  }

  fn resolve(&mut self, env: Env, done: bool) -> Result<Self::JsValue> {
    Ok(done)
  }
}

pub struct AsyncFileTask {
  path: String,
  asset: Arc<CImageAsset>,
}

impl Task for AsyncFileTask {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    match CString::new(self.path.as_str()) {
      Ok(path) => {
        let done =
          canvas_native_image_asset_load_from_path(Arc::as_ptr(&self.asset), path.as_ptr());
        Ok(done)
      }
      Err(e) => Err(Error::from_reason(e.to_string()))?,
    }
  }

  fn resolve(&mut self, env: Env, done: bool) -> Result<Self::JsValue> {
    Ok(done)
  }
}

pub struct AsyncBase64Task {
  base64: String,
  asset: Arc<CImageAsset>,
}

impl Task for AsyncBase64Task {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    let decoded = canvas_c::canvas_native_helper_base64_decode_str(self.base64.as_str())
      .ok_or(Error::from_reason("Invalid Base64".to_owned()))?;

    let loaded = canvas_c::canvas_native_image_asset_load_from_raw_encoded(
      Arc::as_ptr(&self.asset),
      decoded.as_ptr(),
      decoded.len(),
    );

    Ok(loaded)
  }

  fn resolve(&mut self, env: Env, done: bool) -> Result<Self::JsValue> {
    Ok(done)
  }
}

pub struct AsyncBytesTask {
  buffer: Buffer,
  encoded: bool,
  width: Option<u32>,
  height: Option<u32>,
  asset: Arc<CImageAsset>,
}

impl Task for AsyncBytesTask {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    let ret = if self.encoded {
      canvas_c::canvas_native_image_asset_load_from_raw(
        Arc::as_ptr(&self.asset),
        self.width.unwrap_or_default(),
        self.height.unwrap_or_default(),
        self.buffer.as_ptr(),
        self.buffer.len(),
      )
    } else {
      canvas_c::canvas_native_image_asset_load_from_raw_encoded(
        Arc::as_ptr(&self.asset),
        self.buffer.as_ptr(),
        self.buffer.len(),
      )
    };
    Ok(ret)
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
      asset: unsafe { Arc::from_raw(canvas_c::canvas_native_image_asset_create()) },
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

  #[napi]
  pub fn from_encoded_bytes_sync(&self, value: Either<&[u8], Buffer>) -> Result<bool> {
    let ret = match value {
      Either::A(buf) => canvas_c::canvas_native_image_asset_load_from_raw_encoded(
        Arc::as_ptr(&self.asset),
        buf.as_ptr(),
        buf.len(),
      ),
      Either::B(buf) => canvas_c::canvas_native_image_asset_load_from_raw_encoded(
        Arc::as_ptr(&self.asset),
        buf.as_ptr(),
        buf.len(),
      ),
    };

    Ok(ret)
  }

  #[napi]
  pub fn from_encoded_bytes(&self, value: Either<&[u8], Buffer>) -> AsyncTask<AsyncBytesTask> {
    match value {
      Either::A(buf) => AsyncTask::new(AsyncBytesTask {
        buffer: Buffer::from(buf),
        asset: Arc::clone(&self.asset),
        width: None,
        height: None,
        encoded: true,
      }),
      Either::B(buf) => AsyncTask::new(AsyncBytesTask {
        buffer: buf,
        asset: Arc::clone(&self.asset),
        width: None,
        height: None,
        encoded: true,
      }),
    }
  }

  #[napi]
  pub fn from_bytes_sync(
    &self,
    width: u32,
    height: u32,
    value: Either<&[u8], Buffer>,
  ) -> Result<bool> {
    let ret = match value {
      Either::A(buf) => canvas_c::canvas_native_image_asset_load_from_raw(
        Arc::as_ptr(&self.asset),
        width,
        height,
        buf.as_ptr(),
        buf.len(),
      ),
      Either::B(buf) => canvas_c::canvas_native_image_asset_load_from_raw(
        Arc::as_ptr(&self.asset),
        width,
        height,
        buf.as_ptr(),
        buf.len(),
      ),
    };
    Ok(ret)
  }

  #[napi]
  pub fn from_bytes(
    &self,
    width: u32,
    height: u32,
    value: Either<&[u8], Buffer>,
  ) -> AsyncTask<AsyncBytesTask> {
    match value {
      Either::A(buf) => AsyncTask::new(AsyncBytesTask {
        buffer: Buffer::from(buf),
        asset: Arc::clone(&self.asset),
        width: Some(width),
        height: Some(height),
        encoded: false,
      }),
      Either::B(buf) => AsyncTask::new(AsyncBytesTask {
        buffer: buf,
        asset: Arc::clone(&self.asset),
        width: Some(width),
        height: Some(height),
        encoded: false,
      }),
    }
  }

  #[napi]
  pub fn from_base_64_sync(&self, value: JsString) -> Result<bool> {
    let value = value.into_utf8()?;
    let value = value.as_str()?;
    let decoded = canvas_c::canvas_native_helper_base64_decode_str(value)
      .ok_or(Error::from_reason("Invalid Base64".to_owned()))?;
    let loaded = canvas_c::canvas_native_image_asset_load_from_raw_encoded(
      Arc::as_ptr(&self.asset),
      decoded.as_ptr(),
      decoded.len(),
    );
    Ok(loaded)
  }

  #[napi]
  pub fn from_base64(&self, value: String) -> AsyncTask<AsyncBase64Task> {
    AsyncTask::new(AsyncBase64Task {
      base64: value,
      asset: Arc::clone(&self.asset),
    })
  }


  pub fn from_url_sync(&self, url: JsString) -> bool {
    let asset = unsafe { &*self.asset };

    if let Some(url) = url.into_utf8().ok() {
      if let Ok(url) = url.as_str() {
        return match CString::new(url) {
          Ok(url) => canvas_c::canvas_native_image_asset_load_from_url(
            Arc::as_ptr(&self.asset),
            url.as_ptr(),
          ),
          Err(_) => false,
        };
      }
    }
    false
  }

  #[napi(ts_return_type = "Promise<boolean>")]
  pub fn from_url(&self, url: String) -> AsyncTask<AsyncUrlTask> {
    AsyncTask::new(AsyncUrlTask {
      url,
      asset: Arc::clone(&self.asset),
    })
  }

  #[napi]
  pub fn from_url_callback(&self, url: String, callback: JsFunction) -> Result<()> {
    let tsfn: ThreadsafeFunction<bool, ErrorStrategy::CalleeHandled> = callback
      .create_threadsafe_function(0, |ctx| ctx.env.get_boolean(ctx.value).map(|v| vec![v]))?;
    let tsfn = tsfn.clone();
    let asset = Arc::clone(&self.asset);
    std::thread::spawn(move || {
      let done = match CString::new(url) {
        Ok(url) => {
          canvas_c::canvas_native_image_asset_load_from_url(Arc::as_ptr(&asset), url.as_ptr())
        }
        Err(_) => false,
      };
      tsfn.call(Ok(done), ThreadsafeFunctionCallMode::NonBlocking);
    });
    Ok(())
  }

  #[napi]
  pub fn from_file_sync(&self, path: String) -> bool {
    match CString::new(path) {
      Ok(path) => {
        canvas_c::canvas_native_image_asset_load_from_path(Arc::as_ptr(&self.asset), path.as_ptr())
      }
      Err(_) => false,
    }
  }

  #[napi(ts_return_type = "Promise<boolean>")]
  pub fn from_file(&self, path: String) -> AsyncTask<AsyncFileTask> {
    AsyncTask::new(AsyncFileTask {
      path,
      asset: Arc::clone(&self.asset),
    })
  }
}
