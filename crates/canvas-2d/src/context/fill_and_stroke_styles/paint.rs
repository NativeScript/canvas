#![allow(non_upper_case_globals)]

use std::os::raw::c_float;

use csscolorparser::parse;
use skia_safe::{BlendMode, Point};
pub use skia_safe::Color;
use skia_safe::paint::{Cap, Style};

use crate::context::Context;
use crate::context::fill_and_stroke_styles::gradient::Gradient;
use crate::context::fill_and_stroke_styles::pattern::Pattern;
use crate::context::filter_quality::FilterQuality;
use crate::context::image_smoothing::ImageSmoothingQuality;
use crate::utils::color::to_parsed_color;

#[derive(Clone)]
pub enum PaintStyle {
    Color(Color),
    Gradient(Gradient),
    Pattern(Pattern),
}

const black: PaintStyle = PaintStyle::Color(Color::BLACK);
const white: PaintStyle = PaintStyle::Color(Color::WHITE);
const red: PaintStyle = PaintStyle::Color(Color::RED);
const green: PaintStyle = PaintStyle::Color(Color::GREEN);
const blue: PaintStyle = PaintStyle::Color(Color::BLUE);
const transparent: PaintStyle = PaintStyle::Color(Color::TRANSPARENT);

impl PaintStyle {
    pub const fn transparent() -> Self {
        transparent
    }
    pub const fn black() -> Self {
        black
    }

    pub const fn white() -> Self {
        white
    }

    pub const fn red() -> Self {
        red
    }
    pub const fn green() -> Self {
        green
    }

    pub const fn blue() -> Self {
        blue
    }

    pub fn new_color_str(color: &str) -> Option<Self> {
        parse(color)
            .map(|color| {
                Self::Color(Color::from_argb(
                    (color.a * 255.) as u8,
                    (color.r * 255.) as u8,
                    (color.g * 255.) as u8,
                    (color.b * 255.) as u8,
                ))
            })
            .ok()
    }

    pub fn new_color(color: u32) -> Self {
        Self::Color(Color::from(color))
    }

    pub fn new_color_rgba(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        Self::Color(Color::from_argb(alpha, r, g, b))
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
                self.fill_paint.set_shader(None);
                if is_fill {
                    self.fill_paint.set_color(*color);
                } else {
                    self.stroke_paint.set_color(*color);
                }
            }
            PaintStyle::Pattern(pattern) => {
                if is_fill {
                    self.fill_paint.set_shader(Pattern::pattern_shader(
                        pattern,
                        self.image_smoothing_quality,
                    ));
                } else {
                    self.stroke_paint.set_shader(Pattern::pattern_shader(
                        pattern,
                        self.image_smoothing_quality,
                    ));
                }
            }
            PaintStyle::Gradient(gradient) => {
                if is_fill {
                    self.fill_paint.set_shader(Gradient::shader(gradient));
                } else {
                    self.stroke_paint.set_shader(Gradient::shader(gradient));
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

    pub fn style(&self, is_fill: bool) -> &PaintStyle {
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

    pub(crate) fn set_blend_mode(&mut self, mode: BlendMode) {
        self.fill_paint.set_blend_mode(mode);
        self.stroke_paint.set_blend_mode(mode);
        self.image_paint.set_blend_mode(mode);
    }

    fn shadow_paint(
        mut paint: skia_safe::Paint,
        offset: Point,
        color: Color,
        blur: c_float,
    ) -> Option<skia_safe::Paint> {
        let sigma = blur / 2.0;
        let filter =
            skia_safe::image_filters::drop_shadow_only(offset, (sigma, sigma), color, None, None);
        paint.set_image_filter(filter);
        return Some(paint);
    }

    pub fn fill_shadow_paint(
        &self,
        offset: Point,
        color: Color,
        blur: c_float,
    ) -> Option<skia_safe::Paint> {
        if !(color != Color::TRANSPARENT && blur > 0.0) {
            return None;
        }
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
        if !(color != Color::TRANSPARENT && blur > 0.0) {
            return None;
        }
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
            .set_style(Style::Fill)
            .set_blend_mode(BlendMode::SrcOver);

        let mut stroke_paint = skia_safe::Paint::default();
        stroke_paint
            .set_anti_alias(true)
            .set_color(Color::BLACK)
            .set_style(Style::Stroke)
            .set_stroke_miter(10.0)
            .set_stroke_width(1.0)
            .set_stroke_cap(Cap::Butt)
            .set_blend_mode(BlendMode::SrcOver);
        let mut image_paint = skia_safe::Paint::default();
        image_paint
            .set_anti_alias(true)
            .set_style(Style::Fill)
            .set_blend_mode(BlendMode::SrcOver);

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

pub fn paint_style_set_color_with_string(context: &mut Context, is_fill: bool, color: &str) {
    if let Ok(color) = color.parse::<csscolorparser::Color>() {
        let color = color.rgba_u8();
        let style = PaintStyle::Color(Color::from_argb(color.3, color.0, color.1, color.2));
        if is_fill {
            context.set_fill_style(style);
        } else {
            context.set_stroke_style(style);
        }
    }
}

pub fn paint_style_set_color_with_rgba(
    context: &mut Context,
    is_fill: bool,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    let style = PaintStyle::Color(Color::from_argb(a, r, g, b));
    if is_fill {
        context.set_fill_style(style);
    } else {
        context.set_stroke_style(style);
    }
}
