use std::ffi::{c_long, c_void};
use std::sync::Arc;

use crate::choreographer::{
    AChoreographer_getInstance,
    AChoreographer_postFrameCallback,
    // AChoreographer_postFrameCallback64,
};

type RafCallback = Option<Box<dyn Fn(i64)>>;

struct RafInner {
    started: bool,
    callback: RafCallback,
    use_deprecated: bool,
    is_prepared: bool
}
pub struct Raf {
    inner: Arc<parking_lot::RwLock<RafInner>>,
}

impl Raf {
    extern "C" fn callback(frame_time_nanos: c_long, data: *mut std::os::raw::c_void) {
        if !data.is_null() {
            let data_ptr = data;
            let data = data as *mut Raf;
            let data_value = unsafe { &mut *data };
            let lock = data_value.inner.read();
            let started = lock.started;
            if !started {
                drop(lock);
                let _ = unsafe { Box::from_raw(data) };
                return;
            }
            if let Some(callback) = lock.callback.as_ref() {
                callback(frame_time_nanos.into());
            }

            unsafe {
                let instance = AChoreographer_getInstance();

                if lock.use_deprecated {
                    AChoreographer_postFrameCallback(instance, Some(Raf::callback), data_ptr);
                    return;
                } else {
                    //  #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
                    //    AChoreographer_postFrameCallback64(instance, Some(Raf::callback), data_ptr);
                }
            }
        }
    }

    pub fn new(callback: RafCallback) -> Self {
        Self {
            inner: Arc::new(parking_lot::RwLock::new(RafInner {
                started: false,
                callback,
                is_prepared: false,
                use_deprecated: true, //*crate::API_LEVEL.get().unwrap_or(&-1) < 24,
            })),
        }
    }

    pub fn start(&mut self) {
        let mut lock = self.inner.write();
        unsafe {
            if !lock.is_prepared {
                ndk::looper::ThreadLooper::prepare();
                lock.is_prepared = true;
            }
            let instance = AChoreographer_getInstance();
            let data = Box::into_raw(Box::new(self.clone())) as *mut c_void;
            if lock.use_deprecated {
                AChoreographer_postFrameCallback(instance, Some(Raf::callback), data);
            } else {
                //   #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
                //    AChoreographer_postFrameCallback64(instance, Some(Raf::callback), data.cast());
            }
        }
        lock.started = true;
    }

    pub fn stop(&mut self) {
        let mut lock = self.inner.write();
        lock.started = false;
    }

    pub fn set_callback(&mut self, callback: RafCallback) {
        let mut lock = self.inner.write();
        lock.callback = callback;
    }

    pub fn started(&self) -> bool {
        self.inner.read().started
    }
}

impl Clone for Raf {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner = Arc::clone(&source.inner)
    }
}
