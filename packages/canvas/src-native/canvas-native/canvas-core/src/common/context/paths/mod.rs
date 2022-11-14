use std::os::raw::c_float;

use crate::common::context::Context;
use crate::common::context::paths::path::Path;

pub mod path;

impl Context {
    #[inline(always)]
    pub fn begin_path(&mut self) {
        self.path.begin_path();
    }

    #[inline(always)]
    pub fn close_path(&mut self) {
        self.path.close_path();
    }

    #[inline(always)]
    pub fn move_to(&mut self, x: c_float, y: c_float) {
        self.path.move_to(x, y);
    }

    #[inline(always)]
    pub fn line_to(&mut self, x: c_float, y: c_float) {
        self.path.line_to(x, y);
    }

    #[inline(always)]
    pub fn bezier_curve_to(
        &mut self,
        cp1x: c_float,
        cp1y: c_float,
        cp2x: c_float,
        cp2y: c_float,
        x: c_float,
        y: c_float,
    ) {
        self.path.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    #[inline(always)]
    pub fn quadratic_curve_to(&mut self, cpx: c_float, cpy: c_float, x: c_float, y: c_float) {
        self.path.quadratic_curve_to(cpx, cpy, x, y);
    }

    #[inline(always)]
    pub fn arc(
        &mut self,
        x: c_float,
        y: c_float,
        radius: c_float,
        start_angle: c_float,
        end_angle: c_float,
        anticlockwise: bool,
    ) {
        let mut path = Path::new();
        path.add_ellipse(
            (x, y),
            (radius, radius),
            0.0,
            start_angle,
            end_angle,
            anticlockwise,
        );
        let transform = self.get_transform();
        self.path.path.add_path(&path.path().with_transform(&transform), (0, 0), skia_safe::path::AddPathMode::Extend);
        // self.path
        //     .arc(x, y, radius, start_angle, end_angle, anticlockwise);
    }

    #[inline(always)]
    pub fn arc_to(&mut self, x1: c_float, y1: c_float, x2: c_float, y2: c_float, radius: c_float) {
        self.path.arc_to(x1, y1, x2, y2, radius);
    }

    #[inline(always)]
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
        let mut path = Path::new();
        path.add_ellipse(
            skia_safe::Point::new(x, y),
            skia_safe::Point::new(radius_x, radius_y),
            rotation,
            start_angle,
            end_angle,
            anticlockwise,
        );

        let transform = self.get_transform();
        self.path.path.add_path(&path.path().with_transform(&transform), (0, 0), skia_safe::path::AddPathMode::Extend);
        // self.path.ellipse(
        //     x,
        //     y,
        //     radius_x,
        //     radius_y,
        //     rotation,
        //     start_angle,
        //     end_angle,
        //     anticlockwise,
        // );
    }

    #[inline(always)]
    pub fn rect(&mut self, x: c_float, y: c_float, width: c_float, height: c_float) {

        let rect = skia_safe::Rect::from_xywh(x, y, width, height);
        let quad = self.get_transform().map_rect_to_quad(rect);
        self.path.path.move_to(quad[0]);
        self.path.path.line_to(quad[1]);
        self.path.path.line_to(quad[2]);
        self.path.path.line_to(quad[3]);
        self.path.path.close();


       // self.path.rect(x, y, width, height);
    }

    #[inline(always)]
    pub fn round_rect(&mut self, x: c_float, y: c_float, width: c_float, height: c_float, radii: [c_float; 8]) {
        self.path.round_rect(x, y, width, height, radii);
    }
}
