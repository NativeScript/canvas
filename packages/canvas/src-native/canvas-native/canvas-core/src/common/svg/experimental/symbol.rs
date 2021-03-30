use roxmltree::Node;
use skia_safe::{EncodedImageFormat, Point, Rect, Vector};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::context::fill_and_stroke_styles::paint::PaintStyle;
use crate::common::context::fill_and_stroke_styles::pattern::{Pattern, Repetition};
use crate::common::context::matrix::Matrix;
use crate::common::svg::{create_context, Display, DrawType, handle_node, handle_transform, HEIGHT_ATTR, HREF_ATTR, IMAGE_NS, parse_length, parse_style, ParsedStyle, PATTERN_TRANSFORM_ATTR, TRANSFORM_ATTR, VIEW_BOX_ATTR, ViewBox, Visibility, WIDTH_ATTR, X_ATTR, XLINK_HREF_NS, Y_ATTR};
use crate::common::svg;

pub fn handle_symbol<'a>(context: &mut Context, symbol: roxmltree::Node<'a, '_>, root: &'a roxmltree::Document<'a>, style: Rc<RefCell<ParsedStyle<'a>>>, parent_attributes: Option<HashMap<&str, &str>>) {
    let mut reference: Option<roxmltree::Node> = None;
    let transform = symbol.attribute(TRANSFORM_ATTR);
    let mut href: Option<&str>;
    match (symbol.attribute(HREF_ATTR), symbol.attribute((XLINK_HREF_NS, HREF_ATTR))) {
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
    let scale = context.device.density;
    let parent_width = context.surface.width();
    let parent_height = context.surface.height();
    let mut width = "auto";
    let mut height = "auto";

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

    if let Some(val) = symbol.attribute(X_ATTR) {
        x = val;
    }

    if let Some(val) = symbol.attribute(Y_ATTR) {
        y = val
    }

    if let Some(val) = symbol.attribute(WIDTH_ATTR)
    {
        width = val;
    }
    if let Some(val) = symbol.attribute(HEIGHT_ATTR)
    {
        height = val
    }

    let x = parse_length(x, parent_width, scale);
    let y = parse_length(y, parent_height, scale);
    let mut width = parse_length(width, parent_width, scale);
    let mut height = parse_length(height, parent_height, scale);

    if width <= 0.0 || height <= 0.0 {
        return;
    }
    let mut view_box = ViewBox::from(symbol.attribute(VIEW_BOX_ATTR).unwrap_or(""));
    if width <= 0.0 && height <= 0.0 {
        return;
    }
    let mut symbol_context = create_context(
        width, height, context.device.density, context.device.alpha, 0, context.device.ppi, context.direction(),
    );

    if view_box.is_valid() {
        let mut scale_x = width / view_box.width;
        let mut scale_y = height / view_box.height;
        let mut matrix = symbol_context.get_transform();
        matrix.pre_scale(
            (scale_x, scale_y), None,
        );
        let mut translate_x = x - (view_box.x * scale_x);
        let mut translate_y = y - (view_box.y * scale_y);
        matrix.post_translate(Vector::new(translate_x, translate_y));
        symbol_context.set_transform_matrix(&matrix);
    }

    if let Some(transform) = transform {
        let transform_map = svg::get_transform_map(transform);
        let width = symbol_context.surface.width();
        let mut matrix = symbol_context.get_transform();
        handle_transform(&transform_map, &mut matrix, width, symbol_context.device.density);
        symbol_context.set_transform_matrix(&matrix)
    }
    let mut update = parse_style(&mut symbol_context, &symbol, root);
    update.parent = Some(Rc::clone(&style));


    if update.visibility == Visibility::Hidden || update.visibility == Visibility::Collapse || update.display == Display::None {
        return;
    }


    let update = Rc::new(RefCell::new(update));
    let mut handle = |symbol: Node<'a, '_>| {
        for node in symbol.children().into_iter() {
            let name = node.tag_name().name();
            if node.is_element() && !name.is_empty() {
                match name {
                    IMAGE_NS => {}
                    _ => {
                        handle_node(&mut symbol_context, node, root, Rc::clone(&style), parent_attributes.clone())
                    }
                }
            }
        }
    };

    if let Some(node) = reference {
        handle(node)
    } else {
        handle(symbol)
    }

    let snapshot = symbol_context.surface.image_snapshot();

    let info = skia_safe::ImageInfo::new(
        skia_safe::ISize::new(width as i32, height as i32),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );
    let mut surface = skia_safe::Surface::new_raster(
        &info,
        None,
        None,
    ).unwrap();
    let canvas = surface.canvas();
    let mut paint = skia_safe::Paint::default();
    paint.set_anti_alias(true);
    paint.set_alpha_f(context.global_alpha());
    canvas.draw_image_rect(
        snapshot, None, skia_safe::Rect::from_xywh(0f32, 0f32, width, height), &paint,
    );
    let snapshot = surface.image_snapshot();

    svg::handle_drawing(context, Rc::clone(&update), None, DrawType::Image, Some(Box::new(move |event, context| {
        match event {
            BEFORE_STROKE_EVENT => {
                context.save();
                context.translate(x, y);
                context.draw_image_with_points(
                    &snapshot,
                    0f32,
                    0f32,
                );
                context.restore();
            }
            _ => {}
        }
    })), root, &view_box);
}
