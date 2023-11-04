use std::os::raw::c_float;

use skia_safe::{Image, Point, TileMode};

use crate::context::fill_and_stroke_styles::gradient::Gradient;
use crate::context::fill_and_stroke_styles::pattern::{Pattern, Repetition};
use crate::context::matrix::Matrix;
use crate::context::Context;

impl Context {
    pub fn create_linear_gradient(
        &self,
        x0: c_float,
        y0: c_float,
        x1: c_float,
        y1: c_float,
    ) -> Gradient {
        Gradient::Linear {
            start: Point::new(x0, y0),
            stop: Point::new(x1, y1),
            colors: Vec::new(),
            stops: Vec::new(),
            matrix: None,
            tile_mode: TileMode::Clamp,
        }
    }

    pub fn create_linear_gradient_with_matrix(
        &self,
        x0: c_float,
        y0: c_float,
        x1: c_float,
        y1: c_float,
        matrix: Matrix,
    ) -> Gradient {
        Gradient::Linear {
            start: Point::new(x0, y0),
            stop: Point::new(x1, y1),
            colors: Vec::new(),
            stops: Vec::new(),
            matrix: Some(matrix),
            tile_mode: TileMode::Clamp,
        }
    }

    pub fn create_conic_gradient(
        &self,
        start_angle: c_float,
        x: c_float,
        y: c_float,
    ) -> Gradient {
        let angle = crate::common::utils::geometry::to_degrees(start_angle) - 90.0;
        Gradient::Conic {
            center: Point::new(x, y),
            angle,
            colors: Vec::new(),
            stops: Vec::new(),
            matrix: None,
            tile_mode: TileMode::Clamp,
        }
    }


    pub fn create_conic_gradient_with_matrix(
        &self,
        start_angle: c_float,
        x: c_float,
        y: c_float,
        matrix: Matrix,
    ) -> Gradient {
        Gradient::Conic {
            center: Point::new(x, y),
            angle: start_angle,
            colors: Vec::new(),
            stops: Vec::new(),
            matrix: Some(matrix),
            tile_mode: TileMode::Clamp,
        }
    }

    pub fn create_pattern(&self, image: Image, rep: Repetition) -> Pattern {
        Pattern::new(image, rep)
    }

    pub fn create_radial_gradient(
        &self,
        x0: c_float,
        y0: c_float,
        r0: c_float,
        x1: c_float,
        y1: c_float,
        r1: c_float,
    ) -> Gradient {
        Gradient::Radial {
            start: Point::new(x0, y0),
            start_radius: r0,
            stop: Point::new(x1, y1),
            stop_radius: r1,
            stops: Vec::new(),
            colors: Vec::new(),
            matrix: None,
            tile_mode: TileMode::Clamp,
        }
    }

    pub fn create_radial_gradient_with_matrix(
        &self,
        x0: c_float,
        y0: c_float,
        r0: c_float,
        x1: c_float,
        y1: c_float,
        r1: c_float,
        matrix: Matrix,
    ) -> Gradient {
        Gradient::Radial {
            start: Point::new(x0, y0),
            start_radius: r0,
            stop: Point::new(x1, y1),
            stop_radius: r1,
            stops: Vec::new(),
            colors: Vec::new(),
            matrix: Some(matrix),
            tile_mode: TileMode::Clamp,
        }
    }
}
