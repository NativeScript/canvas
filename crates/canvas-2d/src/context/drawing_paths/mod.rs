use crate::context::drawing_paths::fill_rule::FillRule;
use crate::context::paths::path::Path;
use crate::context::Context;
use skia_safe::{ClipOp, Color, Matrix, Point};

pub mod fill_rule;

impl Context {
    fn fill_or_stroke(
        &mut self,
        is_fill: bool,
        path: Option<&mut Path>,
        fill_rule: Option<FillRule>,
    ) {
        #[cfg(any(target_os = "macos", target_os = "ios", target_os = "visionos"))]
        let _ = unsafe { objc2_foundation::NSAutoreleasePool::new() };

        #[cfg(feature = "gl")]
        {
            if let Some(ref context) = self.gl_context {
                context.make_current();
            }
        }

        let current_rule = match path {
            None => self.path.fill_type(),
            Some(ref path) => path.fill_type(),
        };

        let fill_rule = fill_rule.unwrap_or(FillRule::default());

        let has_shadow = self.state.shadow_color != Color::TRANSPARENT
            && self.state.shadow_blur > 0.0;

        if has_shadow {
            // Slow path: need to save/modify/restore image filter + clone paint
            self.fill_or_stroke_with_shadow(is_fill, path, fill_rule, current_rule);
        } else {
            // Fast path: no shadow — skip all shadow overhead.
            let paint = if is_fill {
                self.state.paint.fill_paint.clone()
            } else {
                self.state.paint.stroke_paint.clone()
            };

            let target_rule = fill_rule.to_fill_type();
            let needs_rule_change = current_rule != target_rule;

            match path {
                Some(path) => {
                    if needs_rule_change {
                        path.set_fill_type_raw(target_rule);
                    }
                    self.render_to_canvas(&paint, |canvas, paint| {
                        canvas.draw_path(path.path(), paint);
                    });
                    if needs_rule_change {
                        path.set_fill_type_raw(current_rule);
                    }
                }
                None => {
                    if needs_rule_change {
                        self.path.set_fill_type_raw(target_rule);
                    }
                    self.render_to_canvas_with_path(&paint, |canvas, paint, path| {
                        canvas.draw_path(path.path(), paint);
                    });
                    if needs_rule_change {
                        self.path.set_fill_type_raw(current_rule);
                    }
                }
            };
        }
    }

    #[cold]
    fn fill_or_stroke_with_shadow(
        &mut self,
        is_fill: bool,
        path: Option<&mut Path>,
        fill_rule: FillRule,
        current_rule: skia_safe::PathFillType,
    ) {
        let saved_image_filter = if is_fill {
            self.state.paint.fill_paint.image_filter()
        } else {
            self.state.paint.stroke_paint.image_filter()
        };

        let current_image_filter = if is_fill {
            self.state.paint.fill_paint.image_filter()
        } else {
            self.state.paint.stroke_paint.image_filter()
        };

        let sigma = self.state.shadow_blur / 2.;

        if let Some(shadow) = skia_safe::image_filters::drop_shadow_only(
            self.state.shadow_offset,
            (sigma, sigma),
            self.state.shadow_color,
            None,
            current_image_filter,
            None,
        ) {
            if is_fill {
                self.state.paint.fill_paint.set_image_filter(shadow);
            } else {
                self.state.paint.stroke_paint.set_image_filter(shadow);
            };
        }

        let paint = if is_fill {
            self.state.paint.fill_paint.clone()
        } else {
            self.state.paint.stroke_paint.clone()
        };

        match path {
            Some(path) => {
                path.set_fill_type_raw(fill_rule.to_fill_type());
                self.render_to_canvas(&paint, |canvas, paint| {
                    canvas.draw_path(path.path(), paint);
                });
                path.set_fill_type_raw(current_rule);
            }
            None => {
                self.path.set_fill_type_raw(fill_rule.to_fill_type());
                self.render_to_canvas_with_path(&paint, |canvas, paint, path| {
                    canvas.draw_path(path.path(), paint);
                });
                self.path.set_fill_type_raw(current_rule);
            }
        };

        // Restore the saved image filter
        if is_fill {
            self.state.paint.fill_paint.set_image_filter(saved_image_filter);
        } else {
            self.state.paint.stroke_paint.set_image_filter(saved_image_filter);
        };
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
        let fill_rule = fill_rule.unwrap_or(FillRule::NonZero);
        match path {
            Some(path) => {
                let current = path.fill_type();
                path.set_fill_type(fill_rule);
                self.surface
                    .canvas()
                    .clip_path(path.path(), Some(ClipOp::Intersect), Some(true));
                path.set_fill_type_raw(current);
            }
            None => {
                let current = self.path.fill_type();
                self.path.set_fill_type(fill_rule);
                self.surface
                    .canvas()
                    .clip_path(self.path.path(), Some(ClipOp::Intersect), Some(true));
                self.path.set_fill_type_raw(current);
            }
        }
    }

    pub fn point_in_path(&self, path: Option<&Path>, x: f32, y: f32, rule: FillRule) -> bool {
        let path = path.unwrap_or(&self.path);

        let total_matrix = self.state.matrix;
        if !is_invertible(&total_matrix) {
            return false;
        }
        if !x.is_finite() || !y.is_finite() {
            return false;
        }
        let inverse = total_matrix.invert().unwrap();
        let transformed_point = inverse.map_point(Point::new(x, y));
        let target_fill = rule.to_fill_type();
        let current_fill = path.path().fill_type();
        if current_fill == target_fill {
            path.path().contains(transformed_point)
        } else {
            let mut path_to_compare = path.path().clone();
            path_to_compare.set_fill_type(target_fill);
            path_to_compare.contains(transformed_point)
        }
    }

    pub fn point_in_stroke(&self, path: Option<&Path>, x: f32, y: f32) -> bool {
        let path = path.unwrap_or(&self.path);
        let matrix = self.state.matrix;
        if !is_invertible(&matrix) {
            return false;
        }
        if !x.is_finite() || !y.is_finite() {
            return false;
        }
        let inverse = matrix.invert().unwrap();
        let transformed_point = inverse.map_point(Point::new(x, y));
        path.path().contains(transformed_point)
    }
}

fn det(matrix: &Matrix) -> f32 {
    let transform = matrix.to_affine().unwrap();
    transform[0] * transform[3] - transform[1] * transform[2]
}

fn is_invertible(matrix: &Matrix) -> bool {
    det(matrix) != 0.0
}
