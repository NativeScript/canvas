//! Dirty tracking and frame-end flushing for rendering contexts.
//!
//! On iOS/Android each context owns a display-link callback (`canvas_native_raf_*`) that flushes
//! it when dirty. Desktop hosts have no such thing per context: draw calls mark the context dirty,
//! the first mark queues it on this thread's pending list and asks the host for a frame, and the
//! host's frame-end hook calls [`flush_all`]. A context that is never drawn to costs nothing per
//! frame.

use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::rc::{Rc, Weak};

use napi::sys;

/// Per-context frame state. The owning wrapper holds the `Rc`; dropping it removes the context
/// from any pending flush.
pub struct FrameSlot {
  dirty: Cell<bool>,
  paused: Cell<bool>,
  target: *mut c_void,
  flush: unsafe fn(*mut c_void),
}

impl FrameSlot {
  /// `target` is the (stable, heap-allocated) canvas-c context `flush` renders.
  pub fn new(target: *mut c_void, flush: unsafe fn(*mut c_void)) -> Rc<FrameSlot> {
    Rc::new(FrameSlot {
      dirty: Cell::new(false),
      paused: Cell::new(false),
      target,
      flush,
    })
  }

  pub fn is_dirty(&self) -> bool {
    self.dirty.get()
  }

  /// `__stopRaf` / `__startRaf`: a paused context stays dirty but is not flushed.
  pub fn set_paused(self: &Rc<Self>, paused: bool) {
    self.paused.set(paused);
    if !paused && self.dirty.get() {
      enqueue(self);
    }
  }

  /// Renders now if dirty (e.g. before a readback).
  pub fn flush_now(&self) {
    if self.dirty.replace(false) {
      unsafe { (self.flush)(self.target) };
    }
  }
}

thread_local! {
  static PENDING: RefCell<Vec<Weak<FrameSlot>>> = const { RefCell::new(Vec::new()) };
  static SCHEDULER: RefCell<Option<Box<dyn Fn()>>> = const { RefCell::new(None) };
}

fn enqueue(slot: &Rc<FrameSlot>) {
  PENDING.with(|p| p.borrow_mut().push(Rc::downgrade(slot)));
  SCHEDULER.with(|s| {
    if let Some(request_frame) = s.borrow().as_ref() {
      request_frame();
    }
  });
}

/// Marks the context dirty; the first mark since the last flush queues it.
#[inline]
pub fn mark_dirty(slot: &Rc<FrameSlot>) {
  if !slot.dirty.replace(true) && !slot.paused.get() {
    enqueue(slot);
  }
}

/// Flushes every queued, still-alive, dirty and unpaused context. Called by the host at the end
/// of a frame (and by `CanvasModule.__flushAll()`).
pub fn flush_all() {
  let pending = PENDING.with(|p| std::mem::take(&mut *p.borrow_mut()));
  for slot in pending.iter().filter_map(Weak::upgrade) {
    if !slot.paused.get() {
      slot.flush_now();
    }
  }
}

/// How the host is asked for a frame when a context becomes dirty. Hosts without one leave it
/// unset and flush explicitly.
pub fn set_scheduler(request_frame: Option<Box<dyn Fn()>>) {
  SCHEDULER.with(|s| *s.borrow_mut() = request_frame);
}

/// `CanvasModule.__flushAll()`: flush every dirty context now (tests, and hosts without frame hooks).
#[napi_derive::napi(js_name = "__flushAll")]
pub fn flush_all_js() {
  flush_all();
}

/// The default scheduler: the first context dirtied in a JS turn queues one microtask that
/// flushes everything, so a turn (a rAF batch, an event handler) presents once, after all of
/// its drawing. Works on any Node-API host; a host with frame-end hooks can replace it.
struct MicrotaskScheduler {
  env: sys::napi_env,
  queue_microtask: sys::napi_ref,
  flush: sys::napi_ref,
  queued: Cell<bool>,
}

thread_local! {
  static MICROTASK: RefCell<Option<Rc<MicrotaskScheduler>>> = const { RefCell::new(None) };
}

unsafe extern "C" fn microtask_flush(env: sys::napi_env, _: sys::napi_callback_info) -> sys::napi_value {
  if let Some(scheduler) = MICROTASK.with(|m| m.borrow().clone()) {
    scheduler.queued.set(false);
  }
  flush_all();
  let mut undefined = std::ptr::null_mut();
  unsafe { sys::napi_get_undefined(env, &mut undefined) };
  undefined
}

impl MicrotaskScheduler {
  fn request(&self) {
    if self.queued.replace(true) {
      return;
    }
    unsafe {
      let (mut queue_microtask, mut flush, mut global, mut result) =
        (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
      let ok = sys::napi_get_reference_value(self.env, self.queue_microtask, &mut queue_microtask) == sys::Status::napi_ok
        && sys::napi_get_reference_value(self.env, self.flush, &mut flush) == sys::Status::napi_ok
        && sys::napi_get_global(self.env, &mut global) == sys::Status::napi_ok
        && sys::napi_call_function(self.env, global, queue_microtask, 1, &flush, &mut result) == sys::Status::napi_ok;
      if !ok {
        // Nothing will flush; let the next dirty context try again.
        self.queued.set(false);
      }
    }
  }
}

unsafe extern "C" fn microtask_teardown(_: *mut c_void) {
  MICROTASK.with(|m| m.borrow_mut().take());
  set_scheduler(None);
}

/// Installs the microtask scheduler for `env`'s thread (no-op without `queueMicrotask`).
pub fn install_microtask_scheduler(env: sys::napi_env) -> napi::Result<()> {
  unsafe {
    let (mut global, mut queue_microtask, mut flush) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
    napi::check_status!(sys::napi_get_global(env, &mut global))?;
    napi::check_status!(sys::napi_get_named_property(env, global, c"queueMicrotask".as_ptr(), &mut queue_microtask))?;
    let mut kind = 0;
    napi::check_status!(sys::napi_typeof(env, queue_microtask, &mut kind))?;
    if kind != sys::ValueType::napi_function {
      return Ok(());
    }
    napi::check_status!(sys::napi_create_function(
      env,
      c"__canvasFlush".as_ptr(),
      -1, // NAPI_AUTO_LENGTH: the name is NUL-terminated
      Some(microtask_flush),
      std::ptr::null_mut(),
      &mut flush,
    ))?;
    let (mut queue_ref, mut flush_ref) = (std::ptr::null_mut(), std::ptr::null_mut());
    napi::check_status!(sys::napi_create_reference(env, queue_microtask, 1, &mut queue_ref))?;
    napi::check_status!(sys::napi_create_reference(env, flush, 1, &mut flush_ref))?;
    napi::check_status!(sys::napi_add_env_cleanup_hook(env, Some(microtask_teardown), std::ptr::null_mut()))?;

    let scheduler = Rc::new(MicrotaskScheduler {
      env,
      queue_microtask: queue_ref,
      flush: flush_ref,
      queued: Cell::new(false),
    });
    MICROTASK.with(|m| *m.borrow_mut() = Some(scheduler.clone()));
    set_scheduler(Some(Box::new(move || scheduler.request())));
  }
  Ok(())
}
