//! Strips SMIL elements from the raw source (Skia's parser drops them) and gives each target an
//! `id`. Two passes, since an animation is written inside the element it animates.

use std::collections::{HashMap, HashSet};

use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};

use super::motion::{Motion, Rotate};
use super::timing::{clock, CalcMode, Timing};
use super::value::Value;
use super::{Animation, Frames, Kind, TransformKind};

/// Must not collide with author ids; visible in `getElementById` results.
const GENERATED_ID_PREFIX: &str = "__nsc_smil_";

pub struct Extracted {
    /// No animation elements, every target addressable by id.
    pub source: Vec<u8>,
    pub animations: Vec<Animation>,
}

fn is_animation_tag(tag: &str) -> bool {
    matches!(tag, "animate" | "set" | "animatetransform" | "animatemotion")
}

/// Lowercased local name, so `svg:animate` and `animate` are the same tag and `xlink:href` and
/// `href` are the same attribute.
fn local_name(element: &BytesStart) -> String {
    String::from_utf8_lossy(element.local_name().as_ref()).to_ascii_lowercase()
}

fn attributes(element: &BytesStart) -> HashMap<String, String> {
    element
        .attributes()
        .flatten()
        .map(|attribute| {
            let key = String::from_utf8_lossy(attribute.key.local_name().as_ref())
                .to_ascii_lowercase();
            let value = attribute
                .unescape_value()
                .map(|v| v.into_owned())
                .unwrap_or_else(|_| String::from_utf8_lossy(&attribute.value).into_owned());
            (key, value)
        })
        .collect()
}

/// An animation element, before its target has been turned into an id.
struct Pending {
    tag: String,
    attributes: HashMap<String, String>,
    /// The element it animates: the index of its parent, or an explicit `href`.
    target: Target,
    /// `<mpath href="#p">` inside an `<animateMotion>`, resolved once every path is known.
    mpath: Option<String>,
}

enum Target {
    /// Position in document order, resolved to an id in the second pass.
    Element(usize),
    Id(String),
}

struct Scan {
    pending: Vec<Pending>,
    /// Element index -> its `id`, for elements that already had one.
    ids: HashMap<usize, String>,
    /// Every id in the document, so generated ones do not collide.
    taken: HashSet<String>,
    /// `id` -> `d`, so `<mpath>` can find the path it points at.
    paths: HashMap<String, String>,
    /// Element index -> index into `pending`, so `<mpath>` reaches its enclosing animation.
    animation_elements: HashMap<usize, usize>,
    /// Element index -> parent index. Skia keeps ids in a side map, not on nodes, so layer
    /// promotion's ancestor check has to come from the source.
    parents: HashMap<usize, usize>,
}

fn scan(bytes: &[u8]) -> Result<Scan, quick_xml::Error> {
    let mut reader = Reader::from_reader(bytes);
    reader.config_mut().trim_text(false);

    let mut result = Scan {
        pending: Vec::new(),
        ids: HashMap::new(),
        taken: HashSet::new(),
        paths: HashMap::new(),
        animation_elements: HashMap::new(),
        parents: HashMap::new(),
    };

    // Indices of the elements currently open, so an animation can find its parent.
    let mut open: Vec<usize> = Vec::new();
    let mut index = 0usize;
    let mut buffer = Vec::new();

    loop {
        let event = reader.read_event_into(&mut buffer)?;
        let (element, is_empty) = match &event {
            Event::Start(element) => (element, false),
            Event::Empty(element) => (element, true),
            Event::End(_) => {
                open.pop();
                buffer.clear();
                continue;
            }
            Event::Eof => break,
            _ => {
                buffer.clear();
                continue;
            }
        };

        let tag = local_name(element);
        let attrs = attributes(element);
        let current = index;
        index += 1;
        if let Some(parent) = open.last() {
            result.parents.insert(current, *parent);
        }

        if let Some(id) = attrs.get("id") {
            result.ids.insert(current, id.clone());
            result.taken.insert(id.clone());
            if let Some(d) = attrs.get("d") {
                result.paths.insert(id.clone(), d.clone());
            }
        }

        if tag == "mpath" {
            // Belongs to the <animateMotion> it is written inside.
            if let Some(index) = open
                .last()
                .and_then(|parent| result.animation_elements.get(parent))
                .copied()
            {
                if result.pending[index].tag == "animatemotion" {
                    result.pending[index].mpath = attrs
                        .get("href")
                        .map(|href| href.trim_start_matches('#').to_owned());
                }
            }
        } else if is_animation_tag(&tag) {
            let target = match attrs.get("href").map(|h| h.trim_start_matches('#')) {
                Some(id) if !id.is_empty() => Target::Id(id.to_owned()),
                // No href: SMIL animates the element it is written inside.
                _ => match open.last() {
                    Some(parent) => Target::Element(*parent),
                    // An animation element at the document root animates nothing.
                    None => {
                        if !is_empty {
                            open.push(current);
                        }
                        buffer.clear();
                        continue;
                    }
                },
            };
            result.animation_elements.insert(current, result.pending.len());
            result.pending.push(Pending {
                tag,
                attributes: attrs,
                target,
                mpath: None,
            });
        }

        if !is_empty {
            open.push(current);
        }
        buffer.clear();
    }

    Ok(result)
}

/// Rewrites the source without the animation elements, adding the ids they need.
fn rewrite(
    bytes: &[u8],
    needed: &HashMap<usize, String>,
) -> Result<Vec<u8>, quick_xml::Error> {
    let mut reader = Reader::from_reader(bytes);
    reader.config_mut().trim_text(false);
    let mut writer = Writer::new(Vec::new());

    let mut index = 0usize;
    // Depth inside an animation element being dropped; its children go with it.
    let mut skipping = 0usize;
    let mut buffer = Vec::new();

    loop {
        let event = reader.read_event_into(&mut buffer)?;
        match &event {
            Event::Eof => break,
            Event::Start(element) | Event::Empty(element) => {
                let is_empty = matches!(event, Event::Empty(_));
                let current = index;
                index += 1;

                if skipping > 0 {
                    if !is_empty {
                        skipping += 1;
                    }
                    buffer.clear();
                    continue;
                }
                if is_animation_tag(&local_name(element)) {
                    if !is_empty {
                        skipping = 1;
                    }
                    buffer.clear();
                    continue;
                }

                match needed.get(&current) {
                    Some(id) => {
                        let mut owned = element.clone();
                        owned.push_attribute(("id", id.as_str()));
                        writer.write_event(if is_empty {
                            Event::Empty(owned)
                        } else {
                            Event::Start(owned)
                        })?;
                    }
                    None => writer.write_event(event.clone())?,
                }
            }
            Event::End(_) => {
                if skipping > 0 {
                    skipping -= 1;
                    buffer.clear();
                    continue;
                }
                writer.write_event(event.clone())?;
            }
            _ => {
                if skipping == 0 {
                    writer.write_event(event.clone())?;
                }
            }
        }
        buffer.clear();
    }

    Ok(writer.into_inner())
}

/// A source without animation elements comes back untouched, with no invented ids.
pub fn extract(bytes: &[u8]) -> Extracted {
    // CSS animations address existing ids, so they need no rewrite; appended to the SMIL set.
    let css = super::css::extract(bytes);

    let unchanged = || Extracted {
        source: bytes.to_vec(),
        animations: ordered(css.clone()),
    };

    // Cheap reject: most SVGs have no SMIL, and a scan is a full XML parse.
    if !contains_animation_tag(bytes) {
        return unchanged();
    }

    let Ok(scanned) = scan(bytes) else {
        return unchanged();
    };
    if scanned.pending.is_empty() {
        return unchanged();
    }

    // Give every target that lacks an id one of ours.
    let mut generated: HashMap<usize, String> = HashMap::new();
    let mut taken = scanned.taken.clone();
    let mut next = 0usize;
    let mut target_ids: Vec<Option<String>> = Vec::with_capacity(scanned.pending.len());

    for pending in &scanned.pending {
        let id = match &pending.target {
            Target::Id(id) => Some(id.clone()),
            Target::Element(element) => Some(
                scanned
                    .ids
                    .get(element)
                    .cloned()
                    .or_else(|| generated.get(element).cloned())
                    .unwrap_or_else(|| {
                        let mut id = format!("{GENERATED_ID_PREFIX}{next}");
                        while taken.contains(&id) {
                            next += 1;
                            id = format!("{GENERATED_ID_PREFIX}{next}");
                        }
                        next += 1;
                        taken.insert(id.clone());
                        generated.insert(*element, id.clone());
                        id
                    }),
            ),
        };
        target_ids.push(id);
    }

    let Ok(source) = rewrite(bytes, &generated) else {
        return unchanged();
    };

    // id -> element index, for listing each animated element's ancestors.
    let mut element_of_id: HashMap<&str, usize> = HashMap::new();
    for (element, id) in scanned.ids.iter() {
        element_of_id.insert(id.as_str(), *element);
    }
    for (element, id) in generated.iter() {
        element_of_id.insert(id.as_str(), *element);
    }
    let id_of = |element: &usize| {
        scanned
            .ids
            .get(element)
            .or_else(|| generated.get(element))
            .cloned()
    };
    let ancestors_of = |element: usize| {
        let mut chain = Vec::new();
        let mut at = element;
        while let Some(parent) = scanned.parents.get(&at) {
            if let Some(id) = id_of(parent) {
                chain.push(id);
            }
            at = *parent;
        }
        chain
    };

    let mut animations: Vec<Animation> = scanned
        .pending
        .iter()
        .zip(target_ids)
        .filter_map(|(pending, target)| {
            let target = target?;
            let element = match &pending.target {
                Target::Element(element) => Some(*element),
                Target::Id(id) => element_of_id.get(id.as_str()).copied(),
            };
            let mut animation = build(pending, target, &scanned.paths)?;
            animation.ancestors = element.map(ancestors_of).unwrap_or_default();
            Some(animation)
        })
        .collect();
    animations.extend(css);

    Extracted {
        source,
        animations: ordered(animations),
    }
}

/// Document order is the tie-break when several animations drive one attribute, so it is
/// assigned once over the combined set rather than per mechanism.
fn ordered(mut animations: Vec<Animation>) -> Vec<Animation> {
    for (order, animation) in animations.iter_mut().enumerate() {
        animation.order = order;
    }
    animations
}

/// False positives only cost a scan. `set` needs its delimiter since many SVGs contain "offset".
fn contains_animation_tag(bytes: &[u8]) -> bool {
    fn contains(haystack: &[u8], needle: &[u8]) -> bool {
        haystack.windows(needle.len()).any(|window| window == needle)
    }
    contains(bytes, b"animate") || contains(bytes, b"<set") || contains(bytes, b":set")
}

fn build(
    pending: &Pending,
    target: String,
    paths: &HashMap<String, String>,
) -> Option<Animation> {
    let attrs = &pending.attributes;
    let get = |name: &str| attrs.get(name).map(String::as_str);

    let kind = match pending.tag.as_str() {
        "animatetransform" => Kind::Transform(TransformKind::parse(get("type").unwrap_or(""))),
        "animatemotion" => {
            let rotate = match get("rotate").map(str::trim) {
                Some("auto") => Rotate::Auto,
                Some("auto-reverse") => Rotate::AutoReverse,
                Some(angle) => Rotate::Angle(angle.parse().unwrap_or(0.0)),
                None => Rotate::Angle(0.0),
            };
            // An inline `path` wins over `<mpath>`, per spec.
            let d = get("path")
                .map(str::to_owned)
                .or_else(|| pending.mpath.as_ref().and_then(|id| paths.get(id).cloned()))?;
            Kind::Motion(Motion::new(&d, rotate)?)
        }
        // `<set>` and `<animate>` both drive a named attribute; `<set>` just never interpolates.
        _ => Kind::Attribute(get("attributename")?.trim().to_owned()),
    };

    let frames = build_frames(&pending.tag, &kind, attrs)?;
    let timing = build_timing(&pending.tag, attrs);

    Some(Animation {
        target,
        ancestors: Vec::new(),
        kind,
        timing,
        order: 0,
        frames,
    })
}

fn build_frames(
    tag: &str,
    kind: &Kind,
    attrs: &HashMap<String, String>,
) -> Option<Frames> {
    let get = |name: &str| attrs.get(name).map(String::as_str);

    // Motion's keyframes are the path itself; the fraction is all it needs.
    if matches!(kind, Kind::Motion(_)) {
        return Some(Frames::Values(Vec::new()));
    }

    // `<set>` holds one value for its whole active duration.
    if tag == "set" {
        return Some(Frames::Values(vec![Value::parse(get("to")?)]));
    }

    if let Some(values) = get("values") {
        let parsed: Vec<Value> = values
            .split(';')
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(Value::parse)
            .collect();
        if !parsed.is_empty() {
            return Some(Frames::Values(parsed));
        }
    }

    match (get("from"), get("to"), get("by")) {
        (Some(from), Some(to), _) => Some(Frames::Values(vec![
            Value::parse(from),
            Value::parse(to),
        ])),
        (Some(from), None, Some(by)) => {
            let start = Value::parse(from);
            let end = start.add(&Value::parse(by));
            Some(Frames::Values(vec![start, end]))
        }
        // Without a `from`, the animation starts from whatever the element already is, which
        // is only known once the document exists.
        (None, Some(to), _) => Some(Frames::ToOnly(Value::parse(to))),
        (None, None, Some(by)) => Some(Frames::ByOnly(Value::parse(by))),
        _ => None,
    }
}

fn build_timing(tag: &str, attrs: &HashMap<String, String>) -> Timing {
    let get = |name: &str| attrs.get(name).map(String::as_str);

    // `begin` may mix clock values with event/syncbase specs ("0s;go.end"); the first clock
    // value wins, and a purely event-driven begin is parked rather than fired at zero.
    let begin = match get("begin") {
        None => 0.0,
        Some(list) => list
            .split(';')
            .find_map(clock)
            .unwrap_or(f64::INFINITY),
    };

    let repeat = match get("repeatcount").map(str::trim) {
        None => 1.0,
        Some("indefinite") => f64::INFINITY,
        Some(count) => count.parse().unwrap_or(1.0),
    };

    let calc_mode = match get("calcmode").map(str::trim) {
        Some("discrete") => CalcMode::Discrete,
        Some("paced") => CalcMode::Paced,
        Some("spline") => CalcMode::Spline,
        // `<set>` never interpolates, whatever it says.
        _ if tag == "set" => CalcMode::Discrete,
        _ => CalcMode::Linear,
    };

    let key_times = get("keytimes").map(|list| {
        list.split(';')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .filter_map(|t| t.parse().ok())
            .collect()
    });

    let key_splines = get("keysplines").map(|list| {
        list.split(';')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|spline| {
                let numbers: Vec<f64> = spline
                    .split(|c: char| c == ',' || c.is_whitespace())
                    .filter(|n| !n.is_empty())
                    .filter_map(|n| n.parse().ok())
                    .collect();
                <[f64; 4]>::try_from(numbers.as_slice()).ok()
            })
            .collect()
    });

    Timing {
        begin,
        dur: get("dur").and_then(clock).unwrap_or(f64::INFINITY),
        repeat,
        freeze: get("fill").map(str::trim) == Some("freeze"),
        additive: get("additive").map(str::trim) == Some("sum"),
        accumulate: get("accumulate").map(str::trim) == Some("sum"),
        calc_mode,
        key_times,
        key_splines,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(bytes: &[u8]) -> String {
        String::from_utf8_lossy(bytes).into_owned()
    }

    #[test]
    fn leaves_a_document_without_smil_completely_alone() {
        let source = br##"<svg><rect width="10" height="10"/></svg>"##;
        let extracted = extract(source);
        assert_eq!(extracted.source, source.to_vec());
        assert!(extracted.animations.is_empty());
    }

    #[test]
    fn strips_the_animation_and_gives_its_target_an_id() {
        let source = br##"<svg><rect width="10"><animate attributeName="width" from="10" to="20" dur="2s"/></rect></svg>"##;
        let extracted = extract(source);
        let rewritten = text(&extracted.source);

        assert!(!rewritten.contains("animate"), "{rewritten}");
        assert!(rewritten.contains(r#"id="__nsc_smil_0""#), "{rewritten}");
        assert_eq!(extracted.animations.len(), 1);
        assert_eq!(extracted.animations[0].target, "__nsc_smil_0");
        assert_eq!(extracted.animations[0].timing.dur, 2.0);
    }

    #[test]
    fn keeps_an_id_the_author_already_wrote() {
        let source = br##"<svg><rect id="box"><animate attributeName="x" to="5" dur="1s"/></rect></svg>"##;
        let extracted = extract(source);
        assert_eq!(extracted.animations[0].target, "box");
        assert!(!text(&extracted.source).contains("__nsc_smil"));
    }

    #[test]
    fn honours_an_href_target() {
        let source = br##"<svg><rect id="box"/><animate href="#box" attributeName="x" to="5" dur="1s"/></svg>"##;
        let extracted = extract(source);
        assert_eq!(extracted.animations[0].target, "box");
    }

    #[test]
    fn several_animations_on_one_element_share_its_generated_id() {
        let source = br##"<svg><rect>
            <animate attributeName="x" to="5" dur="1s"/>
            <animate attributeName="y" to="5" dur="1s"/>
        </rect></svg>"##;
        let extracted = extract(source);
        assert_eq!(extracted.animations.len(), 2);
        assert_eq!(extracted.animations[0].target, extracted.animations[1].target);
        assert_eq!(text(&extracted.source).matches("__nsc_smil_0").count(), 1);
    }

    #[test]
    fn generated_ids_do_not_collide_with_the_authors() {
        let source = br##"<svg><rect id="__nsc_smil_0"/><circle><animate attributeName="r" to="5" dur="1s"/></circle></svg>"##;
        let extracted = extract(source);
        assert_ne!(extracted.animations[0].target, "__nsc_smil_0");
    }

    #[test]
    fn drops_mpath_along_with_its_animation() {
        let source = br##"<svg><path id="track" d="M0 0 L100 0"/><circle><animateMotion dur="2s"><mpath href="#track"/></animateMotion></circle></svg>"##;
        let extracted = extract(source);
        let rewritten = text(&extracted.source);
        assert!(!rewritten.contains("mpath"), "{rewritten}");
        assert!(rewritten.contains(r#"id="track""#));
        assert_eq!(extracted.animations.len(), 1);
        assert!(matches!(extracted.animations[0].kind, Kind::Motion(_)));
    }

    #[test]
    fn parses_the_timing_attributes() {
        let source = br##"<svg><rect><animate attributeName="x" values="0;10;20" keyTimes="0;0.8;1"
            begin="1s" dur="500ms" repeatCount="indefinite" fill="freeze" additive="sum"
            accumulate="sum" calcMode="spline" keySplines="0 0 1 1;.5 0 .5 1"/></rect></svg>"##;
        let timing = &extract(source).animations[0].timing;
        assert_eq!(timing.begin, 1.0);
        assert_eq!(timing.dur, 0.5);
        assert!(timing.repeat.is_infinite());
        assert!(timing.freeze && timing.additive && timing.accumulate);
        assert_eq!(timing.calc_mode, CalcMode::Spline);
        assert_eq!(timing.key_times.as_ref().unwrap().len(), 3);
        assert_eq!(timing.key_splines.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn an_event_driven_begin_is_parked_rather_than_fired_at_zero() {
        let source = br##"<svg><rect><animate attributeName="x" to="5" dur="1s" begin="click"/></rect></svg>"##;
        assert!(extract(source).animations[0].timing.begin.is_infinite());
    }

    #[test]
    fn preserves_the_rest_of_the_document_verbatim() {
        let source = br##"<?xml version="1.0"?><!-- keep --><svg xmlns="http://www.w3.org/2000/svg"><g fill="red"><text>hi</text><rect><set attributeName="x" to="1"/></rect></g></svg>"##;
        let rewritten = text(&extract(source).source);
        assert!(rewritten.contains("<!-- keep -->"), "{rewritten}");
        assert!(rewritten.contains(r#"xmlns="http://www.w3.org/2000/svg""#));
        assert!(rewritten.contains("<text>hi</text>"));
        assert!(!rewritten.contains("<set"));
    }
}
