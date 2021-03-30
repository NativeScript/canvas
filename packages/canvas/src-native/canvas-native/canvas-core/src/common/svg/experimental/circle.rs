use std::cell::RefCell;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::svg;
use crate::common::svg::{CX_ATTR, CY_ATTR, DrawType, ParsedStyle, R_ATTR};

use super::ViewBox;

pub fn handle_circle<'a>(
    context: &mut Context,
    circle: roxmltree::Node,
    style: Rc<RefCell<ParsedStyle>>,
    parent_attributes: Option<HashMap<&str, &str>>,
    root: &'a roxmltree::Document<'a>,
) {
    let scale = context.device.density;
    let mut cx = "0";
    let mut cy = "0";
    let mut r = "0";

    if let Some(map) = parent_attributes {
        if let Some(cx_) = map.get("cx") {
            cx = &*cx_
        }

        if let Some(cy_) = map.get("cy") {
            cy = &*cy_
        }

        if let Some(r_) = map.get("r") {
            r = &*r_
        }
    }

    if let Some(cx_) = circle.attribute(CX_ATTR) {
        cx = &*cx_
    }

    if let Some(cy_) = circle.attribute(CY_ATTR) {
        cy = &*cy_
    }

    if let Some(r_) = circle.attribute(R_ATTR) {
        r = &*r_
    }

    let cx = cx.parse::<f32>().unwrap_or(0f32) * scale;
    let cy = cy.parse::<f32>().unwrap_or(0f32) * scale;
    let r = r.parse::<f32>().unwrap_or(0f32) * scale;

    context.save();
    context.begin_path();
    context.arc(cx, cy, r, 0f32, 2f32 * PI, false);
    let view_box = ViewBox::new(cx - r, cy - r, cx + r, cy + r);
    svg::handle_drawing(context, style, None, DrawType::FillStroke, None, root, &view_box);
    context.restore();
}
