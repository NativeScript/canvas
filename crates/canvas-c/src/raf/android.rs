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
    is_prepared: bool,
}
pub struct Raf(Arc<parking_lot::Mutex<RafInner>>);

impl Raf {
    extern "C" fn callback(frame_time_nanos: c_long, data: *mut std::os::raw::c_void) {
        if !data.is_null() {
            let data_ptr = data;
            let data = data as *const Raf;
            let data_value = unsafe { &*data };
            let lock = data_value.0.lock();
            let started = lock.started;
            if !started {
                drop(lock);
                let _ = unsafe { Arc::from_raw(data) };
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
        Self(Arc::new(parking_lot::Mutex::new(RafInner {
            started: false,
            callback,
            is_prepared: false,
            use_deprecated: true, //*crate::API_LEVEL.get().unwrap_or(&-1) < 24,
        })))
    }

    pub fn start(&self) {
        let mut lock = self.0.lock();
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

    pub fn stop(&self) {
        let mut lock = self.0.lock();
        lock.started = false;
    }

    pub fn set_callback(&self, callback: RafCallback) {
        let mut lock = self.0.lock();
        lock.callback = callback;
    }

    pub fn started(&self) -> bool {
        self.0.lock().started
    }
}

impl Clone for Raf {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = Arc::clone(&source.0)
    }
}
