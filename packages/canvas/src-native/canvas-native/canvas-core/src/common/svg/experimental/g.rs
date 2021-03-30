use roxmltree::Node;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::svg;
use crate::common::svg::{Display, handle_transform, parse_style, ParsedStyle, TRANSFORM_ATTR, Visibility};

pub fn handle_g<'a>(context: &mut Context, g: Node<'a, '_>, root: &'a roxmltree::Document<'a>, style: Rc<RefCell<ParsedStyle<'a>>>, parent_attributes: Option<HashMap<&str, &str>>) {
    let mut g_style = parse_style(context, &g, root);
    g_style.parent = Some(Rc::clone(&style));
    if g_style.visibility == Visibility::Hidden || g_style.visibility == Visibility::Collapse || g_style.display == Display::None {
        return;
    }
    let transform = g.attribute(TRANSFORM_ATTR);
    context.save();
    if let Some(transform) = transform {
        let transform_map = svg::get_transform_map(transform);
        let width = context.surface.width();
        let mut matrix = context.get_transform();
        handle_transform(&transform_map, &mut matrix, width, context.device.density);
        context.set_transform_matrix(&matrix);
    }
    let style = Rc::new(RefCell::new(g_style));
    for node in g.children().into_iter() {
        if node.is_element() && !node.tag_name().name().is_empty() {
            svg::handle_node(context, node, root, Rc::clone(&style), None);
        }
    }
    context.restore();
}
