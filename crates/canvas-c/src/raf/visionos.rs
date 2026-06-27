//! visionOS requestAnimationFrame backend.
//!
//! The `display-link` crate (used by the iOS backend) is hard-gated to
//! `target_os = "ios"` / `"macos"`, so it produces nothing on visionOS. CADisplayLink
//! *is* available on visionOS, so this implements the same `Raf` surface as
//! `raf::ios` directly on top of `objc2-quartz-core`'s `CADisplayLink`.
//!
//! The public API mirrors `raf/ios.rs` exactly (`new`, `start`, `stop`,
//! `wait_until_idle`, `clear_callback`, `started`) so `canvas-c`'s FFI layer is
//! platform-agnostic.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{define_class, msg_send, sel, AnyThread, DefinedClass};
use objc2_foundation::{NSRunLoop, NSRunLoopCommonModes};
use objc2_quartz_core::CADisplayLink;
use parking_lot::Mutex;

type RafCallback = Option<Box<dyn Fn(i64)>>;

/// State shared between the public `Raf` handle and the CADisplayLink target object.
struct RafShared {
    started: Arc<AtomicBool>,
    active: Arc<AtomicUsize>,
    callback: Arc<Mutex<RafCallback>>,
}

define_class!(
    // SAFETY:
    // - NSObject has no subclassing requirements.
    // - RafTarget does not implement Drop (its ivars' Drop is run by objc2).
    #[unsafe(super(NSObject))]
    #[name = "NSCRafDisplayLinkTarget"]
    #[ivars = RafShared]
    struct RafTarget;

    impl RafTarget {
        // Invoked by CADisplayLink once per vsync on the run loop it was added to.
        #[unsafe(method(step:))]
        fn step(&self, link: &CADisplayLink) {
            let ivars = self.ivars();
            if !ivars.started.load(Ordering::SeqCst) {
                return;
            }
            ivars.active.fetch_add(1, Ordering::SeqCst);
            // targetTimestamp is the time (seconds) the upcoming frame will display.
            let nanos = (link.targetTimestamp() * 1_000_000_000.0) as i64;
            if let Some(callback) = ivars.callback.lock().as_ref() {
                callback(nanos);
            }
            ivars.active.fetch_sub(1, Ordering::SeqCst);
        }
    }
);

/// Owns the CADisplayLink. Shared via `Arc` so cloned `Raf` handles share one link and
/// `invalidate()` runs exactly once, when the last handle is dropped (mirrors how
/// `raf/ios.rs` shares its `DisplayLink` through `Arc`).
struct Inner {
    display_link: Retained<CADisplayLink>,
    // Kept alive for the lifetime of the link (CADisplayLink also retains its target).
    _target: Retained<RafTarget>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        // Removes the link from the run loop; without this it would keep firing,
        // since the run loop also retains the CADisplayLink.
        self.display_link.invalidate();
    }
}

// CADisplayLink/target are !Send/!Sync; the callback only fires on the run loop thread it
// was scheduled on (the main thread). Matches the unsafe posture of raf/ios.rs.
unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

#[derive(Clone)]
pub struct Raf {
    started: Arc<AtomicBool>,
    active: Arc<AtomicUsize>,
    callback: Arc<Mutex<RafCallback>>,
    inner: Arc<Inner>,
}

// The closure is not required to be Send/Sync (matches raf/ios.rs).
unsafe impl Send for Raf {}
unsafe impl Sync for Raf {}

impl Raf {
    pub fn new(callback: RafCallback) -> Self {
        let started = Arc::new(AtomicBool::new(false));
        let active = Arc::new(AtomicUsize::new(0));
        let callback = Arc::new(Mutex::new(callback));

        let target = RafTarget::alloc().set_ivars(RafShared {
            started: Arc::clone(&started),
            active: Arc::clone(&active),
            callback: Arc::clone(&callback),
        });
        let target: Retained<RafTarget> = unsafe { msg_send![super(target), init] };

        let display_link =
            unsafe { CADisplayLink::displayLinkWithTarget_selector(&*target, sel!(step:)) };

        // Start paused; `start()` unpauses. Schedule on the main run loop for common modes
        // so it keeps firing during UI tracking (scrolling, etc.).
        display_link.setPaused(true);
        let run_loop = NSRunLoop::mainRunLoop();
        unsafe { display_link.addToRunLoop_forMode(&run_loop, NSRunLoopCommonModes) };

        Self {
            started,
            active,
            callback,
            inner: Arc::new(Inner {
                display_link,
                _target: target,
            }),
        }
    }

    pub fn start(&self) {
        self.started.store(true, Ordering::SeqCst);
        self.inner.display_link.setPaused(false);
    }

    pub fn stop(&self) {
        self.started.store(false, Ordering::SeqCst);
        self.inner.display_link.setPaused(true);
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
        *self.callback.lock() = None;
    }

    pub fn started(&self) -> bool {
        self.started.load(Ordering::SeqCst)
    }
}
