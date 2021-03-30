use std::collections::HashMap;
use std::f32::consts::PI;

use roxmltree::{Document, Node};

use crate::common::context::{Context, Device};
use crate::common::context::compositing::composite_operation_type::CompositeOperationType;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::context::fill_and_stroke_styles::pattern::Repetition;
use crate::common::context::paths::path::Path;
use crate::common::svg::attribute_names::Attribute;
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::parser::{Parser, StyleMap};
use crate::common::svg::elements::renderer::{
    handle_mask, handle_style_data, render_mask, Renderer, set_filters, set_mask,
};
use crate::common::svg::elements::svg::{create_context, Svg};
use crate::common::svg::prelude::ColorConversation;
use crate::common::svg::units::length::{convert_length, Length};
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;

#[derive(Clone, Debug, PartialEq)]
pub struct Circle {
    cx: Length,
    cy: Length,
    r: Length,
    style: StyleMap,
    bounding_box: BoundingBox,
}

impl Circle {
    pub fn new(cx: Length, cy: Length, r: Length) -> Self {
        Self {
            cx,
            cy,
            r,
            style: Default::default(),
            bounding_box: Default::default(),
        }
    }

    pub fn zero() -> Self {
        Self {
            cx: Length::zero(),
            cy: Length::zero(),
            r: Length::zero(),
            style: HashMap::new(),
            bounding_box: Default::default(),
        }
    }

    pub fn cx(&self) -> &Length {
        &self.cx
    }

    pub fn cy(&self) -> &Length {
        &self.cy
    }

    pub fn r(&self) -> &Length {
        &self.r
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
        let cx = convert_length(
            self.cx,
            Attribute::Cx,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        let cy = convert_length(
            self.cy,
            Attribute::Cy,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );

        let r = convert_length(
            self.r,
            Attribute::R,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        self.bounding_box.set_all(cx - r, cy - r, cx + r, cy + r);
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self::zero()
    }
}

impl Parser for Circle {
    fn parse_from(node: Node) -> Self {
        let style = Circle::style_from(node);
        let mut circle = Circle::default();
        circle.cx = Length::length_from_style(&style, Attribute::Cx, Length::zero());
        circle.cy = Length::length_from_style(&style, Attribute::Cy, Length::zero());
        circle.r = Length::length_from_style(&style, Attribute::R, Length::zero());
        circle.style.extend(style);
        circle
    }
}

impl Renderer for Circle {
    fn render(&mut self, context: &mut Context, node: Node, root_element: Node) {
        context.save();
        let device = context.device;
        let bounding_box = self.bounding_box;
        let parsed = handle_style_data(self.style(), context, root_element, bounding_box);
        if !parsed.is_visible() {
            return;
        }
        let view_box = ViewBox::new_with_context(context);
        context.begin_path();

        let cx = convert_length(
            self.cx,
            Attribute::Cx,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        let cy = convert_length(
            self.cy,
            Attribute::Cy,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        let r = convert_length(
            self.r,
            Attribute::R,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );

        let handle_draw = |ctx: &mut Context| {
            ctx.arc(cx, cy, r, 0f32, 2f32 * PI, false);

            if parsed.fill() {
                ctx.fill(None, parsed.fill_rule())
            }

            if parsed.stroke() {
                ctx.stroke(None)
            }
        };

        if let Some(mask) = parsed.mask() {
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

        context.restore()
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
