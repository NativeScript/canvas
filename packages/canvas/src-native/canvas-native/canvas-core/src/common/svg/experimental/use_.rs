use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::svg::{get_real_node, handle_transform, HEIGHT_ATTR, HREF_ATTR, IMAGE_NS, parse_style, ParsedStyle, SYMBOL_NS, TRANSFORM_ATTR, WIDTH_ATTR, X_ATTR, XLINK_HREF_NS, Y_ATTR};
use crate::common::svg;
use crate::common::svg::{image, symbol};

pub fn handle_use<'a>(context: &mut Context, use_node: roxmltree::Node<'a, '_>, root: &'a roxmltree::Document<'a>, style: Rc<RefCell<ParsedStyle<'a>>>, parent_attributes: Option<HashMap<&str, &str>>) {
    let mut map = HashMap::new();
    let mut x = 0f32;
    let mut y = 0f32;
    let scale = context.device.density;
    if let Some(x_) = use_node.attribute(X_ATTR) {
        map.insert(X_ATTR, x_);
        x = x_.parse::<f32>().unwrap_or(0f32) * scale;
    }
    if let Some(y_) = use_node.attribute(Y_ATTR) {
        map.insert(Y_ATTR, y_);
        y = y_.parse::<f32>().unwrap_or(0f32) * scale;
    }
    if let Some(width) = use_node.attribute(WIDTH_ATTR) {
        map.insert("width", width);
    }
    if let Some(height) = use_node.attribute(HEIGHT_ATTR) {
        map.insert("height", height);
    }
    let mut url = "";
    if let Some(href) = use_node.attribute((XLINK_HREF_NS, HREF_ATTR)) {
        url = href;
    } else if let Some(href) = use_node.attribute(HREF_ATTR) {
        url = href;
    }
    let mut parent_style = parse_style(context, &use_node, root);
    parent_style.parent = Some(Rc::clone(&style));
    let style = Rc::new(RefCell::new(parent_style));
    if !url.is_empty() {
        context.save();
        get_real_node(url, root, |node, is_last| {
            if is_last {
                let map = (&map).clone();
                let transform = use_node.attribute(TRANSFORM_ATTR);
                if let Some(transform) = transform {
                    let transform_map = svg::get_transform_map(transform);
                    let width = context.surface.width();
                    let mut matrix = context.get_transform();
                    handle_transform(&transform_map, &mut matrix, width, context.device.density);
                    context.set_transform_matrix(&matrix)
                }
                let style = Rc::clone(&style);
                match node.tag_name().name() {
                    SYMBOL_NS => {
                        symbol::handle_symbol(context, node, root, style, Some(map));
                    }
                    IMAGE_NS => {
                        image::handle_image(context, node, root, style, None);
                    }
                    _ => {
                        svg::handle_node(context, node, root, style, Some(map));
                    }
                }
            }
        });
        context.restore();
    }
}
