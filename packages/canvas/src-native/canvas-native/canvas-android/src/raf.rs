use std::ffi::c_long;
use std::sync::Arc;

use crate::choregrapher::{
    AChoreographer, AChoreographer_frameCallback64, AChoreographer_getInstance,
    AChoreographer_postFrameCallback, AChoreographer_postFrameCallback64,
};

type RafCallback = Option<Box<dyn Fn(i64)>>;

struct RafInner {
    started: bool,
    callback: RafCallback,
    use_deprecated: bool,
}

pub struct Raf {
    inner: Arc<parking_lot::Mutex<RafInner>>,
}

impl Raf {
    extern "C" fn callback(frame_time_nanos: c_long, data: *mut std::os::raw::c_void) {
        if !data.is_null() {
            let data_ptr = data;
            let data = data as *mut Raf;
            let data = unsafe { &mut *data };
            let lock = data.inner.lock();
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
                    AChoreographer_postFrameCallback64(instance, Some(Raf::callback), data_ptr);
                }
            }
        }
    }

    pub fn new(callback: RafCallback) -> Self {
        Self {
            inner: Arc::new(parking_lot::Mutex::new(RafInner {
                started: false,
                callback,
                use_deprecated: *crate::API_LEVEL.get().unwrap_or(&-1) < 24,
            })),
        }
    }

    pub fn start(&mut self) {
        unsafe {
            ndk::looper::ThreadLooper::prepare();
            let instance = AChoreographer_getInstance();
            let data = Box::into_raw(Box::new(self.clone()));
            let lock = self.inner.lock();
            if lock.use_deprecated {
                AChoreographer_postFrameCallback(instance, Some(Raf::callback), data.cast());
            } else {
                AChoreographer_postFrameCallback64(instance, Some(Raf::callback), data.cast());
            }
        }
        let mut lock = self.inner.lock();
        lock.started = true;
    }

    pub fn stop(&mut self) {
        let mut lock = self.inner.lock();
        lock.started = false;
    }

    pub fn set_callback(&mut self, callback: RafCallback) {
        let mut lock = self.inner.lock();
        lock.callback = callback;
    }

    pub fn started(&self) -> bool {
        let mut started = false;
        {
            let lock = self.inner.lock();
            started = lock.started;
        }
        started
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
