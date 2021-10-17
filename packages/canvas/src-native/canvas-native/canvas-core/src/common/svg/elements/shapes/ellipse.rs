use roxmltree::Node;
use skia_safe::{RCHandle, Rect};
use std::collections::HashMap;
use std::f32::consts::PI;

use crate::common::context::{Context, Device};
use crate::common::svg::attribute_names::Attribute;
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::parser::{Parser, StyleMap};
use crate::common::svg::elements::renderer::{
    handle_style_data, render_mask, Renderer, set_filters, set_mask,
};
use crate::common::svg::units::length::{convert_length, Length};
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;

#[derive(Clone, Debug, PartialEq)]
pub struct Ellipse {
    cx: Length,
    cy: Length,
    rx: Length,
    ry: Length,
    style: StyleMap,
    bounding_box: BoundingBox,
}

impl Ellipse {
    pub fn new(cx: Length, cy: Length, rx: Length, ry: Length) -> Self {
        Self {
            cx,
            cy,
            rx,
            ry,
            style: Default::default(),
            bounding_box: Default::default(),
        }
    }
    pub fn zero() -> Self {
        Self {
            cx: Length::zero(),
            cy: Length::zero(),
            rx: Length::zero(),
            ry: Length::zero(),
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

    pub fn rx(&self) -> &Length {
        &self.rx
    }

    pub fn ry(&self) -> &Length {
        &self.ry
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

        let rx = convert_length(
            self.rx,
            Attribute::Rx,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );

        let ry = convert_length(
            self.ry,
            Attribute::Ry,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        self.bounding_box
            .set_all(cx - rx, cy - ry, cx + rx, cy + ry)
    }
}

impl Default for Ellipse {
    fn default() -> Self {
        Self::zero()
    }
}

impl Parser for Ellipse {
    fn parse_from(node: Node) -> Self {
        let style = Ellipse::style_from(node);
        let mut ellipse = Ellipse::default();
        ellipse.cx = Length::length_from_style(&style, Attribute::Cx, Length::zero());
        ellipse.cy = Length::length_from_style(&style, Attribute::Cy, Length::zero());
        ellipse.rx = Length::length_from_style(&style, Attribute::Rx, Length::zero());
        ellipse.ry = Length::length_from_style(&style, Attribute::Ry, Length::zero());
        ellipse.style.extend(style);
        ellipse
    }
}

impl Renderer for Ellipse {
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
        let rx = convert_length(
            self.rx,
            Attribute::Rx,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        let ry = convert_length(
            self.ry,
            Attribute::Ry,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        let handle_draw = |ctx: &mut Context| {
            ctx.ellipse(cx, cy, rx, ry, 0f32, 0f32, 2f32 * PI, false);

            if parsed.fill() {
                ctx.fill(None, parsed.fill_rule())
            }

            if parsed.stroke() {
                ctx.stroke(None)
            }
        };

        match parsed.mask() {
            None => {}
            Some(mask) => match set_mask(
                mask,
                context,
                view_box,
                bounding_box,
                root_element,
                self.style(),
            ) {
                None => {
                    if let Some(filter) = parsed.filter() {
                        set_filters(context, filter);
                    }
                    handle_draw(context);
                }
                Some(mut mask) => {
                    handle_draw(&mut mask.0);
                    render_mask(context, mask)
                }
            },
        }
        if let Some(mask) = parsed.mask() {
            set_mask(
                mask,
                context,
                view_box,
                bounding_box,
                root_element,
                self.style(),
            );
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
