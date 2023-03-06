pub use text_direction::*;

use crate::{
    context::text_styles::text_align::TextAlign, context::text_styles::text_baseline::TextBaseLine,
    context::text_styles::text_direction::TextDirection, context::Context,
};

pub mod text_align;
pub mod text_baseline;
pub mod text_direction;

impl Context {
    pub fn set_direction(&mut self, direction: TextDirection) {
        self.state.direction = direction;
    }

    pub fn direction(&self) -> TextDirection {
        self.state.direction
    }

    pub fn set_font(&mut self, font: &str) {
        self.state.font.set_font(font);
    }

    pub fn font(&self) -> &str {
        self.state.font.get_font_details()
    }

    pub fn set_text_align(&mut self, align: TextAlign) {
        self.state.text_align = align;
    }

    pub fn text_align(&self) -> TextAlign {
        self.state.text_align
    }

    pub fn set_text_baseline(&mut self, base: TextBaseLine) {
        self.state.text_baseline = base;
    }

    pub fn text_baseline(&self) -> TextBaseLine {
        self.state.text_baseline
    }
}
