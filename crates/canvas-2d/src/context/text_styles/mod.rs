pub use text_direction::*;

use crate::context::drawing_text::typography::Font;
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
        if let Ok(value) = Font::new(font) {
            self.state.font = font.to_owned();
            self.state.font_style = value;
        }
    }

    pub fn font(&self) -> &str {
        self.state.font.as_str()
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

    pub fn set_word_spacing(&mut self, value: &str) {
        let size_regexp = crate::context::drawing_text::typography::FONT_REGEXP
            .get_or_init(crate::context::drawing_text::typography::init_font_regexp);

        if let Some(cap) = size_regexp.captures(value) {
            let size_str = cap.get(7).or_else(|| cap.get(5)).unwrap().as_str();
            let size = if size_str.ends_with('%') {
                size_str
                    .parse::<f32>()
                    .map(|v| v / 100.0 * crate::context::drawing_text::typography::FONT_MEDIUM_PX)
                    .ok()
            } else {
                size_str.parse::<f32>().ok()
            };

            if let Some(size) = size {
                let size_px = crate::context::drawing_text::typography::parse_size_px(
                    size,
                    cap.get(8).map(|m| m.as_str()).unwrap_or("px"),
                );
                self.state.word_spacing_value = value.to_string();
                self.state.word_spacing = size_px;
            };
        }
    }

    pub fn get_word_spacing(&self) -> &str {
        return self.state.word_spacing_value.as_str();
    }
    pub fn set_letter_spacing(&mut self, value: &str) {
        let size_regexp = crate::context::drawing_text::typography::FONT_REGEXP
            .get_or_init(crate::context::drawing_text::typography::init_font_regexp);

        if let Some(cap) = size_regexp.captures(value) {
            let size_str = cap.get(7).or_else(|| cap.get(5)).unwrap().as_str();
            let size = if size_str.ends_with('%') {
                size_str
                    .parse::<f32>()
                    .map(|v| v / 100.0 * crate::context::drawing_text::typography::FONT_MEDIUM_PX)
                    .ok()
            } else {
                size_str.parse::<f32>().ok()
            };

            if let Some(size) = size {
                let size_px = crate::context::drawing_text::typography::parse_size_px(
                    size,
                    cap.get(8).map(|m| m.as_str()).unwrap_or("px"),
                );
                self.state.letter_spacing_value = value.to_string();
                self.state.letter_spacing = size_px;
            };
        }
    }

    pub fn get_letter_spacing(&self) -> &str {
        self.state.letter_spacing_value.as_str()
    }
}
