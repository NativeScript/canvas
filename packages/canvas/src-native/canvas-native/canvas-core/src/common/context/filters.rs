use std::f32::consts::PI;

use skia_safe::{color_filters, image_filters, ImageFilter, Point, table_color_filter, TileMode};

use crate::common::context::Context;
use crate::common::utils::color::parse_color;
use crate::common::utils::dimensions::parse_size;

#[derive(Copy, Clone, Debug)]
enum FilterType<'a> {
    Blur(&'a str),
    Brightness(&'a str),
    Contrast(&'a str),
    Grayscale(&'a str),
    Invert(&'a str),
    Opacity(&'a str),
    Saturate(&'a str),
    Sepia(&'a str),
    HueRotate(&'a str),
    DropShadow(&'a str),
    None,
}

pub fn to_radians(degrees: f32) -> f32 {
    degrees / 180.0 * PI
}

impl Context {
    pub fn set_filter(&mut self, value: &str) {
        if value.eq("none") {
            return;
        }
        let mut filters: Vec<&str> = value.split(")").collect();
        let filters: Vec<FilterType> = filters
            .into_iter()
            .filter(|x| {
                let x = *x;
                !x.eq(";")
            })
            .map(|x| {
                if x.contains("blur") {
                    FilterType::Blur(x)
                } else if x.contains("brightness") {
                    FilterType::Brightness(x)
                } else if x.contains("contrast") {
                    FilterType::Contrast(x)
                } else if x.contains("grayscale") || x.contains("greyscale") {
                    FilterType::Grayscale(x)
                } else if x.contains("invert") {
                    FilterType::Invert(x)
                } else if x.contains("opacity") {
                    FilterType::Opacity(x)
                } else if x.contains("saturate") {
                    FilterType::Saturate(x)
                } else if x.contains("sepia") {
                    FilterType::Sepia(x)
                } else if x.contains("hue-rotate") {
                    FilterType::HueRotate(x)
                } else if x.contains("drop-shadow") {
                    FilterType::DropShadow(x)
                } else {
                    FilterType::None
                }
            })
            .collect();
        let filter = filters
            .iter()
            .fold(None, |chain, next_filter| match next_filter {
                FilterType::Blur(blur) => {
                    let value = blur.replace("blur(", "").replace(")", "");
                    return if value.contains("px") {
                        if let Ok(value) = value.replace("px", "").parse::<f32>() {
                            image_filters::blur((value, value), TileMode::Clamp, chain, None)
                        } else {
                            chain
                        }
                    } else {
                        chain
                    };
                }
                FilterType::Brightness(brightness) => {
                    let value = brightness.replace("brightness(", "").replace(")", "");
                    if value.contains("%") {
                        if let Ok(value) = value.replace("%", "").parse::<f32>() {
                            let amt = value.max(0.0);
                            let color_matrix = color_filters::matrix_row_major(&[
                                amt, 0.0, 0.0, 0.0, 0.0, 0.0, amt, 0.0, 0.0, 0.0, 0.0, 0.0, amt,
                                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                            ]);
                            image_filters::color_filter(color_matrix, chain, None)
                        } else {
                            chain
                        }
                    } else {
                        chain
                    }
                }
                FilterType::Contrast(contrast) => {
                    let value = contrast.replace("contrast(", "").replace(")", "");
                    if value.contains("%") {
                        if let Ok(value) = value.replace("%", "").parse::<f32>() {
                            let amt = value.max(0.0);
                            let mut ramp = [0u8; 256];
                            for (i, val) in ramp.iter_mut().take(256).enumerate() {
                                let orig = i as f32;
                                *val = (127.0 + amt * orig - (127.0 * amt)) as u8;
                            }
                            let table = Some(&ramp);
                            let color_table =
                                table_color_filter::from_argb(None, table, table, table);
                            image_filters::color_filter(color_table, chain, None)
                        } else {
                            chain
                        }
                    } else {
                        chain
                    }
                }
                FilterType::Grayscale(grayscale) => {
                    let value = grayscale.replace("grayscale(", "").replace(")", "");
                    if value.contains("%") {
                        if let Ok(value) = value.replace("%", "").parse::<f32>() {
                            let amt = 1.0 - value.max(0.0).min(1.0);
                            let color_matrix = color_filters::matrix_row_major(&[
                                (0.2126 + 0.7874 * amt),
                                (0.7152 - 0.7152 * amt),
                                (0.0722 - 0.0722 * amt),
                                0.0,
                                0.0,
                                (0.2126 - 0.2126 * amt),
                                (0.7152 + 0.2848 * amt),
                                (0.0722 - 0.0722 * amt),
                                0.0,
                                0.0,
                                (0.2126 - 0.2126 * amt),
                                (0.7152 - 0.7152 * amt),
                                (0.0722 + 0.9278 * amt),
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                1.0,
                                0.0,
                            ]);
                            image_filters::color_filter(color_matrix, chain, None)
                        } else {
                            chain
                        }
                    } else {
                        chain
                    }
                }
                FilterType::Invert(invert) => {
                    let value = invert.replace("invert(", "").replace(")", "");
                    if value.contains("%") {
                        if let Ok(value) = value.replace("%", "").parse::<f32>() {
                            let amt = value.max(0.0).min(1.0);
                            let mut ramp = [0u8; 256];
                            for (i, val) in ramp
                                .iter_mut()
                                .take(256)
                                .enumerate()
                                .map(|(i, v)| (i as f32, v))
                            {
                                let (orig, inv) = (i, 255.0 - i);
                                *val = (orig * (1.0 - amt) + inv * amt) as u8;
                            }
                            let table = Some(&ramp);
                            let color_table =
                                table_color_filter::from_argb(None, table, table, table);
                            image_filters::color_filter(color_table, chain, None)
                        } else {
                            chain
                        }
                    } else {
                        chain
                    }
                }
                FilterType::Opacity(opacity) => {
                    let value = opacity.replace("opacity(", "").replace(")", "");
                    if value.contains("%") {
                        if let Ok(value) = value.replace("%", "").parse::<f32>() {
                            let amt = value.max(0.0).min(1.0);
                            let color_matrix = color_filters::matrix_row_major(&[
                                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                                0.0, 0.0, 0.0, 0.0, 0.0, amt, 0.0,
                            ]);
                            image_filters::color_filter(color_matrix, chain, None)
                        } else {
                            chain
                        }
                    } else {
                        chain
                    }
                }
                FilterType::Saturate(saturate) => {
                    let value = saturate.replace("saturate(", "").replace(")", "");
                    if value.contains("%") {
                        if let Ok(value) = value.replace("%", "").parse::<f32>() {
                            let amt = value.max(0.0);
                            let color_matrix = color_filters::matrix_row_major(&[
                                (0.2126 + 0.7874 * amt),
                                (0.7152 - 0.7152 * amt),
                                (0.0722 - 0.0722 * amt),
                                0.0,
                                0.0,
                                (0.2126 - 0.2126 * amt),
                                (0.7152 + 0.2848 * amt),
                                (0.0722 - 0.0722 * amt),
                                0.0,
                                0.0,
                                (0.2126 - 0.2126 * amt),
                                (0.7152 - 0.7152 * amt),
                                (0.0722 + 0.9278 * amt),
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                1.0,
                                0.0,
                            ]);
                            image_filters::color_filter(color_matrix, chain, None)
                        } else {
                            chain
                        }
                    } else {
                        chain
                    }
                }
                FilterType::Sepia(sepia) => {
                    let value = sepia.replace("sepia(", "").replace(")", "");
                    if value.contains("%") {
                        if let Ok(value) = value.replace("%", "").parse::<f32>() {
                            let amt = 1.0 - value.max(0.0).min(1.0);
                            let color_matrix = color_filters::matrix_row_major(&[
                                (0.393 + 0.607 * amt),
                                (0.769 - 0.769 * amt),
                                (0.189 - 0.189 * amt),
                                0.0,
                                0.0,
                                (0.349 - 0.349 * amt),
                                (0.686 + 0.314 * amt),
                                (0.168 - 0.168 * amt),
                                0.0,
                                0.0,
                                (0.272 - 0.272 * amt),
                                (0.534 - 0.534 * amt),
                                (0.131 + 0.869 * amt),
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                1.0,
                                0.0,
                            ]);
                            image_filters::color_filter(color_matrix, chain, None)
                        } else {
                            chain
                        }
                    } else {
                        chain
                    }
                }
                FilterType::HueRotate(hue_rotate) => {
                    let value = hue_rotate.replace("hue-rotate(", "").replace(")", "");
                    if value.contains("deg") {
                        if let Ok(value) = value.replace("deg", "").parse::<f32>() {
                            let cos = to_radians(value).cos();
                            let sin = to_radians(value).sin();
                            let color_matrix = color_filters::matrix_row_major(&[
                                (0.213 + cos * 0.787 - sin * 0.213),
                                (0.715 - cos * 0.715 - sin * 0.715),
                                (0.072 - cos * 0.072 + sin * 0.928),
                                0.0,
                                0.0,
                                (0.213 - cos * 0.213 + sin * 0.143),
                                (0.715 + cos * 0.285 + sin * 0.140),
                                (0.072 - cos * 0.072 - sin * 0.283),
                                0.0,
                                0.0,
                                (0.213 - cos * 0.213 - sin * 0.787),
                                (0.715 - cos * 0.715 + sin * 0.715),
                                (0.072 + cos * 0.928 + sin * 0.072),
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                0.0,
                                1.0,
                                0.0,
                            ]);
                            image_filters::color_filter(color_matrix, chain, None)
                        } else {
                            chain
                        }
                    } else {
                        chain
                    }
                }
                FilterType::DropShadow(shadow) => {
                    let shadow = shadow
                        .replace("drop-shadow(", "")
                        .replace(")", "");
                    let value: Vec<&str> = shadow.split(" ")
                        .collect();
                    if value.len() < 3 {
                        return chain;
                    }
                    let mut offset = Point::new(0.0, 0.0);
                    let mut blur = 0f32;
                    let mut color = self.font_color;
                    for (i, value) in value.into_iter().enumerate() {
                        match i {
                            0 => {
                                offset.x = parse_size(value, self.device);
                            }
                            1 => {
                                offset.y = parse_size(value, self.device);
                            }
                            2 => {
                                blur = parse_size(value, self.device);
                            }
                            3 => {
                                if let Some(parsed_color) = parse_color(value) {
                                    color = parsed_color;
                                } else {
                                    blur = 0.0;
                                }
                            }
                            _ => {}
                        }
                    }
                    if blur > 0.0 {
                        let sigma = blur / 2.0;
                        return image_filters::drop_shadow(
                            offset,
                            (sigma, sigma),
                            color,
                            chain,
                            None,
                        );
                    }
                    chain
                }
                _ => chain,
            });

        self.state.filter = value.to_string();
        self.state.paint.fill_paint_mut().set_image_filter(filter.clone());
        self.state.paint.stroke_paint_mut().set_image_filter(filter.clone());
        self.state.paint.image_paint_mut().set_image_filter(filter);
    }

    pub fn get_filter(&self) -> &str {
        &self.state.filter
    }
}
