//! Views of one document share a recording per geometry.

use canvas_svg::SvgDocument;

const SOURCE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 120 120">
    <rect width="120" height="120" fill="#202040"/>
    <circle cx="30" cy="60" r="20" fill="orange">
        <animate attributeName="cx" values="30;90;30" dur="2s" repeatCount="indefinite"/>
    </circle>
</svg>"##;

fn shared() -> SvgDocument {
    let mut doc = SvgDocument::from_bytes(SOURCE.as_bytes()).expect("parse");
    doc.set_frame_sharing(true);
    doc
}

fn draw(doc: &mut SvgDocument, size: i32, scale: f32) -> Vec<u8> {
    let mut s = skia_safe::surfaces::raster_n32_premul((size, size)).expect("surface");
    s.canvas().clear(skia_safe::Color::TRANSPARENT);
    doc.draw(s.canvas(), size, size, scale);
    let info = skia_safe::ImageInfo::new(
        (size, size),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );
    let mut out = vec![0u8; (size * size * 4) as usize];
    assert!(s.read_pixels(&info, &mut out, (size * 4) as usize, (0, 0)));
    out
}

fn direct(doc: &mut SvgDocument, size: i32, scale: f32) -> Vec<u8> {
    doc.set_frame_sharing(false);
    let pixels = draw(doc, size, scale);
    doc.set_frame_sharing(true);
    pixels
}

#[test]
fn views_of_the_same_size_share_one_recording() {
    let mut doc = shared();
    doc.set_container_size(120.0, 120.0);
    doc.advance(0.5);

    let a = doc.frame(240, 240, 2.0).expect("record");
    let b = doc.frame(240, 240, 2.0).expect("cached");
    assert_eq!(
        a.picture_id(),
        b.picture_id(),
        "a second view of the same geometry recorded again instead of reusing the frame"
    );
}

#[test]
fn a_shared_frame_matches_rendering_directly() {
    let mut doc = shared();
    doc.set_container_size(120.0, 120.0);
    for step in 0..8 {
        let t = step as f64 / 4.0;
        doc.advance(t);
        let replayed = draw(&mut doc, 120, 1.0);
        assert_eq!(direct(&mut doc, 120, 1.0), replayed, "shared replay differed at t={t}s");
    }
}

#[test]
fn advancing_the_clock_invalidates_the_recording() {
    let mut doc = shared();
    doc.set_container_size(120.0, 120.0);
    doc.advance(0.0);
    let before = doc.frame(120, 120, 1.0).expect("record").picture_id();
    doc.advance(0.5);
    let after = doc.frame(120, 120, 1.0).expect("record").picture_id();
    assert_ne!(before, after, "the frame was reused after the clock moved");
}

#[test]
fn an_unchanged_tick_keeps_the_recording() {
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="120">
        <rect width="120" height="120" fill="red">
            <set attributeName="fill" to="blue" begin="1s" fill="freeze"/>
        </rect>
    </svg>"##;
    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_frame_sharing(true);
    doc.set_container_size(120.0, 120.0);
    doc.advance(0.1);
    let first = doc.frame(120, 120, 1.0).expect("record").picture_id();
    doc.advance(0.2);
    let second = doc.frame(120, 120, 1.0).expect("record").picture_id();
    assert_eq!(first, second, "a tick that changed nothing threw the recording away");
}

/// A `100%` root lays out differently per container size, so those must not share.
#[test]
fn views_of_different_sizes_get_their_own_layout() {
    let mut doc = shared();
    doc.advance(0.5);

    doc.set_container_size(120.0, 120.0);
    let big = draw(&mut doc, 120, 1.0);
    doc.set_container_size(60.0, 60.0);
    let small = draw(&mut doc, 60, 1.0);

    doc.set_container_size(120.0, 120.0);
    assert_eq!(big, draw(&mut doc, 120, 1.0), "the large view picked up the small view's frame");
    assert_eq!(big, direct(&mut doc, 120, 1.0));
    doc.set_container_size(60.0, 60.0);
    assert_eq!(small, direct(&mut doc, 60, 1.0));
}

/// A direct mutation is invisible to the document, so its owner has to invalidate.
#[test]
fn invalidating_after_a_mutation_records_again() {
    let mut doc = shared();
    doc.set_container_size(120.0, 120.0);
    let before = doc.frame(120, 120, 1.0).expect("record").picture_id();
    doc.invalidate_frames();
    let after = doc.frame(120, 120, 1.0).expect("record").picture_id();
    assert_ne!(before, after);
}

#[test]
fn sharing_off_never_caches() {
    let mut doc = SvgDocument::from_bytes(SOURCE.as_bytes()).expect("parse");
    doc.set_container_size(120.0, 120.0);
    let a = doc.frame(120, 120, 1.0).expect("record").picture_id();
    let b = doc.frame(120, 120, 1.0).expect("record").picture_id();
    assert_ne!(a, b, "an unshared document must record every frame");
}
