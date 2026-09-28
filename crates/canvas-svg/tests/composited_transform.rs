//! A promoted subtree that only moves is composited from its cached raster. Subpixel offsets
//! resample edges, so those are checked against a tolerance rather than for equality.

use canvas_svg::SvgDocument;

/// A static background plus an expensive masked group that only translates.
const SOURCE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
    <defs>
        <mask id="m" maskUnits="userSpaceOnUse" x="0" y="0" width="200" height="200">
            <circle cx="40" cy="40" r="35" fill="white"/>
        </mask>
    </defs>
    <rect x="0" y="0" width="200" height="200" fill="#202040"/>
    <g id="mover" mask="url(#m)">
        <animateTransform attributeName="transform" type="translate" dur="2s"
                          values="0 0; 60 40; 0 0" repeatCount="indefinite"/>
        <rect x="10" y="10" width="120" height="120" fill="orange"/>
    </g>
</svg>"##;

fn pixels(doc: &mut SvgDocument, size: i32) -> Vec<u8> {
    let mut surface = skia_safe::surfaces::raster_n32_premul((size, size)).expect("surface");
    surface.canvas().clear(skia_safe::Color::TRANSPARENT);
    doc.render_frame(surface.canvas(), size, size, 1.0);
    let info = skia_safe::ImageInfo::new(
        (size, size),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );
    let mut out = vec![0u8; (size * size * 4) as usize];
    assert!(surface.read_pixels(&info, &mut out, (size * 4) as usize, (0, 0)));
    out
}

fn load(layer: Option<&str>) -> SvgDocument {
    let mut doc = SvgDocument::from_bytes(SOURCE.as_bytes()).expect("parse");
    doc.set_container_size(200.0, 200.0);
    doc.set_layer(layer);
    doc
}

/// How far the two renderings are apart, as a fraction of all bytes.
fn divergence(a: &[u8], b: &[u8]) -> f64 {
    a.iter().zip(b).filter(|(x, y)| x != y).count() as f64 / a.len() as f64
}

#[test]
fn a_whole_pixel_move_is_bit_identical() {
    let (mut plain, mut promoted) = (load(None), load(Some("mover")));
    // Each t lands on a whole-pixel translation, e.g. 0.5s is translate(30,20).
    for t in [0.0f64, 0.5, 1.0, 1.5] {
        plain.advance(t);
        promoted.advance(t);
        assert_eq!(
            pixels(&mut plain, 200),
            pixels(&mut promoted, 200),
            "a whole-pixel move should composite exactly, at t={t}s"
        );
    }
}

#[test]
fn a_subpixel_move_only_resamples_edges() {
    let (mut plain, mut promoted) = (load(None), load(Some("mover")));
    for step in 0..24 {
        let t = step as f64 / 12.0;
        plain.advance(t);
        promoted.advance(t);
        let d = divergence(&pixels(&mut plain, 200), &pixels(&mut promoted, 200));
        assert!(
            d < 0.01,
            "compositing diverged by {:.2}% at t={t}s, more than edge resampling",
            d * 100.0
        );
    }
}

#[test]
fn changing_the_subtree_itself_re_renders() {
    let source = SOURCE.replace(
        r#"<rect x="10" y="10" width="120" height="120" fill="orange"/>"#,
        r#"<rect x="10" y="10" width="120" height="120" fill="orange">
             <animate attributeName="fill" values="orange;red;orange" dur="2s" repeatCount="indefinite"/>
           </rect>"#,
    );
    let mut plain = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    plain.set_container_size(200.0, 200.0);
    let mut promoted = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    promoted.set_container_size(200.0, 200.0);
    promoted.set_layer(Some("mover"));

    for step in 0..12 {
        let t = step as f64 / 6.0;
        plain.advance(t);
        promoted.advance(t);
        let d = divergence(&pixels(&mut plain, 200), &pixels(&mut promoted, 200));
        assert!(
            d < 0.01,
            "a colour change inside the layer was served from a stale raster at t={t}s ({:.1}% apart)",
            d * 100.0
        );
    }
}

#[test]
fn a_scaling_layer_falls_back_to_rendering() {
    let source = SOURCE.replace(r#"type="translate""#, r#"type="scale""#)
        .replace(r#"values="0 0; 60 40; 0 0""#, r#"values="1; 1.5; 1""#);
    let mut plain = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    plain.set_container_size(200.0, 200.0);
    let mut promoted = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    promoted.set_container_size(200.0, 200.0);
    promoted.set_layer(Some("mover"));

    for step in 0..12 {
        let t = step as f64 / 6.0;
        plain.advance(t);
        promoted.advance(t);
        assert_eq!(
            pixels(&mut plain, 200),
            pixels(&mut promoted, 200),
            "a scaled layer must fall back to rendering, not be resampled, at t={t}s"
        );
    }
}
