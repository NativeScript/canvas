use std::os::raw::c_float;

use skia_safe::Color;

use crate::context::Context;

impl Context {
    #[inline(always)]
    pub fn set_shadow_blur(&mut self, blur: c_float) {
        // TODO ?
        self.state.shadow_blur = blur;
    }

    #[inline(always)]
    pub fn shadow_blur(&self) -> c_float {
        self.state.shadow_blur
    }

    #[inline(always)]
    pub fn set_shadow_offset_x(&mut self, x: c_float) {
        self.state.shadow_offset.x = x;
    }

    #[inline(always)]
    pub fn shadow_offset_x(&self) -> c_float {
        self.state.shadow_offset.x
    }

    #[inline(always)]
    pub fn set_shadow_offset_y(&mut self, y: c_float) {
        self.state.shadow_offset.y = y;
    }

    #[inline(always)]
    pub fn shadow_offset_y(&self) -> c_float {
        self.state.shadow_offset.y
    }

    #[inline(always)]
    pub fn set_shadow_color(&mut self, color: Color) {
        self.state.shadow_color = color;
    }

    #[inline(always)]
    pub fn set_shadow_color_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.state.shadow_color = Color::from_argb(a, r, g, b);
    }

    #[inline(always)]
    pub fn shadow_color(&self) -> Color {
        self.state.shadow_color
    }

    #[inline(always)]
    pub fn shadow_color_rgba(&self, r: &mut u8, g: &mut u8, b: &mut u8, a: &mut u8) {
        let color = self.state.shadow_color;
        *r = color.r();
        *g = color.g();
        *b = color.b();
        *a = color.a();
    }
}
