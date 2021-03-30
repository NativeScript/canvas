use roxmltree::Node;
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::{
    context::fill_and_stroke_styles::gradient::Gradient,
    svg::{
        length::{convert_length, Units},
        SVG_NS, ViewBox,
    },
};
use crate::common::{
    context::Context,
    svg::{attribute_names::Attribute, attribute_names::NodeExt, length::Length},
};
use crate::common::context::matrix::Matrix;
use crate::common::svg;
use crate::common::svg::{get_real_node, gradient, GRADIENT_TRANSFORM_ATTR, GRADIENT_UNITS_ATTR, handle_transform, HREF_ATTR, SPREAD_METHOD_ATTR, STOP_NS, VIEW_BOX_ATTR, XLINK_HREF_NS};

pub fn handle_linear_gradient<'a>(
    context: &mut Context,
    linear: &roxmltree::Node,
    root: &'a roxmltree::Document<'a>,
) -> Gradient {
    let mut reference: Option<roxmltree::Node> = None;
    let width = context.surface.width();
    let height = context.surface.height();
    let scale = context.device.density;
    let mut href: Option<&str>;
    match (
        linear.attribute(HREF_ATTR),
        linear.attribute((XLINK_HREF_NS, HREF_ATTR)),
    ) {
        (Some(href_), Some(xlink_href)) => href = Some(href_),
        (Some(href_), _) => href = Some(href_),
        (_, Some(xlink_href)) => href = Some(xlink_href),
        (_, _) => href = None,
    }

    let mut x1 = Length::new(0.0, svg::length::LengthUnit::Percent);
    let mut y1 = Length::new(0.0, svg::length::LengthUnit::Percent);
    let mut x2 = Length::new(100.0, svg::length::LengthUnit::Percent);
    let mut y2 = Length::new(0.0, svg::length::LengthUnit::Percent);
    // scale ??
    let mut gradient_transform: Option<&str> = None;
    if let Some(href) = href {
        get_real_node(href, root, |node, last| {
            x1 = node.get_length(Attribute::X1, x1);
            y1 = node.get_length(Attribute::Y1, y1);
            x2 = node.get_length(Attribute::X2, x2);
            y2 = node.get_length(Attribute::Y2, y2);
            if let Some(transform) = node.attribute(GRADIENT_TRANSFORM_ATTR) {
                gradient_transform = Some(transform);
            }
            if last {
                reference = Some(node);
            }
        });
    }

    x1 = linear.get_length(Attribute::X1, x1);
    y1 = linear.get_length(Attribute::Y1, y1);
    x2 = linear.get_length(Attribute::X2, x2);
    y2 = linear.get_length(Attribute::Y2, y2);

    let mut view_box = ViewBox::new(0.0, 0.0, 0.0, 0.0);
    let mut root_node: Option<Node> = None;
    for desc in root.descendants() {
        if desc.tag_name().name() == SVG_NS {
            root_node = Some(desc);
            break;
        }
    }

    if let Some(node) = root_node {
        if let Some(value) = node.attribute(VIEW_BOX_ATTR) {
            view_box = ViewBox::from(value);
            if !view_box.is_valid() {
                view_box = ViewBox::new(0.0, 0.0, width as f32, height as f32);
            }
        } else {
            view_box = ViewBox::new(0.0, 0.0, width as f32, height as f32);
        }
    } else {
        view_box = ViewBox::new(0.0, 0.0, width as f32, height as f32);
    }
    let unit = match linear.attribute(GRADIENT_UNITS_ATTR).unwrap_or("userSpaceOnUse") {
        "userSpaceOnUse" => Units::UserSpaceOnUse,
        _ => Units::ObjectBoundingBox
    };

    let x1 = convert_length(
        x1,
        Attribute::X1,
        unit,
        &context.device,
        &view_box,
    );

    let y1 = convert_length(
        y1,
        Attribute::Y1,
        unit,
        &context.device,
        &view_box,
    );

    let x2 = convert_length(
        x2,
        Attribute::X2,
        unit,
        &context.device,
        &view_box,
    );

    let y2 = convert_length(
        y2,
        Attribute::Y2,
        unit,
        &context.device,
        &view_box,
    );

    let spread_method = linear.attribute(SPREAD_METHOD_ATTR).unwrap_or("pad");

    if let Some(transform) = linear.attribute(GRADIENT_TRANSFORM_ATTR) {
        gradient_transform = Some(transform);
    }

    let mut gradient;
    if let Some(transform) = gradient_transform {
        if !transform.is_empty() {
            let transform_map = svg::get_transform_map(transform);
            let mut matrix = skia_safe::Matrix::default();
            handle_transform(&transform_map, &mut matrix, width, scale);
            if let Some(m) = matrix.invert() {
                matrix = m
            }
            let matrix = Matrix::from(&matrix);
            gradient = context.create_linear_gradient_with_matrix(x1, y1, x2, y2, matrix);
        } else {
            gradient = context.create_linear_gradient(x1, y1, x2, y2);
        }
    } else {
        gradient = context.create_linear_gradient(x1, y1, x2, y2);
    }

    match spread_method {
        "pad" => gradient.set_tile_mode(skia_safe::TileMode::Clamp),
        "reflect" => gradient.set_tile_mode(skia_safe::TileMode::Mirror),
        "repeat" => gradient.set_tile_mode(skia_safe::TileMode::Repeat),
        _ => {}
    }

    if let Some(reference) = reference {
        for node in reference.children().into_iter() {
            if node.is_element() && node.tag_name().name() == STOP_NS {
                let style = Rc::new(RefCell::new(svg::parse_style(context, linear, root)));
                gradient::handle_stop(&node, &mut gradient, style);
            }
        }
    } else {
        for node in linear.children().into_iter() {
            if node.is_element() && node.tag_name().name() == STOP_NS {
                let style = Rc::new(RefCell::new(svg::parse_style(context, linear, root)));
                gradient::handle_stop(&node, &mut gradient, style);
            }
        }
    }
    gradient
}
