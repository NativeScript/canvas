//! `<animateMotion>`: position and tangent along the path come from Skia's `PathMeasure`.

use skia_safe::{Path, PathMeasure};

use super::value::Value;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rotate {
    Angle(f64),
    Auto,
    AutoReverse,
}

#[derive(Clone, Debug)]
pub struct Motion {
    path: Path,
    length: f32,
    rotate: Rotate,
}

impl Motion {
    /// `None` when Skia cannot parse `d` or the path has zero length.
    pub fn new(d: &str, rotate: Rotate) -> Option<Self> {
        let path = Path::from_svg(d)?;
        let length = PathMeasure::new(&path, false, None).length();
        if !(length > 0.0) {
            return None;
        }
        Some(Self {
            path,
            length,
            rotate,
        })
    }

    pub fn rotate(&self) -> Rotate {
        self.rotate
    }

    /// Complete `transform` functions, ready to concatenate.
    pub fn sample(&self, fraction: f64) -> Value {
        let distance = self.length * (fraction.clamp(0.0, 1.0) as f32);
        let Some((position, tangent)) =
            PathMeasure::new(&self.path, false, None).pos_tan(distance)
        else {
            return Value::Discrete(String::new());
        };

        let mut transform = format!("translate({},{})", trim(position.x), trim(position.y));
        let degrees = match self.rotate {
            Rotate::Angle(angle) => Some(angle),
            Rotate::Auto => Some(tangent_degrees(tangent.x, tangent.y)),
            Rotate::AutoReverse => Some(tangent_degrees(tangent.x, tangent.y) + 180.0),
        };
        if let Some(degrees) = degrees.filter(|d| d.abs() > 1e-6) {
            transform.push_str(&format!(" rotate({})", trim(degrees as f32)));
        }
        Value::Discrete(transform)
    }
}

fn tangent_degrees(x: f32, y: f32) -> f64 {
    (y as f64).atan2(x as f64).to_degrees()
}

fn trim(value: f32) -> String {
    let text = format!("{value:.4}");
    text.trim_end_matches('0').trim_end_matches('.').to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walks_a_straight_line_at_constant_speed() {
        let motion = Motion::new("M0 0 L100 0", Rotate::Angle(0.0)).unwrap();
        assert_eq!(motion.sample(0.0).to_attribute(), "translate(0,0)");
        assert_eq!(motion.sample(0.5).to_attribute(), "translate(50,0)");
        assert_eq!(motion.sample(1.0).to_attribute(), "translate(100,0)");
    }

    #[test]
    fn auto_rotation_follows_the_tangent() {
        // SVG is y-down, so straight down is +90 degrees.
        let motion = Motion::new("M0 0 L0 100", Rotate::Auto).unwrap();
        assert_eq!(motion.sample(0.5).to_attribute(), "translate(0,50) rotate(90)");
    }

    #[test]
    fn rejects_a_path_with_nothing_to_walk() {
        assert!(Motion::new("M0 0", Rotate::Auto).is_none());
        assert!(Motion::new("not a path", Rotate::Auto).is_none());
    }
}
