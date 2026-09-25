//! SMIL end to end: extraction, Skia's parser, the clock and the attribute writes.

use canvas_svg::SvgDocument;

fn document(source: &str) -> SvgDocument {
    SvgDocument::from_bytes(source.as_bytes()).expect("well-formed test document")
}

fn attribute(document: &mut SvgDocument, id: &str, name: &str) -> Option<String> {
    let node = document.get_element_by_id(id)?;
    canvas_svg::get_attribute(&node.typed(), name)
}

#[test]
fn a_document_without_smil_reports_no_animation() {
    let mut doc = document(r#"<svg><rect id="a" width="10" height="10"/></svg>"#);
    assert!(!doc.has_animations());
    // Driving the clock on a static document is a no-op, not an error.
    assert!(!doc.set_current_time(1.0));
}

#[test]
fn the_animated_element_survives_extraction_and_is_still_addressable() {
    let mut doc = document(
        r#"<svg><rect id="box" width="10" height="10">
             <animate attributeName="width" from="10" to="20" dur="2s"/>
           </rect></svg>"#,
    );
    assert!(doc.has_animations());
    assert_eq!(doc.animation_count(), 1);
    assert!(doc.get_element_by_id("box").is_some());
}

#[test]
fn interpolates_an_attribute_across_the_duration() {
    let mut doc = document(
        r#"<svg><rect id="box" width="10" height="10">
             <animate attributeName="width" from="10" to="20" dur="2s" fill="freeze"/>
           </rect></svg>"#,
    );

    doc.set_current_time(0.0);
    assert_eq!(attribute(&mut doc, "box", "width").as_deref(), Some("10"));

    doc.set_current_time(1.0);
    assert_eq!(attribute(&mut doc, "box", "width").as_deref(), Some("15"));

    doc.set_current_time(2.0);
    assert_eq!(attribute(&mut doc, "box", "width").as_deref(), Some("20"));
}

#[test]
fn an_element_with_no_id_of_its_own_still_animates() {
    // The extractor has to invent an id here, and then find the element by it.
    let mut doc = document(
        r#"<svg><rect width="10" height="10">
             <animate attributeName="width" from="10" to="30" dur="2s" fill="freeze"/>
           </rect></svg>"#,
    );
    doc.set_current_time(1.0);
    assert_eq!(
        attribute(&mut doc, "__nsc_smil_0", "width").as_deref(),
        Some("20")
    );
}

#[test]
fn freeze_holds_and_remove_reverts() {
    let source = |fill: &str| {
        format!(
            r#"<svg><rect id="box" width="10" height="10">
                 <animate attributeName="width" from="10" to="20" dur="1s" fill="{fill}"/>
               </rect></svg>"#
        )
    };

    let mut frozen = document(&source("freeze"));
    frozen.set_current_time(5.0);
    assert_eq!(attribute(&mut frozen, "box", "width").as_deref(), Some("20"));

    let mut removed = document(&source("remove"));
    removed.set_current_time(0.5);
    assert_eq!(
        attribute(&mut removed, "box", "width").as_deref(),
        Some("15")
    );
    // Past the end the element goes back to the value it was authored with.
    removed.set_current_time(5.0);
    assert_eq!(
        attribute(&mut removed, "box", "width").as_deref(),
        Some("10")
    );
}

#[test]
fn a_to_animation_starts_from_the_elements_own_value() {
    let mut doc = document(
        r#"<svg><circle id="dot" cx="0" cy="0" r="4">
             <animate attributeName="r" to="14" dur="1s" fill="freeze"/>
           </circle></svg>"#,
    );
    doc.set_current_time(0.5);
    assert_eq!(attribute(&mut doc, "dot", "r").as_deref(), Some("9"));
}

#[test]
fn colors_blend_rather_than_step() {
    let mut doc = document(
        r#"<svg><rect id="box" width="10" height="10" fill="black">
             <animate attributeName="fill" from="black" to="white" dur="2s" fill="freeze"/>
           </rect></svg>"#,
    );
    doc.set_current_time(1.0);
    let fill = attribute(&mut doc, "box", "fill").expect("fill");
    // Skia normalises the colour; it only has to be the midpoint grey.
    assert!(
        fill.contains("128") || fill.eq_ignore_ascii_case("#808080"),
        "expected a mid grey, got {fill}"
    );
}

#[test]
fn transform_animation_composes_with_the_elements_own_transform() {
    let mut doc = document(
        r#"<svg><g id="knob" transform="translate(10,20)">
             <animateTransform attributeName="transform" type="rotate"
                               from="0" to="90" dur="2s" fill="freeze" additive="sum"/>
           </g></svg>"#,
    );
    doc.set_current_time(1.0);
    let transform = attribute(&mut doc, "knob", "transform").expect("transform");

    // Skia reads back `translate(10,20) rotate(45)` as a matrix; the authored 10,20 must
    // survive, which it wouldn't if the animation replaced the transform instead of adding.
    let numbers: Vec<f32> = transform
        .trim_start_matches("matrix(")
        .trim_end_matches(')')
        .split(',')
        .map(|n| n.trim().parse().expect("matrix component"))
        .collect();
    assert_eq!(numbers.len(), 6, "{transform}");

    let half = std::f32::consts::FRAC_1_SQRT_2; // cos(45°)
    for (index, expected) in [half, half, -half, half, 10.0, 20.0].iter().enumerate() {
        assert!(
            (numbers[index] - expected).abs() < 1e-4,
            "component {index} of {transform}: expected {expected}"
        );
    }
}

#[test]
fn several_animations_on_one_element_all_land() {
    let mut doc = document(
        r#"<svg><rect id="box" x="0" y="0" width="10" height="10">
             <animate attributeName="x" from="0" to="10" dur="1s" fill="freeze"/>
             <animate attributeName="y" from="0" to="20" dur="1s" fill="freeze"/>
           </rect></svg>"#,
    );
    doc.set_current_time(0.5);
    assert_eq!(attribute(&mut doc, "box", "x").as_deref(), Some("5"));
    assert_eq!(attribute(&mut doc, "box", "y").as_deref(), Some("10"));
}

#[test]
fn a_begin_offset_delays_the_start() {
    let mut doc = document(
        r#"<svg><rect id="box" width="10" height="10">
             <animate attributeName="width" from="10" to="20" begin="1s" dur="1s" fill="freeze"/>
           </rect></svg>"#,
    );
    doc.set_current_time(0.5);
    assert_eq!(attribute(&mut doc, "box", "width").as_deref(), Some("10"));
    doc.set_current_time(1.5);
    assert_eq!(attribute(&mut doc, "box", "width").as_deref(), Some("15"));
}

#[test]
fn set_applies_its_value_for_the_whole_active_duration() {
    let mut doc = document(
        r#"<svg><rect id="box" width="10" height="10" fill="red">
             <set attributeName="fill" to="blue" begin="1s" dur="1s"/>
           </rect></svg>"#,
    );
    doc.set_current_time(0.5);
    let before = attribute(&mut doc, "box", "fill").expect("fill");
    doc.set_current_time(1.5);
    let during = attribute(&mut doc, "box", "fill").expect("fill");
    assert_ne!(before, during, "the set should have taken effect");
}

#[test]
fn an_indefinite_repeat_never_reports_itself_finished() {
    let mut doc = document(
        r#"<svg><rect id="box" width="10" height="10">
             <animate attributeName="width" from="10" to="20" dur="1s" repeatCount="indefinite"/>
           </rect></svg>"#,
    );
    assert_eq!(doc.animation_duration(), None);
    assert!(doc.set_current_time(1_000.0), "should still be running");
}

#[test]
fn a_finite_animation_eventually_stops_asking_for_frames() {
    let mut doc = document(
        r#"<svg><rect id="box" width="10" height="10">
             <animate attributeName="width" from="10" to="20" dur="1s" fill="freeze"/>
           </rect></svg>"#,
    );
    assert_eq!(doc.animation_duration(), Some(1.0));
    assert!(doc.set_current_time(0.5));
    assert!(!doc.set_current_time(1.5));
}

#[test]
fn rendering_an_animated_document_at_successive_times_actually_changes_pixels() {
    let mut doc = document(
        r#"<svg width="40" height="40" viewBox="0 0 40 40">
             <rect id="box" x="0" y="0" width="40" height="40" fill="black">
               <animate attributeName="x" from="0" to="40" dur="2s" fill="freeze"/>
             </rect>
           </svg>"#,
    );
    doc.set_container_size(40.0, 40.0);

    let snapshot = |doc: &mut SvgDocument, time: f64| -> Vec<u8> {
        doc.set_current_time(time);
        let info = skia_safe::ImageInfo::new_n32_premul(skia_safe::ISize::new(40, 40), None);
        let mut surface = skia_safe::surfaces::raster(&info, None, None).expect("raster surface");
        surface.canvas().clear(skia_safe::Color::TRANSPARENT);
        doc.render_frame(surface.canvas(), 40, 40, 1.0);
        let image = surface.image_snapshot();
        let mut pixels = vec![0u8; 40 * 40 * 4];
        assert!(
            image.read_pixels(
                &info,
                &mut pixels,
                40 * 4,
                skia_safe::IPoint::new(0, 0),
                skia_safe::image::CachingHint::Disallow,
            ),
            "read_pixels"
        );
        pixels
    };

    let start = snapshot(&mut doc, 0.0);
    let middle = snapshot(&mut doc, 1.0);
    assert_ne!(start, middle, "the rect should have moved");
}

/// `rocket.svg`: exported animation built from `<set fill="freeze">` on path data plus a
/// discrete `<animate>` on `visibility`.
const ROCKET: &str = include_str!("../../../apps/demo/src/assets/file-assets/svg/rocket.svg");

#[test]
fn rocket_svg_carries_a_lot_of_animation() {
    let doc = document(ROCKET);
    assert!(doc.has_animations());
    assert!(
        doc.animation_count() > 50,
        "expected the exported animation to survive extraction, got {}",
        doc.animation_count()
    );
    // It repeats forever, so it has no end.
    assert_eq!(doc.animation_duration(), None);
}

/// Skia ignores the root group's `visibility="hidden"`, so it paints from the first frame.
#[test]
fn rocket_svg_paints_and_keeps_changing() {
    let mut doc = document(ROCKET);
    doc.set_container_size(300.0, 300.0);

    let info = skia_safe::ImageInfo::new_n32_premul(skia_safe::ISize::new(300, 300), None);
    let mut surface = skia_safe::surfaces::raster(&info, None, None).expect("raster surface");

    let painted = |doc: &mut SvgDocument, surface: &mut skia_safe::Surface, time: f64| -> usize {
        doc.set_current_time(time);
        surface.canvas().clear(skia_safe::Color::TRANSPARENT);
        doc.render_frame(surface.canvas(), 300, 300, 1.0);
        let mut pixels = vec![0u8; 300 * 300 * 4];
        assert!(surface.image_snapshot().read_pixels(
            &info,
            &mut pixels,
            300 * 4,
            skia_safe::IPoint::new(0, 0),
            skia_safe::image::CachingHint::Disallow,
        ));
        pixels.chunks_exact(4).filter(|p| p[3] != 0).count()
    };

    let counts: Vec<usize> = (0..12)
        .map(|step| painted(&mut doc, &mut surface, step as f64 * 0.25))
        .collect();

    assert!(
        counts.iter().all(|n| *n > 1000),
        "the document should paint at every moment, got {counts:?}"
    );
    assert!(
        counts.iter().any(|n| *n != counts[0]),
        "nothing moved across three seconds: {counts:?}"
    );
}

#[test]
fn rocket_svg_animates_without_falling_over() {
    let mut doc = document(ROCKET);
    doc.set_container_size(300.0, 300.0);

    let info = skia_safe::ImageInfo::new_n32_premul(skia_safe::ISize::new(300, 300), None);
    let mut surface = skia_safe::surfaces::raster(&info, None, None).expect("raster surface");

    // Two and a half seconds at 30fps, over the stretch where the exported `<set>`s fire.
    let mut frames = Vec::new();
    for step in 0..75 {
        let time = step as f64 / 30.0;
        doc.set_current_time(time);
        surface.canvas().clear(skia_safe::Color::TRANSPARENT);
        doc.render_frame(surface.canvas(), 300, 300, 1.0);

        let mut pixels = vec![0u8; 300 * 300 * 4];
        assert!(surface.image_snapshot().read_pixels(
            &info,
            &mut pixels,
            300 * 4,
            skia_safe::IPoint::new(0, 0),
            skia_safe::image::CachingHint::Disallow,
        ));
        frames.push(pixels);
    }

    // The frames must not all be identical.
    let distinct = frames
        .iter()
        .filter(|frame| *frame != &frames[0])
        .count();
    assert!(distinct > 0, "every frame was identical, nothing animated");
}
