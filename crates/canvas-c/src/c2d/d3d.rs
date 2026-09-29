//! Threaded 2D canvases on Windows: the parts that stay on the UI thread (a panel shows the
//! render thread's swapchain, a XAML surface is handed to it) and video frames drawn on the
//! render thread's device.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use canvas_2d::context::recording::{ExternalImage, ExternalRelease};
use canvas_2d::context::Context;
use canvas_core::gpu::dxgi::{self, CompositionSwapChain, XamlHandoff};
use windows::core::{IUnknown, Interface};
use windows::Win32::Foundation::{CloseHandle, DuplicateHandle, DUPLICATE_SAME_ACCESS, HANDLE};
use windows::Win32::Graphics::Direct3D12::{ID3D12Fence, ID3D12Resource};
use windows::Win32::System::Threading::GetCurrentProcess;

use super::context::CanvasRenderingContext2D;
use crate::webgpu::gpu_shared_frame::CanvasD3DSharedFrame;

/// COM objects handed between the threads, used by one at a time.
struct Sendable<T>(T);
unsafe impl<T> Send for Sendable<T> {}

#[derive(Clone)]
enum Shown {
    Panel(IUnknown),
    /// `handoff` is `None` while the canvas is lost.
    Xaml { source: IUnknown, handoff: Option<Arc<XamlHandoff>> },
}

thread_local! {
    /// What each threaded canvas on this (UI) thread is shown in, by the canvas's address.
    static SHOWN: RefCell<HashMap<usize, Shown>> = RefCell::new(HashMap::new());
}

/// The render thread's device's adapter: shared video frames must come from it.
static RENDER_ADAPTER: AtomicU64 = AtomicU64::new(0);

/// Render thread, whenever it has a device for canvases.
pub(crate) fn note_render_thread_device(context: &Context) {
    if let Some(device) = context.d3d_device() {
        RENDER_ADAPTER.store(device.adapter_luid(), Ordering::Release);
    }
}

fn remember(context: *mut CanvasRenderingContext2D, shown: Shown) {
    let previous = SHOWN.with(|entries| entries.borrow_mut().insert(context as usize, shown));
    if let Some(Shown::Xaml { handoff: Some(handoff), .. }) = previous {
        // Shows its last frame, if the render thread drew one it has not ended.
        handoff.end_draw();
    }
}

/// Ends the canvas's last XAML draw when dropped: after the canvas, which may still draw.
pub(crate) struct Forgotten(Option<Shown>);

impl Drop for Forgotten {
    fn drop(&mut self) {
        if let Some(Shown::Xaml { handoff: Some(handoff), .. }) = self.0.take() {
            handoff.end_draw();
        }
    }
}

pub(crate) fn forget_shown(context: *mut CanvasRenderingContext2D) -> Forgotten {
    Forgotten(SHOWN.try_with(|entries| entries.borrow_mut().remove(&(context as usize))).ok().flatten())
}

/// UI thread. The render thread makes the swapchain; only binding it has to happen here.
pub(crate) unsafe fn attach_swap_chain_panel(context: *mut CanvasRenderingContext2D, panel: *mut c_void) -> bool {
    let Some(target) = (unsafe { &*context }).render_target() else { return false };
    let Some(panel_ref) = (unsafe { IUnknown::from_raw_borrowed(&panel) }).cloned() else { return false };
    let Some((lost, swap_chain)) = target.sync(|real| {
        let lost = real.gpu_lost();
        (lost, real.get_context_mut().create_panel_swap_chain().map(Sendable))
    }) else {
        return false;
    };
    if lost {
        // Shown there once restored.
        remember(context, Shown::Panel(panel_ref));
        return true;
    }
    let Some(Sendable(swap_chain)) = swap_chain else { return false };
    if let Err(error) = unsafe { dxgi::bind_swap_chain(panel, swap_chain.as_raw()) } {
        log::error!("canvas: could not show the canvas's swapchain in its panel: {error}");
        return false;
    }
    remember(context, Shown::Panel(panel_ref));
    true
}

/// UI thread: XAML takes the render thread's device here, then the render thread presents.
pub(crate) unsafe fn attach_xaml_surface(context: *mut CanvasRenderingContext2D, source: *mut c_void) -> bool {
    let Some(target) = (unsafe { &*context }).render_target() else { return false };
    let Some(source_ref) = (unsafe { IUnknown::from_raw_borrowed(&source) }).cloned() else { return false };
    let Some((lost, device)) = target.sync(|real| (real.gpu_lost(), real.get_context().xaml_device().map(Sendable))) else {
        return false;
    };
    if lost {
        remember(context, Shown::Xaml { source: source_ref, handoff: None });
        return true;
    }
    let Some(Sendable((device, width, height))) = device else { return false };
    let handoff = match unsafe { XamlHandoff::new(source, &device, width, height) } {
        Ok(handoff) => handoff,
        Err(error) => {
            log::error!("canvas: the XAML surface cannot be drawn from the render thread: {error}");
            return false;
        }
    };
    let shared = Arc::clone(&handoff);
    let attached = target
        .sync(move |real| real.get_context_mut().attach_xaml_handoff(shared))
        .unwrap_or(false);
    if attached {
        remember(context, Shown::Xaml { source: source_ref, handoff: Some(handoff) });
    }
    attached
}

/// Before a restore: whatever still shows a lost canvas holds its removed device, and the adapter
/// makes no new device while it is held. (Unthreaded canvases do this in `release_lost_device`;
/// here it has to happen on the UI thread.)
fn release_lost_shown() {
    let entries: Vec<(usize, Shown)> =
        SHOWN.with(|entries| entries.borrow().iter().map(|(key, shown)| (*key, shown.clone())).collect());
    for (key, shown) in entries {
        // Registered canvases are alive: `canvas_native_context_release` forgets them.
        let Some(target) = (unsafe { &*(key as *const CanvasRenderingContext2D) }).render_target() else { continue };
        if !target.sync(|real| real.gpu_lost()).unwrap_or(false) {
            continue;
        }
        match shown {
            Shown::Panel(panel) => unsafe { CompositionSwapChain::unbind_panel(panel.as_raw()) },
            Shown::Xaml { handoff: Some(handoff), .. } => handoff.release_device(),
            Shown::Xaml { handoff: None, .. } => {}
        }
    }
}

/// UI thread: `restore_d3d` on the render thread, then shown again where it was (or in `panel`).
pub(crate) unsafe fn restore(context: *mut CanvasRenderingContext2D, panel: *mut c_void) -> bool {
    let Some(target) = (unsafe { &*context }).render_target() else { return false };
    if !target.sync(|real| real.gpu_lost()).unwrap_or(false) {
        return true;
    }
    release_lost_shown();
    let restored = target
        .sync(|real| {
            let context = real.get_context_mut();
            let restored = unsafe { context.restore_d3d(std::ptr::null_mut()) };
            if restored {
                note_render_thread_device(context);
            }
            restored
        })
        .unwrap_or(false);
    if !restored {
        return false;
    }
    // The recorder answers state queries: default state too, and nothing drawn while lost.
    let recorder = unsafe { &mut *context }.get_context_mut();
    let (width, height) = (recorder.surface_data().width(), recorder.surface_data().height());
    recorder.resize_recording(width, height);
    match SHOWN.with(|entries| entries.borrow().get(&(context as usize)).cloned()) {
        Some(Shown::Panel(panel)) => unsafe { attach_swap_chain_panel(context, panel.as_raw()) },
        Some(Shown::Xaml { source, .. }) => unsafe { attach_xaml_surface(context, source.as_raw()) },
        None => panel.is_null() || unsafe { attach_swap_chain_panel(context, panel) },
    }
}

/// The render thread's device's adapter and whether it is WARP, if it made one.
pub fn render_thread_adapter() -> Option<(String, bool)> {
    crate::c2d::render_thread::on_render_thread(|| {
        canvas_core::gpu::d3d::D3D12Context::current_shared().map(|device| (device.adapter_name(), device.is_warp()))
    })
    .flatten()
}

struct OwnedHandle(HANDLE);
unsafe impl Send for OwnedHandle {}

impl OwnedHandle {
    /// The producer can close its handle before the frame is drawn.
    unsafe fn duplicate(handle: *mut c_void) -> Option<Self> {
        let process = unsafe { GetCurrentProcess() };
        let mut duplicate = HANDLE::default();
        unsafe { DuplicateHandle(process, HANDLE(handle), process, &mut duplicate, 0, false, DUPLICATE_SAME_ACCESS) }.ok()?;
        Some(Self(duplicate))
    }
}

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(self.0) };
    }
}

/// A `CanvasD3DSharedFrame` without the producer's handles.
struct SharedFrame {
    texture_id: u64,
    ready_fence_id: u64,
    ready_value: u64,
    release_fence_id: u64,
    release_value: u64,
    texture: OwnedHandle,
    ready: OwnedHandle,
    release: OwnedHandle,
}

impl SharedFrame {
    /// On the rasterizing context: the texture is sampled once the producer's ready fence is
    /// reached, and given back (the release fence) once that sampling is submitted.
    fn image(self, context: &mut Context, width: i32, height: i32) -> Option<(canvas_2d::context::Image, ExternalRelease)> {
        let texture: ID3D12Resource = context.d3d_open_shared(self.texture_id, self.texture.0)?;
        let ready: ID3D12Fence = context.d3d_open_shared(self.ready_fence_id, self.ready.0)?;
        let release: ID3D12Fence = context.d3d_open_shared(self.release_fence_id, self.release.0)?;
        let device = context.d3d_device()?;
        unsafe { device.queue().Wait(&ready, self.ready_value) }.ok()?;
        let release_value = self.release_value;
        let image = context.d3d_borrow_texture(&texture, width, height)?;
        let release = Sendable(release);
        let give_back: ExternalRelease = Box::new(move |context: &mut Context| {
            let Sendable(release) = release;
            if let Some(direct_context) = context.gpu_context() {
                direct_context.flush_and_submit();
            }
            if let Some(device) = context.d3d_device() {
                let _ = unsafe { device.queue().Signal(&release, release_value) };
            }
        });
        Some((image, give_back))
    }
}

/// Draws a video frame another device shares (canvas-media's `gpuFrame()`: a
/// `CanvasD3DSharedFrame`) on the context's device, without a readback; threaded contexts draw it
/// on the render thread's. `false` when it cannot (another adapter, not a D3D context), and the
/// frame is then not consumed: the caller draws it some other way.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_context_draw_d3d_shared_frame(
    context: *mut CanvasRenderingContext2D,
    frame: *mut c_void,
    width: i32,
    height: i32,
    sx: f32,
    sy: f32,
    sw: f32,
    sh: f32,
    dx: f32,
    dy: f32,
    dw: f32,
    dh: f32,
) -> bool {
    let Some(context) = (unsafe { context.as_mut() }) else { return false };
    let Some(desc) = (unsafe { (frame as *const CanvasD3DSharedFrame).as_ref() })
        .filter(|desc| desc.size as usize == std::mem::size_of::<CanvasD3DSharedFrame>())
        .copied()
    else {
        return false;
    };
    if desc.texture.is_null() || desc.ready_fence.is_null() || desc.release_fence.is_null() || width <= 0 || height <= 0 {
        return false;
    }
    let adapter = if context.is_threaded() {
        RENDER_ADAPTER.load(Ordering::Acquire)
    } else {
        context.get_context().d3d_device().map_or(0, |device| device.adapter_luid())
    };
    if adapter == 0 || adapter != desc.adapter_luid {
        return false;
    }
    let (Some(texture), Some(ready), Some(release)) = (unsafe {
        (
            OwnedHandle::duplicate(desc.texture),
            OwnedHandle::duplicate(desc.ready_fence),
            OwnedHandle::duplicate(desc.release_fence),
        )
    }) else {
        return false;
    };
    // The producer keeps the texture until the release fence says it was read.
    unsafe { std::ptr::write_volatile(&mut (*(frame as *mut CanvasD3DSharedFrame)).consumed, 1) };
    let shared = SharedFrame {
        texture_id: desc.texture_id,
        ready_fence_id: desc.ready_fence_id,
        ready_value: desc.ready_value,
        release_fence_id: desc.release_fence_id,
        release_value: desc.release_value,
        texture,
        ready,
        release,
    };
    let make: ExternalImage = Box::new(move |context: &mut Context| shared.image(context, width, height));
    context.get_context_mut().draw_external_image(
        width,
        height,
        skia_safe::Rect::from_xywh(sx, sy, sw, sh),
        skia_safe::Rect::from_xywh(dx, dy, dw, dh),
        make,
    );
    true
}
