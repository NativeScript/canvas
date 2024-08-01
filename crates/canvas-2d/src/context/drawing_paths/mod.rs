use std::borrow::BorrowMut;

use skia_safe::{ClipOp, Matrix, Point};

use crate::context::Context;
use crate::context::drawing_paths::fill_rule::FillRule;
use crate::context::paths::path::Path;

pub mod fill_rule;

impl Context {
    fn fill_or_stroke(&self, is_fill: bool, path: Option<&mut Path>, fill_rule: Option<FillRule>) {
        let paint;
        if is_fill {
            paint = self.state.paint.fill_paint();
        } else {
            paint = self.state.paint.stroke_paint();
        }

        if let Some(rule) = fill_rule {
            let mut path = path.map(|path| path.clone()).unwrap_or(self.path.clone());
            path.0.set_fill_type(rule.to_fill_type());
            let path = path.path();

            self.render_to_canvas(paint,|canvas, paint| {
                if let Some(paint) = self.state.paint.fill_shadow_paint(
                    self.state.shadow_offset,
                    self.state.shadow_color,
                    self.state.shadow_blur,
                ) {
                    canvas.draw_path(path, &paint);
                }

                canvas.draw_path(path, paint);
            });
        } else {
            let mut path = path.map(|path| path.clone()).unwrap_or(self.path.clone());
            path.0.set_fill_type(skia_safe::path::FillType::Winding);
            let path = path.path();

            self.render_to_canvas(paint,|canvas, paint| {
                if is_fill {
                    if let Some(paint) = self.state.paint.fill_shadow_paint(
                        self.state.shadow_offset,
                        self.state.shadow_color,
                        self.state.shadow_blur,
                    ) {
                        canvas.draw_path(path, &paint);
                    }
                } else {
                    if let Some(paint) = self.state.paint.stroke_shadow_paint(
                        self.state.shadow_offset,
                        self.state.shadow_color,
                        self.state.shadow_blur,
                    ) {
                        canvas.draw_path(path, &paint);
                    }
                }

                canvas.draw_path(path, paint);
            });
        }
    }

    pub fn fill(&mut self, path: Option<&mut Path>) {
        self.fill_or_stroke(true, path, None);
    }

    pub fn fill_rule(&mut self, path: Option<&mut Path>, fill_rule: FillRule) {
        self.fill_or_stroke(true, path, Some(fill_rule));
    }

    pub fn stroke(&mut self, path: Option<&mut Path>) {
        self.fill_or_stroke(false, path, None);
    }

    pub fn clip(&mut self, path: Option<&mut Path>, fill_rule: Option<FillRule>) {
        let mut clip = path
            .map(|path| path.clone())
            .unwrap_or_else(|| self.path.clone());

        clip.set_fill_type(fill_rule.unwrap_or(FillRule::NonZero));

        self.state.clip = match &self.state.clip {
            Some(old_clip) => match old_clip
                .path()
                .op(clip.path(), skia_safe::PathOp::Intersect)
            {
                None => None,
                Some(path) => Some(Path(path)),
            },
            None => Some(clip.clone()),
        };

        self.with_recorder(|mut recorder| {
            recorder.set_clip(&self.state.clip)
        });
    }

    pub fn point_in_path(&self, path: Option<&Path>, x: f32, y: f32, rule: FillRule) -> bool {
        let path = path.unwrap_or(&self.path);

        let total_matrix = self.state.matrix;
        let invertible = is_invertible(&total_matrix);
        if !invertible {
            return false;
        }
        if !x.is_finite() || !y.is_finite() {
            return false;
        }
        let matrix = total_matrix.clone();
        let inverse = matrix.invert().unwrap();
        let point: Point = (x, y).into();
        let transformed_point = inverse.map_point(point);
        let mut path_to_compare = path.path().clone();
        path_to_compare.set_fill_type(rule.to_fill_type());
        path_to_compare.contains(transformed_point)
    }

    pub fn point_in_stroke(&self, path: Option<&Path>, x: f32, y: f32) -> bool {
        let path = path.unwrap_or(&self.path);
        let matrix = self.state.matrix;
        let invertible = is_invertible(&matrix);
        if !invertible {
            return false;
        }
        if !x.is_finite() || !y.is_finite() {
            return false;
        }
        let inverse = matrix.invert().unwrap();
        let point: Point = (x, y).into();
        let transformed_point = inverse.map_point(point);
        let path_to_compare = path.path();
        path_to_compare.contains(transformed_point)
    }
}

fn det(matrix: &Matrix) -> f32 {
    let transform = matrix.to_affine().unwrap();
    return transform[0] * transform[3] - transform[1] * transform[2];
}

fn is_invertible(matrix: &Matrix) -> bool {
    return det(matrix) != 0.0;
}
