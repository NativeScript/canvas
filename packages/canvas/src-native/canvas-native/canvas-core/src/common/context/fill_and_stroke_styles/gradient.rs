use std::os::raw::c_float;

use skia_safe::{Color, Point, Shader, TileMode};
use skia_safe::gradient_shader::GradientShaderColors;

#[derive(Clone)]
pub enum Gradient {
    Linear {
        start: Point,
        stop: Point,
        stops: Vec<f32>,
        colors: Vec<Color>,
    },
    Radial {
        start: Point,
        start_radius: f32,
        stop: Point,
        stop_radius: f32,
        stops: Vec<f32>,
        colors: Vec<Color>,
    },
}

impl Gradient {
    pub fn set_shader(&mut self, shader: Option<Shader>) {
        match self {
            Gradient::Linear { .. } => {}
            Gradient::Radial { .. } => {}
        }
    }

    pub fn to_shader(gradient: &Gradient) -> Option<Shader> {
        match gradient {
            Gradient::Linear { start, stop, stops, colors, .. } => {
                Gradient::to_linear_gradient_shader(
                    *start, *stop, stops.as_slice(), colors.as_slice(),
                )
            }
            Gradient::Radial { start, start_radius, stop, stop_radius, stops, colors, .. } => {
                Gradient::to_radial_gradient_shader(
                    *start, *start_radius, *stop, *stop_radius, stops.as_slice(), colors.as_slice(),
                )
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
    ) -> Option<Shader> {
        let color_array = GradientShaderColors::Colors(colors);
        Shader::two_point_conical_gradient(
            start,
            start_radius,
            stop,
            stop_radius,
            color_array,
            Some(stops),
            TileMode::Clamp,
            None,
            None,
        )
    }

    fn to_linear_gradient_shader(
        start: Point,
        stop: Point,
        stops: &[f32],
        colors: &[Color],
    ) -> Option<Shader> {
        let color_array = GradientShaderColors::Colors(colors);
        Shader::linear_gradient(
            (start, stop),
            color_array,
            Some(stops),
            TileMode::Clamp,
            None,
            None,
        )
    }

    pub fn add_color_stop(&mut self, offset: c_float, color: Color) {
        let mut stops = match self {
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
