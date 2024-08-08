use std::f32::consts::PI;

use skia_safe::Matrix;

use crate::context::Context;

const DEG: f32 = 180.0 / PI;

impl Context {
    pub fn get_transform(&mut self) -> crate::context::matrix::Matrix {
        // crate::context::matrix::Matrix::from(&self.state.matrix)
        let mat = self.surface.canvas().local_to_device();
        crate::context::matrix::Matrix(mat)
    }
    pub fn rotate(&mut self, angle: f32) {
        // self.with_matrix(|mat| {
        //     mat.pre_rotate(angle * DEG, None);
        //     mat
        // });
        self.surface.canvas().rotate(angle * DEG, None);
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        // self.with_matrix(|mat| {
        //     mat.pre_scale((x, y), None);
        //     mat
        // });
        self.surface.canvas().scale((x, y));
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        // self.with_matrix(|mat| {
        //     mat.pre_translate(skia_safe::Vector::new(x, y));
        //     mat
        // });
        self.surface.canvas().translate((x, y));
    }

    pub fn transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        let affine = [a, b, c, d, e, f];
        let transform = Matrix::from_affine(&affine);
        let mut current = self.surface.canvas().local_to_device_as_3x3();
        current.pre_concat(&transform);
        let current = skia_safe::M44::from(current);
        self.surface.canvas().set_matrix(
            &current
        );
        // self.with_matrix(|mat| {
        //     mat.pre_concat(&transform);
        //     mat
        // });
    }

    pub fn transform_with_matrix(&mut self, matrix: &Matrix) {
        let mut current = self.surface.canvas().local_to_device_as_3x3();
        current.pre_concat(&matrix);
        let current = skia_safe::M44::from(current);
        self.surface.canvas().set_matrix(&current);
        // self.with_matrix(|mat| {
        //     mat.pre_concat(&matrix);
        //     mat
        // });
    }

    pub fn set_transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        let affine = [a, b, c, d, e, f];
        let mat = Matrix::from_affine(&affine);
        let mat = skia_safe::M44::from(mat);
        self.surface.canvas().set_matrix(&mat);
        // self.with_matrix(|mat| {
        //     mat.set_affine(&affine);
        //     mat
        // });
    }

    pub fn set_transform_matrix(&mut self, matrix: &crate::context::matrix::Matrix) {
        let matrix = matrix.clone();
        self.surface.canvas().set_matrix(&matrix.0);
        // self.with_matrix(|mat| {
        //     *mat = matrix.0.to_m33();
        //     mat
        // });
    }

    pub fn reset_transform(&mut self) {
        self.surface.canvas().reset_matrix();
        // self.with_matrix(|mat| {
        //     mat.reset();
        //     mat
        // });
    }
}
