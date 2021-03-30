use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::context::paths::path::Path;
use crate::common::svg;
use crate::common::svg::{D_ATTR, DrawType, ParsedStyle};

use super::ViewBox;

pub fn handle_path<'a>(
    context: &mut Context,
    path: roxmltree::Node,
    style: Rc<RefCell<ParsedStyle>>,
    parent_attributes: Option<HashMap<&str, &str>>,
    root: &'a roxmltree::Document<'a>,
) {
    if let Some(d) = path.attribute(D_ATTR) {
        if !d.is_empty() {
            let scale = context.device.density;
            let mut path = Path::from_str(d);
            context.save();
            let bounds = path.path.bounds();
            let view_box = ViewBox::new(bounds.x(), bounds.y(), bounds.width(), bounds.height());
            svg::handle_drawing(context, style, Some(path), DrawType::FillStroke, None, root, &view_box);
            context.restore();
        }
    }
}
