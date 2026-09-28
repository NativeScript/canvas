//! Text nodes. Skia can't remove a text element's children, so removal hides the fragment.

use canvas_svg::{create_text_node, set_text, text, SvgDocument, SvgElementHandle};

const W: i32 = 240;
const H: i32 = 60;

fn doc_with_empty_text() -> (SvgDocument, SvgElementHandle) {
    let source = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}"><text id="t" x="5" y="40" font-size="24" fill="black"></text></svg>"#
    );
    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_container_size(W as f32, H as f32);
    let node = doc.get_element_by_id("t").expect("text");
    (doc, SvgElementHandle::new(node.typed()))
}

fn doc_with(body: &str) -> SvgDocument {
    let source = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}"><text id="t" x="5" y="40" font-size="24" fill="black">{body}</text></svg>"#
    );
    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_container_size(W as f32, H as f32);
    doc
}

fn literal(value: &str) -> SvgElementHandle {
    SvgElementHandle::new(create_text_node(value))
}

fn pixels(doc: &mut SvgDocument) -> Vec<u8> {
    let mut surface = skia_safe::surfaces::raster_n32_premul((W, H)).expect("surface");
    surface.canvas().clear(skia_safe::Color::WHITE);
    doc.render_frame(surface.canvas(), W, H, 1.0);
    let info = skia_safe::ImageInfo::new(
        (W, H),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );
    let mut out = vec![0u8; (W * H * 4) as usize];
    assert!(surface.read_pixels(&info, &mut out, (W * 4) as usize, (0, 0)));
    out
}

#[test]
fn a_text_node_reads_and_rewrites_its_text() {
    let mut node = create_text_node("before");
    assert_eq!(text(&node).as_deref(), Some("before"));
    assert!(set_text(&mut node, "after"));
    assert_eq!(text(&node).as_deref(), Some("after"));
}

#[test]
fn only_text_nodes_have_text() {
    let mut rect = canvas_svg::create_element("rect").unwrap();
    assert_eq!(text(&rect), None);
    assert!(!set_text(&mut rect, "x"));
}

/// The demo's `label.textContent = ...`: rewriting the text node must reach the screen.
#[test]
fn rewriting_a_text_node_renders_the_new_text() {
    let (mut doc, mut label) = doc_with_empty_text();
    let mut node = literal("before mutation");
    label.append_child(&node).unwrap();
    let before = pixels(&mut doc);

    assert!(set_text(&mut node.node, "after mutation"));
    let after = pixels(&mut doc);

    assert_ne!(before, after, "the text did not change on screen");
    assert_eq!(after, pixels(&mut doc_with("after mutation")));
}

/// A removed fragment must render as if it had never been there.
#[test]
fn removing_from_a_text_element_renders_as_if_it_were_gone() {
    let (mut doc, mut label) = doc_with_empty_text();
    label.append_child(&literal("keep ")).unwrap();
    label.append_child(&literal("drop")).unwrap();

    let removed = label.remove_child(1).expect("remove");
    assert_eq!(text(&removed).as_deref(), Some("drop"));
    assert_eq!(label.children.len(), 1);
    assert_eq!(pixels(&mut doc), pixels(&mut doc_with("keep ")));
}

/// What `textContent = ...` does from JS: remove every child, then append one text node.
#[test]
fn replacing_every_child_renders_only_the_new_text() {
    let (mut doc, mut label) = doc_with_empty_text();
    label.append_child(&literal("before ")).unwrap();
    let mut span = SvgElementHandle::new(canvas_svg::create_element("tspan").unwrap());
    span.append_child(&literal("mutation")).unwrap();
    label.append_child(&span).unwrap();

    while !label.children.is_empty() {
        label.remove_child(label.children.len() - 1).unwrap();
    }
    label.append_child(&literal("after mutation")).unwrap();

    assert_eq!(pixels(&mut doc), pixels(&mut doc_with("after mutation")));
}

/// Plain containers still remove for real.
#[test]
fn removing_from_a_group_still_detaches() {
    let mut group = SvgElementHandle::new(canvas_svg::create_element("g").unwrap());
    group
        .append_child(&SvgElementHandle::new(canvas_svg::create_element("rect").unwrap()))
        .unwrap();
    group.remove_child(0).expect("remove");
    assert!(group.children.is_empty());
}
