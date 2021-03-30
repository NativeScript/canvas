use roxmltree::{Document, Node};

use crate::common::context::Context;
use crate::common::svg::attribute_names::Attribute;
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::parser::StyleMap;
use crate::common::svg::elements::prelude::Parser;
use crate::common::svg::elements::renderer::{handle_render_children, handle_render_children_with_bounding_box, handle_style_data, Renderer, set_filters};
use crate::common::svg::elements::svg::Svg;

#[derive(Clone, Debug)]
pub struct G {
    id: String,
    style: StyleMap,
    bounding_box: BoundingBox,
}

impl G {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            style: Default::default(),
            bounding_box: Default::default(),
        }
    }

    pub fn style(&self) -> &StyleMap {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut StyleMap {
        &mut self.style
    }
}

impl Default for G {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for G {
    fn parse_from(node: Node) -> Self {
        let style = G::style_from(node);
        let mut g = G::default();
        g.id = node.attribute(Attribute::Id.to_str()).unwrap_or("").to_string();
        g.style.extend(style);
        g
    }
}

impl Renderer for G {
    fn render(&mut self, context: &mut Context, node: Node, root_element: Node) {
        context.save();
        let mut bounding_box = self.bounding_box;
        let parsed = handle_style_data(self.style(), context, root_element, bounding_box);
        if !parsed.is_visible() {
            return;
        }
        if let Some(filter) = parsed.filter() {
            set_filters(context, filter);
        }
        let mut children = node.children();
        handle_render_children_with_bounding_box(context, &mut children, root_element, Some(&mut bounding_box), Some(self.style()));
        context.restore();
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
