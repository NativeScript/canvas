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

/// CSS kept outside the SVG, as in a CodePen CSS panel, still animates it through
/// `add_stylesheet`. Reproduces codepen.io/shahbokhari/pen/oBbmXG.
#[test]
fn a_stylesheet_added_after_load_animates_the_element_it_names() {
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
        <rect id="dot" x="0" y="40" width="20" height="20" fill="red"/>
    </svg>"##;
    let css = r##"
        #dot { animation: slide 2s linear infinite }
        @keyframes slide {
            0% { transform: translate(0px,0px) }
            100% { transform: translate(60px,0px) }
        }
    "##;

    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_container_size(100.0, 100.0);
    assert!(!doc.has_animations(), "nothing to animate before the stylesheet is added");

    assert!(doc.add_stylesheet(css), "the added animation should be running");
    assert_eq!(doc.animation_count(), 1);

    doc.advance(0.0);
    let start = frame_hash(&mut doc, 100, 100);
    doc.advance(1.0);
    let middle = frame_hash(&mut doc, 100, 100);
    assert_ne!(start, middle, "the separately-supplied stylesheet never reached the element");
}

/// Ids are resolved per frame, not at extraction, so an unknown id is harmless.
#[test]
fn a_stylesheet_with_no_matching_id_resolves_to_nothing_safely() {
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect width="10" height="10"/></svg>"##;
    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_container_size(10.0, 10.0);
    doc.add_stylesheet(
        "#missing { animation: spin 1s linear infinite } \
         @keyframes spin { from { transform: rotate(0deg) } to { transform: rotate(360deg) } }",
    );

    doc.advance(0.0);
    let start = frame_hash(&mut doc, 10, 10);
    doc.advance(0.5);
    let later = frame_hash(&mut doc, 10, 10);
    assert_eq!(start, later, "an animation targeting a nonexistent id should not change the frame");
}

fn pixel(doc: &mut SvgDocument, width: i32, height: i32, x: i32, y: i32) -> [u8; 4] {
    let mut surface = skia_safe::surfaces::raster_n32_premul((width, height)).expect("surface");
    surface.canvas().clear(skia_safe::Color::TRANSPARENT);
    doc.render_frame(surface.canvas(), width, height, 1.0);
    let info = skia_safe::ImageInfo::new(
        (1, 1),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );
    let mut out = [0u8; 4];
    assert!(surface.read_pixels(&info, &mut out, 4, (x, y)));
    out
}

/// The shape of codepen.io/shahbokhari/pen/oBbmXG: the animated element is a shimmer inside a
/// `<mask>`, and only shows through the shapes that use the mask.
#[test]
fn an_animated_element_inside_a_mask_redraws_what_uses_it() {
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="300" height="70" viewBox="0 0 300 70">
        <style>
            #mask { animation: mask 975ms ease infinite }
            @keyframes mask { from { transform: translateX(0) } to { transform: translateX(280px) } }
        </style>
        <defs>
            <mask id="mask-element">
                <path fill="#777" d="M0 0h300v70H0z"/>
                <path fill="hsla(200,0%,10%,.6)" id="mask" d="M0 0h20v70H0z"/>
            </mask>
        </defs>
        <path mask="url(#mask-element)" fill="#dadada" d="M10 10h280v50H10z"/>
    </svg>"##;

    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_container_size(300.0, 70.0);
    assert_eq!(doc.animation_count(), 1);
    doc.advance(0.0);
    let start = frame_hash(&mut doc, 300, 70);
    doc.advance(0.5);
    let middle = frame_hash(&mut doc, 300, 70);
    assert_ne!(start, middle, "the shimmer inside the mask never moved");
}

/// Rules are matched by the stylesheet cascade, so any selector it supports can start an
/// animation, not only `#id`.
#[test]
fn a_class_rule_animates_every_element_it_matches() {
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
        <style>
            .dot { animation: slide 2s linear infinite }
            @keyframes slide { from { transform: translate(0px,0px) } to { transform: translate(60px,0px) } }
        </style>
        <rect class="dot" x="0" y="10" width="20" height="20" fill="red"/>
        <rect class="dot" x="0" y="60" width="20" height="20" fill="blue"/>
    </svg>"##;

    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_container_size(100.0, 100.0);
    assert_eq!(doc.animation_count(), 2, "one animation per matched element");
    doc.advance(0.0);
    let start = frame_hash(&mut doc, 100, 100);
    doc.advance(1.0);
    assert_ne!(start, frame_hash(&mut doc, 100, 100));
}

#[test]
fn an_inline_animation_uses_the_stylesheets_keyframes() {
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
        <style>@keyframes fade { from { opacity: 1 } to { opacity: 0 } }</style>
        <rect style="animation: fade 1s linear forwards" width="100" height="100" fill="red"/>
    </svg>"##;

    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_container_size(100.0, 100.0);
    assert_eq!(doc.animation_count(), 1);
    doc.advance(0.0);
    assert!(pixel(&mut doc, 100, 100, 50, 50)[3] > 200);
    doc.advance(0.9);
    assert!(pixel(&mut doc, 100, 100, 50, 50)[3] < 60, "never faded");
}

#[test]
fn a_rule_and_the_id_it_matches_animate_once() {
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10">
        <style>
            #a { animation: fade 1s } rect { animation: fade 1s }
            @keyframes fade { from { opacity: 1 } to { opacity: 0 } }
        </style>
        <rect id="a" width="10" height="10"/>
    </svg>"##;
    let doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    assert_eq!(doc.animation_count(), 1, "the cascade's winner, not one per rule");
}

#[test]
fn translate_y_moves_down_not_across() {
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
        <style>
            #box { animation: drop 1s linear forwards }
            @keyframes drop { from { transform: translateY(0) } to { transform: translateY(50px) } }
        </style>
        <rect id="box" width="10" height="10" fill="red"/>
    </svg>"##;

    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_container_size(100.0, 100.0);
    doc.advance(0.99);
    assert!(pixel(&mut doc, 100, 100, 5, 55)[3] > 200, "the box should have dropped to y~50");
    assert_eq!(pixel(&mut doc, 100, 100, 55, 5)[3], 0, "translateY moved it along x");
}
