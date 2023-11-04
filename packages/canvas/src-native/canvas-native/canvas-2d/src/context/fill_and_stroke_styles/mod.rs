use crate::{context::fill_and_stroke_styles::paint::PaintStyle, context::Context};

pub mod gradient;
pub mod paint;
pub mod pattern;

impl Context {
    #[inline]
    pub fn set_fill_style_with_color(&mut self, color: &str) {
        if let Some(style) = PaintStyle::new_color_str(color) {
            self.state.paint.set_style(true, style)
        }
    }

    #[inline]
    pub fn set_stroke_style_with_color(&mut self, color: &str) {
        if let Some(style) = PaintStyle::new_color_str(color) {
            self.state.paint.set_style(false, style)
        }
    }

    #[inline]
    pub fn set_fill_style(&mut self, style: PaintStyle) {
        self.state.paint.set_style(true, style)
    }

    #[inline]
    pub fn fill_style(&self) -> &PaintStyle {
        self.state.paint.style(true)
    }

    #[inline]
    pub fn set_stroke_style(&mut self, style: PaintStyle) {
        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
        self.state.paint.set_style(false, style)
    }

    #[inline]
    pub fn stroke_style(&self) -> &PaintStyle {
        self.state.paint.style(false)
    }
}
