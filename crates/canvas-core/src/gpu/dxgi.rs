//! Composition swapchains presented through a WinUI `SwapChainPanel`, on a D3D12 queue (2D) or
//! a D3D11 device (WebGL on ANGLE).
//!
//! The panel only accepts swapchains created with `CreateSwapChainForComposition`, and the panel
//! lays them out in DIPs: the swapchain is sized in physical pixels and `SetMatrixTransform`
//! maps it back (1 / composition scale), which is also how the canvas "fit" modes are applied
//! without resizing buffers.

use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

use windows::core::{Interface, Result, GUID, HRESULT};
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

const MAX_FRAME_DEFERRAL: Duration = Duration::from_millis(250);

thread_local! {
    static PRESENT_DEFERRED: Cell<bool> = const { Cell::new(false) };
}

pub fn take_present_deferred() -> bool {
    PRESENT_DEFERRED.with(|deferred| deferred.replace(false))
}

/// WinUI 3's `ISurfaceImageSourceNative` (microsoft.ui.xaml.media.dxinterop.h; not the UWP IID).
#[windows::core::interface("e4cecd6c-f14b-4f46-83c3-8bbda27c6504")]
pub unsafe trait ISurfaceImageSourceNative: windows::core::IUnknown {
    fn SetDevice(&self, device: *mut c_void) -> HRESULT;
    fn BeginDraw(
        &self,
        update_rect: windows::Win32::Foundation::RECT,
        surface: *mut *mut c_void,
        offset: *mut windows::Win32::Foundation::POINT,
    ) -> HRESULT;
    fn EndDraw(&self) -> HRESULT;
}

/// WinUI 3's `ISurfaceImageSourceNativeWithD2D`: the one XAML lets other threads draw through.
#[windows::core::interface("cb833102-d5d1-448b-a31a-52a9509f24e6")]
pub unsafe trait ISurfaceImageSourceNativeWithD2D: windows::core::IUnknown {
    fn SetDevice(&self, device: *mut c_void) -> HRESULT;
    fn BeginDraw(
        &self,
        update_rect: *const windows::Win32::Foundation::RECT,
        iid: *const GUID,
        update_object: *mut *mut c_void,
        offset: *mut windows::Win32::Foundation::POINT,
    ) -> HRESULT;
    fn EndDraw(&self) -> HRESULT;
    fn SuspendDraw(&self) -> HRESULT;
    fn ResumeDraw(&self) -> HRESULT;
}

/// A XAML `SurfaceImageSource` a canvas presents into when it has to blend with the page: XAML
/// composites it like any image. (A SwapChainPanel is external content in WinUI 3: nothing
/// behind it shows through, whatever the swapchain's alpha mode.) Frames are copied in, BGRA
/// premultiplied, with a D3D11 device.
pub struct XamlSurface {
    native: XamlNative,
    context: windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext,
    width: u32,
    height: u32,
}

enum XamlNative {
    UiThread(ISurfaceImageSourceNative),
    Handoff(Arc<XamlHandoff>),
}

impl XamlSurface {
    /// `source`: any COM pointer of the `SurfaceImageSource` (made `width` x `height`, not
    /// opaque). `device`: the D3D11 device frames are copied with. UI thread.
    pub unsafe fn new(
        source: *mut c_void,
        device: &windows::Win32::Graphics::Direct3D11::ID3D11Device,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let unknown = unsafe { windows::core::IUnknown::from_raw_borrowed(&source) }
            .ok_or_else(windows::core::Error::empty)?;
        let native: ISurfaceImageSourceNative = unknown.cast()?;
        let dxgi: windows::Win32::Graphics::Dxgi::IDXGIDevice = device.cast()?;
        unsafe { native.SetDevice(dxgi.as_raw()) }.ok()?;
        let context = unsafe { device.GetImmediateContext() }?;
        Ok(Self {
            native: XamlNative::UiThread(native),
            context,
            width,
            height,
        })
    }

    /// Presents through `handoff` (made on the UI thread with `device`) from the calling thread.
    pub fn with_handoff(
        handoff: Arc<XamlHandoff>,
        device: &windows::Win32::Graphics::Direct3D11::ID3D11Device,
    ) -> Result<Self> {
        let context = unsafe { device.GetImmediateContext() }?;
        let (width, height) = (handoff.width, handoff.height);
        Ok(Self {
            native: XamlNative::Handoff(handoff),
            context,
            width,
            height,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Presented from another thread: the UI thread shows its frames and releases its device.
    pub fn is_handoff(&self) -> bool {
        matches!(self.native, XamlNative::Handoff(_))
    }

    /// The `SurfaceImageSource`, to attach again (e.g. on a new device).
    pub fn source(&self) -> windows::core::IUnknown {
        match &self.native {
            XamlNative::UiThread(native) => native.clone().into(),
            XamlNative::Handoff(handoff) => handoff.source(),
        }
    }

    /// Makes the image let go of the device (lost), until it is attached again. XAML keeps the
    /// device it was given until it is given another (`SetDevice(null)` does not release it), so
    /// it gets a stand-in; it lets go of the old one shortly after, not during the call. UI thread.
    /// A handoff's is released through [`XamlHandoff::release_device`].
    pub fn release_device(&self) {
        let XamlNative::UiThread(native) = &self.native else { return };
        let (width, height) = (self.width, self.height);
        blank_with_stand_in(
            |device| unsafe { native.SetDevice(device) },
            |rect, surface, offset| unsafe { native.BeginDraw(*rect, surface, offset) },
            || unsafe { native.EndDraw() },
            width,
            height,
        );
    }

    /// Copies `texture` (on this surface's device, this surface's size) in. UI thread, or for a
    /// handoff the thread it was made for.
    pub fn present(&self, texture: &windows::Win32::Graphics::Direct3D11::ID3D11Resource) -> Result<()> {
        let native = match &self.native {
            XamlNative::UiThread(native) => native,
            XamlNative::Handoff(handoff) => return handoff.present(&self.context, texture),
        };
        let rect = windows::Win32::Foundation::RECT {
            left: 0,
            top: 0,
            right: self.width as i32,
            bottom: self.height as i32,
        };
        let mut surface = std::ptr::null_mut();
        let mut offset = windows::Win32::Foundation::POINT::default();
        unsafe { native.BeginDraw(rect, &mut surface, &mut offset) }.ok()?;
        // The update rectangle lives in XAML's atlas at `offset`.
        let copied = unsafe { windows::Win32::Graphics::Dxgi::IDXGISurface::from_raw(surface) }
            .cast::<windows::Win32::Graphics::Direct3D11::ID3D11Resource>()
            .map(|target| unsafe { copy_into(&self.context, &target, offset, texture) });
        let ended = unsafe { native.EndDraw() }.ok();
        copied.and(ended)
    }
}

unsafe fn copy_into(
    context: &windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext,
    target: &windows::Win32::Graphics::Direct3D11::ID3D11Resource,
    offset: windows::Win32::Foundation::POINT,
    texture: &windows::Win32::Graphics::Direct3D11::ID3D11Resource,
) {
    unsafe {
        context.CopySubresourceRegion(target, 0, offset.x.max(0) as u32, offset.y.max(0) as u32, 0, texture, 0, None)
    }
}

/// Gives the image the stand-in device and draws it blank: XAML keeps what it last drew with the
/// old device until it draws again.
fn blank_with_stand_in(
    set_device: impl FnOnce(*mut c_void) -> HRESULT,
    begin: impl FnOnce(&windows::Win32::Foundation::RECT, *mut *mut c_void, *mut windows::Win32::Foundation::POINT) -> HRESULT,
    end: impl FnOnce() -> HRESULT,
    width: u32,
    height: u32,
) {
    use windows::Win32::Graphics::Direct3D11::{ID3D11RenderTargetView, ID3D11Resource};
    let Some((device, context)) = stand_in_device() else { return };
    let Ok(dxgi) = device.cast::<windows::Win32::Graphics::Dxgi::IDXGIDevice>() else { return };
    if set_device(dxgi.as_raw()).is_err() {
        return;
    }
    let rect = windows::Win32::Foundation::RECT {
        left: 0,
        top: 0,
        right: width as i32,
        bottom: height as i32,
    };
    let mut surface = std::ptr::null_mut();
    let mut offset = windows::Win32::Foundation::POINT::default();
    if begin(&rect, &mut surface, &mut offset).is_err() {
        return;
    }
    if let Ok(target) = unsafe { windows::Win32::Graphics::Dxgi::IDXGISurface::from_raw(surface) }.cast::<ID3D11Resource>() {
        let mut view: Option<ID3D11RenderTargetView> = None;
        if unsafe { device.CreateRenderTargetView(&target, None, Some(&mut view)) }.is_ok() {
            // Only the update rectangle: the surface can be an atlas shared with other images.
            let area = windows::Win32::Foundation::RECT {
                left: offset.x,
                top: offset.y,
                right: offset.x + width as i32,
                bottom: offset.y + height as i32,
            };
            if let Some(view) = view {
                unsafe { context.ClearView(&view, &[0.0; 4], Some(&[area])) };
            }
        }
    }
    let _ = end();
}

/// A `SurfaceImageSource` presented from a render thread. XAML lets any thread begin (or resume)
/// and suspend a draw, but only the UI thread end it, which is what shows the frame; so the
/// render thread posts the UI thread a message to end it, and never waits on it. Frames drawn
/// before the UI thread gets to it land in the same draw.
pub struct XamlHandoff {
    native: ISurfaceImageSourceNativeWithD2D,
    width: u32,
    height: u32,
    draw: parking_lot::Mutex<HandoffDraw>,
    wake_queued: AtomicBool,
    /// The UI thread's message-only window (`wake_window`).
    window: isize,
}

#[derive(Default)]
struct HandoffDraw {
    /// Begun, and suspended between frames, until the UI thread ends it.
    begun: bool,
    /// The begun draw's update surface, and where the image sits in it.
    target: Option<(windows::Win32::Graphics::Direct3D11::ID3D11Resource, windows::Win32::Foundation::POINT)>,
    /// Lost: the image has the stand-in device until attached again.
    released: bool,
}

// XAML makes the interface for use from other threads; the device is multithread-protected.
unsafe impl Send for XamlHandoff {}
unsafe impl Sync for XamlHandoff {}

const WM_END_XAML_DRAWS: u32 = windows::Win32::UI::WindowsAndMessaging::WM_APP + 0x2d0;

thread_local! {
    static WAKE_WINDOW: Cell<isize> = const { Cell::new(0) };
    static HANDOFFS: RefCell<Vec<Weak<XamlHandoff>>> = const { RefCell::new(Vec::new()) };
}

unsafe extern "system" fn wake_proc(
    window: windows::Win32::Foundation::HWND,
    message: u32,
    wparam: windows::Win32::Foundation::WPARAM,
    lparam: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::LRESULT {
    if message == WM_END_XAML_DRAWS {
        end_xaml_draws();
        return windows::Win32::Foundation::LRESULT(0);
    }
    unsafe { windows::Win32::UI::WindowsAndMessaging::DefWindowProcW(window, message, wparam, lparam) }
}

/// This thread's message-only window, which ends the draws other threads hand it.
fn wake_window() -> Result<isize> {
    use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, RegisterClassW, HWND_MESSAGE, WINDOW_EX_STYLE, WINDOW_STYLE, WNDCLASSW};
    let existing = WAKE_WINDOW.get();
    if existing != 0 {
        return Ok(existing);
    }
    let instance: windows::Win32::Foundation::HINSTANCE =
        unsafe { windows::Win32::System::LibraryLoader::GetModuleHandleW(None) }?.into();
    let class = windows::core::w!("NSCanvasXamlHandoff");
    let description = WNDCLASSW {
        lpfnWndProc: Some(wake_proc),
        hInstance: instance,
        lpszClassName: class,
        ..Default::default()
    };
    // 0 once another UI thread registered it.
    unsafe { RegisterClassW(&description) };
    let window = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class,
            windows::core::PCWSTR::null(),
            WINDOW_STYLE(0),
            0,
            0,
            0,
            0,
            Some(HWND_MESSAGE),
            None,
            Some(instance),
            None,
        )
    }?;
    WAKE_WINDOW.set(window.0 as isize);
    Ok(window.0 as isize)
}

/// Ends the draws handed to this (UI) thread, which shows their frames.
pub fn end_xaml_draws() {
    let handoffs: Vec<Arc<XamlHandoff>> = HANDOFFS.with(|handoffs| {
        let mut handoffs = handoffs.borrow_mut();
        handoffs.retain(|handoff| handoff.strong_count() > 0);
        handoffs.iter().filter_map(Weak::upgrade).collect()
    });
    for handoff in handoffs {
        handoff.end_draw();
    }
}

impl XamlHandoff {
    /// `source`: any COM pointer of the `SurfaceImageSource` (made `width` x `height`, not
    /// opaque). `device`: the D3D11 device the other thread copies frames with. UI thread.
    pub unsafe fn new(
        source: *mut c_void,
        device: &windows::Win32::Graphics::Direct3D11::ID3D11Device,
        width: u32,
        height: u32,
    ) -> Result<Arc<Self>> {
        use windows::Win32::Graphics::Direct3D11::ID3D11Multithread;
        let unknown = unsafe { windows::core::IUnknown::from_raw_borrowed(&source) }
            .ok_or_else(windows::core::Error::empty)?;
        let native: ISurfaceImageSourceNativeWithD2D = unknown.cast()?;
        // XAML uses the device on the UI thread while the render thread copies with it.
        let context = unsafe { device.GetImmediateContext() }?;
        let _ = unsafe { context.cast::<ID3D11Multithread>()?.SetMultithreadProtected(true) };
        let dxgi: windows::Win32::Graphics::Dxgi::IDXGIDevice = device.cast()?;
        unsafe { native.SetDevice(dxgi.as_raw()) }.ok()?;
        let window = wake_window()?;
        let handoff = Arc::new(Self {
            native,
            width,
            height,
            draw: parking_lot::Mutex::new(HandoffDraw::default()),
            wake_queued: AtomicBool::new(false),
            window,
        });
        HANDOFFS.with(|handoffs| {
            let mut handoffs = handoffs.borrow_mut();
            handoffs.retain(|handoff| handoff.strong_count() > 0);
            handoffs.push(Arc::downgrade(&handoff));
        });
        Ok(handoff)
    }

    pub fn source(&self) -> windows::core::IUnknown {
        self.native.clone().into()
    }

    fn present(
        &self,
        context: &windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext,
        texture: &windows::Win32::Graphics::Direct3D11::ID3D11Resource,
    ) -> Result<()> {
        let presented = {
            let mut draw = self.draw.lock();
            if draw.released {
                return Ok(());
            }
            if draw.begun {
                unsafe { self.native.ResumeDraw() }.ok()?;
            } else {
                let rect = windows::Win32::Foundation::RECT {
                    left: 0,
                    top: 0,
                    right: self.width as i32,
                    bottom: self.height as i32,
                };
                let mut surface = std::ptr::null_mut();
                let mut offset = windows::Win32::Foundation::POINT::default();
                let iid = windows::Win32::Graphics::Dxgi::IDXGISurface::IID;
                unsafe { self.native.BeginDraw(&rect, &iid, &mut surface, &mut offset) }.ok()?;
                draw.begun = true;
                draw.target = unsafe { windows::Win32::Graphics::Dxgi::IDXGISurface::from_raw(surface) }
                    .cast::<windows::Win32::Graphics::Direct3D11::ID3D11Resource>()
                    .ok()
                    .map(|target| (target, offset));
            }
            let copied = match draw.target.as_ref() {
                Some((target, offset)) => {
                    unsafe { copy_into(context, target, *offset, texture) };
                    Ok(())
                }
                None => Err(windows::core::Error::empty()),
            };
            let suspended = unsafe { self.native.SuspendDraw() }.ok();
            copied.and(suspended)
        };
        // After unlocking: a UI thread that found the draw busy is woken again.
        if !self.wake_queued.swap(true, Ordering::AcqRel) {
            let window = windows::Win32::Foundation::HWND(self.window as *mut c_void);
            let posted = unsafe {
                windows::Win32::UI::WindowsAndMessaging::PostMessageW(
                    Some(window),
                    WM_END_XAML_DRAWS,
                    windows::Win32::Foundation::WPARAM(0),
                    windows::Win32::Foundation::LPARAM(0),
                )
            };
            if posted.is_err() {
                self.wake_queued.store(false, Ordering::Release);
            }
        }
        presented
    }

    /// UI thread.
    pub fn end_draw(&self) {
        self.wake_queued.store(false, Ordering::Release);
        // Busy: the render thread is drawing and wakes this thread again when done.
        let Some(mut draw) = self.draw.try_lock() else { return };
        if std::mem::take(&mut draw.begun) {
            draw.target = None;
            if let Err(error) = unsafe { self.native.EndDraw() }.ok() {
                log::warn!("canvas: presenting into the XAML surface failed: {error}");
            }
        }
    }

    /// Like [`XamlSurface::release_device`], for a lost canvas: nothing is drawn into the image
    /// until it is attached again. UI thread.
    pub fn release_device(&self) {
        let mut draw = self.draw.lock();
        draw.released = true;
        if std::mem::take(&mut draw.begun) {
            draw.target = None;
            let _ = unsafe { self.native.EndDraw() };
        }
        let native = &self.native;
        blank_with_stand_in(
            |device| unsafe { native.SetDevice(device) },
            |rect, surface, offset| unsafe {
                native.BeginDraw(rect, &windows::Win32::Graphics::Dxgi::IDXGISurface::IID, surface, offset)
            },
            || unsafe { native.EndDraw() },
            self.width,
            self.height,
        );
    }
}

thread_local! {
    static STAND_IN_DEVICE: std::cell::OnceCell<
        Option<(windows::Win32::Graphics::Direct3D11::ID3D11Device, windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext1)>,
    > = const { std::cell::OnceCell::new() };
}

/// A D3D11 WARP device (made once per thread) that lost XAML surfaces hold instead of a removed
/// device's. D3D11 devices, unlike D3D12 ones, are not per-adapter singletons: it blocks nothing.
fn stand_in_device() -> Option<(
    windows::Win32::Graphics::Direct3D11::ID3D11Device,
    windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext1,
)> {
    use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_WARP;
    use windows::Win32::Graphics::Direct3D11::{D3D11CreateDevice, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION};
    STAND_IN_DEVICE.with(|cell| {
        cell.get_or_init(|| {
            let (mut device, mut context) = (None, None);
            unsafe {
                D3D11CreateDevice(
                    None,
                    D3D_DRIVER_TYPE_WARP,
                    windows::Win32::Foundation::HMODULE::default(),
                    D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                    None,
                    D3D11_SDK_VERSION,
                    Some(&mut device),
                    None,
                    Some(&mut context),
                )
            }
            .ok()?;
            Some((device?, context?.cast().ok()?))
        })
        .clone()
    })
}

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

/// A swapchain made off the UI thread, to show there with `bind_swap_chain`.
pub type SwapChainRef = windows::core::IUnknown;

/// Shows `swap_chain` (any COM pointer to it) in the panel (any COM pointer to it). UI thread.
pub unsafe fn bind_swap_chain(panel: *mut c_void, swap_chain: *mut c_void) -> Result<()> {
    let unknown = unsafe { windows::core::IUnknown::from_raw_borrowed(&panel) }.ok_or_else(windows::core::Error::empty)?;
    let native: ISwapChainPanelNative = unknown.cast()?;
    unsafe { native.SetSwapChain(swap_chain) }.ok()
}

pub struct CompositionSwapChain {
    swap_chain: IDXGISwapChain3,
    waitable: HANDLE,
    width: u32,
    height: u32,
    flags: DXGI_SWAP_CHAIN_FLAG,
    deferred_since: parking_lot::Mutex<Option<Instant>>,
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
            deferred_since: parking_lot::Mutex::new(None),
        })
    }

    /// Shows this swapchain in the panel. `panel` is any COM pointer of the `SwapChainPanel` (the
    /// runtime's `NSWinRT.interop.pointerKey(panel.handle)`). Must run on the UI thread.
    pub unsafe fn bind_panel(&self, panel: *mut c_void) -> Result<()> {
        bind_swap_chain(panel, self.swap_chain.as_raw())
    }

    /// For binding on the UI thread (`bind_swap_chain`) while another thread presents.
    pub fn as_unknown(&self) -> windows::core::IUnknown {
        self.swap_chain.clone().into()
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

    /// `false`: the display hasn't taken the queued frames yet; skip this present.
    pub fn acquire_frame(&self) -> bool {
        let mut deferred_since = self.deferred_since.lock();
        if self.frame_ready() {
            *deferred_since = None;
            return true;
        }
        let now = Instant::now();
        if now.duration_since(*deferred_since.get_or_insert(now)) >= MAX_FRAME_DEFERRAL {
            *deferred_since = None;
            return true;
        }
        PRESENT_DEFERRED.with(|deferred| deferred.set(true));
        false
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
