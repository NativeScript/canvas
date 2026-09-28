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
        #[cfg(any(target_os = "macos", target_os = "ios", target_os = "visionos", target_os = "tvos"))]
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

    /// The point is device-space per spec, paths are user-space. Note `state.matrix`
    /// is not the live matrix: the transform lives on the Skia canvas.
    fn point_in_user_space(&mut self, x: f32, y: f32) -> Option<Point> {
        if !x.is_finite() || !y.is_finite() {
            return None;
        }
        let matrix = self.surface.canvas().local_to_device_as_3x3();
        if !is_invertible(&matrix) {
            return None;
        }
        matrix.invert().map(|inverse| inverse.map_point(Point::new(x, y)))
    }

    pub fn point_in_path(&mut self, path: Option<&Path>, x: f32, y: f32, rule: FillRule) -> bool {
        let transformed_point = match self.point_in_user_space(x, y) {
            Some(point) => point,
            None => return false,
        };
        let path = path.unwrap_or(&self.path);
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

    pub fn point_in_stroke(&mut self, path: Option<&Path>, x: f32, y: f32) -> bool {
        let transformed_point = match self.point_in_user_space(x, y) {
            Some(point) => point,
            None => return false,
        };
        let path = path.unwrap_or(&self.path);
        // Hit-test the stroked outline: a line has no area to contain a point.
        let mut stroke_paint = self.state.paint.stroke_paint().clone();
        stroke_paint.set_style(skia_safe::paint::Style::Stroke);
        let mut outline = skia_safe::PathBuilder::new();
        if !skia_safe::path_utils::fill_path_with_paint(
            path.path(),
            &stroke_paint,
            &mut outline,
            None,
            None,
        ) {
            return false;
        }
        outline.detach().contains(transformed_point)
    }
}

fn det(matrix: &Matrix) -> f32 {
    let transform = matrix.to_affine().unwrap();
    transform[0] * transform[3] - transform[1] * transform[2]
}

fn is_invertible(matrix: &Matrix) -> bool {
    det(matrix) != 0.0
}
