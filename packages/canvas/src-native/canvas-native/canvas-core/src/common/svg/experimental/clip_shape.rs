use roxmltree::Node;

use crate::common::svg::{
    CIRCLE_NS, CX_ATTR, CY_ATTR, D_ATTR, ELLIPSE_NS, FONT_FAMILY_ATTR, FONT_SIZE_ATTR,
    FONT_STRETCH_ATTR, FONT_STYLE_ATTR, FONT_VARIANT_ATTR, FONT_WEIGHT_ATTR, HEIGHT_ATTR,
    LINE_NS, parse_style_attribute, PATH_NS, POINTS_ATTR, POLYGON_NS, POLYLINE_NS, R_ATTR, RECT_NS, RX_ATTR,
    RY_ATTR, STYLE_ATTR, TEXT_NS, WIDTH_ATTR, X1_ATTR, X2_ATTR, X_ATTR, Y1_ATTR, Y2_ATTR, Y_ATTR,
};
use crate::common::svg::length::Units;

use super::{attribute_names::NodeExt, length::Length};

#[derive(Clone, Debug, PartialEq)]
pub struct Clip<'a> {
    units: Units,
    clips: Vec<ClipShape<'a>>,
}

impl<'a> Clip<'a> {
    pub fn units(&self) -> Units {
        self.units
    }

    pub fn set_units(&mut self, units: Units) {
        self.units = units;
    }

    pub fn clips(&self) -> &[ClipShape] {
        self.clips.as_slice()
    }

    pub fn set_clips<'b>(&mut self, clips: &[ClipShape<'a>]) {
        self.clips.as_mut_slice().copy_from_slice(clips)
    }

    pub fn new(units: Units, clips: &'a mut [ClipShape<'a>]) -> Self {
        Self {
            units,
            clips: clips.to_vec(),
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.clips.len() == 1
            && match self.clips.get(0) {
            None => true,
            Some(val) => *val == ClipShape::None,
        };
    }
}

impl<'a> Default for Clip<'a> {
    fn default() -> Self {
        Self {
            units: Units::ObjectBoundingBox,
            clips: vec![ClipShape::None],
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ClipShape<'a> {
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
        points: &'a str,
    },
    Polyline {
        points: &'a str,
    },
    Rect {
        x: Length,
        y: Length,
        width: Length,
        height: Length,
    },
    Path {
        d: &'a str,
    },
    Text {
        x: Length,
        y: Length,
        text: &'a str,
        font_family: &'a str,
        font_size: &'a str,
        font_stretch: &'a str,
        font_style: &'a str,
        font_variant: &'a str,
        font_weight: &'a str,
    },
}

impl<'a, 'node: 'a> From<&Node<'node, 'node>> for ClipShape<'a> {
    fn from(val: &Node<'node, 'node>) -> Self {
        if val.is_element() {
            return match val.tag_name().name() {
                CIRCLE_NS => Self::Circle {
                    cx: val.get_length(super::attribute_names::Attribute::Cx, Length::zero()),
                    cy: val.get_length(super::attribute_names::Attribute::Cy, Length::zero()),
                    r: val.get_length(super::attribute_names::Attribute::R, Length::zero()),
                },
                ELLIPSE_NS => Self::Ellipse {
                    cx: val.get_length(super::attribute_names::Attribute::Cx, Length::zero()),
                    cy: val.get_length(super::attribute_names::Attribute::Cy, Length::zero()),
                    rx: val.get_length(super::attribute_names::Attribute::Rx, Length::zero()),
                    ry: val.get_length(super::attribute_names::Attribute::Ry, Length::zero()),
                },
                LINE_NS => Self::Line {
                    x1: val.get_length(super::attribute_names::Attribute::X1, Length::zero()),
                    y1: val.get_length(super::attribute_names::Attribute::Y1, Length::zero()),
                    x2: val.get_length(super::attribute_names::Attribute::X2, Length::zero()),
                    y2: val.get_length(super::attribute_names::Attribute::Y2, Length::zero()),
                },
                POLYGON_NS => Self::Polygon {
                    points: val.attribute(POINTS_ATTR).unwrap_or(""),
                },
                POLYLINE_NS => Self::Polyline {
                    points: val.attribute(POINTS_ATTR).unwrap_or(""),
                },
                RECT_NS => Self::Rect {
                    x: val.get_length(super::attribute_names::Attribute::X, Length::zero()),
                    y: val.get_length(super::attribute_names::Attribute::Y, Length::zero()),
                    width: val.get_length(super::attribute_names::Attribute::Width, Length::zero()),
                    height: val.get_length(super::attribute_names::Attribute::Height, Length::zero()),
                },
                PATH_NS => Self::Path {
                    d: val.attribute(D_ATTR).unwrap_or(""),
                },
                TEXT_NS => {
                    let mut font_size = val.attribute(FONT_SIZE_ATTR).unwrap_or("10px");
                    let mut font_style = val.attribute(FONT_STYLE_ATTR).unwrap_or("normal");
                    let mut font_variant = val.attribute(FONT_VARIANT_ATTR).unwrap_or("normal");
                    let mut font_weight = val.attribute(FONT_WEIGHT_ATTR).unwrap_or("normal");
                    let mut font_family = val.attribute(FONT_FAMILY_ATTR).unwrap_or("sans-serif");
                    let mut font_stretch = val.attribute(FONT_STRETCH_ATTR).unwrap_or("normal");

                    if let Some(style) = parse_style_attribute(val.attribute(STYLE_ATTR)) {
                        for key_val in style.into_iter() {
                            let key = key_val.first();
                            let value = key_val.last();
                            if let (Some(key), Some(val)) = (key, value) {
                                let key = *key;
                                let val = *val;
                                if !key.is_empty() {
                                    match key {
                                        FONT_SIZE_ATTR => {
                                            font_size = val;
                                        }
                                        FONT_STYLE_ATTR => font_style = val,
                                        FONT_VARIANT_ATTR => font_variant = val,
                                        FONT_WEIGHT_ATTR => font_weight = val,
                                        FONT_FAMILY_ATTR => font_family = val,
                                        FONT_STRETCH_ATTR => font_stretch = val,
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    Self::Text {
                        x: val.get_length(super::attribute_names::Attribute::X, Length::zero()),
                        y: val.get_length(super::attribute_names::Attribute::Y, Length::zero()),
                        text: val.text().unwrap_or(""),
                        font_family,
                        font_size,
                        font_style,
                        font_variant,
                        font_weight,
                        font_stretch,
                    }
                }
                _ => Self::None,
            };
        }
        Self::None
    }
}
