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
    // Incremented on every start(). Pending frame callbacks belonging to an
    // older generation drop themselves on their next fire instead of
    // re-posting, so a stop()/start() pair can never spawn duplicate chains.
    generation: u64,
    thread_id: Option<std::thread::ThreadId>,
}

pub struct Raf(
    Arc<parking_lot::Mutex<RafInner>>,
    Arc<AtomicUsize>,
    Arc<parking_lot::Condvar>,
);

// Payload handed to AChoreographer_postFrameCallback. A posted frame callback
// cannot be cancelled, so the callback itself is the SOLE owner of this
// allocation: on each fire it either re-posts the same box for the next frame
// or drops it. No other code path may free it — freeing it from stop() or
// clear_callback() while a callback is still queued is a guaranteed
// use-after-free at the next vsync.
struct FrameChain {
    raf: Raf,
    generation: u64,
}

impl Raf {
    extern "C" fn callback(frame_time_nanos: c_long, data: *mut std::os::raw::c_void) {
        if data.is_null() {
            return;
        }

        // Take ownership for this invocation; re-posted below if still live.
        let chain = unsafe { Box::from_raw(data as *mut FrameChain) };
        let raf = &chain.raf;

        {
            let lock = raf.0.lock();
            if !lock.started || lock.generation != chain.generation {
                // Chain is dead (stopped, or superseded by a newer start()).
                // Dropping `chain` releases the allocation.
                return;
            }
            raf.1.fetch_add(1, Ordering::SeqCst);
        }

        {
            let lock = raf.0.lock();
            if let Some(callback) = lock.callback.as_ref() {
                callback(frame_time_nanos.into());
            }
        }

        raf.1.fetch_sub(1, Ordering::SeqCst);
        raf.2.notify_all();

        let repost = {
            let lock = raf.0.lock();
            lock.started && lock.generation == chain.generation && lock.use_deprecated
        };

        if repost {
            unsafe {
                let instance = AChoreographer_getInstance();
                AChoreographer_postFrameCallback(
                    instance,
                    Some(Raf::callback),
                    Box::into_raw(chain) as *mut c_void,
                );
            }
        }
        // else: `chain` drops here, ending this frame chain.
    }

    pub fn new(callback: RafCallback) -> Self {
        Self(
            Arc::new(parking_lot::Mutex::new(RafInner {
                started: false,
                callback,
                is_prepared: false,
                use_deprecated: true, //*crate::API_LEVEL.get().unwrap_or(&-1) < 24,
                generation: 0,
                thread_id: None,
            })),
            Arc::new(AtomicUsize::new(0)),
            Arc::new(parking_lot::Condvar::new()),
        )
    }

    pub fn start(&self) {
        let generation;
        {
            let mut lock = self.0.lock();
            if lock.started {
                return;
            }
            if !lock.is_prepared {
                unsafe {
                    ndk::looper::ThreadLooper::prepare();
                }
                lock.is_prepared = true;
            }

            lock.thread_id = Some(std::thread::current().id());
            lock.generation += 1;
            lock.started = true;
            generation = lock.generation;
            if !lock.use_deprecated {
                //   #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
                //    AChoreographer_postFrameCallback64(...)
                return;
            }
        }

        let chain = Box::new(FrameChain {
            raf: self.clone(),
            generation,
        });
        unsafe {
            let instance = AChoreographer_getInstance();
            AChoreographer_postFrameCallback(
                instance,
                Some(Raf::callback),
                Box::into_raw(chain) as *mut c_void,
            );
        }
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
                    // Same thread as the choreographer callback: a callback
                    // cannot be mid-flight right now, nothing to wait for.
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
        // Only detach the JS-side callback. The FrameChain allocation stays
        // alive until the already-queued choreographer callback fires, sees
        // started == false (or a stale generation), and drops it itself.
        let mut lock = self.0.lock();
        lock.callback = None;
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
