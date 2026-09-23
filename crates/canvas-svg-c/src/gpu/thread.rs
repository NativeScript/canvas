//! Replays display lists recorded on the document's thread; the document is never touched here.
//! The GPU surface is created and used only on this thread (EGL contexts and `VkQueue`s are
//! single-thread), and pacing comes from the committer plus the blocking present.

use std::sync::{Arc, Condvar, Mutex};

use canvas_svg::{FrameSlot, RecordedFrame};

use super::{Backend, FrameStatus, SvgGpuSurface};

/// Only dereferenced on the render thread; the owner keeps the window alive until `destroy` joins.
struct Window(*mut std::ffi::c_void);
unsafe impl Send for Window {}

#[derive(Default)]
struct State {
    running: bool,
    /// Applied before the next present.
    resize: Option<(i32, i32)>,
    status: Option<FrameStatus>,
    started: bool,
    usable: bool,
}

struct Shared {
    state: Mutex<State>,
    signal: Condvar,
}

pub struct RenderThread {
    slot: Arc<FrameSlot>,
    shared: Arc<Shared>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl RenderThread {
    /// Does not wait for the surface: blocking the UI thread's surface callback while this
    /// thread connects to the window deadlocks the buffer queue (ANR). See [`take_status`].
    pub fn new(
        window: *mut std::ffi::c_void,
        width: i32,
        height: i32,
        backend: Backend,
    ) -> Option<Self> {
        let slot = Arc::new(FrameSlot::new());
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                running: true,
                ..Default::default()
            }),
            signal: Condvar::new(),
        });

        let window = Window(window);
        let handle = {
            let slot = Arc::clone(&slot);
            let shared = Arc::clone(&shared);
            std::thread::Builder::new()
                .name("nsc-svg-render".to_owned())
                .spawn(move || run(window, width, height, backend, slot, shared))
                .ok()?
        };

        Some(Self {
            slot,
            shared,
            handle: Some(handle),
        })
    }

    pub fn commit(&self, frame: RecordedFrame) {
        self.slot.commit(frame);
        self.wake();
    }

    pub fn resize(&self, width: i32, height: i32) {
        if let Ok(mut state) = self.shared.state.lock() {
            state.resize = Some((width, height));
        }
        self.wake();
    }

    /// Cleared on read; `None` means nothing has presented since the last check.
    pub fn take_status(&self) -> Option<FrameStatus> {
        self.shared.state.lock().ok().and_then(|mut s| s.status.take())
    }

    fn wake(&self) {
        self.shared.signal.notify_all();
    }
}

impl Drop for RenderThread {
    fn drop(&mut self) {
        if let Ok(mut state) = self.shared.state.lock() {
            state.running = false;
        }
        self.shared.signal.notify_all();
        // Join, not detach: GPU teardown after the caller releases the window crashes.
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn run(
    window: Window,
    width: i32,
    height: i32,
    backend: Backend,
    slot: Arc<FrameSlot>,
    shared: Arc<Shared>,
) {
    let mut surface = SvgGpuSurface::new(window.0, width, height, backend);
    {
        let Ok(mut state) = shared.state.lock() else { return };
        state.started = true;
        state.usable = surface.is_some();
        if surface.is_none() {
            // Nobody waits on startup, so report failure as a loss or the view stays blank.
            state.status = Some(FrameStatus::Lost);
        }
        shared.signal.notify_all();
    }
    let Some(surface) = surface.as_mut() else { return };

    loop {
        let resize = {
            let Ok(mut state) = shared.state.lock() else { return };
            while state.running && state.resize.is_none() && !slot.has_frame() {
                let Ok(next) = shared.signal.wait(state) else { return };
                state = next;
            }
            if !state.running {
                return;
            }
            state.resize.take()
        };

        if let Some((width, height)) = resize {
            surface.resize(width, height);
        }

        if let Some(frame) = slot.take() {
            let status = surface.present(&frame);
            if let Ok(mut state) = shared.state.lock() {
                state.status = Some(status);
            }
        }
    }
}
