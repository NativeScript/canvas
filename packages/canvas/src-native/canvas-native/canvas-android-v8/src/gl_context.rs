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
}

impl Default  for GLContext{
    fn default() -> Self {
      Self{
          surface: None,
          context: None,
          display: None
      }
    }
}
