use std::cell::UnsafeCell;
use std::f32::consts::PI;
use std::f32::EPSILON;
use std::os::raw::c_float;

use skia_safe::{PathBuilder, PathEffect, Point, RRect, Rect};

use crate::context::drawing_paths::fill_rule::FillRule;
use crate::context::matrix::Matrix;
use crate::utils::geometry::to_degrees;

/// A path that lazily materializes from a PathBuilder.
/// Operations accumulate in `builder` without copying.
/// The `skia_safe::Path` is only produced when `path()` is called.
pub struct Path {
    builder: PathBuilder,
    /// Cached materialized path; invalidated on mutation.
    /// Wrapped in UnsafeCell for interior mutability in `path()`.
    cached_path: UnsafeCell<Option<skia_safe::Path>>,
}

impl Clone for Path {
    fn clone(&self) -> Self {
        // Materialize before cloning to get a cheap Path clone
        let path = self.path_snapshot();
        Self {
            builder: PathBuilder::new_path(&path),
            cached_path: UnsafeCell::new(Some(path)),
        }
    }
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
    /// Returns a reference to the materialized skia Path (caches it).
    pub fn path(&self) -> &skia_safe::Path {
        // SAFETY: Path is not Sync, so no concurrent access is possible.
        // We only write to the cache when it's None, and we return a shared ref.
        let cached = unsafe { &mut *self.cached_path.get() };
        if cached.is_none() {
            *cached = Some(self.builder.clone().detach());
        }
        cached.as_ref().unwrap()
    }

    /// Same as path() but takes &mut self, avoiding unsafe.
    fn path_mut(&mut self) -> &skia_safe::Path {
        let cached = self.cached_path.get_mut();
        if cached.is_none() {
            *cached = Some(self.builder.clone().detach());
        }
        cached.as_ref().unwrap()
    }

    /// Materialize and return a clone of the path (for operations that need ownership).
    fn path_snapshot(&self) -> skia_safe::Path {
        let cached = unsafe { &*self.cached_path.get() };
        if let Some(ref path) = cached {
            path.clone()
        } else {
            self.builder.clone().detach()
        }
    }

    /// Invalidate the cached materialized path after mutations.
    #[inline(always)]
    fn invalidate(&mut self) {
        *self.cached_path.get_mut() = None;
    }

    pub fn path_fill_type(&self) -> PathFillType {
        self.builder.fill_type().into()
    }

    pub fn set_path_fill_type(&mut self, value: PathFillType) {
        self.builder.set_fill_type(value.into());
        self.invalidate();
    }

    pub fn fill_type(&self) -> skia_safe::PathFillType {
        self.builder.fill_type()
    }

    pub fn set_fill_type_raw(&mut self, value: skia_safe::PathFillType) {
        self.builder.set_fill_type(value);
        self.invalidate();
    }

    pub fn make_scale(&mut self, (sx, sy): (f32, f32)) -> Self {
        let path = self.path_snapshot().make_scale((sx, sy));
        Self {
            builder: PathBuilder::new_path(&path),
            cached_path: UnsafeCell::new(Some(path)),
        }
    }

    pub fn with_transform(&self, matrix: &skia_safe::Matrix) -> Path {
        let path = self.path_snapshot().with_transform(matrix);
        Self {
            builder: PathBuilder::new_path(&path),
            cached_path: UnsafeCell::new(Some(path)),
        }
    }

    pub fn new() -> Self {
        Self {
            builder: PathBuilder::default(),
            cached_path: UnsafeCell::new(None),
        }
    }

    pub fn from_str(val: &str) -> Self {
        let path = skia_safe::Path::from_svg(val).unwrap_or(skia_safe::Path::default());
        Self {
            builder: PathBuilder::new_path(&path),
            cached_path: UnsafeCell::new(Some(path)),
        }
    }

    pub fn from_path(path: &skia_safe::Path) -> Self {
        Self {
            builder: PathBuilder::new_path(path),
            cached_path: UnsafeCell::new(Some(path.clone())),
        }
    }

    pub fn set_fill_type(&mut self, fill_type: FillRule) {
        self.builder.set_fill_type(fill_type.to_fill_type());
        self.invalidate();
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

        // Fast path: skip matrix creation/inversion when rotation is zero (common for arc())
        let has_rotation = rotation != 0.0;
        if has_rotation {
            let mut rotated = skia_safe::matrix::Matrix::new_identity();
            rotated
                .pre_translate((x, y))
                .pre_rotate(to_degrees(rotation), None)
                .pre_translate((-x, -y));
            self.builder.transform(&rotated.invert().unwrap());

            self.add_ellipse_arcs(oval, start_angle, end_angle);

            self.builder.transform(&rotated);
        } else {
            self.add_ellipse_arcs(oval, start_angle, end_angle);
        }
        self.invalidate();
    }

    #[inline(always)]
    fn add_ellipse_arcs(&mut self, oval: Rect, start_angle: f32, end_angle: f32) {
        // rounding degrees to 4 decimals eliminates ambiguity from f32 imprecision dealing with radians
        let sweep_deg = (to_degrees(end_angle - start_angle) * 10000.0).round() / 10000.0;
        let start_deg = (to_degrees(start_angle) * 10000.0).round() / 10000.0;

        // Full circle: use add_oval for a single efficient Skia path verb instead of two arc_to calls
        if sweep_deg >= 360.0 - EPSILON {
            self.builder.add_oval(oval, skia_safe::PathDirection::CW, 0);
        } else if sweep_deg <= -360.0 + EPSILON {
            self.builder.add_oval(oval, skia_safe::PathDirection::CCW, 0);
        } else {
            self.builder.arc_to(oval, start_deg, sweep_deg, false);
        }
    }

    pub fn add_path(&mut self, path: &Path, matrix: Option<&Matrix>) {
        match matrix {
            None => {
                self.builder.add_path_with_transform(
                    path.path(),
                    &skia_safe::Matrix::new_identity(),
                    skia_safe::path::AddPathMode::Append,
                );
            }
            Some(matrix) => {
                let matrix_2d = matrix.to_m33();
                self.builder.add_path_with_transform(
                    path.path(),
                    &matrix_2d,
                    skia_safe::path::AddPathMode::Append,
                );
            }
        }
        self.invalidate();
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
        self.builder.arc_to_tangent(Point::new(x1, y1), Point::new(x2, y2), radius);
        self.invalidate();
    }

    #[inline]
    pub fn begin_path(&mut self) {
        if self.builder.is_empty() && self.cached_path.get_mut().is_none() {
            return;
        }
        self.builder = PathBuilder::default();
        self.invalidate();
    }

    #[inline]
    pub fn move_to(&mut self, x: c_float, y: c_float) {
        self.builder.move_to(Point::new(x, y));
        self.invalidate();
    }

    #[inline]
    pub fn line_to(&mut self, x: c_float, y: c_float) {
        let point = Point::new(x, y);
        if self.builder.is_empty() {
            self.builder.move_to(point);
        }
        self.builder.line_to(point);
        self.invalidate();
    }

    #[inline]
    pub fn close_path(&mut self) {
        self.builder.close();
        self.invalidate();
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
        self.builder.cubic_to(
            Point::new(cp1x, cp1y),
            Point::new(cp2x, cp2y),
            Point::new(x, y),
        );
        self.invalidate();
    }

    pub fn quadratic_curve_to(&mut self, cpx: c_float, cpy: c_float, x: c_float, y: c_float) {
        self.builder.quad_to(Point::new(cpx, cpy), Point::new(x, y));
        self.invalidate();
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
        self.builder.add_rect(&rect, Some(direction), Some(0_usize));
        self.invalidate();
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
            self.builder.add_rrect(rrect, Some(direction), Some(0_usize));
            self.invalidate();
        }
    }

    pub fn trim(&mut self, start: f32, end: f32) {
        if start != 0. && end != 1. {
            if let Some(effect) = PathEffect::trim(start, end, None) {
                let path = self.path_mut();
                if let Some((trimmed, _)) = effect.filter_path(
                    path,
                    &skia_safe::StrokeRec::new_hairline(),
                    Rect::default(),
                ) {
                    let trimmed: skia_safe::Path = trimmed.into();
                    self.builder = PathBuilder::new_path(&trimmed);
                    *self.cached_path.get_mut() = Some(trimmed);
                }
            }
        }
    }
}
