use std::cell::RefCell;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::svg::{CX_ATTR, CY_ATTR, DrawType, ParsedStyle, RX_ATTR, RY_ATTR};
use crate::common::svg;

use super::ViewBox;

pub fn handle_ellipse<'a>(context: &mut Context, ellipse: roxmltree::Node, style: Rc<RefCell<ParsedStyle>>, parent_attributes: Option<HashMap<&str, &str>>, root: &'a roxmltree::Document<'a>) {
    let scale = context.device.density;
    let mut cx = "0";
    let mut cy = "0";
    let mut rx = "0";
    let mut ry = "0";

    if let Some(map) = parent_attributes {
        if let Some(cx_) = map.get("cx") {
            cx = &*cx_
        }

        if let Some(cy_) = map.get("cy") {
            cy = &*cy_
        }


        if let Some(rx_) = map.get("rx") {
            rx = &*rx_
        }

        if let Some(ry_) = map.get("ry") {
            ry = &*ry_
        }
    }


    if let Some(cx_) = ellipse.attribute(CX_ATTR) {
        cx = &*cx_
    }

    if let Some(cy_) = ellipse.attribute(CY_ATTR) {
        cy = &*cy_
    }

    if let Some(rx_) = ellipse.attribute(RX_ATTR) {
        rx = &*rx_
    }

    if let Some(ry_) = ellipse.attribute(RY_ATTR) {
        ry = &*ry_
    }


    let cx = cx.parse::<f32>().unwrap_or(0f32) * scale;
    let cy = cy.parse::<f32>().unwrap_or(0f32) * scale;
    let rx = rx.parse::<f32>().unwrap_or(0f32) * scale;
    let ry = ry.parse::<f32>().unwrap_or(0f32) * scale;
    context.save();
    context.begin_path();
    context.ellipse(cx, cy, rx, ry, 0f32, 0f32, 2f32 * PI, false);
    let view_box = ViewBox::new(cx - rx, cy - ry, cx + rx, cy + ry);
    svg::handle_drawing(context, style, None, DrawType::FillStroke, None, root, &view_box);
    context.restore();
}
