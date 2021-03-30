use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::svg::{DrawType, HEIGHT_ATTR, ParsedStyle, WIDTH_ATTR, X_ATTR, Y_ATTR};
use crate::common::svg;

use super::ViewBox;

pub fn handle_rect<'a>(context: &mut Context, rect: roxmltree::Node, style: Rc<RefCell<ParsedStyle>>, parent_attributes: Option<HashMap<&str, &str>>, root: &'a roxmltree::Document) {
    let scale = context.device.density;
    let mut x = "0";
    let mut y = "0";
    let mut width = "0";
    let mut height = "0";

    if let Some(map) = parent_attributes {
        if let Some(x_) = map.get("x") {
            x = &*x_
        }

        if let Some(y_) = map.get("y") {
            y = &*y_
        }


        if let Some(width_) = map.get("width") {
            width = &*width_
        }

        if let Some(height_) = map.get("height") {
            height = &*height_
        }
    }


    if let Some(val) = rect.attribute(X_ATTR) {
        x = val
    }

    if let Some(val) = rect.attribute(Y_ATTR) {
        y = val
    }

    if let Some(val) = rect.attribute(WIDTH_ATTR) {
        width = val
    }

    if let Some(val) = rect.attribute(HEIGHT_ATTR) {
        height = val
    }


    let x = x.parse::<f32>().unwrap_or(0f32) * scale;
    let y = y.parse::<f32>().unwrap_or(0f32) * scale;
    let width = width.parse::<f32>().unwrap_or(0f32) * scale;
    let height = height.parse::<f32>().unwrap_or(0f32) * scale;
    context.save();
    context.begin_path();
    context.rect(x, y, width, height);
    let view_box = ViewBox::new(x, y, width, height);
    svg::handle_drawing(context, style, None, DrawType::FillStroke, None, root, &view_box);
    context.restore();
}
