use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::os::raw::c_void;
use std::sync::Arc;

#[cfg(target_os = "macos")]
use glutin::api::cgl::{
    config::Config, context::PossiblyCurrentContext, display::Display, surface::Surface,
};
#[cfg(any(target_os = "ios", target_os = "android"))]
use glutin::api::egl::{
    config::Config, context::PossiblyCurrentContext, display::Display, surface::Surface,
};
use glutin::config::{
    Api, AsRawConfig, ConfigSurfaceTypes, ConfigTemplate, ConfigTemplateBuilder, GetGlConfig,
};
use glutin::context::{ContextApi, GlContext, Version};
use glutin::display::GetGlDisplay;
use glutin::display::{AsRawDisplay, DisplayApiPreference};
use glutin::prelude::GlSurface;
use glutin::prelude::*;
use glutin::surface::{PbufferSurface, PixmapSurface, SurfaceAttributes, WindowSurface};
use parking_lot::{MappedRwLockReadGuard, MutexGuard, RwLock, RwLockReadGuard};
use raw_window_handle::{
    AndroidDisplayHandle, AppKitDisplayHandle, RawDisplayHandle, RawWindowHandle,
    UiKitDisplayHandle,
};

use crate::context_attributes::ContextAttributes;

#[derive(Debug)]
pub enum SurfaceHelper {
    Window(Surface<WindowSurface>),
    Pbuffer(Surface<PbufferSurface>),
    Pixmap(Surface<PixmapSurface>),
}

#[derive(Debug, Default)]
pub struct GLContextInner {
    surface: Option<SurfaceHelper>,
    context: Option<PossiblyCurrentContext>,
    display: Option<Display>,
}

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
        ConfigTemplateBuilder::new()
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
            .with_alpha_size(if value.get_alpha() { 8 } else { 0 })
            .with_depth_size(if value.get_depth() { 16 } else { 0 })
            .with_stencil_size(if value.get_stencil() { 8 } else { 0 })
            .with_transparency(value.get_alpha())
            .build()
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

impl GLContext {
    pub fn set_surface(
        &mut self,
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        window: RawWindowHandle,
    ) -> bool {
        let cfg = context_attrs.into();
        let mut lock = self.inner.write();
        unsafe {
            if let Some(display) = lock.display.as_ref() {
                let config = display
                    .find_configs(cfg)
                    .map(|c| {
                        c.reduce(|acc, cconfig| {
                            if cconfig.supports_transparency().unwrap_or_default()
                                && context_attrs.get_alpha()
                                && context_attrs.get_alpha()
                                && cconfig.alpha_size() == 8u8
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == 8u8
                                && context_attrs.get_depth()
                                && cconfig.depth_size() == 16u8
                            {
                                return cconfig;
                            }

                            acc
                        })
                    })
                    .ok()
                    .flatten();

                return match config {
                    Some(config) => {
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

                        lock.surface = surface;

                        ret
                    }
                    None => false,
                };
            }
        }
        false
    }

    #[cfg(target_os = "macos")]
    pub fn create_window_surface(
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

                let cfg = context_attrs.into();
                let config = display
                    .find_configs(cfg)
                    .map(|c| {
                        c.reduce(|acc, cconfig| {
                            if cconfig.supports_transparency().unwrap_or_default()
                                && context_attrs.get_alpha()
                                && context_attrs.get_alpha()
                                && cconfig.alpha_size() == 8u8
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == 8u8
                                && context_attrs.get_depth()
                                && cconfig.depth_size() == 16u8
                            {
                                return cconfig;
                            }

                            acc
                        })
                    })
                    .ok()
                    .flatten();

                match config {
                    Some(config) => {
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
                            inner: Arc::new(RwLock::new(GLContextInner {
                                surface,
                                context,
                                display: Some(display),
                            })),
                        })
                    }
                    None => None,
                }
            },
            Err(_) => None,
        }
    }

    #[cfg(target_os = "ios")]
    pub fn create_window_surface(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        window: RawWindowHandle,
    ) -> Option<GLContext> {
        match unsafe { Display::new(RawDisplayHandle::UiKit(UiKitDisplayHandle::empty())) } {
            Ok(display) => unsafe {
                gl_bindings::load_with(|symbol| {
                    let symbol = CString::new(symbol).unwrap();
                    display.get_proc_address(symbol.as_c_str()).cast()
                });

                let cfg = context_attrs.into();
                let config = display
                    .find_configs(cfg)
                    .map(|c| {
                        c.reduce(|acc, cconfig| {
                            if cconfig.supports_transparency().unwrap_or_default()
                                && context_attrs.get_alpha()
                                && context_attrs.get_alpha()
                                && cconfig.alpha_size() == 8u8
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == 8u8
                                && context_attrs.get_depth()
                                && cconfig.depth_size() == 16u8
                            {
                                return cconfig;
                            }

                            acc
                        })
                    })
                    .ok()
                    .flatten();

                match config {
                    Some(config) => {
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

                        let context_attrs = glutin::context::ContextAttributesBuilder::new()
                            .with_context_api(ContextApi::Gles(Some(
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
                            inner: Arc::new(RwLock::new(GLContextInner {
                                surface,
                                context,
                                display: Some(display),
                            })),
                        })
                    }
                    None => None,
                }
            },
            Err(_) => None,
        }
    }

    #[cfg(target_os = "android")]
    pub fn create_window_surface(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
        window: RawWindowHandle,
    ) -> Option<GLContext> {
        match unsafe { Display::new(RawDisplayHandle::Android(AndroidDisplayHandle::empty())) } {
            Ok(display) => unsafe {
                gl_bindings::load_with(|symbol| {
                    let symbol = CString::new(symbol).unwrap();
                    display.get_proc_address(symbol.as_c_str()).cast()
                });

                let cfg = context_attrs.into();
                let config = display
                    .find_configs(cfg)
                    .map(|c| {
                        c.reduce(|acc, cconfig| {
                            if cconfig.supports_transparency().unwrap_or_default()
                                && context_attrs.get_alpha()
                                && context_attrs.get_alpha()
                                && cconfig.alpha_size() == 8u8
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == 8u8
                                && context_attrs.get_depth()
                                && cconfig.depth_size() == 16u8
                            {
                                return cconfig;
                            }

                            acc
                        })
                    })
                    .ok()
                    .flatten();

                match config {
                    Some(config) => {
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

                        let context_attrs = glutin::context::ContextAttributesBuilder::new()
                            .with_context_api(ContextApi::Gles(Some(
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
                            inner: Arc::new(RwLock::new(GLContextInner {
                                surface,
                                context,
                                display: Some(display),
                            })),
                        })
                    }
                    None => None,
                }
            },
            Err(_) => None,
        }
    }


    #[cfg(target_os = "android")]
    pub fn set_window_surface(
        &mut self,
        context_attrs: &ContextAttributes,
        width: i32,
        height: i32,
        window: RawWindowHandle,
    )  {
        unsafe {
            if let Some(display) = self.display() {
                gl_bindings::load_with(|symbol| {
                    let symbol = CString::new(symbol).unwrap();
                    display.get_proc_address(symbol.as_c_str()).cast()
                });

                let cfg = context_attrs.clone().into();
                let config = display
                    .find_configs(cfg)
                    .map(|c| {
                        c.reduce(|acc, cconfig| {
                            if cconfig.supports_transparency().unwrap_or_default()
                                && context_attrs.get_alpha()
                                && context_attrs.get_alpha()
                                && cconfig.alpha_size() == 8u8
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == 8u8
                                && context_attrs.get_depth()
                                && cconfig.depth_size() == 16u8
                            {
                                return cconfig;
                            }

                            acc
                        })
                    })
                    .ok()
                    .flatten();

                if let Some(config) = config {
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


                    let mut lock = self.inner.write();
                    lock.surface = surface;
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub fn create_pbuffer(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) -> Option<GLContext> {
        match unsafe { Display::new(RawDisplayHandle::AppKit(AppKitDisplayHandle::empty())) } {
            Ok(display) => unsafe {
                gl_bindings::load_with(|symbol| {
                    let symbol = CString::new(symbol).unwrap();
                    display.get_proc_address(symbol.as_c_str()).cast()
                });

                let cfg = context_attrs.into();
                let config = display
                    .find_configs(cfg)
                    .map(|c| {
                        c.reduce(|acc, cconfig| {
                            if cconfig.supports_transparency().unwrap_or_default()
                                && context_attrs.get_alpha()
                                && context_attrs.get_alpha()
                                && cconfig.alpha_size() == 8u8
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == 8u8
                                && context_attrs.get_depth()
                                && cconfig.depth_size() == 16u8
                            {
                                return cconfig;
                            }

                            acc
                        })
                    })
                    .ok()
                    .flatten();

                match config {
                    Some(config) => {
                        let surface_attr =
                            glutin::surface::SurfaceAttributesBuilder::<PbufferSurface>::new()
                                .build(
                                    NonZeroU32::try_from(width as u32).unwrap(),
                                    NonZeroU32::try_from(height as u32).unwrap(),
                                );

                        let surface = display
                            .create_pbuffer_surface(&config, &surface_attr)
                            .map(|v| SurfaceHelper::Pbuffer(v))
                            .ok();

                        let context_attrs = glutin::context::ContextAttributesBuilder::new()
                            .with_context_api(ContextApi::OpenGl(Some(
                                if context_attrs.get_is_canvas() {
                                    Version::new(2, 1)
                                } else {
                                    Version::new(3, 0)
                                },
                            )))
                            .build(None);

                        let context = display
                            .create_context(&config, &context_attrs)
                            .map(|v| v.treat_as_possibly_current())
                            .ok();

                        Some(GLContext {
                            inner: Arc::new(RwLock::new(GLContextInner {
                                surface,
                                context,
                                display: Some(display),
                            })),
                        })
                    }
                    None => None,
                }
            },
            Err(_) => None,
        }
    }

    #[cfg(target_os = "ios")]
    pub fn create_pbuffer(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) -> Option<GLContext> {
        match unsafe { Display::new(RawDisplayHandle::Android(AndroidDisplayHandle::empty())) } {
            Ok(display) => unsafe {
                gl_bindings::load_with(|symbol| {
                    let symbol = CString::new(symbol).unwrap();
                    display.get_proc_address(symbol.as_c_str()).cast()
                });

                let cfg = context_attrs.into();
                let config = display
                    .find_configs(cfg)
                    .map(|c| {
                        c.reduce(|acc, cconfig| {
                            if cconfig.supports_transparency().unwrap_or_default()
                                && context_attrs.get_alpha()
                                && context_attrs.get_alpha()
                                && cconfig.alpha_size() == 8u8
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == 8u8
                                && context_attrs.get_depth()
                                && cconfig.depth_size() == 16u8
                            {
                                return cconfig;
                            }

                            acc
                        })
                    })
                    .ok()
                    .flatten();

                match config {
                    Some(config) => {
                        let surface_attr =
                            glutin::surface::SurfaceAttributesBuilder::<PbufferSurface>::new()
                                .build(
                                    NonZeroU32::try_from(width as u32).unwrap(),
                                    NonZeroU32::try_from(height as u32).unwrap(),
                                );

                        let surface = display
                            .create_pbuffer_surface(&config, &surface_attr)
                            .map(|v| SurfaceHelper::Pbuffer(v))
                            .ok();

                        let context_attrs = glutin::context::ContextAttributesBuilder::new()
                            .with_context_api(ContextApi::Gles(Some(
                                if context_attrs.get_is_canvas() {
                                    Version::new(2, 1)
                                } else {
                                    Version::new(3, 0)
                                },
                            )))
                            .build(None);

                        let context = display
                            .create_context(&config, &context_attrs)
                            .map(|v| v.treat_as_possibly_current())
                            .ok();

                        Some(GLContext {
                            inner: Arc::new(RwLock::new(GLContextInner {
                                surface,
                                context,
                                display: Some(display),
                            })),
                        })
                    }
                    None => None,
                }
            },
            Err(_) => None,
        }
    }

    #[cfg(target_os = "android")]
    pub fn create_pbuffer(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) -> Option<GLContext> {
        match unsafe { Display::new(RawDisplayHandle::UiKit(UiKitDisplayHandle::empty())) } {
            Ok(display) => unsafe {
                gl_bindings::load_with(|symbol| {
                    let symbol = CString::new(symbol).unwrap();
                    display.get_proc_address(symbol.as_c_str()).cast()
                });

                let cfg = context_attrs.into();
                let config = display
                    .find_configs(cfg)
                    .map(|c| {
                        c.reduce(|acc, cconfig| {
                            if cconfig.supports_transparency().unwrap_or_default()
                                && context_attrs.get_alpha()
                                && context_attrs.get_alpha()
                                && cconfig.alpha_size() == 8u8
                                && context_attrs.get_stencil()
                                && cconfig.stencil_size() == 8u8
                                && context_attrs.get_depth()
                                && cconfig.depth_size() == 16u8
                            {
                                return cconfig;
                            }

                            acc
                        })
                    })
                    .ok()
                    .flatten();

                match config {
                    Some(config) => {
                        let surface_attr =
                            glutin::surface::SurfaceAttributesBuilder::<PbufferSurface>::new()
                                .build(
                                    NonZeroU32::try_from(width as u32).unwrap(),
                                    NonZeroU32::try_from(height as u32).unwrap(),
                                );

                        let surface = display
                            .create_pbuffer_surface(&config, &surface_attr)
                            .map(|v| SurfaceHelper::Pbuffer(v))
                            .ok();

                        let context_attrs = glutin::context::ContextAttributesBuilder::new()
                            .with_context_api(ContextApi::Gles(Some(
                                if context_attrs.get_is_canvas() {
                                    Version::new(2, 1)
                                } else {
                                    Version::new(3, 0)
                                },
                            )))
                            .build(None);

                        let context = display
                            .create_context(&config, &context_attrs)
                            .map(|v| v.treat_as_possibly_current())
                            .ok();

                        Some(GLContext {
                            inner: Arc::new(RwLock::new(GLContextInner {
                                surface,
                                context,
                                display: Some(display),
                            })),
                        })
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

    #[cfg(target_os = "macos")]
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

    #[cfg(target_os = "ios")]
    pub fn has_gl2support() -> bool {
        match unsafe { Display::new(RawDisplayHandle::UiKit(UiKitDisplayHandle::empty())) } {
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

    #[cfg(target_os = "android")]
    pub fn has_gl2support() -> bool {
        match unsafe { Display::new(RawDisplayHandle::Android(AndroidDisplayHandle::empty())) } {
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

    pub fn new_with_no_window(
        width: i32,
        height: i32,
        config: &mut ContextAttributes,
    ) -> Option<Self> {
        GLContext::create_pbuffer(config, width, height)
    }

    pub fn surface(&self) -> Option<MappedRwLockReadGuard<SurfaceHelper>> {
        RwLockReadGuard::try_map(self.inner.read(), |lock| lock.surface.as_ref()).ok()
    }

    pub fn context(&self) -> Option<MappedRwLockReadGuard<PossiblyCurrentContext>> {
        RwLockReadGuard::try_map(self.inner.read(), |lock| lock.context.as_ref()).ok()
    }

    pub fn display(&self) -> Option<MappedRwLockReadGuard<Display>> {
        RwLockReadGuard::try_map(self.inner.read(), |lock| lock.display.as_ref()).ok()
    }

    pub fn make_current(&self) -> bool {
        let lock = self.inner.read();
        match (lock.context.as_ref(), lock.surface.as_ref()) {
            (Some(context), Some(surface)) => match surface {
                SurfaceHelper::Window(window) => context.make_current(window).is_ok(),
                SurfaceHelper::Pbuffer(buffer) => context.make_current(buffer).is_ok(),
                SurfaceHelper::Pixmap(map) => context.make_current(map).is_ok(),
            },
            _ => false,
        }
    }

    pub fn remove_if_current(&self) {
        if let Some(context) = self.inner.write().context.as_ref() {}
    }

    pub fn swap_buffers(&self) -> bool {
        let lock = self.inner.read();
        match (lock.context.as_ref(), lock.surface.as_ref()) {
            (Some(context), Some(surface)) => match surface {
                SurfaceHelper::Window(window) => window.swap_buffers(context).is_ok(),
                SurfaceHelper::Pbuffer(buffer) => buffer.swap_buffers(context).is_ok(),
                SurfaceHelper::Pixmap(map) => map.swap_buffers(context).is_ok(),
            },
            _ => false,
        }
    }

    pub fn get_surface_width(&self) -> i32 {
        self.inner
            .read()
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
        self.inner
            .read()
            .surface
            .as_ref()
            .map(|v| match v {
                SurfaceHelper::Window(window) => window.height().unwrap_or_default() as i32,
                SurfaceHelper::Pbuffer(buffer) => buffer.height().unwrap_or_default() as i32,
                SurfaceHelper::Pixmap(pixmap) => pixmap.height().unwrap_or_default() as i32,
            })
            .unwrap_or_default()
    }
}
