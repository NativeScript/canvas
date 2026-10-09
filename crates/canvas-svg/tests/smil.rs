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

/// The GitHub corner's octocat arm: a rotate with a centre point (`angle cx cy`) in
/// `values`, inside a viewBox scaled down to the view.
const OCTO_BODY: &str = "M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z";
const OCTO_ARM: &str = "M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2";

fn github_corner(animation: &str) -> String {
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="80" height="80" viewBox="0 0 250 250">
  <path fill="#64ceaa" d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"/>
  <path fill="#192d38" d="{OCTO_ARM}">{animation}</path>
  <path fill="#192d38" d="{OCTO_BODY}"/>
</svg>"##
    )
}

fn render_80(doc: &mut SvgDocument, time: f64) -> Vec<u8> {
    doc.set_container_size(80.0, 80.0);
    doc.set_current_time(time);
    let info = skia_safe::ImageInfo::new_n32_premul(skia_safe::ISize::new(80, 80), None);
    let mut surface = skia_safe::surfaces::raster(&info, None, None).expect("raster surface");
    surface.canvas().clear(skia_safe::Color::TRANSPARENT);
    doc.render_frame(surface.canvas(), 80, 80, 1.0);
    let mut pixels = vec![0u8; 80 * 80 * 4];
    assert!(surface.image_snapshot().read_pixels(
        &info,
        &mut pixels,
        80 * 4,
        skia_safe::IPoint::new(0, 0),
        skia_safe::image::CachingHint::Disallow,
    ));
    pixels
}

#[test]
fn a_rotate_with_a_centre_leaves_the_rest_of_the_drawing_alone() {
    let wave = r#"<animateTransform attributeName="transform" type="rotate" dur="0.56s" begin="0.4s" fill="freeze"
      values="0 130 106; -25 130 106; 10 130 106; -25 130 106; 10 130 106; 0 130 106"
      keyTimes="0; 0.2; 0.4; 0.6; 0.8; 1"/>"#;
    let still = render_80(&mut document(&github_corner("")), 0.0);
    let mut animated = document(&github_corner(wave));

    // Before `begin` nothing is written, and the frozen end is rotate(0 130 106): both
    // must draw exactly what the document without the animation draws.
    assert!(
        render_80(&mut animated, 0.0) == still,
        "frame before begin differs from the static drawing"
    );
    assert!(
        render_80(&mut animated, 1.2) == still,
        "frozen identity rotation differs from the static drawing"
    );
    // And in between, the arm does move: 0.4s + 0.2 * 0.56s is the -25° keyframe.
    assert!(
        render_80(&mut animated, 0.512) != still,
        "the arm should be rotated mid-wave"
    );
}

/// Asserts `id`'s transform is `rotate(angle cx cy)`, which Skia reads back as a matrix.
fn assert_rotation(doc: &mut SvgDocument, id: &str, angle: f32, cx: f32, cy: f32) {
    let transform = attribute(doc, id, "transform").expect("transform");
    let numbers: Vec<f32> = transform
        .trim_start_matches("matrix(")
        .trim_end_matches(')')
        .split(',')
        .map(|n| n.trim().parse().expect("matrix component"))
        .collect();
    let (sin, cos) = angle.to_radians().sin_cos();
    let expected = [
        cos,
        sin,
        -sin,
        cos,
        cx - cos * cx + sin * cy,
        cy - sin * cx - cos * cy,
    ];
    for (index, expected) in expected.iter().enumerate() {
        assert!(
            (numbers[index] - expected).abs() < 1e-3,
            "component {index} of {transform}: expected rotate({angle} {cx} {cy})"
        );
    }
}

fn rotating(animation: &str) -> SvgDocument {
    document(&format!(
        r#"<svg><rect id="arm" width="10" height="10">{animation}</rect></svg>"#
    ))
}

#[test]
fn a_to_rotation_about_a_centre_interpolates_from_zero_about_that_centre() {
    let mut doc = rotating(
        r#"<animateTransform attributeName="transform" type="rotate" to="90 50 50" dur="2s" fill="freeze"/>"#,
    );
    doc.set_current_time(0.5);
    assert_rotation(&mut doc, "arm", 22.5, 50.0, 50.0);
}

#[test]
fn a_by_rotation_about_a_centre_keeps_the_centre() {
    let mut doc = rotating(
        r#"<animateTransform attributeName="transform" type="rotate" by="90 50 50" dur="2s" fill="freeze"/>"#,
    );
    doc.set_current_time(0.5);
    assert_rotation(&mut doc, "arm", 22.5, 50.0, 50.0);
}

#[test]
fn from_plus_by_adds_the_angles_not_the_centres() {
    let mut doc = rotating(
        r#"<animateTransform attributeName="transform" type="rotate" from="0 50 50" by="90 50 50" dur="2s" fill="freeze"/>"#,
    );
    doc.set_current_time(1.0);
    assert_rotation(&mut doc, "arm", 45.0, 50.0, 50.0);
}

#[test]
fn accumulating_a_rotation_keeps_its_centre() {
    let mut doc = rotating(
        r#"<animateTransform attributeName="transform" type="rotate" from="0 50 50" to="90 50 50"
             dur="1s" repeatCount="2" accumulate="sum"/>"#,
    );
    // Half way through the second repeat: 45° on top of the first repeat's 90°.
    doc.set_current_time(1.5);
    assert_rotation(&mut doc, "arm", 135.0, 50.0, 50.0);
}

#[test]
fn a_bare_angle_in_rotate_values_takes_the_lists_centre() {
    let mut doc = rotating(
        r#"<animateTransform attributeName="transform" type="rotate" values="0; 90 50 50" dur="2s" fill="freeze"/>"#,
    );
    doc.set_current_time(1.0);
    assert_rotation(&mut doc, "arm", 45.0, 50.0, 50.0);
}
