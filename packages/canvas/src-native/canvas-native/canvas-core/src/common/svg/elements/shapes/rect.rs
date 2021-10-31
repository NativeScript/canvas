use roxmltree::Node;
use std::collections::HashMap;

use crate::common::context::{Context, Device};
use crate::common::svg::attribute_names::Attribute;
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::parser::{Parser, StyleMap};
use crate::common::svg::elements::renderer::{
    handle_style_data, render_mask,
    Renderer, set_filters, set_mask,
};
use crate::common::svg::units::length::{convert_length, Length};
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;

#[derive(Clone, Debug, PartialEq)]
pub struct Rect {
    x: Length,
    y: Length,
    width: Length,
    height: Length,
    style: StyleMap,
    bounding_box: BoundingBox,
}

impl Rect {
    pub fn new(x: Length, y: Length, width: Length, height: Length) -> Self {
        Self {
            x,
            y,
            width,
            height,
            style: Default::default(),
            bounding_box: BoundingBox::zero(),
        }
    }
    pub fn zero() -> Self {
        Self {
            x: Length::zero(),
            y: Length::zero(),
            width: Length::zero(),
            height: Length::zero(),
            style: HashMap::new(),
            bounding_box: BoundingBox::default(),
        }
    }

    pub fn x(&self) -> &Length {
        &self.x
    }

    pub fn y(&self) -> &Length {
        &self.y
    }
    pub fn width(&self) -> &Length {
        &self.width
    }
    pub fn height(&self) -> &Length {
        &self.height
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

    pub fn update_bounding_box(&mut self, width: f32, height: f32, device: Device) {
        let view_box = ViewBox::new_wh(width, height);
        self.bounding_box.set_all(
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
            convert_length(
                self.width,
                Attribute::Width,
                Units::UserSpaceOnUse,
                device,
                &view_box,
            ),
            convert_length(
                self.height,
                Attribute::Height,
                Units::UserSpaceOnUse,
                device,
                &view_box,
            ),
        )
    }
}

impl Default for Rect {
    fn default() -> Rect {
        Rect::zero()
    }
}

impl Parser for Rect {
    fn parse_from(node: Node) -> Self {
        let style = Rect::style_from(node);
        let mut rect = Rect::default();
        rect.x = Length::length_from_style(&style, Attribute::X, Length::zero());
        rect.y = Length::length_from_style(&style, Attribute::Y, Length::zero());
        rect.width = Length::length_from_style(&style, Attribute::Width, Length::zero());
        rect.height = Length::length_from_style(&style, Attribute::Height, Length::zero());
        rect.style.extend(style);
        rect
    }
}

impl Renderer for Rect {
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
            self.x,
            Attribute::X,
            Units::UserSpaceOnUse,
            context.device,
            &view_box,
        );
        let y = convert_length(
            self.y,
            Attribute::Y,
            Units::UserSpaceOnUse,
            context.device,
            &view_box,
        );
        let width = convert_length(
            self.width,
            Attribute::Width,
            Units::UserSpaceOnUse,
            context.device,
            &view_box,
        );
        let height = convert_length(
            self.height,
            Attribute::Height,
            Units::UserSpaceOnUse,
            context.device,
            &view_box,
        );
        let handle_draw = |ctx: &mut Context| {
            ctx.rect(x, y, width, height);

            if parsed.fill() {
                ctx.fill(None, parsed.fill_rule())
            }

            if parsed.stroke() {
                ctx.stroke(None)
            }
        };

        if let Some(mask) = parsed.mask() {
            let view_box = ViewBox::new_with_context(context);
            if let Some(mut mask) = set_mask(
                mask,
                context,
                view_box,
                bounding_box,
                root_element,
                self.style(),
            ) {
                handle_draw(&mut mask.0);
                render_mask(context, mask)
            }
        } else {
            if let Some(filter) = parsed.filter() {
                set_filters(context, filter);
            }
            handle_draw(context);
        }

        context.restore();
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
