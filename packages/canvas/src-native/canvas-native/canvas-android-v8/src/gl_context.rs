use egl::{EGLContext, EGLDisplay, EGLSurface};

pub struct GLContext {
    surface: Option<egl::EGLSurface>,
    context: Option<egl::EGLContext>,
    display: Option<egl::EGLDisplay>,
}

impl GLContext {
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
            self.context.unwrap_or(egl::EGL_NO_CONTEXT),
        )
    }

    pub fn remove_if_current(&self){
        if self.context.is_some() && (egl::get_current_context() == self.context) {
            egl::make_current(
                egl::EGL_NO_DISPLAY,
                egl::EGL_NO_SURFACE,
                egl::EGL_NO_SURFACE,
                egl::EGL_NO_CONTEXT
            )
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
            egl::query_surface(*display, *surface, egl::EGL_WIDTH, &mut width);
            return width;
        }
        0
    }

    pub fn get_surface_height(&self) -> i32 {
        if let (Some(display), Some(surface)) = (self.display(), self.surface()) {
            let mut height = 0;
            egl::query_surface(*display, *surface, egl::EGL_HEIGHT, &mut height);
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
