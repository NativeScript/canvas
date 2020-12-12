use std::f32::consts::PI;
use std::os::raw::c_float;

use skia_safe::{Matrix, Point};

use crate::common::context::Context;

impl Context {
    pub fn get_transform(&mut self) -> Matrix {
        self.surface.canvas().total_matrix()
    }

    pub fn rotate(&mut self, angle: c_float) {
        self.surface.canvas().rotate(angle * (180.0 / PI), None);
    }

    pub fn scale(&mut self, x: c_float, y: c_float) {
        self.surface.canvas().scale((x, y));
    }

    pub fn translate(&mut self, x: c_float, y: c_float) {
        self.surface.canvas().translate(Point::new(x, y));
    }

    pub fn transform(
        &mut self,
        a: c_float,
        b: c_float,
        c: c_float,
        d: c_float,
        e: c_float,
        f: c_float,
    ) {
        let affine = [a, b, c, d, e, f];
        let transform = Matrix::from_affine(&affine);
        let mut matrix = self.surface.canvas().total_matrix().clone();
        matrix.pre_concat(&transform);
        self.surface.canvas().set_matrix(&matrix);
    }

    pub fn set_transform(
        &mut self,
        a: c_float,
        b: c_float,
        c: c_float,
        d: c_float,
        e: c_float,
        f: c_float,
    ) {
        let affine = [a, b, c, d, e, f];
        let matrix = Matrix::from_affine(&affine);
        self.surface.canvas().reset_matrix();
        self.surface.canvas().set_matrix(&matrix);
    }

    pub fn set_transform_matrix(&mut self, matrix: &Matrix) {
        self.surface.canvas().reset_matrix();
        self.surface.canvas().set_matrix(matrix);
    }

    pub fn reset_transform(&mut self) {
        self.surface.canvas().reset_matrix();
    }
}
