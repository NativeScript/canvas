use std::f32::consts::PI;
use std::f32::EPSILON;
use std::os::raw::c_float;

use skia_safe::{PathEffect, Point, RRect, Rect};

use crate::context::drawing_paths::fill_rule::FillRule;
use crate::context::matrix::Matrix;
use crate::utils::geometry::to_degrees;

#[derive(Clone)]
pub struct Path(pub(crate) skia_safe::Path);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PathFillType {
    Winding = 0,
    EvenOdd = 1,
    InverseWinding = 2,
    InverseEvenOdd = 3,
}

impl Into<PathFillType> for skia_safe::PathFillType {
    fn into(self) -> PathFillType {
        match self {
            skia_safe::PathFillType::Winding => PathFillType::Winding,
            skia_safe::PathFillType::EvenOdd => PathFillType::EvenOdd,
            skia_safe::PathFillType::InverseWinding => PathFillType::InverseWinding,
            skia_safe::PathFillType::InverseEvenOdd => PathFillType::InverseEvenOdd,
        }
    }
}

impl Into<skia_safe::PathFillType> for PathFillType {
    fn into(self) -> skia_safe::PathFillType {
        match self {
            PathFillType::Winding => skia_safe::PathFillType::Winding,
            PathFillType::EvenOdd => skia_safe::PathFillType::EvenOdd,
            PathFillType::InverseWinding => skia_safe::PathFillType::InverseWinding,
            PathFillType::InverseEvenOdd => skia_safe::PathFillType::InverseEvenOdd,
        }
    }
}
impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl Path {
    pub fn path(&self) -> &skia_safe::Path {
        &self.0
    }

    pub fn path_fill_type(&self) -> PathFillType {
        self.0.fill_type().into()
    }

    pub fn set_path_fill_type(&mut self, value: PathFillType) {
        self.0.set_fill_type(value.into());
    }

    pub fn make_scale(&mut self, (sx, sy): (f32, f32)) -> Self {
        Self(self.0.make_scale((sx, sy)))
    }

    pub fn with_transform(&self, matrix: &skia_safe::Matrix) -> Path {
        Self(self.0.with_transform(matrix))
    }

    pub fn new() -> Self {
        Self(skia_safe::Path::default())
    }

    pub fn from_str(val: &str) -> Self {
        Self(skia_safe::Path::from_svg(val).unwrap_or(skia_safe::Path::default()))
    }

    pub fn from_path(path: &skia_safe::Path) -> Self {
        Self(path.clone())
    }

    pub fn set_fill_type(&mut self, fill_type: FillRule) {
        self.0.set_fill_type(fill_type.to_fill_type());
    }

    pub(crate) fn add_ellipse(
        &mut self,
        origin: impl Into<Point>,
        radii: impl Into<Point>,
        rotation: f32,
        start_angle: f32,
        end_angle: f32,
        ccw: bool,
    ) {
        let Point { x, y } = origin.into();
        let Point {
            x: x_radius,
            y: y_radius,
        } = radii.into();

        // based off of CanonicalizeAngle in Chrome
        let tau = 2.0 * PI;
        let mut new_start_angle = start_angle % tau;
        if new_start_angle < 0.0 {
            new_start_angle += tau;
        }
        let delta = new_start_angle - start_angle;
        let start_angle = new_start_angle;
        let mut end_angle = end_angle + delta;

        // Originally based off of AdjustEndAngle in Chrome, but does not limit to 360 degree sweep.
        if !ccw && start_angle > end_angle {
            end_angle = start_angle + (tau - (start_angle - end_angle) % tau);
        } else if ccw && start_angle < end_angle {
            end_angle = start_angle - (tau - (end_angle - start_angle) % tau);
        }

        let oval = Rect::new(x - x_radius, y - y_radius, x + x_radius, y + y_radius);

        let mut rotated = skia_safe::matrix::Matrix::new_identity();
        rotated
            .pre_translate((x, y))
            .pre_rotate(to_degrees(rotation), None)
            .pre_translate((-x, -y));

        self.0.transform(&rotated.invert().unwrap());
        {
            // Based off of Chrome's implementation in
            // https://cs.chromium.org/chromium/src/third_party/blink/renderer/platform/graphics/path.cc
            // of note, can't use addArc or addOval because they close the arc, which
            // the spec says not to do (unless the user explicitly calls closePath).
            // This throws off points being in/out of the arc.

            // rounding degrees to 4 decimals eliminates ambiguity from f32 imprecision dealing with radians
            let mut sweep_deg = (to_degrees(end_angle - start_angle) * 10000.0).round() / 10000.0;
            let mut start_deg = (to_degrees(start_angle) * 10000.0).round() / 10000.0;

            // draw 360° ellipses in two 180° segments; trying to draw the full ellipse at once draws nothing.
            if sweep_deg >= 360.0 - EPSILON {
                self.0.arc_to(oval, start_deg, 180.0, false);
                self.0.arc_to(oval, start_deg + 180.0, 180.0, false);
            } else if sweep_deg <= -360.0 + EPSILON {
                self.0.arc_to(oval, start_deg, -180.0, false);
                self.0.arc_to(oval, start_deg - 180.0, -180.0, false);
            } else {
                // Draw incomplete (< 360°) ellipses in a single arc.
                self.0.arc_to(oval, start_deg, sweep_deg, false);
            }
        }
        self.0.transform(&rotated);
    }

    pub fn add_path(&mut self, path: &Path, matrix: Option<&Matrix>) {
        match matrix {
            None => {
                self.0.add_path_matrix(
                    path.path(),
                    &skia_safe::Matrix::new_identity(),
                    skia_safe::path::AddPathMode::Append,
                );
            }
            Some(matrix) => {
                let matrix_2d = matrix.to_m33();
                self.0.add_path_matrix(
                    path.path(),
                    &matrix_2d,
                    skia_safe::path::AddPathMode::Append,
                );
            }
        }
    }

    pub fn arc(
        &mut self,
        x: c_float,
        y: c_float,
        radius: c_float,
        start_angle: c_float,
        end_angle: c_float,
        anticlockwise: bool,
    ) {
        self.add_ellipse(
            (x, y),
            (radius, radius),
            0.0,
            start_angle,
            end_angle,
            anticlockwise,
        );
    }

    pub fn arc_to(&mut self, x1: c_float, y1: c_float, x2: c_float, y2: c_float, radius: c_float) {
        self.0
            .arc_to_tangent(Point::new(x1, y1), Point::new(x2, y2), radius);
    }

    #[inline]
    pub fn begin_path(&mut self) {
        if self.0.is_empty() {
            return;
        }
        let mut new_path = skia_safe::Path::default();
        self.0.swap(&mut new_path);
    }

    #[inline]
    pub fn move_to(&mut self, x: c_float, y: c_float) {
        self.0.move_to(Point::new(x, y));
    }

    #[inline]
    pub fn line_to(&mut self, x: c_float, y: c_float) {
        // web impl
        let point = Point::new(x, y);
        if self.0.is_empty() {
            self.0.move_to(point);
        }
        self.0.line_to(point);
    }

    #[inline]
    pub fn close_path(&mut self) {
        self.0.close();
    }

    pub fn bezier_curve_to(
        &mut self,
        cp1x: c_float,
        cp1y: c_float,
        cp2x: c_float,
        cp2y: c_float,
        x: c_float,
        y: c_float,
    ) {
        self.0.cubic_to(
            Point::new(cp1x, cp1y),
            Point::new(cp2x, cp2y),
            Point::new(x, y),
        );
    }

    pub fn quadratic_curve_to(&mut self, cpx: c_float, cpy: c_float, x: c_float, y: c_float) {
        self.0.quad_to(Point::new(cpx, cpy), Point::new(x, y));
    }

    pub fn ellipse(
        &mut self,
        x: c_float,
        y: c_float,
        radius_x: c_float,
        radius_y: c_float,
        rotation: c_float,
        start_angle: c_float,
        end_angle: c_float,
        anticlockwise: bool,
    ) {
        self.add_ellipse(
            Point::new(x, y),
            Point::new(radius_x, radius_y),
            rotation,
            start_angle,
            end_angle,
            anticlockwise,
        )
    }

    pub fn rect(&mut self, x: c_float, y: c_float, width: c_float, height: c_float) {
        let rect = Rect::from_xywh(x, y, width, height);
        let direction = if width.signum() == height.signum() {
            skia_safe::PathDirection::CW
        } else {
            skia_safe::PathDirection::CCW
        };
        self.0.add_rect(&rect, Some((direction, 0)));
    }

    pub fn round_rect(
        &mut self,
        x: c_float,
        y: c_float,
        width: c_float,
        height: c_float,
        radii: &[c_float],
    ) {
        let rect = Rect::from_xywh(x, y, width, height);
        let mut rrect: Option<RRect> = None;
        if radii.len() == 8 {
            let radii: Vec<Point> = radii.chunks(2).map(|xy| Point::new(xy[0], xy[1])).collect();
            rrect = Some(RRect::new_rect_radii(
                rect,
                &[radii[0], radii[1], radii[2], radii[3]],
            ));
        } else if radii.len() == 4 {
            rrect = Some(RRect::new_rect_radii(
                rect,
                &[
                    Point::new(radii[0], radii[0]),
                    Point::new(radii[1], radii[1]),
                    Point::new(radii[2], radii[2]),
                    Point::new(radii[3], radii[3]),
                ],
            ));
        } else if radii.len() == 3 {
            rrect = Some(RRect::new_rect_radii(
                rect,
                &[
                    Point::new(radii[0], radii[0]),
                    Point::new(radii[1], radii[1]),
                    Point::new(radii[2], radii[2]),
                    Point::new(radii[1], radii[1]),
                ],
            ));
        } else if radii.len() == 2 {
            rrect = Some(RRect::new_rect_radii(
                rect,
                &[
                    Point::new(radii[0], radii[0]),
                    Point::new(radii[1], radii[1]),
                    Point::new(radii[0], radii[0]),
                    Point::new(radii[1], radii[1]),
                ],
            ));
        } else if radii.len() == 1 {
            rrect = Some(RRect::new_rect_radii(
                rect,
                &[
                    Point::new(radii[0], radii[0]),
                    Point::new(radii[0], radii[0]),
                    Point::new(radii[0], radii[0]),
                    Point::new(radii[0], radii[0]),
                ],
            ));
        }

        if let Some(rrect) = rrect {
            let direction = if width.signum() == height.signum() {
                skia_safe::PathDirection::CW
            } else {
                skia_safe::PathDirection::CCW
            };
            self.0.add_rrect(rrect, Some((direction, 0)));
        }
    }

    pub fn trim(&mut self, start: f32, end: f32) {
        if start != 0. && end != 1. {
            if let Some(effect) = PathEffect::trim(start, end, None) {
                if let Some((mut path, _)) = effect.filter_path(
                    &self.0,
                    &skia_safe::StrokeRec::new_hairline(),
                    Rect::default(),
                ) {
                    self.0.swap(&mut path);
                }
            }
        }
    }
}
