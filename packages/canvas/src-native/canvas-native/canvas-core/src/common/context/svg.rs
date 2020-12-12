#[repr(C)]
pub struct SVGCanvasNative {
    pub(crate) surface: Surface,
    pub(crate) context: Option<Context>,
}


pub(crate) fn draw_svg_image(svg_canvas_native_ptr: c_longlong, svg: *const c_char) -> c_longlong {
    if svg_canvas_native_ptr == 0 {
        return 0;
    }
    let mut svg_canvas_native: Box<SVGCanvasNative> =
        unsafe { Box::from_raw(svg_canvas_native_ptr as *mut _) };
    let svg_surface = &mut svg_canvas_native.surface;
    let canvas = svg_surface.canvas();
    let mut rect = Rect::new_empty();
    let mut svg_canvas = Canvas::new(rect.clone(), None);
    if !svg.is_null() {
        let svg_string = unsafe { CStr::from_ptr(svg as _) };
        let string = svg_string.to_str().unwrap_or("");
        if !string.is_empty() {
            let mut reader = Reader::from_str(string);
            let mut buf = Vec::new();
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => match e.name() {
                        b"svg" => {
                            let attributes = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                            for attribute in attributes.iter() {
                                let key = String::from_utf8_lossy(attribute.key).to_string();
                                let val = attribute.unescape_and_decode_value(&reader).unwrap();
                                match key.as_str() {
                                    "width" => {
                                        &rect.set_wh(val.parse::<f32>().unwrap(), rect.height());
                                    }
                                    "height" => {
                                        &rect.set_wh(rect.width(), val.parse::<f32>().unwrap());
                                    }
                                    _ => {}
                                }
                            }
                            svg_canvas = Canvas::new(rect.clone(), None);
                        }
                        b"circle" => {
                            let attributes = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                            let mut path = Path::new();
                            let mut fill_paint = Paint::default();
                            fill_paint.set_anti_alias(true);
                            fill_paint.set_style(Style::Fill);
                            let mut stroke_paint = Paint::default();
                            stroke_paint.set_anti_alias(true);
                            stroke_paint.set_style(Style::Stroke);
                            let mut point = Point::new(0.0, 0.0);
                            let mut radius = 0f32;
                            for attribute in attributes.iter() {
                                let key = String::from_utf8_lossy(attribute.key).to_string();
                                let val = attribute.unescape_and_decode_value(&reader).unwrap();
                                match key.as_str() {
                                    "cx" => {
                                        point.x = val.parse::<f32>().unwrap();
                                    }
                                    "cy" => {
                                        point.y = val.parse::<f32>().unwrap();
                                    }
                                    "r" => {
                                        radius = val.parse::<f32>().unwrap();
                                    }
                                    "stroke" => {
                                        &stroke_paint
                                            .set_color(ColorParser::from_str(val.as_str()));
                                    }
                                    "stroke-width" => {
                                        &stroke_paint.set_stroke_width(val.parse::<f32>().unwrap());
                                    }
                                    "fill" => {
                                        &fill_paint.set_color(ColorParser::from_str(val.as_str()));
                                    }
                                    _ => {}
                                }
                            }
                            path.add_circle(point, radius, None);
                            &svg_canvas.draw_path(&path, &fill_paint);
                            &svg_canvas.draw_path(&path, &stroke_paint);
                        }
                        b"text" => {}
                        _ => {}
                    },
                    Ok(Event::Empty(ref e)) => match e.name() {
                        b"circle" => {
                            let attributes = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                            let mut path = Path::new();
                            let mut fill_paint = Paint::default();
                            fill_paint.set_anti_alias(true);
                            fill_paint.set_anti_alias(true);
                            fill_paint.set_style(Style::Fill);
                            let mut stroke_paint = Paint::default();
                            stroke_paint.set_anti_alias(true);
                            stroke_paint.set_style(Style::Stroke);
                            let mut point = Point::new(0.0, 0.0);
                            let mut radius = 0f32;
                            for attribute in attributes.iter() {
                                let key = String::from_utf8_lossy(attribute.key).to_string();
                                let val = attribute.unescape_and_decode_value(&reader).unwrap();
                                match key.as_str() {
                                    "cx" => {
                                        point.x = val.parse::<f32>().unwrap();
                                    }
                                    "cy" => {
                                        point.y = val.parse::<f32>().unwrap();
                                    }
                                    "r" => {
                                        radius = val.parse::<f32>().unwrap();
                                    }
                                    "stroke" => {
                                        &stroke_paint
                                            .set_color(ColorParser::from_str(val.as_str()));
                                    }
                                    "stroke-width" => {
                                        &stroke_paint.set_stroke_width(val.parse::<f32>().unwrap());
                                    }
                                    "fill" => {
                                        &fill_paint.set_color(ColorParser::from_str(val.as_str()));
                                    }
                                    _ => {}
                                }
                            }
                            path.add_circle(point, radius, None);
                            &canvas.draw_path(&path, &fill_paint);
                            &canvas.draw_path(&path, &stroke_paint);
                        }
                        b"rect" => {
                            let attributes = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                            let mut path = Path::new();
                            let mut fill_paint = Paint::default();
                            fill_paint.set_anti_alias(true);
                            fill_paint.set_style(Style::Fill);
                            let mut stroke_paint = Paint::default();
                            stroke_paint.set_anti_alias(true);
                            stroke_paint.set_style(Style::Stroke);
                            let mut rect = Rect::new_empty();
                            for attribute in attributes.iter() {
                                let key = String::from_utf8_lossy(attribute.key).to_string();
                                let val = attribute.unescape_and_decode_value(&reader).unwrap();
                                match key.as_str() {
                                    "width" => {
                                        rect.right = val.parse::<f32>().unwrap();
                                    }
                                    "height" => {
                                        rect.bottom = val.parse::<f32>().unwrap();
                                    }
                                    "style" => {
                                        let mut styles = StyleParser::from_str(val.as_ref());
                                        for style in styles.iter() {
                                            match style.0 {
                                                "width" => {
                                                    rect.right = style.1.parse::<f32>().unwrap();
                                                }
                                                "height" => {
                                                    rect.bottom = style.1.parse::<f32>().unwrap();
                                                }
                                                "stroke" => {
                                                    &stroke_paint
                                                        .set_color(ColorParser::from_str(style.1));
                                                }
                                                "stroke-width" => {
                                                    &stroke_paint.set_stroke_width(
                                                        style.1.parse::<f32>().unwrap(),
                                                    );
                                                }
                                                "fill" => {
                                                    &fill_paint
                                                        .set_color(ColorParser::from_str(style.1));
                                                }
                                                "stroke-opacity" => {
                                                    &stroke_paint.set_alpha(
                                                        (style.1.parse::<f32>().unwrap_or(1.0)
                                                            * 255.0)
                                                            as u8,
                                                    );
                                                }
                                                "fill-opacity" => {
                                                    &fill_paint.set_alpha(
                                                        (style.1.parse::<f32>().unwrap_or(1.0)
                                                            * 255.0)
                                                            as u8,
                                                    );
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    "stroke" => {
                                        &stroke_paint
                                            .set_color(ColorParser::from_str(val.as_str()));
                                    }
                                    "stroke-width" => {
                                        &stroke_paint.set_stroke_width(val.parse::<f32>().unwrap());
                                    }
                                    "fill" => {
                                        &fill_paint.set_color(ColorParser::from_str(val.as_str()));
                                    }
                                    "stroke-opacity" => {
                                        &stroke_paint.set_alpha(val.parse::<u8>().unwrap_or(255));
                                    }
                                    "fill-opacity" => {
                                        &fill_paint.set_alpha(val.parse::<u8>().unwrap_or(255));
                                    }
                                    _ => {}
                                }
                            }
                            path.add_rect(rect, None);
                            &canvas.draw_path(&path, &fill_paint);
                            &canvas.draw_path(&path, &stroke_paint);
                        }
                        b"path" => {
                            let attributes = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                            let mut path = Path::new();
                            let mut fill_paint = Paint::default();
                            fill_paint.set_anti_alias(true);
                            fill_paint.set_style(Style::Fill);
                            let mut stroke_paint = Paint::default();
                            stroke_paint.set_anti_alias(true);
                            stroke_paint.set_style(Style::Stroke);
                            let mut fill = false;
                            let mut stroke = false;
                            for attribute in attributes.iter() {
                                let key = String::from_utf8_lossy(attribute.key).to_string();
                                let val = attribute.unescape_and_decode_value(&reader).unwrap();
                                match key.as_str() {
                                    "d" => path = from_svg(val.as_str()).unwrap_or(Path::new()),
                                    "stroke" => {
                                        let value = val.as_str();
                                        stroke = !value.eq("none");
                                        &stroke_paint.set_color(ColorParser::from_str(value));
                                    }
                                    "stroke-width" => {
                                        &stroke_paint.set_stroke_width(val.parse::<f32>().unwrap());
                                    }
                                    "fill" => {
                                        let value = val.as_str();
                                        fill = !value.eq("none");
                                        &fill_paint.set_color(ColorParser::from_str(value));
                                    }
                                    _ => {}
                                }
                            }
                            if fill {
                                &canvas.draw_path(&path, &fill_paint);
                            }

                            if stroke {
                                &canvas.draw_path(&path, &stroke_paint);
                            }
                        }
                        _ => {}
                    },
                    Ok(Event::Text(e)) => {
                        /* let font_native_type_face = Typeface::from_name("sans-serif", font_native_style).unwrap_or(Typeface::default());
                        let font = Font::from_typeface(&font_native_type_face, 10.0);
                        let blob = TextBlob::from_str(e.unescape_and_decode(&reader).unwrap(), &font);
                        let mut paint = Paint::default();
                        let mut point = Point::new(0.0, 0.0);
                        &canvas.draw_text_blob(&blob.unwrap(), &point, &paint);*/
                    }
                    Ok(Event::End(ref e)) => {}
                    Ok(Event::Eof) => break,
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    _ => (), //
                }
                buf.clear();
            }
        }

        canvas.flush();
    }
    Box::into_raw(svg_canvas_native) as *mut _ as i64
}

pub(crate) struct StyleParser {}

impl StyleParser {
    pub fn from_str(style: &str) -> Vec<(&str, &str)> {
        let mut values: Vec<(_, _)> = Vec::new();
        let mut styles: Vec<&str> = style.split(";").collect();
        for style in styles.iter() {
            let value = *style;
            let key_value: Vec<_> = value.split(":").collect();
            let default = "";
            let k = key_value.get(0).unwrap_or(&default).to_owned();
            let v = key_value.get(1).unwrap_or(&default).to_owned();
            values.push((k, v));
        }
        values
    }
}

pub(crate) struct ColorParser {}

impl ColorParser {
    pub fn is_color(value: &str) -> bool {
        value.starts_with("#") || value.starts_with("rgb") || value.starts_with("hsl")
    }
    pub fn from_str(color: &str) -> Color {
        let mut value = color.to_lowercase();
        if value.starts_with("#") {
            Color::BLACK
        } else if value.starts_with("rgb") {
            value = value.replace("rgba(", "");
            value = value.replace("rgb(", "");
            value = value.replace(")", "");
            let mut rgb_rgba: Vec<_> = value.split(",").collect();
            let default = "255";
            let mut r = rgb_rgba.get(0).unwrap_or(&default).parse().unwrap_or(255);
            let mut g = rgb_rgba.get(1).unwrap_or(&default).parse().unwrap_or(255);
            let mut b = rgb_rgba.get(2).unwrap_or(&default).parse().unwrap_or(255);
            let mut a = rgb_rgba.get(3).unwrap_or(&default).parse().unwrap_or(255);

            Color::from_argb(a, r, g, b)
        } else if value.starts_with("hsl") {
            Color::BLACK
        } else {
            match value.as_str() {
                "red" => Color::RED,
                "blue" => Color::BLUE,
                "green" => Color::GREEN,
                "pink" => Color::from_rgb(255, 192, 203),
                _ => Color::BLACK,
            }
        }
    }
}