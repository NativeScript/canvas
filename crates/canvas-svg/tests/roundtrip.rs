//! Reading back what `set_attribute` wrote through Skia's own attribute parser.

fn set_get(tag: &str, name: &str, value: &str) -> Option<String> {
    let mut node = canvas_svg::create_element(tag).expect("element");
    assert!(canvas_svg::set_attribute(&mut node, name, value), "{tag}/{name} not handled");
    canvas_svg::get_attribute(&node, name)
}

#[test]
fn lengths() {
    assert_eq!(set_get("rect", "width", "120").as_deref(), Some("120"));
    assert_eq!(set_get("circle", "r", "60").as_deref(), Some("60"));
    assert_eq!(set_get("svg", "height", "300").as_deref(), Some("300"));
}

#[test]
fn polygon_points() {
    assert_eq!(
        set_get("polygon", "points", "75,120 100,145 50,145").as_deref(),
        Some("75,120 100,145 50,145")
    );
}

#[test]
fn href_accepts_both_spellings() {
    // Skia only knows `xlink:href`; `set_attribute` maps the modern spelling onto it.
    assert_eq!(set_get("use", "href", "#shared").as_deref(), Some("#shared"));
    assert_eq!(set_get("use", "xlink:href", "#shared").as_deref(), Some("#shared"));
}

#[test]
fn attributes_the_old_hand_rolled_dispatch_did_not_reach() {
    let mut node = canvas_svg::create_element("g").unwrap();
    assert!(canvas_svg::set_attribute(&mut node, "transform", "translate(80,10)"));

    let mut svg = canvas_svg::create_element("svg").unwrap();
    assert!(canvas_svg::set_attribute(&mut svg, "viewBox", "0 0 100 100"));

    let mut rect = canvas_svg::create_element("rect").unwrap();
    assert!(canvas_svg::set_attribute(&mut rect, "style", "fill:lime;stroke:purple"));

    let mut blur = canvas_svg::create_element("filter").unwrap();
    assert!(canvas_svg::set_attribute(&mut blur, "filterUnits", "userSpaceOnUse"));
}

#[test]
fn use_resolves_a_programmatically_registered_id() {
    let mut doc = canvas_svg::SvgDocument::new();

    let mut circle = canvas_svg::create_element("circle").unwrap();
    for (k, v) in [("cx", "20"), ("cy", "20"), ("r", "10"), ("fill", "orange")] {
        canvas_svg::set_attribute(&mut circle, k, v);
    }
    doc.register_id("shared", circle.clone().into_node());

    assert!(doc.get_element_by_id("shared").is_some());
    assert!(doc.get_element_by_id("nope").is_none());

    doc.unregister_id("shared");
    assert!(doc.get_element_by_id("shared").is_none());
}

#[test]
fn svg_view_box() {
    assert_eq!(set_get("svg", "viewBox", "0 0 300 300").as_deref(), Some("0 0 300 300"));
}
