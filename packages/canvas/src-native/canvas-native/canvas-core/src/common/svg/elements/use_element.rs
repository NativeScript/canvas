use roxmltree::{Node};

use crate::common::{
    context::Context,
    svg::{attribute_names::Attribute, bounding_box::BoundingBox},
};
use crate::common::svg::attribute_names::NodeExt;
use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::prelude::{Parser, StyleMapUtils};
use crate::common::svg::elements::renderer::{handle_render_child, handle_style_data};
use crate::common::svg::elements::symbol::Symbol;
use crate::common::svg::units::length::{convert_length, Length};
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;

use super::{prelude::StyleMap, renderer::Renderer};

#[derive(Clone, Debug)]
pub struct Use {
    x: Length,
    y: Length,
    width: Length,
    height: Length,
    style: StyleMap,
    bounding_box: BoundingBox,
    href: String,
}

impl Use {
    pub fn new(x: Length, y: Length, width: Length, height: Length) -> Self {
        Self {
            x,
            y,
            width,
            height,
            style: Default::default(),
            bounding_box: Default::default(),
            href: "".to_string(),
        }
    }
    pub fn zero() -> Self {
        Use::new(
            Length::zero(),
            Length::zero(),
            Length::zero(),
            Length::zero(),
        )
    }

    pub fn style(&self) -> &StyleMap {
        &self.style
    }
}

impl Default for Use {
    fn default() -> Self {
        Self::zero()
    }
}

impl Parser for Use {
    fn parse_from(node: Node) -> Self {
        let style = Use::style_from(node);
        let mut use_element = Use::default();
        use_element.x = Length::length_from_style(&style, Attribute::X, Length::zero());
        use_element.y = Length::length_from_style(&style, Attribute::Y, Length::zero());
        use_element.width = Length::length_from_style(&style, Attribute::Width, Length::zero());
        use_element.height = Length::length_from_style(&style, Attribute::Height, Length::zero());
        use_element.style.extend(style);
        use_element.href = node.get_href().unwrap_or("").to_string();
        use_element
    }
}

impl Renderer for Use {
    fn render(&mut self, context: &mut Context, node: Node, root_element: Node) {
        let mut style = StyleMap::new();
        let style_ref = &mut style;
        let mut render = |node: Node, style: &mut StyleMap| {
            context.save();
            let device = context.device;
            let mut bounding_box = self.bounding_box;
            style.update_with_style(self.style(), false);
            let parsed = handle_style_data(style, context, root_element, bounding_box);
            if !parsed.is_visible() {
                return;
            }
            let view_box = ViewBox::new_with_context(context);

            context.translate(
                convert_length(
                    self.x,
                    Attribute::X,
                    Units::UserSpaceOnUse,
                    device,
                    &view_box,
                ),
                convert_length(
                    self.y,
                    Attribute::Y,
                    Units::UserSpaceOnUse,
                    device,
                    &view_box,
                ),
            );

            if node.tag_name().name() == ElementName::Symbol.to_str() {
                Symbol::from_id(context, view_box, node, root_element);
            } else {
                handle_render_child(
                    context,
                    node,
                    root_element,
                    Some(&mut bounding_box),
                    Some(style),
                );
            }

            context.restore();
        };
        match node.get_node_with_style(self.href.as_str(), Some(style_ref), false) {
            (Some(first), Some(second)) => {
                render(second, style_ref);
            }
            (Some(first), None) => {
                render(first, style_ref);
            }
            _ => {}
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
