use std::ops::Range;
use std::os::raw::c_float;

use skia_safe::{Canvas, Paint};

use crate::context::Context;
use crate::context::drawing_text::global_fonts::FONT_LIBRARY;
use crate::context::drawing_text::text_metrics::TextMetrics;
use crate::context::drawing_text::typography::Font;
use crate::context::text_styles::text_align::TextAlign;
use crate::context::text_styles::text_baseline::TextBaseLine;
use crate::context::text_styles::text_direction::TextDirection;

pub(crate) const MAX_TEXT_WIDTH: f32 = 100_000.0;

pub mod global_fonts;
pub mod text_metrics;
pub(crate) mod typography;

const HANGING_AS_PERCENT_OF_ASCENT: f32 = 80.;

impl Context {
    pub fn fill_text(&mut self, text: &str, x: c_float, y: c_float, width: Option<c_float>) {
        #[cfg(feature = "gl")]{
            if let Some(ref context) = self.gl_context {
                context.make_current();
            }
        }
        let width = width.unwrap_or(MAX_TEXT_WIDTH);
        let shadow_paint = self.state.paint.fill_shadow_paint(
            (0., 0.).into(),
            self.state.shadow_color,
            self.state.shadow_blur,
        );

        let text = text.replace('\n', " ");

        let paint = self.state.paint.fill_paint().clone();
        let shadow_offset_x = self.state.shadow_offset.x;
        let shadow_offset_y = self.state.shadow_offset.y;

        let direction = self.state.direction;
        let word_spacing = self.state.word_spacing;
        let letter_spacing = self.state.letter_spacing;
        let text_baseline = self.state.text_baseline;
        let text_align = self.state.text_align;

        self.render_text_to_canvas(&paint, |canvas, paint, font| {
            if let Some(shadow_paint) = &shadow_paint {
                canvas.save();
                Context::apply_shadow_offset_matrix(
                    canvas,
                    shadow_offset_x,
                    shadow_offset_y,
                );
                Context::draw_text(Some(canvas), font, direction, word_spacing, letter_spacing, text_baseline, text_align, text.as_str(), x, y, width, None, shadow_paint);
                canvas.restore();
            }

            Context::draw_text(Some(canvas), font, direction, word_spacing, letter_spacing, text_baseline, text_align, text.as_str(), x, y, width, None, paint);
        });
    }

    pub fn stroke_text(&mut self, text: &str, x: c_float, y: c_float, width: Option<c_float>) {
        #[cfg(feature = "gl")]{
            if let Some(ref context) = self.gl_context {
                context.make_current();
            }
        }
        let width = width.unwrap_or(MAX_TEXT_WIDTH);
        let shadow_paint = self.state.paint.stroke_shadow_paint(
            (0., 0.).into(),
            self.state.shadow_color,
            self.state.shadow_blur,
        );

        let text = text.replace('\n', " ");

        let paint = self.state.paint.stroke_paint().clone();
        let shadow_offset_x = self.state.shadow_offset.x;
        let shadow_offset_y = self.state.shadow_offset.y;
        let direction = self.state.direction;
        let word_spacing = self.state.word_spacing;
        let letter_spacing = self.state.letter_spacing;
        let text_baseline = self.state.text_baseline;
        let text_align = self.state.text_align;

        self.render_text_to_canvas(&paint, |canvas, paint, font| {
            if let Some(shadow_paint) = &shadow_paint {
                canvas.save();
                Context::apply_shadow_offset_matrix(
                    canvas,
                    shadow_offset_x,
                    shadow_offset_y,
                );
                Context::draw_text(Some(canvas), font, direction, word_spacing, letter_spacing, text_baseline, text_align, text.as_str(), x, y, width, None, shadow_paint);
                canvas.restore();
            }

            Context::draw_text(Some(canvas), font, direction, word_spacing, letter_spacing, text_baseline, text_align, text.as_str(), x, y, width, None, paint);
        });
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
        canvas: Option<&Canvas>,
        font_style: &Font,
        direction: TextDirection,
        word_spacing: f32,
        letter_spacing: f32,
        text_baseline: TextBaseLine,
        text_align: TextAlign,
        text: &str,
        x: c_float,
        y: c_float,
        max_width: c_float,
        metrics: Option<&mut TextMetrics>,
        paint: &Paint,
    ) {
        let font_collection = &FONT_LIBRARY.lock().collection;
        let weight = skia_safe::font_style::Weight::from(font_style.weight as i32);
        let stretch = font_style.stretch;
        let slant = font_style.style;
        let families: Vec<_> = font_style.family.split(',').collect();
        let font_size = font_style.size;
        let font_style = skia_safe::FontStyle::new(weight, stretch.into(), slant.into());
        let text_direction = direction;

        let mut text_style = skia_safe::textlayout::TextStyle::new();
        text_style.set_font_families(families.as_slice());
        text_style.set_font_size(font_size);
        text_style.set_word_spacing(word_spacing);
        text_style.set_letter_spacing(letter_spacing);
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

        if metrics_vec.is_empty() {
            return;
        }
        let line_metrics = &metrics_vec[0];

        let font = paragraph.get_font_at(0);
        let (_, font_metrics) = font.metrics();
        if font_metrics.has_bounds() {}
        let glyphs = font.str_to_glyphs_vec(text);
        let glyphs_size = glyphs.len();
        if glyphs_size == 0 {
            return;
        }
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
        for tbox in text_box.iter() {
            line_width += tbox.rect.width();
        }

        if line_width == 0. {
            return;
        }

        let first_char_bounds = bounds[0];
        let mut descent = first_char_bounds.bottom;
        let mut ascent = first_char_bounds.top;
        let last_char_bounds = bounds[glyphs_size - 1];
        let last_char_pos_x = last_char_bounds.x();

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

        let css_baseline = text_baseline;
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
                baseline_offset = -paragraph.height() / 2.;
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

        match text_align {
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

        match (metrics, canvas) {
            (None, Some(canvas)) => {
                let need_scale = line_width > max_width;
                let ratio = if need_scale {
                    max_width / line_width
                } else {
                    1.0
                };
                let current = canvas.local_to_device_as_3x3();
                let current_y = current.scale_y();
                canvas.save();
                if need_scale {
                    canvas.scale((ratio, current_y));
                }
                let paint_y = y + baseline_offset;

                text_style.set_foreground_paint(paint);

                paragraph.paint(
                    canvas,
                    (
                        if need_scale {
                            (paint_x + (1. - ratio) * offset_x) / ratio
                        } else {
                            paint_x
                        },
                        paint_y,
                    ),
                );

                canvas.restore();
            }
            (Some(text_metrics), _) => {
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
            _ => {}
        }
    }

    pub fn measure_text(&self, text: &str) -> TextMetrics {
        let mut text_metrics = TextMetrics::default();

        if text.is_empty() {
            return text_metrics;
        }

        let paint = self.state.paint.fill_paint().clone();

        Context::draw_text(None, &self.state.font_style, self.state.direction, self.state.word_spacing, self.state.letter_spacing, self.state.text_baseline, self.state.text_align, text, 0., 0., -1., Some(&mut text_metrics), &paint);

        text_metrics
    }
}
