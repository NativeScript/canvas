#![deny(clippy::all)]

use std::cell::Cell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Once;

use napi::{Error, Result};

mod buffer;
mod context;
mod node;
mod param;

pub use buffer::{AudioBuffer, PeriodicWave};
pub use context::AudioContext;
pub use node::AudioNode;
pub use param::AudioParam;

thread_local! {
  static GUARDED: Cell<bool> = const { Cell::new(false) };
}

/// web-audio-api panics on spec violations; a panic unwinding into Node-API aborts the app, so
/// every call into it returns the panic as an error instead ("<Name>Error: ...").
pub(crate) fn guard<T>(f: impl FnOnce() -> T) -> Result<T> {
  static QUIET_HOOK: Once = Once::new();
  QUIET_HOOK.call_once(|| {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
      if !GUARDED.get() {
        hook(info);
      }
    }));
  });
  let outer = GUARDED.replace(true);
  let result = catch_unwind(AssertUnwindSafe(f));
  GUARDED.set(outer);
  result.map_err(|payload| {
    let message = payload
      .downcast_ref::<&str>()
      .map(|message| message.to_string())
      .or_else(|| payload.downcast_ref::<String>().cloned())
      .unwrap_or_else(|| "The audio engine failed".to_owned());
    error(match message.split_once(" - ") {
      Some((name, rest)) if name.ends_with("Error") && !name.contains(' ') => format!("{name}: {rest}"),
      _ => message,
    })
  })
}

pub(crate) fn error(message: impl Into<String>) -> Error {
  Error::from_reason(message.into())
}
