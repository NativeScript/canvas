//! `advance` must report unchanged frames so the loop can skip redrawing them.

use canvas_svg::SvgDocument;

#[test]
fn a_discrete_animation_reports_change_only_at_its_steps() {
    // Three steps over 3s: the value changes at 0s, 1s and 2s and holds in between.
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
        <rect x="10" y="10" width="20" height="20">
            <animate attributeName="fill" values="red;green;blue" calcMode="discrete"
                     dur="3s" repeatCount="indefinite"/>
        </rect>
    </svg>"##;
    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");

    // Walk a whole cycle in 100ms steps and count how many frames actually changed.
    let mut changed = 0;
    let mut total = 0;
    for step in 0..30 {
        let applied = doc.advance(step as f64 * 0.1);
        total += 1;
        if applied.changed {
            changed += 1;
        }
        assert!(applied.running, "an indefinite repeat never stops");
    }
    println!("{changed} of {total} frames changed");
    assert!(changed >= 3, "each discrete step should register as a change");
    assert!(
        changed < total / 2,
        "a discrete animation should hold its value for most frames, not redraw every one"
    );
}

#[test]
fn a_frozen_animation_stops_reporting_change() {
    let source = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
        <rect x="10" y="10" width="20" height="20" fill="red">
            <set attributeName="fill" to="blue" begin="0.5s" fill="freeze"/>
        </rect>
    </svg>"##;
    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");

    doc.advance(0.0);
    assert!(doc.advance(0.6).changed, "the set should land");
    // Once frozen the value never moves again, so no frame after it needs redrawing.
    for step in 0..10 {
        let applied = doc.advance(0.7 + step as f64 * 0.1);
        assert!(!applied.changed, "a frozen value must not report a change");
    }
}
