//! Skia on Direct3D 12 (Windows).
//!
//! A canvas draws into its own persistent render target (canvas contents survive between frames,
//! which flip-model swapchain buffers do not). Presenting copies that target into the current
//! back buffer of a composition swapchain shown by a WinUI `SwapChainPanel`. All canvases on a
//! thread share one D3D12 device and one Skia `DirectContext`, so drawing one canvas into another
//! stays on the GPU.

use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

use canvas_core::context_attributes::ColorSpace;
use canvas_core::gpu::d3d::{D3D12Context, PowerPreference};
use canvas_core::gpu::dxgi::{CompositionSwapChain, BUFFER_COUNT};
use skia_safe::gpu::d3d::TextureResourceInfo;
use skia_safe::gpu::{self, Budgeted, DirectContext, FlushInfo, Protected, SurfaceOrigin};
use skia_safe::surfaces::BackendSurfaceAccess;
use skia_safe::{AlphaType, BlendMode, Color, ColorType, ISize, ImageInfo, Paint, Surface};
use windows::Win32::Graphics::Direct3D12::D3D12_RESOURCE_STATE_PRESENT;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN};

use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{Context, State, SurfaceData, SurfaceEngine, SurfaceState};

thread_local! {
    /// The thread's Skia context and the device it was made on.
    static SKIA_D3D: RefCell<Option<(Rc<D3D12Context>, DirectContext)>> = const { RefCell::new(None) };
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
        let context = device.make_direct_context()?;
        *shared = Some((device.clone(), context.clone()));
        Some(context)
    })
}

/// The D3D12 side of a canvas: the device, and when the canvas is on screen, its swapchain.
pub struct D3DTarget {
    /// Skia surfaces over the swapchain buffers; dropped before the swapchain.
    back_buffers: Vec<Surface>,
    swap_chain: Option<CompositionSwapChain>,
    device: Rc<D3D12Context>,
    alpha: bool,
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
        let device = D3D12Context::shared(PowerPreference::Default)?;
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
                back_buffers: Vec::new(),
                swap_chain: None,
                device,
                alpha,
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
        })
    }

    /// Shows this canvas in a `SwapChainPanel` (any COM pointer to it). Must run on the UI thread.
    pub unsafe fn attach_swap_chain_panel(&mut self, panel: *mut c_void) -> bool {
        let (width, height) = (self.surface.width() as u32, self.surface.height() as u32);
        let Some(direct_context) = self.direct_context.as_mut() else { return false };
        let Some(target) = self.d3d.as_mut() else { return false };
        target.back_buffers.clear();
        target.swap_chain = None;
        let Ok(swap_chain) = CompositionSwapChain::new(&target.device, width, height, target.alpha) else {
            return false;
        };
        if swap_chain.bind_panel(panel).is_err() {
            return false;
        }
        let Some(back_buffers) = wrap_back_buffers(direct_context, &swap_chain) else { return false };
        target.back_buffers = back_buffers;
        target.swap_chain = Some(swap_chain);
        true
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
        self.d3d.as_ref().is_some_and(|target| target.device.is_removed())
    }

    /// After a device loss: moves the canvas to a new device (shared again by the thread's
    /// canvases) with a cleared drawing buffer and default state, as the web restores a lost 2D
    /// context. `panel`: the SwapChainPanel it is shown in, or null offscreen.
    pub unsafe fn restore_d3d(&mut self, panel: *mut c_void) -> bool {
        let Some(target) = self.d3d.as_mut() else { return false };
        target.back_buffers.clear();
        target.swap_chain = None;
        // Everything on the old device is gone; abandoning frees Skia's side without calling it.
        if let Some(context) = self.direct_context.as_mut() {
            context.abandon();
        }
        let Some(device) = D3D12Context::shared(PowerPreference::Default) else {
            log::error!("canvas: restoring a lost 2D context: no Direct3D 12 device");
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
        target.device = device;
        self.surface = surface;
        self.direct_context = Some(direct_context);
        self.surface_data.state = Default::default();
        self.surface_state = SurfaceState::None;
        self.path = Path::default();
        self.reset_state();
        if !panel.is_null() && !self.attach_swap_chain_panel(panel) {
            log::error!("canvas: restoring a lost 2D context: could not show it in its panel again");
            return false;
        }
        true
    }

    /// Flushes pending drawing and, when on screen, presents it.
    pub fn present_d3d(&mut self) {
        if self.d3d_lost() {
            return;
        }
        self.flush_surface();
        let Some(target) = self.d3d.as_mut() else { return };
        let Some(swap_chain) = target.swap_chain.as_ref() else { return };
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

    /// Resizing clears the canvas (as on the web) and resizes the swapchain with it.
    pub fn resize_d3d(context: &mut Context, width: f32, height: f32) {
        let alpha = !context.surface_data.is_opaque;
        let color_space = context.surface_data.color_space;
        let Some(direct_context) = context.direct_context.as_mut() else { return };
        let Some(surface) = render_target(direct_context, &target_info(width, height, alpha, color_space)) else {
            return;
        };
        if let Some(target) = context.d3d.as_mut() {
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
