//! Documents animated with CSS `@keyframes` rather than SMIL.

use canvas_svg::SvgDocument;

/// A cheap fingerprint of what was drawn, so two frames can be told apart.
fn frame_hash(doc: &mut SvgDocument, width: i32, height: i32) -> u64 {
    let mut surface = skia_safe::surfaces::raster_n32_premul((width, height)).expect("surface");
    surface.canvas().clear(skia_safe::Color::TRANSPARENT);
    doc.render_frame(surface.canvas(), width, height, 1.0);
    let info = skia_safe::ImageInfo::new(
        (width, height),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );
    let mut pixels = vec![0u8; (width * height * 4) as usize];
    assert!(surface.read_pixels(&info, &mut pixels, (width * 4) as usize, (0, 0)));
    pixels.iter().enumerate().fold(0u64, |acc, (index, byte)| {
        acc.wrapping_mul(31).wrapping_add((*byte as u64) ^ (index as u64 & 0xff))
    })
}

#[test]
fn the_solar_system_export_animates() {
    let source = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../apps/demo/src/assets/file-assets/svg/solar-system-animation.svg"
    ))
    .expect("solar-system-animation.svg");

    // It is a CSS-animated export: no SMIL elements anywhere in it.
    let text = String::from_utf8_lossy(&source);
    assert!(!text.contains("<animateTransform"), "fixture is supposed to be CSS-driven");
    assert!(text.contains("@keyframes"));

    let mut doc = SvgDocument::from_bytes(&source).expect("parse");
    doc.set_container_size(700.0, 400.0);
    assert!(doc.has_animations(), "the stylesheet's animations were not picked up");
    println!("extracted {} animations", doc.animation_count());

    let mut hashes = Vec::new();
    for t in [0.0f64, 1.0, 2.5, 5.0] {
        doc.advance(t);
        hashes.push(frame_hash(&mut doc, 700, 400));
    }
    assert!(
        hashes.windows(2).all(|w| w[0] != w[1]),
        "every sampled frame should differ; the document is still frozen"
    );
}

#[test]
fn a_css_animation_moves_the_element_it_names() {
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
        <style>
            #dot { animation: slide 2s linear infinite }
            @keyframes slide {
                0% { transform: translate(0px,0px) }
                100% { transform: translate(60px,0px) }
            }
        </style>
        <rect id="dot" x="0" y="40" width="20" height="20" fill="red"/>
    </svg>"##;

    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_container_size(100.0, 100.0);
    assert_eq!(doc.animation_count(), 1);

    // Halfway through, the rect should have moved ~30 units right.
    doc.advance(0.0);
    let start = frame_hash(&mut doc, 100, 100);
    doc.advance(1.0);
    let middle = frame_hash(&mut doc, 100, 100);
    assert_ne!(start, middle, "the transform never reached the element");
}

#[test]
fn a_document_with_no_stylesheet_is_unaffected() {
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect width="10" height="10"/></svg>"##;
    let doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    assert!(!doc.has_animations());
}
