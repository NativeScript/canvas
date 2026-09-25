//! Layer promotion must match a plain full render.

const SRC: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
  <rect id="bg" x="0" y="0" width="100" height="100" fill="blue"/>
  <circle id="dot" cx="30" cy="30" r="10" fill="red"/>
</svg>"#;

fn render(doc: &mut canvas_svg::SvgDocument, w: i32, h: i32) -> Vec<u8> {
    let info = skia_safe::ImageInfo::new_n32_premul(skia_safe::ISize::new(w, h), None);
    let mut surface = skia_safe::surfaces::raster(&info, None, None).unwrap();
    let canvas = surface.canvas();
    canvas.clear(skia_safe::Color::TRANSPARENT);
    doc.render_frame(canvas, w, h, 1.0);
    let image = surface.image_snapshot();
    let mut pixels = vec![0u8; (w * h * 4) as usize];
    assert!(image.read_pixels(
        &info,
        &mut pixels,
        (w * 4) as usize,
        skia_safe::IPoint::new(0, 0),
        skia_safe::image::CachingHint::Allow
    ));
    pixels
}

#[test]
fn promoted_layer_matches_a_full_render() {
    let mut plain = canvas_svg::SvgDocument::from_bytes(SRC.as_bytes()).unwrap();
    plain.set_container_size(100.0, 100.0);
    let expected = render(&mut plain, 100, 100);

    let mut layered = canvas_svg::SvgDocument::from_bytes(SRC.as_bytes()).unwrap();
    layered.set_container_size(100.0, 100.0);
    layered.set_layer(Some("dot"));
    let first = render(&mut layered, 100, 100);
    // Second frame goes through the cached backdrop rather than a fresh capture.
    let cached = render(&mut layered, 100, 100);

    assert_eq!(expected, first, "first layered frame differs from a full render");
    assert_eq!(expected, cached, "cached layered frame differs from a full render");
}

#[test]
fn moving_the_promoted_node_still_composites_correctly() {
    let mut layered = canvas_svg::SvgDocument::from_bytes(SRC.as_bytes()).unwrap();
    layered.set_container_size(100.0, 100.0);
    layered.set_layer(Some("dot"));
    let _ = render(&mut layered, 100, 100);

    // Move the dot; the backdrop must stay valid and the dot must not ghost.
    let mut dot = layered.get_element_by_id("dot").unwrap().typed();
    canvas_svg::set_attribute(&mut dot, "cx", "70");
    let moved = render(&mut layered, 100, 100);

    let mut plain = canvas_svg::SvgDocument::from_bytes(SRC.as_bytes()).unwrap();
    plain.set_container_size(100.0, 100.0);
    let mut pdot = plain.get_element_by_id("dot").unwrap().typed();
    canvas_svg::set_attribute(&mut pdot, "cx", "70");
    let expected = render(&mut plain, 100, 100);

    assert_eq!(expected, moved, "moved layer does not match a full render");
}

#[test]
fn clearing_the_layer_returns_to_whole_document_rendering() {
    let mut doc = canvas_svg::SvgDocument::from_bytes(SRC.as_bytes()).unwrap();
    doc.set_container_size(100.0, 100.0);
    doc.set_layer(Some("dot"));
    let _ = render(&mut doc, 100, 100);
    doc.set_layer(None);
    assert!(doc.layer().is_none());

    let mut plain = canvas_svg::SvgDocument::from_bytes(SRC.as_bytes()).unwrap();
    plain.set_container_size(100.0, 100.0);
    assert_eq!(render(&mut plain, 100, 100), render(&mut doc, 100, 100));
}
