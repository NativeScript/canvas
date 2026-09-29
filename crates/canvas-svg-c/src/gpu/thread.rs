//! One render thread per process replays every threaded view's display lists. Each view's GPU
//! surface lives only on this thread, since EGL contexts and `VkQueue`s are single-thread.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
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
    /// Set by the owner as it detaches, so the thread neither builds nor presents to a surface
    /// whose window is going away.
    removed: AtomicBool,
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

/// Hands a view's window back once its surface is gone. Called on the render thread.
pub type ReleaseWindow = unsafe extern "C" fn(*mut std::ffi::c_void);

/// How the render thread tells a view's owner that its surface is gone.
enum Removal {
    /// The owner is blocked in `drop` until this is set.
    Wait(Removed),
    /// The owner has moved on, so the thread releases the window itself.
    Release(Window, ReleaseWindow),
}

#[derive(Default)]
struct Queue {
    next_id: u64,
    adds: Vec<Add>,
    removes: Vec<(u64, Removal)>,
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
    window: *mut std::ffi::c_void,
    target: Arc<Target>,
    worker: Arc<Worker>,
    /// Set by [`Self::release`], which has already queued the removal.
    released: bool,
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
            removed: AtomicBool::new(false),
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
        Some(Self {
            id,
            window,
            target,
            worker,
            released: false,
        })
    }

    /// Detaches without waiting: the thread tears the surface down and then passes the window
    /// to `release`, so the caller must not release it as well. Dropping instead blocks until
    /// the thread gets to the removal, which can be behind a context being built for every
    /// other view that just appeared: seconds of frozen UI when a page of them is swapped out.
    pub fn release(mut self, release: ReleaseWindow) {
        self.target.removed.store(true, Ordering::Release);
        self.queue_removal(Removal::Release(Window(self.window), release));
        self.released = true;
    }

    fn queue_removal(&self, removal: Removal) {
        if let Ok(mut queue) = self.worker.queue.lock() {
            queue.removes.push((self.id, removal));
        }
        self.worker.wake();
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
        if self.released {
            return;
        }
        self.target.removed.store(true, Ordering::Release);
        let removed: Removed = Arc::new((Mutex::new(false), Condvar::new()));
        self.queue_removal(Removal::Wait(Arc::clone(&removed)));
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
        let mut adds = VecDeque::from(adds);
        remove(removes, &mut adds, &mut views);

        // One at a time, each presented as soon as it exists: every surface is a whole GPU
        // context, and a page of views would otherwise sit blank until the last one is up.
        // Removals queued meanwhile are taken between them rather than after the lot.
        loop {
            remove(take_removes(&worker), &mut adds, &mut views);
            let Some(add) = adds.pop_front() else { break };
            if add.target.removed.load(Ordering::Acquire) {
                // Detached while queued; its removal is already on its way.
                continue;
            }
            let surface = SvgGpuSurface::new(add.window.0, add.width, add.height, add.backend);
            if surface.is_none() {
                // Nobody waits on startup, so report failure as a loss or the view stays blank.
                if let Ok(mut state) = add.target.state.lock() {
                    state.status = Some(FrameStatus::Lost);
                }
            }
            let mut view = View {
                id: add.id,
                target: add.target,
                surface,
            };
            present(&mut view);
            views.push(view);
        }

        for view in views.iter_mut() {
            present(view);
        }
    }
}

fn take_removes(worker: &Worker) -> Vec<(u64, Removal)> {
    worker
        .queue
        .lock()
        .map(|mut queue| std::mem::take(&mut queue.removes))
        .unwrap_or_default()
}

fn remove(removes: Vec<(u64, Removal)>, adds: &mut VecDeque<Add>, views: &mut Vec<View>) {
    for (id, removal) in removes {
        // A view gone before its add was seen never gets a surface.
        adds.retain(|add| add.id != id);
        // Dropping the view tears its surface down before the window is released.
        views.retain(|view| view.id != id);
        match removal {
            Removal::Wait(removed) => {
                let (done, signal) = &*removed;
                if let Ok(mut done) = done.lock() {
                    *done = true;
                }
                signal.notify_all();
            }
            Removal::Release(window, release) => unsafe { release(window.0) },
        }
    }
}

fn present(view: &mut View) {
    // Its window may already be abandoned, and a failed present would rebuild the context.
    if view.target.removed.load(Ordering::Acquire) {
        return;
    }
    let Some(surface) = view.surface.as_mut() else {
        return;
    };
    let resize = view.target.state.lock().ok().and_then(|mut s| s.resize.take());
    if let Some((width, height)) = resize {
        surface.resize(width, height);
    }
    if let Some(frame) = view.target.slot.take() {
        let target = &view.target;
        let status = surface.present(&frame, &|| !target.removed.load(Ordering::Acquire));
        if let Ok(mut state) = view.target.state.lock() {
            state.status = Some(status);
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

    static RELEASED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    unsafe extern "C" fn count_release(_window: *mut std::ffi::c_void) {
        RELEASED.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn release_returns_at_once_and_the_thread_releases_every_window() {
        let before = RELEASED.load(Ordering::SeqCst);
        for _ in 0..50 {
            RenderThread::new(std::ptr::null_mut(), 10, 10, Backend::Auto)
                .unwrap()
                .release(count_release);
        }
        let deadline = Instant::now() + Duration::from_secs(5);
        while RELEASED.load(Ordering::SeqCst) < before + 50 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(RELEASED.load(Ordering::SeqCst) >= before + 50);
    }

    #[test]
    fn commits_and_resizes_for_a_dead_view_are_ignored() {
        let view = RenderThread::new(std::ptr::null_mut(), 10, 10, Backend::Auto).unwrap();
        assert_eq!(wait_for_status(&view), Some(FrameStatus::Lost));
        view.resize(20, 20);
        assert_eq!(view.take_status(), None);
    }
}
