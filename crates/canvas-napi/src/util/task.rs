//! Background work that completes on the JS thread.
//!
//! The V8 bindings run decode/IO work on a `WorkerPool` and post the result back through the
//! platform run loop (`AsyncCallback.h`). Here the pool is a few std threads and the way back is a
//! Node-API threadsafe function, which every Node-API host drains on its JS thread.

use std::ffi::c_void;
use std::ptr;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Mutex, OnceLock};

use napi::sys;

type Job = Box<dyn FnOnce() + Send>;

fn pool() -> &'static Mutex<Sender<Job>> {
    static POOL: OnceLock<Mutex<Sender<Job>>> = OnceLock::new();
    POOL.get_or_init(|| {
        let (tx, rx) = channel::<Job>();
        let rx = std::sync::Arc::new(Mutex::new(rx));
        let workers = std::thread::available_parallelism().map_or(2, |n| n.get()).clamp(2, 4);
        for i in 0..workers {
            let rx = rx.clone();
            let _ = std::thread::Builder::new()
                .name(format!("canvas-worker-{i}"))
                .spawn(move || loop {
                    let job = match rx.lock() {
                        Ok(rx) => rx.recv(),
                        Err(_) => return,
                    };
                    match job {
                        Ok(job) => {
                            // A panicking job must not take the worker down with it.
                            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(job));
                        }
                        Err(_) => return,
                    }
                });
        }
        Mutex::new(tx)
    })
}

/// Runs `job` on the worker pool.
pub fn spawn(job: impl FnOnce() + Send + 'static) {
    if let Ok(tx) = pool().lock() {
        let _ = tx.send(Box::new(job));
    }
}

type Completion = Box<dyn FnOnce(sys::napi_env, sys::napi_value) + Send>;

/// A one-shot way back to the JS thread, optionally carrying a JS function to call there.
/// Keeps the host's event loop alive until it is used (or dropped).
pub struct JsThread {
    tsfn: sys::napi_threadsafe_function,
}

// The threadsafe-function handle is designed to be used from any thread.
unsafe impl Send for JsThread {}

unsafe extern "C" fn call_js(env: sys::napi_env, js_callback: sys::napi_value, _context: *mut c_void, data: *mut c_void) {
    let completion = Box::from_raw(data as *mut Completion);
    // A null env means the environment is tearing down: drop the work without running JS.
    if !env.is_null() {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| completion(env, js_callback)));
    }
}

impl JsThread {
    /// `callback` may be null when the completion only settles a promise.
    pub unsafe fn new(env: sys::napi_env, callback: sys::napi_value) -> Option<JsThread> {
        let mut name = ptr::null_mut();
        sys::napi_create_string_utf8(env, c"canvas".as_ptr(), 6, &mut name);
        let mut tsfn = ptr::null_mut();
        let status = sys::napi_create_threadsafe_function(
            env,
            callback,
            ptr::null_mut(),
            name,
            0,
            1,
            ptr::null_mut(),
            None,
            ptr::null_mut(),
            Some(call_js),
            &mut tsfn,
        );
        (status == sys::Status::napi_ok).then_some(JsThread { tsfn })
    }

    /// Runs `completion(env, callback)` on the JS thread.
    pub fn complete(self, completion: impl FnOnce(sys::napi_env, sys::napi_value) + Send + 'static) {
        let data: Box<Completion> = Box::new(Box::new(completion));
        let data = Box::into_raw(data) as *mut c_void;
        unsafe {
            if sys::napi_call_threadsafe_function(self.tsfn, data, sys::ThreadsafeFunctionCallMode::nonblocking)
                != sys::Status::napi_ok
            {
                drop(Box::from_raw(data as *mut Completion));
            }
        }
        // Drop releases the handle; the queued call still runs.
    }
}

impl Drop for JsThread {
    fn drop(&mut self) {
        unsafe {
            sys::napi_release_threadsafe_function(self.tsfn, sys::ThreadsafeFunctionReleaseMode::release);
        }
    }
}

/// A pending promise; settle it on the JS thread only.
pub struct Deferred(sys::napi_deferred);

// Only settled from `JsThread` completions, which run on the JS thread.
unsafe impl Send for Deferred {}

impl Deferred {
    pub unsafe fn new(env: sys::napi_env) -> (sys::napi_value, Deferred) {
        let mut deferred = ptr::null_mut();
        let mut promise = ptr::null_mut();
        sys::napi_create_promise(env, &mut deferred, &mut promise);
        (promise, Deferred(deferred))
    }

    pub unsafe fn resolve(self, env: sys::napi_env, value: sys::napi_value) {
        let value = if value.is_null() { super::ret::undefined_value(env) } else { value };
        sys::napi_resolve_deferred(env, self.0, value);
    }

    pub unsafe fn reject(self, env: sys::napi_env, message: &str) {
        let mut msg = ptr::null_mut();
        let mut error = ptr::null_mut();
        sys::napi_create_string_utf8(env, message.as_ptr() as *const _, message.len() as isize, &mut msg);
        sys::napi_create_error(env, ptr::null_mut(), msg, &mut error);
        sys::napi_reject_deferred(env, self.0, error);
    }
}

/// Runs `work` on the pool and hands its result to `done` on the JS thread, with `callback`.
pub unsafe fn run<R: Send + 'static>(
    env: sys::napi_env,
    callback: sys::napi_value,
    work: impl FnOnce() -> R + Send + 'static,
    done: impl FnOnce(sys::napi_env, sys::napi_value, R) + Send + 'static,
) -> bool {
    let Some(thread) = JsThread::new(env, callback) else { return false };
    spawn(move || {
        let result = work();
        thread.complete(move |env, callback| done(env, callback, result));
    });
    true
}

/// Calls a JS function with `args`, `this` = undefined.
pub unsafe fn call(env: sys::napi_env, func: sys::napi_value, args: &[sys::napi_value]) {
    if func.is_null() {
        return;
    }
    let mut global = ptr::null_mut();
    sys::napi_get_global(env, &mut global);
    let mut result = ptr::null_mut();
    sys::napi_call_function(env, global, func, args.len(), args.as_ptr(), &mut result);
}
