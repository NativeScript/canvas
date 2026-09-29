//! Rasterizes and presents every threaded 2D canvas. It must never wait on the JS thread: readbacks
//! make the JS thread wait on it.

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Condvar, Mutex, OnceLock};
use std::time::Duration;

use canvas_2d::context::recording::Frame;

use super::context::CanvasRenderingContext2D;

type Job = Box<dyn FnOnce(&mut Targets) + Send>;

#[derive(Default)]
struct Targets {
    contexts: HashMap<u64, Entry>,
    presents: Vec<u64>,
}

/// Boxed: a context stays at its address while it lives (`register_d3d`).
struct Entry {
    context: Box<CanvasRenderingContext2D>,
    lost: Arc<AtomicBool>,
}

impl Entry {
    fn update_lost(&self) {
        self.lost.store(self.context.gpu_lost(), Ordering::Release);
    }
}

/// How soon a present the display was not ready for is tried again.
const RETRY_PRESENT: Duration = Duration::from_millis(4);

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
    #[cfg(target_os = "windows")]
    unsafe {
        use windows::Win32::System::Threading::{GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_ABOVE_NORMAL};
        let _ = SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_ABOVE_NORMAL);
        // XAML surfaces are drawn from here.
        let _ = windows::Win32::System::Com::CoInitializeEx(None, windows::Win32::System::Com::COINIT_MULTITHREADED);
    }
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
    let mut retry: Vec<u64> = Vec::new();
    loop {
        let jobs: Vec<Job> = {
            let Ok(mut queue) = worker.queue.lock() else {
                return;
            };
            while queue.jobs.is_empty() {
                if retry.is_empty() {
                    queue = match worker.signal.wait(queue) {
                        Ok(queue) => queue,
                        Err(_) => return,
                    };
                    continue;
                }
                let timed_out;
                (queue, timed_out) = match worker.signal.wait_timeout(queue, RETRY_PRESENT) {
                    Ok((queue, timeout)) => (queue, timeout.timed_out()),
                    Err(_) => return,
                };
                if timed_out {
                    break;
                }
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
        presents.append(&mut retry);
        presents.sort_unstable();
        presents.dedup();
        for id in presents {
            if let Some(entry) = targets.contexts.get_mut(&id) {
                take_present_deferred();
                entry.context.render();
                if take_present_deferred() {
                    retry.push(id);
                }
                entry.update_lost();
            }
        }
    }
}

/// The display was not ready for the last present (Windows: the swapchain's frame latency).
fn take_present_deferred() -> bool {
    #[cfg(all(feature = "d3d", target_os = "windows"))]
    {
        canvas_core::gpu::dxgi::take_present_deferred()
    }
    #[cfg(not(all(feature = "d3d", target_os = "windows")))]
    {
        false
    }
}

/// Runs `f` on the render thread and waits for it; `None` if the thread was never started. What
/// it does to the thread's device shows in every context's `is_lost` straight away.
pub fn on_render_thread<R, F>(f: F) -> Option<R>
where
    R: Send + 'static,
    F: FnOnce() -> R + Send + 'static,
{
    let worker = WORKER.get()?.clone()?;
    let (tx, rx) = mpsc::sync_channel(1);
    worker.push(Box::new(move |targets: &mut Targets| {
        let result = f();
        for entry in targets.contexts.values() {
            entry.update_lost();
        }
        let _ = tx.send(result);
    }));
    rx.recv().ok()
}

/// Drop waits for the real context to go, so its window can be released afterwards.
pub struct RenderTarget {
    id: u64,
    worker: Arc<Worker>,
    open: Mutex<Option<Slot>>,
    lost: Arc<AtomicBool>,
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
        let lost = Arc::new(AtomicBool::new(false));
        let shared = Arc::clone(&lost);
        worker.push(Box::new(move |targets: &mut Targets| {
            if let Some(context) = create() {
                let mut context = Box::new(context);
                context.settle();
                targets.contexts.insert(id, Entry { context, lost: shared });
            }
        }));
        Some(Self {
            id,
            worker,
            open: Mutex::new(None),
            lost,
        })
    }

    /// The real context's GPU device was lost, as of its last present or job.
    pub fn is_lost(&self) -> bool {
        self.lost.load(Ordering::Acquire)
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
            if let (Some(frame), Some(entry)) = (frame, targets.contexts.get_mut(&id)) {
                entry.context.get_context_mut().replay(frame);
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
            if let Some(entry) = targets.contexts.get_mut(&id) {
                f(&mut entry.context);
                entry.update_lost();
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
            let result = targets.contexts.get_mut(&id).map(|entry| {
                let result = f(&mut entry.context);
                entry.update_lost();
                result
            });
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
