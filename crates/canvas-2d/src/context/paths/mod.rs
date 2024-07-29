use std::os::raw::c_float;

use crate::context::Context;

pub mod path;

impl Context {
    pub fn begin_path(&mut self) {
        self.path.begin_path();
    }

    pub fn close_path(&mut self) {
        self.path.close_path();
    }

    pub fn move_to(&mut self, x: c_float, y: c_float) {
        self.path.move_to(x, y);
    }

    pub fn line_to(&mut self, x: c_float, y: c_float) {
        self.path.line_to(x, y);
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
        self.path.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    pub fn quadratic_curve_to(&mut self, cpx: c_float, cpy: c_float, x: c_float, y: c_float) {
        self.path.quadratic_curve_to(cpx, cpy, x, y);
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
        self.path
            .arc(x, y, radius, start_angle, end_angle, anticlockwise);
    }

    pub fn arc_to(&mut self, x1: c_float, y1: c_float, x2: c_float, y2: c_float, radius: c_float) {
        self.path.arc_to(x1, y1, x2, y2, radius);
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
        self.path.ellipse(
            x,
            y,
            radius_x,
            radius_y,
            rotation,
            start_angle,
            end_angle,
            anticlockwise,
        );
    }

    pub fn rect(&mut self, x: c_float, y: c_float, width: c_float, height: c_float) {
        self.path.rect(x, y, width, height);
    }

    pub fn round_rect(
        &mut self,
        x: c_float,
        y: c_float,
        width: c_float,
        height: c_float,
        radii: &[c_float],
    ) {
        self.path.round_rect(x, y, width, height, radii);
    }
}
