//! Composition swapchains presented through a WinUI `SwapChainPanel`.
//!
//! The panel only accepts swapchains created with `CreateSwapChainForComposition`, and the panel
//! lays them out in DIPs: the swapchain is sized in physical pixels and `SetMatrixTransform`
//! maps it back (1 / composition scale), which is also how the canvas "fit" modes are applied
//! without resizing buffers.

use std::ffi::c_void;

use windows::core::{Interface, Result, HRESULT};
use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0};
use windows::Win32::Graphics::Direct3D12::ID3D12Resource;
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_ALPHA_MODE_IGNORE, DXGI_ALPHA_MODE_PREMULTIPLIED, DXGI_FORMAT_B8G8R8A8_UNORM,
    DXGI_SAMPLE_DESC,
};
use windows::Win32::Graphics::Dxgi::{
    IDXGIFactory2, IDXGISwapChain1, IDXGISwapChain2, IDXGISwapChain3, DXGI_MATRIX_3X2_F,
    DXGI_PRESENT, DXGI_PRESENT_DO_NOT_WAIT, DXGI_SCALING_STRETCH, DXGI_SWAP_CHAIN_DESC1,
    DXGI_SWAP_CHAIN_FLAG, DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT,
    DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT,
};
use windows::Win32::System::Threading::WaitForSingleObjectEx;

use crate::gpu::d3d::D3D12Context;

/// WinUI 3's `ISwapChainPanelNative` (microsoft.ui.xaml.media.dxinterop.h). Not the UWP interface
/// of the same name, which has a different IID.
#[windows::core::interface("63aad0b8-7c24-40ff-85a8-640d944cc325")]
pub unsafe trait ISwapChainPanelNative: windows::core::IUnknown {
    fn SetSwapChain(&self, swap_chain: *mut c_void) -> HRESULT;
}

pub const BUFFER_COUNT: u32 = 2;

pub struct CompositionSwapChain {
    swap_chain: IDXGISwapChain3,
    waitable: HANDLE,
    width: u32,
    height: u32,
    flags: DXGI_SWAP_CHAIN_FLAG,
}

impl CompositionSwapChain {
    /// A flip-model BGRA swapchain on `device`'s direct queue, `width` x `height` physical pixels.
    pub fn new(device: &D3D12Context, width: u32, height: u32, alpha: bool) -> Result<Self> {
        let factory: IDXGIFactory2 = device.factory().cast()?;
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
            unsafe { factory.CreateSwapChainForComposition(device.queue(), &desc, None) }?;
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

    pub fn buffer(&self, index: u32) -> Result<ID3D12Resource> {
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
