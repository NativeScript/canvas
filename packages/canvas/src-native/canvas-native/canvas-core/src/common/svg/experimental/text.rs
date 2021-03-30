use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::svg::{AFTER_FILL_EVENT, AFTER_STROKE_EVENT, DrawType, ParsedStyle, X_ATTR, Y_ATTR};
use crate::common::svg;

use super::ViewBox;

pub fn handle_text<'a>(context: &mut Context, text: roxmltree::Node, style: Rc<RefCell<ParsedStyle>>, parent_attributes: Option<HashMap<&str, &str>>, root: &'a roxmltree::Document<'a>) {
    let scale = context.device.density;
    let mut x = "0";
    let mut y = "0";
    if let Some(map) = parent_attributes {
        if let Some(x_) = map.get(X_ATTR) {
            x = &*x_
        }

        if let Some(y_) = map.get(Y_ATTR) {
            y = &*y_
        }
    }


    if let Some(x_) = text.attribute(X_ATTR) {
        x = x_
    }

    if let Some(y_) = text.attribute(Y_ATTR) {
        y = y_
    }

    let x = x.parse::<f32>().unwrap_or(0f32) * scale;
    let y = y.parse::<f32>().unwrap_or(0f32) * scale;
    if let Some(text) = text.text() {
        let text = text.to_string();
        let width = context.measure_text(&text).width;
        let view_box = ViewBox::new(x, y, width, y);
        context.save();
        svg::handle_drawing(context, style, None, DrawType::Text, Some(Box::new(move |event, context| {
            match event {
                AFTER_FILL_EVENT => {
                    context.fill_text(&text, x, y, 0f32)
                }
                AFTER_STROKE_EVENT => {
                    context.stroke_text(&text, x, y, 0f32)
                }
                _ => {}
            }
        })), root, &view_box);
        context.restore();
    }
}
