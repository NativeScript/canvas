//! WebGPU on a SwapChainPanel: wgpu binds each swapchain it makes with
//! `ISwapChainPanelNative::SetSwapChain`, which a XAML panel only takes on its own thread. Presents,
//! and so the reconfigurations a resize leaves pending, run on the device's queue thread.
//!
//!     cargo test -p canvas-c --features 2d,webgl,gl,d3d --test webgpu_panel_d3d
#![cfg(all(target_os = "windows", feature = "d3d"))]

use std::ffi::{c_char, c_void};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use canvas_c::webgpu::gpu::{CanvasGPURequestAdapterOptions, CanvasWebGPUInstance};
use canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter;
use canvas_c::webgpu::gpu_canvas_context::*;
use canvas_c::webgpu::gpu_device::CanvasGPUDevice;
use canvas_core::gpu::dxgi::{ISwapChainPanelNative, ISwapChainPanelNative_Impl};
use windows::core::{IUnknown, Interface, HRESULT};
use windows::Win32::Graphics::Dxgi::IDXGISwapChain1;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE};

const RPC_E_WRONG_THREAD: HRESULT = HRESULT(0x8001010E_u32 as i32);

#[derive(Default)]
struct PanelLog {
    swap_chain: Option<IDXGISwapChain1>,
    binds: usize,
    wrong_thread: usize,
}

/// Like a XAML SwapChainPanel: only its thread may set the swapchain.
#[windows_core::implement(ISwapChainPanelNative)]
struct UiPanel {
    thread: u32,
    log: Arc<Mutex<PanelLog>>,
}

impl ISwapChainPanelNative_Impl for UiPanel_Impl {
    unsafe fn SetSwapChain(&self, swap_chain: *mut c_void) -> HRESULT {
        let mut log = self.log.lock().unwrap();
        if unsafe { GetCurrentThreadId() } != self.thread {
            log.wrong_thread += 1;
            return RPC_E_WRONG_THREAD;
        }
        log.binds += 1;
        log.swap_chain = unsafe { IUnknown::from_raw_borrowed(&swap_chain) }.and_then(|unknown| unknown.cast().ok());
        HRESULT(0)
    }
}

fn pump() {
    let mut msg = MSG::default();
    unsafe {
        while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

extern "C" fn adapter_ready(adapter: *const CanvasGPUAdapter, data: *mut c_void) {
    let sender = unsafe { &*(data as *const mpsc::Sender<usize>) };
    let _ = sender.send(adapter as usize);
}

extern "C" fn device_ready(_error: *mut c_char, device: *const CanvasGPUDevice, data: *mut c_void) {
    let sender = unsafe { &*(data as *const mpsc::Sender<usize>) };
    let _ = sender.send(device as usize);
}

fn device(instance: *const CanvasWebGPUInstance) -> Option<*const CanvasGPUDevice> {
    let (sender, receiver) = mpsc::channel::<usize>();
    let data = &sender as *const _ as *mut c_void;
    let options = CanvasGPURequestAdapterOptions::default();
    unsafe { canvas_c::webgpu::gpu::canvas_native_webgpu_request_adapter(instance, &options, adapter_ready, data) };
    let adapter = receiver.recv_timeout(Duration::from_secs(20)).ok()? as *const CanvasGPUAdapter;
    if adapter.is_null() {
        return None;
    }
    canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_request_device(
        adapter,
        std::ptr::null(),
        std::ptr::null(),
        0,
        std::ptr::null(),
        device_ready,
        data,
    );
    let device = receiver.recv_timeout(Duration::from_secs(20)).ok()? as *const CanvasGPUDevice;
    (!device.is_null()).then_some(device)
}

fn frame(context: *const CanvasGPUCanvasContext) {
    let texture = canvas_native_webgpu_context_get_current_texture(context);
    assert!(!texture.is_null());
    unsafe { canvas_native_webgpu_context_present_surface(context, texture) };
    pump();
}

fn bound_width(log: &Mutex<PanelLog>) -> Option<u32> {
    let log = log.lock().unwrap();
    let desc = unsafe { log.swap_chain.as_ref()?.GetDesc1() }.ok()?;
    Some(desc.Width)
}

#[test]
fn a_resize_rebinds_the_swapchain_on_the_panel_thread() {
    let instance = canvas_c::webgpu::gpu::canvas_native_webgpu_instance_create();
    let Some(device) = device(instance) else {
        eprintln!("no Direct3D 12 adapter: skipped");
        return;
    };
    let log = Arc::new(Mutex::new(PanelLog::default()));
    let panel: ISwapChainPanelNative = UiPanel {
        thread: unsafe { GetCurrentThreadId() },
        log: log.clone(),
    }
    .into();
    let context = unsafe {
        canvas_native_webgpu_context_create_swap_chain_panel(instance, panel.as_raw(), 64, 48)
    };
    assert!(!context.is_null());

    let config = CanvasGPUSurfaceConfiguration {
        alphaMode: CanvasGPUSurfaceAlphaMode::Opaque,
        // RENDER_ATTACHMENT | COPY_DST | COPY_SRC, as GPUCanvasContext.configure passes it.
        usage: 0x13,
        presentMode: CanvasGPUPresentMode::Fifo,
        view_formats: std::ptr::null(),
        view_formats_size: 0,
        size: std::ptr::null(),
        format: canvas_c::webgpu::enums::CanvasOptionalGPUTextureFormat::None,
    };
    unsafe { canvas_native_webgpu_context_configure(context, device, &config) };
    for _ in 0..3 {
        frame(context);
    }
    assert_eq!(bound_width(&log), Some(64));

    // Applied by the queue thread's next present.
    unsafe { canvas_native_webgpu_context_resize_swap_chain_panel(context, 96, 72) };
    let deadline = Instant::now() + Duration::from_secs(10);
    while bound_width(&log) != Some(96) && Instant::now() < deadline {
        frame(context);
        std::thread::sleep(Duration::from_millis(16));
    }
    {
        let log = log.lock().unwrap();
        assert_eq!(log.wrong_thread, 0, "SetSwapChain called off the panel's thread");
        assert!(log.binds >= 2, "binds: {}", log.binds);
    }
    assert_eq!(bound_width(&log), Some(96));

    unsafe {
        canvas_native_webgpu_context_release(context);
        canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_release(device);
    }
    canvas_c::webgpu::gpu::canvas_native_webgpu_instance_release(instance);
}
