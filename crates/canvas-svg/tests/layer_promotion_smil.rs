//! The cached backdrop must survive animated writes that land inside the promoted layer.

use canvas_svg::SvgDocument;

/// A static background plus a moving dot, with the moving part inside `#moving`.
const SOURCE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
    <rect x="0" y="0" width="100" height="60" fill="navy"/>
    <g id="moving">
        <circle cx="20" cy="80" r="8" fill="orange">
            <animate attributeName="cx" values="20;80;20" dur="2s" repeatCount="indefinite"/>
        </circle>
    </g>
</svg>"##;

fn render(doc: &mut SvgDocument, size: i32) -> usize {
    let mut surface = skia_safe::surfaces::raster_n32_premul((size, size)).expect("surface");
    surface.canvas().clear(skia_safe::Color::TRANSPARENT);
    doc.render_frame(surface.canvas(), size, size, 1.0);
    let info = skia_safe::ImageInfo::new(
        (size, size),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );
    let mut pixels = vec![0u8; (size * size * 4) as usize];
    assert!(surface.read_pixels(&info, &mut pixels, (size * 4) as usize, (0, 0)));
    pixels.chunks(4).filter(|p| p[3] != 0).count()
}

#[test]
fn an_animation_inside_the_promoted_layer_keeps_the_backdrop() {
    let mut doc = SvgDocument::from_bytes(SOURCE.as_bytes()).expect("parse");
    doc.set_container_size(100.0, 100.0);
    doc.set_layer(Some("moving"));

    doc.set_current_time(0.0);
    render(&mut doc, 100);
    assert!(doc.has_backdrop(), "the first frame should have captured a backdrop");

    // Advancing the clock moves the dot, which lives inside `#moving`.
    for t in [0.2f64, 0.4, 0.6, 0.8] {
        doc.set_current_time(t);
        assert!(
            doc.has_backdrop(),
            "the backdrop was thrown away at t={t}s by a write inside the promoted layer"
        );
        render(&mut doc, 100);
    }
}

#[test]
fn an_animation_outside_the_promoted_layer_still_invalidates_it() {
    // Promote the moving group, but animate something that is *not* inside it.
    let source = SOURCE.replace(
        r#"<rect x="0" y="0" width="100" height="60" fill="navy"/>"#,
        r#"<rect id="bg" x="0" y="0" width="100" height="60" fill="navy">
             <animate attributeName="width" values="100;40;100" dur="2s" repeatCount="indefinite"/>
           </rect>"#,
    );
    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_container_size(100.0, 100.0);
    doc.set_layer(Some("moving"));

    doc.set_current_time(0.0);
    render(&mut doc, 100);
    assert!(doc.has_backdrop());

    doc.set_current_time(0.5);
    assert!(
        !doc.has_backdrop(),
        "a write outside the promoted layer must invalidate the cached backdrop"
    );
}

#[test]
fn promotion_does_not_change_the_picture() {
    let mut plain = SvgDocument::from_bytes(SOURCE.as_bytes()).expect("parse");
    plain.set_container_size(100.0, 100.0);
    let mut promoted = SvgDocument::from_bytes(SOURCE.as_bytes()).expect("parse");
    promoted.set_container_size(100.0, 100.0);
    promoted.set_layer(Some("moving"));

    for t in [0.0f64, 0.3, 0.7, 1.1] {
        plain.set_current_time(t);
        promoted.set_current_time(t);
        let (a, b) = (render(&mut plain, 100), render(&mut promoted, 100));
        assert_eq!(a, b, "promotion changed the rendering at t={t}s");
    }
}
