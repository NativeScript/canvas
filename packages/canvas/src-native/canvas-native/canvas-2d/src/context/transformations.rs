use std::f32::consts::PI;
use std::os::raw::c_float;

use skia_safe::{Matrix, Point, M44};

use crate::context::Context;

const DEG: f32 = 180.0 / PI;

impl Context {
    pub fn get_transform(&mut self) -> Matrix {
        let mut matrix = self.surface.canvas().local_to_device_as_3x3();
        if self.state.use_device_scale {
            // return a non density scaled matrix
            let x = matrix.scale_x() / self.device.density;
            let y = matrix.scale_y() / self.device.density;
            matrix.set_scale((x, y), None);
        }
        matrix
    }

    pub fn rotate(&mut self, angle: c_float) {
        self.surface.canvas().rotate(angle * DEG, None);
    }

    pub fn scale(&mut self, x: c_float, y: c_float) {
        if self.state.use_device_scale {
            let density = self.device.density;
            self.surface.canvas().scale((x * density, y * density));
        } else {
            self.surface.canvas().scale((x, y));
        }
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
        let mut transform = Matrix::from_affine(&affine);
        if self.state.use_device_scale {
            let density = self.device.density;
            let x_scale = transform.scale_x() * density;
            let y_scale = transform.scale_y() * density;
            transform.set_scale_x(x_scale);
            transform.set_scale_y(y_scale);
        }
        self.surface.canvas().concat(&transform);
    }

    pub fn transform_with_matrix(&mut self, matrix: &Matrix) {
        let mut current = self.surface.canvas().local_to_device_as_3x3();
        current.pre_concat(matrix);

        if self.state.use_device_scale {
            let density = self.device.density;
            let x_scale = current.scale_x() * density;
            let y_scale = current.scale_y() * density;
            current.set_scale_x(x_scale);
            current.set_scale_y(y_scale);
        }

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
        let mut matrix = Matrix::from_affine(&affine);

        if self.state.use_device_scale {
            let density = self.device.density;
            let x_scale = matrix.scale_x() * density;
            let y_scale = matrix.scale_y() * density;
            matrix.set_scale_x(x_scale);
            matrix.set_scale_y(y_scale);
        }

        let m44 = M44::from(matrix);
        self.surface.canvas().set_matrix(&m44);
    }

    pub fn set_transform_matrix(&mut self, matrix: &Matrix) {
        self.surface.canvas().reset_matrix();
        let mut matrix = matrix.clone();
        if self.state.use_device_scale {
            let density = self.device.density;
            let x_scale = matrix.scale_x() * density;
            let y_scale = matrix.scale_y() * density;
            matrix.set_scale_x(x_scale);
            matrix.set_scale_y(y_scale);
        }
        let m44 = M44::from(matrix);
        self.surface.canvas().set_matrix(&m44);
    }

    pub fn reset_transform(&mut self) {
        self.surface.canvas().reset_matrix();
    }
}
