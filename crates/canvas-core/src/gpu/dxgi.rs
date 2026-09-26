//! Composition swapchains presented through a WinUI `SwapChainPanel`, on a D3D12 queue (2D) or
//! a D3D11 device (WebGL on ANGLE).
//!
//! The panel only accepts swapchains created with `CreateSwapChainForComposition`, and the panel
//! lays them out in DIPs: the swapchain is sized in physical pixels and `SetMatrixTransform`
//! maps it back (1 / composition scale), which is also how the canvas "fit" modes are applied
//! without resizing buffers.

use std::ffi::c_void;

use windows::core::{Interface, Result, HRESULT};
use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0};
#[cfg(feature = "gl")]
use windows::Win32::Graphics::Direct3D11::ID3D11Device;
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_ALPHA_MODE_IGNORE, DXGI_ALPHA_MODE_PREMULTIPLIED, DXGI_FORMAT_B8G8R8A8_UNORM,
    DXGI_SAMPLE_DESC,
};
use windows::Win32::Graphics::Dxgi::{
    IDXGIAdapter, IDXGIDevice, IDXGIFactory2, IDXGISwapChain1, IDXGISwapChain2, IDXGISwapChain3, DXGI_MATRIX_3X2_F,
    DXGI_PRESENT, DXGI_PRESENT_DO_NOT_WAIT, DXGI_SCALING_STRETCH, DXGI_SWAP_CHAIN_DESC1,
    DXGI_SWAP_CHAIN_FLAG, DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT,
    DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT,
};
use windows::Win32::System::Threading::WaitForSingleObjectEx;

#[cfg(feature = "d3d")]
use crate::gpu::d3d::D3D12Context;

/// WinUI 3's `ISwapChainPanelNative` (microsoft.ui.xaml.media.dxinterop.h). Not the UWP interface
/// of the same name, which has a different IID.
#[windows::core::interface("63aad0b8-7c24-40ff-85a8-640d944cc325")]
pub unsafe trait ISwapChainPanelNative: windows::core::IUnknown {
    fn SetSwapChain(&self, swap_chain: *mut c_void) -> HRESULT;
}

pub const BUFFER_COUNT: u32 = 2;

/// A `SwapChainPanel` for a library that binds its own swapchain to it (wgpu takes an
/// `ISwapChainPanelNative` and calls `SetSwapChain` itself). The library gets a stand-in that
/// forwards to the panel and remembers the swapchain, so its matrix transform (DPI scale and the
/// canvas fit) stays ours to set, across the library's reconfigurations.
pub struct PanelSurfaceTarget {
    proxy: ISwapChainPanelNative,
    panel: ISwapChainPanelNative,
    state: std::sync::Arc<parking_lot::Mutex<ProxyState>>,
}

// Only used on the UI thread; the wrapper travels with the context that owns it.
unsafe impl Send for PanelSurfaceTarget {}
unsafe impl Sync for PanelSurfaceTarget {}

#[derive(Default)]
struct ProxyState {
    swap_chain: Option<IDXGISwapChain2>,
    transform: Option<DXGI_MATRIX_3X2_F>,
}

#[windows_core::implement(ISwapChainPanelNative)]
struct PanelProxy {
    panel: ISwapChainPanelNative,
    state: std::sync::Arc<parking_lot::Mutex<ProxyState>>,
}

impl ISwapChainPanelNative_Impl for PanelProxy_Impl {
    unsafe fn SetSwapChain(&self, swap_chain: *mut c_void) -> HRESULT {
        let mut state = self.state.lock();
        state.swap_chain = unsafe { windows::core::IUnknown::from_raw_borrowed(&swap_chain) }
            .and_then(|unknown| unknown.cast::<IDXGISwapChain2>().ok());
        if let (Some(swap_chain), Some(transform)) = (state.swap_chain.as_ref(), state.transform.as_ref()) {
            let _ = unsafe { swap_chain.SetMatrixTransform(transform) };
        }
        unsafe { self.panel.SetSwapChain(swap_chain) }
    }
}

impl PanelSurfaceTarget {
    /// `panel`: any COM pointer of the `SwapChainPanel`. UI thread.
    pub unsafe fn new(panel: *mut c_void) -> Result<Self> {
        let unknown = unsafe { windows::core::IUnknown::from_raw_borrowed(&panel) }.ok_or_else(windows::core::Error::empty)?;
        let panel: ISwapChainPanelNative = unknown.cast()?;
        let state = std::sync::Arc::new(parking_lot::Mutex::new(ProxyState::default()));
        let proxy: ISwapChainPanelNative = PanelProxy {
            panel: panel.clone(),
            state: state.clone(),
        }
        .into();
        Ok(Self { proxy, panel, state })
    }

    /// The `ISwapChainPanelNative` to hand to the library.
    pub fn as_raw(&self) -> *mut c_void {
        self.proxy.as_raw()
    }

    /// Scale then translate the swapchain inside the panel (DIPs = pixels * scale + offset); kept
    /// for swapchains the library binds later.
    pub fn set_transform(&self, scale_x: f32, scale_y: f32, offset_x: f32, offset_y: f32) -> Result<()> {
        let transform = DXGI_MATRIX_3X2_F {
            _11: scale_x,
            _22: scale_y,
            _31: offset_x,
            _32: offset_y,
            ..Default::default()
        };
        let mut state = self.state.lock();
        state.transform = Some(transform);
        match state.swap_chain.as_ref() {
            Some(swap_chain) => unsafe { swap_chain.SetMatrixTransform(&transform) },
            None => Ok(()),
        }
    }
}

impl Drop for PanelSurfaceTarget {
    fn drop(&mut self) {
        // Detach whatever the library bound, so the panel does not keep a dead swapchain.
        let _ = unsafe { self.panel.SetSwapChain(std::ptr::null_mut()) };
    }
}

/// A stand-in `SwapChainPanel` for tests without a XAML window: it accepts (and holds) the
/// swapchain it is given, so the on-screen paths (binding, presenting, resizing, restoring) run
/// headless. Nothing is shown.
#[windows_core::implement(ISwapChainPanelNative)]
struct HeadlessPanel {
    swap_chain: parking_lot::Mutex<Option<windows::core::IUnknown>>,
}

impl ISwapChainPanelNative_Impl for HeadlessPanel_Impl {
    unsafe fn SetSwapChain(&self, swap_chain: *mut c_void) -> HRESULT {
        *self.swap_chain.lock() = unsafe { windows::core::IUnknown::from_raw_borrowed(&swap_chain) }.cloned();
        HRESULT(0)
    }
}

/// A new `HeadlessPanel`, as a COM pointer the caller owns one reference to.
pub fn headless_panel() -> windows::core::IUnknown {
    let panel: ISwapChainPanelNative = HeadlessPanel {
        swap_chain: parking_lot::Mutex::new(None),
    }
    .into();
    panel.into()
}

pub struct CompositionSwapChain {
    swap_chain: IDXGISwapChain3,
    waitable: HANDLE,
    width: u32,
    height: u32,
    flags: DXGI_SWAP_CHAIN_FLAG,
}

impl CompositionSwapChain {
    /// A flip-model BGRA swapchain on `device`'s direct queue, `width` x `height` physical pixels.
    #[cfg(feature = "d3d")]
    pub fn new(device: &D3D12Context, width: u32, height: u32, alpha: bool) -> Result<Self> {
        let factory: IDXGIFactory2 = device.factory().cast()?;
        Self::create(&factory, &device.queue().cast()?, width, height, alpha)
    }

    /// The same on a D3D11 device (ANGLE's, for WebGL).
    #[cfg(feature = "gl")]
    pub fn new_d3d11(device: &ID3D11Device, width: u32, height: u32, alpha: bool) -> Result<Self> {
        let adapter: IDXGIAdapter = unsafe { device.cast::<IDXGIDevice>()?.GetAdapter() }?;
        let factory: IDXGIFactory2 = unsafe { adapter.GetParent() }?;
        Self::create(&factory, &device.cast()?, width, height, alpha)
    }

    /// `device`: the D3D12 command queue or the D3D11 device that renders into the buffers.
    fn create(
        factory: &IDXGIFactory2,
        device: &windows::core::IUnknown,
        width: u32,
        height: u32,
        alpha: bool,
    ) -> Result<Self> {
        let flags = DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT;
        let desc = DXGI_SWAP_CHAIN_DESC1 {
            Width: width.max(1),
            Height: height.max(1),
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: BUFFER_COUNT,
            Scaling: DXGI_SCALING_STRETCH,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            AlphaMode: if alpha {
                DXGI_ALPHA_MODE_PREMULTIPLIED
            } else {
                DXGI_ALPHA_MODE_IGNORE
            },
            Flags: flags.0 as u32,
            ..Default::default()
        };
        let swap_chain: IDXGISwapChain1 =
            unsafe { factory.CreateSwapChainForComposition(device, &desc, None) }?;
        let swap_chain: IDXGISwapChain3 = swap_chain.cast()?;
        let waitable = {
            let swap_chain2: IDXGISwapChain2 = swap_chain.cast()?;
            unsafe { swap_chain2.SetMaximumFrameLatency(2) }?;
            unsafe { swap_chain2.GetFrameLatencyWaitableObject() }
        };
        Ok(Self {
            swap_chain,
            waitable,
            width: width.max(1),
            height: height.max(1),
            flags,
        })
    }

    /// Shows this swapchain in the panel. `panel` is any COM pointer of the `SwapChainPanel` (the
    /// runtime's `NSWinRT.interop.pointerKey(panel.handle)`). Must run on the UI thread.
    pub unsafe fn bind_panel(&self, panel: *mut c_void) -> Result<()> {
        let unknown = windows::core::IUnknown::from_raw_borrowed(&panel)
            .ok_or_else(windows::core::Error::empty)?;
        let native: ISwapChainPanelNative = unknown.cast()?;
        native.SetSwapChain(self.swap_chain.as_raw()).ok()
    }

    /// Detaches whatever swapchain the panel shows.
    pub unsafe fn unbind_panel(panel: *mut c_void) {
        if let Some(unknown) = windows::core::IUnknown::from_raw_borrowed(&panel) {
            if let Ok(native) = unknown.cast::<ISwapChainPanelNative>() {
                let _ = native.SetSwapChain(std::ptr::null_mut());
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn current_index(&self) -> u32 {
        unsafe { self.swap_chain.GetCurrentBackBufferIndex() }
    }

    /// Back buffer `index` as a D3D12 resource, or (D3D11, where only buffer 0 -- the current
    /// back buffer -- is accessible) an `ID3D11Texture2D`.
    pub fn buffer<T: Interface>(&self, index: u32) -> Result<T> {
        unsafe { self.swap_chain.GetBuffer(index) }
    }

    /// Resizes the buffers. Every reference to the old buffers must have been released first.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        let (width, height) = (width.max(1), height.max(1));
        unsafe {
            self.swap_chain.ResizeBuffers(
                BUFFER_COUNT,
                width,
                height,
                DXGI_FORMAT_B8G8R8A8_UNORM,
                self.flags,
            )
        }?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    /// `true` when a new frame can be presented without blocking (frame-latency waitable).
    pub fn frame_ready(&self) -> bool {
        if self.waitable.is_invalid() {
            return true;
        }
        unsafe { WaitForSingleObjectEx(self.waitable, 0, false) == WAIT_OBJECT_0 }
    }

    pub fn present(&self, vsync: bool) -> Result<()> {
        let (interval, flags) = if vsync {
            (1, DXGI_PRESENT::default())
        } else {
            (0, DXGI_PRESENT_DO_NOT_WAIT)
        };
        unsafe { self.swap_chain.Present(interval, flags) }.ok()
    }

    /// Scale then translate the swapchain inside the panel (DIPs = pixels * scale + offset).
    pub fn set_transform(&self, scale_x: f32, scale_y: f32, offset_x: f32, offset_y: f32) -> Result<()> {
        let swap_chain2: IDXGISwapChain2 = self.swap_chain.cast()?;
        let matrix = DXGI_MATRIX_3X2_F {
            _11: scale_x,
            _22: scale_y,
            _31: offset_x,
            _32: offset_y,
            ..Default::default()
        };
        unsafe { swap_chain2.SetMatrixTransform(&matrix) }
    }
}

impl Drop for CompositionSwapChain {
    fn drop(&mut self) {
        if !self.waitable.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.waitable);
            }
        }
    }
}
