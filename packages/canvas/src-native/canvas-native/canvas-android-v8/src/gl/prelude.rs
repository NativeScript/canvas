use crate::gl_context::GLContext;

pub struct WebGLState {
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: String,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    gl_context: GLContext,
    clear_stencil: i32,
    clear_color: [f32; 4],
    scissor_enabled: bool,
    clear_depth: f32,
    color_mask: [bool; 4],
    stencil_mask: u32,
    stencil_mask_back: u32,
    stencil_func_ref: i32,
    stencil_func_ref_back: i32,
    stencil_func_mask: u32,
    stencil_func_mask_back: u32,
    depth_mask: bool,
}

impl WebGLState {
    pub fn get_alpha(&self) -> bool {
        self.alpha
    }
    pub fn get_antialias(&self) -> bool {
        self.antialias
    }
    pub fn get_depth(&self) -> bool {
        self.depth
    }
    pub fn get_fail_if_major_performance_caveat(&self) -> bool {
        self.fail_if_major_performance_caveat
    }
    pub fn get_power_preference(&self) -> &str {
        self.power_preference.as_str()
    }
    pub fn get_premultiplied_alpha(&self) -> bool {
        self.premultiplied_alpha
    }

    pub fn get_preserve_drawing_buffer(&self) -> bool {
        self.preserve_drawing_buffer
    }
    pub fn get_stencil(&self) -> bool {
        self.stencil
    }
    pub fn get_desynchronized(&self) -> bool {
        self.desynchronized
    }
    pub fn get_xr_compatible(&self) -> bool {
        self.xr_compatible
    }

    pub fn drawing_buffer_width(&self) -> i32 {
        self.gl_context.get_surface_width()
    }

    pub fn drawing_buffer_height(&self) -> i32 {
        self.gl_context.get_surface_height()
    }

    pub fn make_current(&self) -> bool {
        self.gl_context.make_current()
    }

    pub fn swap_buffers(&self)-> bool {
        self.gl_context.swap_buffers()
    }
}

impl Default for WebGLState {
    fn default() -> Self {
        Self {
            alpha: true,
            antialias: true,
            depth: true,
            fail_if_major_performance_caveat: false,
            power_preference: "default".to_string(),
            premultiplied_alpha: true,
            preserve_drawing_buffer: false,
            stencil: false,
            desynchronized: false,
            xr_compatible: false,
            gl_context: Default::default(),
            clear_stencil: 0,
            clear_color: [0., 0., 0., 0.],
            scissor_enabled: false,
            clear_depth: 1.,
            color_mask: [true, true, true, true],
            stencil_mask: 0xFFFFFFFF,
            stencil_mask_back: 0xFFFFFFFF,
            stencil_func_ref: 0,
            stencil_func_ref_back: 0,
            stencil_func_mask: 0xFFFFFFFF,
            stencil_func_mask_back: 0xFFFFFFFF,
            depth_mask: true,
        }
    }
}
