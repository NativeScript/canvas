use std::os::raw::c_float;

use skia_safe::Color;

use crate::common::context::Context;

impl Context {
    pub fn set_shadow_blur(&mut self, blur: c_float) {
        self.state.shadow_blur = blur;
    }

    pub fn shadow_blur(&self) -> c_float {
        self.state.shadow_blur
    }

    pub fn set_shadow_offset_x(&mut self, x: c_float) {
        self.state.shadow_offset.x = x;
    }

    pub fn shadow_offset_x(&self) -> c_float {
        self.state.shadow_offset.x
    }

    pub fn set_shadow_offset_y(&mut self, y: c_float) {
        self.state.shadow_offset.y = y;
    }

    pub fn shadow_offset_y(&self) -> c_float {
        self.state.shadow_offset.y
    }

    pub fn set_shadow_color(&mut self, color: Color) {
        self.state.shadow_color = color;
    }

    pub fn shadow_color(&self) -> Color {
        self.state.shadow_color
    }
}
