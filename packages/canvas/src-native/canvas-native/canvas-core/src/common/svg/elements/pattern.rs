use std::{cell::RefCell, rc::Rc, str::FromStr};

use roxmltree::Node;
use skia_safe::Matrix;

use crate::common::context::Context;
use crate::common::context::fill_and_stroke_styles::pattern::Repetition;
use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::bounding_box::BoundingBox;
use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::element_names::ElementName::Rect;
use crate::common::svg::elements::parser::parse_transform;
use crate::common::svg::elements::prelude::style_from;
use crate::common::svg::elements::reference_element::ReferenceElement;
use crate::common::svg::elements::renderer::{
    handle_render_children, handle_style, handle_style_data,
};
use crate::common::svg::elements::svg::create_context;
use crate::common::svg::enums::preserve_aspect_ratio::view_box_to_transform;
use crate::common::svg::prelude::ColorConversation;
use crate::common::svg::units::length::{convert_length, Length};
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;

#[derive(Clone, Debug)]
pub struct Pattern {
    x: Length,
    y: Length,
    width: Length,
    height: Length,
    pattern_content_units: Units,
    pattern_units: Units,
    pattern_transform: skia_safe::Matrix,
    view_box: Option<ViewBox>,
    node_to_render: Option<String>,
}

impl Pattern {
    pub fn x(&self) -> Length {
        self.x
    }

    pub fn y(&self) -> Length {
        self.y
    }

    pub fn width(&self) -> Length {
        self.width
    }

    pub fn height(&self) -> Length {
        self.height
    }

    pub fn pattern_content_units(&self) -> Units {
        self.pattern_content_units
    }

    pub fn pattern_units(&self) -> Units {
        self.pattern_units
    }

    pub fn pattern_transform(&self) -> &skia_safe::Matrix {
        &self.pattern_transform
    }

    pub fn pattern_transform_mut(&mut self) -> &mut skia_safe::Matrix {
        &mut self.pattern_transform
    }

    pub fn create_pattern(
        &self,
        context: &mut Context,
        view_box: ViewBox,
        bounding_box: BoundingBox,
        root_element: Node,
    ) -> Option<crate::common::context::fill_and_stroke_styles::pattern::Pattern> {
        if let Some(id) = self.node_to_render.as_ref() {
            if let (Some(node), _) = root_element.get_node(id) {
                let density = context.device.density;
                let alpha = context.device.alpha;
                let font_color = context.font_color.to_int() as i32;
                let ppi = context.device.ppi;
                let direction = context.direction();
                let mut view_box = view_box;
                match self.pattern_units {
                    Units::ObjectBoundingBox => {
                        view_box = bounding_box.to_view_box();
                    }
                    _ => {}
                }

                let mut x = convert_length(
                    self.x,
                    Attribute::X,
                    self.pattern_units,
                    context.device,
                    &view_box,
                );
                let mut y = convert_length(
                    self.y,
                    Attribute::Y,
                    self.pattern_units,
                    context.device,
                    &view_box,
                );

                let mut width = convert_length(
                    self.width,
                    Attribute::Width,
                    self.pattern_units,
                    context.device,
                    &view_box,
                );
                let mut height = convert_length(
                    self.height,
                    Attribute::Height,
                    self.pattern_units,
                    context.device,
                    &view_box,
                );

                if self.pattern_units == Units::ObjectBoundingBox {
                    x = x * view_box.width() + view_box.x();
                    y = y * view_box.width() + view_box.y();
                    width = width * view_box.width();
                    height = height * view_box.height();
                }

                let mut pattern_context =
                    create_context(width, height, density, alpha, font_color, ppi, direction);

                if let Some(view_box) = self.view_box {
                    let preserve_aspect_ratio = node.get_preserve_aspect_ratio();
                    let mat =
                        view_box_to_transform(&view_box, preserve_aspect_ratio, width, height);
                    pattern_context.transform_with_matrix(&mat);
                } else if self.pattern_content_units == Units::ObjectBoundingBox {
                    pattern_context.scale(view_box.width(), view_box.height());
                }

                let style = style_from(node);
                let parsed =
                    handle_style_data(&style, &mut pattern_context, root_element, bounding_box);

                if !parsed.is_visible() {
                    return None;
                }

                pattern_context.translate(x, y);

                if node.tag_name().name() == ElementName::Pattern.to_str() {
                    let mut children = node.children();
                    handle_render_children(&mut pattern_context, &mut children, root_element);
                }
                let image = pattern_context.surface.image_snapshot();
                return Some(context.create_pattern(image, Repetition::Repeat));
            }
        }
        None
    }
}

impl ReferenceElement for Pattern {
    fn element_type() -> ElementName {
        ElementName::Pattern
    }

    fn from_node(node: (Option<Node>, Option<Node>)) -> Option<Self> {
        if node.0.is_none() {
            return None;
        }

        let mut x: Option<Length> = None;
        let mut y: Option<Length> = None;
        let mut width: Option<Length> = None;
        let mut height: Option<Length> = None;
        let mut pattern_content_units: Option<Units> = None;
        let mut pattern_units: Option<Units> = None;
        let mut pattern_transform: Option<skia_safe::Matrix> = None;
        let mut view_box: Option<ViewBox> = None;
        let mut node_to_render: Option<String> = None;

        let mut handle_pattern = |node: &Node| {
            if pattern_transform.is_none() {
                match node.attribute(Attribute::PatternTransform.to_str()) {
                    // TODO
                    Some(transform) => {
                        pattern_transform = parse_transform(transform, 1.0);
                    }
                    _ => {}
                }
            }
            if x.is_none() {
                x = node.get_length_opt(Attribute::X);
            }

            if y.is_none() {
                y = node.get_length_opt(Attribute::Y);
            }
            if width.is_none() {
                width = node.get_length_opt(Attribute::Width);
            }

            if height.is_none() {
                height = node.get_length_opt(Attribute::Height);
            }

            if pattern_content_units.is_none() {
                pattern_content_units = Units::from_str(
                    node.attribute(Attribute::PatternContentUnits.to_str())
                        .unwrap_or(""),
                );
            }

            if pattern_units.is_none() {
                pattern_units = Units::from_str(
                    node.attribute(Attribute::PatternUnits.to_str())
                        .unwrap_or(""),
                );
            }

            if view_box.is_none() {
                if let Some(vb) = node.attribute(Attribute::ViewBox.to_str()) {
                    if let Ok(vb) = ViewBox::from_str(vb) {
                        view_box = Some(vb)
                    }
                }
            }
        };
        match node {
            (Some(node), Some(ref_node)) => {
                handle_pattern(&node);
                handle_pattern(&ref_node);
                if let Some(id) = node.attribute(Attribute::Id.to_str()) {
                    node_to_render = Some(id.to_owned());
                }
            }
            (Some(node), None) => {
                handle_pattern(&node);
                if let Some(id) = node.attribute(Attribute::Id.to_str()) {
                    node_to_render = Some(id.to_owned());
                }
            }
            (_, _) => return None,
        };

        Some(Self {
            x: x.unwrap_or(Length::zero()),
            y: y.unwrap_or(Length::zero()),
            width: width.unwrap_or(Length::zero()),
            height: height.unwrap_or(Length::zero()),
            pattern_content_units: pattern_content_units.unwrap_or(Units::UserSpaceOnUse),
            pattern_units: pattern_units.unwrap_or(Units::ObjectBoundingBox),
            pattern_transform: pattern_transform.unwrap_or(skia_safe::Matrix::default()),
            view_box,
            node_to_render,
        })
    }
}
