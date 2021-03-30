use roxmltree::Node;

use crate::common::svg::attribute_names::{Attribute, NodeExt};
use crate::common::svg::elements::filter::filter_element::FilterElement;
use crate::common::svg::units::length::{Length, LengthUnit};
use crate::common::svg::units::Units;

#[derive(Clone, Debug)]
pub struct Filter {
    x: Length,
    y: Length,
    width: Length,
    height: Length,
    filter_units: Units,
    primitive_units: Units,
    filters: Vec<FilterElement>,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            x: Length::new(-10.0, LengthUnit::Percent),
            y: Length::new(-10.0, LengthUnit::Percent),
            width: Length::new(120.0, LengthUnit::Percent),
            height: Length::new(120.0, LengthUnit::Percent),
            filter_units: Units::ObjectBoundingBox,
            primitive_units: Units::UserSpaceOnUse,
            filters: vec![],
        }
    }
}

impl Filter {
    pub fn new(
        x: Length,
        y: Length,
        width: Length,
        height: Length,
        filter_units: Units,
        primitive_units: Units,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            filter_units,
            primitive_units,
            filters: vec![],
        }
    }

    pub fn x(&self) -> Length {
        self.x
    }

    pub fn y(&self) -> Length {
        self.y
    }

    pub fn width(&self) -> Length {
        self.width
    }
    pub fn height(&self) -> Length {
        self.height
    }

    pub fn filters(&self) -> &Vec<FilterElement> {
        &self.filters
    }

    pub fn filters_mut(&mut self) -> &mut Vec<FilterElement> {
        &mut self.filters
    }

    pub fn from_node(node: (Option<Node>, Option<Node>)) -> Self {
        if node.0.is_none() {
            return Self::default();
        }

        let mut x: Option<Length> = None;
        let mut y: Option<Length> = None;
        let mut width: Option<Length> = None;
        let mut height: Option<Length> = None;
        let mut filter_units: Option<Units> = None;
        let mut primitive_units: Option<Units> = None;

        let mut parse_values = |node: &Node| {
            x = node.get_length_opt(Attribute::X);
            y = node.get_length_opt(Attribute::Y);
            width = node.get_length_opt(Attribute::Width);
            height = node.get_length_opt(Attribute::Height);
            filter_units = Units::from_str(
                node.attribute(Attribute::FilterUnits.to_str())
                    .unwrap_or(""),
            );
            primitive_units = Units::from_str(
                node.attribute(Attribute::PrimitiveUnits.to_str())
                    .unwrap_or(""),
            );
        };

        match node {
            (Some(node), Some(real_node)) => {
                parse_values(&real_node);
                parse_values(&node);
            }
            (Some(node), None) => parse_values(&node),
            (_, _) => {}
        };

        Filter::new(
            x.unwrap_or(Length::new(-10.0, LengthUnit::Percent)),
            y.unwrap_or(Length::new(-10.0, LengthUnit::Percent)),
            width.unwrap_or(Length::new(120.0, LengthUnit::Percent)),
            height.unwrap_or(Length::new(120.0, LengthUnit::Percent)),
            filter_units.unwrap_or(Units::UserSpaceOnUse),
            primitive_units.unwrap_or(Units::ObjectBoundingBox),
        )
    }
}
