use std::os::raw::c_float;

use skia_safe::{BlendMode, Contains, Paint, Rect};

use crate::context::Context;

impl Context {
    pub fn clear_rect(&mut self, x: c_float, y: c_float, width: c_float, height: c_float) {
        match self
            .state
            .matrix
            .map_rect(Rect::new(x, y, width, height))
            .0
            .contains(&self.surface_data.bounds)
        {
            // if rect fully encloses canvas, erase existing content (but preserve CTM, path, etc.)
            true => {
                self.with_recorder(|mut recorder|{
                    recorder.set_bounds(self.surface_data.bounds);
                    recorder.set_matrix(self.state.matrix);
                    recorder.set_clip(&self.state.clip);
                });
            }

            // otherwise, paint over the specified region but preserve overdrawn vectors
            false => {
                let mut paint = Paint::default();
                paint
                    .set_anti_alias(true)
                    .set_style(skia_safe::PaintStyle::Fill)
                    .set_blend_mode(BlendMode::Clear);

                self.render_to_canvas(&paint, |canvas, paint| {
                    let rect = Rect::from_xywh(x, y, width, height);
                    canvas.draw_rect(&rect, paint);
                });
            }
        }
    }

    pub fn fill_rect_xywh(&mut self, x: c_float, y: c_float, width: c_float, height: c_float) {
        let rect = Rect::from_xywh(x, y, width, height);
        self.fill_rect(&rect);
    }

    pub fn fill_rect(&mut self, rect: &Rect) {
        self.render_to_canvas(self.state.paint.fill_paint(), |canvas, paint| {
            if let Some(paint) = self.state.paint.fill_shadow_paint(
                self.state.shadow_offset,
                self.state.shadow_color,
                self.state.shadow_blur,
            ) {
                canvas.draw_rect(rect, &paint);
            }
            canvas.draw_rect(rect, paint);
        });
    }

    pub fn stroke_rect_xywh(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let rect = Rect::from_xywh(x, y, width, height);
        self.stroke_rect(&rect);
    }

    pub fn stroke_rect(&mut self, rect: &Rect) {
        self.render_to_canvas(self.state.paint.stroke_paint(), |canvas, paint| {
            if let Some(paint) = &mut self.state.paint.stroke_shadow_paint(
                self.state.shadow_offset,
                self.state.shadow_color,
                self.state.shadow_blur,
            ) {
                canvas.draw_rect(rect, &paint);
            }
            canvas.draw_rect(rect, paint);
        });
    }
}
