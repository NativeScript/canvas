use std::ffi::CString;
use std::num::NonZeroU32;
use std::sync::Arc;

#[cfg(target_os = "macos")]
use glutin::api::cgl::{context::PossiblyCurrentContext, display::Display, surface::Surface};
use glutin::config::{Api, ColorBufferType, ConfigTemplate, ConfigTemplateBuilder, GetGlConfig};
use glutin::context::{ContextApi, GlContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::prelude::GlSurface;
use glutin::surface::{PbufferSurface, PixmapSurface, SwapInterval, WindowSurface};
use once_cell::sync::OnceCell;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use raw_window_handle::{
    AppKitDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::context_attributes::ContextAttributes;

pub static IS_GL_SYMBOLS_LOADED: OnceCell<bool> = OnceCell::new();

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
    event: Option<EventLoop<()>>,
    transfer_surface_info: crate::gl::TransferSurface,
}

unsafe impl Sync for GLContextInner {}

unsafe impl Send for GLContextInner {}

#[derive(Debug, Default)]
pub struct GLContext {
    inner: Arc<RwLock<GLContextInner>>,
}

impl GLContext {
    // pointer has to
    pub fn as_raw_inner(&self) -> *const RwLock<GLContextInner> {
        Arc::into_raw(Arc::clone(&self.inner))
    }

    pub fn from_raw_inner(raw: *const RwLock<GLContextInner>) -> Self {
        Self {
            inner: unsafe { Arc::from_raw(raw) },
        }
    }
}

impl Clone for GLContext {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Into<ConfigTemplate> for ContextAttributes {
    fn into(self) -> ConfigTemplate {
        let mut builder = ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_alpha_size(if self.get_alpha() { 8 } else { 0 })
            .with_depth_size(if self.get_depth() { 16 } else { 0 })
            .with_stencil_size(if self.get_stencil() { 8 } else { 0 })
            .with_buffer_type(ColorBufferType::Rgb {
                r_size: 8,
                g_size: 8,
                b_size: 8,
            })
            .with_transparency(self.get_alpha());

        if !self.get_is_canvas() && self.get_antialias() {
            builder = builder.with_multisampling(4)
        }
        builder.build()
    }
}

impl From<&mut ContextAttributes> for ConfigTemplate {
    fn from(value: &mut ContextAttributes) -> Self {
        let mut builder = ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_alpha_size(if value.get_alpha() { 8 } else { 0 })
            .with_depth_size(if value.get_depth() { 16 } else { 0 })
            .with_stencil_size(if value.get_stencil() { 8 } else { 0 })
            .with_buffer_type(ColorBufferType::Rgb {
                r_size: 8,
                g_size: 8,
                b_size: 8,
            })
            .with_transparency(value.get_alpha());

        if !value.get_is_canvas() && value.get_antialias() {
            builder = builder.with_multisampling(4)
        }
        builder.build()
    }
}

impl Into<ConfigTemplateBuilder> for ContextAttributes {
    fn into(self) -> ConfigTemplateBuilder {
        let mut builder = ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_alpha_size(if self.get_alpha() { 8 } else { 0 })
            .with_depth_size(if self.get_depth() { 16 } else { 0 })
            .with_stencil_size(if self.get_stencil() { 8 } else { 0 })
            .with_buffer_type(ColorBufferType::Rgb {
                r_size: 8,
                g_size: 8,
                b_size: 8,
            })
            .with_transparency(self.get_alpha());

        if !self.get_is_canvas() && self.get_antialias() {
            builder = builder.with_multisampling(4)
        }
        builder
    }
}

impl From<&mut ContextAttributes> for ConfigTemplateBuilder {
    fn from(value: &mut ContextAttributes) -> Self {
        let mut builder = ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_multisampling(if value.get_antialias() { 4 } else { 0 })
            .with_alpha_size(if value.get_alpha() { 8 } else { 0 })
            .with_depth_size(if value.get_depth() { 16 } else { 0 })
            .with_stencil_size(if value.get_stencil() { 8 } else { 0 })
            .with_buffer_type(ColorBufferType::Rgb {
                r_size: 8,
                g_size: 8,
                b_size: 8,
            })
            .with_transparency(value.get_alpha());

        if !value.get_is_canvas() && value.get_antialias() {
            builder = builder.with_multisampling(4)
        }
        builder
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
//             .with_buffer_type(ColorBufferType::Rgb {
//                 r_size: 8,
//                 g_size: 8,
//                 b_size: 8,
//             })
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
        let multi_sample = context_attrs.get_antialias();
        let cfg = context_attrs.into();
        let mut inner = self.inner.write();
        unsafe {
            if let Some(display) = inner.display.as_ref() {
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

                            let alpha_requested = context_attrs.get_alpha();

                            let mut alpha_size = if alpha_requested { 8u8 } else { 0u8 };
                            let mut stencil_size = if context_attrs.get_stencil() {
                                8u8
                            } else {
                                0u8
                            };
                            let mut depth_size = if context_attrs.get_depth() { 16u8 } else { 0u8 };

                            if multi_sample {
                                if supports_transparency == alpha_requested
                                    && cconfig.alpha_size() == alpha_size
                                    && context_attrs.get_stencil()
                                    && cconfig.stencil_size() == stencil_size
                                    && context_attrs.get_depth()
                                    && cconfig.depth_size() >= depth_size
                                    && cconfig.num_samples() > 0
                                {
                                    if accum.supports_transparency().unwrap_or(false)
                                        == alpha_requested
                                        && accum.alpha_size() == alpha_size
                                        && context_attrs.get_stencil()
                                        && accum.stencil_size() == stencil_size
                                        && context_attrs.get_depth()
                                        && accum.depth_size() >= depth_size
                                        && accum.num_samples() > cconfig.num_samples()
                                    {
                                        return accum;
                                    }

                                    return cconfig;
                                }
                            } else {
                                if supports_transparency == alpha_requested
                                    && cconfig.alpha_size() == alpha_size
                                    && context_attrs.get_stencil()
                                    && cconfig.stencil_size() == stencil_size
                                    && context_attrs.get_depth()
                                    && cconfig.depth_size() >= depth_size
                                    && cconfig.num_samples() == 0
                                {
                                    if accum.supports_transparency().unwrap_or(false)
                                        == alpha_requested
                                        && accum.alpha_size() == alpha_size
                                        && context_attrs.get_stencil()
                                        && accum.stencil_size() == stencil_size
                                        && context_attrs.get_depth()
                                        && accum.depth_size() >= depth_size
                                        && accum.num_samples() == 0
                                    {
                                        return accum;
                                    }

                                    return cconfig;
                                }
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

                        inner.surface = surface;

                        ret
                    }
                    None => false,
                };
            }
        }
        false
    }

    pub fn create_shared_window_context(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        window: RawWindowHandle,
        context: &GLContext,
    ) -> Option<GLContext> {
        GLContext::create_window_context_internal(
            context_attrs,
            width,
            height,
            window,
            Some(context),
        )
    }

    pub fn create_window_context(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        window: RawWindowHandle,
    ) -> Option<GLContext> {
        GLContext::create_window_context_internal(context_attrs, width, height, window, None)
    }

    fn create_window_context_internal(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        window: RawWindowHandle,
        context: Option<&GLContext>,
    ) -> Option<GLContext> {
        match unsafe { Display::new(RawDisplayHandle::AppKit(AppKitDisplayHandle::empty())) } {
            Ok(display) => unsafe {
                let dsply = display.clone();
                IS_GL_SYMBOLS_LOADED.get_or_init(move || {
                    gl_bindings::load_with(|symbol| {
                        let symbol = CString::new(symbol).unwrap();
                        dsply.get_proc_address(symbol.as_c_str()).cast()
                    });
                    true
                });

                let is_2d = context_attrs.get_is_canvas();
                let multi_sample = context_attrs.get_antialias();
                let cfg = context_attrs.into();
                let config = display
                    .find_configs(cfg)
                    .map(|mut c| {
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

                            let alpha_requested = context_attrs.get_alpha();

                            let mut alpha_size = if alpha_requested { 8u8 } else { 0u8 };
                            let mut stencil_size = if context_attrs.get_stencil() {
                                8u8
                            } else {
                                0u8
                            };
                            let mut depth_size = if context_attrs.get_depth() { 16u8 } else { 0u8 };

                            if multi_sample {
                                if supports_transparency == alpha_requested
                                    && cconfig.alpha_size() == alpha_size
                                    && context_attrs.get_stencil()
                                    && cconfig.stencil_size() == stencil_size
                                    && context_attrs.get_depth()
                                    && cconfig.depth_size() == depth_size
                                    && cconfig.num_samples() > 0
                                {
                                    if accum.supports_transparency().unwrap_or(false)
                                        == alpha_requested
                                        && accum.alpha_size() == alpha_size
                                        && context_attrs.get_stencil()
                                        && accum.stencil_size() == stencil_size
                                        && context_attrs.get_depth()
                                        && accum.depth_size() == depth_size
                                        && accum.num_samples() > cconfig.num_samples()
                                    {
                                        return accum;
                                    }

                                    return cconfig;
                                }
                            } else {
                                if supports_transparency == alpha_requested
                                    && cconfig.alpha_size() == alpha_size
                                    && context_attrs.get_stencil()
                                    && cconfig.stencil_size() == stencil_size
                                    && context_attrs.get_depth()
                                    && cconfig.depth_size() >= depth_size
                                    && cconfig.num_samples() == 0
                                {
                                    if accum.supports_transparency().unwrap_or(false)
                                        == alpha_requested
                                        && accum.alpha_size() == alpha_size
                                        && context_attrs.get_stencil()
                                        && accum.stencil_size() == stencil_size
                                        && context_attrs.get_depth()
                                        && accum.depth_size() >= depth_size
                                        && accum.num_samples() == 0
                                    {
                                        return accum;
                                    }

                                    return cconfig;
                                }
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

                        let mut context_attrs_builder =
                            glutin::context::ContextAttributesBuilder::new().with_context_api(
                                ContextApi::OpenGl(Some(if context_attrs.get_is_canvas() {
                                    Version::new(2, 1)
                                } else {
                                    Version::new(3, 0)
                                })),
                            );

                        if let Some(context) = context {
                            let inner = context.inner.read();
                            if let Some(context) = inner.context.as_ref() {
                                context_attrs_builder =
                                    context_attrs_builder.with_context_api(context.context_api());
                            }
                        }

                        let context_attrs = context_attrs_builder.build(Some(window));

                        let context = display
                            .create_context(&config, &context_attrs)
                            .map(|v| v.treat_as_possibly_current())
                            .ok();

                        Some(GLContext {
                            inner: Arc::new(RwLock::new(GLContextInner {
                                surface,
                                context,
                                display: Some(display),
                                window: None,
                                event: None,
                                transfer_surface_info: Default::default(),
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
            let mut inner = self.inner.write();
            if let Some(display) = inner.display.as_ref() {
                let dsply = display.clone();
                IS_GL_SYMBOLS_LOADED.get_or_init(move || {
                    gl_bindings::load_with(|symbol| {
                        let symbol = CString::new(symbol).unwrap();
                        dsply.get_proc_address(symbol.as_c_str()).cast()
                    });
                    true
                });

                let is_2d = context_attrs.get_is_canvas();
                let cfg = context_attrs.clone().into();
                let multi_sample = context_attrs.get_antialias();
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

                            let alpha_requested = context_attrs.get_alpha();

                            let alpha_size = if alpha_requested { 8u8 } else { 0u8 };
                            let stencil_size = if context_attrs.get_stencil() {
                                8u8
                            } else {
                                0u8
                            };
                            let depth_size = if context_attrs.get_depth() { 16u8 } else { 0u8 };

                            if multi_sample {
                                if supports_transparency == alpha_requested
                                    && cconfig.alpha_size() == alpha_size
                                    && context_attrs.get_stencil()
                                    && cconfig.stencil_size() == stencil_size
                                    && context_attrs.get_depth()
                                    && cconfig.depth_size() >= depth_size
                                    && cconfig.num_samples() > 0
                                {
                                    if accum.supports_transparency().unwrap_or(false)
                                        == alpha_requested
                                        && accum.alpha_size() == alpha_size
                                        && context_attrs.get_stencil()
                                        && accum.stencil_size() == stencil_size
                                        && context_attrs.get_depth()
                                        && accum.depth_size() >= depth_size
                                        && accum.num_samples() > cconfig.num_samples()
                                    {
                                        return accum;
                                    }

                                    return cconfig;
                                }
                            } else if supports_transparency == alpha_requested
                                && cconfig.alpha_size() == alpha_size
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == stencil_size
                                && context_attrs.get_depth()
                                && cconfig.depth_size() >= depth_size
                                && cconfig.num_samples() == 0
                            {
                                if accum.supports_transparency().unwrap_or(false) == alpha_requested
                                    && accum.alpha_size() == alpha_size
                                    && context_attrs.get_stencil()
                                    && accum.stencil_size() == stencil_size
                                    && context_attrs.get_depth()
                                    && accum.depth_size() >= depth_size
                                    && accum.num_samples() == 0
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

                    inner.surface = surface;
                }
            }
        }
    }

    pub fn create_offscreen_context(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) -> Option<GLContext> {
        use winit::event_loop::EventLoop;
        use winit::window::WindowBuilder;

        let event_loop = EventLoop::new().unwrap();
        let window_builder = WindowBuilder::new();

        let window = window_builder
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        let raw_window_handle = window.raw_window_handle();

        GLContext::create_window_context(context_attrs, width, height, raw_window_handle).map(
            |ctx| {
                {
                    let mut context = ctx.inner.write();
                    context.window = Some(window);
                    context.event = Some(event_loop);
                }
                ctx
            },
        )
    }

    pub fn create_offscreen_context_with_event_loop(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        event_loop: &EventLoop<()>,
    ) -> Option<GLContext> {
        use winit::window::WindowBuilder;

        let window_builder = WindowBuilder::new();

        let window = window_builder
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .with_visible(false)
            .build(event_loop)
            .unwrap();

        let raw_window_handle = window.raw_window_handle();

        GLContext::create_window_context(context_attrs, width, height, raw_window_handle).map(
            |ctx| {
                {
                    let mut context = ctx.inner.write();
                    context.window = Some(window);
                    context.event = None;
                }
                ctx
            },
        )
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

    #[inline(always)]
    pub fn set_vsync(&self, sync: bool) -> bool {
        let inner = self.inner.read();
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
        let inner = self.inner.read();
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
        let inner = self.inner.read();
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
    }

    #[inline(always)]
    pub fn swap_buffers(&self) -> bool {
        let inner = self.inner.read();
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
        let inner = self.inner.read();
        inner
            .surface
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
        let inner = self.inner.read();
        inner
            .surface
            .as_ref()
            .map(|v| match v {
                SurfaceHelper::Window(window) => window.height().unwrap_or_default() as i32,
                SurfaceHelper::Pbuffer(buffer) => buffer.height().unwrap_or_default() as i32,
                SurfaceHelper::Pixmap(pixmap) => pixmap.height().unwrap_or_default() as i32,
            })
            .unwrap_or_default()
    }

    #[inline(always)]
    pub fn get_surface_dimensions(&self) -> (i32, i32) {
        let inner = self.inner.read();
        inner
            .surface
            .as_ref()
            .map(|v| match v {
                SurfaceHelper::Window(window) => (
                    window.width().unwrap_or_default() as i32,
                    window.height().unwrap_or_default() as i32,
                ),
                SurfaceHelper::Pbuffer(buffer) => (
                    buffer.width().unwrap_or_default() as i32,
                    buffer.height().unwrap_or_default() as i32,
                ),
                SurfaceHelper::Pixmap(pixmap) => (
                    pixmap.width().unwrap_or_default() as i32,
                    pixmap.height().unwrap_or_default() as i32,
                ),
            })
            .unwrap_or_default()
    }

    pub fn get_transfer_surface_info(&self) -> MappedRwLockReadGuard<crate::gl::TransferSurface> {
        RwLockReadGuard::map(self.inner.read(), |v| &v.transfer_surface_info)
    }

    pub fn get_transfer_surface_info_mut(
        &self,
    ) -> MappedRwLockWriteGuard<crate::gl::TransferSurface> {
        RwLockWriteGuard::map(self.inner.write(), |v| &mut v.transfer_surface_info)
    }
}
