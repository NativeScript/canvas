use std::f32::consts::PI;

use skia_safe::Matrix;

use crate::context::Context;

const DEG: f32 = 180.0 / PI;

impl Context {
    pub fn get_transform(&self) -> crate::context::matrix::Matrix {
        self.state.matrix.clone()
    }
    pub fn rotate(&mut self, angle: f32) {
        self.with_matrix(|mat| {
            let matrix = skia_safe::M44::from(Matrix::rotate_deg(angle * DEG));
            mat.0.pre_concat(&matrix);
            mat
        });
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        self.with_matrix(|mat| {
            mat.0.pre_scale(x, y);
            mat
        });
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.with_matrix(|mat| {
            mat.0.pre_translate(x, y, None);
            mat
        });
    }

    pub fn transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        let affine = [a, b, c, d, e, f];
        let transform = Matrix::from_affine(&affine);
        self.with_matrix(|mat| {
            let matrix = skia_safe::M44::from(transform);
            mat.0.pre_concat(&matrix);
            mat
        });
    }

    pub fn transform_with_matrix(&mut self, matrix: &Matrix) {
        self.with_matrix(|mat| {
            let matrix = skia_safe::M44::from(matrix);
            mat.0.pre_concat(&matrix);
            mat
        });
    }

    pub fn set_transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        let affine = [a, b, c, d, e, f];
        let matrix = Matrix::from_affine(&affine);

        self.with_matrix(|mat| {
            let matrix = skia_safe::M44::from(matrix);
            mat.0.pre_concat(&matrix);
            mat
        });
    }

    pub fn set_transform_matrix(&mut self, matrix: &crate::context::matrix::Matrix) {
        let matrix = matrix.clone();

        self.with_matrix(|mat| {
            mat.0 = matrix.0;
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
