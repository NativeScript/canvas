use std::os::raw::c_float;

use skia_safe::gradient as sk_gradient;
use skia_safe::{Color, Color4f, Point, Shader, TileMode};
use canvas_core::context_attributes::ColorSpace;
use crate::context::matrix::Matrix;

#[derive(Clone, Debug)]
pub enum Gradient {
    Linear {
        start: Point,
        stop: Point,
        stops: Vec<f32>,
        colors: Vec<Color>,
        matrix: Option<Matrix>,
        tile_mode: TileMode,
        color_space: ColorSpace
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
        color_space: ColorSpace
    },
    Conic {
        center: Point,
        angle: f32,
        stops: Vec<f32>,
        colors: Vec<Color>,
        matrix: Option<Matrix>,
        tile_mode: TileMode,
        color_space: ColorSpace
    },
}

impl Gradient {
    pub fn set_shader(&mut self, _shader: Option<Shader>) {
        match &self {
            Gradient::Linear { .. } => {}
            Gradient::Radial { .. } => {}
            Gradient::Conic { .. } => {}
        }
    }

    pub fn shader(gradient: &Gradient) -> Option<Shader> {
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
                    let matrix = matrix.to_m33();
                    Gradient::linear_gradient_shader(
                        *start,
                        *stop,
                        stops.as_slice(),
                        colors.as_slice(),
                        Some(&matrix),
                        *tile_mode,
                    )
                } else {
                    Gradient::linear_gradient_shader(
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
                    let matrix = matrix.to_m33();
                    Gradient::radial_gradient_shader(
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
                    Gradient::radial_gradient_shader(
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
            Gradient::Conic {
                center,
                angle,
                stops,
                colors,
                matrix,
                tile_mode,
                ..
            } => {
                if let Some(matrix) = matrix {
                    let matrix = matrix.to_m33();
                    Gradient::conic_gradient_shader(
                        *center,
                        *angle,
                        stops.as_slice(),
                        colors.as_slice(),
                        Some(&matrix),
                        *tile_mode,
                    )
                } else {
                    Gradient::conic_gradient_shader(
                        *center,
                        *angle,
                        stops.as_slice(),
                        colors.as_slice(),
                        None,
                        *tile_mode,
                    )
                }
            }
        }
    }

    fn radial_gradient_shader(
        start: Point,
        start_radius: c_float,
        stop: Point,
        stop_radius: c_float,
        stops: &[f32],
        colors: &[Color],
        matrix: Option<&skia_safe::Matrix>,
        tile_mode: TileMode,
    ) -> Option<Shader> {
        let colors = to_color4f(colors);
        let gradient = sk_gradient::Gradient::new(
            sk_gradient::Colors::new(&colors, Some(stops), tile_mode, None),
            sk_gradient::Interpolation::default(),
        );
        sk_gradient::shaders::two_point_conical_gradient((start, start_radius), (stop, stop_radius), &gradient, matrix)
    }

    fn linear_gradient_shader(
        start: Point,
        stop: Point,
        stops: &[f32],
        colors: &[Color],
        matrix: Option<&skia_safe::Matrix>,
        tile_mode: TileMode,
    ) -> Option<Shader> {
        let colors = to_color4f(colors);
        let gradient = sk_gradient::Gradient::new(
            sk_gradient::Colors::new(&colors, Some(stops), tile_mode, None),
            sk_gradient::Interpolation::default(),
        );
        sk_gradient::shaders::linear_gradient((start, stop), &gradient, matrix)
    }

    fn conic_gradient_shader(
        center: Point,
        angle: f32,
        stops: &[f32],
        colors: &[Color],
        matrix: Option<&skia_safe::Matrix>,
        tile_mode: TileMode,
    ) -> Option<Shader> {
        let colors = to_color4f(colors);
        let gradient = sk_gradient::Gradient::new(
            sk_gradient::Colors::new(&colors, Some(stops), tile_mode, None),
            sk_gradient::Interpolation::default(),
        );

        let mut rotated = skia_safe::Matrix::new_identity();
        rotated.pre_translate((center.x, center.y))
            .pre_rotate(angle, None)
            .pre_translate((-center.x, -center.y));

        if let Some(matrix) = matrix {
            rotated = skia_safe::Matrix::concat(&rotated, matrix);
        }

        //   rotated

        sk_gradient::shaders::sweep_gradient(center, (0., 360.), &gradient, Some(&rotated))
    }

    pub fn add_color_stop_str(&mut self, offset: c_float, color: &str) {
        if let Ok(color) = color.parse::<csscolorparser::Color>() {
            let color = color.to_rgba8();
            self.add_color_stop(offset, Color::from_argb(color[3], color[0], color[1], color[2]))
        }
    }

    pub fn add_color_stop(&mut self, offset: c_float, color: Color) {
        let stops = match self {
            Gradient::Linear { stops, .. } => stops,
            Gradient::Radial { stops, .. } => stops,
            Gradient::Conic { stops, .. } => stops,
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
            Gradient::Conic { colors, stops, .. } => {
                colors.insert(idx, color);
                stops.insert(idx, offset);
            }
        };
    }

    pub fn add_color4f_stop(&mut self, offset: c_float, color: Color4f) {
        let color = color.to_color();
        self.add_color_stop(offset, color);
    }

    /// Add a color stop from a packed ARGB u32 value.
    /// Avoids needing skia_safe::Color in the caller.
    pub fn add_color_stop_with_argb(&mut self, offset: c_float, argb: u32) {
        self.add_color_stop(offset, Color::from(argb));
    }
}

fn to_color4f(colors: &[Color]) -> Vec<Color4f> {
    colors.iter().map(|&color| Color4f::from(color)).collect()
}
