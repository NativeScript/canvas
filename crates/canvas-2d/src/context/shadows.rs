use std::os::raw::c_float;

use skia_safe::Color;

use crate::context::Context;
use crate::utils::color::parse_color;

impl Context {
    pub fn set_shadow_blur(&mut self, blur: c_float) {
        // TODO ?
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

    pub fn set_shadow_color_str(&mut self, color: &str) {
        if let Some(color) = parse_color(color) {
            self.state.shadow_color = color;
        }
    }

    pub fn set_shadow_color_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.state.shadow_color = Color::from_argb(a, r, g, b);
    }

    pub fn shadow_color(&self) -> Color {
        self.state.shadow_color
    }

    pub fn shadow_color_rgba(&self, r: &mut u8, g: &mut u8, b: &mut u8, a: &mut u8) {
        let color = self.state.shadow_color;
        *r = color.r();
        *g = color.g();
        *b = color.b();
        *a = color.a();
    }
}
