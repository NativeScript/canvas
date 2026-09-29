//! Skia on Direct3D 12 (Windows).
//!
//! A canvas draws into its own persistent render target (canvas contents survive between frames,
//! which flip-model swapchain buffers do not). Presenting copies that target into the current
//! back buffer of a composition swapchain shown by a WinUI `SwapChainPanel`. All canvases on a
//! thread share one D3D12 device and one Skia `DirectContext`, so drawing one canvas into another
//! stays on the GPU.

use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::rc::Rc;
use std::time::{Duration, Instant};

use canvas_core::context_attributes::ColorSpace;
use canvas_core::gpu::d3d::{D3D12Context, PowerPreference};
use canvas_core::gpu::dxgi::{CompositionSwapChain, XamlHandoff, XamlSurface, BUFFER_COUNT};
use skia_safe::gpu::d3d::TextureResourceInfo;
use skia_safe::gpu::{self, Budgeted, DirectContext, FlushInfo, Protected, SurfaceOrigin};
use skia_safe::surfaces::BackendSurfaceAccess;
use skia_safe::{AlphaType, BlendMode, Color, ColorType, ISize, ImageInfo, Paint, Surface};
use windows::core::{IUnknown, Interface};
use windows::Win32::Graphics::Direct3D12::D3D12_RESOURCE_STATE_PRESENT;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN};

use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{Context, State, SurfaceData, SurfaceEngine, SurfaceState};

thread_local! {
    /// The thread's Skia context and the device it was made on.
    static SKIA_D3D: RefCell<Option<(Rc<D3D12Context>, DirectContext)>> = const { RefCell::new(None) };
    /// The thread's D3D canvases (`Context::register_d3d`), for `release_lost_canvases`.
    static CANVASES: RefCell<Vec<*mut Context>> = const { RefCell::new(Vec::new()) };
    static LAST_PURGE: Cell<Option<Instant>> = const { Cell::new(None) };
    /// What other devices share with the thread's (video frames), opened on it, by their
    /// producer's key. Dropped with a lost device, which it would otherwise keep alive.
    static SHARED: RefCell<std::collections::HashMap<u64, IUnknown>> = RefCell::new(Default::default());
}

const RESOURCE_CACHE_LIMIT: usize = 64 << 20;
const MAX_SHARED: usize = 16;

fn purge_idle_resources(context: &mut DirectContext) {
    let now = Instant::now();
    if LAST_PURGE.with(|last| last.get().is_some_and(|at| now - at < Duration::from_secs(1))) {
        return;
    }
    LAST_PURGE.with(|last| last.set(Some(now)));
    context.perform_deferred_cleanup(Duration::from_secs(5), None);
}

/// The thread's device, a new one after a device loss. The adapter makes no new device while the
/// removed one is held, so every canvas on it lets go of it first (`caller`, the canvas asking,
/// has done so itself).
fn thread_device(caller: *mut Context) -> Option<Rc<D3D12Context>> {
    if D3D12Context::is_shared_removed() {
        release_lost_canvases(caller);
    }
    D3D12Context::shared(PowerPreference::Default)
}

fn release_lost_canvases(caller: *mut Context) {
    let canvases = CANVASES.with(|canvases| canvases.borrow().clone());
    for canvas in canvases {
        if canvas != caller {
            // Registered canvases stay at their address until dropped, which unregisters them;
            // none is borrowed while another one's methods run (one thread, no reentrancy).
            unsafe { (*canvas).release_lost_device() };
        }
    }
    let stale = SKIA_D3D.with(|shared| {
        let mut shared = shared.borrow_mut();
        if shared.as_ref().is_some_and(|(device, _)| device.is_removed()) {
            shared.take()
        } else {
            None
        }
    });
    if let Some((_, mut context)) = stale {
        context.abandon();
        SHARED.with(|shared| shared.borrow_mut().clear());
    }
}

/// A lost canvas draws into this until it is restored: the drawing is discarded, readbacks keep
/// the canvas's size.
fn stand_in(info: &ImageInfo) -> Option<Surface> {
    skia_safe::surfaces::raster(info, None, None)
        .or_else(|| skia_safe::surfaces::raster(&info.with_dimensions((1, 1)), None, None))
}

/// The thread's Skia context on the shared D3D12 device (a new one once that device changes,
/// after a device loss).
fn shared_direct_context(device: &Rc<D3D12Context>) -> Option<DirectContext> {
    SKIA_D3D.with(|shared| {
        let mut shared = shared.borrow_mut();
        if let Some((made_on, context)) = shared.as_mut() {
            if Rc::ptr_eq(made_on, device) && !context.abandoned() {
                return Some(context.clone());
            }
        }
        if let Some((_, mut stale)) = shared.take() {
            stale.abandon();
        }
        let mut context = device.make_direct_context()?;
        context.set_resource_cache_limit(RESOURCE_CACHE_LIMIT);
        *shared = Some((device.clone(), context.clone()));
        Some(context)
    })
}

/// The D3D12 side of a canvas: the device, and when the canvas is on screen, its swapchain or
/// XAML surface.
pub struct D3DTarget {
    xaml: Option<XamlTarget>,
    /// Skia surfaces over the swapchain buffers; dropped before the swapchain.
    back_buffers: Vec<Surface>,
    swap_chain: Option<CompositionSwapChain>,
    /// The `SwapChainPanel` showing `swap_chain`, which it holds.
    panel: Option<IUnknown>,
    /// `None` once the canvas, lost, has let go of its removed device (`release_lost_device`).
    device: Option<Rc<D3D12Context>>,
    /// While lost: the XAML surface it presents into once restored.
    lost_xaml_source: Option<IUnknown>,
    alpha: bool,
    /// The canvas's registered address (`Context::register_d3d`), or null.
    registered: *mut Context,
}

impl Drop for D3DTarget {
    fn drop(&mut self) {
        if !self.registered.is_null() {
            let registered = self.registered;
            let _ = CANVASES.try_with(|canvases| canvases.borrow_mut().retain(|&canvas| canvas != registered));
        }
    }
}

/// Presenting into a XAML `SurfaceImageSource` (a canvas that blends with the page): Skia draws
/// the frame into `texture` as into a swapchain buffer, then D3D11On12 copies it into the image,
/// on the same queue.
struct XamlTarget {
    /// Skia's surface over `texture`; dropped first.
    back_buffer: Surface,
    /// `texture` for D3D11.
    wrapped: windows::Win32::Graphics::Direct3D11::ID3D11Resource,
    surface: XamlSurface,
    _texture: windows::Win32::Graphics::Direct3D12::ID3D12Resource,
}

impl XamlTarget {
    unsafe fn new(
        direct_context: &mut DirectContext,
        device: &D3D12Context,
        width: u32,
        height: u32,
        surface: impl FnOnce(&windows::Win32::Graphics::Direct3D11::ID3D11Device, u32, u32) -> windows::core::Result<XamlSurface>,
    ) -> Option<Self> {
        use windows::Win32::Graphics::Direct3D11::{ID3D11Resource, D3D11_BIND_RENDER_TARGET};
        use windows::Win32::Graphics::Direct3D11on12::D3D11_RESOURCE_FLAGS;
        use windows::Win32::Graphics::Direct3D12::*;
        use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;
        let (width, height) = (width.max(1), height.max(1));
        let on12 = device.d3d11_on_12()?;
        let desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Width: width as u64,
            Height: height,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Flags: D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET,
            ..Default::default()
        };
        let heap = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_DEFAULT,
            ..Default::default()
        };
        let mut texture: Option<ID3D12Resource> = None;
        unsafe {
            device.device().CreateCommittedResource(
                &heap,
                D3D12_HEAP_FLAG_NONE,
                &desc,
                D3D12_RESOURCE_STATE_COMMON,
                None,
                &mut texture,
            )
        }
        .ok()?;
        let texture = texture?;
        // Like a swapchain buffer, it rests in PRESENT (== COMMON) between frames.
        let target = gpu::backend_render_targets::make_d3d(
            (width as i32, height as i32),
            &TextureResourceInfo {
                resource: texture.clone(),
                alloc: None,
                resource_state: D3D12_RESOURCE_STATE_PRESENT,
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                sample_count: 1,
                level_count: 1,
                sample_quality_pattern: DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN,
                protected: Protected::No,
            },
        );
        let back_buffer = gpu::surfaces::wrap_backend_render_target(
            direct_context,
            &target,
            SurfaceOrigin::TopLeft,
            ColorType::BGRA8888,
            None,
            None,
        )?;
        let flags = D3D11_RESOURCE_FLAGS {
            BindFlags: D3D11_BIND_RENDER_TARGET.0 as u32,
            ..Default::default()
        };
        let mut wrapped: Option<ID3D11Resource> = None;
        unsafe {
            on12.on12.CreateWrappedResource(
                &texture,
                &flags,
                D3D12_RESOURCE_STATE_COMMON,
                D3D12_RESOURCE_STATE_COMMON,
                &mut wrapped,
            )
        }
        .ok()?;
        let wrapped = wrapped?;
        let surface = surface(&on12.device, width, height).ok()?;
        Some(Self {
            back_buffer,
            wrapped,
            surface,
            _texture: texture,
        })
    }
}

fn target_info(width: f32, height: f32, alpha: bool, color_space: ColorSpace) -> ImageInfo {
    ImageInfo::new(
        ISize::new((width as i32).max(1), (height as i32).max(1)),
        ColorType::RGBA8888,
        if alpha { AlphaType::Premul } else { AlphaType::Opaque },
        <ColorSpace as Into<Option<skia_safe::ColorSpace>>>::into(color_space),
    )
}

fn render_target(context: &mut DirectContext, info: &ImageInfo) -> Option<Surface> {
    gpu::surfaces::render_target(
        context,
        Budgeted::Yes,
        info,
        None,
        SurfaceOrigin::TopLeft,
        None,
        false,
        None,
    )
}

fn wrap_back_buffers(context: &mut DirectContext, swap_chain: &CompositionSwapChain) -> Option<Vec<Surface>> {
    (0..BUFFER_COUNT)
        .map(|index| {
            let resource = swap_chain.buffer(index).ok()?;
            let target = gpu::backend_render_targets::make_d3d(
                (swap_chain.width() as i32, swap_chain.height() as i32),
                &TextureResourceInfo {
                    resource,
                    alloc: None,
                    resource_state: D3D12_RESOURCE_STATE_PRESENT,
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    sample_count: 1,
                    level_count: 1,
                    sample_quality_pattern: DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN,
                    protected: Protected::No,
                },
            );
            gpu::surfaces::wrap_backend_render_target(
                context,
                &target,
                SurfaceOrigin::TopLeft,
                ColorType::BGRA8888,
                None,
                None,
            )
        })
        .collect()
}

impl Context {
    /// A canvas on the thread's D3D12 device. `None` when there is no usable D3D12 device.
    pub fn new_d3d(
        width: f32,
        height: f32,
        density: f32,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
        color_space: ColorSpace,
    ) -> Option<Self> {
        let device = thread_device(std::ptr::null_mut())?;
        let mut direct_context = shared_direct_context(&device)?;
        let surface = render_target(&mut direct_context, &target_info(width, height, alpha, color_space))?;

        let mut state = State::default();
        state.direction = direction;

        Some(Context {
            surface_data: SurfaceData {
                bounds: skia_safe::Rect::from_wh(width, height),
                scale: density,
                ppi,
                engine: SurfaceEngine::D3D,
                state: Default::default(),
                is_opaque: !alpha,
                color_space,
            },
            surface,
            surface_state: SurfaceState::None,
            direct_context: Some(direct_context),
            d3d: Some(D3DTarget {
                xaml: None,
                back_buffers: Vec::new(),
                swap_chain: None,
                panel: None,
                device: Some(device),
                lost_xaml_source: None,
                alpha,
                registered: std::ptr::null_mut(),
            }),
            #[cfg(feature = "vulkan")]
            vulkan_context: None,
            #[cfg(feature = "vulkan")]
            vulkan_texture: None,
            #[cfg(feature = "metal")]
            metal_context: None,
            #[cfg(feature = "metal")]
            metal_texture_info: None,
            #[cfg(feature = "gl")]
            gl_context: None,
            cpu_context: None,
            path: Path::default(),
            state,
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
            recording: None,
            #[cfg(feature = "gl")]
            window_surface: None,
        })
    }

    /// Tracks this canvas so that, after a device loss, it lets go of the removed device when
    /// another canvas is restored or created (see `thread_device`). Once the canvas is at the
    /// address it keeps until dropped (its owner's box).
    pub unsafe fn register_d3d(&mut self) {
        let canvas = self as *mut Context;
        let Some(target) = self.d3d.as_mut() else { return };
        if target.registered.is_null() {
            target.registered = canvas;
            CANVASES.with(|canvases| canvases.borrow_mut().push(canvas));
        }
    }

    /// Shows this canvas in a `SwapChainPanel` (any COM pointer to it). Must run on the UI thread.
    /// A lost canvas is shown in it by `restore_d3d`.
    pub unsafe fn attach_swap_chain_panel(&mut self, panel: *mut c_void) -> bool {
        let (width, height) = (self.surface.width() as u32, self.surface.height() as u32);
        let Some(target) = self.d3d.as_mut() else { return false };
        let Some(device) = target.device.as_ref() else { return true };
        let Some(direct_context) = self.direct_context.as_mut() else { return false };
        target.xaml = None;
        target.back_buffers.clear();
        target.swap_chain = None;
        target.panel = None;
        let Ok(swap_chain) = CompositionSwapChain::new(device, width, height, target.alpha) else {
            return false;
        };
        if swap_chain.bind_panel(panel).is_err() {
            return false;
        }
        let Some(back_buffers) = wrap_back_buffers(direct_context, &swap_chain) else { return false };
        target.back_buffers = back_buffers;
        target.swap_chain = Some(swap_chain);
        target.panel = unsafe { IUnknown::from_raw_borrowed(&panel) }.cloned();
        true
    }

    /// Presents into a XAML `SurfaceImageSource` (any COM pointer to it, made at the canvas's
    /// size) instead of a swapchain, so the canvas blends with what is behind it. UI thread.
    /// A lost canvas presents into it once restored.
    pub unsafe fn attach_xaml_surface(&mut self, source: *mut c_void) -> bool {
        let (width, height) = (self.surface.width() as u32, self.surface.height() as u32);
        let Some(target) = self.d3d.as_mut() else { return false };
        let Some(device) = target.device.as_ref() else {
            target.lost_xaml_source = unsafe { IUnknown::from_raw_borrowed(&source) }.cloned();
            return target.lost_xaml_source.is_some();
        };
        let Some(direct_context) = self.direct_context.as_mut() else { return false };
        target.back_buffers.clear();
        target.swap_chain = None;
        target.panel = None;
        target.xaml = unsafe {
            XamlTarget::new(direct_context, device, width, height, |on12, width, height| {
                XamlSurface::new(source, on12, width, height)
            })
        };
        target.xaml.is_some()
    }

    /// Render thread: a swapchain for the UI thread to show in its panel
    /// (`dxgi::bind_swap_chain`); this canvas presents into it. `None` when lost.
    pub fn create_panel_swap_chain(&mut self) -> Option<IUnknown> {
        let (width, height) = (self.surface.width() as u32, self.surface.height() as u32);
        let target = self.d3d.as_mut()?;
        let device = target.device.as_ref()?;
        let direct_context = self.direct_context.as_mut()?;
        target.xaml = None;
        target.back_buffers.clear();
        target.swap_chain = None;
        target.panel = None;
        let swap_chain = CompositionSwapChain::new(device, width, height, target.alpha).ok()?;
        target.back_buffers = wrap_back_buffers(direct_context, &swap_chain)?;
        let unknown = swap_chain.as_unknown();
        target.swap_chain = Some(swap_chain);
        Some(unknown)
    }

    /// Render thread: the D3D11 device a `XamlHandoff` for this canvas is made with (on the UI
    /// thread), and the size it presents at. `None` when lost.
    pub fn xaml_device(&self) -> Option<(windows::Win32::Graphics::Direct3D11::ID3D11Device, u32, u32)> {
        let device = self.d3d.as_ref()?.device.as_ref()?;
        let (width, height) = (self.surface.width().max(1) as u32, self.surface.height().max(1) as u32);
        Some((device.d3d11_on_12()?.device.clone(), width, height))
    }

    /// Render thread: presents through `handoff` (made at the canvas's size with `xaml_device`).
    pub fn attach_xaml_handoff(&mut self, handoff: std::sync::Arc<XamlHandoff>) -> bool {
        let (width, height) = (self.surface.width() as u32, self.surface.height() as u32);
        let Some(target) = self.d3d.as_mut() else { return false };
        let Some(device) = target.device.as_ref() else { return false };
        let Some(direct_context) = self.direct_context.as_mut() else { return false };
        target.back_buffers.clear();
        target.swap_chain = None;
        target.panel = None;
        target.xaml = unsafe {
            XamlTarget::new(direct_context, device, width, height, |on12, _, _| {
                XamlSurface::with_handoff(handoff, on12)
            })
        };
        target.xaml.is_some()
    }

    pub fn d3d_device(&self) -> Option<Rc<D3D12Context>> {
        self.d3d.as_ref()?.device.clone()
    }

    /// A resource or fence another device shares (`handle`, an NT handle), opened on this
    /// canvas's device; `key` identifies it for as long as its producer keeps it (handle values
    /// get reused).
    pub fn d3d_open_shared<T: Interface>(&self, key: u64, handle: windows::Win32::Foundation::HANDLE) -> Option<T> {
        let device = self.d3d_device().filter(|device| !device.is_removed())?;
        if let Some(opened) = SHARED.with(|shared| shared.borrow().get(&key).and_then(|opened| opened.cast::<T>().ok())) {
            return Some(opened);
        }
        let mut opened: Option<T> = None;
        unsafe { device.device().OpenSharedHandle(handle, &mut opened) }.ok()?;
        let opened = opened?;
        SHARED.with(|shared| {
            let mut shared = shared.borrow_mut();
            // The producer makes new ones when the video size changes.
            if shared.len() >= MAX_SHARED {
                shared.clear();
            }
            shared.insert(key, opened.cast().ok()?);
            Some(())
        });
        Some(opened)
    }

    /// An image over a texture on this canvas's device (in COMMON, read only), for this frame.
    /// Declared in a shader-read state, so Skia issues no barrier: it is promoted implicitly and
    /// decays back to COMMON after each submit, as a texture shared with another device must.
    pub fn d3d_borrow_texture(
        &mut self,
        resource: &windows::Win32::Graphics::Direct3D12::ID3D12Resource,
        width: i32,
        height: i32,
    ) -> Option<skia_safe::Image> {
        use windows::Win32::Graphics::Direct3D12::D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE;
        let direct_context = self.direct_context.as_mut()?;
        let texture = gpu::backend_textures::make_d3d(
            (width, height),
            &TextureResourceInfo {
                resource: resource.clone(),
                alloc: None,
                resource_state: D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE,
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                sample_count: 1,
                level_count: 1,
                sample_quality_pattern: DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN,
                protected: Protected::No,
            },
            "external",
        );
        gpu::images::borrow_texture_from(
            direct_context,
            &texture,
            SurfaceOrigin::TopLeft,
            ColorType::BGRA8888,
            AlphaType::Premul,
            None,
        )
    }

    /// Maps the swapchain into the panel: DIPs = pixels * scale + offset.
    pub fn set_swap_chain_transform(&self, scale_x: f32, scale_y: f32, offset_x: f32, offset_y: f32) -> bool {
        self.d3d
            .as_ref()
            .and_then(|t| t.swap_chain.as_ref())
            .is_some_and(|s| s.set_transform(scale_x, scale_y, offset_x, offset_y).is_ok())
    }

    /// The canvas's D3D12 device was removed: nothing draws until `restore_d3d`.
    pub fn d3d_lost(&self) -> bool {
        self.d3d
            .as_ref()
            .is_some_and(|target| target.device.as_ref().is_none_or(|device| device.is_removed()))
    }

    /// A lost canvas lets go of everything on its removed device: its Skia context and surfaces,
    /// its swapchain (unbound from the panel, which holds it) or XAML surface. Until restored it
    /// draws into a raster stand-in.
    fn release_lost_device(&mut self) {
        let Some(target) = self.d3d.as_mut() else { return };
        if !target.device.as_ref().is_some_and(|device| device.is_removed()) {
            return;
        }
        // Abandoning frees Skia's side without calling the removed device.
        if let Some(context) = self.direct_context.as_mut() {
            context.abandon();
        }
        // A handoff's image is released and attached again from the UI thread.
        target.lost_xaml_source = target.xaml.take().filter(|xaml| !xaml.surface.is_handoff()).map(|xaml| {
            xaml.surface.release_device();
            xaml.surface.source()
        });
        target.back_buffers.clear();
        if let Some(panel) = target.panel.take() {
            unsafe { CompositionSwapChain::unbind_panel(panel.as_raw()) };
        }
        target.swap_chain = None;
        let bounds = self.surface_data.bounds;
        let info = target_info(bounds.width(), bounds.height(), !self.surface_data.is_opaque, self.surface_data.color_space);
        if let Some(surface) = stand_in(&info) {
            self.surface = surface;
        }
        self.direct_context = None;
        // Styles can hold images on the old device.
        self.reset_state();
        if let Some(target) = self.d3d.as_mut() {
            target.device = None;
        }
    }

    /// After a device loss: moves the canvas to a new device (shared again by the thread's
    /// canvases) with a cleared drawing buffer and default state, as the web restores a lost 2D
    /// context. `panel`: the SwapChainPanel it is shown in, or null offscreen.
    pub unsafe fn restore_d3d(&mut self, panel: *mut c_void) -> bool {
        if self.d3d.is_none() {
            return false;
        }
        if !self.d3d_lost() {
            return true;
        }
        self.release_lost_device();
        let Some(device) = thread_device(self) else {
            log::warn!("canvas: restoring a lost 2D context: no Direct3D 12 device (yet)");
            return false;
        };
        let Some(mut direct_context) = shared_direct_context(&device) else {
            log::error!("canvas: restoring a lost 2D context: Skia could not use the new device");
            return false;
        };
        let bounds = self.surface_data.bounds;
        let info = target_info(bounds.width(), bounds.height(), !self.surface_data.is_opaque, self.surface_data.color_space);
        let Some(surface) = render_target(&mut direct_context, &info) else {
            log::error!("canvas: restoring a lost 2D context: no {}x{} render target", bounds.width(), bounds.height());
            return false;
        };
        let Some(target) = self.d3d.as_mut() else { return false };
        let xaml_source = target.lost_xaml_source.take();
        target.device = Some(device);
        self.surface = surface;
        self.direct_context = Some(direct_context);
        self.surface_data.state = Default::default();
        self.surface_state = SurfaceState::None;
        self.path = Path::default();
        self.reset_state();
        let shown = match xaml_source {
            Some(source) => self.attach_xaml_surface(source.as_raw()),
            None => panel.is_null() || self.attach_swap_chain_panel(panel),
        };
        if !shown {
            log::error!("canvas: restoring a lost 2D context: could not show it again");
        }
        shown
    }

    /// Flushes pending drawing and, when on screen, presents it.
    pub fn present_d3d(&mut self) {
        if self.d3d_lost() {
            return;
        }
        self.flush_surface();
        if let Some(direct_context) = self.direct_context.as_mut() {
            purge_idle_resources(direct_context);
        }
        let Some(target) = self.d3d.as_mut() else { return };
        if let Some(xaml) = target.xaml.as_mut() {
            let Some(direct_context) = self.direct_context.as_mut() else { return };
            let snapshot = self.surface.image_snapshot();
            let mut paint = Paint::default();
            paint.set_blend_mode(BlendMode::Src);
            xaml.back_buffer.canvas().draw_image(&snapshot, (0, 0), Some(&paint));
            drop(snapshot);
            direct_context.flush_surface_with_access(&mut xaml.back_buffer, BackendSurfaceAccess::Present, &FlushInfo::default());
            direct_context.submit(None);
            if let Some(on12) = target.device.as_ref().and_then(|device| device.d3d11_on_12()) {
                let resources = [Some(xaml.wrapped.clone())];
                unsafe { on12.on12.AcquireWrappedResources(&resources) };
                if let Err(error) = xaml.surface.present(&xaml.wrapped) {
                    log::warn!("canvas: presenting into the XAML surface failed: {error}");
                }
                unsafe {
                    on12.on12.ReleaseWrappedResources(&resources);
                    on12.context.Flush();
                }
            }
            return;
        }
        let Some(swap_chain) = target.swap_chain.as_ref() else { return };
        if !swap_chain.acquire_frame() {
            return;
        }
        let Some(direct_context) = self.direct_context.as_mut() else { return };
        let index = swap_chain.current_index() as usize;
        let Some(back_buffer) = target.back_buffers.get_mut(index) else { return };

        let snapshot = self.surface.image_snapshot();
        let mut paint = Paint::default();
        paint.set_blend_mode(BlendMode::Src);
        back_buffer.canvas().draw_image(&snapshot, (0, 0), Some(&paint));
        // Let the persistent surface keep drawing without a copy-on-write of this frame.
        drop(snapshot);

        direct_context.flush_surface_with_access(back_buffer, BackendSurfaceAccess::Present, &FlushInfo::default());
        direct_context.submit(None);
        let _ = swap_chain.present(true);
    }

    /// Resizing clears the canvas (as on the web) and resizes the swapchain with it. A lost canvas
    /// takes the size for its restore.
    pub fn resize_d3d(context: &mut Context, width: f32, height: f32) {
        let alpha = !context.surface_data.is_opaque;
        let color_space = context.surface_data.color_space;
        let info = target_info(width, height, alpha, color_space);
        context.release_lost_device();
        let Some(direct_context) = context.direct_context.as_mut() else {
            if let Some(surface) = stand_in(&info) {
                context.surface = surface;
            }
            context.surface_data.bounds = skia_safe::Rect::from_wh(width, height);
            if let Some(target) = context.d3d.as_mut() {
                target.lost_xaml_source = None;
            }
            return;
        };
        let Some(surface) = render_target(direct_context, &info) else {
            return;
        };
        if let Some(target) = context.d3d.as_mut() {
            // A SurfaceImageSource has a fixed size: the host attaches one of the new size.
            target.xaml = None;
            if let Some(swap_chain) = target.swap_chain.as_mut() {
                target.back_buffers.clear();
                // Skia may still reference the old buffers until its work is done.
                direct_context.flush_submit_and_sync_cpu();
                if swap_chain.resize(width as u32, height as u32).is_ok() {
                    target.back_buffers = wrap_back_buffers(direct_context, swap_chain).unwrap_or_default();
                }
            }
        }
        context.surface = surface;
        context.surface_data.bounds = skia_safe::Rect::from_wh(width, height);
        context.surface_data.state = Default::default();
        context.surface_state = SurfaceState::None;
        context.path = Path::default();
        context.reset_state();
    }
}
