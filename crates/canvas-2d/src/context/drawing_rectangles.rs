use std::os::raw::c_float;
use std::sync::Arc;

use skia_safe::{BlendMode, Contains, Paint, Rect};

use crate::context::{Context, Recorder};

impl Context {
    pub fn clear_rect(&mut self, x: c_float, y: c_float, width: c_float, height: c_float) {
        match self
            .state
            .matrix
            .0
            .to_m33()
            .map_rect(Rect::new(x, y, width, height))
            .0
            .contains(&self.surface_data.bounds)
        {
            // if rect fully encloses canvas, erase existing content (but preserve CTM, path, etc.)
            true => {
                let mut recorder = skia_safe::PictureRecorder::new();
                let canvas = recorder.begin_recording(self.surface_data.bounds, None);
                canvas.set_matrix(&self.state.matrix.0);

                if let Some(clip) = self.state.clip.as_ref() {
                    canvas.clip_path(clip.path(), skia_safe::ClipOp::Intersect, Some(true));
                }

                self.recorder = Arc::new(parking_lot::Mutex::new(Recorder {
                    bounds: self.surface_data.bounds,
                    current: recorder,
                    is_dirty: false,
                    layers: vec![],
                    cache: None,
                    matrix: skia_safe::Matrix::new_identity(),
                    clip: None,
                }));
            }

            // otherwise, paint over the specified region but preserve overdrawn vectors
            false => {
                self.with_canvas(|canvas| {
                    let mut paint = Paint::default();
                    paint
                        .set_anti_alias(true)
                        .set_style(skia_safe::PaintStyle::Fill)
                        .set_blend_mode(BlendMode::Clear);
                    let rect = Rect::from_xywh(x, y, width, height);
                    canvas.draw_rect(&rect, &paint);
                });
            }
        }
    }

    pub fn fill_rect_xywh(&mut self, x: c_float, y: c_float, width: c_float, height: c_float) {
        let rect = Rect::from_xywh(x, y, width, height);
        self.fill_rect(&rect);
    }

    pub fn fill_rect(&mut self, rect: &Rect) {
        self.with_canvas(|canvas| {
            if let Some(paint) = self.state.paint.fill_shadow_paint(
                self.state.shadow_offset,
                self.state.shadow_color,
                self.state.shadow_blur,
            ) {
                canvas.draw_rect(rect, &paint);
            }
            canvas.draw_rect(rect, self.state.paint.fill_paint());
        });
    }

    pub fn stroke_rect_xywh(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let rect = Rect::from_xywh(x, y, width, height);
        self.stroke_rect(&rect);
    }

    pub fn stroke_rect(&mut self, rect: &Rect) {
        self.with_canvas(|canvas| {
            if let Some(paint) = &mut self.state.paint.stroke_shadow_paint(
                self.state.shadow_offset,
                self.state.shadow_color,
                self.state.shadow_blur,
            ) {
                canvas.draw_rect(rect, &paint);
            }
            canvas.draw_rect(rect, self.state.paint.stroke_paint());
        });
    }
}
