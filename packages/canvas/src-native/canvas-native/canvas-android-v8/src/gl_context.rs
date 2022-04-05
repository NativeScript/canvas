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
}
