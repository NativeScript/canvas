use std::str::FromStr;

use roxmltree::Children;
use skia_safe::Color;

use crate::common::svg::attribute_names::Attribute;
use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::parser::{style_from_style_attr, StyleMap};
use crate::common::svg::units::length::{Length, LengthUnit};
use crate::common::utils::color::parse_color;

pub fn handle_stop(children: &mut Children, stop_colors: &mut Vec<(f32, Color)>) {
    let mut checked = false;
    for stop_node in children.into_iter() {
        if !checked {
            if !stop_colors.is_empty() {
                stop_colors.clear()
            }
            checked = true;
        }
        match ElementName::from_str(stop_node.tag_name().name()) {
            None => {}
            Some(name) => match name {
                ElementName::Stop => {
                    let mut stop_opacity = Length::from_str(
                        stop_node
                            .attribute(Attribute::StopOpacity.to_str())
                            .unwrap_or("1"),
                    )
                        .unwrap_or(Length::new(100.0, LengthUnit::Percent));
                    let mut stop_color = stop_node
                        .attribute(Attribute::StopColor.to_str())
                        .unwrap_or("black")
                        .to_owned();

                    let stop_offset = Length::from_str(
                        stop_node
                            .attribute(Attribute::Offset.to_str())
                            .unwrap_or("0"),
                    )
                        .unwrap_or(Length::new(0.0, LengthUnit::Percent));

                    let stop_offset = match stop_offset.unit() {
                        LengthUnit::Percent => stop_offset.value() / 100.0,
                        LengthUnit::None => {
                            if stop_offset.value() > 1.0 {
                                1.0
                            } else {
                                stop_offset.value()
                            }
                        }
                        _ => 0.0,
                    };

                    for style in style_from_style_attr(stop_node).into_iter() {
                        match style.0 {
                            Attribute::StopOpacity => {
                                stop_opacity = Length::from_str(style.1.as_str())
                                    .unwrap_or(Length::new(100.0, LengthUnit::Percent));
                            }
                            Attribute::StopColor => {
                                stop_color = style.1;
                            }
                            _ => {}
                        }
                    }

                    let stop_opacity = match stop_opacity.unit() {
                        LengthUnit::Percent => stop_opacity.value() / 100.0,
                        LengthUnit::None => {
                            if stop_opacity.value() > 1.0 {
                                1.0
                            } else {
                                stop_opacity.value()
                            }
                        }
                        _ => 1.0,
                    };

                    if let Some(color) = parse_color(&stop_color) {
                        stop_colors.push((
                            stop_offset,
                            Color::from_argb(
                                (color.a() as f32 * stop_opacity) as u8,
                                color.r(),
                                color.g(),
                                color.b(),
                            ),
                        ))
                    }
                }
                _ => {}
            },
        }
    }
}
