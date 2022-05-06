use std::io::{Read, Seek, SeekFrom};

use skia_safe::svg::SvgDom;

use crate::context::Context;

pub fn draw_svg_from_path(context: &mut Context, path: &str) {
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
                            let _device = context.device;
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

pub fn draw_svg(context: &mut Context, svg: &str) {
    match skia_safe::svg::SvgDom::from_bytes(svg.as_bytes()) {
        Ok(mut svg) => {
            let _device = context.device;
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
}
