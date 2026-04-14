#![allow(non_upper_case_globals)]

use crate::context::fill_and_stroke_styles::gradient::Gradient;
use crate::context::fill_and_stroke_styles::pattern::Pattern;
use crate::context::filter_quality::FilterQuality;
use crate::context::image_smoothing::ImageSmoothingQuality;
use crate::context::Context;
use crate::utils::color::{to_parsed_color, to_parsed_color_4f};
use canvas_core::context_attributes::ColorSpace;
use csscolorparser::parse;
use regex_lite::Regex;
use skia_safe::paint::{Cap, Style};
pub use skia_safe::Color;
use skia_safe::{BlendMode, Color4f, Point};
use std::os::raw::c_float;
use std::sync::OnceLock;

pub(crate) static COLOR_P3_REGEXP: OnceLock<Regex> = OnceLock::new();

#[derive(Clone)]
pub enum PaintStyle {
    Color(Color),
    Gradient(Gradient),
    Pattern(Pattern),
    Color4f(Color4f),
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

    #[inline]
    pub fn new_color_str(color: &str) -> Option<Self> {
        // Fast path for common named colors and hex patterns to avoid csscolorparser overhead
        match color {
            "black" | "#000" | "#000000" => return Some(Self::Color(Color::BLACK)),
            "white" | "#fff" | "#FFF" | "#ffffff" | "#FFFFFF" => return Some(Self::Color(Color::WHITE)),
            "red" | "#f00" | "#ff0000" | "#FF0000" => return Some(Self::Color(Color::RED)),
            "green" | "#008000" => return Some(Self::Color(Color::from_argb(255, 0, 128, 0))),
            "blue" | "#00f" | "#0000ff" | "#0000FF" => return Some(Self::Color(Color::BLUE)),
            "transparent" => return Some(Self::Color(Color::TRANSPARENT)),
            _ => {}
        }
        // Fast path for #RRGGBB hex (most common programmatic format)
        if color.len() == 7 && color.as_bytes()[0] == b'#' {
            if let (Ok(r), Ok(g), Ok(b_val)) = (
                u8::from_str_radix(&color[1..3], 16),
                u8::from_str_radix(&color[3..5], 16),
                u8::from_str_radix(&color[5..7], 16),
            ) {
                return Some(Self::Color(Color::from_argb(255, r, g, b_val)));
            }
        }
        // Fast path for rgb(r,g,b) and rgba(r,g,b,a) — the most common programmatic formats
        if let Some(parsed) = Self::parse_rgb_rgba_fast(color) {
            return Some(parsed);
        }
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

    /// Fast inline parser for `rgb(r,g,b)` and `rgba(r,g,b,a)` strings.
    /// Avoids the full csscolorparser overhead for the most common programmatic color format.
    #[inline]
    fn parse_rgb_rgba_fast(color: &str) -> Option<Self> {
        let bytes = color.as_bytes();
        let len = bytes.len();
        // Minimum: "rgb(0,0,0)" = 10 chars
        if len < 10 {
            return None;
        }
        let is_rgba = bytes[0] == b'r' && bytes[1] == b'g' && bytes[2] == b'b' && bytes[3] == b'a' && bytes[4] == b'(';
        let is_rgb = !is_rgba && bytes[0] == b'r' && bytes[1] == b'g' && bytes[2] == b'b' && bytes[3] == b'(';
        if !is_rgba && !is_rgb {
            return None;
        }
        // Must end with ')'
        if bytes[len - 1] != b')' {
            return None;
        }
        let start = if is_rgba { 5 } else { 4 };
        let inner = &color[start..len - 1];

        let mut parts = inner.splitn(4, ',');
        let r: u8 = parts.next()?.trim().parse().ok()?;
        let g: u8 = parts.next()?.trim().parse().ok()?;
        let b: u8 = parts.next()?.trim().parse().ok()?;
        let a = if is_rgba {
            let a_str = parts.next()?.trim();
            let a_f: f32 = a_str.parse().ok()?;
            (a_f.clamp(0.0, 1.0) * 255.0) as u8
        } else {
            255
        };
        Some(Self::Color(Color::from_argb(a, r, g, b)))
    }

    pub fn new_color(color: u32) -> Self {
        Self::Color(Color::from(color))
    }

    pub fn new_color_rgba(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        Self::Color(Color::from_argb(alpha, r, g, b))
    }

    pub fn new_color_rgba_4f(r: f32, g: f32, b: f32, alpha: f32) -> Self {
        Self::Color4f(Color4f { r, g, b, a: alpha })
    }

    pub fn get_parsed_color(&self) -> Option<String> {
        match self {
            PaintStyle::Color(color) => Some(to_parsed_color(*color)),
            PaintStyle::Color4f(color) => Some(to_parsed_color_4f(*color)),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Paint {
    pub(crate) fill_paint: skia_safe::Paint,
    pub(crate) stroke_paint: skia_safe::Paint,
    pub(crate) image_paint: skia_safe::Paint,
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
                    self.fill_paint.set_shader(None);
                    self.fill_paint.set_color(*color);
                } else {
                    self.stroke_paint.set_shader(None);
                    self.stroke_paint.set_color(*color);
                }
            }
            PaintStyle::Color4f(color) => {
                // only the p3 is colorSpace is currently supported on the web
                let color_space: Option<skia_safe::ColorSpace> = ColorSpace::P3.into();
                if is_fill {
                    self.fill_paint.set_shader(None);
                    self.fill_paint.set_color4f(*color, color_space.as_ref());
                } else {
                    self.stroke_paint.set_shader(None);
                    self.stroke_paint.set_color4f(*color, color_space.as_ref());
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
        let filter = skia_safe::image_filters::drop_shadow_only(
            offset,
            (sigma, sigma),
            color,
            None,
            None,
            None,
        );
        paint.set_image_filter(filter);
        Some(paint)
    }

    #[inline]
    pub fn fill_shadow_paint(
        &self,
        offset: Point,
        color: Color,
        blur: c_float,
    ) -> Option<skia_safe::Paint> {
        // Fast path: skip shadow allocation entirely when no shadow is configured
        if color == Color::TRANSPARENT || (blur == 0.0 && offset.x == 0.0 && offset.y == 0.0) {
            return None;
        }
        let mut paint = self.fill_paint().clone();
        paint.set_color(color);
        Self::shadow_paint(paint, offset, color, blur)
    }

    #[inline]
    pub fn stroke_shadow_paint(
        &self,
        offset: Point,
        color: Color,
        blur: c_float,
    ) -> Option<skia_safe::Paint> {
        // Fast path: skip shadow allocation entirely when no shadow is configured
        if color == Color::TRANSPARENT || (blur == 0.0 && offset.x == 0.0 && offset.y == 0.0) {
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
        let color = color.to_rgba8();
        let style = PaintStyle::Color(Color::from_argb(color[3], color[0], color[1], color[2]));
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

pub fn paint_style_set_color_with_rgba_4f(
    context: &mut Context,
    is_fill: bool,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    let style = PaintStyle::Color4f(Color4f { r, g, b, a });
    if is_fill {
        context.set_fill_style(style);
    } else {
        context.set_stroke_style(style);
    }
}
