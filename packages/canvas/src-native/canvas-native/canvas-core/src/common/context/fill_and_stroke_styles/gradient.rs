use std::os::raw::c_float;

use skia_safe::{Color, Point, Shader, TileMode};
use skia_safe::gradient_shader::GradientShaderColors;

use crate::common::context::matrix::Matrix;

#[derive(Clone)]
pub enum Gradient {
    Linear {
        start: Point,
        stop: Point,
        stops: Vec<f32>,
        colors: Vec<Color>,
        matrix: Option<Matrix>,
        tile_mode: TileMode,
    },
    Radial {
        start: Point,
        start_radius: f32,
        stop: Point,
        stop_radius: f32,
        stops: Vec<f32>,
        colors: Vec<Color>,
        matrix: Option<Matrix>,
        tile_mode: TileMode,
    },
}

impl Gradient {
    pub fn set_shader(&mut self, _shader: Option<Shader>) {
        match &self {
            Gradient::Linear { .. } => {}
            Gradient::Radial { .. } => {}
        }
    }

    pub(crate) fn set_tile_mode(&mut self, mode: TileMode) {
        match self {
            Gradient::Linear {
                ref mut tile_mode, ..
            } => *tile_mode = mode,
            Gradient::Radial {
                ref mut tile_mode, ..
            } => *tile_mode = mode,
        }
    }

    pub fn to_shader(gradient: &Gradient) -> Option<Shader> {
        match gradient {
            Gradient::Linear {
                start,
                stop,
                stops,
                colors,
                matrix,
                tile_mode,
                ..
            } => {
                if let Some(matrix) = matrix {
                    let matrix = matrix.matrix.to_m33();
                    Gradient::to_linear_gradient_shader(
                        *start,
                        *stop,
                        stops.as_slice(),
                        colors.as_slice(),
                        Some(&matrix),
                        *tile_mode,
                    )
                } else {
                    Gradient::to_linear_gradient_shader(
                        *start,
                        *stop,
                        stops.as_slice(),
                        colors.as_slice(),
                        None,
                        *tile_mode,
                    )
                }
            }
            Gradient::Radial {
                start,
                start_radius,
                stop,
                stop_radius,
                stops,
                colors,
                matrix,
                tile_mode,
                ..
            } => {
                if let Some(matrix) = matrix {
                    let matrix = matrix.matrix.to_m33();
                    Gradient::to_radial_gradient_shader(
                        *start,
                        *start_radius,
                        *stop,
                        *stop_radius,
                        stops.as_slice(),
                        colors.as_slice(),
                        Some(&matrix),
                        *tile_mode,
                    )
                } else {
                    Gradient::to_radial_gradient_shader(
                        *start,
                        *start_radius,
                        *stop,
                        *stop_radius,
                        stops.as_slice(),
                        colors.as_slice(),
                        None,
                        *tile_mode,
                    )
                }
            }
        }
    }

    fn to_radial_gradient_shader(
        start: Point,
        start_radius: c_float,
        stop: Point,
        stop_radius: c_float,
        stops: &[f32],
        colors: &[Color],
        matrix: Option<&skia_safe::Matrix>,
        tile_mode: TileMode,
    ) -> Option<Shader> {
        let color_array = GradientShaderColors::Colors(colors);
        Shader::two_point_conical_gradient(
            start,
            start_radius,
            stop,
            stop_radius,
            color_array,
            Some(stops),
            tile_mode,
            None,
            matrix,
        )
    }

    fn to_linear_gradient_shader(
        start: Point,
        stop: Point,
        stops: &[f32],
        colors: &[Color],
        matrix: Option<&skia_safe::Matrix>,
        tile_mode: TileMode,
    ) -> Option<Shader> {
        let color_array = GradientShaderColors::Colors(colors);

        Shader::linear_gradient(
            (start, stop),
            color_array,
            Some(stops),
            tile_mode,
            None,
            matrix,
        )
    }

    pub fn add_color_stop(&mut self, offset: c_float, color: Color) {
        let stops = match self {
            Gradient::Linear { stops, .. } => stops,
            Gradient::Radial { stops, .. } => stops,
        };

        // insert the new entries at the right index to keep the vectors sorted
        let idx = stops
            .binary_search_by(|n| (n - f32::EPSILON).partial_cmp(&offset).unwrap())
            .unwrap_or_else(|x| x);
        match self {
            Gradient::Linear { colors, stops, .. } => {
                colors.insert(idx, color);
                stops.insert(idx, offset);
            }
            Gradient::Radial { colors, stops, .. } => {
                colors.insert(idx, color);
                stops.insert(idx, offset);
            }
        };
    }
}
