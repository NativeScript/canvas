use std::ffi::CString;
use std::fmt::{Debug, Formatter};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::sync::OnceLock;

use glutin::api::egl::{
    context::{NotCurrentContext, PossiblyCurrentContext}, display::Display, surface::Surface,
};
use glutin::config::{
    Api, ColorBufferType, ConfigSurfaceTypes, ConfigTemplate, ConfigTemplateBuilder,
};
use glutin::context::{ContextApi, Version};
use glutin::display::{GetGlDisplay, RawDisplay};
use glutin::display::AsRawDisplay;
use glutin::prelude::*;
use glutin::prelude::GlSurface;
use glutin::surface::{PbufferSurface, PixmapSurface, SwapInterval, WindowSurface};
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use raw_window_handle::{AndroidDisplayHandle, RawDisplayHandle, RawWindowHandle};

use crate::context_attributes::ContextAttributes;

pub static IS_GL_SYMBOLS_LOADED: OnceLock<bool> = OnceLock::new();

pub(crate) enum SurfaceHelper {
    Window(Surface<WindowSurface>),
    Pbuffer(Surface<PbufferSurface>),
    Pixmap(Surface<PixmapSurface>),
}

#[derive(Default)]
pub struct GLContextInner {
    surface: Option<SurfaceHelper>,
    context: Option<PossiblyCurrentContext>,
    display: Option<Display>,
    transfer_surface_info: crate::gl::TransferSurface,
}

unsafe impl Sync for GLContextInner {}

unsafe impl Send for GLContextInner {}

#[derive(Default)]
pub struct GLContext(Arc<RwLock<GLContextInner>>);

impl Debug for GLContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // todo
        f.debug_struct("GLContext")
            .finish()
    }
}

impl GLContext {
    // pointer has to
    pub fn as_raw_inner(&self) -> *const RwLock<GLContextInner> {
        Arc::into_raw(Arc::clone(&self.0))
    }

    pub fn from_raw_inner(raw: *const RwLock<GLContextInner>) -> Self {
        Self(unsafe { Arc::from_raw(raw) })
    }

    pub fn get_strong_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }

    pub fn increment_strong_count(self) {
        let ptr = Arc::into_raw(self.0);
        unsafe { Arc::increment_strong_count(ptr) }
    }

    pub fn decrement_strong_count(self) {
        let ptr = Arc::into_raw(self.0);
        unsafe { Arc::decrement_strong_count(ptr) }
        let _ = unsafe { Arc::from_raw(ptr) };
    }
}

impl Clone for GLContext {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Into<ConfigTemplate> for ContextAttributes {
    fn into(self) -> ConfigTemplate {
        let mut builder = ConfigTemplateBuilder::new()
            .with_api(if self.get_is_canvas() {
                Api::GLES2
            } else {
                Api::GLES3
            })
            .prefer_hardware_accelerated(Some(true))
            .with_alpha_size(if self.get_alpha() { 8 } else { 0 })
            .with_depth_size(if self.get_depth() { 24 } else { 0 })
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
            .with_api(if value.get_is_canvas() {
                glutin::config::Api::GLES2
            } else {
                glutin::config::Api::GLES3
            })
            .prefer_hardware_accelerated(Some(true))
            .with_alpha_size(if value.get_alpha() { 8 } else { 0 })
            .with_depth_size(if value.get_depth() { 24 } else { 0 })
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
            .with_api(if self.get_is_canvas() {
                glutin::config::Api::GLES2
            } else {
                glutin::config::Api::GLES3
            })
            .prefer_hardware_accelerated(Some(true))
            .with_alpha_size(if self.get_alpha() { 8 } else { 0 })
            .with_depth_size(if self.get_depth() { 24 } else { 0 })
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
            .with_api(if value.get_is_canvas() {
                glutin::config::Api::GLES2
            } else {
                glutin::config::Api::GLES3
            })
            .prefer_hardware_accelerated(Some(true))
            .with_alpha_size(if value.get_alpha() { 8 } else { 0 })
            .with_depth_size(if value.get_depth() { 24 } else { 0 })
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
//             .with_transparency(self.get_alpha())
//             .build()
//     }
// }

#[cfg(target_os = "android")]
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
        let cfg = context_attrs.clone().into();

        let mut inner = self.0.write();
        unsafe {
            if let Some(display) = inner.display.as_ref() {
                let mut config = display
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

                            return if cconfig.num_samples() > accum.num_samples() {
                                cconfig
                            } else {
                                accum
                            };
                        })
                    })
                    .ok()
                    .flatten();

                if config.is_none() {
                    let mut cfg: ConfigTemplateBuilder = context_attrs.clone().into();

                    cfg = cfg.with_depth_size(if context_attrs.get_depth() { 16 } else { 0 });

                    let cfg = cfg.build();

                    config = display
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

                                return if cconfig.num_samples() > accum.num_samples() {
                                    cconfig
                                } else {
                                    accum
                                };
                            })
                        })
                        .ok()
                        .flatten();
                }

                if config.is_none() && multi_sample {
                    context_attrs.set_antialias(false);
                    let cfg = context_attrs.into();
                    config = display
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

                                return if cconfig.num_samples() > accum.num_samples() {
                                    cconfig
                                } else {
                                    accum
                                };
                            })
                        })
                        .ok()
                        .flatten();
                }

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

                        drop(inner);

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
        shared_context: &GLContext,
    ) -> Option<GLContext> {
        GLContext::create_window_context_internal(
            context_attrs,
            width,
            height,
            window,
            Some(shared_context),
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

    pub(crate) fn create_window_context_internal(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        window: RawWindowHandle,
        shared_context: Option<&GLContext>,
    ) -> Option<GLContext> {
        match unsafe { Display::new(RawDisplayHandle::Android(AndroidDisplayHandle::new())) } {
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

                let mut config = display
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

                            return if cconfig.num_samples() > accum.num_samples() {
                                cconfig
                            } else {
                                accum
                            };
                        })
                    })
                    .ok()
                    .flatten();

                if config.is_none() {
                    let mut cfg: ConfigTemplateBuilder = context_attrs.clone().into();

                    cfg = cfg.with_depth_size(if context_attrs.get_depth() { 16 } else { 0 });

                    let cfg = cfg.build();

                    config = display
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

                                return if cconfig.num_samples() > accum.num_samples() {
                                    cconfig
                                } else {
                                    accum
                                };
                            })
                        })
                        .ok()
                        .flatten();
                }

                if config.is_none() && multi_sample {
                    context_attrs.set_antialias(false);
                    let cfg = context_attrs.into();
                    config = display
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

                                return if cconfig.num_samples() > accum.num_samples() {
                                    cconfig
                                } else {
                                    accum
                                };
                            })
                        })
                        .ok()
                        .flatten();
                }

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
                            .map(|v| SurfaceHelper::Window(v))
                            .ok();

                        let mut context_attrs = glutin::context::ContextAttributesBuilder::new()
                            .with_context_api(ContextApi::Gles(Some(
                                if context_attrs.get_is_canvas() {
                                    Version::new(2, 0)
                                } else {
                                    Version::new(3, 0)
                                },
                            )));

                        if let Some(inner) = shared_context {
                            let inner = inner.0.read();
                            if let Some(context) = inner.context.as_ref() {
                                context_attrs = context_attrs.with_sharing(context);
                            }
                            drop(inner);
                        }

                        let context_attrs = context_attrs.build(Some(window));

                        let context = display
                            .create_context(&config, &context_attrs)
                            .map(|v| v.treat_as_possibly_current())
                            .ok();

                        let gl_context = GLContextInner {
                            surface,
                            context,
                            display: Some(display),
                            transfer_surface_info: Default::default(),
                        };

                        let ret = GLContext(Arc::new(RwLock::new(gl_context)));

                        Some(ret)
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
            let mut inner = self.0.write();

            // remove current surface
            let context = inner.context.take();
            let surface = inner.surface.take();
            match context {
                Some(context) => {
                    let context = context.make_not_current().ok();
                    // only drop the surface only after

                    drop(surface);

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
                        let multi_sample = context_attrs.get_antialias();
                        let cfg = context_attrs.clone().into();
                        let mut config = display
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

                                    return if cconfig.num_samples() > accum.num_samples() {
                                        cconfig
                                    } else {
                                        accum
                                    };
                                })
                            })
                            .ok()
                            .flatten();

                        if config.is_none() {
                            let mut cfg: ConfigTemplateBuilder = context_attrs.clone().into();

                            cfg = cfg.with_depth_size(if context_attrs.get_depth() { 16 } else { 0 });

                            let cfg = cfg.build();

                            config = display
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

                                        return if cconfig.num_samples() > accum.num_samples() {
                                            cconfig
                                        } else {
                                            accum
                                        };
                                    })
                                })
                                .ok()
                                .flatten();
                        }

                        if config.is_none() && multi_sample {
                            context_attrs.set_antialias(false);
                            let cfg = context_attrs.into();
                            config = display
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

                                        return if cconfig.num_samples() > accum.num_samples() {
                                            cconfig
                                        } else {
                                            accum
                                        };
                                    })
                                })
                                .ok()
                                .flatten();
                        }

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
                            inner.context = match (context, inner.surface.as_ref()) {
                                (Some(context), Some(surface)) => {
                                    match surface {
                                        SurfaceHelper::Window(window) => {
                                            context.make_current(window).ok()
                                        }
                                        SurfaceHelper::Pbuffer(buffer) => context.make_current(buffer).ok(),
                                        SurfaceHelper::Pixmap(map) => context.make_current(map).ok(),
                                    }
                                }
                                _ => None
                            };
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn resize_pbuffer(
        &mut self,
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) {
        unsafe {
            let mut inner = self.0.write();
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
                let multi_sample = context_attrs.get_antialias();
                let cfg: ConfigTemplateBuilder = context_attrs.into();

                let cfg = cfg
                    .with_surface_type(ConfigSurfaceTypes::PBUFFER)
                    .with_pbuffer_sizes(
                        (width as u32).try_into().unwrap(),
                        (height as u32).try_into().unwrap(),
                    )
                    .build();

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

                            return if cconfig.num_samples() > accum.num_samples() {
                                cconfig
                            } else {
                                accum
                            };
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
                        glutin::surface::SurfaceAttributesBuilder::<PbufferSurface>::new().build(
                            NonZeroU32::try_from(width as u32).unwrap(),
                            NonZeroU32::try_from(height as u32).unwrap(),
                        );

                    let surface = display
                        .create_pbuffer_surface(&config, &surface_attr)
                        .map(SurfaceHelper::Pbuffer)
                        .ok();

                    inner.surface = surface;
                    drop(inner);
                }
            }
        }
    }

    pub fn create_pbuffer(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) -> Option<GLContext> {
        match unsafe { Display::new(RawDisplayHandle::Android(AndroidDisplayHandle::new())) } {
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
                let mut cfg: ConfigTemplateBuilder = context_attrs.into();

                let cfg = cfg
                    .with_surface_type(ConfigSurfaceTypes::PBUFFER)
                    .with_pbuffer_sizes(
                        (width as u32).try_into().unwrap(),
                        (height as u32).try_into().unwrap(),
                    )
                    .build();

                let mut config = display
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

                            return if cconfig.num_samples() > accum.num_samples() {
                                cconfig
                            } else {
                                accum
                            };
                        })
                    })
                    .ok()
                    .flatten();

                if config.is_none() {
                    let mut cfg: ConfigTemplateBuilder = context_attrs.clone().into();

                    cfg = cfg.with_depth_size(if context_attrs.get_depth() { 16 } else { 0 });

                    let cfg = cfg.build();

                    config = display
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

                                return if cconfig.num_samples() > accum.num_samples() {
                                    cconfig
                                } else {
                                    accum
                                };
                            })
                        })
                        .ok()
                        .flatten();
                }

                if config.is_none() && multi_sample {
                    context_attrs.set_antialias(false);

                    let mut cfg: ConfigTemplateBuilder = context_attrs.into();

                    let cfg = cfg
                        .with_surface_type(ConfigSurfaceTypes::PBUFFER)
                        .with_pbuffer_sizes(
                            (width as u32).try_into().unwrap(),
                            (height as u32).try_into().unwrap(),
                        )
                        .build();

                    config = display
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

                                return if cconfig.num_samples() > accum.num_samples() {
                                    cconfig
                                } else {
                                    accum
                                };
                            })
                        })
                        .ok()
                        .flatten();
                }

                match config {
                    Some(config) => {
                        if context_attrs.get_antialias() && config.num_samples() == 0 {
                            context_attrs.set_antialias(false);
                        }

                        context_attrs.set_samples(config.num_samples());

                        let surface_attr =
                            glutin::surface::SurfaceAttributesBuilder::<PbufferSurface>::new()
                                .build(
                                    NonZeroU32::try_from(width as u32).unwrap(),
                                    NonZeroU32::try_from(height as u32).unwrap(),
                                );

                        let surface = display
                            .create_pbuffer_surface(&config, &surface_attr)
                            .map(SurfaceHelper::Pbuffer)
                            .ok();

                        let context_attrs = glutin::context::ContextAttributesBuilder::new()
                            .with_context_api(ContextApi::Gles(Some(
                                if context_attrs.get_is_canvas() {
                                    Version::new(2, 0)
                                } else {
                                    Version::new(3, 0)
                                },
                            )))
                            .build(None);

                        let context = display
                            .create_context(&config, &context_attrs)
                            .map(|v| v.treat_as_possibly_current())
                            .ok();

                        Some(GLContext(Arc::new(RwLock::new(GLContextInner {
                            surface,
                            context,
                            display: Some(display),
                            transfer_surface_info: Default::default(),
                        }))))
                    }
                    None => None,
                }
            },
            Err(_) => None,
        }
    }

    fn has_extension(extensions: &str, name: &str) -> bool {
        !extensions.split(' ').into_iter().any(|s| s == name)
    }

    pub fn has_gl2support() -> bool {
        match unsafe { Display::new(RawDisplayHandle::Android(AndroidDisplayHandle::new())) } {
            Ok(display) => unsafe {
                display
                    .find_configs(
                        ConfigTemplateBuilder::default()
                            .with_api(Api::GLES3)
                            .build(),
                    )
                    .is_ok()
            },
            Err(_) => false,
        }
    }

    pub fn create_offscreen_context(
        config: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) -> Option<Self> {
        GLContext::create_pbuffer(config, width, height)
    }

    pub fn set_vsync(&self, sync: bool) -> bool {
        let inner = self.0.read();
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

    pub fn make_current(&self) -> bool {
        let inner = self.0.read();
        match (inner.context.as_ref(), inner.surface.as_ref()) {
            (Some(context), Some(surface)) => {
                if context.is_current() {
                    return true;
                }

                return match surface {
                    SurfaceHelper::Window(window) => context.make_current(window).is_ok(),
                    SurfaceHelper::Pbuffer(buffer) => context.make_current(buffer).is_ok(),
                    SurfaceHelper::Pixmap(map) => context.make_current(map).is_ok(),
                };
            }
            _ => false,
        }
    }

    pub fn remove_if_current(&self) {
        let inner = self.0.read();
        let is_current = match (inner.context.as_ref(), inner.surface.as_ref()) {
            (Some(context), Some(surface)) => {
                if !context.is_current() {
                    false
                } else {
                    match surface {
                        SurfaceHelper::Window(window) => window.is_current(context),
                        SurfaceHelper::Pbuffer(buffer) => buffer.is_current(context),
                        SurfaceHelper::Pixmap(map) => map.is_current(context),
                    }
                }
            }
            _ => false,
        };

        if !is_current {
            return;
        }

        {
            let display = inner
                .display
                .as_ref()
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

    pub fn swap_buffers(&self) -> bool {
        let inner = self.0.read();
        match (inner.context.as_ref(), inner.surface.as_ref()) {
            (Some(context), Some(surface)) => match surface {
                SurfaceHelper::Window(window) => window.swap_buffers(context).is_ok(),
                SurfaceHelper::Pbuffer(buffer) => buffer.swap_buffers(context).is_ok(),
                SurfaceHelper::Pixmap(map) => map.swap_buffers(context).is_ok(),
            },
            _ => false,
        }
    }

    pub fn get_surface_width(&self) -> i32 {
        let inner = self.0.read();
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

    pub fn get_surface_height(&self) -> i32 {
        let inner = self.0.read();
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

    pub fn get_surface_dimensions(&self) -> (i32, i32) {
        let inner = self.0.read();
        inner
            .surface
            .as_ref()
            .map(|v| match v {
                crate::gl::SurfaceHelper::Window(window) => (
                    window.width().unwrap_or_default() as i32,
                    window.height().unwrap_or_default() as i32,
                ),
                crate::gl::SurfaceHelper::Pbuffer(buffer) => (
                    buffer.width().unwrap_or_default() as i32,
                    buffer.height().unwrap_or_default() as i32,
                ),
                crate::gl::SurfaceHelper::Pixmap(pixmap) => (
                    pixmap.width().unwrap_or_default() as i32,
                    pixmap.height().unwrap_or_default() as i32,
                ),
            })
            .unwrap_or_default()
    }

    pub fn get_transfer_surface_info(&self) -> MappedRwLockReadGuard<crate::gl::TransferSurface> {
        RwLockReadGuard::map(self.0.read(), |v| &v.transfer_surface_info)
    }

    pub fn get_transfer_surface_info_mut(
        &self,
    ) -> MappedRwLockWriteGuard<crate::gl::TransferSurface> {
        RwLockWriteGuard::map(self.0.write(), |v| &mut v.transfer_surface_info)
    }
}
