use std::f32::consts::PI;

use skia_safe::Matrix;

use crate::context::Context;

const DEG: f32 = 180.0 / PI;

impl Context {
    pub fn get_transform(&self) -> crate::context::matrix::Matrix {
        crate::context::matrix::Matrix::from(&self.state.matrix)
    }
    pub fn rotate(&mut self, angle: f32) {
        self.with_matrix(|mat| {
            mat.pre_rotate(angle * DEG, None);
            mat
        });
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        self.with_matrix(|mat| {
            mat.pre_scale((x, y), None);
            mat
        });
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.with_matrix(|mat| {
            mat.pre_translate(skia_safe::Vector::new(x, y));
            mat
        });
    }

    pub fn transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        let affine = [a, b, c, d, e, f];
        let transform = Matrix::from_affine(&affine);
        self.with_matrix(|mat| {
            mat.pre_concat(&transform);
            mat
        });
    }

    pub fn transform_with_matrix(&mut self, matrix: &Matrix) {
        self.with_matrix(|mat| {
            mat.pre_concat(&matrix);
            mat
        });
    }

    pub fn set_transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        let affine = [a, b, c, d, e, f];
        self.with_matrix(|mat| {
            mat.set_affine(&affine);
            mat
        });
    }

    pub fn set_transform_matrix(&mut self, matrix: &crate::context::matrix::Matrix) {
        let matrix = matrix.clone();

        self.with_matrix(|mat| {
            *mat = matrix.0.to_m33();
            mat
        });
    }

    pub fn reset_transform(&mut self) {
        self.with_matrix(|mat| {
            mat.reset();
            mat
        });
    }
}
