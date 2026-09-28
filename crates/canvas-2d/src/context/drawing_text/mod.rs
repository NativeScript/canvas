use std::os::raw::c_float;

use skia_safe::{Canvas, Paint};

use crate::context::drawing_text::shaping::ShapedText;
use crate::context::drawing_text::text_metrics::TextMetrics;
use crate::context::text_styles::text_align::TextAlign;
use crate::context::text_styles::text_baseline::TextBaseLine;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::Context;

pub(crate) const MAX_TEXT_WIDTH: f32 = 100_000.0;

pub mod global_fonts;
pub(crate) mod shaping;
pub mod text_metrics;
pub(crate) mod typography;

const HANGING_AS_PERCENT_OF_ASCENT: f32 = 80.;

/// Where a laid-out line lands once the CSS baseline and alignment are applied.
struct Placement {
    x: f32,
    y: f32,
    /// Distance from `x` to the alignment anchor, so the `maxWidth` squeeze shrinks
    /// towards the anchor rather than to the left.
    offset_x: f32,
}

impl Context {
    pub fn fill_text(&mut self, text: &str, x: c_float, y: c_float, width: Option<c_float>) {
        #[cfg(feature = "gl")]
        {
            if let Some(ref context) = self.gl_context {
                context.make_current();
            }
        }
        let paint = self.state.paint.fill_paint().clone();
        let shadow_paint = self.state.paint.fill_shadow_paint(
            (0., 0.).into(),
            self.state.shadow_color,
            self.state.shadow_blur,
        );
        self.draw_text_run(text, x, y, width, paint, shadow_paint);
    }

    pub fn stroke_text(&mut self, text: &str, x: c_float, y: c_float, width: Option<c_float>) {
        #[cfg(feature = "gl")]
        {
            if let Some(ref context) = self.gl_context {
                context.make_current();
            }
        }
        let paint = self.state.paint.stroke_paint().clone();
        let shadow_paint = self.state.paint.stroke_shadow_paint(
            (0., 0.).into(),
            self.state.shadow_color,
            self.state.shadow_blur,
        );
        self.draw_text_run(text, x, y, width, paint, shadow_paint);
    }

    /// The half of `fillText`/`strokeText` that is independent of the paint, so a
    /// shadow can reuse the layout instead of re-shaping it.
    fn draw_text_run(
        &mut self,
        text: &str,
        x: c_float,
        y: c_float,
        width: Option<c_float>,
        paint: Paint,
        shadow_paint: Option<Paint>,
    ) {
        let max_width = width.unwrap_or(MAX_TEXT_WIDTH);

        let owned;
        let text = if text.contains('\n') {
            owned = text.replace('\n', " ");
            owned.as_str()
        } else {
            text
        };

        let shaped = shaping::shape(
            text,
            &self.state.font_style,
            self.state.direction,
            self.state.word_spacing,
            self.state.letter_spacing,
        );

        let Some(blob) = shaped.blob.as_ref() else {
            return;
        };
        if shaped.line_width == 0. {
            return;
        }

        let placement = Self::place(
            &shaped,
            self.state.text_baseline,
            self.state.text_align,
            self.state.direction,
            x,
            y,
        );
        let shadow_offset_x = self.state.shadow_offset.x;
        let shadow_offset_y = self.state.shadow_offset.y;
        let line_width = shaped.line_width;

        self.render_text_to_canvas(&paint, |canvas, paint| {
            if let Some(shadow_paint) = &shadow_paint {
                canvas.save();
                Context::apply_shadow_offset_matrix(canvas, shadow_offset_x, shadow_offset_y);
                Context::paint_blob(canvas, blob, &placement, line_width, max_width, shadow_paint);
                canvas.restore();
            }

            Context::paint_blob(canvas, blob, &placement, line_width, max_width, paint);
        });
    }

    fn apply_shadow_offset_matrix(canvas: &Canvas, shadow_offset_x: f32, shadow_offset_y: f32) {
        // Compute the current transform once — the original called local_to_device_as_3x3()
        // twice (once to store, once to invert), wasting a matrix decomposition.
        let current_transform = canvas.local_to_device_as_3x3();
        if let Some(invert) = current_transform.invert() {
            canvas.concat(&invert);
            let mut shadow_offset = current_transform;
            shadow_offset.pre_translate(skia_safe::Vector::new(shadow_offset_x, shadow_offset_y));
            canvas.concat(&shadow_offset);
            canvas.concat(&current_transform);
        }
    }

    /// Resolves `textBaseline` and `textAlign` against a layout's metrics.
    fn place(
        shaped: &ShapedText,
        text_baseline: TextBaseLine,
        text_align: TextAlign,
        direction: TextDirection,
        x: f32,
        y: f32,
    ) -> Placement {
        let font_metrics = &shaped.font_metrics;
        let alphabetic_baseline = shaped.alphabetic_baseline;

        let baseline_offset = match text_baseline {
            TextBaseLine::TOP => {
                -alphabetic_baseline
                    - font_metrics.ascent
                    - font_metrics.underline_position().unwrap_or_default()
                    - font_metrics.underline_thickness().unwrap_or_default()
            }
            TextBaseLine::HANGING => {
                // https://github.com/chromium/chromium/blob/104.0.5092.1/third_party/blink/renderer/core/html/canvas/text_metrics.cc#L21-L25
                // According to
                // http://wiki.apache.org/xmlgraphics-fop/LineLayout/AlignmentHandling
                // "FOP (Formatting Objects Processor) puts the hanging baseline at 80% of
                // the ascender height"
                -alphabetic_baseline - font_metrics.ascent * HANGING_AS_PERCENT_OF_ASCENT / 100.0
            }
            TextBaseLine::MIDDLE => -shaped.height / 2.,
            TextBaseLine::ALPHABETIC => -alphabetic_baseline,
            TextBaseLine::IDEOGRAPHIC => -shaped.ideographic_baseline,
            TextBaseLine::BOTTOM => {
                -alphabetic_baseline
                    + font_metrics.strikeout_position().unwrap_or_default()
                    + font_metrics.strikeout_thickness().unwrap_or_default()
            }
        };

        let line_width = shaped.line_width;
        let line_center = line_width / 2.;
        let paint_x;
        let mut offset_x = 0.0;

        match text_align {
            TextAlign::START => {
                if direction == TextDirection::LTR {
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
                if direction == TextDirection::RTL {
                    paint_x = x;
                } else {
                    paint_x = x - line_width;
                    offset_x = line_width;
                }
            }
        }

        Placement {
            x: paint_x,
            y: y + baseline_offset,
            offset_x,
        }
    }

    fn paint_blob(
        canvas: &Canvas,
        blob: &skia_safe::TextBlob,
        placement: &Placement,
        line_width: f32,
        max_width: f32,
        paint: &Paint,
    ) {
        if line_width > max_width {
            let ratio = max_width / line_width;
            let current_y = canvas.local_to_device_as_3x3().scale_y();
            canvas.save();
            canvas.scale((ratio, current_y));
            canvas.draw_text_blob(
                blob,
                (
                    (placement.x + (1. - ratio) * placement.offset_x) / ratio,
                    placement.y,
                ),
                paint,
            );
            canvas.restore();
        } else {
            canvas.draw_text_blob(blob, (placement.x, placement.y), paint);
        }
    }

    pub fn measure_text(&self, text: &str) -> TextMetrics {
        let mut text_metrics = TextMetrics::default();

        if text.is_empty() {
            return text_metrics;
        }

        let shaped = shaping::shape(
            text,
            &self.state.font_style,
            self.state.direction,
            self.state.word_spacing,
            self.state.letter_spacing,
        );

        if shaped.line_width == 0. {
            return text_metrics;
        }

        let extents = shaped.extents(text);
        if extents.empty {
            return text_metrics;
        }

        let placement = Self::place(
            &shaped,
            self.state.text_baseline,
            self.state.text_align,
            self.state.direction,
            0.,
            0.,
        );

        // `place` folds the baseline into y; measure_text reports from the alphabetic
        // baseline, so take it back out.
        let offset = -(placement.y) - shaped.alphabetic_baseline;
        let font_metrics = &shaped.font_metrics;

        text_metrics.actual_bounding_box_ascent = -extents.ascent + offset;
        text_metrics.actual_bounding_box_descent = extents.descent - offset;
        text_metrics.actual_bounding_box_left = -placement.x + shaped.line_left - extents.first_left;
        text_metrics.actual_bounding_box_right = placement.x + extents.last_x + extents.last_right;
        text_metrics.width = shaped.line_width;
        text_metrics.font_bounding_box_ascent = -font_metrics.ascent + offset;
        text_metrics.font_bounding_box_descent = font_metrics.descent - offset;
        text_metrics.alphabetic_baseline = -font_metrics.ascent + offset;

        text_metrics
    }
}

#[cfg(test)]
mod parity {
    //! Pins `fillText`/`measureText` against the paragraph-per-call implementation
    //! the shaped-layout cache replaced, transcribed below as the reference.

    use super::*;
    use crate::context::drawing_text::global_fonts::FONT_LIBRARY;
    use crate::context::drawing_text::typography::Font;
    use crate::context::ColorSpace;

    #[allow(clippy::too_many_arguments)]
    fn reference(
        canvas: Option<&Canvas>,
        font_style: &Font,
        direction: TextDirection,
        word_spacing: f32,
        letter_spacing: f32,
        text_baseline: TextBaseLine,
        text_align: TextAlign,
        text: &str,
        x: f32,
        y: f32,
        max_width: f32,
        mut metrics: Option<&mut TextMetrics>,
        paint: &Paint,
    ) {
        let weight = skia_safe::font_style::Weight::from(font_style.weight as i32);
        let sk_font_style =
            skia_safe::FontStyle::new(weight, font_style.stretch.into(), font_style.style.into());

        let families: Vec<&str> = font_style.family.iter().map(|s| s.as_str()).collect();

        let mut text_style = skia_safe::textlayout::TextStyle::new();
        text_style.set_font_families(families.as_slice());
        text_style.set_font_size(font_style.size);
        text_style.set_word_spacing(word_spacing);
        text_style.set_letter_spacing(letter_spacing);
        text_style.set_height(1.);
        text_style.set_font_style(sk_font_style);
        text_style.set_foreground_paint(paint);
        text_style.set_text_baseline(skia_safe::textlayout::TextBaseline::Alphabetic);

        let font_collection = FONT_LIBRARY.lock().collect_fonts(&text_style);

        let mut paragraph_style = skia_safe::textlayout::ParagraphStyle::new();
        paragraph_style.set_text_style(&text_style);
        paragraph_style.set_text_direction(match direction {
            TextDirection::LTR => skia_safe::textlayout::TextDirection::LTR,
            TextDirection::RTL => skia_safe::textlayout::TextDirection::RTL,
        });

        let mut builder =
            skia_safe::textlayout::ParagraphBuilder::new(&paragraph_style, &font_collection);
        builder.add_text(text);
        let mut paragraph = builder.build();
        paragraph.layout(MAX_TEXT_WIDTH);

        let font = paragraph.get_font_at(0);
        let (_, font_metrics) = font.metrics();
        let line_width = paragraph.max_intrinsic_width();
        if line_width == 0. {
            return;
        }
        let alphabetic_baseline = paragraph.alphabetic_baseline();

        let baseline_offset = match text_baseline {
            TextBaseLine::TOP => {
                -alphabetic_baseline
                    - font_metrics.ascent
                    - font_metrics.underline_position().unwrap_or_default()
                    - font_metrics.underline_thickness().unwrap_or_default()
            }
            TextBaseLine::HANGING => {
                -alphabetic_baseline
                    - font_metrics.ascent * HANGING_AS_PERCENT_OF_ASCENT / 100.0
            }
            TextBaseLine::MIDDLE => -paragraph.height() / 2.,
            TextBaseLine::ALPHABETIC => -alphabetic_baseline,
            TextBaseLine::IDEOGRAPHIC => -paragraph.ideographic_baseline(),
            TextBaseLine::BOTTOM => {
                -alphabetic_baseline
                    + font_metrics.strikeout_position().unwrap_or_default()
                    + font_metrics.strikeout_thickness().unwrap_or_default()
            }
        };

        let line_center = line_width / 2.;
        let paint_x;
        let mut offset_x = 0.0;
        match text_align {
            TextAlign::START => {
                if direction == TextDirection::LTR {
                    paint_x = x;
                } else {
                    paint_x = x - line_width;
                    offset_x = line_width;
                }
            }
            TextAlign::LEFT => paint_x = x,
            TextAlign::CENTER => {
                paint_x = x - line_center;
                offset_x = line_center;
            }
            TextAlign::RIGHT => {
                paint_x = x - line_width;
                offset_x = line_width;
            }
            TextAlign::END => {
                if direction == TextDirection::RTL {
                    paint_x = x;
                } else {
                    paint_x = x - line_width;
                    offset_x = line_width;
                }
            }
        }

        match (metrics.as_mut(), canvas) {
            (None, Some(canvas)) => {
                let need_scale = line_width > max_width;
                canvas.save();
                let paint_y = y + baseline_offset;
                if need_scale {
                    let ratio = max_width / line_width;
                    let current_y = canvas.local_to_device_as_3x3().scale_y();
                    canvas.scale((ratio, current_y));
                    paragraph.paint(canvas, ((paint_x + (1. - ratio) * offset_x) / ratio, paint_y));
                } else {
                    paragraph.paint(canvas, (paint_x, paint_y));
                }
                canvas.restore();
            }
            (Some(text_metrics), _) => {
                let line_left = paragraph
                    .get_line_metrics()
                    .first()
                    .map(|lm| lm.left as f32)
                    .unwrap_or(0.0);

                let glyphs = font.str_to_glyphs_vec(text);
                let glyphs_size = glyphs.len();
                if glyphs_size == 0 {
                    return;
                }
                let mut bounds = vec![skia_safe::Rect::default(); glyphs_size];
                font.get_bounds(glyphs.as_slice(), bounds.as_mut_slice(), None);

                let first_char_bounds = bounds[0];
                let mut descent = first_char_bounds.bottom;
                let mut ascent = first_char_bounds.top;
                let last_char_bounds = bounds[glyphs_size - 1];
                let last_char_pos_x = last_char_bounds.x();

                for b in bounds.iter().take(glyphs_size).skip(1) {
                    if b.bottom > descent {
                        descent = b.bottom;
                    }
                    if b.top < ascent {
                        ascent = b.top;
                    }
                }

                let offset = -baseline_offset - alphabetic_baseline;
                text_metrics.actual_bounding_box_ascent = -ascent + offset;
                text_metrics.actual_bounding_box_descent = descent - offset;
                text_metrics.actual_bounding_box_left =
                    -paint_x + line_left - first_char_bounds.left;
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

    const W: f32 = 420.;
    const H: f32 = 160.;

    fn context() -> Context {
        Context::new(W, H, 1.0, true, 0, 96., TextDirection::LTR, ColorSpace::Srgb)
    }

    fn pixels(ctx: &mut Context) -> Vec<u8> {
        let mut out = vec![0u8; (W * H * 4.) as usize];
        ctx.get_pixels(&mut out, (0, 0), (W as i32, H as i32));
        out
    }

    const ALIGNMENTS: [TextAlign; 5] = [
        TextAlign::START,
        TextAlign::LEFT,
        TextAlign::CENTER,
        TextAlign::RIGHT,
        TextAlign::END,
    ];
    const BASELINES: [TextBaseLine; 6] = [
        TextBaseLine::TOP,
        TextBaseLine::HANGING,
        TextBaseLine::MIDDLE,
        TextBaseLine::ALPHABETIC,
        TextBaseLine::IDEOGRAPHIC,
        TextBaseLine::BOTTOM,
    ];

    #[test]
    fn fill_text_matches_the_paragraph_implementation() {
        for spec in ["24px sans-serif", "italic bold 30px serif", "18px monospace"] {
            for align in ALIGNMENTS {
                for baseline in BASELINES {
                    for (text, max_width) in
                        [("Handgloves AVW", None), ("Handgloves AVW", Some(40.0f32))]
                    {
                        let mut ours = context();
                        ours.set_font(spec);
                        ours.set_text_align(align);
                        ours.set_text_baseline(baseline);
                        ours.set_fill_style_with_color("white");
                        ours.fill_text(text, 200., 90., max_width);
                        let actual = pixels(&mut ours);

                        let mut theirs = context();
                        theirs.set_font(spec);
                        theirs.set_fill_style_with_color("white");
                        let font_style = theirs.state.font_style.clone();
                        let paint = theirs.state.paint.fill_paint().clone();
                        theirs.with_canvas_dirty(|canvas| {
                            reference(
                                Some(canvas),
                                &font_style,
                                TextDirection::LTR,
                                0.,
                                0.,
                                baseline,
                                align,
                                text,
                                200.,
                                90.,
                                max_width.unwrap_or(MAX_TEXT_WIDTH),
                                None,
                                &paint,
                            );
                        });
                        let expected = pixels(&mut theirs);

                        assert!(
                            expected.iter().any(|b| *b != 0),
                            "{spec}/{align:?}/{baseline:?}: reference drew nothing"
                        );
                        let diff = expected
                            .iter()
                            .zip(actual.iter())
                            .filter(|(a, b)| a.abs_diff(**b) > 1)
                            .count();
                        assert_eq!(
                            diff, 0,
                            "{spec}/{align:?}/{baseline:?}/max_width={max_width:?}: \
                             {diff} bytes differ"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn measure_text_matches_the_paragraph_implementation() {
        for spec in ["24px sans-serif", "italic bold 30px serif", "18px monospace"] {
            for align in ALIGNMENTS {
                for baseline in BASELINES {
                    for text in ["Handgloves AVW", "j", "  spaced  out  "] {
                        let mut ctx = context();
                        ctx.set_font(spec);
                        ctx.set_text_align(align);
                        ctx.set_text_baseline(baseline);
                        let actual = ctx.measure_text(text);

                        let mut expected = TextMetrics::default();
                        let paint = ctx.state.paint.fill_paint().clone();
                        reference(
                            None,
                            &ctx.state.font_style,
                            TextDirection::LTR,
                            0.,
                            0.,
                            baseline,
                            align,
                            text,
                            0.,
                            0.,
                            -1.,
                            Some(&mut expected),
                            &paint,
                        );

                        let label = format!("{spec}/{align:?}/{baseline:?}/`{text}`");
                        let fields: [(&str, f32, f32); 8] = [
                            ("width", actual.width, expected.width),
                            ("bbox_left", actual.actual_bounding_box_left, expected.actual_bounding_box_left),
                            ("bbox_right", actual.actual_bounding_box_right, expected.actual_bounding_box_right),
                            ("bbox_ascent", actual.actual_bounding_box_ascent, expected.actual_bounding_box_ascent),
                            ("bbox_descent", actual.actual_bounding_box_descent, expected.actual_bounding_box_descent),
                            ("font_ascent", actual.font_bounding_box_ascent, expected.font_bounding_box_ascent),
                            ("font_descent", actual.font_bounding_box_descent, expected.font_bounding_box_descent),
                            ("alphabetic", actual.alphabetic_baseline, expected.alphabetic_baseline),
                        ];
                        for (name, a, e) in fields {
                            assert!(
                                (a - e).abs() <= 0.001,
                                "{label}: {name} {a} != {e}"
                            );
                        }
                    }
                }
            }
        }
    }
}

