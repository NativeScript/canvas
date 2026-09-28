//! One render thread per process replays every threaded view's display lists. Each view's GPU
//! surface lives only on this thread, since EGL contexts and `VkQueue`s are single-thread.

use std::sync::{Arc, Condvar, Mutex, OnceLock};

use canvas_svg::{FrameSlot, RecordedFrame};

use super::{Backend, FrameStatus, SvgGpuSurface};

/// Only dereferenced on the render thread; the owner keeps the window alive until drop returns.
struct Window(*mut std::ffi::c_void);
unsafe impl Send for Window {}

/// What a view and the render thread share.
struct Target {
    slot: FrameSlot,
    state: Mutex<TargetState>,
}

#[derive(Default)]
struct TargetState {
    /// Applied before the next present.
    resize: Option<(i32, i32)>,
    status: Option<FrameStatus>,
}

struct Add {
    id: u64,
    window: Window,
    width: i32,
    height: i32,
    backend: Backend,
    target: Arc<Target>,
}

/// Set by the render thread once the view's surface is gone.
type Removed = Arc<(Mutex<bool>, Condvar)>;

#[derive(Default)]
struct Queue {
    next_id: u64,
    adds: Vec<Add>,
    removes: Vec<(u64, Removed)>,
    /// Anything to do since the thread last looked: a frame, a resize, an add or a remove.
    dirty: bool,
}

struct Worker {
    queue: Mutex<Queue>,
    signal: Condvar,
}

impl Worker {
    fn wake(&self) {
        if let Ok(mut queue) = self.queue.lock() {
            queue.dirty = true;
        }
        self.signal.notify_one();
    }
}

static WORKER: OnceLock<Option<Arc<Worker>>> = OnceLock::new();

fn worker() -> Option<Arc<Worker>> {
    WORKER
        .get_or_init(|| {
            let worker = Arc::new(Worker {
                queue: Mutex::new(Queue::default()),
                signal: Condvar::new(),
            });
            let shared = Arc::clone(&worker);
            std::thread::Builder::new()
                .name("nsc-svg-render".to_owned())
                .spawn(move || run(shared))
                .ok()
                .map(|_| worker)
        })
        .clone()
}

/// A view's registration on the shared render thread.
pub struct RenderThread {
    id: u64,
    target: Arc<Target>,
    worker: Arc<Worker>,
}

impl RenderThread {
    /// Does not wait for the surface: blocking the UI thread's surface callback while the render
    /// thread connects to the window deadlocks the buffer queue (ANR). See [`take_status`].
    pub fn new(
        window: *mut std::ffi::c_void,
        width: i32,
        height: i32,
        backend: Backend,
    ) -> Option<Self> {
        let worker = worker()?;
        let target = Arc::new(Target {
            slot: FrameSlot::new(),
            state: Mutex::new(TargetState::default()),
        });
        let id = {
            let mut queue = worker.queue.lock().ok()?;
            queue.next_id += 1;
            let id = queue.next_id;
            queue.adds.push(Add {
                id,
                window: Window(window),
                width,
                height,
                backend,
                target: Arc::clone(&target),
            });
            id
        };
        worker.wake();
        Some(Self { id, target, worker })
    }

    pub fn commit(&self, frame: RecordedFrame) {
        self.target.slot.commit(frame);
        self.worker.wake();
    }

    pub fn resize(&self, width: i32, height: i32) {
        if let Ok(mut state) = self.target.state.lock() {
            state.resize = Some((width, height));
        }
        self.worker.wake();
    }

    /// Cleared on read; `None` means nothing has presented since the last check.
    pub fn take_status(&self) -> Option<FrameStatus> {
        self.target.state.lock().ok().and_then(|mut s| s.status.take())
    }
}

impl Drop for RenderThread {
    fn drop(&mut self) {
        let removed: Removed = Arc::new((Mutex::new(false), Condvar::new()));
        if let Ok(mut queue) = self.worker.queue.lock() {
            queue.removes.push((self.id, Arc::clone(&removed)));
        }
        self.worker.wake();
        // Wait, not detach: GPU teardown after the caller releases the window crashes.
        let (done, signal) = &*removed;
        if let Ok(mut done) = done.lock() {
            while !*done {
                match signal.wait(done) {
                    Ok(next) => done = next,
                    Err(_) => return,
                }
            }
        };
    }
}

struct View {
    id: u64,
    target: Arc<Target>,
    surface: Option<SvgGpuSurface>,
}

fn run(worker: Arc<Worker>) {
    let mut views: Vec<View> = Vec::new();
    loop {
        let (adds, removes) = {
            let Ok(mut queue) = worker.queue.lock() else { return };
            while !queue.dirty {
                let Ok(next) = worker.signal.wait(queue) else { return };
                queue = next;
            }
            queue.dirty = false;
            (std::mem::take(&mut queue.adds), std::mem::take(&mut queue.removes))
        };

        // Adds first: a view dropped before its add was seen is in the same batch.
        for add in adds {
            let surface = SvgGpuSurface::new(add.window.0, add.width, add.height, add.backend);
            if surface.is_none() {
                // Nobody waits on startup, so report failure as a loss or the view stays blank.
                if let Ok(mut state) = add.target.state.lock() {
                    state.status = Some(FrameStatus::Lost);
                }
            }
            views.push(View {
                id: add.id,
                target: add.target,
                surface,
            });
        }

        for (id, removed) in removes {
            // Dropping the view tears its surface down before the owner releases the window.
            views.retain(|view| view.id != id);
            let (done, signal) = &*removed;
            if let Ok(mut done) = done.lock() {
                *done = true;
            }
            signal.notify_all();
        }

        for view in views.iter_mut() {
            let Some(surface) = view.surface.as_mut() else {
                continue;
            };
            let resize = view.target.state.lock().ok().and_then(|mut s| s.resize.take());
            if let Some((width, height)) = resize {
                surface.resize(width, height);
            }
            if let Some(frame) = view.target.slot.take() {
                let status = surface.present(&frame);
                if let Ok(mut state) = view.target.state.lock() {
                    state.status = Some(status);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn wait_for_status(thread: &RenderThread) -> Option<FrameStatus> {
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            if let Some(status) = thread.take_status() {
                return Some(status);
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        None
    }

    // A null window never gets a surface, which exercises the lifecycle without a GPU.
    #[test]
    fn every_view_hears_back_from_the_shared_thread() {
        let views: Vec<_> = (0..32)
            .map(|_| RenderThread::new(std::ptr::null_mut(), 10, 10, Backend::Auto).unwrap())
            .collect();
        for view in &views {
            assert_eq!(wait_for_status(view), Some(FrameStatus::Lost));
        }
        assert!(Arc::ptr_eq(&views[0].worker, &views[31].worker));
    }

    #[test]
    fn drop_returns_even_before_the_add_is_seen() {
        let start = Instant::now();
        for _ in 0..100 {
            drop(RenderThread::new(std::ptr::null_mut(), 10, 10, Backend::Auto).unwrap());
        }
        assert!(start.elapsed() < Duration::from_secs(5));
    }

    #[test]
    fn commits_and_resizes_for_a_dead_view_are_ignored() {
        let view = RenderThread::new(std::ptr::null_mut(), 10, 10, Backend::Auto).unwrap();
        assert_eq!(wait_for_status(&view), Some(FrameStatus::Lost));
        view.resize(20, 20);
        assert_eq!(view.take_status(), None);
    }
}
