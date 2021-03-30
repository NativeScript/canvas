use std::collections::HashMap;

use roxmltree::{Document, Node};

use crate::common::context::{Context, Device};
use crate::common::svg::attribute_names::Attribute;
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::mask::Mask;
use crate::common::svg::elements::parser::{Parser, StyleMap};
use crate::common::svg::elements::renderer::{
    handle_mask, handle_style_data, render_mask, Renderer, set_filters, set_mask,
};
use crate::common::svg::elements::svg::{create_context, Svg};
use crate::common::svg::prelude::ColorConversation;
use crate::common::svg::view_box::ViewBox;

#[derive(Clone, Debug)]
pub struct Path {
    d: String,
    style: StyleMap,
    bounding_box: BoundingBox,
}

impl Path {
    pub fn new(d: &str) -> Self {
        Self {
            d: d.to_owned(),
            style: Default::default(),
            bounding_box: Default::default(),
        }
    }
    pub fn zero() -> Self {
        Self {
            d: "".to_owned(),
            style: HashMap::new(),
            bounding_box: Default::default(),
        }
    }

    pub fn d(&self) -> &str {
        &self.d
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
        if let Some(path) = skia_safe::Path::from_svg(&self.d) {
            let bounds = path.bounds();
            self.bounding_box
                .set_all(bounds.x(), bounds.y(), bounds.width(), bounds.height())
        }
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::zero()
    }
}

impl Parser for Path {
    fn parse_from(node: Node) -> Self {
        let style = Path::style_from(node);
        let mut path = Path::default();
        path.d = node
            .attribute(Attribute::D.to_str())
            .unwrap_or("")
            .to_string();
        path.style.extend(style);
        path
    }
}

impl Renderer for Path {
    fn render(&mut self, context: &mut Context, node: Node, root_element: Node) {
        context.save();
        let bounding_box = self.bounding_box;
        let parsed = handle_style_data(self.style(), context, root_element, bounding_box);
        if !parsed.is_visible() {
            return;
        }
        context.begin_path();
        let handle_draw = |ctx: &mut Context| {
            let mut path = crate::common::context::paths::path::Path::from_str(self.d());

            if parsed.fill() {
                ctx.fill(Some(&mut path), parsed.fill_rule())
            }

            if parsed.stroke() {
                ctx.stroke(Some(&mut path))
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

        // let mut points = Vec::new();
        // path.path.get_points(&mut points);
        //
        // points[0].
        // return Math.atan2(point.y - this.y, point.x - this.x);

        context.restore();
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
