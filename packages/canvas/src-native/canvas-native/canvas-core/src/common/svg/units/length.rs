use std::str::FromStr;

use crate::common::context::Device;
use crate::common::svg::attribute_names::Attribute;
use crate::common::svg::elements::parser::StyleMap;
use crate::common::svg::error::{Error, Result};
use crate::common::svg::units::stream::Stream;
use crate::common::svg::units::Units;
use crate::common::svg::view_box::ViewBox;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LengthUnit {
    None,
    Px,
    Pt,
    Pc,
    Em,
    Ex,
    In,
    Cm,
    Mm,
    Percent,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Length {
    value: f32,
    unit: LengthUnit,
}

impl Length {
    pub fn value(&self) -> f32 {
        self.value
    }
    pub fn unit(&self) -> LengthUnit {
        self.unit
    }
    pub fn new(value: f32, unit: LengthUnit) -> Self {
        Self { value, unit }
    }

    pub fn new_number(value: f32) -> Self {
        Self {
            value,
            unit: LengthUnit::None,
        }
    }

    pub fn zero() -> Self {
        Self {
            value: 0.0,
            unit: LengthUnit::None,
        }
    }

    pub fn length_from_style(style: &StyleMap, attr: Attribute, def: Length) -> Length {
        match style.get(&attr) {
            None => def,
            Some(val) => Length::from_str(val).unwrap_or(def),
        }
    }
}

impl Default for Length {
    fn default() -> Self {
        Length::zero()
    }
}

impl FromStr for Length {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut stream = Stream::from(s);
        let val = stream.parse_length()?;
        if !stream.at_end() {
            return Err(Error::UnexpectedData);
        }
        let val = Length::new(val.value, val.unit);
        // if val.unit == LengthUnit::None && val.value > 0.0 && val.value <= 1.0 {
        //     val.unit = LengthUnit::Percent;
        //     val.value = val.value * 100.0;
        // }
        Ok(val)
    }
}

pub fn convert_length(
    length: Length,
    attr: Attribute,
    unit: Units,
    device: Device,
    view_box: &ViewBox,
) -> f32 {
    let value = length.value;
    let dpi = device.ppi;
    match length.unit {
        LengthUnit::None | LengthUnit::Px => value,
        LengthUnit::Pt => value * dpi / 72.0,
        LengthUnit::Pc => value * dpi / 6.0,
        LengthUnit::Em => value * 10.0, //(need to get parent font size)
        LengthUnit::Ex => value * 10.0 / 2.0, //(need to get parent font size)
        LengthUnit::In => value * dpi,
        LengthUnit::Cm => value * dpi / 2.54,
        LengthUnit::Mm => value * dpi / 25.4,
        LengthUnit::Percent => {
            if unit == Units::ObjectBoundingBox {
                length.value / 100.0
            } else {
                match attr {
                    Attribute::X
                    | Attribute::Cx
                    | Attribute::X1
                    | Attribute::X2
                    | Attribute::Dx
                    | Attribute::Width => convert_percent(length, view_box.width()),
                    Attribute::Y
                    | Attribute::Cy
                    | Attribute::Y1
                    | Attribute::Y2
                    | Attribute::Dy
                    | Attribute::Height => convert_percent(length, view_box.height()),
                    _ => {
                        let vb_len = (view_box.width() * view_box.width()
                            + view_box.height() * view_box.height())
                            .sqrt()
                            / 2.0_f32.sqrt();

                        convert_percent(length, vb_len)
                    }
                }
            }
        }
    }
}

pub fn convert_percent(length: Length, dem: f32) -> f32 {
    length.value / 100.0 * dem
}
