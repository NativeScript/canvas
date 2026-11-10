use display_link::DisplayLink;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

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
    active: Arc<AtomicUsize>,
    /// A `request_after_frame` call is queued and has not run yet.
    after_frame_pending: Arc<AtomicBool>,
}

impl Clone for Raf {
    fn clone(&self) -> Self {
        Self {
            started: Arc::clone(&self.started),
            inner: Arc::clone(&self.inner),
            active: Arc::clone(&self.active),
            after_frame_pending: Arc::clone(&self.after_frame_pending),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.started = Arc::clone(&source.started);
        self.inner = Arc::clone(&source.inner);
        self.active = Arc::clone(&source.active);
        self.after_frame_pending = Arc::clone(&source.after_frame_pending);
    }
}

unsafe impl Send for Raf {}

unsafe impl Sync for Raf {}

impl Raf {
    pub fn new(callback: RafCallback) -> Self {
        let inner = Arc::new(parking_lot::Mutex::new(RafInner { dl: None, callback }));

        let started = Arc::new(AtomicBool::new(false));

        let started2 = Arc::clone(&started);
        let clone = inner.clone();
        let active = Arc::new(AtomicUsize::new(0));
        let active_clone = Arc::clone(&active);

        {
            let mut lock = inner.lock();
            lock.dl = DisplayLink::new(move |ts| {
                if !started2.load(Ordering::SeqCst) {
                    return;
                }

                active_clone.fetch_add(1, Ordering::SeqCst);

                let lock = clone.lock();

                if let Some(callback) = lock.callback.as_ref() {
                    callback(ts.nanos_since_zero);
                }

                active_clone.fetch_sub(1, Ordering::SeqCst);
            });
        }

        Self {
            started,
            inner,
            active,
            after_frame_pending: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Calls back once, when the run loop callout running now returns: from a page's
    /// requestAnimationFrame callback, that is after the frame's callbacks, where a browser
    /// presents. The timestamp passed is 0. False if the Raf is stopped or the thread has no
    /// run loop.
    pub fn request_after_frame(&self) -> bool {
        if !self.started() {
            return false;
        }
        if self.after_frame_pending.swap(true, Ordering::SeqCst) {
            return true;
        }
        let raf = self.clone();
        let posted = super::after_frame::post(Box::new(move || {
            raf.after_frame_pending.store(false, Ordering::SeqCst);
            if !raf.started() {
                return;
            }
            raf.active.fetch_add(1, Ordering::SeqCst);
            {
                let lock = raf.inner.lock();
                if let Some(callback) = lock.callback.as_ref() {
                    callback(0);
                }
            }
            raf.active.fetch_sub(1, Ordering::SeqCst);
        }));
        if posted.is_err() {
            self.after_frame_pending.store(false, Ordering::SeqCst);
        }
        posted.is_ok()
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

    pub fn wait_until_idle(&self, timeout_ms: u64) -> bool {
        let start = Instant::now();
        while self.active.load(Ordering::SeqCst) != 0 {
            if start.elapsed() > Duration::from_millis(timeout_ms) {
                return false;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        true
    }

    pub fn clear_callback(&self) {
        let mut lock = self.inner.lock();
        lock.callback = None;
    }

    pub fn started(&self) -> bool {
        self.started.load(Ordering::SeqCst)
    }
}
