use std::os::raw::c_float;
use std::sync::OnceLock;

use skia_safe::{BlendMode, Color, Paint, Rect};

use crate::context::Context;

fn clear_paint() -> &'static Paint {
    static CLEAR_PAINT: OnceLock<Paint> = OnceLock::new();
    CLEAR_PAINT.get_or_init(|| {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Fill);
        paint.set_blend_mode(BlendMode::Clear);
        paint.set_color(Color::TRANSPARENT);
        paint
    })
}

impl Context {
    #[inline]
    pub fn clear_rect(&mut self, x: c_float, y: c_float, width: c_float, height: c_float) {
        let paint = clear_paint();
        self.render_to_canvas(paint, |canvas, paint| {
            let rect = Rect::from_xywh(x, y, width, height);
            canvas.draw_rect(&rect, paint);
        });
    }

    pub fn fill_rect_xywh(&mut self, x: c_float, y: c_float, width: c_float, height: c_float) {
        let rect = Rect::from_xywh(x, y, width, height);
        self.fill_rect(&rect);
    }

    pub fn fill_rect(&mut self, rect: &Rect) {
        let paint = self.state.paint.fill_paint().clone();

        let has_shadow = self.state.shadow_color != Color::TRANSPARENT
            && (self.state.shadow_blur > 0.0
                || self.state.shadow_offset.x != 0.0
                || self.state.shadow_offset.y != 0.0);

        if has_shadow {
            let shadow_paint = self.state.paint.fill_shadow_paint(
                self.state.shadow_offset,
                self.state.shadow_color,
                self.state.shadow_blur,
            );
            self.render_to_canvas(&paint, |canvas, paint| {
                if let Some(paint) = &shadow_paint {
                    canvas.draw_rect(rect, paint);
                }
                canvas.draw_rect(rect, paint);
            });
        } else {
            self.render_to_canvas(&paint, |canvas, paint| {
                canvas.draw_rect(rect, paint);
            });
        }
    }

    pub fn stroke_rect_xywh(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let rect = Rect::from_xywh(x, y, width, height);
        self.stroke_rect(&rect);
    }

    pub fn stroke_rect(&mut self, rect: &Rect) {
        let paint = self.state.paint.stroke_paint().clone();

        let has_shadow = self.state.shadow_color != Color::TRANSPARENT
            && (self.state.shadow_blur > 0.0
                || self.state.shadow_offset.x != 0.0
                || self.state.shadow_offset.y != 0.0);

        if has_shadow {
            let shadow_paint = self.state.paint.stroke_shadow_paint(
                self.state.shadow_offset,
                self.state.shadow_color,
                self.state.shadow_blur,
            );
            self.render_to_canvas(&paint, |canvas, paint| {
                if let Some(paint) = &shadow_paint {
                    canvas.draw_rect(rect, paint);
                }
                canvas.draw_rect(rect, paint);
            });
        } else {
            self.render_to_canvas(&paint, |canvas, paint| {
                canvas.draw_rect(rect, paint);
            });
        }
    }
}
