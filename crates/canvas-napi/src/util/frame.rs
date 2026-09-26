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
