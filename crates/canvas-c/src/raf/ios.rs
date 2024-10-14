use display_link::DisplayLink;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

type RafCallback = Option<Box<dyn Fn(i64)>>;


struct RafInner {
    dl: Option<DisplayLink>,
    callback: RafCallback,
}

unsafe impl Send for RafInner {}

unsafe impl Sync for RafInner {}

pub struct Raf {
    started: Arc<AtomicBool>,
    inner: Arc<parking_lot::Mutex<RafInner>>,
}

impl Clone for Raf {
    fn clone(&self) -> Self {
        Self {
            started: Arc::clone(&self.started),
            inner: Arc::clone(&self.inner),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.started = Arc::clone(&source.started);
        self.inner = Arc::clone(&source.inner);
    }
}


unsafe impl Send for Raf {}

unsafe impl Sync for Raf {}

impl Raf {
    pub fn new(callback: RafCallback) -> Self {
        let inner = Arc::new(parking_lot::Mutex::new(RafInner {
            dl: None,
            callback,
        }));

        let started = Arc::new(AtomicBool::new(false));

        let started2 = Arc::clone(&started);
        let clone = inner.clone();

        {
            let mut lock = inner.lock();
            lock.dl = DisplayLink::new(move |ts| {
                if !started2.load(Ordering::SeqCst) {
                    return;
                }
                let lock = clone.lock();

                if let Some(callback) = lock.callback.as_ref() {
                    callback(ts.nanos_since_zero);
                }
            });
        }

        Self {
            started,
            inner,
        }
    }

    pub fn start(&self) {
        let mut lock = self.inner.lock();
        if let Some(dl) = lock.dl.as_mut() {
            let _ = dl.resume();
            self.started.store(!dl.is_paused(), Ordering::SeqCst);
        }
    }

    pub fn stop(&self) {
        let mut lock = self.inner.lock();
        if let Some(dl) = lock.dl.as_mut() {
            let _ = dl.pause();
            self.started.store(!dl.is_paused(), Ordering::SeqCst);
        }
    }


    pub fn started(&self) -> bool {
        self.started.load(Ordering::SeqCst)
    }
}