use image as img;
use image::{DynamicImage, ImageError};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::context::Context;
use crate::common::svg::{BEFORE_STROKE_EVENT, DrawType, HEIGHT_ATTR, HREF_ATTR, parse_length, ParsedStyle, WIDTH_ATTR, X_ATTR, XLINK_HREF_NS, Y_ATTR};
use crate::common::svg;

use super::ViewBox;

pub fn handle_image<'a>(context: &mut Context, image: roxmltree::Node, root: &'a roxmltree::Document<'a>, style: Rc<RefCell<ParsedStyle>>, parent_attributes: Option<HashMap<&str, &str>>) {
    let scale = context.device.density;
    let mut x = "0";
    let mut y = "0";
    let mut width = "0";
    let mut height = "0";
    if let Some(map) = parent_attributes {
        if let Some(x_) = map.get("x") {
            x = &*x_
        }

        if let Some(y_) = map.get("y") {
            y = &*y_
        }

        if let Some(w) = map.get("width") {
            width = &*w
        }

        if let Some(h) = map.get("height") {
            height = &*h
        }
    }

    if let Some(x_) = image.attribute(X_ATTR) {
        x = x_
    }

    if let Some(y_) = image.attribute(Y_ATTR) {
        y = y_
    }

    if let Some(width_) = image.attribute(WIDTH_ATTR) {
        width = width_
    }

    if let Some(height_) = image.attribute(HEIGHT_ATTR) {
        height = height_
    }


    let mut x = x.parse::<f32>().unwrap_or(0f32) * scale;
    let mut y = y.parse::<f32>().unwrap_or(0f32) * scale;
    let parent_width = context.surface.width();
    let parent_height = context.surface.height();
    let mut url = "";

    if let Some(href) = image.attribute((XLINK_HREF_NS, HREF_ATTR)) {
        url = href;
    } else if let Some(href) = image.attribute(HREF_ATTR) {
        url = href;
    }

    let width = parse_length(width, parent_width, scale);
    let height = parse_length(height, parent_height, scale);
    let view_box = ViewBox::new(x, y, width, height);
    context.save();
    if url.starts_with("data:") {
        let url: Vec<_> = url.split(",").collect();
        if url.len() > 1 {
            let data = decode_base64(url[1]);
            svg::handle_drawing(context, style, None, DrawType::Image, Some(Box::new(move |event, context| {
                match event {
                    BEFORE_FILL_EVENT => {
                        unsafe {
                            let data = skia_safe::Data::new_bytes(&data);
                            if let Some(image) = skia_safe::Image::from_encoded(data) {
                                context.draw_image(
                                    &image,
                                    skia_safe::Rect::from_xywh(0f32, 0f32, image.width() as f32, image.height() as f32),
                                    skia_safe::Rect::from_xywh(x, y, width, height),
                                )
                            }
                        }
                    }
                    _ => {}
                }
            })), root, &view_box);
        }
    } else if url.contains("http") {
        let url = url.to_string();
        svg::handle_drawing(context, style, None, DrawType::Image, Some(Box::new(move |event, context| {
            match event {
                BEFORE_FILL_EVENT => {
                    if let Ok(response) = reqwest::blocking::get(&url) {
                        if let Ok(bytes) = response.bytes() {
                            unsafe {
                                let data = skia_safe::Data::new_bytes(&bytes);
                                if let Some(image) = skia_safe::Image::from_encoded(data) {
                                    context.draw_image(
                                        &image,
                                        skia_safe::Rect::from_xywh(0f32, 0f32, image.width() as f32, image.height() as f32),
                                        skia_safe::Rect::from_xywh(x, y, width, height),
                                    )
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        })), root, &view_box);
    }
    context.restore();
}

type StaticCharVec = &'static [char];

const HTML_SPACE_CHARACTERS: StaticCharVec =
    &['\u{0020}', '\u{0009}', '\u{000a}', '\u{000c}', '\u{000d}'];

// https://github.com/servo/servo/blob/1610bd2bc83cea8ff0831cf999c4fba297788f64/components/script/dom/window.rs#L575
fn decode_base64(value: &str) -> Vec<u8> {
    fn is_html_space(c: char) -> bool {
        HTML_SPACE_CHARACTERS.iter().any(|&m| m == c)
    }
    let without_spaces = value
        .chars()
        .filter(|&c| !is_html_space(c))
        .collect::<String>();
    let mut input = &*without_spaces;

    if input.len() % 4 == 0 {
        if input.ends_with("==") {
            input = &input[..input.len() - 2]
        } else if input.ends_with("=") {
            input = &input[..input.len() - 1]
        }
    }

    if input.len() % 4 == 1 {
        return Vec::new();
    }

    if input
        .chars()
        .any(|c| c != '+' && c != '/' && !c.is_alphanumeric())
    {
        return Vec::new();
    }
    match base64::decode_config(&input, base64::STANDARD.decode_allow_trailing_bits(true)) {
        Ok(bytes) => bytes,
        Err(e) => {
            return Vec::new();
        }
    }
}
