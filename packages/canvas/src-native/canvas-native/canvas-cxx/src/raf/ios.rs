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

pub struct Raf {
    inner: Arc<parking_lot::RwLock<RafInner>>,
}

unsafe impl Send for Raf {}
unsafe impl Sync for Raf {}

impl Raf {
    pub fn new(callback: RafCallback) -> Self {
        let inner = Arc::new(parking_lot::RwLock::new(RafInner {
            dl: None,
            started: false,
            callback,
        }));

        let clone = Arc::clone(&inner);

        {
            let mut lock = inner.write();
            lock.dl = DisplayLink::new(move |ts| {
                let lock = clone.read();
                if !lock.started {
                    return;
                }

                if let Some(callback) = lock.callback.as_ref() {
                    callback(ts.nanos_since_zero);
                }
            });
        }

        Self { inner }
    }

    pub fn start(&mut self) {
        let mut lock = self.inner.write();
        if let Some(dl) = lock.dl.as_mut() {
            let _ = dl.resume();
            lock.started = !dl.is_paused();
        }
    }

    pub fn stop(&mut self) {
        let mut lock = self.inner.write();
        if let Some(dl) = lock.dl.as_mut() {
            let _ = dl.pause();
            lock.started = !dl.is_paused();
        }
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
