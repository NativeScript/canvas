use std::str::FromStr;

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
pub struct RadialGradient {
    id: String,
    cx: Length,
    cy: Length,
    r: Length,
    fr: Length,
    fx: Length,
    fy: Length,
    gradient_units: Units,
    gradient_transform: String,
    spread_method: SpreadMethod,
    stop_colors: Vec<(f32, Color)>,
}

impl Default for RadialGradient {
    fn default() -> Self {
        let stop_colors: Vec<(f32, Color)> = vec![];
        Self {
            id: "".to_string(),
            cx: Length::new(50.0, LengthUnit::Percent),
            cy: Length::new(50.0, LengthUnit::Percent),
            r: Length::new(50.0, LengthUnit::Percent),
            fx: Length::new(50.0, LengthUnit::Percent),
            fy: Length::new(50.0, LengthUnit::Percent),
            fr: Length::zero(),
            spread_method: SpreadMethod::Pad,
            gradient_units: Units::ObjectBoundingBox,
            gradient_transform: String::new(),
            stop_colors,
        }
    }
}

impl RadialGradient {
    pub fn cx(&self) -> Length {
        self.cx
    }

    pub fn cy(&self) -> Length {
        self.cy
    }

    pub fn r(&self) -> Length {
        self.r
    }

    pub fn fx(&self) -> Length {
        self.fx
    }

    pub fn fy(&self) -> Length {
        self.fy
    }

    pub fn fr(&self) -> Length {
        self.fr
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

impl ReferenceElement for RadialGradient {
    fn element_type() -> ElementName {
        ElementName::RadialGradient
    }

    fn from_node(node: (Option<Node>, Option<Node>)) -> Option<Self> {
        if node.0.is_none() {
            return None;
        }
        let mut cx: Option<Length> = None;
        let mut cy: Option<Length> = None;
        let mut r: Option<Length> = None;
        let mut fx: Option<Length> = None;
        let mut fy: Option<Length> = None;
        let mut fr: Option<Length> = None;
        let mut gradient_units: Option<Units> = None;
        let mut gradient_transform = String::new();
        let mut did_transform = false;
        let mut spread_method: Option<SpreadMethod> = None;
        let mut stop_colors: Vec<(f32, Color)> = Vec::new();
        let mut id = String::new();
        let mut handle_gradient = |node: &Node| {
            if cx.is_none() {
                cx = node.get_length_opt(Attribute::Cx);
            }
            if cy.is_none() {
                cy = node.get_length_opt(Attribute::Cy);
            }
            if r.is_none() {
                r = node.get_length_opt(Attribute::R);
            }
            if fx.is_none() {
                fx = node.get_length_opt(Attribute::Fx);
            }
            if fy.is_none() {
                fy = node.get_length_opt(Attribute::Fy);
            }
            if fr.is_none() {
                fr = node.get_length_opt(Attribute::Fr);
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
        }

        Some(RadialGradient {
            id,
            cx: cx.unwrap_or(Length::new(50.0, LengthUnit::Percent)),
            cy: cy.unwrap_or(Length::new(50.0, LengthUnit::Percent)),
            r: r.unwrap_or(Length::new(50.0, LengthUnit::Percent)),
            fr: fr.unwrap_or(Length::zero()),
            fx: fx.unwrap_or(Length::new(50.0, LengthUnit::Percent)),
            fy: fy.unwrap_or(Length::new(50.0, LengthUnit::Percent)),
            gradient_units: gradient_units.unwrap_or(Units::ObjectBoundingBox),
            gradient_transform,
            spread_method: spread_method.unwrap_or(SpreadMethod::Pad),
            stop_colors,
        })
    }
}
