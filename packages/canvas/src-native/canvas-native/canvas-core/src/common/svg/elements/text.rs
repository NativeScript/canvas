use roxmltree::{Document, Node};
use std::collections::HashMap;

use crate::common::context::{Context, Device};
use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::prelude::*;
use crate::common::svg::elements::renderer::{handle_style_data, Renderer, set_filters};
use crate::common::svg::elements::svg::Svg;
use crate::common::svg::units::length::{convert_length, Length};
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;

#[derive(Clone, Debug)]
pub struct Text {
    x: Length,
    y: Length,
    dx: Length,
    dy: Length,
    text: String,
    style: StyleMap,
    bounding_box: BoundingBox,
}

impl Text {
    pub fn new(text: String, x: Length, y: Length, dx: Length, dy: Length) -> Self {
        Self {
            x,
            y,
            dx,
            dy,
            text,
            style: Default::default(),
            bounding_box: Default::default(),
        }
    }
    pub fn zero() -> Self {
        Self {
            x: Length::zero(),
            y: Length::zero(),
            dx: Default::default(),
            dy: Default::default(),
            text: "".to_owned(),
            style: HashMap::new(),
            bounding_box: Default::default(),
        }
    }

    pub fn x(&self) -> &Length {
        &self.x
    }

    pub fn y(&self) -> &Length {
        &self.y
    }

    pub fn dx(&self) -> &Length {
        &self.dx
    }

    pub fn dy(&self) -> &Length {
        &self.dy
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    pub fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }

    pub fn style(&self) -> &StyleMap {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut StyleMap {
        &mut self.style
    }

    pub fn update_bounding_box(
        &mut self,
        width: f32,
        height: f32,
        device: Device,
        font_size: f32,
        text_size: f32,
    ) {
        let view_box = ViewBox::new_wh(width, height);
        let x = convert_length(
            self.x,
            Attribute::X,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        ) + convert_length(
            self.dx,
            Attribute::Dx,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        let y = convert_length(
            self.y,
            Attribute::Y,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        ) + convert_length(
            self.dy,
            Attribute::Dy,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        self.bounding_box
            .set_all(x, y - font_size, x + text_size, y)
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::zero()
    }
}

impl Parser for Text {
    fn parse_from(node: Node) -> Self {
        let style = Text::style_from(node);
        let mut text = Text::default();
        text.x = node.get_length(Attribute::X, Length::zero());
        text.y = node.get_length(Attribute::X, Length::zero());
        text.dx = node.get_length(Attribute::Dx, Length::zero());
        text.dy = node.get_length(Attribute::Dy, Length::zero());
        text.text = node.text().unwrap_or("").to_owned();
        text.style.extend(style);
        text
    }
}

impl Renderer for Text {
    fn render(&mut self, context: &mut Context, node: Node, root_element: Node) {
        context.save();
        let bounding_box = self.bounding_box;
        let parsed = handle_style_data(self.style(), context, root_element, bounding_box);
        if !parsed.is_visible() {
            return;
        }
        let view_box = ViewBox::new_with_context(context);
        context.begin_path();
        let x = convert_length(
            self.dx,
            Attribute::Dx,
            Units::UserSpaceOnUse,
            context.device,
            &view_box,
        ) + convert_length(
            self.x,
            Attribute::X,
            Units::UserSpaceOnUse,
            context.device,
            &view_box,
        );

        let y = convert_length(
            self.dy,
            Attribute::Dy,
            Units::UserSpaceOnUse,
            context.device,
            &view_box,
        ) + convert_length(
            self.y,
            Attribute::Y,
            Units::UserSpaceOnUse,
            context.device,
            &view_box,
        );

        if let Some(filter) = parsed.filter() {
            set_filters(context, filter);
        }

        if parsed.fill() {
            context.fill_text(self.text(), x, y, 0.0);
        }

        if parsed.stroke() {
            context.stroke_text(self.text(), x, y, 0.0);
        }
        context.restore();
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
