use roxmltree::Node;
use std::collections::HashMap;

use crate::common::context::{Context, Device};
use crate::common::context::paths::path::Path;
use crate::common::svg::attribute_names::Attribute;
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::marker::{Marker, MarkerPosition};
use crate::common::svg::elements::parser::{Parser, points_from, StyleMap};
use crate::common::svg::elements::reference_element::ReferenceElement;
use crate::common::svg::elements::renderer::{
    handle_style_data, render_mask, Renderer, set_filters, set_mask,
};
use crate::common::svg::view_box::ViewBox;

#[derive(Clone, Debug)]
pub struct PolyShape {
    points: Vec<Vec<f32>>,
    style: StyleMap,
    is_polygon: bool,
    bounding_box: BoundingBox,
}

impl PolyShape {
    fn new(is_polygon: bool, points: Vec<Vec<f32>>) -> Self {
        Self {
            points,
            style: Default::default(),
            is_polygon,
            bounding_box: Default::default(),
        }
    }
    pub fn new_polygon(points: Vec<Vec<f32>>) -> Self {
        Self {
            is_polygon: true,
            points,
            bounding_box: Default::default(),
            style: Default::default(),
        }
    }

    pub fn new_line(points: Vec<Vec<f32>>) -> Self {
        Self {
            is_polygon: false,
            points,
            bounding_box: Default::default(),
            style: Default::default(),
        }
    }

    fn zero(is_polygon: bool) -> Self {
        Self {
            is_polygon,
            points: Vec::new(),
            style: HashMap::new(),
            bounding_box: Default::default(),
        }
    }

    pub fn zero_polygon() -> Self {
        Self::zero(true)
    }

    pub fn zero_polyline() -> Self {
        Self::zero(false)
    }

    pub fn points(&self) -> &Vec<Vec<f32>> {
        &self.points
    }

    pub fn style(&self) -> &StyleMap {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut StyleMap {
        &mut self.style
    }

    pub fn is_polygon(&self) -> bool {
        self.is_polygon
    }

    pub fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    pub fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }

    pub fn update_bounding_box(&mut self, width: f32, height: f32, device: Device) {
        if let Some(path) = self.render_values(device.density) {
            let bounds = path.path.bounds();
            self.bounding_box
                .set_all(bounds.x(), bounds.y(), bounds.width(), bounds.height())
        }
    }

    fn render_values(&self, scale: f32) -> Option<Path> {
        if !self.points.is_empty() {
            let mut poly = Path::new();
            for (index, value) in self.points.iter().enumerate() {
                if index == 0 {
                    &mut poly.move_to(*value.first().unwrap() * scale, *value.last().unwrap() * scale);
                } else {
                    &mut poly.line_to(*value.first().unwrap() * scale, *value.last().unwrap() * scale);
                }
            }

            if !self.is_polygon {
                poly.close_path();
            }
            return Some(poly);
        }
        None
    }
}

impl Parser for PolyShape {
    fn parse_from(node: Node) -> Self {
        let style = PolyShape::style_from(node);
        let is_polygon = node.tag_name().name() == ElementName::Polygon.to_str();
        let mut polyshape = PolyShape::zero(is_polygon);
        polyshape.points = points_from(node);
        polyshape.style.extend(style);
        polyshape
    }
}

impl Renderer for PolyShape {
    fn render(&mut self, context: &mut Context, node: Node, root_element: Node) {
        if let Some(mut poly) = self.render_values(context.device.density) {
            context.save();
            let bounding_box = self.bounding_box;
            let parsed = handle_style_data(self.style(), context, root_element, bounding_box);
            if !parsed.is_visible() {
                return;
            }
            context.begin_path();

            let mut handle_draw = |ctx: &mut Context| {
                if parsed.fill() {
                    ctx.fill(Some(&mut poly), parsed.fill_rule())
                }

                if parsed.stroke() {
                    ctx.stroke(Some(&mut poly))
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

            let view_box = ViewBox::new_with_context(context);
            let stroke_width = context.line_width();
            match self.style.get(&Attribute::MarkerStart) {
                None => {}
                Some(start) => {
                    let mut points = Vec::new();
                    let mut angles = Vec::new();
                    poly.path.get_points(&mut points);
                    let last = points.len() - 1;
                    let mut position = 0;
                    for point in points.iter() {
                        if position == last {
                            return;
                        }
                        if let Some(next_point) = points.get(position + 1) {
                            let next_point = *next_point;
                            angles.push((next_point.y - point.y).atan2(next_point.x - point.x));
                        }

                        position += 1;
                    }

                    if let Some(start) = Marker::from_id(start, root_element) {
                        start.render_marker(context, view_box, root_element, stroke_width, Some((points.as_slice(), angles.as_slice())), MarkerPosition::Start);
                    }
                }
            }
            match self.style.get(&Attribute::MarkerMid) {
                None => {}
                Some(mid) => {}
            }

            match self.style.get(&Attribute::MarkerEnd) {
                None => {}
                Some(end) => {}
            }

            context.restore();
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
