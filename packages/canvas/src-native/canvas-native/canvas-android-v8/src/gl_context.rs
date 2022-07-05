use std::fmt::format;
use std::os::raw::c_void;

use egl::{
    EGLBoolean, EGLConfig, EGLContext, EGLDisplay, EGLNativeWindowType, EGLSurface, EGLint,
    EGL_ALPHA_SIZE, EGL_BLUE_SIZE, EGL_BUFFER_PRESERVED, EGL_CONTEXT_CLIENT_VERSION,
    EGL_DEFAULT_DISPLAY, EGL_DEPTH_SIZE, EGL_EXTENSIONS, EGL_GREEN_SIZE, EGL_HEIGHT, EGL_NONE,
    EGL_NO_CONTEXT, EGL_OPENGL_ES2_BIT, EGL_OPENGL_ES_API, EGL_OPENGL_ES_BIT, EGL_PBUFFER_BIT,
    EGL_PIXMAP_BIT, EGL_RED_SIZE, EGL_RENDERABLE_TYPE, EGL_SAMPLES, EGL_SAMPLE_BUFFERS,
    EGL_STENCIL_SIZE, EGL_SURFACE_TYPE, EGL_SWAP_BEHAVIOR, EGL_TRUE, EGL_VERSION,
    EGL_VG_ALPHA_FORMAT_PRE_BIT, EGL_WIDTH, EGL_WINDOW_BIT,
};

use crate::bridges::context::console_log;
use crate::gl::prelude::ContextAttributes;

const EGL_CONTEXT_MINOR_VERSION: i32 = 0x000030fb;

#[derive(Clone, Debug)]
pub struct GLContext {
    surface: Option<EGLSurface>,
    context: Option<EGLContext>,
    display: Option<EGLDisplay>,
}

const EGL_CONTEXT_CLIENT_MINOR_VERSION: EGLint = 0x30FB;

extern "C" {
    pub fn eglChooseConfig(
        dpy: EGLDisplay,
        attrib_list: *const EGLint,
        configs: *mut EGLConfig,
        config_size: EGLint,
        num_config: *mut EGLint,
    ) -> EGLBoolean;

    pub fn eglCreateContext(
        dpy: EGLDisplay,
        config: EGLConfig,
        share_context: EGLContext,
        attrib_list: *const EGLint,
    ) -> EGLContext;
}

impl GLContext {
    pub fn num_filtered_configs(display: EGLDisplay, attrib_list: &[EGLint]) -> i32 {
        let mut count: i32 = 0;
        unsafe {
            eglChooseConfig(
                display,
                attrib_list.as_ptr(),
                std::ptr::null_mut(),
                0,
                &mut count,
            );
        }
        return count;
    }

    pub fn get_filtered_configs(
        display: EGLDisplay,
        attrib_list: &[EGLint],
        configs: &mut [EGLConfig],
    ) -> i32 {
        let mut count: i32 = 0;
        unsafe {
            eglChooseConfig(
                display,
                attrib_list.as_ptr(),
                std::mem::transmute(configs.as_mut_ptr()),
                configs.len() as i32,
                &mut count,
            );
        };
        count
    }

    fn get_config(
        surface_type: i32,
        config: &mut ContextAttributes,
    ) -> Option<(EGLDisplay, EGLConfig, EGLContext)> {
        if surface_type < 0 {
            return None;
        }
        let display = egl::get_display(EGL_DEFAULT_DISPLAY);

        if display.is_none() {
            return None;
        }
        let mut major_version = 0;
        let mut minor_version = 0;
        let display = display.unwrap();

        if !egl::initialize(display, &mut major_version, &mut minor_version) {
            return None;
        }

        // Find a compatible EGLConfig
        let configs_count = 1;
        let mut type_ = EGL_OPENGL_ES2_BIT;
        let mut depth_size = 16;
        let mut stencil_size = 0;
        let mut alpha = 8;
        let mut use_alpha = true;

        let has_gl2support = GLContext::has_gl2support();

        if surface_type == 2 && has_gl2support {
            type_ = EGL_VG_ALPHA_FORMAT_PRE_BIT;
        }
        if config.get_stencil() && !config.get_is_canvas() {
            stencil_size = 8;
        }
        if !config.get_depth() || config.get_is_canvas() {
            config.set_depth(false);
            depth_size = 0
        }

        use_alpha = config.get_alpha();

        let mut antialias = if config.get_is_canvas() {
            config.set_antialias(false);
            false
        } else {
            config.get_antialias()
        };
        if !use_alpha {
            alpha = 0;
        }

        let mut config_spec: Vec<EGLint> =
            vec![EGL_RED_SIZE, 5, EGL_GREEN_SIZE, 6, EGL_BLUE_SIZE, 5];

        if stencil_size > 0 {
            config_spec.extend([EGL_STENCIL_SIZE, stencil_size]);
        }

        if use_alpha {
            config_spec.extend([EGL_ALPHA_SIZE, alpha]);
        }

        if depth_size > 0 {
            config_spec.extend([EGL_DEPTH_SIZE, depth_size])
        }

        if antialias {
            config_spec.extend([EGL_SAMPLE_BUFFERS, 1, EGL_SAMPLES, 4, EGL_NONE]);
        } else {
            config_spec.extend([EGL_NONE]);
        }

        /* can be used to improve selection

        let count = GLContext::num_filtered_configs(display, config_spec.as_slice());

        let mut configs = vec![0 as EGLConfig; count as usize];

        GLContext::get_filtered_configs(display, config_spec.as_slice(), configs.as_mut_slice());

        for cfg in configs.iter() {
            let mut type_ = 0i32;
            let mut red = 0i32;
            let mut green = 0i32;
            let mut blue = 0i32;
            let mut alpha = 0i32;
            let mut depth = 0i32;
            let mut stencil = 0i32;

            egl::get_config_attrib(display, *cfg, EGL_RENDERABLE_TYPE, &mut type_);

            egl::get_config_attrib(display, *cfg, EGL_RED_SIZE, &mut red);

            egl::get_config_attrib(display, *cfg, EGL_GREEN_SIZE, &mut green);

            egl::get_config_attrib(display, *cfg, EGL_BLUE_SIZE, &mut blue);

            egl::get_config_attrib(display, *cfg, EGL_ALPHA_SIZE, &mut alpha);

            egl::get_config_attrib(display, *cfg, EGL_DEPTH_SIZE, &mut depth);

            egl::get_config_attrib(display, *cfg, EGL_STENCIL_SIZE, &mut stencil);
        }

        */

        let mut egl_config = egl::choose_config(display, config_spec.as_slice(), 1);

        if egl_config.is_none() {
            if antialias {
                config.set_antialias(false);
                antialias = false;

                let configSpec = [
                    EGL_RENDERABLE_TYPE,
                    type_,
                    EGL_RED_SIZE,
                    8,
                    EGL_GREEN_SIZE,
                    8,
                    EGL_BLUE_SIZE,
                    8,
                    EGL_ALPHA_SIZE,
                    alpha,
                    EGL_DEPTH_SIZE,
                    depth_size,
                    EGL_STENCIL_SIZE,
                    stencil_size,
                    EGL_NONE,
                ];

                egl_config = egl::choose_config(display, &configSpec, 1);
            } else {
                return None;
            }
        }

        if egl_config.is_none() {
            return None;
        }

        let egl_config = egl_config.unwrap();

        egl::bind_api(EGL_OPENGL_ES_API);

        let mut egl_context: Option<EGLContext> = None;

        if surface_type == 2 && has_gl2support {
            // try for gl 3.1
            if crate::utils::get_sdk_version() >= 21 {
                if egl_context.is_none() {
                    egl_context = GLContext::create_context(display, egl_config, 3, 1);
                }
            }

            // try for gl 3.0
            match egl_context.as_ref() {
                Some(context) => {
                    if context.is_null() {
                        egl_context = GLContext::create_context(display, egl_config, 3, 0);
                    }
                }
                None => {
                    egl_context = GLContext::create_context(display, egl_config, 3, 0);
                }
            }
        }

        // try for gl 2.0
        match egl_context.as_ref() {
            Some(context) => {
                if context.is_null() {
                    egl_context = GLContext::create_context(display, egl_config, 2, 0);
                }
            }
            None => {
                egl_context = GLContext::create_context(display, egl_config, 2, 0);
            }
        }

        if egl_context.is_none() {
            return None;
        }

        let egl_context = egl_context.unwrap();

        Some((display, egl_config, egl_context))
    }

    fn create_context(
        display: EGLDisplay,
        egl_config: EGLConfig,
        context_version: i32,
        minor_version: i32,
    ) -> Option<EGLContext> {
        let attrs = [
            EGL_CONTEXT_CLIENT_VERSION,
            context_version,
            EGL_CONTEXT_MINOR_VERSION,
            minor_version,
            EGL_NONE,
        ];

        egl::create_context(display, egl_config, EGL_NO_CONTEXT, &attrs)
    }

    fn create_surface(
        surface_type: i32,
        config: &mut ContextAttributes,
        width: i32,
        height: i32,
        window: *mut c_void,
    ) -> Option<(EGLDisplay, EGLConfig, EGLContext, EGLSurface)> {
        match GLContext::get_config(surface_type, config) {
            Some((display, egl_config, context)) => {
                let mut egl_surface: Option<EGLSurface> = None;

                if window.is_null() {
                    let mut width = width;
                    let mut height = height;

                    if width == 0 {
                        width = 1
                    }
                    if height == 0 {
                        height = 1
                    }
                    let attrs = [EGL_WIDTH, width, EGL_HEIGHT, height, EGL_NONE];

                    egl_surface = egl::create_pbuffer_surface(display, egl_config, &attrs);
                } else {
                    let attrs = [EGL_NONE];
                    egl_surface = egl::create_window_surface(
                        display,
                        egl_config,
                        window as EGLNativeWindowType,
                        &attrs,
                    );
                }

                if egl_surface.is_none() {
                    return None;
                }

                let egl_surface = egl_surface.unwrap();
                egl::make_current(display, egl_surface, egl_surface, context);

                if config.get_preserve_drawing_buffer() {
                    let did_enable_buffer_preservation = egl::surface_attrib(
                        display,
                        egl_surface,
                        EGL_SWAP_BEHAVIOR,
                        EGL_BUFFER_PRESERVED,
                    );
                    if !did_enable_buffer_preservation {
                        config.set_preserve_drawing_buffer(false);
                    }
                }

                unsafe { gl_bindings::glClearColor(0., 0., 0., 0.) }
                let mut bit = gl_bindings::GL_COLOR_BUFFER_BIT;
                if config.get_depth() {
                    unsafe { gl_bindings::glClearDepthf(1.) }
                    bit = bit | gl_bindings::GL_DEPTH_BUFFER_BIT;
                }
                if config.get_stencil() {
                    unsafe { gl_bindings::glClearStencil(0) }
                    bit = bit | gl_bindings::GL_STENCIL_BUFFER_BIT;
                }
                unsafe { gl_bindings::glClear(bit) }

                egl::swap_buffers(display, egl_surface);

                Some((display, egl_config, context, egl_surface))
            }
            None => None,
        }
    }

    fn has_extension(extensions: &str, name: &str) -> bool {
        extensions
            .split(" ")
            .into_iter()
            .find(|s| *s == name)
            .is_none()
    }

    fn has_gl2support() -> bool {
        let mut highest_es_version = 0;

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
    }

    pub fn new_with_no_window(
        width: i32,
        height: i32,
        surface_type: i32,
        config: &mut ContextAttributes,
    ) -> Self {
        match GLContext::create_surface(surface_type, config, width, height, std::ptr::null_mut()) {
            Some((display, _, context, surface)) => Self {
                surface: Some(surface),
                context: Some(context),
                display: Some(display),
            },
            None => Self::default(),
        }
    }

    pub fn get_current() -> Self {
        Self {
            surface: egl::get_current_surface(egl::EGL_DRAW),
            context: egl::get_current_context(),
            display: egl::get_current_display(),
        }
    }

    pub fn surface(&self) -> Option<&EGLSurface> {
        self.surface.as_ref()
    }

    pub fn context(&self) -> Option<&EGLContext> {
        self.context.as_ref()
    }

    pub fn display(&self) -> Option<&EGLDisplay> {
        self.display.as_ref()
    }

    pub fn make_current(&self) -> bool {
        egl::make_current(
            self.display.unwrap_or(egl::EGL_NO_DISPLAY),
            self.surface.unwrap_or(egl::EGL_NO_SURFACE),
            self.surface.unwrap_or(egl::EGL_NO_SURFACE),
            self.context.unwrap_or(EGL_NO_CONTEXT),
        )
    }

    pub fn remove_if_current(&self) {
        if self.context.is_some() && (egl::get_current_context() == self.context) {
            egl::make_current(
                egl::EGL_NO_DISPLAY,
                egl::EGL_NO_SURFACE,
                egl::EGL_NO_SURFACE,
                egl::EGL_NO_CONTEXT,
            );
        }
    }

    pub fn swap_buffers(&self) -> bool {
        egl::swap_buffers(
            self.display.unwrap_or(egl::EGL_NO_DISPLAY),
            self.surface.unwrap_or(egl::EGL_NO_SURFACE),
        )
    }

    pub fn get_surface_width(&self) -> i32 {
        if let (Some(display), Some(surface)) = (self.display(), self.surface()) {
            let mut width = 0;
            egl::query_surface(*display, *surface, EGL_WIDTH, &mut width);
            return width;
        }
        0
    }

    pub fn get_surface_height(&self) -> i32 {
        if let (Some(display), Some(surface)) = (self.display(), self.surface()) {
            let mut height = 0;
            egl::query_surface(*display, *surface, EGL_HEIGHT, &mut height);
            return height;
        }
        0
    }
}

impl Default for GLContext {
    fn default() -> Self {
        Self {
            surface: None,
            context: None,
            display: None,
        }
    }
}
