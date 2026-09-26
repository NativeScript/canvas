use std::cell::RefCell;
use std::sync::Arc;

/// A canvas-c object an early `destroy()` / `__releaseHandle()` can let go of before the wrapper
/// is collected (the V8 bindings' `ArcHandle::reset`): encoders, passes, command buffers and
/// views `packages/canvas` releases as soon as they are consumed. Once released, the wrapper's
/// methods see a null pointer, which canvas-c treats as a no-op.
pub(crate) struct Handle<T>(RefCell<Option<Arc<T>>>);

impl<T> Handle<T> {
  pub(crate) fn new(value: Arc<T>) -> Self {
    Self(RefCell::new(Some(value)))
  }

  /// Takes over a pointer canvas-c returned from `Arc::into_raw`; `None` for null.
  pub(crate) unsafe fn from_raw(value: *const T) -> Option<Self> {
    (!value.is_null()).then(|| Self::new(unsafe { Arc::from_raw(value) }))
  }

  /// The canvas-c pointer, null once released.
  pub(crate) fn ptr(&self) -> *const T {
    self
      .0
      .borrow()
      .as_ref()
      .map_or(std::ptr::null(), Arc::as_ptr)
  }

  pub(crate) fn release(&self) {
    let value = self.0.borrow_mut().take();
    drop(value);
  }
}
