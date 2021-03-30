use roxmltree::Node;
use skia_safe::{EncodedImageFormat, Point, Vector};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::context::fill_and_stroke_styles::pattern::{Pattern, Repetition};
use crate::common::context::matrix::Matrix;
use crate::common::svg::{create_context, handle_node, handle_transform, HEIGHT_ATTR, HREF_ATTR, parse_length, parse_length_min, parse_style, ParsedStyle, PATTERN_TRANSFORM_ATTR, VIEW_BOX_ATTR, ViewBox, WIDTH_ATTR, X_ATTR, XLINK_HREF_NS, Y_ATTR};
use crate::common::svg;

pub fn get_pattern_attributes(pattern: &roxmltree::Node) {}

pub fn handle_pattern<'a>(context: &mut Context, pattern: &roxmltree::Node, root: &'a roxmltree::Document<'a>, parent_attributes: Option<HashMap<&str, &str>>) -> Option<Pattern> {
    let mut reference: Option<roxmltree::Node> = None;
    let pattern_transform = pattern.attribute(PATTERN_TRANSFORM_ATTR);
    let mut href: Option<&str>;
    let scale = context.device.density;
    match (pattern.attribute(HREF_ATTR), pattern.attribute((XLINK_HREF_NS, HREF_ATTR))) {
        (Some(href_), Some(xlink_href)) => {
            href = Some(href_)
        }
        (Some(href_), _) => {
            href = Some(href_)
        }
        (_, Some(xlink_href)) => {
            href = Some(xlink_href)
        }
        (_, _) => {
            href = None
        }
    }
    let mut x = "0";
    let mut y = "0";
    let parent_width = context.surface.width();
    let parent_height = context.surface.height();
    let mut width = "0";
    let mut height = "0";


    if let Some(href) = href {
        let id = href.replace("url('#", "").replace("')", "").replace("url(#", "").replace(")", "").replace("#", "");
        if let Some(node) = root.descendants().find(|v| v.attribute("id") == Some(&id)) {
            reference = Some(node);
            if let Some(val) = node.attribute(X_ATTR) {
                x = val;
            }

            if let Some(val) = node.attribute(Y_ATTR) {
                y = val
            }

            if let Some(val) = node.attribute(WIDTH_ATTR)
            {
                width = val;
            }
            if let Some(val) = node.attribute(HEIGHT_ATTR)
            {
                height = val
            }
        }
    }

    if let Some(val) = pattern.attribute(X_ATTR) {
        x = val;
    }

    if let Some(val) = pattern.attribute(Y_ATTR) {
        y = val
    }

    if let Some(val) = pattern.attribute(WIDTH_ATTR)
    {
        width = val;
    }
    if let Some(val) = pattern.attribute(HEIGHT_ATTR)
    {
        height = val
    }

    let x = parse_length(x, parent_width, scale);
    let y = parse_length(y, parent_height, scale);
    let mut width = parse_length_min(width, parent_width, scale, 1.0);
    let mut height = parse_length_min(height, parent_height, scale, 1.0);


    if width <= 0.0 || height <= 0.0 {
        return None;
    }

    let view_box = ViewBox::from(pattern.attribute(VIEW_BOX_ATTR).unwrap_or(""));

    let mut pattern_context = create_conqwtext(
        width, height, context.device.density, context.device.alpha, 0, context.device.ppi, context.direction(),
    );

    if view_box.is_valid() {
        let mut scale_x = width / view_box.width;
        let mut scale_y = height / view_box.height;
        let mut matrix = pattern_context.get_transform();
        matrix.pre_scale(
            (scale_x, scale_y), None,
        );
        let mut translate_x = x - (view_box.x * scale_x);
        let mut translate_y = y - (view_box.y * scale_y);
        matrix.post_translate(Vector::new(translate_x, translate_y));
        pattern_context.set_transform_matrix(&matrix);
    }


    let mut handle = |pattern: &Node| {
        //let pattern = pattern.clone();
        let mut parent = Rc::new(RefCell::new(parse_style(&mut pattern_context, &pattern, root)));
        for node in pattern.children().into_iter() {
            let name = node.tag_name().name();
            if node.is_element() && !name.is_empty() {
                handle_node(&mut pattern_context, node, root, Rc::clone(&parent), None)
            }
        }
    };

    if let Some(node) = reference {
        handle(&node)
    } else {
        handle(pattern)
    }

    let snapshot = pattern_context.surface.image_snapshot();
    let info = skia_safe::ISize::new(width as i32, height as i32);
    let mut surface = skia_safe::Surface::new_raster_n32_premul(
        info
    ).unwrap();
    let canvas = surface.canvas();
    let mut paint = skia_safe::Paint::default();
    paint.set_anti_alias(true);
    paint.set_alpha_f(context.global_alpha());
    canvas.draw_image_rect(
        snapshot, None, skia_safe::Rect::from_xywh(0f32, 0f32, width, height), &paint,
    );
    let snapshot = surface.image_snapshot();
    let mut pattern = context.create_pattern(snapshot, Repetition::Repeat);
    if let Some(transform) = pattern_transform {
        let transform_map = svg::get_transform_map(transform);
        let mut matrix = skia_safe::Matrix::default();
        handle_transform(&transform_map, &mut matrix, width as i32, context.device.density);
        if let Some(m) = matrix.invert() {
            matrix = m
        }
        let matrix = Matrix::from(&matrix);
        pattern.set_pattern_transform(&matrix);
    } else {}
    let mut matrix = skia_safe::Matrix::translate(Vector::new(x, y));
    if let Some(inverted) = matrix.invert() {
        //  matrix = inverted
    }
    let pattern_matrix = Matrix::from(&matrix);
    //pattern.set_pattern_transform(&pattern_matrix);
    Some(pattern)
}
