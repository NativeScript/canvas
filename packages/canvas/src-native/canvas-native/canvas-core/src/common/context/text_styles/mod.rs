use crate::{
    common::context::Context,
    common::context::text_styles::text_align::TextAlign,
    common::context::text_styles::text_baseline::TextBaseLine, common::context::text_styles::text_direction::TextDirection,
};
use crate::common::context::drawing_text::typography::parse_font;

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
        if let Ok(font) = parse_font(font, &self.device) {
            self.state.font.font_details = font.0;
        }
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
