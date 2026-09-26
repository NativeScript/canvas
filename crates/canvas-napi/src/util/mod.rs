pub mod class;
pub mod cx;
pub mod frame;
pub mod native;
pub mod ret;
pub mod task;

use std::panic::{catch_unwind, AssertUnwindSafe};

use napi::sys;

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        format!("canvas: internal error: {message}")
    } else if let Some(message) = payload.downcast_ref::<String>() {
        format!("canvas: internal error: {message}")
    } else {
        "canvas: internal error".to_string()
    }
}

/// Runs `f`, turning a Rust panic into a JS exception instead of unwinding into the engine.
#[inline]
pub fn guard<R>(env: sys::napi_env, f: impl FnOnce() -> R) -> Option<R> {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(value) => Some(value),
        Err(payload) => {
            ret::throw_error(env, &panic_message(payload));
            None
        }
    }
}

#[inline]
pub fn guard_value(env: sys::napi_env, f: impl FnOnce() -> sys::napi_value) -> sys::napi_value {
    guard(env, f).unwrap_or(std::ptr::null_mut())
}
