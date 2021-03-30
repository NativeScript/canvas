use std::collections::HashMap;

use roxmltree::{Document, Node};
use skia_safe::{Point, RCHandle, Rect};

use crate::common::context::{Context, Device};
use crate::common::context::paths::path::Path;
use crate::common::svg::attribute_names::Attribute;
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::marker::{Marker, MarkerPosition};
use crate::common::svg::elements::mask::Mask;
use crate::common::svg::elements::parser::{Parser, StyleMap};
use crate::common::svg::elements::reference_element::ReferenceElement;
use crate::common::svg::elements::renderer::{
    handle_mask, handle_style_data, render_mask, Renderer, set_filters, set_mask,
};
use crate::common::svg::elements::svg::{create_context, Svg};
use crate::common::svg::prelude::ColorConversation;
use crate::common::svg::units::length::{convert_length, Length};
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;

#[derive(Clone)]
pub struct Line {
    x1: Length,
    x2: Length,
    y1: Length,
    y2: Length,
    style: StyleMap,
    bounding_box: BoundingBox,
    markers: (Vec<Point>, Vec<f32>),
}

impl Line {
    pub fn new(x1: Length, y1: Length, x2: Length, y2: Length) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            style: Default::default(),
            bounding_box: Default::default(),
            markers: (vec![], vec![]),
        }
    }
    pub fn zero() -> Self {
        Self {
            x1: Length::zero(),
            y1: Length::zero(),
            x2: Length::zero(),
            y2: Length::zero(),
            style: HashMap::new(),
            bounding_box: Default::default(),
            markers: (vec![], vec![]),
        }
    }

    pub fn x1(&self) -> &Length {
        &self.x1
    }

    pub fn y1(&self) -> &Length {
        &self.y1
    }

    pub fn x2(&self) -> &Length {
        &self.x2
    }

    pub fn y2(&self) -> &Length {
        &self.y2
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
                self.x1,
                Attribute::X1,
                Units::UserSpaceOnUse,
                device,
                &view_box,
            ),
            convert_length(
                self.y1,
                Attribute::Y1,
                Units::UserSpaceOnUse,
                device,
                &view_box,
            ),
            convert_length(
                self.x2,
                Attribute::X2,
                Units::UserSpaceOnUse,
                device,
                &view_box,
            ),
            convert_length(
                self.y2,
                Attribute::Y2,
                Units::UserSpaceOnUse,
                device,
                &view_box,
            ),
        )
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::zero()
    }
}

impl Parser for Line {
    fn parse_from(node: Node) -> Self {
        let style = Line::style_from(node);
        let mut line = Line::default();
        line.x1 = Length::length_from_style(&style, Attribute::X1, Length::zero());
        line.y1 = Length::length_from_style(&style, Attribute::Y1, Length::zero());
        line.x2 = Length::length_from_style(&style, Attribute::X2, Length::zero());
        line.y2 = Length::length_from_style(&style, Attribute::Y2, Length::zero());
        line.style.extend(style);
        line
    }
}

impl Renderer for Line {
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
        let x1 = convert_length(
            self.x1,
            Attribute::X1,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        let y1 = convert_length(
            self.y1,
            Attribute::Y1,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        let x2 = convert_length(
            self.x2,
            Attribute::X2,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        let y2 = convert_length(
            self.y2,
            Attribute::Y2,
            Units::UserSpaceOnUse,
            device,
            &view_box,
        );
        let mut line = Path::new();
        let mut handle_draw = |ctx: &mut Context| {
            line = Path::new();
            line.move_to(x1, y1);
            line.line_to(x2, y2);


            if parsed.fill() {
                ctx.fill(Some(&mut line), parsed.fill_rule())
            }

            if parsed.stroke() {
                ctx.stroke(Some(&mut line))
            }
        };

        match parsed.mask() {
            None => {
                if let Some(filter) = parsed.filter() {
                    set_filters(context, filter);
                }

                handle_draw(context);
            }
            Some(mask) => match set_mask(
                mask,
                context,
                view_box,
                bounding_box,
                root_element,
                self.style(),
            ) {
                None => {}
                Some(_) => {}
            },
        }

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


        let stroke_width = context.line_width();
        Marker::set_markers(&mut self.markers, &line);
        match self.style.get(&Attribute::MarkerStart) {
            None => {}
            Some(start) => {
                if let Some(start) = Marker::from_id(start, root_element) {
                    start.render_marker(
                        context,
                        view_box,
                        root_element,
                        stroke_width,
                        Some((self.markers.0.as_slice(), self.markers.1.as_slice())),
                        MarkerPosition::Start,
                    );
                }
            }
        }

        match self.style.get(&Attribute::MarkerMid) {
            None => {}
            Some(mid) => {
                if let Some(mid) = Marker::from_id(mid, root_element) {
                    mid.render_marker(
                        context,
                        view_box,
                        root_element,
                        stroke_width,
                        Some((self.markers.0.as_slice(), self.markers.1.as_slice())),
                        MarkerPosition::Mid,
                    );
                }
            }
        }

        match self.style.get(&Attribute::MarkerEnd) {
            None => {}
            Some(end) => {
                if let Some(end) = Marker::from_id(end, root_element) {
                    end.render_marker(
                        context,
                        view_box,
                        root_element,
                        stroke_width,
                        Some((self.markers.0.as_slice(), self.markers.1.as_slice())),
                        MarkerPosition::End,
                    );
                }
            }
        }

        context.restore();
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
