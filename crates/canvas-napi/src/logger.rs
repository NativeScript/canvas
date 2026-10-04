//! Native log records (`log`: canvas-c, canvas-core, wgpu) on Node-API hosts, which have no
//! logcat / os_log: they go to the host's JS `console` (from any thread, through a threadsafe
//! function that never keeps the host alive) and, on Windows, to the debugger
//! (`OutputDebugString`).
//!
//! `CANVAS_LOG=error|warn|info|debug|trace` sets the level for every crate. By default the canvas
//! crates log warnings and the rest (wgpu, naga) errors.

use std::ffi::c_void;
use std::ptr;
use std::sync::{Mutex, OnceLock};

use log::{Level, LevelFilter, Log, Metadata, Record};
use napi::sys;

struct Console(sys::napi_threadsafe_function, std::thread::ThreadId);

// Only called through napi's threadsafe-function API, which is thread-safe.
unsafe impl Send for Console {}

static CONSOLE: Mutex<Option<Console>> = Mutex::new(None);

struct Logger {
  /// `CANVAS_LOG`, for every crate.
  level: Option<LevelFilter>,
}

impl Logger {
  fn threshold(&self, target: &str) -> LevelFilter {
    self.level.unwrap_or(if target.starts_with("canvas") {
      LevelFilter::Warn
    } else {
      LevelFilter::Error
    })
  }
}

impl Log for Logger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= self.threshold(metadata.target())
  }

  fn log(&self, record: &Record) {
    if !self.enabled(record.metadata()) {
      return;
    }
    let message = format!("[{}] {}", record.target(), record.args());
    #[cfg(target_os = "windows")]
    debugger_output(&message);

    let console = CONSOLE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(Console(function, _)) = console.as_ref() {
      let data = Box::into_raw(Box::new((record.level(), message)));
      let status = unsafe {
        sys::napi_call_threadsafe_function(*function, data as *mut c_void, sys::ThreadsafeFunctionCallMode::nonblocking)
      };
      if status != sys::Status::napi_ok {
        drop(unsafe { Box::from_raw(data) });
      }
    }
  }

  fn flush(&self) {}
}

#[cfg(target_os = "windows")]
fn debugger_output(message: &str) {
  #[link(name = "kernel32")]
  extern "system" {
    fn OutputDebugStringW(output: *const u16);
  }
  let wide: Vec<u16> = message.encode_utf16().chain([u16::from(b'\n'), 0]).collect();
  unsafe { OutputDebugStringW(wide.as_ptr()) };
}

/// `console.error` / `console.warn` / `console.log` with the record, on the JS thread.
unsafe extern "C" fn call_console(env: sys::napi_env, _: sys::napi_value, _: *mut c_void, data: *mut c_void) {
  let (level, message) = *unsafe { Box::from_raw(data as *mut (Level, String)) };
  if env.is_null() {
    return;
  }
  let method = match level {
    Level::Error => c"error",
    Level::Warn => c"warn",
    _ => c"log",
  };
  unsafe {
    let (mut global, mut console, mut function, mut text, mut result) =
      (ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    let _ = sys::napi_get_global(env, &mut global) == sys::Status::napi_ok
      && sys::napi_get_named_property(env, global, c"console".as_ptr(), &mut console) == sys::Status::napi_ok
      && sys::napi_get_named_property(env, console, method.as_ptr(), &mut function) == sys::Status::napi_ok
      && sys::napi_create_string_utf8(env, message.as_ptr() as *const _, message.len() as isize, &mut text)
        == sys::Status::napi_ok
      && sys::napi_call_function(env, console, function, 1, &text, &mut result) == sys::Status::napi_ok;
  }
}

/// Node finalizes the function at teardown; nothing may call it after.
unsafe extern "C" fn console_finalized(_: sys::napi_env, _: *mut c_void, _: *mut c_void) {
  CONSOLE.lock().unwrap_or_else(|e| e.into_inner()).take();
}

/// Routes `log` records to `env`'s console (the logger itself is installed once per process).
pub fn install(env: sys::napi_env) -> napi::Result<()> {
  static LOGGER: OnceLock<()> = OnceLock::new();
  LOGGER.get_or_init(|| {
    let level = std::env::var("CANVAS_LOG").ok().and_then(|level| level.parse().ok());
    if log::set_boxed_logger(Box::new(Logger { level })).is_ok() {
      log::set_max_level(level.unwrap_or(LevelFilter::Warn));
    }
  });

  let thread = std::thread::current().id();
  // A Worker's env keeps the main thread's console.
  if CONSOLE
    .lock()
    .unwrap_or_else(|e| e.into_inner())
    .as_ref()
    .is_some_and(|console| console.1 != thread)
  {
    return Ok(());
  }
  let mut function = ptr::null_mut();
  unsafe {
    let mut name = ptr::null_mut();
    napi::check_status!(sys::napi_create_string_utf8(env, c"canvasLog".as_ptr(), -1, &mut name))?;
    napi::check_status!(sys::napi_create_threadsafe_function(
      env,
      ptr::null_mut(),
      ptr::null_mut(),
      name,
      0,
      1,
      ptr::null_mut(),
      Some(console_finalized),
      ptr::null_mut(),
      Some(call_console),
      &mut function,
    ))?;
    // Logging never keeps the host alive.
    napi::check_status!(sys::napi_unref_threadsafe_function(env, function))?;
  }
  let previous = CONSOLE.lock().unwrap_or_else(|e| e.into_inner()).replace(Console(function, thread));
  if let Some(Console(previous, _)) = previous {
    unsafe { sys::napi_release_threadsafe_function(previous, sys::ThreadsafeFunctionReleaseMode::release) };
  }
  Ok(())
}
