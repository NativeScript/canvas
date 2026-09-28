use super::value::Value;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CalcMode {
    Discrete,
    Linear,
    Paced,
    Spline,
}

pub struct Sample {
    /// 0..=1 through the current repetition.
    pub fraction: f64,
    /// Whole-number repetition index.
    pub iteration: f64,
}

#[derive(Clone, Debug)]
pub struct Timing {
    /// Seconds from the document's start.
    pub begin: f64,
    /// Seconds per repetition; infinite when the source gave no `dur`.
    pub dur: f64,
    pub repeat: f64,
    pub freeze: bool,
    pub additive: bool,
    pub accumulate: bool,
    pub calc_mode: CalcMode,
    pub key_times: Option<Vec<f64>>,
    pub key_splines: Option<Vec<[f64; 4]>>,
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            begin: 0.0,
            dur: f64::INFINITY,
            repeat: 1.0,
            freeze: false,
            additive: false,
            accumulate: false,
            calc_mode: CalcMode::Linear,
            key_times: None,
            key_splines: None,
        }
    }
}

impl Timing {
    fn active_duration(&self) -> f64 {
        if !self.dur.is_finite() || !self.repeat.is_finite() {
            return f64::INFINITY;
        }
        self.dur * self.repeat
    }

    /// `None` before it begins or after it ends without `fill="freeze"`.
    pub fn sample(&self, time: f64) -> Option<Sample> {
        let local = time - self.begin;
        if local < 0.0 {
            return None;
        }

        // No `dur` holds the final keyframe forever, which is `<set>`.
        if !self.dur.is_finite() {
            return Some(Sample {
                fraction: 1.0,
                iteration: 0.0,
            });
        }

        let active = self.active_duration();
        if local >= active {
            if !self.freeze {
                return None;
            }
            return Some(Sample {
                fraction: 1.0,
                iteration: (self.repeat - 1.0).max(0.0).floor(),
            });
        }

        let progress = local / self.dur;
        let iteration = progress.floor();
        Some(Sample {
            fraction: progress - iteration,
            iteration,
        })
    }

    pub fn end(&self) -> Option<f64> {
        let active = self.active_duration();
        active.is_finite().then(|| self.begin + active)
    }

    /// Applies `keySplines` to a bare fraction, for `<animateMotion>` whose keyframes are a path.
    pub fn ease(&self, fraction: f64) -> f64 {
        if self.calc_mode != CalcMode::Spline {
            return fraction;
        }
        self.key_splines
            .as_ref()
            .and_then(|splines| splines.first())
            .map_or(fraction, |spline| solve_bezier(*spline, fraction))
    }

    /// A frozen animation past its end is constant, so it is not running.
    pub fn is_running(&self, time: f64) -> bool {
        let active = self.active_duration();
        if !active.is_finite() {
            return true;
        }
        time < self.begin + active
    }

    pub fn interpolate(&self, frames: &[Value], fraction: f64) -> Value {
        match frames.len() {
            0 => Value::Discrete(String::new()),
            1 => frames[0].clone(),
            _ => {
                let times = self.resolved_key_times(frames);
                let fraction = fraction.clamp(0.0, 1.0);

                let mut index = 0;
                while index + 2 < times.len() && fraction >= times[index + 1] {
                    index += 1;
                }

                if self.calc_mode == CalcMode::Discrete {
                    // Without keyTimes, n values are n equal steps (not n-1 segments).
                    let step = ((fraction * frames.len() as f64).floor() as usize)
                        .min(frames.len() - 1);
                    let step = if self.key_times.is_some() { index } else { step };
                    return frames[step].clone();
                }

                let span = times[index + 1] - times[index];
                let local = if span > 0.0 {
                    (fraction - times[index]) / span
                } else {
                    0.0
                };
                let eased = match self.calc_mode {
                    CalcMode::Spline => self
                        .key_splines
                        .as_ref()
                        .and_then(|splines| splines.get(index))
                        .map_or(local, |spline| solve_bezier(*spline, local)),
                    _ => local,
                };
                frames[index].lerp(&frames[index + 1], eased)
            }
        }
    }

    fn resolved_key_times(&self, frames: &[Value]) -> Vec<f64> {
        if let Some(times) = &self.key_times {
            if times.len() == frames.len() {
                return times.clone();
            }
        }
        if self.calc_mode == CalcMode::Paced {
            if let Some(times) = paced_key_times(frames) {
                return times;
            }
        }
        even_key_times(frames.len())
    }
}

fn even_key_times(count: usize) -> Vec<f64> {
    if count < 2 {
        return vec![0.0];
    }
    let last = (count - 1) as f64;
    (0..count).map(|i| i as f64 / last).collect()
}

/// Key times proportional to distance between values. `None` (even spacing) for non-numeric values.
fn paced_key_times(frames: &[Value]) -> Option<Vec<f64>> {
    let mut distances = Vec::with_capacity(frames.len());
    distances.push(0.0);
    let mut total = 0.0;

    for pair in frames.windows(2) {
        let (a, b) = (pair[0].numbers()?, pair[1].numbers()?);
        if a.len() != b.len() {
            return None;
        }
        let distance = a
            .iter()
            .zip(b)
            .map(|(x, y)| (y - x).powi(2))
            .sum::<f64>()
            .sqrt();
        total += distance;
        distances.push(total);
    }

    if total <= 0.0 {
        return None;
    }
    Some(distances.into_iter().map(|d| d / total).collect())
}

/// y of the cubic bezier (0,0),(x1,y1),(x2,y2),(1,1) at `x`. Newton, then bisection if too flat.
fn solve_bezier([x1, y1, x2, y2]: [f64; 4], x: f64) -> f64 {
    const EPSILON: f64 = 1e-6;

    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }

    let curve = |a: f64, b: f64, t: f64| {
        let inv = 1.0 - t;
        3.0 * inv * inv * t * a + 3.0 * inv * t * t * b + t * t * t
    };
    let slope = |a: f64, b: f64, t: f64| {
        let inv = 1.0 - t;
        3.0 * inv * inv * a + 6.0 * inv * t * (b - a) + 3.0 * t * t * (1.0 - b)
    };

    let mut t = x;
    for _ in 0..8 {
        let error = curve(x1, x2, t) - x;
        if error.abs() < EPSILON {
            return curve(y1, y2, t);
        }
        let derivative = slope(x1, x2, t);
        if derivative.abs() < EPSILON {
            break;
        }
        t -= error / derivative;
    }

    let (mut low, mut high) = (0.0, 1.0);
    t = x;
    for _ in 0..32 {
        let value = curve(x1, x2, t);
        if (value - x).abs() < EPSILON {
            break;
        }
        if value < x {
            low = t;
        } else {
            high = t;
        }
        t = (low + high) / 2.0;
    }
    curve(y1, y2, t)
}

/// A SMIL clock value in seconds; `None` for `indefinite` or anything unrecognised.
pub fn clock(input: &str) -> Option<f64> {
    let value = input.trim();
    if value.is_empty() || value.eq_ignore_ascii_case("indefinite") {
        return None;
    }

    if value.contains(':') {
        let mut seconds = 0.0;
        for part in value.split(':') {
            seconds = seconds * 60.0 + part.trim().parse::<f64>().ok()?;
        }
        return Some(seconds);
    }

    // Longest suffix first: "ms" also ends in "s".
    for (suffix, scale) in [("ms", 0.001), ("min", 60.0), ("h", 3600.0), ("s", 1.0)] {
        if let Some(number) = value.strip_suffix(suffix) {
            return number.trim().parse::<f64>().ok().map(|n| n * scale);
        }
    }
    value.parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frames(values: &[&str]) -> Vec<Value> {
        values.iter().map(|v| Value::parse(v)).collect()
    }

    #[test]
    fn parses_clock_values() {
        assert_eq!(clock("5"), Some(5.0));
        assert_eq!(clock("5s"), Some(5.0));
        assert_eq!(clock("250ms"), Some(0.25));
        assert_eq!(clock("1.5min"), Some(90.0));
        assert_eq!(clock("01:30"), Some(90.0));
        assert_eq!(clock("1:00:00"), Some(3600.0));
        assert_eq!(clock("indefinite"), None);
    }

    #[test]
    fn does_not_start_before_begin_or_linger_after_end() {
        let timing = Timing {
            begin: 1.0,
            dur: 2.0,
            ..Timing::default()
        };
        assert!(timing.sample(0.5).is_none());
        assert!((timing.sample(2.0).unwrap().fraction - 0.5).abs() < 1e-9);
        assert!(timing.sample(3.5).is_none());
        assert!(!timing.is_running(3.0));
    }

    #[test]
    fn freeze_holds_the_last_value_forever() {
        let timing = Timing {
            dur: 2.0,
            freeze: true,
            ..Timing::default()
        };
        let sample = timing.sample(100.0).unwrap();
        assert_eq!(sample.fraction, 1.0);
        assert!(!timing.is_running(100.0));
    }

    #[test]
    fn repeats_wrap_and_count() {
        let timing = Timing {
            dur: 2.0,
            repeat: 3.0,
            ..Timing::default()
        };
        let sample = timing.sample(5.0).unwrap();
        assert_eq!(sample.iteration, 2.0);
        assert!((sample.fraction - 0.5).abs() < 1e-9);
        assert!(timing.sample(6.0).is_none());
    }

    #[test]
    fn indefinite_repeat_never_settles() {
        let timing = Timing {
            dur: 1.0,
            repeat: f64::INFINITY,
            ..Timing::default()
        };
        assert!(timing.is_running(1_000_000.0));
    }

    #[test]
    fn key_times_reshape_the_interpolation() {
        let timing = Timing {
            key_times: Some(vec![0.0, 0.9, 1.0]),
            ..Timing::default()
        };
        let values = frames(&["0", "10", "20"]);
        assert_eq!(timing.interpolate(&values, 0.45).to_attribute(), "5");
    }

    #[test]
    fn discrete_steps_without_blending() {
        let timing = Timing {
            calc_mode: CalcMode::Discrete,
            ..Timing::default()
        };
        let values = frames(&["red", "green", "blue"]);
        assert_eq!(timing.interpolate(&values, 0.0), values[0]);
        assert_eq!(timing.interpolate(&values, 0.5), values[1]);
        assert_eq!(timing.interpolate(&values, 0.99), values[2]);
    }

    #[test]
    fn paced_spaces_by_distance_not_by_count() {
        let timing = Timing {
            calc_mode: CalcMode::Paced,
            ..Timing::default()
        };
        let values = frames(&["0", "10", "100"]);
        assert_eq!(timing.interpolate(&values, 0.5).to_attribute(), "50");
    }

    #[test]
    fn spline_easing_is_monotonic_and_hits_both_ends() {
        let timing = Timing {
            calc_mode: CalcMode::Spline,
            key_splines: Some(vec![[0.42, 0.0, 0.58, 1.0]]),
            ..Timing::default()
        };
        let values = frames(&["0", "100"]);
        assert_eq!(timing.interpolate(&values, 0.0).to_attribute(), "0");
        assert_eq!(timing.interpolate(&values, 1.0).to_attribute(), "100");
        let quarter = timing.interpolate(&values, 0.25).numbers().unwrap()[0];
        assert!(quarter < 25.0, "expected ease-in, got {quarter}");
    }
}
