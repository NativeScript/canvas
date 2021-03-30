use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::context::paths::path::Path;
use crate::common::svg::{DrawType, ParsedStyle, POINTS_ATTR};
use crate::common::svg;

use super::ViewBox;

pub fn handle_polygon<'a>(context: &mut Context, polygon: roxmltree::Node, is_line: bool, style: Rc<RefCell<ParsedStyle>>, parent_attributes: Option<HashMap<&str, &str>>, root: &'a roxmltree::Document<'a>) {
    let points: Vec<&str> = polygon.attribute(POINTS_ATTR).unwrap_or("").split(" ").collect();
    let scale = context.device.density;
    let points: Vec<_> = points.into_iter().map(|value| {
        let points: Vec<_> = value.split(",").collect();
        let points = points.into_iter().fold(Vec::new(), |mut acc, value| {
            if let Ok(value) = value.parse::<f32>() {
                acc.push(value);
            }
            acc
        });
        points
    }).filter(|value| value.len() == 2).collect();
    if !points.is_empty() {
        let mut poly = Path::new();
        for (index, value) in points.into_iter().enumerate() {
            if index == 0 {
                &mut poly.move_to(*value.first().unwrap(), *value.last().unwrap());
            } else {
                &mut poly.line_to(*value.first().unwrap(), *value.last().unwrap());
            }
        }

        if !is_line {
            poly.close_path();
        }
        context.save();
        let rect = poly.bounds();
        let view_box = ViewBox::new(rect.x, rect.y, rect.width, rect.height);
        svg::handle_drawing(context, style, Some(poly), DrawType::FillStroke, None, root, &view_box);
        context.restore();
    }
}
