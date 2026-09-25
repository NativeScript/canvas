//! Animatable attribute values: number lists and colours blend, anything else only steps.

use super::color::{self, Rgba};

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    /// One or more numbers sharing a unit, e.g. `10px`, `0 0 100 100`, `4,2`.
    Numbers { values: Vec<f64>, unit: String },
    Color(Rgba),
    /// Not interpolable (`d`, `none`, `url(#grad)`, `visible`); steps at the keyframe boundary.
    Discrete(String),
}

impl Value {
    pub fn parse(input: &str) -> Self {
        let trimmed = input.trim();

        if let Some(rgba) = color::parse(trimmed) {
            return Value::Color(rgba);
        }

        match parse_numbers(trimmed) {
            Some((values, unit)) if !values.is_empty() => Value::Numbers { values, unit },
            _ => Value::Discrete(trimmed.to_owned()),
        }
    }

    /// `self` at `t == 0`, `other` at `t == 1`. Mismatched shapes step at the halfway point.
    pub fn lerp(&self, other: &Value, t: f64) -> Value {
        match (self, other) {
            (
                Value::Numbers { values: a, unit },
                Value::Numbers {
                    values: b,
                    unit: other_unit,
                },
            ) if a.len() == b.len() => Value::Numbers {
                values: a.iter().zip(b).map(|(x, y)| x + (y - x) * t).collect(),
                // Prefer whichever end has a unit so `0` -> `100%` animates in percent.
                unit: if unit.is_empty() {
                    other_unit.clone()
                } else {
                    unit.clone()
                },
            },
            (Value::Color(a), Value::Color(b)) => Value::Color(a.lerp(*b, t)),
            _ => {
                if t < 0.5 {
                    self.clone()
                } else {
                    other.clone()
                }
            }
        }
    }

    /// For `additive="sum"` and `accumulate="sum"`. Only number lists add; otherwise `other` wins.
    pub fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Numbers { values: a, unit }, Value::Numbers { values: b, .. })
                if a.len() == b.len() =>
            {
                Value::Numbers {
                    values: a.iter().zip(b).map(|(x, y)| x + y).collect(),
                    unit: unit.clone(),
                }
            }
            _ => other.clone(),
        }
    }

    /// Multiplies every number by `n`, for the repeat count in `accumulate="sum"`.
    pub fn scale(&self, n: f64) -> Value {
        match self {
            Value::Numbers { values, unit } => Value::Numbers {
                values: values.iter().map(|v| v * n).collect(),
                unit: unit.clone(),
            },
            other => other.clone(),
        }
    }

    /// Back to something Skia's attribute parser accepts.
    pub fn to_attribute(&self) -> String {
        match self {
            Value::Numbers { values, unit } => {
                let numbers = values
                    .iter()
                    .map(|v| format_number(*v))
                    .collect::<Vec<_>>()
                    .join(" ");
                if values.len() == 1 {
                    format!("{numbers}{unit}")
                } else {
                    numbers
                }
            }
            Value::Color(rgba) => rgba.to_css(),
            Value::Discrete(text) => text.clone(),
        }
    }

    pub fn numbers(&self) -> Option<&[f64]> {
        match self {
            Value::Numbers { values, .. } => Some(values),
            _ => None,
        }
    }
}

/// Snaps near-integers so an exact endpoint prints `10`, not `10.0000000001`.
fn format_number(value: f64) -> String {
    if value.is_finite() && (value - value.round()).abs() < 1e-9 {
        return format!("{}", value.round() as i64);
    }
    let text = format!("{value:.6}");
    let trimmed = text.trim_end_matches('0').trim_end_matches('.');
    trimmed.to_owned()
}

/// Numbers plus their shared unit. Anything else returns `None`, so `url(#x)` and path data
/// stay `Discrete`.
fn parse_numbers(input: &str) -> Option<(Vec<f64>, String)> {
    let bytes = input.as_bytes();
    let mut values = Vec::new();
    let mut unit = String::new();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index].is_ascii_whitespace() || bytes[index] == b',' {
            index += 1;
            continue;
        }
        // Nothing may follow a unit.
        if !unit.is_empty() {
            return None;
        }

        let start = index;
        if matches!(bytes[index], b'+' | b'-') {
            index += 1;
        }
        let digits_start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        if index < bytes.len() && bytes[index] == b'.' {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
        }
        if index == digits_start {
            return None;
        }
        // Exponent only if digits follow: `10em` starts the same way.
        if index < bytes.len() && matches!(bytes[index], b'e' | b'E') {
            let mut probe = index + 1;
            if probe < bytes.len() && matches!(bytes[probe], b'+' | b'-') {
                probe += 1;
            }
            if probe < bytes.len() && bytes[probe].is_ascii_digit() {
                while probe < bytes.len() && bytes[probe].is_ascii_digit() {
                    probe += 1;
                }
                index = probe;
            }
        }

        values.push(input[start..index].parse::<f64>().ok()?);

        let unit_start = index;
        while index < bytes.len()
            && (bytes[index].is_ascii_alphabetic() || bytes[index] == b'%')
        {
            index += 1;
        }
        if index > unit_start {
            unit = input[unit_start..index].to_owned();
        }
    }

    Some((values, unit))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn numbers(input: &str) -> Vec<f64> {
        Value::parse(input).numbers().unwrap().to_vec()
    }

    #[test]
    fn splits_numbers_from_their_unit() {
        assert_eq!(numbers("10"), vec![10.0]);
        assert_eq!(numbers(" -2.5px "), vec![-2.5]);
        assert_eq!(numbers("0 0 100 100"), vec![0.0, 0.0, 100.0, 100.0]);
        assert_eq!(numbers("4,2"), vec![4.0, 2.0]);
        assert_eq!(Value::parse("50%").to_attribute(), "50%");
    }

    #[test]
    fn leaves_non_numeric_values_alone() {
        assert!(matches!(Value::parse("none"), Value::Discrete(_)));
        assert!(matches!(Value::parse("url(#grad)"), Value::Discrete(_)));
        assert!(matches!(Value::parse("M0 0 L10 10"), Value::Discrete(_)));
        assert!(matches!(Value::parse("visible"), Value::Discrete(_)));
    }

    #[test]
    fn interpolates_only_matching_shapes() {
        let a = Value::parse("0");
        let b = Value::parse("10");
        assert_eq!(a.lerp(&b, 0.25).to_attribute(), "2.5");

        let pair = Value::parse("0 0");
        assert_eq!(pair.lerp(&b, 0.4).to_attribute(), "0 0");
        assert_eq!(pair.lerp(&b, 0.6).to_attribute(), "10");
    }

    #[test]
    fn colors_interpolate_as_colors() {
        let red = Value::parse("red");
        let blue = Value::parse("#0000ff");
        assert_eq!(red.lerp(&blue, 0.5).to_attribute(), "rgb(128,0,128)");
    }

    #[test]
    fn exact_endpoints_do_not_pick_up_float_noise() {
        let a = Value::parse("0");
        let b = Value::parse("360");
        assert_eq!(a.lerp(&b, 1.0).to_attribute(), "360");
    }
}
