use std::ffi::{c_long, c_void};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

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
    data: Option<*mut c_void>,
    thread_id: Option<std::thread::ThreadId>,
}

pub struct Raf(
    Arc<parking_lot::Mutex<RafInner>>,
    Arc<AtomicUsize>,
    Arc<parking_lot::Condvar>,
);

impl Raf {
    extern "C" fn callback(frame_time_nanos: c_long, data: *mut std::os::raw::c_void) {
        if data.is_null() {
            return;
        }

        let data_ptr = data;
        let raf_ptr = data as *mut Raf;
        let raf_ref = unsafe { &*raf_ptr };

        {
            let mut lock = raf_ref.0.lock();
            if !lock.started {
                let stored = lock.data.take();
                drop(lock);
                if let Some(p) = stored {
                    let _ = unsafe { Box::from_raw(p as *mut Raf) };
                }
                return;
            }
            raf_ref.1.fetch_add(1, Ordering::SeqCst);
        }

        {
            let lock = raf_ref.0.lock();
            if let Some(callback) = lock.callback.as_ref() {
                callback(frame_time_nanos.into());
            }
        }

        raf_ref.1.fetch_sub(1, Ordering::SeqCst);

        raf_ref.2.notify_all();

        unsafe {
            let instance = AChoreographer_getInstance();
            let mut lock = raf_ref.0.lock();
            if lock.started {
                if lock.use_deprecated {
                    AChoreographer_postFrameCallback(instance, Some(Raf::callback), data_ptr);
                } else {
                    // post 64 variant if available
                }
            } else {
                let stored = lock.data.take();
                drop(lock);
                if let Some(p) = stored {
                    let _ = Box::from_raw(p as *mut Raf);
                }
            }
        }
    }

    pub fn new(callback: RafCallback) -> Self {
        Self(
            Arc::new(parking_lot::Mutex::new(RafInner {
                started: false,
                callback,
                is_prepared: false,
                use_deprecated: true, //*crate::API_LEVEL.get().unwrap_or(&-1) < 24,
                data: None,
                thread_id: None,
            })),
            Arc::new(AtomicUsize::new(0)),
            Arc::new(parking_lot::Condvar::new()),
        )
    }

    pub fn start(&self) {
        let mut lock = self.0.lock();
        if !lock.is_prepared {
            unsafe {
                ndk::looper::ThreadLooper::prepare();
            }
            lock.is_prepared = true;
        }

        lock.thread_id = Some(std::thread::current().id());
        unsafe {
            let instance = AChoreographer_getInstance();
            let data = Box::into_raw(Box::new(self.clone())) as *mut c_void;
            lock.data = Some(data);
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
        self.2.notify_all();
    }

    pub fn wait_until_idle(&self, timeout_ms: u64) -> bool {
        {
            let lock = self.0.lock();
            if let Some(tid) = lock.thread_id {
                if tid == std::thread::current().id() {
                    drop(lock);
                    self.clear_callback();
                    return true;
                }
            }
        }

        let start = Instant::now();
        let mut guard = self.0.lock();
        while self.1.load(Ordering::SeqCst) != 0 {
            let elapsed = start.elapsed();
            if elapsed >= Duration::from_millis(timeout_ms) {
                return false;
            }
            let remaining = Duration::from_millis(timeout_ms) - elapsed;
            let wait_dur = remaining.min(Duration::from_millis(50));
            self.2.wait_for(&mut guard, wait_dur);
        }
        true
    }

    pub fn clear_callback(&self) {
        let mut lock = self.0.lock();
        lock.callback = None;
        if self.1.load(Ordering::SeqCst) == 0 {
            if let Some(p) = lock.data.take() {
                drop(lock);
                let _ = unsafe { Box::from_raw(p as *mut Raf) };
            }
        }
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
        Self(
            Arc::clone(&self.0),
            Arc::clone(&self.1),
            Arc::clone(&self.2),
        )
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = Arc::clone(&source.0);
        self.1 = Arc::clone(&source.1);
        self.2 = Arc::clone(&source.2);
    }
}
