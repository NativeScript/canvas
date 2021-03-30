use roxmltree::Node;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use crate::common::{context::Context, svg::SVG_NS};
use crate::common::context::fill_and_stroke_styles::gradient::Gradient;
use crate::common::context::matrix::Matrix;
use crate::common::svg;
use crate::common::svg::{convert_length_min, CX_ATTR, CY_ATTR, FR_ATTR, FX_ATTR, FY_ATTR, get_real_node, gradient, GRADIENT_TRANSFORM_ATTR, GRADIENT_UNITS_ATTR, handle_transform, HREF_ATTR, parse_length, ParsedStyle, R_ATTR, SPREAD_METHOD_ATTR, STOP_NS, VIEW_BOX_ATTR, ViewBox, XLINK_HREF_NS};
use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::length::{convert_length, Length, LengthUnit, Units};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Line {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl Line {
    /// Creates a new line.
    #[inline]
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line { x1, y1, x2, y2 }
    }

    /// Calculates the line length.
    #[inline]
    pub fn length(&self) -> f32 {
        let x = self.x2 - self.x1;
        let y = self.y2 - self.y1;
        (x * x + y * y).sqrt()
    }

    /// Sets the line length.
    pub fn set_length(&mut self, len: f32) {
        let x = self.x2 - self.x1;
        let y = self.y2 - self.y1;
        let len2 = (x * x + y * y).sqrt();
        let line = Line {
            x1: self.x1,
            y1: self.y1,
            x2: self.x1 + x / len2,
            y2: self.y1 + y / len2,
        };

        self.x2 = self.x1 + (line.x2 - line.x1) * len;
        self.y2 = self.y1 + (line.y2 - line.y1) * len;
    }
}

fn prepare_focal(cx: f32, cy: f32, r: f32, fx: f32, fy: f32) -> (f32, f32) {
    let max_r = r - r * 0.001;

    let mut line = Line::new(cx, cy, fx, fy);

    if line.length() > max_r {
        line.set_length(max_r);
    }

    (line.x2, line.y2)
}

pub fn handle_radial_gradient<'a>(
    context: &mut Context,
    radial: &roxmltree::Node,
    root: &'a roxmltree::Document<'a>,
) -> Gradient {
    let mut reference: Option<roxmltree::Node> = None;
    let width = context.surface.width();
    let height = context.surface.height();
    let scale = context.device.density;

    let mut href: Option<&str>;
    match (
        radial.attribute(HREF_ATTR),
        radial.attribute((XLINK_HREF_NS, HREF_ATTR)),
    ) {
        (Some(href_), Some(xlink_href)) => href = Some(href_),
        (Some(href_), _) => href = Some(href_),
        (_, Some(xlink_href)) => href = Some(xlink_href),
        (_, _) => href = None,
    }

    let mut cx = Length::new(50.0, LengthUnit::Percent);
    let mut cy = Length::new(50.0, LengthUnit::Percent);
    let mut fr = Length::new(0.0, LengthUnit::None);
    let mut fx = Length::new(50.0, LengthUnit::Percent);
    let mut fy = Length::new(50.0, LengthUnit::Percent);
    let mut r = Length::new(50.0, LengthUnit::Percent);
    let mut spread_method = radial.attribute(SPREAD_METHOD_ATTR).unwrap_or("pad");
    // scale ??
    let mut gradient_transform: Option<&str> = None;

    if let Some(href) = href {
        get_real_node(href, root, |node, last| {
            cx = node.get_length(Attribute::Cx, Length::new(50.0, LengthUnit::Percent));
            cy = node.get_length(Attribute::Cy, Length::new(50.0, LengthUnit::Percent));
            r = node.get_length(Attribute::R, Length::new(50.0, LengthUnit::Percent));
            fx = node.get_length(Attribute::Fx, Length::new(50.0, LengthUnit::Percent));
            fy = node.get_length(Attribute::Fy, Length::new(50.0, LengthUnit::Percent));

            if let Some(transform) = node.attribute(GRADIENT_TRANSFORM_ATTR) {
                gradient_transform = Some(transform);
            }
            if last {
                reference = Some(node);
            }
        });
    }

    let cx = radial.get_length(Attribute::Cx, cx);
    let cy = radial.get_length(Attribute::Cy, cy);
    let r = radial.get_length(Attribute::R, r);
    let fx = radial.get_length(Attribute::Fx, fx);
    let fy = radial.get_length(Attribute::Fy, fy);

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
    let unit = match radial.attribute(GRADIENT_UNITS_ATTR).unwrap_or("userSpaceOnUse") {
        "userSpaceOnUse" => Units::UserSpaceOnUse,
        _ => Units::ObjectBoundingBox
    };
    let cx = convert_length(
        cx,
        Attribute::Cx,
        unit,
        &context.device,
        &view_box,
    );
    let cy = convert_length(
        cy,
        Attribute::Cy,
        unit,
        &context.device,
        &view_box,
    );
    let r = convert_length(
        r,
        Attribute::R,
        unit,
        &context.device,
        &view_box,
    );
    let fx = convert_length(
        fx,
        Attribute::Fx,
        unit,
        &context.device,
        &view_box,
    );
    let fy = convert_length(
        fy,
        Attribute::Fy,
        unit,
        &context.device,
        &view_box,
    );

    let fr = 0.0;

    if let Some(transform) = radial.attribute(GRADIENT_TRANSFORM_ATTR) {
        gradient_transform = Some(transform);
    }


    let mut width = view_box.width();
    let mut height = view_box.height();
    // let fx = fx * scale * width;
    // let fy = fy * scale * height;
    // let fr = 0.0;
    // let r = ((width + height) / 2.0) * r * scale;
    // let cx = cx * scale * width;
    // let cy = cy * scale * height;
    //let (fx, fy) = prepare_focal(cx, cy, r, fx, fy);
    let mut gradient;
    if let Some(transform) = gradient_transform {
        if !transform.is_empty() {
            let transform_map = svg::get_transform_map(transform);
            let mut matrix = skia_safe::Matrix::default();
            handle_transform(&transform_map, &mut matrix, width as i32, scale);
            if let Some(m) = matrix.invert() {
                matrix = m
            }
            let matrix = Matrix::from(&matrix);
            gradient = context.create_radial_gradient_with_matrix(fx, fy, fr, cx, cy, r, matrix);
        } else {
            gradient = context.create_radial_gradient(fx, fy, fr, cx, cy, r);
        }
    } else {
        gradient = context.create_radial_gradient(fx, fy, fr, cx, cy, r);
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
                let style = Rc::new(RefCell::new(svg::parse_style(context, radial, root)));
                gradient::handle_stop(&node, &mut gradient, style);
            }
        }
    } else {
        for node in radial.children().into_iter() {
            if node.is_element() && node.tag_name().name() == STOP_NS {
                let style = Rc::new(RefCell::new(svg::parse_style(context, radial, root)));
                gradient::handle_stop(&node, &mut gradient, style);
            }
        }
    }
    gradient
}
