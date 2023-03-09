#![allow(dead_code)]

use std::f32::consts::PI;

use skia_safe::paint::{Cap, Join};
use skia_safe::{Paint, Point, Size};

pub fn almost_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.00001
}

pub fn to_degrees(radians: f32) -> f32 {
    radians / PI * 180.0
}

pub fn to_radians(degrees: f32) -> f32 {
    degrees / 180.0 * PI
}

pub(crate) fn inflate_stroke_rect(rect: &mut (Point, Size), paint: &Paint) {
    // Fast approximation of the stroke's bounding rect.
    // This yields a slightly oversized rect but is very fast
    // compared to Path::strokeBoundingRect().
    let root_2 = (2.0 as f32).sqrt();
    let mut delta: f32 = paint.stroke_width() / 2.0;
    if paint.stroke_join() == Join::Miter {
        delta *= paint.stroke_miter();
    } else if paint.stroke_cap() == Cap::Square {
        delta *= root_2;
    }
    inflate_rect(rect, delta)
}

pub(crate) fn inflate_rect(rect: &mut (Point, Size), value: f32) {
    inflate_rect_x(rect, value);
    inflate_rect_y(rect, value);
}

pub(crate) fn inflate_rect_x(rect: &mut (Point, Size), value: f32) {
    rect.0.x = rect.0.x - value;
    rect.1.width = rect.1.width + value + value;
}

pub(crate) fn inflate_rect_y(rect: &mut (Point, Size), value: f32) {
    rect.0.y = rect.0.y - value;
    rect.1.height = rect.1.height + value + value;
}
