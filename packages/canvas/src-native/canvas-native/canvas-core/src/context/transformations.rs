use std::f32::consts::PI;
use std::os::raw::{c_float, c_double};

use skia_safe::{M44, Matrix, Point};

use crate::context::Context;

const DEG: f32 = 180.0 / PI;

impl Context {
    pub fn get_transform(&mut self) -> Matrix {
        self.surface.canvas().local_to_device_as_3x3()
    }

    pub fn rotate(&mut self, angle: c_float) {
        self.surface.canvas().rotate(angle * DEG, None);
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
        self.surface.canvas().concat(&transform);
    }

    pub fn transform_with_matrix(&mut self, matrix: &Matrix) {
        let mut current = self.surface.canvas().local_to_device_as_3x3();
        current.pre_concat(matrix);
        let m = M44::from(&current);
        self.surface.canvas().set_matrix(&m);
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
        let m44 = M44::from(matrix);
        self.surface.canvas().set_matrix(&m44);
    }

    pub fn set_transform_matrix(&mut self, matrix: &Matrix) {
        self.surface.canvas().reset_matrix();
        let matrix = matrix.clone();
        let m44 = M44::from(matrix);
        self.surface.canvas().set_matrix(&m44);
    }

    pub fn reset_transform(&mut self) {
        self.surface.canvas().reset_matrix();
    }
}
