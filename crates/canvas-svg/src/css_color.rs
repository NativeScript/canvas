//! Colour functions Skia's SVG parser can't read, rewritten to ones it can. Skia takes hex,
//! named colours, `rgb(r,g,b)` and `rgba(r,g,b,a)`; `hsl()`, `hsla()`, `hwb()` and the
//! `rgb(r g b / a)` form fail to parse, and a paint that fails to parse draws black. Canvas 2D
//! reads all of them through `csscolorparser`, so the same parser decides here and a colour
//! means the same thing on a canvas and in an svg.

use std::borrow::Cow;

use quick_xml::events::{BytesCData, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};

const FUNCTIONS: [&str; 5] = ["hsl", "hsla", "hwb", "rgb", "rgba"];

/// Rewrites each colour function Skia can't read in `value` (an attribute, a `style`
/// declaration list or a stylesheet), leaving everything else as is.
pub(crate) fn normalize(value: &str) -> Cow<'_, str> {
    let mut out = String::new();
    let mut rest = value;
    let mut changed = false;
    while let Some((start, name_len)) = find_function(rest) {
        let args = start + name_len + 1;
        let Some(close) = rest[args..].find(')') else {
            break;
        };
        let end = args + close + 1;
        let function = &rest[start..end];
        let name = &rest[start..start + name_len];
        // Comma-separated `rgb()`/`rgba()` is already fine.
        let unreadable = !name.eq_ignore_ascii_case("rgb") && !name.eq_ignore_ascii_case("rgba")
            || function.contains('/');
        let rewritten = unreadable
            .then(|| csscolorparser::parse(function).ok())
            .flatten();
        out.push_str(&rest[..start]);
        match rewritten {
            Some(color) => {
                out.push_str(&to_css(&color));
                changed = true;
            }
            None => out.push_str(function),
        }
        rest = &rest[end..];
    }
    if !changed {
        return Cow::Borrowed(value);
    }
    out.push_str(rest);
    Cow::Owned(out)
}

/// Rewrites attribute values and `<style>` contents; text content is left alone, since
/// "hsl(…)" there is words, not paint. The source comes back borrowed when nothing changed.
pub(crate) fn normalize_source(bytes: &[u8]) -> Cow<'_, [u8]> {
    // Cheap reject: a full XML pass for every document is not.
    let might = |name: &[u8]| {
        bytes
            .windows(name.len())
            .any(|window| window.eq_ignore_ascii_case(name))
    };
    if !might(b"hsl") && !might(b"hwb") && !might(b"rgb") {
        return Cow::Borrowed(bytes);
    }
    match rewrite(bytes) {
        Ok(Some(rewritten)) => Cow::Owned(rewritten),
        // Unchanged, or not XML Skia would parse anyway: leave it to Skia.
        _ => Cow::Borrowed(bytes),
    }
}

fn rewrite(bytes: &[u8]) -> Result<Option<Vec<u8>>, quick_xml::Error> {
    let mut reader = Reader::from_reader(bytes);
    reader.config_mut().trim_text(false);
    let mut writer = Writer::new(Vec::with_capacity(bytes.len()));
    let mut buffer = Vec::new();
    let mut in_style = 0usize;
    let mut changed = false;

    loop {
        let event = reader.read_event_into(&mut buffer)?;
        match &event {
            Event::Eof => break,
            Event::Start(element) | Event::Empty(element) => {
                let is_empty = matches!(event, Event::Empty(_));
                if !is_empty && element.local_name().as_ref().eq_ignore_ascii_case(b"style") {
                    in_style += 1;
                }
                match rewrite_attributes(element) {
                    Some(owned) => {
                        changed = true;
                        writer.write_event(if is_empty {
                            Event::Empty(owned)
                        } else {
                            Event::Start(owned)
                        })?;
                    }
                    None => writer.write_event(event.clone())?,
                }
            }
            Event::End(element) => {
                if element.local_name().as_ref().eq_ignore_ascii_case(b"style") {
                    in_style = in_style.saturating_sub(1);
                }
                writer.write_event(event.clone())?;
            }
            Event::Text(text) if in_style > 0 => {
                let css = text.unescape()?;
                match normalize(&css) {
                    Cow::Owned(css) => {
                        changed = true;
                        writer.write_event(Event::Text(BytesText::new(&css)))?;
                    }
                    Cow::Borrowed(_) => writer.write_event(event.clone())?,
                }
            }
            Event::CData(data) if in_style > 0 => {
                let css = String::from_utf8_lossy(data.as_ref());
                match normalize(&css) {
                    Cow::Owned(css) => {
                        changed = true;
                        writer.write_event(Event::CData(BytesCData::new(css)))?;
                    }
                    Cow::Borrowed(_) => writer.write_event(event.clone())?,
                }
            }
            _ => writer.write_event(event.clone())?,
        }
        buffer.clear();
    }

    Ok(changed.then(|| writer.into_inner()))
}

/// `None` when no attribute needed rewriting, so the element is written back byte for byte.
fn rewrite_attributes(element: &BytesStart) -> Option<BytesStart<'static>> {
    let attributes: Vec<_> = element.attributes().flatten().collect();
    let rewritten: Vec<Option<String>> = attributes
        .iter()
        .map(|attribute| {
            let value = attribute.unescape_value().ok()?;
            match normalize(&value) {
                Cow::Owned(value) => Some(value),
                Cow::Borrowed(_) => None,
            }
        })
        .collect();
    if rewritten.iter().all(Option::is_none) {
        return None;
    }
    let name = String::from_utf8_lossy(element.name().as_ref()).into_owned();
    let mut owned = BytesStart::new(name);
    for (attribute, value) in attributes.iter().zip(rewritten) {
        match value {
            Some(value) => {
                let key = String::from_utf8_lossy(attribute.key.as_ref());
                owned.push_attribute((key.as_ref(), value.as_str()));
            }
            None => owned.push_attribute(attribute.clone()),
        }
    }
    Some(owned.into_owned())
}

/// The next `name(` for a name in [`FUNCTIONS`], as (start, name length). Only at the start of
/// a word, so `--my-hsl(` or `xhsl(` is not taken for one.
fn find_function(s: &str) -> Option<(usize, usize)> {
    let bytes = s.as_bytes();
    let is_ident = |b: u8| b.is_ascii_alphanumeric() || b == b'-' || b == b'_';
    let mut i = 0;
    while i < bytes.len() {
        if !bytes[i].is_ascii_alphabetic() || (i > 0 && is_ident(bytes[i - 1])) {
            i += 1;
            continue;
        }
        let mut j = i;
        while j < bytes.len() && bytes[j].is_ascii_alphabetic() {
            j += 1;
        }
        if bytes.get(j) == Some(&b'(')
            && FUNCTIONS
                .iter()
                .any(|name| name.eq_ignore_ascii_case(&s[i..j]))
        {
            return Some((i, j - i));
        }
        i = j;
    }
    None
}

fn to_css(color: &csscolorparser::Color) -> String {
    let [r, g, b, a] = color.to_rgba8();
    if a == u8::MAX {
        format!("rgb({r},{g},{b})")
    } else {
        let alpha = (color.a.clamp(0.0, 1.0) * 1000.0).round() / 1000.0;
        format!("rgba({r},{g},{b},{alpha})")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_what_skia_cannot_read() {
        assert_eq!(normalize("hsl(0, 100%, 50%)"), "rgb(255,0,0)");
        assert_eq!(normalize("HSLA(120, 100%, 50%, 0.5)"), "rgba(0,255,0,0.5)");
        assert_eq!(normalize("hsl(240deg 100% 50% / 25%)"), "rgba(0,0,255,0.25)");
        assert_eq!(normalize("hwb(0 0% 0%)"), "rgb(255,0,0)");
        assert_eq!(normalize("rgb(255 0 0 / 0.5)"), "rgba(255,0,0,0.5)");
    }

    #[test]
    fn leaves_what_skia_reads_and_what_is_not_a_colour() {
        for value in ["rgb(1,2,3)", "rgba(1, 2, 3, 0.5)", "#abc", "red", "url(#g)", "none", "--my-hsl(1)", "hsl(nonsense)"] {
            assert!(matches!(normalize(value), Cow::Borrowed(_)), "{value}");
        }
    }

    #[test]
    fn rewrites_every_function_in_a_list() {
        assert_eq!(
            normalize("fill: hsl(0,100%,50%); stroke: url(#g); color: hsl(120 100% 50%)"),
            "fill: rgb(255,0,0); stroke: url(#g); color: rgb(0,255,0)"
        );
        assert_eq!(normalize("hsl(0,100%,50%);hsl(240,100%,50%)"), "rgb(255,0,0);rgb(0,0,255)");
    }

    #[test]
    fn rewrites_attributes_and_style_but_not_text() {
        let source = br##"<svg xmlns="http://www.w3.org/2000/svg"><style>.a { fill: hsl(0,100%,50%) }</style><stop stop-color="hsl(120, 100%, 50%)" offset="1"/><text fill="#fff">hsl(1,2%,3%)</text></svg>"##;
        let out = String::from_utf8(normalize_source(source).into_owned()).unwrap();
        assert_eq!(
            out,
            r##"<svg xmlns="http://www.w3.org/2000/svg"><style>.a { fill: rgb(255,0,0) }</style><stop stop-color="rgb(0,255,0)" offset="1"/><text fill="#fff">hsl(1,2%,3%)</text></svg>"##
        );
    }

    #[test]
    fn an_untouched_source_comes_back_borrowed() {
        let source = br#"<svg xmlns="http://www.w3.org/2000/svg"><rect fill="rgb(1,2,3)"/></svg>"#;
        assert!(matches!(normalize_source(source), Cow::Borrowed(_)));
    }
}

