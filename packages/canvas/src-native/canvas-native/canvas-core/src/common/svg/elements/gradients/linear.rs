use roxmltree::Node;
use skia_safe::{Color, Matrix};

use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::elements::element_names::ElementName;
use crate::common::svg::elements::gradients::utils::handle_stop;
use crate::common::svg::elements::reference_element::ReferenceElement;
use crate::common::svg::enums::spread_method::SpreadMethod;
use crate::common::svg::units::length::{Length, LengthUnit};
use crate::common::svg::units::Units;

#[derive(Clone, Debug)]
pub struct LinearGradient {
    id: String,
    x1: Length,
    y1: Length,
    x2: Length,
    y2: Length,
    gradient_units: Units,
    gradient_transform: String,
    spread_method: SpreadMethod,
    stop_colors: Vec<(f32, Color)>,
}

impl Default for LinearGradient {
    fn default() -> Self {
        Self {
            id: "".to_owned(),
            x1: Length::zero(),
            y1: Length::zero(),
            x2: Length::new(100.0, LengthUnit::Percent),
            y2: Length::zero(),
            gradient_units: Units::ObjectBoundingBox,
            gradient_transform: String::new(),
            spread_method: SpreadMethod::Pad,
            stop_colors: vec![],
        }
    }
}

impl LinearGradient {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn x1(&self) -> Length {
        self.x1
    }

    pub fn y1(&self) -> Length {
        self.y1
    }

    pub fn x2(&self) -> Length {
        self.x2
    }

    pub fn y2(&self) -> Length {
        self.y2
    }

    pub fn gradient_units(&self) -> Units {
        self.gradient_units
    }

    pub fn gradient_transform(&self) -> &str {
        &self.gradient_transform
    }

    pub fn spread_method(&self) -> SpreadMethod {
        self.spread_method
    }

    pub fn stop_colors(&self) -> &Vec<(f32, Color)> {
        &self.stop_colors
    }
}

impl ReferenceElement for LinearGradient {
    fn element_type() -> ElementName {
        ElementName::LinearGradient
    }
    fn from_node(node: (Option<Node>, Option<Node>)) -> Option<Self> {
        if node.0.is_none() {
            return None;
        }

        let mut x1: Option<Length> = None;
        let mut y1: Option<Length> = None;
        let mut x2: Option<Length> = None;
        let mut y2: Option<Length> = None;
        let mut gradient_units: Option<Units> = None;
        let mut gradient_transform = String::new();
        let mut did_transform = false;
        let mut spread_method: Option<SpreadMethod> = None;
        let mut stop_colors: Vec<(f32, Color)> = Vec::new();
        let mut id = String::new();
        let mut handle_gradient = |node: &Node| {
            if x1.is_none() {
                x1 = node.get_length_opt(Attribute::X1);
            }
            if x2.is_none() {
                x2 = node.get_length_opt(Attribute::X2);
            }

            if y1.is_none() {
                y1 = node.get_length_opt(Attribute::Y1);
            }

            if y2.is_none() {
                y2 = node.get_length_opt(Attribute::Y2);
            }

            if gradient_units.is_none() {
                gradient_units = Units::from_str(
                    node.attribute(Attribute::GradientUnits.to_str())
                        .unwrap_or(""),
                );
            }
            if !did_transform {
                if let Some(transform) = node.attribute(Attribute::GradientTransform.to_str()) {
                    gradient_transform = transform.to_string();
                    did_transform = true;
                }
            }
            if spread_method.is_none() {
                spread_method = SpreadMethod::from_str(
                    node.attribute(Attribute::SpreadMethod.to_str())
                        .unwrap_or(""),
                );
            }
            let mut children = node.children();
            handle_stop(&mut children, &mut stop_colors)
        };

        match node {
            (Some(node), Some(ref_node)) => {
                id = node.get_id().unwrap_or_default().to_owned();
                handle_gradient(&node);
                handle_gradient(&ref_node);
            }
            (Some(node), None) => {
                id = node.get_id().unwrap_or_default().to_owned();
                handle_gradient(&node);
            }
            (_, _) => {}
        };

        Some(LinearGradient {
            id,
            x1: x1.unwrap_or(Length::zero()),
            y1: y1.unwrap_or(Length::zero()),
            x2: x2.unwrap_or(Length::new(100.0, LengthUnit::Percent)),
            y2: y2.unwrap_or(Length::zero()),
            gradient_units: gradient_units.unwrap_or(Units::ObjectBoundingBox),
            gradient_transform,
            spread_method: spread_method.unwrap_or(SpreadMethod::Pad),
            stop_colors,
        })
    }
}
