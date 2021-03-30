use std::io::{Read, Seek, SeekFrom};
use skia_safe::{
    AlphaType, Color, ColorType, ImageFilter, ImageInfo, ISize, Paint, Point, Rect, Surface, Vector,
};
use skia_safe::paint::Style;

use crate::common::context::{Context};
mod attribute_names;
mod bounding_box;
mod elements;
mod enums;
mod error;
mod prelude;
mod units;
mod view_box;
use crate::common::svg::prelude::*;

use self::elements::{element_names::ElementName, svg::Svg};

pub(crate) fn draw_svg_from_path(context: &mut Context, path: &str) {
    let file = std::fs::File::open(path);
    match file {
        Ok(file) => {
            let mut reader = std::io::BufReader::new(file);
            let mut bytes = [0; 16];
            let result = reader.read(&mut bytes);
            match result {
                Ok(_) => {
                    let _ = reader.seek(SeekFrom::Start(0));
                    // TODO check bytes to verify it's an svg

                    let mut svg = Vec::new();
                    match reader.read_to_end(&mut svg) {
                        Ok(_) => {
                            let svg = std::string::String::from_utf8_lossy(&svg);
                            draw_svg(context, &svg);
                        }
                        Err(e) => {
                            println!("svg read to string error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("svg file read error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("svg file open error: {}", e);
        }
    }
}

pub(crate) fn draw_svg(context: &mut Context, svg: &str) {
    let mut svg = String::from(svg);
    if !svg.contains(r#"xmlns:xlink="http://www.w3.org/1999/xlink""#) {
        svg = svg.replace(
            "<svg ",
            "<svg xmlns:xlink=\"http://www.w3.org/1999/xlink\" ",
        );
    }

    match roxmltree::Document::parse(&svg) {
        Ok(document) => {
            let root_element = document.root_element();
            let device = context.device;
            let max_width = context.surface.width() as f32;
            let max_height = context.surface.height() as f32;
            if root_element.is_element() && root_element.tag_name().name() == ElementName::Svg.to_str() {
                if let Some(mut svg) = Svg::new_node(
                    root_element,
                    device,
                    max_width,
                    max_height,
                    device.density,
                    device.alpha,
                    context.font_color.to_int() as i32,
                    device.ppi,
                    context.direction(),
                ) {
                    svg.render_to_context(context);
                }
            }
        }
        Err(error) => {
            log::debug!("document parse error: {}", error);
        }
    }
}
