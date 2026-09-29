//! Rasterizes and presents every threaded 2D canvas. It must never wait on the JS thread: readbacks
//! make the JS thread wait on it.

use std::collections::{HashMap, VecDeque};
use std::sync::{mpsc, Arc, Condvar, Mutex, OnceLock};

use canvas_2d::context::recording::Frame;

use super::context::CanvasRenderingContext2D;

type Job = Box<dyn FnOnce(&mut Targets) + Send>;

#[derive(Default)]
struct Targets {
    contexts: HashMap<u64, CanvasRenderingContext2D>,
    presents: Vec<u64>,
}

/// Commits merge into a frame until this thread picks it up, so committing never blocks.
type Slot = Arc<Mutex<Option<Frame>>>;

#[derive(Default)]
struct Queue {
    jobs: VecDeque<Job>,
    next_id: u64,
}

struct Worker {
    queue: Mutex<Queue>,
    signal: Condvar,
}

impl Worker {
    fn push(&self, job: Job) {
        if let Ok(mut queue) = self.queue.lock() {
            queue.jobs.push_back(job);
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
                .name("nsc-2d-render".to_owned())
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
    #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos", target_os = "macos"))]
    unsafe {
        libc::pthread_set_qos_class_self_np(libc::qos_class_t::QOS_CLASS_USER_INTERACTIVE, 0);
    }
}

fn run(worker: Arc<Worker>) {
    raise_priority();
    let mut targets = Targets::default();
    loop {
        let jobs: Vec<Job> = {
            let Ok(mut queue) = worker.queue.lock() else {
                return;
            };
            while queue.jobs.is_empty() {
                queue = match worker.signal.wait(queue) {
                    Ok(queue) => queue,
                    Err(_) => return,
                };
            }
            queue.jobs.drain(..).collect()
        };

        // No run loop drains this thread; without a pool the layer runs out of drawables.
        #[cfg(feature = "metal")]
        let _pool = canvas_core::gpu::metal::MetalContext::new_release_pool();

        for job in jobs {
            job(&mut targets);
        }

        let mut presents = std::mem::take(&mut targets.presents);
        presents.sort_unstable();
        presents.dedup();
        for id in presents {
            if let Some(context) = targets.contexts.get_mut(&id) {
                context.render();
            }
        }
    }
}

/// Drop waits for the real context to go, so its window can be released afterwards.
pub struct RenderTarget {
    id: u64,
    worker: Arc<Worker>,
    open: Mutex<Option<Slot>>,
}

impl RenderTarget {
    /// Does not wait for `create`: blocking a surface callback on it can deadlock the buffer queue.
    pub fn new<F>(create: F) -> Option<Self>
    where
        F: FnOnce() -> Option<CanvasRenderingContext2D> + Send + 'static,
    {
        let worker = worker()?;
        let id = {
            let mut queue = worker.queue.lock().ok()?;
            queue.next_id += 1;
            queue.next_id
        };
        worker.push(Box::new(move |targets: &mut Targets| {
            if let Some(context) = create() {
                targets.contexts.insert(id, context);
            }
        }));
        Some(Self {
            id,
            worker,
            open: Mutex::new(None),
        })
    }

    pub fn commit(&self, frame: Frame) {
        if frame.is_empty() {
            return;
        }
        let Ok(mut open) = self.open.lock() else {
            return;
        };
        let mut frame = Some(frame);
        if let Some(slot) = open.as_ref() {
            if let Ok(mut pending) = slot.lock() {
                if let Some(queued) = pending.as_mut() {
                    queued.append(frame.take().expect("set above"));
                    return;
                }
            }
        }
        let slot: Slot = Arc::new(Mutex::new(frame));
        *open = Some(Arc::clone(&slot));
        let id = self.id;
        self.worker.push(Box::new(move |targets: &mut Targets| {
            let frame = slot.lock().ok().and_then(|mut pending| pending.take());
            if let (Some(frame), Some(context)) = (frame, targets.contexts.get_mut(&id)) {
                context.get_context_mut().replay(frame);
                targets.presents.push(id);
            }
        }));
    }

    /// Later commits must not merge into a frame queued before other work.
    fn seal(&self) {
        if let Ok(mut open) = self.open.lock() {
            *open = None;
        }
    }

    pub fn post<F>(&self, f: F)
    where
        F: FnOnce(&mut CanvasRenderingContext2D) + Send + 'static,
    {
        self.seal();
        let id = self.id;
        self.worker.push(Box::new(move |targets: &mut Targets| {
            if let Some(context) = targets.contexts.get_mut(&id) {
                f(context);
            }
        }));
    }

    pub fn sync<R, F>(&self, f: F) -> Option<R>
    where
        R: Send + 'static,
        F: FnOnce(&mut CanvasRenderingContext2D) -> R + Send + 'static,
    {
        self.seal();
        let (tx, rx) = mpsc::sync_channel(1);
        let id = self.id;
        self.worker.push(Box::new(move |targets: &mut Targets| {
            let result = targets.contexts.get_mut(&id).map(f);
            let _ = tx.send(result);
        }));
        rx.recv().ok().flatten()
    }
}

impl Drop for RenderTarget {
    fn drop(&mut self) {
        let (tx, rx) = mpsc::sync_channel::<()>(1);
        let id = self.id;
        self.worker.push(Box::new(move |targets: &mut Targets| {
            targets.presents.retain(|target| *target != id);
            drop(targets.contexts.remove(&id));
            let _ = tx.send(());
        }));
        let _ = rx.recv();
    }
}
