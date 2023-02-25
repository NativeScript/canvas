pub struct ContextAttributes {
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
    is_canvas: bool,
}

impl Default for ContextAttributes {
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
            is_canvas: false,
        }
    }
}

impl ContextAttributes {
    pub fn new(
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: &str,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
        is_canvas: bool,
    ) -> Self {
        Self {
            alpha,
            antialias,
            depth,
            fail_if_major_performance_caveat,
            power_preference: power_preference.to_string(),
            premultiplied_alpha,
            preserve_drawing_buffer,
            stencil,
            desynchronized,
            xr_compatible,
            is_canvas,
        }
    }

    pub fn get_is_canvas(&self) -> bool {
        self.is_canvas
    }
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
    pub fn get_power_preference(&self) -> String {
        self.power_preference.clone()
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

    pub fn set_alpha(&mut self, value: bool) {
        self.alpha = value;
    }
    pub fn set_antialias(&mut self, value: bool) {
        self.antialias = value;
    }
    pub fn set_depth(&mut self, value: bool) {
        self.depth = value;
    }
    pub fn set_fail_if_major_performance_caveat(&mut self, value: bool) {
        self.fail_if_major_performance_caveat = value;
    }

    pub fn set_power_preference(&mut self, value: &str) {
        self.power_preference = value.to_string();
    }

    pub fn set_premultiplied_alpha(&mut self, value: bool) {
        self.premultiplied_alpha = value;
    }

    pub fn set_preserve_drawing_buffer(&mut self, value: bool) {
        self.preserve_drawing_buffer = value;
    }
    pub fn set_stencil(&mut self, value: bool) {
        self.stencil = value;
    }

    pub fn set_desynchronized(&mut self, value: bool) {
        self.desynchronized = value;
    }

    pub fn set_xr_compatible(&mut self, value: bool) {
        self.xr_compatible = value;
    }
}
