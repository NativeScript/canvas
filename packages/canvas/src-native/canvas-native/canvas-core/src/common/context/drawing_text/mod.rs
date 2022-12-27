use std::os::raw::c_float;

use skia_safe::{Point, Size};
use skia_safe::paint::Style;

use crate::common::context::Context;
use crate::common::context::drawing_text::text_metrics::TextMetrics;
use crate::common::context::drawing_text::typography::{get_font_baseline, to_real_text_align};
use crate::common::context::text_styles::text_align::TextAlign;
use crate::common::utils::geometry::inflate_stroke_rect;

pub mod text_metrics;
pub(crate) mod typography;

impl Context {
    pub fn fill_text(&mut self, text: &str, x: c_float, y: c_float, width: c_float) {
        self.draw_text(true, text, x, y, width);
    }

    pub fn stroke_text(&mut self, text: &str, x: c_float, y: c_float, width: c_float) {
        self.draw_text(false, text, x, y, width);
    }

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
            paint = self.state.paint.fill_paint().clone();
            shadow_paint = self.state.paint.fill_shadow_paint(
                self.state.shadow_offset,
                self.state.shadow_color,
                self.state.shadow_blur,
            ).map(|p| p.clone());
        } else {
            paint = self.state.paint.stroke_paint().clone();
            shadow_paint = self.state.paint.stroke_shadow_paint(
                self.state.shadow_offset,
                self.state.shadow_color,
                self.state.shadow_blur,
            ).map(|p| p.clone());
        }
        let font = &self.state.font;

        let measurement = font.to_skia().measure_str(text, Some(&paint));
        let font_width = measurement.0;
        let max_width = width;
        let width: f32;
        let use_max_width = max_width > 0.0 && max_width < font_width;
        if use_max_width {
            width = max_width
        } else {
            width = font_width;
        }
        let metrics = font.to_skia().metrics();
        let baseline = get_font_baseline(metrics.1, self.state.text_baseline);
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
        let (line_spacing, metrics) = font.to_skia().metrics();

        let mut rect: (Point, Size) = (
            (
                location.x - metrics.cap_height / 2.0,
                location.y - metrics.ascent - metrics.leading,
            )
                .into(),
            (width + metrics.cap_height, line_spacing).into(),
        );
        if paint.style() == Style::Stroke {
            inflate_stroke_rect(&mut rect, &paint);
        }

        let mut did_save = false;
        if use_max_width {
            self.surface.canvas().save();
            did_save = true;
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

        self.set_scale_for_device();

        if let Some(shadow_paint) = shadow_paint {
            self.surface
                .canvas()
                .draw_str(text, (location.x, location.y), &font, &shadow_paint);
        }

        {
            self.surface
                .canvas()
                .draw_str(text, (location.x, location.y), &font, &paint);
        }

        self.clear_scale_for_device();

        if did_save {
            self.surface.canvas().restore();
        }
    }

    pub fn measure_text(&self, text: &str) -> TextMetrics {
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
    }
}
