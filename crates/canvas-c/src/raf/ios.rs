use std::ffi::{c_long, c_longlong, c_void};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

use display_link::DisplayLink;

type RafCallback = Option<Box<dyn Fn(i64)>>;

struct RafInner {
    started: bool,
    dl: Option<DisplayLink>,
    callback: RafCallback,
}

unsafe impl Send for RafInner {}

unsafe impl Sync for RafInner {}

pub struct Raf(Arc<parking_lot::Mutex<RafInner>>);

unsafe impl Send for Raf {}

unsafe impl Sync for Raf {}

impl Raf {
    pub fn new(callback: RafCallback) -> Self {
        let inner = Arc::new(parking_lot::Mutex::new(RafInner {
            dl: None,
            started: false,
            callback,
        }));

        let clone = Arc::clone(&inner);

        {
            let mut lock = inner.lock();
            lock.dl = DisplayLink::new(move |ts| {
                let lock = clone.lock();
                if !lock.started {
                    return;
                }

                if let Some(callback) = lock.callback.as_ref() {
                    callback(ts.nanos_since_zero);
                }
            });
        }

        Self(inner)
    }

    pub fn start(&self) {
        let mut lock = self.0.lock();
        if let Some(dl) = lock.dl.as_mut() {
            let _ = dl.resume();
            lock.started = !dl.is_paused();
        }
    }

    pub fn stop(&self) {
        let mut lock = self.0.lock();
        if let Some(dl) = lock.dl.as_mut() {
            let _ = dl.pause();
            lock.started = !dl.is_paused();
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
        Self ( Arc::clone(&self.0))
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = Arc::clone(&source.0)
    }
}
