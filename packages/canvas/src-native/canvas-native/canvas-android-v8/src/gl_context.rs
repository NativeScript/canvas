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

    pub fn swap_buffers(&self) -> bool {
        egl::swap_buffers(self.display.unwrap_or(egl::EGL_NO_DISPLAY),self.surface.unwrap_or(egl::EGL_NO_SURFACE))
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
