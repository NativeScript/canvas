//! Getting results from wgpu's threads back to JS.
//!
//! canvas-c reports adapters, devices, mappings and errors from its own threads (the request
//! threads, the mapping poller) and sometimes synchronously. Callback-shaped APIs
//! (`requestAdapter(options, cb)`, ...) go through napi-rs threadsafe functions, boxed as the
//! canvas-c `userdata` ([`into_userdata`] / [`deliver`]); promise-shaped ones (`mapAsync`,
//! `lost`) through [`PendingPromise`]. What may never fire (`device.lost`, the
//! `setuncapturederror` callback, see [`WeakCallback`]) never keeps the host alive, and the
//! persistent callback is not a GC root either.

use std::ffi::c_void;
use std::ptr;
use std::sync::{Arc, Mutex};

use napi::{check_status, sys, Env, Result};

/// A boxed one-shot delivery, the `userdata` canvas-c carries to its callback.
pub(crate) type Delivery<P> = Box<dyn FnOnce(P) + Send>;

/// `Box::into_raw` of a [`Delivery`], for canvas-c's `void *userdata`.
pub(crate) fn into_userdata<P: 'static>(deliver: Delivery<P>) -> *mut c_void {
  Box::into_raw(Box::new(deliver)) as *mut c_void
}

/// Runs the delivery `userdata` came from (once).
pub(crate) unsafe fn deliver<P: 'static>(userdata: *mut c_void, payload: P) {
  if userdata.is_null() {
    return;
  }
  let deliver = unsafe { Box::from_raw(userdata as *mut Delivery<P>) };
  deliver(payload);
}

/// How a pending promise ends.
pub(crate) enum Settled {
  Resolve(sys::napi_value),
  Reject(sys::napi_value),
}

type Settler = Box<dyn FnOnce(&Env) -> Result<Settled> + Send>;

/// What a threadsafe function's finalizer and its owner share: whether the function still
/// exists. At env teardown Node finalizes (and frees) threadsafe functions before the wrappers
/// holding them are finalized, so an owner must not call or release one that is gone (napi-rs's
/// `ThreadsafeFunction` guards itself the same way).
struct TsfnState {
  alive: Mutex<bool>,
  /// A (weak) reference to delete with the function, if any.
  reference: sys::napi_ref,
}

// `reference` is only touched on the JS thread (the finalizer, the call_js callback).
unsafe impl Send for TsfnState {}
unsafe impl Sync for TsfnState {}

unsafe extern "C" fn finalize_tsfn(env: sys::napi_env, data: *mut c_void, _hint: *mut c_void) {
  if data.is_null() {
    return;
  }
  let state = unsafe { Arc::from_raw(data as *const TsfnState) };
  match state.alive.lock() {
    Ok(mut alive) => *alive = false,
    Err(poisoned) => *poisoned.into_inner() = false,
  }
  if !env.is_null() && !state.reference.is_null() {
    unsafe { sys::napi_delete_reference(env, state.reference) };
  }
}

/// A threadsafe function without a JS function of its own (`call_js` does the work), callable
/// and releasable from any thread for as long as it exists.
struct Tsfn {
  raw: sys::napi_threadsafe_function,
  state: Arc<TsfnState>,
}

// Threadsafe functions are made to be called (and released) from any thread.
unsafe impl Send for Tsfn {}
unsafe impl Sync for Tsfn {}

impl Tsfn {
  /// `context` is handed to `call_js`; `None` hands it the shared state (for `reference`).
  /// `weak`: do not keep the event loop alive.
  fn new(
    env: &Env,
    context: Option<*mut c_void>,
    call_js: sys::napi_threadsafe_function_call_js,
    reference: sys::napi_ref,
    weak: bool,
  ) -> Result<Self> {
    let raw_env = env.raw();
    let state = Arc::new(TsfnState {
      alive: Mutex::new(true),
      reference,
    });
    let context = context.unwrap_or(Arc::as_ptr(&state) as *mut c_void);
    let mut name = ptr::null_mut();
    check_status!(unsafe {
      sys::napi_create_string_utf8(raw_env, c"canvas_webgpu".as_ptr(), -1, &mut name)
    })?;
    let finalize_data = Arc::into_raw(Arc::clone(&state)) as *mut c_void;
    let mut raw = ptr::null_mut();
    let status = unsafe {
      sys::napi_create_threadsafe_function(
        raw_env,
        ptr::null_mut(),
        ptr::null_mut(),
        name,
        0,
        1,
        finalize_data,
        Some(finalize_tsfn),
        context,
        call_js,
        &mut raw,
      )
    };
    if status != sys::Status::napi_ok {
      // Not created: take the finalizer's share back without deleting `reference` (the
      // caller still owns it).
      drop(unsafe { Arc::from_raw(finalize_data as *const TsfnState) });
      check_status!(status)?;
    }
    if weak {
      unsafe { sys::napi_unref_threadsafe_function(raw_env, raw) };
    }
    Ok(Self { raw, state })
  }

  /// Queues `data` for `call_js`; false (and `data` not taken) when that cannot happen.
  fn call(&self, data: *mut c_void) -> bool {
    let alive = match self.state.alive.lock() {
      Ok(alive) => alive,
      Err(poisoned) => poisoned.into_inner(),
    };
    *alive
      && unsafe {
        sys::napi_call_threadsafe_function(
          self.raw,
          data,
          sys::ThreadsafeFunctionCallMode::nonblocking,
        )
      } == sys::Status::napi_ok
  }
}

impl Drop for Tsfn {
  fn drop(&mut self) {
    let alive = match self.state.alive.lock() {
      Ok(alive) => alive,
      Err(poisoned) => poisoned.into_inner(),
    };
    if *alive {
      unsafe {
        sys::napi_release_threadsafe_function(self.raw, sys::ThreadsafeFunctionReleaseMode::release)
      };
    }
  }
}

/// A promise settled from any thread. Dropping it unsettled leaves the promise pending.
pub(crate) struct PendingPromise {
  tsfn: Tsfn,
}

unsafe extern "C" fn settle_on_js_thread(
  env: sys::napi_env,
  _js_callback: sys::napi_value,
  context: *mut c_void,
  data: *mut c_void,
) {
  if data.is_null() {
    return;
  }
  let settler = unsafe { Box::from_raw(data as *mut Settler) };
  // Queued work drained at teardown: there is nothing left to settle.
  if env.is_null() {
    return;
  }
  let deferred = context as sys::napi_deferred;
  let js_env = Env::from_raw(env);
  unsafe {
    match settler(&js_env) {
      Ok(Settled::Resolve(value)) => {
        sys::napi_resolve_deferred(env, deferred, value);
      }
      Ok(Settled::Reject(value)) => {
        sys::napi_reject_deferred(env, deferred, value);
      }
      Err(error) => {
        let value = js_error(env, &error.reason);
        sys::napi_reject_deferred(env, deferred, value);
      }
    }
  }
}

impl PendingPromise {
  /// A new promise and the handle that settles it. `weak`: do not keep the event loop alive
  /// while it is pending.
  pub(crate) fn new(env: &Env, weak: bool) -> Result<(Self, sys::napi_value)> {
    let mut deferred = ptr::null_mut();
    let mut promise = ptr::null_mut();
    check_status!(unsafe { sys::napi_create_promise(env.raw(), &mut deferred, &mut promise) })?;
    let tsfn = Tsfn::new(
      env,
      Some(deferred as *mut c_void),
      Some(settle_on_js_thread),
      ptr::null_mut(),
      weak,
    )?;
    Ok((Self { tsfn }, promise))
  }

  /// Settles the promise with what `settler` makes of it, on the JS thread.
  pub(crate) fn settle(self, settler: impl FnOnce(&Env) -> Result<Settled> + Send + 'static) {
    let data = Box::into_raw(Box::new(Box::new(settler) as Settler));
    if !self.tsfn.call(data as *mut c_void) {
      drop(unsafe { Box::from_raw(data) });
    }
    // Dropping `self` releases the threadsafe function; the queued call still runs.
  }

  pub(crate) fn resolve_undefined(self) {
    self.settle(|env| Ok(Settled::Resolve(undefined(env.raw()))));
  }

  pub(crate) fn reject_message(self, message: String) {
    self.settle(move |env| Ok(Settled::Reject(js_error(env.raw(), &message))));
  }
}

/// A promise already resolved with `value`.
pub(crate) fn resolved(env: &Env, value: sys::napi_value) -> Result<sys::napi_value> {
  let mut deferred = ptr::null_mut();
  let mut promise = ptr::null_mut();
  check_status!(unsafe { sys::napi_create_promise(env.raw(), &mut deferred, &mut promise) })?;
  check_status!(unsafe { sys::napi_resolve_deferred(env.raw(), deferred, value) })?;
  Ok(promise)
}

pub(crate) fn undefined(env: sys::napi_env) -> sys::napi_value {
  let mut value = ptr::null_mut();
  unsafe { sys::napi_get_undefined(env, &mut value) };
  value
}

pub(crate) fn null(env: sys::napi_env) -> sys::napi_value {
  let mut value = ptr::null_mut();
  unsafe { sys::napi_get_null(env, &mut value) };
  value
}

/// `new Error(message)`.
pub(crate) fn js_error(env: sys::napi_env, message: &str) -> sys::napi_value {
  let mut text = ptr::null_mut();
  let mut error = ptr::null_mut();
  unsafe {
    sys::napi_create_string_utf8(
      env,
      message.as_ptr().cast(),
      message.len() as isize,
      &mut text,
    );
    sys::napi_create_error(env, ptr::null_mut(), text, &mut error);
  }
  error
}

/// A persistent callback canvas-c may call from any thread, at any time, with no way to
/// unregister it: the slot itself is leaked (a few bytes per device) and its owner empties it
/// when it goes away, after which calls are dropped.
pub(crate) struct Slot<T: ?Sized>(Mutex<Option<Box<T>>>);

impl<T: ?Sized> Slot<T> {
  pub(crate) fn leak() -> &'static Slot<T> {
    Box::leak(Box::new(Slot(Mutex::new(None))))
  }

  pub(crate) fn set(&self, value: Option<Box<T>>) {
    let previous = match self.0.lock() {
      Ok(mut slot) => std::mem::replace(&mut *slot, value),
      Err(poisoned) => std::mem::replace(&mut *poisoned.into_inner(), value),
    };
    drop(previous);
  }

  pub(crate) fn take(&self) -> Option<Box<T>> {
    match self.0.lock() {
      Ok(mut slot) => slot.take(),
      Err(poisoned) => poisoned.into_inner().take(),
    }
  }

  pub(crate) fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
    let slot = match self.0.lock() {
      Ok(slot) => slot,
      Err(poisoned) => poisoned.into_inner(),
    };
    slot.as_deref().map(f)
  }
}

/// A JS callback native code calls from any thread without keeping it (or anything it
/// references) alive: the threadsafe function has no JS function of its own and holds a weak
/// reference, while the callback is kept as a hidden property of `holder` (the wrapper whose
/// lifetime it shares). `packages/canvas` passes `device._uncapturederror.bind(device)`, so a
/// strong reference would root the device forever.
pub(crate) struct WeakCallback {
  tsfn: Tsfn,
}

/// `(error type, message)`: what `setuncapturederror` callbacks receive.
pub(crate) type ErrorArgs = (u32, Option<String>);

unsafe extern "C" fn call_weak(
  env: sys::napi_env,
  _js_callback: sys::napi_value,
  context: *mut c_void,
  data: *mut c_void,
) {
  if data.is_null() {
    return;
  }
  let (kind, message) = *unsafe { Box::from_raw(data as *mut ErrorArgs) };
  // Drained at teardown (the state may be gone by then).
  if env.is_null() || context.is_null() {
    return;
  }
  unsafe {
    let reference = (*(context as *const TsfnState)).reference;
    let mut callback = ptr::null_mut();
    if sys::napi_get_reference_value(env, reference, &mut callback) != sys::Status::napi_ok
      || callback.is_null()
    {
      return;
    }
    let mut argv = [ptr::null_mut(); 2];
    sys::napi_create_uint32(env, kind, &mut argv[0]);
    argv[1] = match message {
      Some(message) => {
        let mut text = ptr::null_mut();
        sys::napi_create_string_utf8(
          env,
          message.as_ptr().cast(),
          message.len() as isize,
          &mut text,
        );
        text
      }
      None => null(env),
    };
    let mut result = ptr::null_mut();
    if sys::napi_call_function(env, undefined(env), callback, 2, argv.as_ptr(), &mut result)
      != sys::Status::napi_ok
    {
      // A throwing callback is an uncaught exception, as for any event handler (and as
      // napi-rs's threadsafe functions report it).
      let mut pending = false;
      sys::napi_is_exception_pending(env, &mut pending);
      if pending {
        let mut error = ptr::null_mut();
        sys::napi_get_and_clear_last_exception(env, &mut error);
        sys::napi_fatal_exception(env, error);
      }
    }
  }
}

impl WeakCallback {
  /// `holder[key] = callback` (non-enumerable) and a weak, loop-neutral way to call it.
  pub(crate) fn new(
    env: &Env,
    holder: sys::napi_value,
    key: &std::ffi::CStr,
    callback: sys::napi_value,
  ) -> Result<Self> {
    let raw = env.raw();
    let property = sys::napi_property_descriptor {
      utf8name: key.as_ptr(),
      name: ptr::null_mut(),
      method: None,
      getter: None,
      setter: None,
      value: callback,
      attributes: sys::PropertyAttributes::writable | sys::PropertyAttributes::configurable,
      data: ptr::null_mut(),
    };
    check_status!(unsafe { sys::napi_define_properties(raw, holder, 1, &property) })?;
    let mut reference = ptr::null_mut();
    check_status!(unsafe { sys::napi_create_reference(raw, callback, 0, &mut reference) })?;
    let tsfn = match Tsfn::new(env, None, Some(call_weak), reference, true) {
      Ok(tsfn) => tsfn,
      Err(error) => {
        unsafe { sys::napi_delete_reference(raw, reference) };
        return Err(error);
      }
    };
    Ok(Self { tsfn })
  }

  pub(crate) fn call(&self, args: ErrorArgs) {
    let data = Box::into_raw(Box::new(args));
    if !self.tsfn.call(data as *mut c_void) {
      drop(unsafe { Box::from_raw(data) });
    }
  }
}
