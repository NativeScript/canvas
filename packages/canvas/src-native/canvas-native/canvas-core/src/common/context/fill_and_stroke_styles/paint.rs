use std::os::raw::c_float;

use skia_safe::{Color, Point};
use skia_safe::paint::{Cap, Style};

use crate::common::context::fill_and_stroke_styles::gradient::Gradient;
use crate::common::context::fill_and_stroke_styles::pattern::Pattern;
use crate::common::context::filter_quality::FilterQuality;
use crate::common::context::image_smoothing::ImageSmoothingQuality;
use crate::common::utils::color::to_parsed_color;

#[derive(Clone)]
pub enum PaintStyle {
    Color(Color),
    Gradient(Gradient),
    Pattern(Pattern),
}

impl PaintStyle {
    pub fn new_color(color: u32) -> Self {
        Self::Color(Color::from(color))
    }

    pub fn new_color_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::Color(Color::from_argb(alpha, red, green, blue))
    }

    pub fn get_parsed_color(&self) -> Option<String> {
        match self {
            PaintStyle::Color(color) => Some(to_parsed_color(*color)),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Paint {
    fill_paint: skia_safe::Paint,
    stroke_paint: skia_safe::Paint,
    image_paint: skia_safe::Paint,
    fill_style: PaintStyle,
    stroke_style: PaintStyle,
    image_smoothing_quality: FilterQuality,
}

impl Paint {
    pub fn image_smoothing_quality_set(&mut self, image_smoothing_quality: FilterQuality) {
        self.image_smoothing_quality = image_smoothing_quality
    }

    fn update_paint_style(&mut self, is_fill: bool) {
        let style;
        if is_fill {
            style = &mut self.fill_style;
        } else {
            style = &mut self.stroke_style;
        }
        match style {
            PaintStyle::Color(color) => {
                if is_fill {
                    self.fill_paint.set_color(*color);
                } else {
                    self.stroke_paint.set_color(*color);
                }
            }
            PaintStyle::Pattern(pattern) => {
                if is_fill {
                    self.fill_paint.set_shader(Pattern::to_pattern_shader(
                        pattern,
                        self.image_smoothing_quality.into(),
                    ));
                } else {
                    self.stroke_paint.set_shader(Pattern::to_pattern_shader(
                        pattern,
                        self.image_smoothing_quality.into(),
                    ));
                }
            }
            PaintStyle::Gradient(gradient) => {
                if is_fill {
                    self.fill_paint.set_shader(Gradient::to_shader(gradient));
                } else {
                    self.stroke_paint.set_shader(Gradient::to_shader(gradient));
                }
            }
        }
    }

    pub fn set_style(&mut self, is_fill: bool, style: PaintStyle) {
        if is_fill {
            self.fill_style = style;
        } else {
            self.stroke_style = style;
        }
        self.update_paint_style(is_fill);
    }

    pub fn style(&mut self, is_fill: bool) -> &PaintStyle {
        if is_fill {
            &self.fill_style
        } else {
            &self.stroke_style
        }
    }

    pub(crate) fn fill_paint(&self) -> &skia_safe::Paint {
        &self.fill_paint
    }

    pub(crate) fn stroke_paint(&self) -> &skia_safe::Paint {
        &self.stroke_paint
    }

    pub(crate) fn image_paint(&self) -> &skia_safe::Paint {
        &self.image_paint
    }

    pub(crate) fn fill_paint_mut(&mut self) -> &mut skia_safe::Paint {
        &mut self.fill_paint
    }

    pub(crate) fn stroke_paint_mut(&mut self) -> &mut skia_safe::Paint {
        &mut self.stroke_paint
    }

    pub(crate) fn image_paint_mut(&mut self) -> &mut skia_safe::Paint {
        &mut self.image_paint
    }

    fn shadow_paint(
        mut paint: skia_safe::Paint,
        offset: Point,
        color: Color,
        blur: c_float,
    ) -> Option<skia_safe::Paint> {
        if color != Color::TRANSPARENT && blur > 0.0 {
            let sigma = blur / 2.0;
            let filter = skia_safe::image_filters::drop_shadow_only(
                offset,
                (sigma, sigma),
                color,
                None,
                None,
            );
            paint.set_image_filter(filter);
            return Some(paint);
        }
        None
    }

    pub fn fill_shadow_paint(
        &self,
        offset: Point,
        color: Color,
        blur: c_float,
    ) -> Option<skia_safe::Paint> {
        let mut paint = self.fill_paint().clone();
        paint.set_color(color);
        Self::shadow_paint(paint, offset, color, blur)
    }

    pub fn stroke_shadow_paint(
        &self,
        offset: Point,
        color: Color,
        blur: c_float,
    ) -> Option<skia_safe::Paint> {
        let mut paint = self.stroke_paint().clone();
        paint.set_color(color);
        Self::shadow_paint(paint, offset, color, blur)
    }
}

impl Default for Paint {
    fn default() -> Self {
        let mut fill_paint = skia_safe::Paint::default();
        fill_paint
            .set_anti_alias(true)
            .set_color(Color::BLACK)
            .set_style(Style::Fill);

        let mut stroke_paint = skia_safe::Paint::default();
        stroke_paint
            .set_anti_alias(true)
            .set_color(Color::BLACK)
            .set_style(Style::Stroke)
            .set_stroke_miter(10.0)
            .set_stroke_width(1.0)
            .set_stroke_cap(Cap::Butt);
        let mut image_paint = skia_safe::Paint::default();
        image_paint.set_anti_alias(true).set_style(Style::Fill);

        Self {
            fill_paint,
            stroke_paint,
            image_paint,
            fill_style: PaintStyle::Color(Color::BLACK),
            stroke_style: PaintStyle::Color(Color::BLACK),
            image_smoothing_quality: ImageSmoothingQuality::default().into(),
        }
    }
}
