use std::ffi::CStr;
use std::os::raw::{c_float, c_ushort};

use skia_safe::{Color, Image, Paint};

use canvas_core::image_asset::ImageAsset;

use crate::context::compositing::composite_operation_type::CompositeOperationType;
use crate::context::Context;
use crate::utils::color;

#[derive(Copy, Clone, Debug)]
pub enum PointMode {
    Points = 0,
    Lines = 1,
    Polygon = 2,
}

impl Into<skia_safe::canvas::PointMode> for PointMode {
    fn into(self) -> skia_safe::canvas::PointMode {
        match self {
            PointMode::Points => skia_safe::canvas::PointMode::Points,
            PointMode::Lines => skia_safe::canvas::PointMode::Lines,
            PointMode::Polygon => skia_safe::canvas::PointMode::Polygon,
        }
    }
}

impl Into<i32> for PointMode {
    fn into(self) -> i32 {
        match self {
            PointMode::Points => 0,
            PointMode::Lines => 1,
            PointMode::Polygon => 2,
        }
    }
}

impl TryFrom<i32> for PointMode {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PointMode::Points),
            1 => Ok(PointMode::Lines),
            2 => Ok(PointMode::Polygon),
            _ => Err("Invalid PointMode"),
        }
    }
}

impl TryFrom<&str> for PointMode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "points" => Ok(PointMode::Points),
            "lines" => Ok(PointMode::Lines),
            "polygon" => Ok(PointMode::Polygon),
            _ => Err("Invalid PointMode"),
        }
    }
}

impl Context {
    pub fn draw_atlas_asset_color(
        &mut self,
        image: &ImageAsset,
        xform: &[f32],
        tex: &[f32],
        colors: Option<&[&CStr]>,
        blend_mode: CompositeOperationType,
    ) {
        let image = image.skia_image();
        if let Some(image) = image {
            let colors: Option<Vec<Color>> = colors.map(|color| {
                color
                    .iter()
                    .map(|color| {
                        let color = color.to_string_lossy();
                        color::parse_color(color.as_ref())
                    })
                    .flatten()
                    .collect()
            });
            self.draw_atlas(&image, xform, tex, colors.as_deref(), blend_mode)
        }
    }
    pub fn draw_atlas_asset(
        &mut self,
        image: &ImageAsset,
        xform: &[f32],
        tex: &[f32],
        colors: Option<&[Color]>,
        blend_mode: CompositeOperationType,
    ) {
        let image = image.skia_image();
        if let Some(image) = image {
            self.draw_atlas(&image, xform, tex, colors, blend_mode)
        }
    }

    pub fn draw_atlas_color(
        &mut self,
        image: &Image,
        xform: &[f32],
        tex: &[f32],
        colors: Option<&[&CStr]>,
        blend_mode: CompositeOperationType,
    ) {
        let colors: Option<Vec<Color>> = colors.map(|color| {
            color
                .iter()
                .map(|color| {
                    let color = color.to_string_lossy();
                    color::parse_color(color.as_ref())
                })
                .flatten()
                .collect()
        });

        self.draw_atlas(&image, xform, tex, colors.as_deref(), blend_mode)
    }

    pub fn draw_atlas(
        &mut self,
        image: &Image,
        xform: &[f32],
        tex: &[f32],
        colors: Option<&[Color]>,
        blend_mode: CompositeOperationType,
    ) {
        if xform.len() % 4 != 0 {
            return;
        }
        let xform: Vec<_> = xform
            .chunks(4)
            .map(|value| {
                skia_safe::RSXform::new(
                    value[0],
                    value[1],
                    skia_safe::Vector::new(value[2], value[3]),
                )
            })
            .collect();
        if tex.len() % 4 != 0 {
            return;
        }
        let tex: Vec<_> = tex
            .chunks(4)
            .map(|value| skia_safe::Rect::from_xywh(value[0], value[1], value[2], value[3]))
            .collect();

        self.with_canvas_dirty(|canvas| {
            canvas.draw_atlas(
                &image,
                xform.as_slice(),
                tex.as_slice(),
                colors,
                blend_mode.get_blend_mode(),
                skia_safe::SamplingOptions::default(),
                None,
                None,
            );
        });
    }

    pub fn fill_oval(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.with_canvas_dirty(|canvas| {
            canvas.draw_oval(
                skia_safe::Rect::from_xywh(x, y, width, height),
                self.state.paint.fill_paint(),
            );
        });
    }

    pub fn stroke_oval(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.with_canvas_dirty(|canvas| {
            canvas.draw_oval(
                skia_safe::Rect::from_xywh(x, y, width, height),
                self.state.paint.stroke_paint(),
            );
        });
    }

    pub fn draw_paint(&mut self, color: &str) {
        self.with_canvas_dirty(|canvas| {
            if let Some(color) = color::parse_color(color) {
                let mut paint = Paint::default();
                paint.set_anti_alias(true).set_color(color);
                canvas.draw_paint(&paint);
            }
        });
    }

    pub fn draw_point(&mut self, x: c_float, y: c_float) {
        self.with_canvas_dirty(|canvas| {
            canvas.draw_point(skia_safe::Point::new(x, y), self.state.paint.stroke_paint());
        });
    }

    pub fn draw_points(&mut self, mode: PointMode, points: &[c_float]) {
        self.with_canvas_dirty(|canvas| {
            let count = points.len();
            if count % 2 == 0 {
                let points: Vec<_> = points
                    .chunks(2)
                    .into_iter()
                    .map(|point| (point[0], point[1]).into())
                    .collect();

                canvas.draw_points(
                    mode.into(),
                    points.as_slice(),
                    self.state.paint.stroke_paint(),
                );
            }
        });
    }

    pub fn draw_vertices_color(
        &mut self,
        vertices: &[c_float],
        mode: skia_safe::vertices::VertexMode,
        indices: &[c_ushort],
        textures: &[c_float],
        colors: &[&CStr],
        blend_mode: CompositeOperationType,
    ) {
        let colors: Vec<_> = colors
            .iter()
            .filter_map(|color| {
                let color = color.to_string_lossy();
                csscolorparser::Color::from_html(color.as_ref())
                    .map(|color| {
                        let color = color.rgba_u8();
                        Color::from_argb(color.3, color.0, color.1, color.2)
                    })
                    .ok()
            })
            .collect();

        self.draw_vertices(
            vertices,
            mode,
            indices,
            textures,
            colors.as_slice(),
            blend_mode,
        );
    }

    pub fn draw_vertices(
        &mut self,
        vertices: &[c_float],
        mode: skia_safe::vertices::VertexMode,
        indices: &[c_ushort],
        textures: &[c_float],
        colors: &[Color],
        blend_mode: CompositeOperationType,
    ) {
        self.with_canvas_dirty(|canvas| {
            let positions: Vec<_> = vertices
                .chunks(2)
                .into_iter()
                .map(|point| (point[0], point[1]).into())
                .collect();

            let texs: Vec<_> = textures
                .chunks(2)
                .into_iter()
                .map(|point| (point[0], point[1]).into())
                .collect();

            if vertices.len() % 2 == 0 && textures.len() % 2 == 0 {
                let v = skia_safe::vertices::Vertices::new_copy(
                    mode,
                    &positions,
                    &texs,
                    &colors,
                    if indices.is_empty() {
                        None
                    } else {
                        Some(indices)
                    },
                );
                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                canvas.draw_vertices(&v, blend_mode.get_blend_mode(), &paint);
            }
        });
    }
}
