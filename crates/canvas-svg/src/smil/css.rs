//! CSS `@keyframes` from `<style>` blocks, turned into the same [`Animation`] records SMIL uses.
//! Skia has no CSS engine beyond inline `style`, so without this such files render frozen.

use std::collections::HashMap;

use super::timing::{CalcMode, Timing};
use super::value::Value;
use super::{Animation, Frames, Kind, TransformKind};

/// One `@keyframes` block.
struct Keyframes {
    frames: Vec<Keyframe>,
}

struct Keyframe {
    offset: f64,
    declarations: Vec<(String, String)>,
}

/// How a rule says an animation runs, from the `animation` shorthand or its longhands.
#[derive(Clone, Debug, Default)]
struct AnimationSpec {
    name: String,
    duration: f64,
    delay: f64,
    iterations: f64,
    direction: Direction,
    fills_forwards: bool,
    /// The rule-level easing, used for any keyframe that does not name its own.
    easing: Option<[f64; 4]>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Direction {
    #[default]
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

/// Pulls every CSS animation out of the document's `<style>` blocks.
/// Only `#id` selectors are honoured, since the timeline addresses elements by id.
pub(super) fn extract(source: &[u8]) -> Vec<Animation> {
    let Ok(text) = std::str::from_utf8(source) else {
        return Vec::new();
    };
    if !text.contains("<style") {
        return Vec::new();
    }

    let mut animations = Vec::new();
    for css in style_blocks(text) {
        let (keyframes, rules) = parse(&css);
        for (selector, declarations) in rules {
            let Some(id) = selector.strip_prefix('#') else {
                continue;
            };
            for spec in animation_specs(&declarations) {
                let Some(frames) = keyframes.get(&spec.name) else {
                    continue;
                };
                build(id, &spec, frames, &mut animations);
            }
        }
    }
    animations
}

/// The text inside every `<style>` element, CDATA unwrapped.
fn style_blocks(text: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("<style") {
        let after = &rest[start..];
        let Some(open_end) = after.find('>') else { break };
        let Some(close) = after.find("</style") else { break };
        if close < open_end {
            break;
        }
        let mut body = after[open_end + 1..close].trim();
        if let Some(inner) = body.strip_prefix("<![CDATA[") {
            body = inner.strip_suffix("]]>").unwrap_or(inner);
        }
        blocks.push(body.to_owned());
        rest = &after[close..];
    }
    blocks
}

/// Splits a stylesheet into `@keyframes` blocks and ordinary rules; other at-rules are skipped.
fn parse(css: &str) -> (HashMap<String, Keyframes>, Vec<(String, Vec<(String, String)>)>) {
    let css = strip_comments(css);
    let bytes: Vec<char> = css.chars().collect();
    let mut keyframes = HashMap::new();
    let mut rules = Vec::new();
    let mut at = 0usize;

    while at < bytes.len() {
        let Some(open) = find(&bytes, at, '{') else { break };
        let prelude: String = bytes[at..open].iter().collect();
        let prelude = prelude.trim().to_owned();
        let Some(close) = matching_brace(&bytes, open) else { break };
        let body: String = bytes[open + 1..close].iter().collect();
        at = close + 1;

        if let Some(name) = prelude.strip_prefix('@') {
            let name = name.trim();
            if let Some(name) = name
                .strip_prefix("keyframes")
                .or_else(|| name.strip_prefix("-webkit-keyframes"))
            {
                let name = name.trim().trim_matches('"').trim_matches('\'');
                keyframes.insert(name.to_owned(), parse_keyframes(&body));
            }
            continue;
        }

        let declarations = parse_declarations(&body);
        for selector in prelude.split(',') {
            let selector = selector.trim();
            if !selector.is_empty() {
                rules.push((selector.to_owned(), declarations.clone()));
            }
        }
    }

    (keyframes, rules)
}

fn parse_keyframes(body: &str) -> Keyframes {
    let chars: Vec<char> = body.chars().collect();
    let mut frames: Vec<Keyframe> = Vec::new();
    let mut at = 0usize;

    while at < chars.len() {
        let Some(open) = find(&chars, at, '{') else { break };
        let selector: String = chars[at..open].iter().collect();
        let Some(close) = matching_brace(&chars, open) else { break };
        let block: String = chars[open + 1..close].iter().collect();
        at = close + 1;

        let declarations = parse_declarations(&block);
        for part in selector.split(',') {
            if let Some(offset) = keyframe_offset(part.trim()) {
                frames.push(Keyframe {
                    offset,
                    declarations: declarations.clone(),
                });
            }
        }
    }

    frames.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap_or(std::cmp::Ordering::Equal));
    Keyframes { frames }
}

fn keyframe_offset(selector: &str) -> Option<f64> {
    match selector {
        "from" => Some(0.0),
        "to" => Some(1.0),
        other => other
            .strip_suffix('%')
            .and_then(|n| n.trim().parse::<f64>().ok())
            .map(|percent| percent / 100.0),
    }
}

fn parse_declarations(block: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for declaration in split_top_level(block, ';') {
        let Some((property, value)) = declaration.split_once(':') else {
            continue;
        };
        let property = property.trim().to_ascii_lowercase();
        let value = value.trim();
        if !property.is_empty() && !value.is_empty() {
            out.push((property, value.to_owned()));
        }
    }
    out
}

/// The animations one rule declares. A rule may name several, comma separated.
fn animation_specs(declarations: &[(String, String)]) -> Vec<AnimationSpec> {
    let mut specs: Vec<AnimationSpec> = Vec::new();
    let lookup = |name: &str| {
        declarations
            .iter()
            .rev()
            .find(|(p, _)| p == name)
            .map(|(_, v)| v.as_str())
    };

    if let Some(shorthand) = lookup("animation") {
        for one in split_top_level(shorthand, ',') {
            if let Some(spec) = parse_shorthand(&one) {
                specs.push(spec);
            }
        }
    } else if let Some(names) = lookup("animation-name") {
        for name in split_top_level(names, ',') {
            specs.push(AnimationSpec {
                name: name.trim().to_owned(),
                ..Default::default()
            });
        }
    }

    // Longhands override the shorthand. Only the first value of each list is applied.
    for spec in specs.iter_mut() {
        if let Some(duration) = lookup("animation-duration").and_then(|v| parse_time(v.trim())) {
            spec.duration = duration;
        }
        if let Some(delay) = lookup("animation-delay").and_then(|v| parse_time(v.trim())) {
            spec.delay = delay;
        }
        if let Some(count) = lookup("animation-iteration-count") {
            spec.iterations = parse_iterations(count.trim());
        }
        if let Some(direction) = lookup("animation-direction") {
            spec.direction = parse_direction(direction.trim());
        }
        if let Some(fill) = lookup("animation-fill-mode") {
            spec.fills_forwards = matches!(fill.trim(), "forwards" | "both");
        }
        if let Some(easing) = lookup("animation-timing-function").and_then(|v| parse_easing(v.trim()))
        {
            spec.easing = Some(easing);
        }
    }

    specs.retain(|s| !s.name.is_empty() && s.duration > 0.0);
    specs
}

/// The `animation` shorthand. Parts may come in any order, so tokens are classified by shape.
fn parse_shorthand(value: &str) -> Option<AnimationSpec> {
    let mut spec = AnimationSpec {
        iterations: 1.0,
        ..Default::default()
    };
    let mut times_seen = 0;

    for token in split_functions(value) {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        if let Some(seconds) = parse_time(token) {
            // Per the grammar, the first time is the duration and the second the delay.
            if times_seen == 0 {
                spec.duration = seconds;
            } else if times_seen == 1 {
                spec.delay = seconds;
            }
            times_seen += 1;
            continue;
        }
        if let Some(easing) = parse_easing(token) {
            spec.easing = Some(easing);
            continue;
        }
        match token {
            "infinite" => spec.iterations = f64::INFINITY,
            "normal" => spec.direction = Direction::Normal,
            "reverse" => spec.direction = Direction::Reverse,
            "alternate" => spec.direction = Direction::Alternate,
            "alternate-reverse" => spec.direction = Direction::AlternateReverse,
            "forwards" | "both" => spec.fills_forwards = true,
            "none" | "backwards" | "running" | "paused" => {}
            other => {
                if let Ok(count) = other.parse::<f64>() {
                    spec.iterations = count;
                } else if spec.name.is_empty() {
                    spec.name = other.to_owned();
                }
            }
        }
    }

    (!spec.name.is_empty() && spec.duration > 0.0).then_some(spec)
}

fn parse_time(token: &str) -> Option<f64> {
    if let Some(ms) = token.strip_suffix("ms") {
        return ms.trim().parse::<f64>().ok().map(|v| v / 1000.0);
    }
    if let Some(s) = token.strip_suffix('s') {
        return s.trim().parse::<f64>().ok();
    }
    None
}

fn parse_iterations(token: &str) -> f64 {
    if token == "infinite" {
        f64::INFINITY
    } else {
        token.parse::<f64>().unwrap_or(1.0)
    }
}

fn parse_direction(token: &str) -> Direction {
    match token {
        "reverse" => Direction::Reverse,
        "alternate" => Direction::Alternate,
        "alternate-reverse" => Direction::AlternateReverse,
        _ => Direction::Normal,
    }
}

/// Only easings that map onto SMIL `keySplines`; `steps()` has no spline form and falls back to linear.
fn parse_easing(token: &str) -> Option<[f64; 4]> {
    let token = token.trim();
    if let Some(args) = token
        .strip_prefix("cubic-bezier(")
        .and_then(|rest| rest.strip_suffix(')'))
    {
        let values: Vec<f64> = args
            .split(',')
            .filter_map(|v| v.trim().parse::<f64>().ok())
            .collect();
        if values.len() == 4 {
            return Some([values[0], values[1], values[2], values[3]]);
        }
        return None;
    }
    match token {
        "linear" => Some([0.0, 0.0, 1.0, 1.0]),
        "ease" => Some([0.25, 0.1, 0.25, 1.0]),
        "ease-in" => Some([0.42, 0.0, 1.0, 1.0]),
        "ease-out" => Some([0.0, 0.0, 0.58, 1.0]),
        "ease-in-out" => Some([0.42, 0.0, 0.58, 1.0]),
        _ => None,
    }
}

/// One [`Animation`] per animated property, and one per transform function.
fn build(id: &str, spec: &AnimationSpec, keyframes: &Keyframes, out: &mut Vec<Animation>) {
    let mut properties: Vec<String> = Vec::new();
    for frame in &keyframes.frames {
        for (property, _) in &frame.declarations {
            if property != "animation-timing-function" && !properties.contains(property) {
                properties.push(property.clone());
            }
        }
    }

    for property in properties {
        let mut offsets: Vec<f64> = Vec::new();
        let mut raw: Vec<String> = Vec::new();
        let mut easings: Vec<Option<[f64; 4]>> = Vec::new();
        for frame in &keyframes.frames {
            let Some((_, value)) = frame.declarations.iter().find(|(p, _)| *p == property) else {
                continue;
            };
            offsets.push(frame.offset);
            raw.push(value.clone());
            easings.push(
                frame
                    .declarations
                    .iter()
                    .find(|(p, _)| p == "animation-timing-function")
                    .and_then(|(_, v)| parse_easing(v))
                    .or(spec.easing),
            );
        }
        if raw.len() < 2 {
            continue;
        }

        if property == "transform" {
            build_transform(id, spec, &offsets, &raw, &easings, out);
        } else {
            let values: Vec<Value> = raw.iter().map(|v| Value::parse(v)).collect();
            push(id, spec, &offsets, values, &easings, Kind::Attribute(property.clone()), out);
        }
    }
}

/// Each transform function becomes its own animation; interpolating the list as one string
/// would step between keyframes instead of moving.
fn build_transform(
    id: &str,
    spec: &AnimationSpec,
    offsets: &[f64],
    raw: &[String],
    easings: &[Option<[f64; 4]>],
    out: &mut Vec<Animation>,
) {
    let per_frame: Vec<Vec<(String, Vec<f64>)>> = raw.iter().map(|v| transform_functions(v)).collect();
    // Pairwise interpolation needs every keyframe to list the same functions in the same order.
    let Some(first) = per_frame.first() else { return };
    if per_frame
        .iter()
        .any(|frame| frame.len() != first.len() || !frame.iter().zip(first).all(|(a, b)| a.0 == b.0))
    {
        return;
    }

    for (index, (function, _)) in first.iter().enumerate() {
        let Some(kind) = transform_kind(function) else {
            continue;
        };
        let values: Vec<Value> = per_frame
            .iter()
            .map(|frame| {
                let args = &frame[index].1;
                Value::parse(
                    &args
                        .iter()
                        .map(|n| format_number(*n))
                        .collect::<Vec<_>>()
                        .join(","),
                )
            })
            .collect();
        push(id, spec, offsets, values, easings, Kind::Transform(kind), out);
    }
}

/// Units are dropped since SVG `transform` is unitless user space; angles are normalised to degrees.
fn transform_functions(value: &str) -> Vec<(String, Vec<f64>)> {
    let mut out = Vec::new();
    let mut rest = value.trim();
    while let Some(open) = rest.find('(') {
        let name = rest[..open].trim().to_ascii_lowercase();
        let Some(close) = rest[open..].find(')') else { break };
        let args = &rest[open + 1..open + close];
        let numbers: Vec<f64> = args
            .split(',')
            .flat_map(|part| part.split_whitespace())
            .filter_map(parse_length)
            .collect();
        if !name.is_empty() && !numbers.is_empty() {
            out.push((name, numbers));
        }
        rest = &rest[open + close + 1..];
    }
    out
}

fn parse_length(token: &str) -> Option<f64> {
    let token = token.trim();
    if token.is_empty() {
        return None;
    }
    for (suffix, factor) in [
        ("deg", 1.0),
        ("turn", 360.0),
        ("rad", 180.0 / std::f64::consts::PI),
        ("grad", 0.9),
        ("px", 1.0),
        ("%", 1.0),
    ] {
        if let Some(number) = token.strip_suffix(suffix) {
            return number.trim().parse::<f64>().ok().map(|v| v * factor);
        }
    }
    token.parse::<f64>().ok()
}

fn transform_kind(function: &str) -> Option<TransformKind> {
    Some(match function {
        "translate" | "translatex" | "translatey" => TransformKind::Translate,
        "scale" | "scalex" | "scaley" => TransformKind::Scale,
        "rotate" => TransformKind::Rotate,
        "skewx" => TransformKind::SkewX,
        "skewy" => TransformKind::SkewY,
        _ => return None,
    })
}

fn format_number(value: f64) -> String {
    let rounded = (value * 1e4).round() / 1e4;
    let mut text = format!("{rounded}");
    if text.ends_with(".0") {
        text.truncate(text.len() - 2);
    }
    text
}

fn push(
    id: &str,
    spec: &AnimationSpec,
    offsets: &[f64],
    values: Vec<Value>,
    easings: &[Option<[f64; 4]>],
    kind: Kind,
    out: &mut Vec<Animation>,
) {
    let (offsets, values, easings, duration) = apply_direction(spec, offsets, values, easings);

    // One spline per segment between keyframes; a missing one is linear.
    let key_splines: Vec<[f64; 4]> = easings
        .iter()
        .take(values.len().saturating_sub(1))
        .map(|easing| easing.unwrap_or([0.0, 0.0, 1.0, 1.0]))
        .collect();
    let has_easing = key_splines.iter().any(|s| *s != [0.0, 0.0, 1.0, 1.0]);

    out.push(Animation {
        target: id.to_owned(),
        ancestors: Vec::new(),
        kind,
        timing: Timing {
            begin: spec.delay,
            dur: duration,
            repeat: spec.iterations,
            freeze: spec.fills_forwards,
            additive: false,
            accumulate: false,
            calc_mode: if has_easing {
                CalcMode::Spline
            } else {
                CalcMode::Linear
            },
            key_times: Some(offsets),
            key_splines: has_easing.then_some(key_splines),
        },
        order: 0,
        frames: Frames::Values(values),
    });
}

/// SMIL has no playback direction, so keyframes are rewritten: reversed ones flip, alternating
/// ones play out and back over twice the duration.
fn apply_direction(
    spec: &AnimationSpec,
    offsets: &[f64],
    values: Vec<Value>,
    easings: &[Option<[f64; 4]>],
) -> (Vec<f64>, Vec<Value>, Vec<Option<[f64; 4]>>, f64) {
    let reverse = |offsets: &[f64], values: &[Value], easings: &[Option<[f64; 4]>]| {
        let flipped_offsets: Vec<f64> = offsets.iter().rev().map(|o| 1.0 - o).collect();
        let flipped_values: Vec<Value> = values.iter().rev().cloned().collect();
        let mut flipped_easings: Vec<Option<[f64; 4]>> = easings.to_vec();
        flipped_easings.reverse();
        (flipped_offsets, flipped_values, flipped_easings)
    };

    match spec.direction {
        Direction::Normal => (offsets.to_vec(), values, easings.to_vec(), spec.duration),
        Direction::Reverse => {
            let (o, v, e) = reverse(offsets, &values, easings);
            (o, v, e, spec.duration)
        }
        Direction::Alternate | Direction::AlternateReverse => {
            let (forward_offsets, forward_values, forward_easings) =
                if spec.direction == Direction::Alternate {
                    (offsets.to_vec(), values.clone(), easings.to_vec())
                } else {
                    reverse(offsets, &values, easings)
                };
            let (back_offsets, back_values, back_easings) =
                reverse(&forward_offsets, &forward_values, &forward_easings);

            let mut merged_offsets: Vec<f64> = forward_offsets.iter().map(|o| o / 2.0).collect();
            let mut merged_values = forward_values;
            let mut merged_easings = forward_easings;
            for (index, offset) in back_offsets.iter().enumerate().skip(1) {
                merged_offsets.push(0.5 + offset / 2.0);
                merged_values.push(back_values[index].clone());
                merged_easings.push(back_easings[index.saturating_sub(1)]);
            }
            (
                merged_offsets,
                merged_values,
                merged_easings,
                spec.duration * 2.0,
            )
        }
    }
}

fn strip_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        match rest[start + 2..].find("*/") {
            Some(end) => rest = &rest[start + 2 + end + 2..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

fn find(chars: &[char], from: usize, needle: char) -> Option<usize> {
    chars[from..].iter().position(|c| *c == needle).map(|i| i + from)
}

fn matching_brace(chars: &[char], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, c) in chars.iter().enumerate().skip(open) {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

/// Splits on a separator that is not inside brackets, so `cubic-bezier(1,0,0,1)` survives.
fn split_top_level(text: &str, separator: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for c in text.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if c == separator && depth <= 0 {
            parts.push(std::mem::take(&mut current));
        } else {
            current.push(c);
        }
    }
    if !current.trim().is_empty() {
        parts.push(current);
    }
    parts
}

/// Splits on whitespace, keeping bracketed groups together.
fn split_functions(text: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for c in text.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if c.is_whitespace() && depth <= 0 {
            if !current.is_empty() {
                parts.push(std::mem::take(&mut current));
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrap(css: &str) -> String {
        format!(r#"<svg xmlns="http://www.w3.org/2000/svg"><style>{css}</style><rect id="a"/></svg>"#)
    }

    #[test]
    fn a_document_without_a_stylesheet_costs_nothing() {
        assert!(extract(br#"<svg xmlns="http://www.w3.org/2000/svg"><rect/></svg>"#).is_empty());
    }

    #[test]
    fn reads_the_shorthand_in_any_order() {
        let spec = parse_shorthand("spin 2s linear infinite alternate forwards").unwrap();
        assert_eq!(spec.name, "spin");
        assert_eq!(spec.duration, 2.0);
        assert_eq!(spec.iterations, f64::INFINITY);
        assert_eq!(spec.direction, Direction::Alternate);
        assert!(spec.fills_forwards);

        let spec = parse_shorthand("infinite 1500ms 250ms reverse spin").unwrap();
        assert_eq!(spec.name, "spin");
        assert_eq!(spec.duration, 1.5);
        assert_eq!(spec.delay, 0.25);
        assert_eq!(spec.direction, Direction::Reverse);
    }

    #[test]
    fn keyframe_percentages_become_key_times() {
        let animations = extract(
            wrap("#a { animation: fade 4s linear } @keyframes fade { 0% {opacity: 0} 25% {opacity: 1} 100% {opacity: 0.5} }")
                .as_bytes(),
        );
        assert_eq!(animations.len(), 1);
        let timing = &animations[0].timing;
        assert_eq!(timing.dur, 4.0);
        assert_eq!(timing.key_times.as_deref(), Some(&[0.0, 0.25, 1.0][..]));
        assert!(matches!(&animations[0].kind, Kind::Attribute(a) if a == "opacity"));
    }

    #[test]
    fn from_and_to_are_zero_and_one() {
        let animations = extract(
            wrap("#a { animation: fade 1s } @keyframes fade { from {opacity: 0} to {opacity: 1} }")
                .as_bytes(),
        );
        assert_eq!(animations[0].timing.key_times.as_deref(), Some(&[0.0, 1.0][..]));
    }

    #[test]
    fn a_transform_list_becomes_one_animation_per_function() {
        let animations = extract(
            wrap(
                "#a { animation: move 1s } @keyframes move { \
                 0% {transform: translate(10px,20px) rotate(0deg)} \
                 100% {transform: translate(30px,20px) rotate(360deg)} }",
            )
            .as_bytes(),
        );
        assert_eq!(animations.len(), 2, "translate and rotate compose separately");
        assert!(matches!(animations[0].kind, Kind::Transform(TransformKind::Translate)));
        assert!(matches!(animations[1].kind, Kind::Transform(TransformKind::Rotate)));
    }

    #[test]
    fn units_are_normalised_for_the_svg_attribute() {
        assert_eq!(
            transform_functions("translate(10px,5px) rotate(0.5turn)"),
            vec![
                ("translate".to_owned(), vec![10.0, 5.0]),
                ("rotate".to_owned(), vec![180.0])
            ]
        );
    }

    #[test]
    fn per_keyframe_easing_becomes_key_splines() {
        let animations = extract(
            wrap(
                "#a { animation: fade 1s } @keyframes fade { \
                 0% {opacity: 0; animation-timing-function: cubic-bezier(1,0,0,1)} \
                 100% {opacity: 1} }",
            )
            .as_bytes(),
        );
        let timing = &animations[0].timing;
        assert_eq!(timing.calc_mode, CalcMode::Spline);
        assert_eq!(timing.key_splines.as_deref(), Some(&[[1.0, 0.0, 0.0, 1.0]][..]));
    }

    #[test]
    fn alternate_plays_out_and_back_over_twice_the_duration() {
        let animations = extract(
            wrap("#a { animation: fade 1s alternate } @keyframes fade { 0% {opacity: 0} 100% {opacity: 1} }")
                .as_bytes(),
        );
        let timing = &animations[0].timing;
        assert_eq!(timing.dur, 2.0, "a full out-and-back cycle is two durations");
        assert_eq!(timing.key_times.as_deref(), Some(&[0.0, 0.5, 1.0][..]));
    }

    #[test]
    fn only_id_selectors_are_taken() {
        let animations = extract(
            wrap(".cls { animation: fade 1s } @keyframes fade { 0% {opacity: 0} 100% {opacity: 1} }")
                .as_bytes(),
        );
        assert!(animations.is_empty(), "class selectors are not resolved to elements");
    }

    #[test]
    fn comments_and_cdata_do_not_confuse_the_parser() {
        let source = r#"<svg xmlns="http://www.w3.org/2000/svg"><style><![CDATA[
            /* a comment { with braces } */
            #a { animation: fade 1s }
            @keyframes fade { 0% {opacity: 0} 100% {opacity: 1} }
        ]]></style><rect id="a"/></svg>"#;
        assert_eq!(extract(source.as_bytes()).len(), 1);
    }
}
