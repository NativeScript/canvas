use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::context::paths::path::Path;
use crate::common::svg::{DrawType, ParsedStyle, X1_ATTR, X2_ATTR, Y1_ATTR, Y2_ATTR};
use crate::common::svg;

use super::ViewBox;

pub fn handle_line<'a>(context: &mut Context, line: roxmltree::Node, style: Rc<RefCell<ParsedStyle>>, parent_attributes: Option<HashMap<&str, &str>>, root: &'a roxmltree::Document<'a>) {
    let scale = context.device.density;
    let mut x1 = "0";
    let mut y1 = "0";
    let mut x2 = "0";
    let mut y2 = "0";

    if let Some(map) = parent_attributes {
        if let Some(x1_) = map.get(X1_ATTR) {
            x1 = &*x1_
        }

        if let Some(y1_) = map.get(Y1_ATTR) {
            y1 = &*y1_
        }


        if let Some(x2_) = map.get(X2_ATTR) {
            x2 = &*x2_
        }

        if let Some(y2_) = map.get(Y2_ATTR) {
            y2 = &*y2_
        }
    }


    if let Some(x1_) = line.attribute(X1_ATTR) {
        x1 = x1_
    }

    if let Some(y1_) = line.attribute(Y1_ATTR) {
        y1 = y1_
    }

    if let Some(x2_) = line.attribute(X2_ATTR) {
        x2 = x2_
    }

    if let Some(y2_) = line.attribute(Y2_ATTR) {
        y2 = y2_
    }


    let x1 = x1.parse::<f32>().unwrap_or(0f32) * scale;
    let y1 = y1.parse::<f32>().unwrap_or(0f32) * scale;
    let x2 = x2.parse::<f32>().unwrap_or(0f32) * scale;
    let y2 = y2.parse::<f32>().unwrap_or(0f32) * scale;


    context.save();
    let mut line = Path::new();
    line.move_to(x1, y1);
    line.line_to(x2, y2);
    let view_box = ViewBox::new(x1, y1, x2, y2);
    svg::handle_drawing(context, style, Some(line), DrawType::Stroke, None, root, &view_box);
    context.restore();
}
