use std::collections::HashMap;
use std::f32::consts::PI;

use roxmltree::Node;
use skia_safe::Vector;

use crate::common::svg::attribute_names::Attribute;

pub type StyleMap = HashMap<Attribute, String>;

pub trait StyleMapUtils {
    fn update_with_style(&mut self, style: &StyleMap, overwrite: bool);
}

impl StyleMapUtils for StyleMap {
    fn update_with_style(&mut self, style: &StyleMap, overwrite: bool) {
        if overwrite {
            let style = style.clone();
            self.extend(style.into_iter());
        } else {
            for s in style.iter() {
                if !self.contains_key(s.0) {
                    self.insert(*s.0, s.1.to_string());
                }
            }
        }
    }
}

pub fn parse_transform(transform: &str, scale: f32) -> Option<skia_safe::Matrix> {
    if transform.is_empty() {
        return None;
    }
    let transform: Vec<_> = transform
        .split(")")
        .into_iter()
        .map(|v| {
            let v = v.trim().replace("\n", "");
            v
        })
        .filter(|v| !v.is_empty())
        .collect();

    if transform.is_empty() {
        return None;
    }

    let mut matrix = skia_safe::Matrix::default();
    for transform in transform.into_iter() {
        if transform.contains("rotate") {
            let value = transform.replace("rotate(", "").replace(" ", ",");
            let values: Vec<_> = value.split(",").filter(|v| !v.is_empty()).collect();
            let slice = values.as_slice();
            if slice.len() == 1 {
                if let Ok(value) = slice[0].parse::<f32>() {
                    matrix.pre_rotate(value, None);
                }
            } else if values.len() == 3 {
                if let Some(points) = Some((slice[0], slice[1], slice[2])) {
                    if let (Ok(value), Ok(x), Ok(y)) = (
                        points.0.parse::<f32>(),
                        points.1.parse::<f32>(),
                        points.2.parse::<f32>(),
                    ) {
                        matrix.pre_rotate(value, Vector::new(x, y));
                    }
                }
            }
        } else if transform.contains("translateX") {
            let value = transform.replace("translateX(", "");
            if value.contains("px") {
                if let Ok(x) = value.replace("px", "").parse::<f32>() {
                    matrix.set_translate_x(x);
                }
            } else {
                if let Ok(x) = value.parse::<f32>() {
                    matrix.set_translate_x(x);
                }
            }
        } else if transform.contains("translateY") {
            let value = transform.replace("translateY(", "");
            if value.contains("px") {
                if let Ok(y) = value.replace("px", "").parse::<f32>() {
                    matrix.set_translate_y(y);
                }
            } else {
                if let Ok(y) = value.parse::<f32>() {
                    matrix.set_translate_y(y);
                }
            }
        } else if transform.contains("translate") {
            let value = transform.replace("translate(", "");
            let values: Vec<_>;
            if value.contains(",") {
                values = value.split(",").filter(|v| !v.is_empty()).collect();
            } else {
                values = value.split(" ").filter(|v| !v.is_empty()).collect();
            }

            let slice = values.as_slice();
            if slice.len() == 1 {
                if let Ok(point) = slice[0].parse::<f32>() {
                    matrix.pre_translate((point, 0.0));
                }
            } else if slice.len() == 2 {
                if let Some(points) = Some((slice[0], slice[1])) {
                    match (points.0.parse::<f32>(), points.1.parse::<f32>()) {
                        (Ok(x), Ok(y)) => {
                            matrix.pre_translate((x, y));
                        }
                        (_, _) => {}
                    }
                }
            }
        } else if transform.contains("skewX") {
            let value = transform.replace("skewX(", "");
            if let Ok(x) = value.parse::<f32>() {
                let x = (PI * x / 180.0).tan();
                matrix.pre_skew((x, 0.0), None);
            }
        } else if transform.contains("skewY") {
            let value = transform.replace("skewY(", "");
            if let Ok(y) = value.parse::<f32>() {
                let y = (PI * matrix.skew_y() / 180.0).tan();
                matrix.pre_skew((0.0, y), None);
            }
        } else if transform.contains("scale") {
            let value = transform.replace("scale(", "");
            let values: Vec<_>;
            if value.contains(",") {
                values = value.split(",").collect();
            } else {
                values = value.split(" ").collect();
            }
            let slice = values.as_slice();
            if slice.len() == 1 {
                if let Ok(value) = slice[0].parse::<f32>() {
                    matrix.pre_scale((value, value), None);
                }
            } else if slice.len() == 2 {
                if let Some(points) = Some((slice[0], slice[1])) {
                    match (points.0.parse::<f32>(), points.1.parse::<f32>()) {
                        (Ok(x), Ok(y)) => {
                            matrix.pre_scale((x, y), None);
                        }
                        (_, _) => {}
                    }
                }
            }
        } else if transform.contains("matrix") {
            let value = transform.replace("matrix(", "");
            let values: Vec<_>;
            if value.contains(",") {
                values = value.split(",").filter(|v| !v.is_empty()).collect();
            } else {
                values = value.split(" ").filter(|v| !v.is_empty()).collect();
            }
            let mut has_error = false;
            let values: Vec<_> = values
                .into_iter()
                .map(|value| {
                    return if let Ok(value) = value.parse() {
                        value
                    } else {
                        has_error = true;
                        -1f32
                    };
                })
                .collect();

            if !has_error {
                let affine = [
                    values.as_slice()[0],
                    values.as_slice()[1],
                    values.as_slice()[2],
                    values.as_slice()[3],
                    values.as_slice()[4],
                    values.as_slice()[5],
                ];
                let transform = skia_safe::Matrix::from_affine(&affine);
                matrix.pre_concat(&transform);
            }
        }
    }

    Some(matrix)
}

fn parse_style_attribute(style: &str) -> Vec<Vec<&str>> {
    style
        .trim()
        .split(";")
        .into_iter()
        .map(|key_val| {
            let value: Vec<_> = key_val
                .split(":")
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .collect();
            value
        })
        .filter(|v| !v.is_empty())
        .collect()
}

pub fn style_from_style_attr(node: Node) -> StyleMap {
    style_from_str(node.attribute(Attribute::Style.to_str()).unwrap_or(""))
}

fn style_from_str(text: &str) -> StyleMap {
    let values = parse_style_attribute(text);
    let mut style = HashMap::new();
    for key_val in values.into_iter() {
        let key = key_val.first();
        let value = key_val.last();
        if let (Some(key), Some(val)) = (key, value) {
            let key = *key;
            let val = *val;
            if !key.is_empty() {
                match Attribute::from_str(key) {
                    None => {}
                    Some(attr) => match attr {
                        Attribute::Style => {}
                        _ => {
                            style.insert(attr, val.to_owned());
                        }
                    },
                }
            }
        }
    }
    style
}

pub(crate) fn points_from(node: Node) -> Vec<Vec<f32>> {
    points_from_str(node.attribute(Attribute::Points.to_str()).unwrap_or(""))
}

pub(crate) fn points_from_str(text: &str) -> Vec<Vec<f32>> {
    let points: Vec<&str> = text.split(" ").collect();
    let points: Vec<_> = points
        .into_iter()
        .map(|value| {
            let points: Vec<_> = value.split(",").collect();
            let points = points.into_iter().fold(Vec::new(), |mut acc, value| {
                if let Ok(value) = value.parse() {
                    acc.push(value);
                }
                acc
            });
            points
        })
        .filter(|value| value.len() == 2)
        .collect();
    points
}

pub fn style_from(node: Node) -> StyleMap {
    let mut styles = HashMap::new();
    for attribute in node.attributes().into_iter() {
        match Attribute::from_str(attribute.name()) {
            Some(attr) => match attr {
                Attribute::Points => {}
                Attribute::Style => {}
                _ => {
                    styles.insert(attr, attribute.value().to_string());
                }
            },
            _ => {}
        }
    }

    let style_attr_val = style_from_style_attr(node);
    styles.extend(style_attr_val.into_iter());

    // if let Some(style) = node.attribute(Attribute::Style.to_str()) {
    //     let val = style_from_str(style);
    //     styles.extend(val.into_iter());
    // }

    styles
}

pub trait Parser {
    fn style_from(node: Node) -> StyleMap {
        style_from(node)
    }
    fn parse_from(node: Node) -> Self;
}
