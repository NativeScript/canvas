//! The module-level `CanvasModule` functions the V8 bindings register in `CanvasJSIModule`
//! `install()` (`createImageBitmap`, `readFile`, `getMime`, `__addFontFamily`, `__addFontData`,
//! `__base64*`), and what the async ones share: a worker pool, byte arguments read in place
//! (and kept alive while a worker reads them), and class-instance checks on untyped arguments.

use std::ffi::{c_char, CStr, CString};
use std::marker::PhantomData;
use std::ptr;
use std::sync::{mpsc, Arc, Mutex, OnceLock};

use canvas_c::U8Buffer;
use napi::bindgen_prelude::{
  ArrayBuffer, AsyncTask, ClassInstance, FnArgs, FromNapiValue, Function, ToNapiValue, TypeName,
  Unknown, ValidateNapiValue,
};
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::{check_status, sys, Env, Error, JsValue, Result, Status, Task, ValueType};

use crate::c2d::image_data::ImageData;
use crate::c2d::CanvasRenderingContext2D;
use crate::image_asset::ImageAsset;
use crate::image_bitmap::{self, BitmapSource, ImageBitmap, Rect};

// ---------------------------------------------------------------------------------------------
// Worker pool
// ---------------------------------------------------------------------------------------------

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Runs `job` on a shared pool of worker threads (the V8 bindings' `WorkerPool`), one per core.
pub(crate) fn spawn(job: impl FnOnce() + Send + 'static) {
  static POOL: OnceLock<Mutex<mpsc::Sender<Job>>> = OnceLock::new();
  let pool = POOL.get_or_init(|| {
    let (sender, receiver) = mpsc::channel::<Job>();
    let receiver = Arc::new(Mutex::new(receiver));
    let threads = std::thread::available_parallelism()
      .map_or(4, |n| n.get())
      .max(2);
    for i in 0..threads {
      let receiver = Arc::clone(&receiver);
      let _ = std::thread::Builder::new()
        .name(format!("canvas-worker-{i}"))
        .spawn(move || loop {
          // The lock is held only while waiting for the next job.
          let job = match receiver.lock() {
            Ok(receiver) => receiver.recv(),
            Err(_) => return,
          };
          match job {
            // A panicking job must not take its worker down with it.
            Ok(job) => drop(std::panic::catch_unwind(std::panic::AssertUnwindSafe(job))),
            Err(_) => return,
          }
        });
    }
    Mutex::new(sender)
  });
  if let Ok(sender) = pool.lock() {
    let _ = sender.send(Box::new(job));
  }
}

// ---------------------------------------------------------------------------------------------
// Argument helpers
// ---------------------------------------------------------------------------------------------

/// `value` as a `#[napi]` class instance, if it is one (checked with `instanceof` and the type
/// tag, as napi-rs checks `Either` arguments), for arguments that take several kinds of object.
pub(crate) fn downcast<'a, T: 'a>(value: &Unknown<'a>) -> Option<ClassInstance<'a, T>>
where
  ClassInstance<'a, T>: FromNapiValue + ValidateNapiValue,
{
  let raw = value.value();
  unsafe {
    <ClassInstance<'a, T> as ValidateNapiValue>::validate(raw.env, raw.value).ok()?;
    <ClassInstance<'a, T> as FromNapiValue>::from_napi_value(raw.env, raw.value).ok()
  }
}

pub(crate) fn type_of(value: &Unknown) -> ValueType {
  value.get_type().unwrap_or(ValueType::Unknown)
}

/// `object[name]`, for option bags read field by field (the V8 bindings ignore fields of the
/// wrong type rather than throwing).
pub(crate) fn property<'a>(object: &Unknown<'a>, name: &CStr) -> Option<Unknown<'a>> {
  if type_of(object) != ValueType::Object {
    return None;
  }
  let raw = object.value();
  let mut value = ptr::null_mut();
  let status =
    unsafe { sys::napi_get_named_property(raw.env, raw.value, name.as_ptr(), &mut value) };
  (status == sys::Status::napi_ok).then(|| unsafe { Unknown::from_raw_unchecked(raw.env, value) })
}

pub(crate) fn as_string(value: &Unknown) -> Option<String> {
  (type_of(value) == ValueType::String).then(|| unsafe { value.cast::<String>() }.ok())?
}

pub(crate) fn as_number(value: &Unknown) -> Option<f64> {
  (type_of(value) == ValueType::Number).then(|| unsafe { value.cast::<f64>() }.ok())?
}

pub(crate) fn as_bool(value: &Unknown) -> Option<bool> {
  (type_of(value) == ValueType::Boolean).then(|| unsafe { value.cast::<bool>() }.ok())?
}

/// `Number(value)`, as the V8 bindings' `NumberValue` reads positional numbers.
fn to_number(value: Option<&Unknown>) -> f64 {
  let Some(value) = value else {
    return f64::NAN;
  };
  let raw = value.value();
  let mut number = ptr::null_mut();
  let mut out = f64::NAN;
  unsafe {
    if sys::napi_coerce_to_number(raw.env, raw.value, &mut number) == sys::Status::napi_ok {
      sys::napi_get_value_double(raw.env, number, &mut out);
    }
  }
  out
}

/// A value created on the JS thread and handed back as is (async task results).
pub struct JsRaw(pub(crate) sys::napi_value);

impl TypeName for JsRaw {
  fn type_name() -> &'static str {
    "unknown"
  }

  fn value_type() -> ValueType {
    ValueType::Unknown
  }
}

impl ToNapiValue for JsRaw {
  unsafe fn to_napi_value(_: sys::napi_env, val: Self) -> Result<sys::napi_value> {
    Ok(val.0)
  }
}

// ---------------------------------------------------------------------------------------------
// Byte arguments
// ---------------------------------------------------------------------------------------------

/// The bytes of an `ArrayBuffer` or any `ArrayBufferView` (typed array or `DataView`), read in
/// place: the view's window only, whatever its element type (the V8 bindings' `GetBufferBytes`).
pub struct JsBytes<'env> {
  env: sys::napi_env,
  value: sys::napi_value,
  data: *const u8,
  len: usize,
  _scope: PhantomData<&'env ()>,
}

fn element_size(kind: sys::napi_typedarray_type) -> usize {
  match kind {
    0..=2 => 1,  // Int8, Uint8, Uint8Clamped
    3 | 4 => 2,  // Int16, Uint16
    5..=7 => 4,  // Int32, Uint32, Float32
    8..=10 => 8, // Float64, BigInt64, BigUint64
    11 => 2,     // Float16
    _ => 1,
  }
}

/// `(data, byte length)` of a buffer or view, `None` for anything else.
unsafe fn buffer_info(
  env: sys::napi_env,
  value: sys::napi_value,
) -> Result<Option<(*const u8, usize)>> {
  let mut is = false;
  let mut data = ptr::null_mut();
  let mut arraybuffer = ptr::null_mut();
  let mut offset = 0;
  check_status!(unsafe { sys::napi_is_arraybuffer(env, value, &mut is) })?;
  if is {
    let mut len = 0;
    check_status!(unsafe { sys::napi_get_arraybuffer_info(env, value, &mut data, &mut len) })?;
    return Ok(Some((data as *const u8, len)));
  }
  check_status!(unsafe { sys::napi_is_typedarray(env, value, &mut is) })?;
  if is {
    let (mut kind, mut length) = (0, 0);
    check_status!(unsafe {
      sys::napi_get_typedarray_info(
        env,
        value,
        &mut kind,
        &mut length,
        &mut data,
        &mut arraybuffer,
        &mut offset,
      )
    })?;
    // `data` already points at the view's first element.
    return Ok(Some((data as *const u8, length * element_size(kind))));
  }
  check_status!(unsafe { sys::napi_is_dataview(env, value, &mut is) })?;
  if is {
    let mut len = 0;
    check_status!(unsafe {
      sys::napi_get_dataview_info(
        env,
        value,
        &mut len,
        &mut data,
        &mut arraybuffer,
        &mut offset,
      )
    })?;
    return Ok(Some((data as *const u8, len)));
  }
  Ok(None)
}

const NOT_A_BUFFER: &str = "The provided value is not of type '(ArrayBuffer or ArrayBufferView)'";

impl<'env> JsBytes<'env> {
  pub(crate) fn from_unknown(value: &Unknown<'env>) -> Option<Self> {
    let raw = value.value();
    unsafe { Self::from_napi_value(raw.env, raw.value) }.ok()
  }

  pub(crate) fn as_slice(&self) -> &[u8] {
    if self.data.is_null() || self.len == 0 {
      &[]
    } else {
      unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }
  }

  /// Keeps the buffer alive so another thread can read it; `release` it on the JS thread.
  pub(crate) fn pin(&self) -> Result<PinnedBytes> {
    let mut reference = ptr::null_mut();
    check_status!(unsafe { sys::napi_create_reference(self.env, self.value, 1, &mut reference) })?;
    Ok(PinnedBytes {
      reference,
      data: self.data,
      len: self.len,
    })
  }
}

impl TypeName for JsBytes<'_> {
  fn type_name() -> &'static str {
    "ArrayBuffer | ArrayBufferView"
  }

  fn value_type() -> ValueType {
    ValueType::Object
  }
}

impl ValidateNapiValue for JsBytes<'_> {
  unsafe fn validate(env: sys::napi_env, value: sys::napi_value) -> Result<sys::napi_value> {
    match unsafe { buffer_info(env, value) }? {
      Some(_) => Ok(ptr::null_mut()),
      None => Err(Error::new(Status::InvalidArg, NOT_A_BUFFER)),
    }
  }
}

impl FromNapiValue for JsBytes<'_> {
  unsafe fn from_napi_value(env: sys::napi_env, value: sys::napi_value) -> Result<Self> {
    match unsafe { buffer_info(env, value) }? {
      Some((data, len)) => Ok(Self {
        env,
        value,
        data,
        len,
        _scope: PhantomData,
      }),
      None => Err(Error::new(Status::InvalidArg, NOT_A_BUFFER)),
    }
  }
}

/// Bytes of a JS buffer a worker reads while a reference keeps the buffer alive (the V8
/// bindings hold the backing store). Must be `release`d on the JS thread.
pub(crate) struct PinnedBytes {
  reference: sys::napi_ref,
  data: *const u8,
  len: usize,
}

// The reference is only touched on the JS thread (`release`); the bytes are read-only.
unsafe impl Send for PinnedBytes {}

impl PinnedBytes {
  pub(crate) fn as_slice(&self) -> &[u8] {
    if self.data.is_null() || self.len == 0 {
      &[]
    } else {
      unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }
  }

  /// On the JS thread, once the bytes are no longer read.
  pub(crate) fn release(self, env: &Env) {
    unsafe { sys::napi_delete_reference(env.raw(), self.reference) };
  }
}

/// An `ArrayBuffer` over a canvas-c `U8Buffer` (no copy); the buffer is freed with it.
unsafe fn u8_buffer_to_arraybuffer(
  env: sys::napi_env,
  buffer: *mut U8Buffer,
) -> Result<sys::napi_value> {
  let env = Env::from_raw(env);
  let len = canvas_c::canvas_native_u8_buffer_get_length(buffer);
  if len == 0 {
    canvas_c::canvas_native_u8_buffer_release(buffer);
    return Ok(ArrayBuffer::from_data(&env, Vec::new())?.raw());
  }
  let data = canvas_c::canvas_native_u8_buffer_get_bytes_mut(buffer);
  let value = unsafe {
    ArrayBuffer::from_external(&env, data, len, buffer as usize, |_, buffer| {
      canvas_c::canvas_native_u8_buffer_release(buffer as *mut U8Buffer)
    })
  }?;
  Ok(value.raw())
}

// ---------------------------------------------------------------------------------------------
// createImageBitmap
// ---------------------------------------------------------------------------------------------

type BitmapCallback<'a> =
  Function<'a, FnArgs<(Option<String>, Option<ImageBitmap>)>, UnknownReturnValue>;

const NOT_DECODED: &str =
  "Failed to execute 'createImageBitmap' : The provided source could not be decoded";

fn bitmap_error(callback: &BitmapCallback, message: &str) -> Result<()> {
  callback.call(FnArgs::from((Some(message.to_owned()), None)))?;
  Ok(())
}

/// `createImageBitmap(source, callback)`, `(source, options, callback)`,
/// `(source, sx, sy, sw, sh, callback)` or `(source, sx, sy, sw, sh, options, callback)`, calling
/// `callback(null, bitmap)` or `callback(message, null)` as the V8 bindings do. Sources are the
/// native objects `packages/canvas` passes: encoded bytes (`ArrayBuffer` or any view), an
/// `ImageAsset`, an `ImageBitmap`, an `ImageData` (all decoded off the JS thread) or a 2D context
/// (snapshotted here, where its surface lives, and reported synchronously).
#[napi(
  js_name = "createImageBitmap",
  ts_args_type = "source: any, ...args: any[]",
  ts_return_type = "void"
)]
pub fn create_image_bitmap(
  image: Unknown,
  a1: Option<Unknown>,
  a2: Option<Unknown>,
  a3: Option<Unknown>,
  a4: Option<Unknown>,
  a5: Option<Unknown>,
  a6: Option<Unknown>,
) -> Result<()> {
  let args = [a1, a2, a3, a4, a5, a6];
  // The callback is the last argument; `len` counts the ones before it, the source included.
  let Some(last) = args.iter().rposition(Option::is_some) else {
    return Err(Error::from_reason("Illegal constructor"));
  };
  let callback = args[last].as_ref().unwrap();
  if type_of(callback) != ValueType::Function || type_of(&image) == ValueType::Function {
    return Err(Error::from_reason("Illegal constructor"));
  }
  let callback: BitmapCallback = unsafe { callback.cast() }?;
  let len = last + 1;

  if matches!(type_of(&image), ValueType::Null | ValueType::Undefined) {
    return bitmap_error(&callback, "Failed to load image");
  }

  let (options, rect) = match len {
    1 => (None, None),
    2 => (args[0].as_ref(), None),
    5 | 6 => {
      let [sx, sy, sw, sh] = [0, 1, 2, 3].map(|i| to_number(args[i].as_ref()));
      if sw == 0. {
        return bitmap_error(
          &callback,
          "Failed to execute 'createImageBitmap' : The crop rect width is 0",
        );
      }
      if sh == 0. {
        return bitmap_error(
          &callback,
          "Failed to execute 'createImageBitmap' : The crop rect height is 0",
        );
      }
      let rect: Rect = (sx as f32, sy as f32, sw as f32, sh as f32);
      (if len == 6 { args[4].as_ref() } else { None }, Some(rect))
    }
    _ => {
      return bitmap_error(
        &callback,
        "Failed to execute 'createImageBitmap' : Invalid argument count",
      )
    }
  };
  let options = image_bitmap::parse_options(options);

  let source = if let Some(bytes) = JsBytes::from_unknown(&image) {
    BitmapSource::Encoded(bytes.pin()?)
  } else if let Some(asset) = downcast::<ImageAsset>(&image) {
    BitmapSource::Asset(Arc::clone(&asset.asset))
  } else if let Some(bitmap) = downcast::<ImageBitmap>(&image) {
    BitmapSource::Asset(Arc::clone(&bitmap.asset))
  } else if let Some(data) = downcast::<ImageData>(&image) {
    // Shares the pixels (canvas-c's ImageData is reference counted).
    BitmapSource::ImageData(data.data.clone())
  } else if let Some(context) = downcast::<CanvasRenderingContext2D>(&image) {
    // Its pixels are read here, on the thread that owns its surface.
    context.flush_pending();
    return match image_bitmap::from_context(&context, rect, options) {
      Some(asset) => {
        callback.call(FnArgs::from((None, Some(ImageBitmap::new(asset)))))?;
        Ok(())
      }
      None => bitmap_error(&callback, NOT_DECODED),
    };
  } else {
    return bitmap_error(&callback, NOT_DECODED);
  };

  let tsfn = callback
    .build_threadsafe_function::<(BitmapSource, Option<Arc<canvas_c::ImageAsset>>)>()
    .build_callback(|ctx| {
      let (source, output) = ctx.value;
      source.release(&ctx.env);
      Ok(FnArgs::from(match output {
        Some(asset) => (None, Some(ImageBitmap::new(asset))),
        None => (Some(NOT_DECODED.to_owned()), None),
      }))
    })?;
  spawn(move || {
    let output = image_bitmap::decode(&source, rect, options);
    tsfn.call((source, output), ThreadsafeFunctionCallMode::NonBlocking);
  });
  Ok(())
}

// ---------------------------------------------------------------------------------------------
// readFile / getMime
// ---------------------------------------------------------------------------------------------

/// A file's bytes, as an `ArrayBuffer` (no copy).
pub struct FileBytes(*mut U8Buffer);

unsafe impl Send for FileBytes {}

impl Drop for FileBytes {
  fn drop(&mut self) {
    if !self.0.is_null() {
      canvas_c::canvas_native_u8_buffer_release(self.0);
    }
  }
}

impl TypeName for FileBytes {
  fn type_name() -> &'static str {
    "ArrayBuffer"
  }

  fn value_type() -> ValueType {
    ValueType::Object
  }
}

impl ToNapiValue for FileBytes {
  unsafe fn to_napi_value(env: sys::napi_env, mut val: Self) -> Result<sys::napi_value> {
    let buffer = std::mem::replace(&mut val.0, ptr::null_mut());
    unsafe { u8_buffer_to_arraybuffer(env, buffer) }
  }
}

/// `readFile`'s result: `{ buffer, mime?, extension? }`.
pub struct FileData {
  buffer: FileBytes,
  mime: Option<String>,
  extension: Option<String>,
}

impl TypeName for FileData {
  fn type_name() -> &'static str {
    "{ buffer: ArrayBuffer, mime?: string, extension?: string }"
  }

  fn value_type() -> ValueType {
    ValueType::Object
  }
}

impl ToNapiValue for FileData {
  unsafe fn to_napi_value(env: sys::napi_env, val: Self) -> Result<sys::napi_value> {
    let mut object = ptr::null_mut();
    check_status!(unsafe { sys::napi_create_object(env, &mut object) })?;
    let set = |name: &CStr, value: sys::napi_value| {
      check_status!(unsafe { sys::napi_set_named_property(env, object, name.as_ptr(), value) })
    };
    set(c"buffer", unsafe {
      FileBytes::to_napi_value(env, val.buffer)
    }?)?;
    if let Some(mime) = val.mime {
      set(c"mime", unsafe { String::to_napi_value(env, mime) }?)?;
    }
    if let Some(extension) = val.extension {
      set(c"extension", unsafe {
        String::to_napi_value(env, extension)
      }?)?;
    }
    Ok(object)
  }
}

unsafe fn take_c_string(value: *const c_char) -> Option<String> {
  (!value.is_null()).then(|| {
    unsafe { CString::from_raw(value as *mut c_char) }
      .to_string_lossy()
      .into_owned()
  })
}

/// On a worker: the file's bytes (and sniffed type), or the error message.
fn read(path: &str) -> std::result::Result<FileData, String> {
  let path = CString::new(path).map_err(|e| e.to_string())?;
  let file = canvas_c::canvas_native_helper_read_file(path.as_ptr());
  let ret = unsafe {
    if canvas_c::canvas_native_helper_read_file_has_error(file) {
      Err(
        take_c_string(canvas_c::canvas_native_helper_read_file_get_error(file)).unwrap_or_default(),
      )
    } else {
      Ok(FileData {
        mime: take_c_string(canvas_c::canvas_native_helper_read_file_get_mime(file)),
        extension: take_c_string(canvas_c::canvas_native_helper_read_file_get_extension(file)),
        buffer: FileBytes(canvas_c::canvas_native_helper_read_file_take_data(file)),
      })
    }
  };
  canvas_c::canvas_native_helper_release(file);
  ret
}

/// Reads `path` on the worker pool and calls `deliver(env, result)`'s arguments back on the JS
/// thread: `(null, value)` or `(Error, null)`.
fn read_file_with<V: ToNapiValue + Send + 'static>(
  path: String,
  callback: Function<'_, FnArgs<(Option<Error>, Option<V>)>, UnknownReturnValue>,
  map: fn(FileData) -> V,
) -> Result<()> {
  let tsfn = callback
    .build_threadsafe_function::<std::result::Result<FileData, String>>()
    .build_callback(move |ctx| {
      Ok(FnArgs::from(match ctx.value {
        Ok(file) => (None, Some(map(file))),
        Err(error) => (Some(Error::from_reason(error)), None),
      }))
    })?;
  spawn(move || {
    tsfn.call(read(&path), ThreadsafeFunctionCallMode::NonBlocking);
  });
  Ok(())
}

/// `readFile(path, callback(error, { buffer, mime?, extension? }))`, read off the JS thread;
/// `error` is an `Error`, as in the V8 bindings.
#[napi(
  js_name = "readFile",
  ts_args_type = "path: string, callback: (error: Error | null, result: { buffer: ArrayBuffer, mime?: string, extension?: string } | null) => void"
)]
pub fn read_file(
  path: String,
  callback: Function<FnArgs<(Option<Error>, Option<FileData>)>, UnknownReturnValue>,
) -> Result<()> {
  read_file_with(path, callback, |file| file)
}

/// `getMime(path, callback(error, ArrayBuffer))`: as the V8 bindings register it, this reads the
/// file like `readFile` and hands back its bytes only.
#[napi(
  js_name = "getMime",
  ts_args_type = "path: string, callback: (error: Error | null, buffer: ArrayBuffer | null) => void"
)]
pub fn get_mime(
  path: String,
  callback: Function<FnArgs<(Option<Error>, Option<FileBytes>)>, UnknownReturnValue>,
) -> Result<()> {
  read_file_with(path, callback, |file| file.buffer)
}

// ---------------------------------------------------------------------------------------------
// Fonts
// ---------------------------------------------------------------------------------------------

fn alias_of(alias: &Unknown) -> Option<CString> {
  as_string(alias).and_then(|alias| CString::new(alias).ok())
}

/// `__addFontFamily(alias | null, [paths])`: registers font files (non-string entries skipped).
#[napi(
  js_name = "__addFontFamily",
  ts_args_type = "alias: string | null, filenames: string[]"
)]
pub fn add_font_family(alias: Unknown, filenames: Unknown) -> Result<()> {
  let raw = filenames.value();
  let mut is_array = false;
  check_status!(unsafe { sys::napi_is_array(raw.env, raw.value, &mut is_array) })?;
  if !is_array {
    return Ok(());
  }
  let filenames = unsafe { filenames.cast::<Vec<Unknown>>() }?;
  let names: Vec<CString> = filenames
    .iter()
    .filter_map(as_string)
    .filter_map(|name| CString::new(name).ok())
    .collect();
  let pointers: Vec<*const c_char> = names.iter().map(|name| name.as_ptr()).collect();
  let alias = alias_of(&alias);
  canvas_c::canvas_native_font_add_family(
    alias.as_ref().map_or(ptr::null(), |alias| alias.as_ptr()),
    pointers.as_ptr(),
    pointers.len(),
  );
  Ok(())
}

/// `__addFontData(alias | null, data)`: registers a font from its bytes (`ArrayBuffer` or view).
#[napi(
  js_name = "__addFontData",
  ts_args_type = "alias: string | null, data: ArrayBuffer | ArrayBufferView"
)]
pub fn add_font_data(alias: Unknown, data: Unknown) {
  let Some(bytes) = JsBytes::from_unknown(&data) else {
    return;
  };
  let bytes = bytes.as_slice();
  if bytes.is_empty() {
    return;
  }
  let alias = alias_of(&alias);
  canvas_c::canvas_native_font_add_family_with_bytes(
    alias.as_ref().map_or(ptr::null(), |alias| alias.as_ptr()),
    bytes.as_ptr(),
    bytes.len(),
  );
}

// ---------------------------------------------------------------------------------------------
// Base64
// ---------------------------------------------------------------------------------------------

/// `__base64Encode(value)`: base64 of the string's UTF-8 bytes (`""` for `""`).
#[napi(js_name = "__base64Encode")]
pub fn base64_encode(value: String) -> String {
  if value.is_empty() {
    return String::new();
  }
  let encoded =
    unsafe { canvas_c::canvas_native_helper_base64_encode(value.as_ptr(), value.len()) };
  unsafe { take_c_string(encoded) }.unwrap_or_default()
}

/// A decoded base64 string, as the V8 bindings hand it back: `[binaryString, ArrayBuffer]` (one
/// Latin-1 character per byte, and the bytes themselves), or `""` if it could not be decoded.
pub struct Base64Decoded(Option<Vec<u8>>);

impl TypeName for Base64Decoded {
  fn type_name() -> &'static str {
    "[string, ArrayBuffer] | string"
  }

  fn value_type() -> ValueType {
    ValueType::Unknown
  }
}

impl ToNapiValue for Base64Decoded {
  unsafe fn to_napi_value(env: sys::napi_env, val: Self) -> Result<sys::napi_value> {
    let Some(bytes) = val.0 else {
      return unsafe { String::to_napi_value(env, String::new()) };
    };
    let mut text = ptr::null_mut();
    check_status!(unsafe {
      sys::napi_create_string_latin1(env, bytes.as_ptr().cast(), bytes.len() as isize, &mut text)
    })?;
    // The bytes move into the ArrayBuffer (no copy).
    let buffer = ArrayBuffer::from_data(&Env::from_raw(env), bytes)?.raw();
    let mut array = ptr::null_mut();
    check_status!(unsafe { sys::napi_create_array_with_length(env, 2, &mut array) })?;
    check_status!(unsafe { sys::napi_set_element(env, array, 0, text) })?;
    check_status!(unsafe { sys::napi_set_element(env, array, 1, buffer) })?;
    Ok(array)
  }
}

fn base64_decode_bytes(value: &str) -> Option<Vec<u8>> {
  canvas_c::canvas_native_helper_base64_decode_str(value)
}

/// `__base64Decode(value)`: `[binaryString, ArrayBuffer]`, or `""`.
#[napi(js_name = "__base64Decode")]
pub fn base64_decode(value: String) -> Base64Decoded {
  Base64Decoded(base64_decode_bytes(&value))
}

pub struct Base64DecodeTask(String);

impl Task for Base64DecodeTask {
  type Output = Option<Vec<u8>>;
  type JsValue = Base64Decoded;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(base64_decode_bytes(&self.0))
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Base64Decoded(output))
  }
}

/// `__base64DecodeAsync(value)`: `__base64Decode` off the JS thread, as a Promise.
#[napi(
  js_name = "__base64DecodeAsync",
  ts_return_type = "Promise<[string, ArrayBuffer] | string>"
)]
pub fn base64_decode_async(value: String) -> AsyncTask<Base64DecodeTask> {
  AsyncTask::new(Base64DecodeTask(value))
}
