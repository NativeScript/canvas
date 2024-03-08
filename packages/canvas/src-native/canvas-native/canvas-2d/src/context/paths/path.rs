use std::f32::consts::PI;
use std::os::raw::c_float;

use skia_safe::{PathEffect, Point, RRect, Rect};

use crate::context::drawing_paths::fill_rule::FillRule;
use crate::context::matrix::Matrix;
use crate::utils::geometry::{almost_equal, to_degrees};

#[derive(Clone)]
pub struct Path {
    pub(crate) path: skia_safe::Path,
}

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
        &self.path
    }

    pub fn path_fill_type(&self) -> PathFillType {
        self.path.fill_type().into()
    }

    pub fn set_path_fill_type(&mut self, value: PathFillType) {
        self.path.set_fill_type(value.into());
    }

    pub fn make_scale(&mut self, (sx, sy): (f32, f32)) -> Self {
        Self {
            path: self.path.make_scale((sx, sy)),
        }
    }

    pub fn with_transform(&self, matrix: &skia_safe::Matrix) -> Path {
        Self {
            path: self.path.with_transform(matrix),
        }
    }

    pub fn new() -> Self {
        Self {
            path: skia_safe::Path::default(),
        }
    }

    pub fn from_str(val: &str) -> Self {
        Self {
            path: skia_safe::Path::from_svg(val).unwrap_or(skia_safe::Path::default()),
        }
    }

    pub fn from_path(path: &skia_safe::Path) -> Self {
        Self { path: path.clone() }
    }

    fn scoot(&mut self, x: f32, y: f32) {
        if self.path.is_empty() {
            self.path.move_to(Point::new(x, y));
        }
    }

    pub fn set_fill_type(&mut self, fill_type: FillRule) {
        self.path.set_fill_type(fill_type.to_fill_type());
    }

    #[inline(always)]
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

        // Based off of AdjustEndAngle in Chrome.
        if !ccw && (end_angle - start_angle) >= tau {
            end_angle = start_angle + tau; // Draw complete ellipse
        } else if ccw && (start_angle - end_angle) >= tau {
            end_angle = start_angle - tau; // Draw complete ellipse
        } else if !ccw && start_angle > end_angle {
            end_angle = start_angle + (tau - (start_angle - end_angle) % tau);
        } else if ccw && start_angle < end_angle {
            end_angle = start_angle - (tau - (end_angle - start_angle) % tau);
        }

        // Based off of Chrome's implementation in
        // https://cs.chromium.org/chromium/src/third_party/blink/renderer/platform/graphics/path.cc
        // of note, can't use addArc or addOval because they close the arc, which
        // the spec says not to do (unless the user explicitly calls closePath).
        // This throws off points being in/out of the arc.
        let oval = Rect::new(x - x_radius, y - y_radius, x + x_radius, y + y_radius);
        let mut rotated = skia_safe::Matrix::new_identity();
        rotated
            .pre_translate((x, y))
            .pre_rotate(to_degrees(rotation), None)
            .pre_translate((-x, -y));
        let unrotated = rotated.invert().unwrap();

        self.path.transform(&unrotated);

        // draw in 2 180 degree segments because trying to draw all 360 degrees at once
        // draws nothing.
        let sweep_deg = to_degrees(end_angle - start_angle);
        let start_deg = to_degrees(start_angle);
        if almost_equal(sweep_deg.abs(), 360.0) {
            let half_sweep = sweep_deg / 2.0;
            self.path.arc_to(oval, start_deg, half_sweep, false);
            self.path
                .arc_to(oval, start_deg + half_sweep, half_sweep, false);
        } else {
            self.path.arc_to(oval, start_deg, sweep_deg, false);
        }

        self.path.transform(&rotated);
    }

    pub fn add_path(&mut self, path: &Path, matrix: Option<&Matrix>) {
        match matrix {
            None => {
                self.path.add_path_matrix(
                    path.path(),
                    &skia_safe::Matrix::new_identity(),
                    skia_safe::path::AddPathMode::Append,
                );
            }
            Some(matrix) => {
                let matrix_2d = matrix.to_m33();
                self.path.add_path_matrix(
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
        self.path
            .arc_to_tangent(Point::new(x1, y1), Point::new(x2, y2), radius);
    }

    pub fn begin_path(&mut self) {
        let mut new_path = skia_safe::Path::default();
        self.path.swap(&mut new_path);
    }

    pub fn move_to(&mut self, x: c_float, y: c_float) {
        self.path.move_to(Point::new(x, y));
    }

    pub fn line_to(&mut self, x: c_float, y: c_float) {
        // web impl
        let point = Point::new(x, y);
        if self.path.is_empty() {
            self.path.move_to(point);
        }
        self.path.line_to(point);
    }

    pub fn close_path(&mut self) {
        self.path.close();
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
        self.path.cubic_to(
            Point::new(cp1x, cp1y),
            Point::new(cp2x, cp2y),
            Point::new(x, y),
        );
    }

    pub fn quadratic_curve_to(&mut self, cpx: c_float, cpy: c_float, x: c_float, y: c_float) {
        self.path.quad_to(Point::new(cpx, cpy), Point::new(x, y));
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
        self.path.add_rect(&rect, Some((direction, 0)));
    }

    pub fn round_rect(
        &mut self,
        x: c_float,
        y: c_float,
        width: c_float,
        height: c_float,
        radii: &[c_float],
    ) {
        if radii.len() == 8 {
            let rect = Rect::from_xywh(x, y, width, height);
            let radii: Vec<Point> = radii.chunks(2).map(|xy| Point::new(xy[0], xy[1])).collect();
            let rrect = RRect::new_rect_radii(rect, &[radii[0], radii[1], radii[2], radii[3]]);
            let direction = if width.signum() == height.signum() {
                skia_safe::PathDirection::CW
            } else {
                skia_safe::PathDirection::CCW
            };
            self.path.add_rrect(rrect, Some((direction, 0)));
        }
    }

    pub fn trim(&mut self, start: f32, end: f32) {
        if start != 0. && end != 1. {
            if let Some(effect) = PathEffect::trim(start, end, None) {
                if let Some((mut path, _)) = effect.filter_path(
                    &self.path,
                    &skia_safe::StrokeRec::new_hairline(),
                    Rect::default(),
                ) {
                    self.path.swap(&mut path);
                }
            }
        }
    }
}
