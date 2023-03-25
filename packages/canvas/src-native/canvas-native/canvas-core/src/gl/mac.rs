use std::cell::{Ref, RefCell, RefMut};
use std::ffi::CString;
use std::num::NonZeroU32;
use std::rc::Rc;

#[cfg(target_os = "macos")]
use glutin::api::cgl::{context::PossiblyCurrentContext, display::Display, surface::Surface};

use glutin::config::{
    Api, AsRawConfig, ConfigSurfaceTypes, ConfigTemplate, ConfigTemplateBuilder, GetGlConfig,
};
use glutin::context::{AsRawContext, ContextApi, GlContext, RawContext, Version};
use glutin::display::{AsRawDisplay, DisplayApiPreference};
use glutin::display::{GetGlDisplay, RawDisplay};
use glutin::prelude::GlSurface;
use glutin::prelude::*;
use glutin::surface::{PbufferSurface, PixmapSurface, SwapInterval, WindowSurface};
use once_cell::sync::Lazy;
use raw_window_handle::{AppKitDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};
use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::context_attributes::ContextAttributes;

#[derive(Debug)]
pub(crate) enum SurfaceHelper {
    Window(Surface<WindowSurface>),
    Pbuffer(Surface<PbufferSurface>),
    Pixmap(Surface<PixmapSurface>),
}

#[derive(Debug, Default)]
pub struct GLContextInner {
    surface: Option<SurfaceHelper>,
    context: Option<PossiblyCurrentContext>,
    display: Option<Display>,
    window: Option<Window>,
    event: Option<EventLoop<()>>
}

unsafe impl Sync for GLContextInner {}

unsafe impl Send for GLContextInner {}

#[derive(Debug, Default)]
pub struct GLContext {
    inner: Rc<RefCell<GLContextInner>>,
}

impl GLContext {
    // pointer has to
    pub fn as_raw_inner(&self) -> *const RefCell<GLContextInner> {
        Rc::into_raw(Rc::clone(&self.inner))
    }

    pub fn from_raw_inner(raw: *const RefCell<GLContextInner>) -> Self {
        Self {
            inner: unsafe { Rc::from_raw(raw) },
        }
    }
}

impl Clone for GLContext {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl Into<ConfigTemplate> for ContextAttributes {
    fn into(self) -> ConfigTemplate {
        ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_alpha_size(if self.get_alpha() { 8 } else { 0 })
            .with_depth_size(if self.get_depth() { 16 } else { 0 })
            .with_stencil_size(if self.get_stencil() { 8 } else { 0 })
            .with_transparency(self.get_alpha())
            .build()
    }
}

impl From<&mut ContextAttributes> for ConfigTemplate {
    fn from(value: &mut ContextAttributes) -> Self {
        ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_alpha_size(if value.get_alpha() { 8 } else { 0 })
            .with_depth_size(if value.get_depth() { 16 } else { 0 })
            .with_stencil_size(if value.get_stencil() { 8 } else { 0 })
            .with_transparency(value.get_alpha())
            .build()
    }
}

impl Into<ConfigTemplateBuilder> for ContextAttributes {
    fn into(self) -> ConfigTemplateBuilder {
        ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_alpha_size(if self.get_alpha() { 8 } else { 0 })
            .with_depth_size(if self.get_depth() { 16 } else { 0 })
            .with_stencil_size(if self.get_stencil() { 8 } else { 0 })
            .with_transparency(self.get_alpha())
    }
}

impl From<&mut ContextAttributes> for ConfigTemplateBuilder {
    fn from(value: &mut ContextAttributes) -> Self {
        ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_alpha_size(if value.get_alpha() { 8 } else { 0 })
            .with_depth_size(if value.get_depth() { 16 } else { 0 })
            .with_stencil_size(if value.get_stencil() { 8 } else { 0 })
            .with_transparency(value.get_alpha())
    }
}

// impl Into<glutin::config::ConfigTemplate> for ContextAttributes {
//     fn into(self) -> ConfigTemplate {
//         ConfigTemplateBuilder::new()
//             .with_alpha_size(
//                 if self.get_alpha() { 8 } else { 0 }
//             )
//             .with_depth_size(if self.get_depth() { 16 } else { 0 })
//             .with_stencil_size(if self.get_stencil() { 8 } else { 0 })
//             .with_transparency(self.get_alpha())
//             .build()
//     }
// }

#[cfg(target_os = "macos")]
impl GLContext {
    pub fn set_surface(
        &mut self,
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        window: RawWindowHandle,
    ) -> bool {
        let is_2d = context_attrs.get_is_canvas();
        let cfg = context_attrs.into();
        unsafe {
            if let Some(display) = self.display() {
                let config = display
                    .find_configs(cfg)
                    .map(|c| {
                        c.reduce(|accum, cconfig| {
                            if is_2d {
                                let transparency_check =
                                    cconfig.supports_transparency().unwrap_or(false)
                                        & !accum.supports_transparency().unwrap_or(false);

                                return if transparency_check
                                    || cconfig.num_samples() < accum.num_samples()
                                {
                                    cconfig
                                } else {
                                    accum
                                };
                            }

                            let supports_transparency =
                                cconfig.supports_transparency().unwrap_or(false);

                            let num_samples = cconfig.num_samples();

                            let alpha_requested = context_attrs.get_alpha();

                            let mut alpha_size = if alpha_requested { 8u8 } else { 0u8 };
                            let mut stencil_size = if context_attrs.get_stencil() {
                                8u8
                            } else {
                                0u8
                            };
                            let mut depth_size = if context_attrs.get_depth() { 16u8 } else { 0u8 };

                            if supports_transparency == alpha_requested
                                && cconfig.alpha_size() == alpha_size
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == stencil_size
                                && context_attrs.get_depth()
                                && cconfig.depth_size() == depth_size
                            {
                                if accum.supports_transparency().unwrap_or(false) == alpha_requested
                                    && accum.alpha_size() == alpha_size
                                    && context_attrs.get_stencil()
                                    && accum.stencil_size() == stencil_size
                                    && context_attrs.get_depth()
                                    && accum.depth_size() == depth_size
                                    && accum.num_samples() > num_samples
                                {
                                    return accum;
                                }

                                return cconfig;
                            }

                            accum
                        })
                    })
                    .ok()
                    .flatten();

                return match config {
                    Some(config) => {
                        if context_attrs.get_antialias() && config.num_samples() == 0 {
                            context_attrs.set_antialias(false);
                        }

                        context_attrs.set_samples(config.num_samples());

                        let surface_attr =
                            glutin::surface::SurfaceAttributesBuilder::<WindowSurface>::new()
                                .build(
                                    window,
                                    NonZeroU32::try_from(width as u32).unwrap(),
                                    NonZeroU32::try_from(height as u32).unwrap(),
                                );

                        let surface = display
                            .create_window_surface(&config, &surface_attr)
                            .map(SurfaceHelper::Window)
                            .ok();

                        let ret = surface.is_some();

                        self.inner.borrow_mut().surface = surface;

                        ret
                    }
                    None => false,
                };
            }
        }
        false
    }

    pub fn create_window_context(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        window: RawWindowHandle,
    ) -> Option<GLContext> {
        match unsafe { Display::new(RawDisplayHandle::AppKit(AppKitDisplayHandle::empty())) } {
            Ok(display) => unsafe {
                gl_bindings::load_with(|symbol| {
                    let symbol = CString::new(symbol).unwrap();
                    display.get_proc_address(symbol.as_c_str()).cast()
                });

                let is_2d = context_attrs.get_is_canvas();
                let cfg = context_attrs.into();
                let config = display
                    .find_configs(cfg)
                    .map(|c| {
                        c.reduce(|accum, cconfig| {
                            if is_2d {
                                let transparency_check =
                                    cconfig.supports_transparency().unwrap_or(false)
                                        & !accum.supports_transparency().unwrap_or(false);

                                return if transparency_check
                                    || cconfig.num_samples() < accum.num_samples()
                                {
                                    cconfig
                                } else {
                                    accum
                                };
                            }

                            let supports_transparency =
                                cconfig.supports_transparency().unwrap_or(false);

                            let num_samples = cconfig.num_samples();

                            let alpha_requested = context_attrs.get_alpha();

                            let mut alpha_size = if alpha_requested { 8u8 } else { 0u8 };
                            let mut stencil_size = if context_attrs.get_stencil() {
                                8u8
                            } else {
                                0u8
                            };
                            let mut depth_size = if context_attrs.get_depth() { 16u8 } else { 0u8 };

                            if supports_transparency == alpha_requested
                                && cconfig.alpha_size() == alpha_size
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == stencil_size
                                && context_attrs.get_depth()
                                && cconfig.depth_size() == depth_size
                            {
                                if accum.supports_transparency().unwrap_or(false) == alpha_requested
                                    && accum.alpha_size() == alpha_size
                                    && context_attrs.get_stencil()
                                    && accum.stencil_size() == stencil_size
                                    && context_attrs.get_depth()
                                    && accum.depth_size() == depth_size
                                    && accum.num_samples() > num_samples
                                {
                                    return accum;
                                }

                                return cconfig;
                            }

                            accum
                        })
                    })
                    .ok()
                    .flatten();

                match config {
                    Some(config) => {
                        if context_attrs.get_antialias() && config.num_samples() == 0 {
                            context_attrs.set_antialias(false);
                        }

                        context_attrs.set_samples(config.num_samples());

                        let surface_attr =
                            glutin::surface::SurfaceAttributesBuilder::<WindowSurface>::new()
                                .build(
                                    window,
                                    NonZeroU32::try_from(width as u32).unwrap(),
                                    NonZeroU32::try_from(height as u32).unwrap(),
                                );

                        let surface = display
                            .create_window_surface(&config, &surface_attr)
                            .map(SurfaceHelper::Window)
                            .ok();

                        let context_attrs = glutin::context::ContextAttributesBuilder::new()
                            .with_context_api(ContextApi::OpenGl(Some(
                                if context_attrs.get_is_canvas() {
                                    Version::new(2, 1)
                                } else {
                                    Version::new(3, 0)
                                },
                            )))
                            .build(Some(window));

                        let context = display
                            .create_context(&config, &context_attrs)
                            .map(|v| v.treat_as_possibly_current())
                            .ok();

                        Some(GLContext {
                            inner: Rc::new(RefCell::new(GLContextInner {
                                surface,
                                context,
                                display: Some(display),
                                window: None,
                                event: None,
                            })),
                        })
                    }
                    None => None,
                }
            },
            Err(_) => None,
        }
    }


    pub fn set_window_surface(
        &mut self,
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        window: RawWindowHandle,
    ) {
        unsafe {
            if let Some(display) = self.display() {
                gl_bindings::load_with(|symbol| {
                    let symbol = CString::new(symbol).unwrap();
                    display.get_proc_address(symbol.as_c_str()).cast()
                });

                let is_2d = context_attrs.get_is_canvas();
                let cfg = context_attrs.clone().into();
                let config = display
                    .find_configs(cfg)
                    .map(|c| {
                        c.reduce(|accum, cconfig| {
                            if is_2d {
                                let transparency_check =
                                    cconfig.supports_transparency().unwrap_or(false)
                                        & !accum.supports_transparency().unwrap_or(false);

                                return if transparency_check
                                    || cconfig.num_samples() < accum.num_samples()
                                {
                                    cconfig
                                } else {
                                    accum
                                };
                            }

                            let supports_transparency =
                                cconfig.supports_transparency().unwrap_or(false);

                            let num_samples = cconfig.num_samples();

                            let alpha_requested = context_attrs.get_alpha();

                            let mut alpha_size = if alpha_requested { 8u8 } else { 0u8 };
                            let mut stencil_size = if context_attrs.get_stencil() {
                                8u8
                            } else {
                                0u8
                            };
                            let mut depth_size = if context_attrs.get_depth() { 16u8 } else { 0u8 };

                            if supports_transparency == alpha_requested
                                && cconfig.alpha_size() == alpha_size
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == stencil_size
                                && context_attrs.get_depth()
                                && cconfig.depth_size() == depth_size
                            {
                                if accum.supports_transparency().unwrap_or(false) == alpha_requested
                                    && accum.alpha_size() == alpha_size
                                    && context_attrs.get_stencil()
                                    && accum.stencil_size() == stencil_size
                                    && context_attrs.get_depth()
                                    && accum.depth_size() == depth_size
                                    && accum.num_samples() > num_samples
                                {
                                    return accum;
                                }

                                return cconfig;
                            }

                            accum
                        })
                    })
                    .ok()
                    .flatten();

                if let Some(config) = config {
                    if context_attrs.get_antialias() && config.num_samples() == 0 {
                        context_attrs.set_antialias(false);
                    }

                    context_attrs.set_samples(config.num_samples());

                    let surface_attr =
                        glutin::surface::SurfaceAttributesBuilder::<WindowSurface>::new().build(
                            window,
                            NonZeroU32::try_from(width as u32).unwrap(),
                            NonZeroU32::try_from(height as u32).unwrap(),
                        );

                    let surface = display
                        .create_window_surface(&config, &surface_attr)
                        .map(SurfaceHelper::Window)
                        .ok();

                    self.inner.borrow_mut().surface = surface;
                }
            }
        }
    }

    pub fn create_offscreen_context(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) -> Option<GLContext> {

        use winit::event::{Event, WindowEvent};
        use winit::event_loop::EventLoop;
        use winit::window::WindowBuilder;


        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new();

        let window = window_builder
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let raw_window_handle = window.raw_window_handle();

        GLContext::create_window_context(context_attrs, width, height, raw_window_handle)
            .map(|ctx| {
                let context = ctx.inner.borrow_mut();
                context.window = Some(window);
                context.event = Some(event_loop);
                ctx
            })
    }


    fn has_extension(extensions: &str, name: &str) -> bool {
        !extensions.split(' ').into_iter().any(|s| s == name)
    }

    pub fn has_gl2support() -> bool {
        match unsafe { Display::new(RawDisplayHandle::AppKit(AppKitDisplayHandle::empty())) } {
            Ok(display) => unsafe {
                display
                    .find_configs(
                        ConfigTemplateBuilder::default()
                            .with_api(Api::OPENGL)
                            .build(),
                    )
                    .is_ok()
            },
            Err(_) => false,
        }
    }

    pub(crate) fn surface(&self) -> Option<&SurfaceHelper> {
        unsafe { (*self.inner.as_ptr()).surface.as_ref() }
    }

    pub(crate) fn context(&self) -> Option<&PossiblyCurrentContext> {
        unsafe { (*self.inner.as_ptr()).context.as_ref() }
    }

    pub(crate) fn display(&self) -> Option<&Display> {
        unsafe { (*self.inner.as_ptr()).display.as_ref() }
    }

    #[inline(always)]
    pub fn set_vsync(&self, sync: bool) -> bool {
        let inner = self.inner.borrow();
        let vsync = if sync {
            SwapInterval::Wait(NonZeroU32::new(1).unwrap())
        } else {
            SwapInterval::DontWait
        };
        match (inner.context.as_ref(), inner.surface.as_ref()) {
            (Some(context), Some(surface)) => match surface {
                SurfaceHelper::Window(window) => window.set_swap_interval(context, vsync).is_ok(),
                SurfaceHelper::Pbuffer(buffer) => buffer.set_swap_interval(context, vsync).is_ok(),
                SurfaceHelper::Pixmap(map) => map.set_swap_interval(context, vsync).is_ok(),
            },
            _ => false,
        }
    }

    #[inline(always)]
    pub fn make_current(&self) -> bool {
        let inner = self.inner.borrow();
        match (inner.context.as_ref(), inner.surface.as_ref()) {
            (Some(context), Some(surface)) => match surface {
                SurfaceHelper::Window(window) => context.make_current(window).is_ok(),
                SurfaceHelper::Pbuffer(buffer) => context.make_current(buffer).is_ok(),
                SurfaceHelper::Pixmap(map) => context.make_current(map).is_ok(),
            },
            _ => false,
        }
    }

    #[inline(always)]
    pub fn remove_if_current(&self) {
        let inner = self.inner.borrow();
        let is_current = match (inner.context.as_ref(), inner.surface.as_ref()) {
            (Some(context), Some(surface)) => match surface {
                SurfaceHelper::Window(window) => window.is_current(context),
                SurfaceHelper::Pbuffer(buffer) => buffer.is_current(context),
                SurfaceHelper::Pixmap(map) => map.is_current(context),
            },
            _ => false,
        };

        if !is_current {
            return;
        }

        #[cfg(any(target_os = "android", target_os = "ios"))]
        {
            let display = self
                .display()
                .map(|display| match display.raw_display() {
                    RawDisplay::Egl(display) => display,
                    _ => 0 as _,
                })
                .unwrap_or(egl::EGL_NO_DISPLAY as _);

            egl::make_current(
                display as _,
                egl::EGL_NO_SURFACE,
                egl::EGL_NO_SURFACE,
                egl::EGL_NO_CONTEXT,
            );
        }
    }

    #[inline(always)]
    pub fn swap_buffers(&self) -> bool {
        let inner = self.inner.borrow();
        match (inner.context.as_ref(), inner.surface.as_ref()) {
            (Some(context), Some(surface)) => match surface {
                SurfaceHelper::Window(window) => window.swap_buffers(context).is_ok(),
                SurfaceHelper::Pbuffer(buffer) => buffer.swap_buffers(context).is_ok(),
                SurfaceHelper::Pixmap(map) => map.swap_buffers(context).is_ok(),
            },
            _ => false,
        }
    }

    #[inline(always)]
    pub fn get_surface_width(&self) -> i32 {
        self.surface()
            .as_ref()
            .map(|v| match v {
                SurfaceHelper::Window(window) => window.width().unwrap_or_default() as i32,
                SurfaceHelper::Pbuffer(buffer) => buffer.width().unwrap_or_default() as i32,
                SurfaceHelper::Pixmap(pixmap) => pixmap.width().unwrap_or_default() as i32,
            })
            .unwrap_or_default()
    }

    #[inline(always)]
    pub fn get_surface_height(&self) -> i32 {
        self.surface()
            .as_ref()
            .map(|v| match v {
                SurfaceHelper::Window(window) => window.height().unwrap_or_default() as i32,
                SurfaceHelper::Pbuffer(buffer) => buffer.height().unwrap_or_default() as i32,
                SurfaceHelper::Pixmap(pixmap) => pixmap.height().unwrap_or_default() as i32,
            })
            .unwrap_or_default()
    }
}
