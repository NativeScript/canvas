use std::f32::consts::PI;
use std::os::raw::c_float;

use skia_safe::{Point, Rect};

use crate::common::context::matrix::Matrix;
use crate::common::utils::geometry::{almost_equal, to_degrees};

#[derive(Clone)]
pub struct Path {
    pub(crate) path: skia_safe::Path,
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

    pub fn make_scale(&mut self, (sx, sy): (f32, f32)) -> Self {
        Self {
            path: self.path.make_scale((sx, sy)),
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

    fn init(&mut self, _x: f32, _y: f32) {
        if self.path.is_empty() {
            //   self.path.move_to(Point::new(x, y));
        }
    }

    fn add_ellipse(
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
                self.path.add_path(path.path(), Point::new(0.0, 0.0), None);
            }
            Some(matrix) => {
                let matrix_2d = matrix.matrix.to_m33();
                self.path.add_path_matrix(path.path(), &matrix_2d, None);
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
        self.init(x1, y1);
        self.path
            .arc_to_tangent(Point::new(x1, y1), Point::new(x2, y2), radius);
    }

    pub fn begin_path(&mut self) {
        if !self.path.is_empty() {
            self.path.reset();
        }
    }

    pub fn move_to(&mut self, x: c_float, y: c_float) {
        self.path.move_to(Point::new(x, y));
    }

    pub fn line_to(&mut self, x: c_float, y: c_float) {
        self.init(x, y);
        self.path.line_to(Point::new(x, y));
    }

    pub fn close_path(&mut self) {
        if !self.path.is_empty() {
            self.path.close();
        }
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
        self.init(x, y);
        self.path.cubic_to(
            Point::new(cp1x, cp1y),
            Point::new(cp2x, cp2y),
            Point::new(x, y),
        );
    }

    pub fn quadratic_curve_to(&mut self, cpx: c_float, cpy: c_float, x: c_float, y: c_float) {
        self.init(x, y);
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
        self.path.add_rect(&rect, None);
    }
}
