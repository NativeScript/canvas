use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::os::raw::c_void;

#[cfg(target_os = "macos")]
use glutin::api::cgl::{
    config::Config, context::PossiblyCurrentContext, display::Display, surface::Surface,
};
#[cfg(any(target_os = "ios", target_os = "android"))]
use glutin::api::egl::{
    config::Config, context::PossiblyCurrentContext, display::Display, surface::Surface,
};
use glutin::config::{ConfigSurfaceTypes, ConfigTemplate, ConfigTemplateBuilder, GetGlConfig};
use glutin::context::{ContextApi, GlContext, Version};
use glutin::display::GetGlDisplay;
use glutin::display::{AsRawDisplay, DisplayApiPreference};
use glutin::prelude::GlSurface;
use glutin::prelude::*;
use glutin::surface::{PbufferSurface, PixmapSurface, SurfaceAttributes, WindowSurface};
use raw_window_handle::{
    AndroidDisplayHandle, AppKitDisplayHandle, RawDisplayHandle, RawWindowHandle,
    UiKitDisplayHandle,
};

use crate::context_attributes::ContextAttributes;
use crate::gl;

const EGL_CONTEXT_MINOR_VERSION: i32 = 0x000030fb;

#[derive(Debug)]
pub enum SurfaceHelper {
    Window(Surface<WindowSurface>),
    Pbuffer(Surface<PbufferSurface>),
    Pixmap(Surface<PixmapSurface>),
}

#[derive(Debug, Default)]
pub struct GLContext {
    surface: Option<SurfaceHelper>,
    context: Option<PossiblyCurrentContext>,
    display: Option<Display>,
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
                            .map(|v| SurfaceHelper::Window(v))
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
                            surface,
                            context,
                            display: Some(display),
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
                            surface,
                            context,
                            display: Some(display),
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
                            surface,
                            context,
                            display: Some(display),
                        })
                    }
                    None => None,
                }
            },
            Err(_) => None,
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
                            surface,
                            context,
                            display: Some(display),
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
                            surface,
                            context,
                            display: Some(display),
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
                            surface,
                            context,
                            display: Some(display),
                        })
                    }
                    None => None,
                }
            },
            Err(_) => None,
        }
    }

    fn has_extension(extensions: &str, name: &str) -> bool {
        !extensions
            .split(' ')
            .into_iter().any(|s| s == name)
    }

    fn has_gl2support() -> bool {
        return true;
        /* let mut highest_es_version = 0;
        glutin::display::Display::new(
            RawDisplayHandle::AppKit()
        )

        let display = egl::get_display(EGL_DEFAULT_DISPLAY);
        if display.is_none() {
            return false;
        }
        let display = display.unwrap();
        let query = egl::query_string(display, EGL_EXTENSIONS);
        if query.is_none() {
            return false;
        }
        let query = query.unwrap();
        let check_es3 =
            GLContext::has_extension(query.to_string_lossy().as_ref(), "EGL_KHR_create_context");

        let list = egl::get_configs(display, 1);

        let mut value = 0i32;
        for _ in 0..list.count {
            if egl::get_config_attrib(display, list.configs, EGL_RENDERABLE_TYPE, &mut value) {
                if check_es3 && value & EGL_VG_ALPHA_FORMAT_PRE_BIT == EGL_VG_ALPHA_FORMAT_PRE_BIT {
                    if highest_es_version < 3 {
                        highest_es_version = 3;
                    }
                } else if value & EGL_OPENGL_ES2_BIT == EGL_OPENGL_ES2_BIT {
                    if highest_es_version < 2 {
                        highest_es_version = 2;
                    }
                } else if value & EGL_OPENGL_ES_BIT == EGL_OPENGL_ES_BIT {
                    if highest_es_version < 1 {
                        highest_es_version = 1
                    }
                }
            }
        }

        if highest_es_version >= 3 {
            return true;
        }
        return false;

        */
    }

    // pub fn new_with_window(
    //     window: *mut c_void,
    //     width: i32,
    //     height: i32,
    //     surface_type: i32,
    //     config: &mut ContextAttributes,
    // ) -> Self {
    //     match GLContext::create_surface(surface_type, config, width, height, window) {
    //         Some((display, _, context, surface)) => Self {
    //             surface: Some(surface),
    //             context: Some(context),
    //             display: Some(display),
    //         },
    //         None => Self::default(),
    //     }
    // }

    pub fn new_with_no_window(
        width: i32,
        height: i32,
        config: &mut ContextAttributes,
    ) -> Option<Self> {
        GLContext::create_pbuffer(config, width, height)
    }

    // pub fn get_current() -> Self {
    //     Self {
    //         surface: egl::get_current_surface(egl::EGL_DRAW),
    //         context: egl::get_current_context(),
    //         display: egl::get_current_display(),
    //     }
    // }

    pub fn surface(&self) -> Option<&SurfaceHelper> {
        self.surface.as_ref()
    }

    pub fn context(&self) -> Option<&PossiblyCurrentContext> {
        self.context.as_ref()
    }

    pub fn display(&self) -> Option<&Display> {
        self.display.as_ref()
    }

    pub fn make_current(&self) -> bool {
        match (self.context.as_ref(), self.surface.as_ref()) {
            (Some(context), Some(surface)) => match surface {
                SurfaceHelper::Window(window) => context.make_current(window).is_ok(),
                SurfaceHelper::Pbuffer(buffer) => context.make_current(buffer).is_ok(),
                SurfaceHelper::Pixmap(map) => context.make_current(map).is_ok(),
            },
            _ => false,
        }
    }

    pub fn remove_if_current(&self) {
        if let Some(context) = self.context.as_ref() {}
    }

    pub fn swap_buffers(&self) -> bool {
        match (self.context.as_ref(), self.surface.as_ref()) {
            (Some(context), Some(surface)) => match surface {
                SurfaceHelper::Window(window) => window.swap_buffers(context).is_ok(),
                SurfaceHelper::Pbuffer(buffer) => buffer.swap_buffers(context).is_ok(),
                SurfaceHelper::Pixmap(map) => map.swap_buffers(context).is_ok(),
            },
            _ => false,
        }
    }

    pub fn get_surface_width(&self) -> i32 {
        self.surface
            .as_ref()
            .map(|v| match v {
                SurfaceHelper::Window(window) => window.width().unwrap_or_default() as i32,
                SurfaceHelper::Pbuffer(buffer) => buffer.width().unwrap_or_default() as i32,
                SurfaceHelper::Pixmap(pixmap) => pixmap.width().unwrap_or_default() as i32,
            })
            .unwrap_or_default()
    }

    pub fn get_surface_height(&self) -> i32 {
        self.surface
            .as_ref()
            .map(|v| match v {
                SurfaceHelper::Window(window) => window.height().unwrap_or_default() as i32,
                SurfaceHelper::Pbuffer(buffer) => buffer.height().unwrap_or_default() as i32,
                SurfaceHelper::Pixmap(pixmap) => pixmap.height().unwrap_or_default() as i32,
            })
            .unwrap_or_default()
    }
}


