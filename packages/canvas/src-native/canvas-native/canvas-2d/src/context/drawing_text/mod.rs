use log::{log, Level};
use skia_safe::{Canvas, Paint};
use std::ops::Range;
use std::os::raw::c_float;

use crate::context::drawing_text::global_fonts::FONT_LIBRARY;
use crate::context::drawing_text::text_metrics::TextMetrics;
use crate::context::text_styles::text_align::TextAlign;
use crate::context::text_styles::text_baseline::TextBaseLine;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::Context;

pub(crate) const MAX_TEXT_WIDTH: f32 = 100_000.0;

pub mod global_fonts;
pub mod text_metrics;
pub(crate) mod typography;

const HANGING_AS_PERCENT_OF_ASCENT: f32 = 80.;

impl Context {
    pub fn fill_text(&mut self, text: &str, x: c_float, y: c_float, width: Option<c_float>) {
        let width = width.unwrap_or(MAX_TEXT_WIDTH);
        let paint = self.state.paint.fill_paint().clone();
        let shadow_paint = self.state.paint.fill_shadow_paint(
            (0., 0.).into(),
            self.state.shadow_color,
            self.state.shadow_blur,
        );

        let text = text.replace('\n', " ");

        if let Some(shadow_paint) = shadow_paint {
            self.surface.canvas().save();
            Context::apply_shadow_offset_matrix(
                self.surface.canvas(),
                self.state.shadow_offset.x,
                self.state.shadow_offset.y,
            );
            self.draw_text(text.as_str(), x, y, width, None, &shadow_paint);
            self.surface.canvas().restore();
        }

        self.draw_text(text.as_str(), x, y, width, None, &paint);
    }

    pub fn stroke_text(&mut self, text: &str, x: c_float, y: c_float, width: Option<c_float>) {
        let width = width.unwrap_or(MAX_TEXT_WIDTH);
        let paint = self.state.paint.stroke_paint().clone();
        let shadow_paint = self.state.paint.stroke_shadow_paint(
            (0., 0.).into(),
            self.state.shadow_color,
            self.state.shadow_blur,
        );

        let text = text.replace('\n', " ");

        if let Some(shadow_paint) = shadow_paint {
            self.surface.canvas().save();
            Context::apply_shadow_offset_matrix(
                self.surface.canvas(),
                self.state.shadow_offset.x,
                self.state.shadow_offset.y,
            );
            self.draw_text(text.as_str(), x, y, width, None, &shadow_paint);
            self.surface.canvas().restore();
        }

        self.draw_text(text.as_str(), x, y, width, None, &paint);
    }

    fn apply_shadow_offset_matrix(canvas: &Canvas, shadow_offset_x: f32, shadow_offset_y: f32) {
        let current_transform = canvas.local_to_device_as_3x3();
        if let Some(invert) = canvas.local_to_device_as_3x3().invert() {
            canvas.concat(&invert);
            let mut shadow_offset = current_transform.clone();
            shadow_offset.pre_translate(skia_safe::Vector::new(shadow_offset_x, shadow_offset_y));
            canvas.concat(&shadow_offset);
            canvas.concat(&current_transform);
        }
    }

    fn draw_text(
        &mut self,
        text: &str,
        x: c_float,
        y: c_float,
        max_width: c_float,
        metrics: Option<&mut TextMetrics>,
        paint: &Paint,
    ) {
        let font_collection = &FONT_LIBRARY.lock().collection;
        let weight = skia_safe::font_style::Weight::from(self.state.font_style.weight as i32);
        let stretch = self.state.font_style.stretch;
        let slant = self.state.font_style.style;
        let font_style = skia_safe::FontStyle::new(weight, stretch.into(), slant.into());
        let text_direction = self.state.direction;
        let mut families = vec![];
        for family in self.state.font_style.family.split(',') {
            families.push(family);
        }
        let mut text_style = skia_safe::textlayout::TextStyle::new();
        text_style.set_font_families(families.as_slice());
        text_style.set_font_size(self.state.font_style.size);
        text_style.set_word_spacing(self.state.word_spacing);
        text_style.set_letter_spacing(self.state.letter_spacing);
        text_style.set_height(1.);
        text_style.set_font_style(font_style);
        text_style.set_foreground_paint(paint);
        text_style.set_text_baseline(skia_safe::textlayout::TextBaseline::Alphabetic);

        let mut paragraph_style = skia_safe::textlayout::ParagraphStyle::new();
        paragraph_style.set_text_style(&text_style);
        match text_direction {
            TextDirection::LTR => {
                paragraph_style.set_text_direction(skia_safe::textlayout::TextDirection::LTR);
            }
            TextDirection::RTL => {
                paragraph_style.set_text_direction(skia_safe::textlayout::TextDirection::RTL);
            }
        }

        let mut builder =
            skia_safe::textlayout::ParagraphBuilder::new(&paragraph_style, font_collection);
        builder.peek_style();
        builder.add_text(text);

        let mut paragraph = builder.build();

        paragraph.layout(100000.);

        let metrics_vec = paragraph.get_line_metrics();
        let line_metrics = &metrics_vec[0];

        let font = paragraph.get_font_at(0);
        let (_, font_metrics) = font.metrics();
        let glyphs = font.str_to_glyphs_vec(text);
        let glyphs_size = glyphs.len();
        let mut bounds = vec![skia_safe::Rect::default(); glyphs_size];
        font.get_bounds(glyphs.as_slice(), bounds.as_mut_slice(), None);
        let range: Range<usize> = 0_usize..glyphs_size;
        let text_box = paragraph.get_rects_for_range(
            range,
            skia_safe::textlayout::RectHeightStyle::Tight,
            skia_safe::textlayout::RectWidthStyle::Tight,
        );
        // line_metrics.width doesn't contain the suffix spaces
        // run.calculateWidth will return 0 if font is rendering as fallback

        let mut line_width = 0.0;
        let first_char_bounds = bounds[0];
        let mut descent = first_char_bounds.bottom;
        let mut ascent = first_char_bounds.top;
        let last_char_bounds = bounds[glyphs_size - 1];
        let last_char_pos_x = last_char_bounds.x();

        for tbox in text_box.iter() {
            line_width += tbox.rect.width();
        }

        let until = glyphs_size - 1;
        for i in 1..=until {
            let char_bounds = bounds[i];
            let char_bottom = char_bounds.bottom;
            if char_bottom > descent {
                descent = char_bottom;
            }
            let char_top = char_bounds.top;
            if char_top < ascent {
                ascent = char_top;
            }
        }
        let alphabetic_baseline = paragraph.alphabetic_baseline();

        let css_baseline = self.state.text_baseline;
        let mut baseline_offset = 0.;

        match css_baseline {
            TextBaseLine::TOP => {
                baseline_offset = -alphabetic_baseline
                    - font_metrics.ascent
                    - font_metrics.underline_position().unwrap_or_default()
                    - font_metrics.underline_thickness().unwrap_or_default();
            }
            TextBaseLine::HANGING => {
                // https://github.com/chromium/chromium/blob/104.0.5092.1/third_party/blink/renderer/core/html/canvas/text_metrics.cc#L21-L25
                // According to
                // http://wiki.apache.org/xmlgraphics-fop/LineLayout/AlignmentHandling
                // "FOP (Formatting Objects Processor) puts the hanging baseline at 80% of
                // the ascender height"
                baseline_offset = -alphabetic_baseline
                    - font_metrics.ascent * HANGING_AS_PERCENT_OF_ASCENT / 100.0;
            }
            TextBaseLine::MIDDLE => {
                baseline_offset = paragraph.height() / 2.;
            }
            TextBaseLine::ALPHABETIC => {
                baseline_offset = -alphabetic_baseline;
            }
            TextBaseLine::IDEOGRAPHIC => {
                baseline_offset = -paragraph.ideographic_baseline();
            }
            TextBaseLine::BOTTOM => {
                baseline_offset = -alphabetic_baseline
                    + font_metrics.strikeout_position().unwrap_or_default()
                    + font_metrics.strikeout_thickness().unwrap_or_default();
            }
        }

        let line_center = line_width / 2.;
        let paint_x;
        let mut offset_x = 0.0;

        match self.state.text_align {
            TextAlign::START => {
                if text_direction == TextDirection::LTR {
                    paint_x = x;
                } else {
                    paint_x = x - line_width;
                    offset_x = line_width;
                }
            }
            TextAlign::LEFT => {
                paint_x = x;
            }
            TextAlign::CENTER => {
                paint_x = x - line_center;
                offset_x = line_center;
            }
            TextAlign::RIGHT => {
                paint_x = x - line_width;
                offset_x = line_width;
            }
            TextAlign::END => {
                if text_direction == TextDirection::RTL {
                    paint_x = x;
                } else {
                    paint_x = x - line_width;
                    offset_x = line_width;
                }
            }
        }

        match metrics {
            None => {
                let need_scale = line_width > max_width;
                let ratio = if need_scale {
                    max_width / line_width
                } else {
                    1.0
                };
                if need_scale {
                    self.surface.canvas().save();
                    self.surface.canvas().scale((ratio, 1.0));
                }
                let paint_y = y + baseline_offset;

                text_style.set_foreground_paint(paint);

                paragraph.paint(
                    self.surface.canvas(),
                    (
                        if need_scale {
                            (paint_x + (1. - ratio) * offset_x) / ratio
                        } else {
                            paint_x
                        },
                        paint_y,
                    ),
                );
                if need_scale {
                    self.surface.canvas().restore();
                }
            }
            Some(text_metrics) => {
                let offset = -baseline_offset - alphabetic_baseline;
                text_metrics.actual_bounding_box_ascent = -ascent + offset;
                text_metrics.actual_bounding_box_descent = descent - offset;
                text_metrics.actual_bounding_box_left =
                    -paint_x + line_metrics.left as f32 - first_char_bounds.left;
                text_metrics.actual_bounding_box_right =
                    paint_x + last_char_pos_x + last_char_bounds.right;
                text_metrics.width = line_width;
                text_metrics.font_bounding_box_ascent = -font_metrics.ascent + offset;
                text_metrics.font_bounding_box_descent = font_metrics.descent - offset;
                text_metrics.alphabetic_baseline = -font_metrics.ascent + offset;
            }
        }
    }

    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
        let mut text_metrics = TextMetrics {
            width: 0.0,
            actual_bounding_box_left: 0.0,
            actual_bounding_box_right: 0.0,
            font_bounding_box_ascent: 0.0,
            font_bounding_box_descent: 0.0,
            actual_bounding_box_ascent: 0.0,
            actual_bounding_box_descent: 0.0,
            em_height_ascent: 0.0,
            em_height_descent: 0.0,
            hanging_baseline: 0.0,
            alphabetic_baseline: 0.0,
            ideographic_baseline: 0.0,
        };

        if text.is_empty() {
            return text_metrics;
        }

        let paint = self.state.paint.fill_paint().clone();

        self.draw_text(text, 0., 0., -1., Some(&mut text_metrics), &paint);

        text_metrics
    }

    /*
    fn draw_text(&mut self, is_fill: bool, text: &str, x: c_float, y: c_float, width: c_float) {
        if x.is_infinite() || y.is_infinite() {
            return;
        }

        if width > 0.0 && width.is_infinite() {
            return;
        }
        let shadow_paint;
        let paint;

        if is_fill {
            paint = self.state.paint.fill_paint();
            shadow_paint = self.state.paint.fill_shadow_paint(
                self.state.shadow_offset,
                self.state.shadow_color,
                self.state.shadow_blur,
            );
        } else {
            paint = self.state.paint.stroke_paint();
            shadow_paint = self.state.paint.stroke_shadow_paint(
                self.state.shadow_offset,
                self.state.shadow_color,
                self.state.shadow_blur,
            );
        }
        let font = &self.state.font;

        let measurement = font.to_skia().measure_str(text, Some(paint));
        let font_width = measurement.0;
        let max_width = width;
        let width: f32;
        let use_max_width = max_width > 0.0 && max_width < font_width;
        if use_max_width {
            width = max_width
        } else {
            width = font_width;
        }
        let (line_spacing, metrics) = font.to_skia().metrics();
        let baseline = get_font_baseline(metrics, self.state.text_baseline);
        let mut location: Point = (x, y + baseline).into();

        match to_real_text_align(self.state.text_align, self.state.direction) {
            TextAlign::RIGHT => {
                location.x = location.x - width;
            }
            TextAlign::CENTER => {
                location.x = location.x - (width / 2.0);
            }
            _ => {
                // NOOP
            }
        }
        let mut rect: (Point, Size) = (
            (
                location.x - metrics.cap_height / 2.0,
                location.y - metrics.ascent - metrics.leading,
            )
                .into(),
            (width + metrics.cap_height, line_spacing).into(),
        );

        if !is_fill {
            inflate_stroke_rect(&mut rect, paint);
        }

        if use_max_width {
            self.surface.canvas().save();
            self.surface
                .canvas()
                .translate(Point::new(location.x, location.y));
            let mut scale_x: f32 = 0.0;
            if font_width > 0.0 {
                scale_x = width / font_width;
            }

            // We draw when font_width is 0 so compositing operations (eg, a "copy" op) still work.
            self.surface.canvas().scale((scale_x, 1.0));
        }

        let font = font.to_skia();
        if let Some(shadow_paint) = shadow_paint {
            self.surface
                .canvas()
                .draw_str(text, (location.x, location.y), &font, &shadow_paint);
        }

        self.surface
            .canvas()
            .draw_str(text, (location.x, location.y), &font, paint);

        if use_max_width {
            self.surface.canvas().restore();
        }
    }

    */

    /* pub fn measure_text(&self, text: &str) -> TextMetrics {
        let (width, bounds) = self
            .state
            .font
            .to_skia()
            .measure_str(text, Some(self.state.paint.fill_paint()));
        let (_, metrics) = self.state.font.to_skia().metrics();
        let ascent = metrics.ascent;
        let descent = metrics.descent;
        let baseline_y = get_font_baseline(metrics, self.state.text_baseline);
        TextMetrics {
            width,
            actual_bounding_box_left: -bounds.x(),
            actual_bounding_box_right: bounds.right(),
            font_bounding_box_ascent: (ascent - baseline_y),
            font_bounding_box_descent: (descent + baseline_y),
            actual_bounding_box_ascent: (-bounds.y() - baseline_y),
            actual_bounding_box_descent: (bounds.bottom() + baseline_y),
            em_height_ascent: 0.0,
            em_height_descent: 0.0,
            hanging_baseline: (-0.8 * ascent + baseline_y),
            alphabetic_baseline: baseline_y,
            ideographic_baseline: (descent + baseline_y),
        }
    }*/
}
