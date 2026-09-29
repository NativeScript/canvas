//! `<style>` rules, which Skia's SVG module ignores. They are applied when a document is parsed:
//! each element's matching declarations are folded into its `style` attribute, which Skia does
//! read, in cascade order. Selector rules beat presentation attributes, inline `style` beats
//! selector rules, and `!important` beats both.
//!
//! Selectors: type, `*`, `#id`, `.class`, attribute (`[a]`, `=`, `~=`, `|=`, `^=`, `$=`, `*=`),
//! `:root`, `:first-child`, `:last-child`, `:only-child`, and the descendant, `>`, `+` and `~`
//! combinators. A rule with anything else is dropped rather than risk matching too much.

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;

/// The source with rules folded in, or `None` when there were none to apply.
pub(crate) fn apply(bytes: &[u8]) -> Option<Vec<u8>> {
    if !bytes.windows(6).any(|window| window == b"<style") {
        return None;
    }
    let (elements, css) = scan(bytes).ok()?;
    let rules = parse_rules(&css);
    if rules.is_empty() {
        return None;
    }
    let styles: Vec<Option<String>> = (0..elements.len())
        .map(|index| cascade(index, &elements, &rules))
        .collect();
    if styles.iter().all(Option::is_none) {
        return None;
    }
    rewrite(bytes, &styles).ok()
}

struct Element {
    tag: String,
    id: Option<String>,
    classes: Vec<String>,
    attributes: Vec<(String, String)>,
    style: Option<String>,
    parent: Option<usize>,
    previous: Option<usize>,
    next: Option<usize>,
}

impl Element {
    fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }
}

/// Every element in document order, with its tree links, and the text of every `<style>`.
fn scan(bytes: &[u8]) -> Result<(Vec<Element>, String), quick_xml::Error> {
    let mut reader = Reader::from_reader(bytes);
    reader.config_mut().trim_text(false);
    let mut elements: Vec<Element> = Vec::new();
    // Open elements, each with its last element child so far.
    let mut open: Vec<(usize, Option<usize>)> = Vec::new();
    let mut last_root: Option<usize> = None;
    let mut css = String::new();
    let mut in_style = 0usize;
    let mut buffer = Vec::new();

    loop {
        let event = reader.read_event_into(&mut buffer)?;
        match &event {
            Event::Eof => break,
            Event::Start(start) | Event::Empty(start) => {
                let index = elements.len();
                let (parent, previous) = match open.last() {
                    Some((parent, last)) => (Some(*parent), *last),
                    None => (None, last_root),
                };
                if let Some(previous) = previous {
                    elements[previous].next = Some(index);
                }
                elements.push(element(start, parent, previous));
                match open.last_mut() {
                    Some((_, last)) => *last = Some(index),
                    None => last_root = Some(index),
                }
                if matches!(event, Event::Start(_)) {
                    if elements[index].tag == "style" {
                        in_style += 1;
                    }
                    open.push((index, None));
                }
            }
            Event::End(_) => {
                if let Some((index, _)) = open.pop() {
                    if elements[index].tag == "style" {
                        in_style = in_style.saturating_sub(1);
                    }
                }
            }
            Event::Text(text) if in_style > 0 => {
                css.push_str(&text.unescape()?);
                css.push('\n');
            }
            Event::CData(data) if in_style > 0 => {
                css.push_str(&String::from_utf8_lossy(data.as_ref()));
                css.push('\n');
            }
            _ => {}
        }
        buffer.clear();
    }
    Ok((elements, css))
}

fn element(start: &BytesStart, parent: Option<usize>, previous: Option<usize>) -> Element {
    let tag = String::from_utf8_lossy(start.local_name().as_ref()).into_owned();
    let attributes: Vec<(String, String)> = start
        .attributes()
        .flatten()
        .map(|attribute| {
            let key = String::from_utf8_lossy(attribute.key.local_name().as_ref()).into_owned();
            let value = attribute
                .unescape_value()
                .map(|value| value.into_owned())
                .unwrap_or_default();
            (key, value)
        })
        .collect();
    let find = |name: &str| {
        attributes
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.clone())
    };
    Element {
        tag,
        id: find("id"),
        classes: find("class")
            .map(|classes| classes.split_whitespace().map(str::to_owned).collect())
            .unwrap_or_default(),
        style: find("style"),
        attributes,
        parent,
        previous,
        next: None,
    }
}

struct Rule {
    selector: Selector,
    specificity: (u32, u32, u32),
    order: usize,
    declarations: Vec<Declaration>,
}

struct Declaration {
    property: String,
    value: String,
    important: bool,
}

fn parse_rules(css: &str) -> Vec<Rule> {
    crate::smil::style_rules(css)
        .into_iter()
        .enumerate()
        .filter_map(|(order, (selector, declarations))| {
            let selector = parse_selector(&selector)?;
            Some(Rule {
                specificity: selector.specificity(),
                selector,
                order,
                declarations: declarations
                    .into_iter()
                    .map(|(property, value)| declaration(property, &value))
                    .collect(),
            })
        })
        .collect()
}

fn declaration(property: String, value: &str) -> Declaration {
    let value = value.trim();
    let (value, important) = match value.rfind('!') {
        Some(bang) if value[bang + 1..].trim().eq_ignore_ascii_case("important") => {
            (value[..bang].trim_end(), true)
        }
        _ => (value, false),
    };
    Declaration {
        property,
        value: value.to_owned(),
        important,
    }
}

/// The element's new `style`, or `None` when no rule matches it.
fn cascade(index: usize, elements: &[Element], rules: &[Rule]) -> Option<String> {
    let mut matched: Vec<&Rule> = rules
        .iter()
        .filter(|rule| rule.selector.matches(index, elements))
        .collect();
    if matched.is_empty() {
        return None;
    }
    matched.sort_by_key(|rule| (rule.specificity, rule.order));

    let inline: Vec<Declaration> = elements[index]
        .style
        .as_deref()
        .map(|style| {
            crate::smil::style_declarations(style)
                .into_iter()
                .map(|(property, value)| declaration(property, &value))
                .collect()
        })
        .unwrap_or_default();

    // Later wins in a `style` attribute, so this order is the cascade.
    let from_rules = |important: bool| {
        matched
            .iter()
            .flat_map(|rule| &rule.declarations)
            .filter(move |declaration| declaration.important == important)
    };
    let from_inline =
        |important: bool| inline.iter().filter(move |declaration| declaration.important == important);
    let style: Vec<String> = from_rules(false)
        .chain(from_inline(false))
        .chain(from_rules(true))
        .chain(from_inline(true))
        .map(|declaration| format!("{}:{}", declaration.property, declaration.value))
        .collect();
    Some(style.join(";"))
}

/// Writes each styled element with its new `style` last: Skia applies attributes in order, so a
/// presentation attribute after it would otherwise win.
fn rewrite(bytes: &[u8], styles: &[Option<String>]) -> Result<Vec<u8>, quick_xml::Error> {
    let mut reader = Reader::from_reader(bytes);
    reader.config_mut().trim_text(false);
    let mut writer = Writer::new(Vec::with_capacity(bytes.len()));
    let mut index = 0usize;
    let mut buffer = Vec::new();

    loop {
        let event = reader.read_event_into(&mut buffer)?;
        match &event {
            Event::Eof => break,
            Event::Start(start) | Event::Empty(start) => {
                let is_empty = matches!(event, Event::Empty(_));
                match styles.get(index).and_then(Option::as_deref) {
                    Some(style) => {
                        let name = String::from_utf8_lossy(start.name().as_ref()).into_owned();
                        let mut owned = BytesStart::new(name);
                        for attribute in start.attributes().flatten() {
                            if attribute.key.local_name().as_ref() != b"style" {
                                owned.push_attribute(attribute);
                            }
                        }
                        owned.push_attribute(("style", style));
                        writer.write_event(if is_empty {
                            Event::Empty(owned)
                        } else {
                            Event::Start(owned)
                        })?;
                    }
                    None => writer.write_event(event.clone())?,
                }
                index += 1;
            }
            _ => writer.write_event(event.clone())?,
        }
        buffer.clear();
    }
    Ok(writer.into_inner())
}

/// Compounds right to left: `compounds[0]` is the subject, and each combinator relates a
/// compound to the next one along.
struct Selector {
    compounds: Vec<(Compound, Combinator)>,
}

#[derive(Clone, Copy)]
enum Combinator {
    Descendant,
    Child,
    Adjacent,
    Sibling,
}

#[derive(Default)]
struct Compound {
    tag: Option<String>,
    id: Option<String>,
    classes: Vec<String>,
    attributes: Vec<AttributeSelector>,
    pseudo: Vec<Pseudo>,
}

struct AttributeSelector {
    name: String,
    test: Option<(AttributeOperator, String, bool)>,
}

#[derive(Clone, Copy)]
enum AttributeOperator {
    Equals,
    Includes,
    Dash,
    Prefix,
    Suffix,
    Substring,
}

#[derive(Clone, Copy)]
enum Pseudo {
    Root,
    FirstChild,
    LastChild,
    OnlyChild,
}

impl Selector {
    fn specificity(&self) -> (u32, u32, u32) {
        self.compounds.iter().fold((0, 0, 0), |(a, b, c), (compound, _)| {
            (
                a + compound.id.is_some() as u32,
                b + (compound.classes.len() + compound.attributes.len() + compound.pseudo.len()) as u32,
                c + compound.tag.is_some() as u32,
            )
        })
    }

    fn matches(&self, index: usize, elements: &[Element]) -> bool {
        self.matches_from(0, index, elements)
    }

    fn matches_from(&self, at: usize, index: usize, elements: &[Element]) -> bool {
        let (compound, combinator) = &self.compounds[at];
        if !compound.matches(&elements[index]) {
            return false;
        }
        if at + 1 == self.compounds.len() {
            return true;
        }
        let element = &elements[index];
        match combinator {
            Combinator::Child => element
                .parent
                .is_some_and(|parent| self.matches_from(at + 1, parent, elements)),
            Combinator::Adjacent => element
                .previous
                .is_some_and(|previous| self.matches_from(at + 1, previous, elements)),
            Combinator::Descendant => {
                std::iter::successors(element.parent, |&ancestor| elements[ancestor].parent)
                    .any(|ancestor| self.matches_from(at + 1, ancestor, elements))
            }
            Combinator::Sibling => {
                std::iter::successors(element.previous, |&sibling| elements[sibling].previous)
                    .any(|sibling| self.matches_from(at + 1, sibling, elements))
            }
        }
    }
}

impl Compound {
    fn matches(&self, element: &Element) -> bool {
        self.tag.as_ref().is_none_or(|tag| *tag == element.tag)
            && self.id.as_ref().is_none_or(|id| element.id.as_ref() == Some(id))
            && self.classes.iter().all(|class| element.classes.contains(class))
            && self.attributes.iter().all(|selector| selector.matches(element))
            && self.pseudo.iter().all(|pseudo| match pseudo {
                Pseudo::Root => element.parent.is_none(),
                Pseudo::FirstChild => element.previous.is_none(),
                Pseudo::LastChild => element.next.is_none(),
                Pseudo::OnlyChild => element.previous.is_none() && element.next.is_none(),
            })
    }
}

impl AttributeSelector {
    fn matches(&self, element: &Element) -> bool {
        let Some(actual) = element.attribute(&self.name) else {
            return false;
        };
        let Some((operator, expected, insensitive)) = &self.test else {
            return true;
        };
        let (actual, expected) = if *insensitive {
            (actual.to_lowercase(), expected.to_lowercase())
        } else {
            (actual.to_owned(), expected.clone())
        };
        match operator {
            AttributeOperator::Equals => actual == expected,
            AttributeOperator::Includes => actual.split_whitespace().any(|word| word == expected),
            AttributeOperator::Dash => {
                actual == expected || actual.starts_with(&format!("{expected}-"))
            }
            AttributeOperator::Prefix => !expected.is_empty() && actual.starts_with(&expected),
            AttributeOperator::Suffix => !expected.is_empty() && actual.ends_with(&expected),
            AttributeOperator::Substring => !expected.is_empty() && actual.contains(&expected),
        }
    }
}

fn parse_selector(text: &str) -> Option<Selector> {
    let chars: Vec<char> = text.trim().chars().collect();
    let mut at = 0usize;
    // Left to right while parsing; reversed at the end.
    let mut compounds: Vec<(Compound, Combinator)> = Vec::new();
    let mut pending = Combinator::Descendant;

    loop {
        let compound = parse_compound(&chars, &mut at)?;
        compounds.push((compound, pending));

        let had_space = skip_space(&chars, &mut at);
        let Some(&next) = chars.get(at) else { break };
        pending = match next {
            '>' => Combinator::Child,
            '+' => Combinator::Adjacent,
            '~' => Combinator::Sibling,
            _ if had_space => Combinator::Descendant,
            _ => return None,
        };
        if matches!(next, '>' | '+' | '~') {
            at += 1;
            skip_space(&chars, &mut at);
        }
    }

    // Each compound's combinator now describes its link to the one before it; right to left,
    // that link belongs to the compound after it.
    let combinators: Vec<Combinator> = compounds.iter().map(|(_, combinator)| *combinator).collect();
    let mut reversed = Vec::with_capacity(compounds.len());
    for (index, (compound, _)) in compounds.into_iter().enumerate().rev() {
        reversed.push((compound, combinators[index]));
    }
    Some(Selector { compounds: reversed })
}

fn parse_compound(chars: &[char], at: &mut usize) -> Option<Compound> {
    let mut compound = Compound::default();
    let start = *at;

    match chars.get(*at) {
        Some('*') => *at += 1,
        Some(c) if is_ident_start(*c) => compound.tag = Some(ident(chars, at)?),
        _ => {}
    }
    loop {
        match chars.get(*at) {
            Some('#') => {
                *at += 1;
                compound.id = Some(ident(chars, at)?);
            }
            Some('.') => {
                *at += 1;
                compound.classes.push(ident(chars, at)?);
            }
            Some('[') => {
                *at += 1;
                compound.attributes.push(attribute_selector(chars, at)?);
            }
            Some(':') => {
                *at += 1;
                let name = ident(chars, at)?.to_ascii_lowercase();
                compound.pseudo.push(match name.as_str() {
                    "root" => Pseudo::Root,
                    "first-child" => Pseudo::FirstChild,
                    "last-child" => Pseudo::LastChild,
                    "only-child" => Pseudo::OnlyChild,
                    _ => return None,
                });
            }
            Some(&c) if c.is_whitespace() || matches!(c, '>' | '+' | '~') => break,
            None => break,
            Some(_) => return None,
        }
    }
    (*at > start).then_some(compound)
}

fn attribute_selector(chars: &[char], at: &mut usize) -> Option<AttributeSelector> {
    skip_space(chars, at);
    let name = ident(chars, at)?;
    skip_space(chars, at);
    if chars.get(*at) == Some(&']') {
        *at += 1;
        return Some(AttributeSelector { name, test: None });
    }
    let operator = match chars.get(*at)? {
        '=' => AttributeOperator::Equals,
        '~' => AttributeOperator::Includes,
        '|' => AttributeOperator::Dash,
        '^' => AttributeOperator::Prefix,
        '$' => AttributeOperator::Suffix,
        '*' => AttributeOperator::Substring,
        _ => return None,
    };
    *at += if matches!(operator, AttributeOperator::Equals) { 1 } else { 2 };
    if !matches!(operator, AttributeOperator::Equals) && chars.get(*at - 1) != Some(&'=') {
        return None;
    }
    skip_space(chars, at);
    let value = match chars.get(*at)? {
        quote @ ('"' | '\'') => {
            let quote = *quote;
            *at += 1;
            let end = chars[*at..].iter().position(|c| *c == quote)? + *at;
            let value: String = chars[*at..end].iter().collect();
            *at = end + 1;
            value
        }
        _ => ident(chars, at)?,
    };
    skip_space(chars, at);
    let insensitive = matches!(chars.get(*at), Some('i' | 'I'));
    if insensitive {
        *at += 1;
        skip_space(chars, at);
    }
    if chars.get(*at) != Some(&']') {
        return None;
    }
    *at += 1;
    Some(AttributeSelector {
        name,
        test: Some((operator, value, insensitive)),
    })
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '-' || !c.is_ascii()
}

fn ident(chars: &[char], at: &mut usize) -> Option<String> {
    let start = *at;
    while let Some(&c) = chars.get(*at) {
        if c.is_alphanumeric() || c == '_' || c == '-' || !c.is_ascii() {
            *at += 1;
        } else if c == '\\' {
            // Escapes are rare in svg exports; not worth guessing at.
            return None;
        } else {
            break;
        }
    }
    (*at > start).then(|| chars[start..*at].iter().collect())
}

fn skip_space(chars: &[char], at: &mut usize) -> bool {
    let start = *at;
    while chars.get(*at).is_some_and(|c| c.is_whitespace()) {
        *at += 1;
    }
    *at > start
}

#[cfg(test)]
mod tests {
    use super::*;

    fn styled(source: &str) -> String {
        String::from_utf8(apply(source.as_bytes()).expect("changed")).unwrap()
    }

    #[test]
    fn a_document_without_rules_is_untouched() {
        assert!(apply(br#"<svg><rect fill="red"/></svg>"#).is_none());
        assert!(apply(br#"<svg><style>@keyframes k { to { opacity: 0 } }</style><rect/></svg>"#).is_none());
        assert!(apply(br#"<svg><style>.nothing { fill: red }</style><rect/></svg>"#).is_none());
    }

    #[test]
    fn class_rules_become_the_style_attribute_last() {
        assert_eq!(
            styled(r#"<svg><style>.a { fill: red; stroke: blue }</style><rect class="a" fill="green" x="1"/></svg>"#),
            r#"<svg><style>.a { fill: red; stroke: blue }</style><rect class="a" fill="green" x="1" style="fill:red;stroke:blue"/></svg>"#
        );
    }

    #[test]
    fn inline_style_beats_rules_and_important_beats_inline() {
        assert!(styled(r#"<svg><style>rect { fill: red }</style><rect style="fill: green"/></svg>"#)
            .contains(r#"style="fill:red;fill:green""#));
        assert!(styled(r#"<svg><style>rect { fill: red !important }</style><rect style="fill: green"/></svg>"#)
            .contains(r#"style="fill:green;fill:red""#));
    }

    #[test]
    fn specificity_then_source_order() {
        let out = styled(r#"<svg><style>#r { fill: red } rect.a { fill: blue } rect { fill: green } .a { fill: black }</style><rect id="r" class="a"/></svg>"#);
        assert!(out.contains(r#"style="fill:green;fill:black;fill:blue;fill:red""#), "{out}");
    }

    #[test]
    fn combinators_and_pseudo_classes() {
        let source = r#"<svg><style>g > rect { fill: red } g circle { stroke: blue } rect + circle { opacity: 0.5 } rect ~ path { fill: green } :root { color: red } path:last-child { stroke: black }</style><g><rect/><circle/><g><circle/></g><path/></g></svg>"#;
        let out = styled(source);
        assert!(out.contains(r#"<svg style="color:red">"#), "{out}");
        assert!(out.contains(r#"<rect style="fill:red"/>"#), "{out}");
        assert!(out.contains(r#"<circle style="stroke:blue;opacity:0.5"/>"#), "{out}");
        assert!(out.contains(r#"<g><circle style="stroke:blue"/></g>"#), "{out}");
        assert!(out.contains(r#"<path style="fill:green;stroke:black"/>"#), "{out}");
    }

    #[test]
    fn attribute_selectors() {
        let out = styled(r#"<svg><style>[data-x] { fill: red } [data-y="a b"] { stroke: blue } [data-z~=b] { opacity: 1 } [data-w^=pre] { color: red }</style><rect data-x="" data-y="a b" data-z="a b c" data-w="prefix"/></svg>"#);
        assert!(out.contains(r#"style="fill:red;stroke:blue;opacity:1;color:red""#), "{out}");
    }

    #[test]
    fn unsupported_selectors_drop_only_their_rule() {
        let out = styled(r#"<svg><style>rect:hover, rect::before, rect:not(.a) { fill: red } rect { stroke: blue }</style><rect/></svg>"#);
        assert!(out.contains(r#"<rect style="stroke:blue"/>"#), "{out}");
    }
}
