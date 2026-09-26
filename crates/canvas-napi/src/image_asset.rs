use canvas_c::{
  canvas_native_image_asset_load_from_path, canvas_native_image_asset_load_from_url,
  ImageAsset as CImageAsset,
};
use napi::bindgen_prelude::{AsyncTask, FnArgs, Function};
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::{Env, Error, JsString, Result, Task};
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

use crate::module::{spawn, JsBytes, PinnedBytes};

#[napi]
#[derive(Clone, Debug)]
pub struct ImageAsset {
  pub(crate) asset: Arc<CImageAsset>,
}

fn load_url(asset: &CImageAsset, url: &str) -> bool {
  match CString::new(url) {
    Ok(url) => canvas_native_image_asset_load_from_url(asset, url.as_ptr()),
    Err(_) => false,
  }
}

fn load_path(asset: &CImageAsset, path: &str) -> bool {
  match CString::new(path) {
    Ok(path) => canvas_native_image_asset_load_from_path(asset, path.as_ptr()),
    Err(_) => false,
  }
}

fn load_encoded(asset: &CImageAsset, bytes: &[u8]) -> bool {
  canvas_c::canvas_native_image_asset_load_from_raw_encoded(asset, bytes.as_ptr(), bytes.len())
}

/// RGBA8 pixels; `premultiplied` when they already are, or they are premultiplied twice.
fn load_raw(
  asset: &CImageAsset,
  width: u32,
  height: u32,
  bytes: &[u8],
  premultiplied: bool,
) -> bool {
  if premultiplied {
    canvas_c::canvas_native_image_asset_load_from_raw_premultiplied(
      asset,
      width,
      height,
      bytes.as_ptr(),
      bytes.len(),
    )
  } else {
    canvas_c::canvas_native_image_asset_load_from_raw(
      asset,
      width,
      height,
      bytes.as_ptr(),
      bytes.len(),
    )
  }
}

/// The V8 bindings call load callbacks with one argument: `done`.
type DoneCallback<'a> = Function<'a, bool, UnknownReturnValue>;
type SaveCallback<'a> = Function<'a, FnArgs<(bool, Option<String>)>, UnknownReturnValue>;

fn save(asset: &CImageAsset, path: &str, format: u32) -> bool {
  match CString::new(path) {
    Ok(path) => canvas_c::canvas_native_image_asset_save_path(asset, path.as_ptr(), format),
    Err(_) => false,
  }
}

/// Runs `load` on the worker pool, then `callback(done)` on the JS thread.
fn load_cb(callback: DoneCallback, load: impl FnOnce() -> bool + Send + 'static) -> Result<()> {
  let tsfn = callback.build_threadsafe_function::<bool>().build()?;
  spawn(move || {
    tsfn.call(load(), ThreadsafeFunctionCallMode::NonBlocking);
  });
  Ok(())
}

/// As `load_cb`, reading `bytes` in place; the buffer is kept alive until the load is done.
fn load_bytes_cb(
  callback: DoneCallback,
  bytes: PinnedBytes,
  load: impl FnOnce(&[u8]) -> bool + Send + 'static,
) -> Result<()> {
  let tsfn = callback
    .build_threadsafe_function::<(bool, PinnedBytes)>()
    .build_callback(|ctx| {
      let (done, bytes) = ctx.value;
      bytes.release(&ctx.env);
      Ok(done)
    })?;
  spawn(move || {
    let done = load(bytes.as_slice());
    tsfn.call((done, bytes), ThreadsafeFunctionCallMode::NonBlocking);
  });
  Ok(())
}

pub struct AsyncUrlTask {
  url: String,
  asset: Arc<CImageAsset>,
}

impl Task for AsyncUrlTask {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    CString::new(self.url.as_str()).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(load_url(&self.asset, &self.url))
  }

  fn resolve(&mut self, _: Env, done: bool) -> Result<Self::JsValue> {
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
    CString::new(self.path.as_str()).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(load_path(&self.asset, &self.path))
  }

  fn resolve(&mut self, _: Env, done: bool) -> Result<Self::JsValue> {
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
    Ok(load_encoded(&self.asset, &decoded))
  }

  fn resolve(&mut self, _: Env, done: bool) -> Result<Self::JsValue> {
    Ok(done)
  }
}

/// Raw or encoded bytes, read in place on a libuv worker; the buffer is released on the JS
/// thread when the promise settles.
pub struct AsyncBytesTask {
  bytes: Option<PinnedBytes>,
  encoded: bool,
  width: u32,
  height: u32,
  asset: Arc<CImageAsset>,
}

impl AsyncBytesTask {
  fn release(&mut self, env: &Env) {
    if let Some(bytes) = self.bytes.take() {
      bytes.release(env);
    }
  }
}

impl Task for AsyncBytesTask {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    let bytes = self.bytes.as_ref().map_or(&[][..], PinnedBytes::as_slice);
    Ok(if self.encoded {
      load_encoded(&self.asset, bytes)
    } else {
      load_raw(&self.asset, self.width, self.height, bytes, false)
    })
  }

  fn resolve(&mut self, env: Env, done: bool) -> Result<Self::JsValue> {
    self.release(&env);
    Ok(done)
  }

  fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
    self.release(&env);
    Err(err)
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

  /// The canvas-c asset pointer, as a decimal string.
  #[napi(getter, js_name = "__addr")]
  pub fn addr(&self) -> String {
    (Arc::as_ptr(&self.asset) as usize).to_string()
  }

  /// Same as `__addr`: the host APIs (Android `NSCImageAsset`, iOS `NSCImageAsset`) take the
  /// pointer back without taking ownership.
  #[napi(js_name = "__getRef")]
  pub fn get_ref(&self) -> String {
    self.addr()
  }

  /// Encodes the image to `path`; `format` is `ImageAssetSaveFormat` (0 JPG, 1 PNG).
  #[napi]
  pub fn save_sync(&self, path: String, format: u32) -> bool {
    save(&self.asset, &path, format)
  }

  /// `saveCb(path, format, callback(success, error))`, encoded and written off the JS thread;
  /// `error` is the asset's error message when it failed.
  #[napi(
    ts_args_type = "path: string, format: number, callback: (success: boolean, error?: string) => void"
  )]
  pub fn save_cb(&self, path: String, format: u32, callback: SaveCallback) -> Result<()> {
    let asset = Arc::clone(&self.asset);
    let tsfn = callback
      .build_threadsafe_function::<(bool, Option<String>)>()
      .build_callback(|ctx| Ok(FnArgs::from(ctx.value)))?;
    spawn(move || {
      let done = save(&asset, &path, format);
      let error = (!done).then(|| asset.error().to_string());
      tsfn.call((done, error), ThreadsafeFunctionCallMode::NonBlocking);
    });
    Ok(())
  }

  #[napi]
  pub fn from_url_sync(&self, url: String) -> bool {
    load_url(&self.asset, &url)
  }

  /// `fromUrlCb(url, callback(done))`, loaded off the JS thread.
  #[napi(ts_args_type = "url: string, callback: (done: boolean) => void")]
  pub fn from_url_cb(&self, url: String, callback: DoneCallback) -> Result<()> {
    let asset = Arc::clone(&self.asset);
    load_cb(callback, move || load_url(&asset, &url))
  }

  #[napi]
  pub fn from_file_sync(&self, path: String) -> bool {
    load_path(&self.asset, &path)
  }

  /// `fromFileCb(path, callback(done))`, loaded off the JS thread.
  #[napi(ts_args_type = "path: string, callback: (done: boolean) => void")]
  pub fn from_file_cb(&self, path: String, callback: DoneCallback) -> Result<()> {
    let asset = Arc::clone(&self.asset);
    load_cb(callback, move || load_path(&asset, &path))
  }

  /// RGBA8 pixels from any `ArrayBuffer` or view (read in place); pass `premultiplied` when they
  /// already are.
  #[napi(
    ts_args_type = "width: number, height: number, bytes: ArrayBuffer | ArrayBufferView, premultiplied?: boolean"
  )]
  pub fn from_bytes_sync(
    &self,
    width: u32,
    height: u32,
    bytes: JsBytes,
    premultiplied: Option<bool>,
  ) -> bool {
    load_raw(
      &self.asset,
      width,
      height,
      bytes.as_slice(),
      premultiplied.unwrap_or(false),
    )
  }

  /// `fromBytesCb(width, height, bytes, callback(done))`, loaded off the JS thread.
  #[napi(
    ts_args_type = "width: number, height: number, bytes: ArrayBuffer | ArrayBufferView, callback: (done: boolean) => void"
  )]
  pub fn from_bytes_cb(
    &self,
    width: u32,
    height: u32,
    bytes: JsBytes,
    callback: DoneCallback,
  ) -> Result<()> {
    let asset = Arc::clone(&self.asset);
    load_bytes_cb(callback, bytes.pin()?, move |bytes| {
      load_raw(&asset, width, height, bytes, false)
    })
  }

  /// An encoded image (PNG, JPEG, WebP, …) from any `ArrayBuffer` or view (read in place).
  #[napi(ts_args_type = "bytes: ArrayBuffer | ArrayBufferView")]
  pub fn from_encoded_bytes_sync(&self, bytes: JsBytes) -> bool {
    load_encoded(&self.asset, bytes.as_slice())
  }

  /// `fromEncodedBytesCb(bytes, callback(done))`, decoded off the JS thread.
  #[napi(ts_args_type = "bytes: ArrayBuffer | ArrayBufferView, callback: (done: boolean) => void")]
  pub fn from_encoded_bytes_cb(&self, bytes: JsBytes, callback: DoneCallback) -> Result<()> {
    let asset = Arc::clone(&self.asset);
    load_bytes_cb(callback, bytes.pin()?, move |bytes| {
      load_encoded(&asset, bytes)
    })
  }

  // Promise forms (not in the V8 bindings; `packages/canvas` uses the `*Cb` ones).

  #[napi(ts_return_type = "Promise<boolean>")]
  pub fn from_url(&self, url: String) -> AsyncTask<AsyncUrlTask> {
    AsyncTask::new(AsyncUrlTask {
      url,
      asset: Arc::clone(&self.asset),
    })
  }

  #[napi(ts_return_type = "Promise<boolean>")]
  pub fn from_file(&self, path: String) -> AsyncTask<AsyncFileTask> {
    AsyncTask::new(AsyncFileTask {
      path,
      asset: Arc::clone(&self.asset),
    })
  }

  #[napi(
    ts_args_type = "width: number, height: number, bytes: ArrayBuffer | ArrayBufferView",
    ts_return_type = "Promise<boolean>"
  )]
  pub fn from_bytes(
    &self,
    width: u32,
    height: u32,
    bytes: JsBytes,
  ) -> Result<AsyncTask<AsyncBytesTask>> {
    Ok(AsyncTask::new(AsyncBytesTask {
      bytes: Some(bytes.pin()?),
      asset: Arc::clone(&self.asset),
      width,
      height,
      encoded: false,
    }))
  }

  #[napi(
    ts_args_type = "bytes: ArrayBuffer | ArrayBufferView",
    ts_return_type = "Promise<boolean>"
  )]
  pub fn from_encoded_bytes(&self, bytes: JsBytes) -> Result<AsyncTask<AsyncBytesTask>> {
    Ok(AsyncTask::new(AsyncBytesTask {
      bytes: Some(bytes.pin()?),
      asset: Arc::clone(&self.asset),
      width: 0,
      height: 0,
      encoded: true,
    }))
  }

  #[napi]
  pub fn from_base_64_sync(&self, value: JsString) -> Result<bool> {
    let value = value.into_utf8()?;
    let value = value.as_str()?;
    let decoded = canvas_c::canvas_native_helper_base64_decode_str(value)
      .ok_or(Error::from_reason("Invalid Base64".to_owned()))?;
    Ok(load_encoded(&self.asset, &decoded))
  }

  #[napi(ts_return_type = "Promise<boolean>")]
  pub fn from_base64(&self, value: String) -> AsyncTask<AsyncBase64Task> {
    AsyncTask::new(AsyncBase64Task {
      base64: value,
      asset: Arc::clone(&self.asset),
    })
  }
}
