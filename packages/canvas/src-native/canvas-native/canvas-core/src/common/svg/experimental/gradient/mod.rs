use skia_safe::{Point, Vector};
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::context::fill_and_stroke_styles::gradient::Gradient;
use crate::common::svg::{OFFSET_ATTR, parse_style_attribute, ParsedStyle, STOP_COLOR_ATTR, STOP_OPACITY_ATTR, STYLE_ATTR};
use crate::common::utils::color::parse_color;

pub mod linear;
pub mod radial;


fn handle_stop(stop: &roxmltree::Node, gradient: &mut Gradient, style: Rc<RefCell<ParsedStyle>>) {
    let mut offset = 0f32;
    let offset_value = stop.attribute(OFFSET_ATTR).unwrap_or("0%");
    if offset_value.contains("%") {
        offset = offset_value.replace("%", "").parse::<f32>().unwrap_or(0.0) / 100.0
    } else {
        offset = offset_value.parse::<f32>().unwrap_or(0f32)
    }
    let mut stop_color = stop.attribute(STOP_COLOR_ATTR).unwrap_or("black");
    let mut stop_opacity = stop.attribute(STOP_OPACITY_ATTR).unwrap_or("1").parse::<f32>().unwrap_or(1f32);

    if let Some(style) = parse_style_attribute(stop.attribute(STYLE_ATTR)) {
        for key_val in style.into_iter() {
            let key = key_val.first();
            let value = key_val.last();
            if let (Some(key), Some(val)) = (key, value) {
                let key = *key;
                let val = *val;
                if !key.is_empty() {
                    match key {
                        STOP_COLOR_ATTR => {
                            stop_color = val;
                        }
                        STOP_OPACITY_ATTR => {
                            if let Ok(opacity) = val.parse::<f32>() {
                                stop_opacity = opacity
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }


    if let Some(color) = style.borrow().stop_color {
        stop_color = color;
    }

    if let Some(opacity) = style.borrow().opacity {
        stop_opacity = opacity
    }

    if let Some(color) = parse_color(stop_color) {
        let color = skia_safe::Color::from_argb(
            (color.a() as f32 * stop_opacity) as u8, color.r(), color.g(), color.b(),
        );
        gradient.add_color_stop(
            offset, color,
        );
    }
}
