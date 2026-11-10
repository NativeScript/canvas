//! The device timeline: queue work (finish, submit, uploads, present) runs in
//! order on one thread per device instead of in the JS call, as a browser's GPU
//! process does. Calls that need the queue's results first wait for it to drain.
//!
//! Callbacks into JS go through the JS thread's loop where there is one (Android,
//! Apple), else wait for the JS thread's next call into WebGPU.

use std::cell::RefCell;
use std::sync::{mpsc, Arc, OnceLock};

use crate::thread_poster::ThreadPoster;

type Job = Box<dyn FnOnce() + Send>;

/// An error scope open on the JS thread. Work queued while it was open reports
/// into it from the queue thread.
#[derive(Debug)]
pub(crate) struct ScopeSlot {
    filter: wgt::error::ErrorFilter,
    error: parking_lot::Mutex<Option<wgt::error::Error>>,
}

impl ScopeSlot {
    pub(crate) fn new(filter: wgt::error::ErrorFilter) -> Arc<Self> {
        Arc::new(ScopeSlot {
            filter,
            error: parking_lot::Mutex::new(None),
        })
    }

    pub(crate) fn take(&self) -> Option<wgt::error::Error> {
        self.error.lock().take()
    }
}

/// The scopes open when a call was made.
#[derive(Clone, Default)]
pub(crate) struct ScopeSnapshot(pub(crate) Vec<Arc<ScopeSlot>>);

thread_local! {
    static CURRENT_SCOPES: RefCell<Option<ScopeSnapshot>> = const { RefCell::new(None) };
}

fn filter_of(error: &wgt::error::Error) -> wgt::error::ErrorFilter {
    match error {
        wgt::error::Error::OutOfMemory { .. } => wgt::error::ErrorFilter::OutOfMemory,
        wgt::error::Error::Validation { .. } => wgt::error::ErrorFilter::Validation,
        _ => wgt::error::ErrorFilter::Internal,
    }
}

/// Runs `f` with errors reported into `scopes`.
pub(crate) fn with_scopes<R>(scopes: &ScopeSnapshot, f: impl FnOnce() -> R) -> R {
    let previous = CURRENT_SCOPES.with(|current| current.replace(Some(scopes.clone())));
    let result = f();
    CURRENT_SCOPES.with(|current| *current.borrow_mut() = previous);
    result
}

/// Stores `error` in the innermost matching scope of the running job. Hands it
/// back if there is none: it is uncaptured.
pub(crate) fn capture(error: wgt::error::Error) -> Result<(), wgt::error::Error> {
    CURRENT_SCOPES.with(|current| {
        let current = current.borrow();
        let Some(scopes) = current.as_ref() else {
            return Err(error);
        };
        let filter = filter_of(&error);
        match scopes.0.iter().rev().find(|slot| slot.filter == filter) {
            Some(slot) => {
                let mut slot = slot.error.lock();
                if slot.is_none() {
                    *slot = Some(error);
                }
                Ok(())
            }
            None => Err(error),
        }
    })
}

/// Display priority, like a browser's GPU thread: frames wait on this thread.
fn raise_priority() {
    #[cfg(target_os = "android")]
    unsafe {
        // THREAD_PRIORITY_DISPLAY
        libc::setpriority(libc::PRIO_PROCESS, libc::gettid() as libc::id_t, -4);
    }
    #[cfg(any(target_os = "ios", target_os = "visionos", target_os = "tvos"))]
    unsafe {
        // QOS_CLASS_USER_INTERACTIVE, as the main thread.
        extern "C" {
            fn pthread_set_qos_class_self_np(qos_class: u32, relative_priority: i32) -> i32;
        }
        pthread_set_qos_class_self_np(0x21, 0);
    }
}

pub(crate) struct QueueThread {
    /// Work recorded since the last send: sent together, one wakeup per batch.
    recorded: parking_lot::Mutex<Vec<Job>>,
    sender: parking_lot::Mutex<Option<mpsc::Sender<Job>>>,
    pending: Arc<(parking_lot::Mutex<usize>, parking_lot::Condvar)>,
    poster: Option<Arc<ThreadPoster>>,
    js_thread: std::thread::ThreadId,
    /// Callbacks for the JS thread when it has no loop to post to.
    for_js: parking_lot::Mutex<Vec<Job>>,
    worker: OnceLock<std::thread::ThreadId>,
}

impl QueueThread {
    fn new() -> Self {
        QueueThread {
            recorded: parking_lot::Mutex::new(Vec::new()),
            sender: parking_lot::Mutex::new(None),
            pending: Arc::default(),
            poster: ThreadPoster::for_current_thread(),
            js_thread: std::thread::current().id(),
            for_js: parking_lot::Mutex::new(Vec::new()),
            worker: OnceLock::new(),
        }
    }

    fn on_worker(&self) -> bool {
        self.worker.get() == Some(&std::thread::current().id())
    }

    /// Runs `job` after everything queued before it, without waking the queue
    /// thread: it goes with the next `enqueue`, `flush` or `drain`.
    pub(crate) fn record(&self, job: Job) {
        if self.on_worker() {
            job();
            return;
        }
        self.deliver_to_js();
        self.recorded.lock().push(job);
    }

    /// Runs `job` after everything queued before it.
    pub(crate) fn enqueue(&self, job: Job) {
        self.record(job);
        self.flush();
    }

    /// Sends what was recorded.
    pub(crate) fn flush(&self) {
        let jobs = std::mem::take(&mut *self.recorded.lock());
        if jobs.is_empty() {
            return;
        }
        self.send(Box::new(move || {
            for job in jobs {
                job();
            }
        }));
    }

    fn send(&self, job: Job) {
        *self.pending.0.lock() += 1;
        let mut sender = self.sender.lock();
        let sender = sender.get_or_insert_with(|| {
            let (tx, rx) = mpsc::channel::<Job>();
            let pending = Arc::clone(&self.pending);
            let handle = std::thread::Builder::new()
                .name("webgpu-queue".into())
                .spawn(move || {
                    raise_priority();
                    while let Ok(job) = rx.recv() {
                        job();
                        let mut count = pending.0.lock();
                        *count -= 1;
                        if *count == 0 {
                            pending.1.notify_all();
                        }
                    }
                })
                .expect("webgpu queue thread");
            let _ = self.worker.set(handle.thread().id());
            tx
        });
        if let Err(mpsc::SendError(job)) = sender.send(job) {
            *self.pending.0.lock() -= 1;
            job();
        }
    }

    /// Blocks until everything queued so far has run.
    pub(crate) fn drain(&self) {
        if self.on_worker() {
            return;
        }
        self.flush();
        let mut count = self.pending.0.lock();
        while *count > 0 {
            self.pending.1.wait(&mut count);
        }
        drop(count);
        self.deliver_to_js();
    }

    /// Runs `task` on the JS thread.
    pub(crate) fn post_to_js(&self, task: Job) {
        if std::thread::current().id() == self.js_thread {
            task();
        } else if let Some(poster) = self.poster.as_ref() {
            poster.post(task);
        } else {
            self.for_js.lock().push(task);
        }
    }

    fn deliver_to_js(&self) {
        if std::thread::current().id() != self.js_thread {
            return;
        }
        let tasks = std::mem::take(&mut *self.for_js.lock());
        for task in tasks {
            task();
        }
    }
}

/// Shared by every object of a device; the thread starts on first use, from the
/// JS thread whose loop callbacks are posted back to.
#[derive(Default)]
pub(crate) struct Timeline(OnceLock<QueueThread>);

impl std::fmt::Debug for Timeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Timeline")
    }
}

impl Timeline {
    pub(crate) fn get(&self) -> &QueueThread {
        self.0.get_or_init(QueueThread::new)
    }

    /// Waits for queued work, if the thread was ever started.
    pub(crate) fn drain(&self) {
        if let Some(thread) = self.0.get() {
            thread.drain();
        }
    }
}
