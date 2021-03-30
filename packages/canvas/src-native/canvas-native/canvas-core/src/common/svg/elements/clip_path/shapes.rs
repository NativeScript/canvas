use roxmltree::Node;

use crate::common::svg::attribute_names::Attribute;
use crate::common::svg::attribute_names::NodeExt;
use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::parser::Parser;
use crate::common::svg::units::length::Length;

#[derive(Clone, Debug, PartialEq)]
pub enum ClipShape {
    None,
    Circle {
        cx: Length,
        cy: Length,
        r: Length,
    },
    Ellipse {
        cx: Length,
        cy: Length,
        rx: Length,
        ry: Length,
    },
    Line {
        x1: Length,
        y1: Length,
        x2: Length,
        y2: Length,
    },
    Polygon {
        points: String,
    },
    Polyline {
        points: String,
    },
    Rect {
        x: Length,
        y: Length,
        width: Length,
        height: Length,
    },
    Path {
        d: String,
    },
    Text {
        x: Length,
        y: Length,
        dx: Length,
        dy: Length,
        text: String,
        font_family: String,
        font_size: String,
        font_stretch: String,
        font_style: String,
        font_variant: String,
        font_weight: String,
    },
}

impl Parser for ClipShape {
    fn parse_from(val: Node) -> Self {
        if val.is_element() {
            return match ElementName::from_str(val.tag_name().name()) {
                Some(ElementName::Circle) => Self::Circle {
                    cx: val.get_length(Attribute::Cx, Length::zero()),
                    cy: val.get_length(Attribute::Cy, Length::zero()),
                    r: val.get_length(Attribute::R, Length::zero()),
                },
                Some(ElementName::Ellipse) => Self::Ellipse {
                    cx: val.get_length(Attribute::Cx, Length::zero()),
                    cy: val.get_length(Attribute::Cy, Length::zero()),
                    rx: val.get_length(Attribute::Rx, Length::zero()),
                    ry: val.get_length(Attribute::Ry, Length::zero()),
                },
                Some(ElementName::Line) => Self::Line {
                    x1: val.get_length(Attribute::X1, Length::zero()),
                    y1: val.get_length(Attribute::Y1, Length::zero()),
                    x2: val.get_length(Attribute::X2, Length::zero()),
                    y2: val.get_length(Attribute::Y2, Length::zero()),
                },
                Some(ElementName::Polygon) => Self::Polygon {
                    points: val
                        .attribute(Attribute::Points.to_str())
                        .unwrap_or("")
                        .to_string(),
                },
                Some(ElementName::Polyline) => Self::Polyline {
                    points: val
                        .attribute(Attribute::Points.to_str())
                        .unwrap_or("")
                        .to_owned(),
                },
                Some(ElementName::Rect) => Self::Rect {
                    x: val.get_length(Attribute::X, Length::zero()),
                    y: val.get_length(Attribute::Y, Length::zero()),
                    width: val.get_length(Attribute::Width, Length::zero()),
                    height: val.get_length(Attribute::Height, Length::zero()),
                },
                Some(ElementName::Path) => Self::Path {
                    d: val
                        .attribute(Attribute::D.to_str())
                        .unwrap_or("")
                        .to_owned(),
                },
                Some(ElementName::Text) => {
                    let mut font_size = val
                        .attribute(Attribute::FontSize.to_str())
                        .unwrap_or("10px").to_owned();
                    let mut font_style = val
                        .attribute(Attribute::FontStyle.to_str())
                        .unwrap_or("normal").to_owned();
                    let mut font_variant = val
                        .attribute(Attribute::FontVariant.to_str())
                        .unwrap_or("normal").to_owned();
                    let mut font_weight = val
                        .attribute(Attribute::FontWeight.to_str())
                        .unwrap_or("normal").to_owned();
                    let mut font_family = val
                        .attribute(Attribute::FontFamily.to_str())
                        .unwrap_or("sans-serif").to_owned();
                    let mut font_stretch = val
                        .attribute(Attribute::FontStretch.to_str())
                        .unwrap_or("normal").to_owned();

                    for (key, val) in ClipShape::style_from(val).into_iter() {
                        match key {
                            Attribute::FontSize => {
                                font_size = val;
                            }
                            Attribute::FontStyle => font_style = val,
                            Attribute::FontVariant => font_variant = val,
                            Attribute::FontWeight => font_weight = val,
                            Attribute::FontFamily => font_family = val,
                            Attribute::FontStretch => font_stretch = val,
                            _ => {}
                        }
                    }
                    Self::Text {
                        x: val.get_length(Attribute::X, Length::zero()),
                        y: val.get_length(Attribute::Y, Length::zero()),
                        dx: val.get_length(Attribute::Dx, Length::zero()),
                        dy: val.get_length(Attribute::Dy, Length::zero()),
                        text: val.text().unwrap_or("").to_string(),
                        font_family: font_family.to_owned(),
                        font_size: font_size.to_owned(),
                        font_style: font_style.to_owned(),
                        font_variant: font_variant.to_owned(),
                        font_weight: font_weight.to_owned(),
                        font_stretch: font_stretch.to_owned(),
                    }
                }
                _ => Self::None,
            };
        }
        Self::None
    }
}
