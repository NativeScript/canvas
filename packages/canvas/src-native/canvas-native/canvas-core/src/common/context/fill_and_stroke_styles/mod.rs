use skia_safe::Color;

use crate::common::context::Context;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;

pub mod gradient;
pub mod paint;
pub mod pattern;

impl Context {
    pub fn set_fill_style(&mut self, style: PaintStyle) {
        self.state.paint.set_style(true, style)
    }

    pub fn fill_style(&mut self) -> &PaintStyle {
        self.state.paint.style(true)
    }

    pub fn set_stroke_style(&mut self, style: PaintStyle) {
        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
        self.state.paint.set_style(false, style)
    }

    pub fn stroke_style(&mut self) -> &PaintStyle {
        self.state.paint.style(false)
    }
}
