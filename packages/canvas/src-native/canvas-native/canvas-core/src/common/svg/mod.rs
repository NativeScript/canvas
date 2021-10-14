use std::io::{Read, Seek, SeekFrom};

use crate::common::context::Context;

// use skia_safe::{
//     AlphaType, Color, ColorType, ImageFilter, ImageInfo, ISize, Paint, Point, Rect, Surface, Vector,
// };
// use skia_safe::paint::Style;

// mod attribute_names;
// mod bounding_box;
// mod elements;
// mod enums;
// mod error;
// mod prelude;
// mod units;
// mod view_box;
//use crate::common::svg::prelude::*;

//use self::elements::{element_names::ElementName, svg::Svg};

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
                    match skia_safe::svg::SvgDom::read(reader) {
                        Ok(mut svg) => {
                            let device = context.device;
                            let size = skia_safe::Size::new(
                                context.surface.width() as f32,
                                context.surface.height() as f32,
                            );
                            let canvas = context.surface.canvas();
                            svg.container_size(&size);
                            //  canvas.scale((device.density, device.density));
                            svg.render(canvas)
                        }
                        Err(e) => {
                            println!("svg read to string error: {}", e);
                        }
                    }
                    // TODO check bytes to verify it's an svg
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
    match skia_safe::svg::SvgDom::from_bytes(svg.as_bytes()) {
        Ok(mut svg) => {
            let device = context.device;
            let size = skia_safe::Size::new(
                context.surface.width() as f32,
                context.surface.height() as f32,
            );
            let canvas = context.surface.canvas();
            svg.container_size(&size);
            // canvas.scale((device.density, device.density));
            svg.render(canvas)
        }
        Err(e) => {
            log::debug!("svg read to string error: {}", e);
        }
    }

    /*
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
    } */
}
