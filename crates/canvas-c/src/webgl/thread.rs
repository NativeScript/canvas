//! Runs every threaded WebGL context. Its GL context is created, used, presented and dropped on
//! this one thread, so the JS thread never waits on the driver: calls that return nothing queue up
//! behind each other, and the rest run here while the caller waits. One thread for all of them keeps
//! reads from one context into another (texImage2D from a WebGL canvas) on a single thread.

use std::cell::Cell;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{mpsc, Arc, Condvar, Mutex, OnceLock};

type Job = Box<dyn FnOnce() + Send>;

struct Worker {
    queue: Mutex<VecDeque<Job>>,
    signal: Condvar,
}

impl Worker {
    fn push(&self, job: Job) {
        if let Ok(mut queue) = self.queue.lock() {
            queue.push_back(job);
        }
        self.signal.notify_one();
    }
}

static WORKER: OnceLock<Option<Arc<Worker>>> = OnceLock::new();

/// Threaded canvases (WebGL here, 2D on its render thread) with a frame waiting behind another
/// that hasn't reached the screen yet: the GPU is behind them.
static CANVASES_BEHIND: AtomicU32 = AtomicU32::new(0);

thread_local! {
    static ON_GL_THREAD: Cell<bool> = const { Cell::new(false) };
}

fn worker() -> Option<Arc<Worker>> {
    WORKER
        .get_or_init(|| {
            let worker = Arc::new(Worker {
                queue: Mutex::new(VecDeque::new()),
                signal: Condvar::new(),
            });
            let shared = Arc::clone(&worker);
            std::thread::Builder::new()
                .name("nsc-webgl".to_owned())
                .spawn(move || run(shared))
                .ok()
                .map(|_| worker)
        })
        .clone()
}

fn raise_priority() {
    #[cfg(target_os = "android")]
    unsafe {
        // THREAD_PRIORITY_DISPLAY, for this thread only.
        libc::setpriority(libc::PRIO_PROCESS, 0, -4);
    }
    #[cfg(any(
        target_os = "ios",
        target_os = "tvos",
        target_os = "visionos",
        target_os = "macos"
    ))]
    unsafe {
        libc::pthread_set_qos_class_self_np(libc::qos_class_t::QOS_CLASS_USER_INTERACTIVE, 0);
    }
}

fn run(worker: Arc<Worker>) {
    raise_priority();
    ON_GL_THREAD.with(|on| on.set(true));
    loop {
        let jobs: Vec<Job> = {
            let Ok(mut queue) = worker.queue.lock() else {
                return;
            };
            while queue.is_empty() {
                queue = match worker.signal.wait(queue) {
                    Ok(queue) => queue,
                    Err(_) => return,
                };
            }
            queue.drain(..).collect()
        };
        for job in jobs {
            job();
        }
    }
}

/// Whether contexts can be threaded at all: false if the thread could not be started.
pub(crate) fn available() -> bool {
    worker().is_some()
}

pub(crate) fn on_gl_thread() -> bool {
    ON_GL_THREAD.with(|on| on.get())
}

/// Queues `job` behind every earlier one. On the GL thread itself (a job calling back into the
/// FFI) it runs at once: it belongs to the job that is running.
pub(crate) fn post(job: impl FnOnce() + Send + 'static) {
    if on_gl_thread() {
        job();
        return;
    }
    match worker() {
        Some(worker) => worker.push(Box::new(job)),
        None => job(),
    }
}

struct AssertSend<T>(T);

// Only built by `sync`, which blocks until the value has crossed back.
unsafe impl<T> Send for AssertSend<T> {}

/// Runs `f` after every queued job and returns its result, blocking the caller until then. Because
/// the caller waits, `f` may borrow from its stack and needn't be `Send`: nothing else touches what
/// it captures while it runs.
pub(crate) fn sync<R>(f: impl FnOnce() -> R) -> R {
    if on_gl_thread() {
        return f();
    }
    let Some(worker) = worker() else {
        return f();
    };
    let (tx, rx) = mpsc::sync_channel::<AssertSend<R>>(1);
    let f = AssertSend(f);
    let job: Box<dyn FnOnce() + '_> = Box::new(move || {
        let f = f;
        let _ = tx.send(AssertSend((f.0)()));
    });
    // SAFETY: `recv` below doesn't return until the job has run and dropped its captures, so the
    // borrows it holds outlive it, and AssertSend covers the thread hop.
    let job: Job = unsafe { std::mem::transmute::<Box<dyn FnOnce() + '_>, Job>(job) };
    worker.push(job);
    match rx.recv() {
        Ok(value) => value.0,
        // The job was dropped without running: only if it panicked.
        Err(_) => panic!("webgl thread job failed"),
    }
}

/// Frames a context may have queued before it counts as behind: one being presented, one next.
pub(crate) const BEHIND_AT: u32 = 2;

/// A context's queued presents went from `before` to `before + 1`.
pub(crate) fn frame_queued(before: u32) {
    if before + 1 == BEHIND_AT {
        canvas_behind();
    }
}

/// A context's queued presents went from `before` to `before - 1`.
pub(crate) fn frame_presented(before: u32) {
    if before == BEHIND_AT {
        canvas_caught_up();
    }
}

pub(crate) fn canvas_behind() {
    CANVASES_BEHIND.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn canvas_caught_up() {
    CANVASES_BEHIND.fetch_sub(1, Ordering::AcqRel);
}

/// How many threaded canvases are behind. The JS side holds requestAnimationFrame back while any
/// are, as a browser does when its compositor falls behind, rather than queue more work.
#[no_mangle]
pub extern "C" fn canvas_native_canvases_behind() -> u32 {
    CANVASES_BEHIND.load(Ordering::Acquire)
}
