use std::ffi::c_void;
use std::sync::{Arc, Weak};

use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::context::Context;
use canvas_core::context_attributes::ColorSpace;
use canvas_core::fit::{surface_transform, CanvasFit, SurfaceTransform};
use canvas_core::gpu::dxgi::{self, CompositionSwapChain, UiThread, XamlHandoff};
use windows::core::{IUnknown, Interface};

use super::{Binding, CanvasOffscreenEvent, CanvasOffscreenSurface, GlRef, Inner};
use crate::c2d::render_thread::TargetHandle;
use crate::webgpu::gpu_canvas_context::CanvasGPUCanvasContext;
use crate::CanvasRenderingContext2D;

/// Only called on the UI thread.
struct UiCom(IUnknown);

unsafe impl Send for UiCom {}

/// Handed from the render thread, used by one thread at a time.
struct Sendable<T>(T);

unsafe impl<T> Send for Sendable<T> {}

pub(super) struct View {
    pub(super) ui: UiThread,
    panel: UiCom,
    xaml: Option<(UiCom, (u32, u32))>,
    /// A 2D context's; a WebGL state keeps its own.
    handoff: Option<Arc<XamlHandoff>>,
    fit: CanvasFit,
    scale: (f32, f32),
    view_size: (f32, f32),
}

impl View {
    fn end_handoff(&mut self) {
        if let Some(handoff) = self.handoff.take() {
            // Shows its last frame, if the render thread drew one it has not ended.
            handoff.end_draw();
        }
    }
}

/// The context, held outside the surface's lock while the UI thread waits on its thread.
enum Target {
    TwoD(TargetHandle),
    WebGL(GlRef),
    WebGPU(Arc<CanvasGPUCanvasContext>),
}

fn target(inner: &Inner) -> Option<Target> {
    match &inner.binding {
        Binding::TwoD(handle) => Some(Target::TwoD(handle.clone())),
        Binding::WebGL(state) => {
            crate::canvas_native_webgl_state_reference(state.0);
            Some(Target::WebGL(GlRef(state.0)))
        }
        Binding::WebGPU(context) => Some(Target::WebGPU(Arc::clone(context))),
        Binding::TwoDDirect | Binding::None => None,
    }
}

unsafe fn attach_panel(target: &Target, panel: *mut c_void) -> bool {
    match target {
        Target::TwoD(handle) => {
            let swap_chain = handle
                .sync(|real| real.get_context_mut().create_panel_swap_chain().map(Sendable))
                .flatten();
            let Some(Sendable(swap_chain)) = swap_chain else { return false };
            match unsafe { dxgi::bind_swap_chain(panel, swap_chain.as_raw()) } {
                Ok(()) => true,
                Err(error) => {
                    log::error!("canvas: could not show the OffscreenCanvas in its panel: {error}");
                    false
                }
            }
        }
        Target::WebGL(state) => unsafe { crate::attach_swap_chain_panel_threaded(state.state(), panel) },
        // wgpu binds its own swapchain.
        Target::WebGPU(_) => true,
    }
}

unsafe fn attach_xaml(target: &Target, source: *mut c_void) -> Option<Option<Arc<XamlHandoff>>> {
    match target {
        Target::TwoD(handle) => {
            let device = handle.sync(|real| real.get_context().xaml_device().map(Sendable)).flatten();
            let Sendable((device, width, height)) = device?;
            let handoff = match unsafe { XamlHandoff::new(source, &device, width, height) } {
                Ok(handoff) => handoff,
                Err(error) => {
                    log::error!("canvas: the XAML surface cannot be drawn from the render thread: {error}");
                    return None;
                }
            };
            let shared = Arc::clone(&handoff);
            handle
                .sync(move |real| real.get_context_mut().attach_xaml_handoff(shared))
                .unwrap_or(false)
                .then_some(Some(handoff))
        }
        Target::WebGL(state) => {
            unsafe { crate::attach_xaml_surface_threaded(state.state(), source) }.then_some(None)
        }
        Target::WebGPU(_) => None,
    }
}

fn apply_transform(target: &Target, t: SurfaceTransform) {
    let SurfaceTransform { scale_x, scale_y, offset_x, offset_y } = t;
    match target {
        Target::TwoD(handle) => handle.post(move |real| {
            real.get_context().set_swap_chain_transform(scale_x, scale_y, offset_x, offset_y);
        }),
        Target::WebGL(state) => {
            crate::canvas_native_webgl_set_swap_chain_transform(state.0, scale_x, scale_y, offset_x, offset_y);
        }
        Target::WebGPU(context) => unsafe {
            crate::webgpu::gpu_canvas_context::canvas_native_webgpu_context_set_swap_chain_transform(
                Arc::as_ptr(context),
                scale_x,
                scale_y,
                offset_x,
                offset_y,
            );
        },
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn threaded_d3d_2d(
    width: f32,
    height: f32,
    density: f32,
    alpha: bool,
    font_color: i32,
    ppi: f32,
    direction: TextDirection,
    color_space: ColorSpace,
) -> Option<CanvasRenderingContext2D> {
    let context = CanvasRenderingContext2D::new_threaded_with(
        width, height, density, alpha, font_color, ppi, direction, color_space,
        move || {
            let context = Context::new_d3d(width, height, density, alpha, font_color, ppi, direction, color_space)?;
            crate::c2d::d3d::note_render_thread_device(&context);
            Some(CanvasRenderingContext2D::new_d3d(context, alpha))
        },
    )?;
    // The render thread may have failed to make it.
    context.render_target()?.sync(|_| ())?;
    Some(context)
}

impl CanvasOffscreenSurface {
    fn weak(&self) -> Weak<Self> {
        // Every surface lives in an `Arc` (`canvas_native_offscreen_surface_create*`).
        let this = unsafe {
            Arc::increment_strong_count(self);
            Arc::from_raw(self)
        };
        Arc::downgrade(&this)
    }

    /// Owner thread: the view hears it on the UI thread, once the context is shown there.
    pub(super) fn tell_windows_view(&self, event: CanvasOffscreenEvent, a: u32, b: u32) {
        let Some(ui) = self.inner().view.as_ref().map(|view| view.ui) else { return };
        let surface = self.weak();
        ui.run(move || {
            let Some(surface) = surface.upgrade() else { return };
            match event {
                CanvasOffscreenEvent::Engine => surface.show(b == 1),
                // The view attaches a SurfaceImageSource of the new size.
                CanvasOffscreenEvent::Resize => {
                    if let Some(view) = surface.inner().view.as_mut() {
                        if view.xaml.as_ref().is_some_and(|(_, size)| *size != (a, b)) {
                            view.xaml = None;
                            view.end_handoff();
                        }
                    }
                }
            }
            surface.place();
            surface.notify(event, a, b);
        });
    }

    /// UI thread. A transparent canvas waits for its view's SurfaceImageSource.
    fn show(&self, alpha: bool) {
        let (target, panel, xaml) = {
            let inner = self.inner();
            let Some(view) = inner.view.as_ref() else { return };
            (target(&inner), view.panel.0.clone(), view.xaml.as_ref().map(|(xaml, _)| xaml.0.clone()))
        };
        let Some(target) = target else { return };
        match xaml {
            Some(source) => {
                self.attach_xaml_to(&target, &source);
            }
            None if !alpha => {
                unsafe { attach_panel(&target, panel.as_raw()) };
            }
            None => {}
        }
    }

    fn attach_xaml_to(&self, target: &Target, source: &IUnknown) -> bool {
        let Some(handoff) = (unsafe { attach_xaml(target, source.as_raw()) }) else {
            return false;
        };
        let mut inner = self.inner();
        if let Some(view) = inner.view.as_mut() {
            view.end_handoff();
            view.handoff = handoff;
        } else if let Some(handoff) = handoff {
            handoff.end_draw();
        }
        true
    }

    fn transform(&self, view: &View) -> SurfaceTransform {
        let (width, height) = self.size();
        surface_transform(view.fit, (width as f32, height as f32), view.scale, view.view_size)
    }

    /// UI thread.
    fn place(&self) {
        let (target, transform) = {
            let inner = self.inner();
            let Some(view) = inner.view.as_ref() else { return };
            (target(&inner), self.transform(view))
        };
        if let Some(target) = target {
            apply_transform(&target, transform);
        }
    }

    /// UI thread.
    pub(super) fn detach_windows_view(&self) {
        let (target, view) = {
            let mut inner = self.inner();
            (target(&inner), inner.view.take())
        };
        let Some(mut view) = view else { return };
        unsafe { CompositionSwapChain::unbind_panel(view.panel.0.as_raw()) };
        view.end_handoff();
        match target {
            Some(Target::TwoD(handle)) => handle.post(|real| real.get_context_mut().detach_d3d_view()),
            Some(Target::WebGL(state)) => crate::detach_view_threaded(state.state()),
            Some(Target::WebGPU(_)) | None => {}
        }
    }

    pub(super) fn panel(&self) -> Option<(IUnknown, UiThread)> {
        let inner = self.inner();
        let view = inner.view.as_ref()?;
        Some((view.panel.0.clone(), view.ui))
    }
}

/// UI thread.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_create_windows(
    width: u32,
    height: u32,
    density: f32,
    ppi: f32,
    direction: u32,
    color_space: crate::CanvasColorSpace,
    panel: *mut c_void,
) -> *const CanvasOffscreenSurface {
    let Some(panel) = (unsafe { IUnknown::from_raw_borrowed(&panel) }).cloned() else {
        return std::ptr::null();
    };
    let ui = match UiThread::current() {
        Ok(ui) => ui,
        Err(error) => {
            log::error!("canvas: an OffscreenCanvas can't be shown from this thread: {error}");
            return std::ptr::null();
        }
    };
    let surface = CanvasOffscreenSurface::new(width.max(1), height.max(1), density, ppi, direction, color_space);
    surface.inner().view = Some(View {
        ui,
        panel: UiCom(panel),
        xaml: None,
        handoff: None,
        fit: CanvasFit::default(),
        scale: (density, density),
        view_size: (0., 0.),
    });
    Arc::into_raw(Arc::new(surface))
}

/// UI thread.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_set_layout(
    surface: *const CanvasOffscreenSurface,
    fit: i32,
    scale_x: f32,
    scale_y: f32,
    view_width: f32,
    view_height: f32,
) {
    let Some(surface) = (unsafe { surface.as_ref() }) else { return };
    {
        let mut inner = surface.inner();
        let Some(view) = inner.view.as_mut() else { return };
        view.fit = CanvasFit::from_i32(fit).unwrap_or(view.fit);
        view.scale = (scale_x, scale_y);
        view.view_size = (view_width, view_height);
    }
    surface.place();
}

/// UI thread. `false` for WebGPU: wgpu owns its swapchain.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_attach_xaml_surface(
    surface: *const CanvasOffscreenSurface,
    source: *mut c_void,
) -> bool {
    let (Some(surface), Some(source)) = (unsafe { surface.as_ref() }, unsafe { IUnknown::from_raw_borrowed(&source) }.cloned())
    else {
        return false;
    };
    let target = {
        let mut inner = surface.inner();
        let target = target(&inner);
        let Some(view) = inner.view.as_mut() else { return false };
        if matches!(target, Some(Target::WebGPU(_))) {
            return false;
        }
        view.xaml = Some((UiCom(source.clone()), surface.size()));
        target
    };
    // Without a context yet, it is shown in it once made.
    target.is_none_or(|target| surface.attach_xaml_to(&target, &source))
}

/// UI thread. `out`: `[scaleX, scaleY, offsetX, offsetY]`.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_get_transform(surface: *const CanvasOffscreenSurface, out: *mut f32) {
    let (Some(surface), false) = (unsafe { surface.as_ref() }, out.is_null()) else { return };
    let inner = surface.inner();
    let Some(view) = inner.view.as_ref() else { return };
    let t = surface.transform(view);
    unsafe { std::ptr::copy_nonoverlapping([t.scale_x, t.scale_y, t.offset_x, t.offset_y].as_ptr(), out, 4) };
}
