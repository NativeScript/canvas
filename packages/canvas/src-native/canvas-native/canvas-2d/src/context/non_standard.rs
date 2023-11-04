use std::os::raw::c_float;

use csscolorparser::Color;
use skia_safe::paint::Style;
use skia_safe::{BlendMode, Paint, Rect};

use crate::context::Context;
use crate::utils::color;

#[repr(C)]
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
    pub fn draw_paint(&mut self, color: &str) {
        if let Some(color) = color::parse_color(color) {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_color(color);
            self.surface.canvas().draw_paint(&paint);
        }
    }

    pub fn draw_point(&mut self, x: c_float, y: c_float) {
        self.surface
            .canvas()
            .draw_point(skia_safe::Point::new(x, y), self.state.paint.stroke_paint());
    }

    pub fn draw_points(&mut self, mode: PointMode, points: &[c_float]) {
        let count = points.len();
        if count % 2 == 0 {
            let points: Vec<_> = points
                .chunks(2)
                .into_iter()
                .map(|point| (point[0], point[1]).into())
                .collect();

            self.surface.canvas().draw_points(
                mode.into(),
                points.as_slice(),
                self.state.paint.stroke_paint(),
            );
        }
    }
}
