//! CSS colour functions Skia's own parser rejects (`hsl()`, `hsla()`, `hwb()`, `/` alpha) draw
//! as those colours, not black, wherever a colour can be written.

use canvas_svg::SvgDocument;

const SIZE: i32 = 10;

/// Straight RGBA of the centre pixel.
fn centre(doc: &mut SvgDocument) -> [u8; 4] {
    doc.set_container_size(SIZE as f32, SIZE as f32);
    let mut surface = skia_safe::surfaces::raster_n32_premul((SIZE, SIZE)).expect("surface");
    surface.canvas().clear(skia_safe::Color::TRANSPARENT);
    doc.render_frame(surface.canvas(), SIZE, SIZE, 1.0);
    let info = skia_safe::ImageInfo::new(
        (SIZE, SIZE),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Unpremul,
        None,
    );
    let mut pixels = vec![0u8; (SIZE * SIZE * 4) as usize];
    assert!(surface.read_pixels(&info, &mut pixels, (SIZE * 4) as usize, (0, 0)));
    let at = ((SIZE / 2 * SIZE + SIZE / 2) * 4) as usize;
    [pixels[at], pixels[at + 1], pixels[at + 2], pixels[at + 3]]
}

fn doc(body: &str) -> SvgDocument {
    let source = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{SIZE}" height="{SIZE}">{body}</svg>"#
    );
    SvgDocument::from_bytes(source.as_bytes()).expect("parse")
}

fn close(actual: [u8; 4], expected: [u8; 4]) -> bool {
    actual
        .iter()
        .zip(expected)
        .all(|(a, e)| (*a as i32 - e as i32).abs() <= 2)
}

#[track_caller]
fn assert_colour(actual: [u8; 4], expected: [u8; 4]) {
    assert!(close(actual, expected), "got {actual:?}, expected {expected:?}");
}

#[test]
fn fill_and_style() {
    assert_colour(
        centre(&mut doc(r#"<rect width="10" height="10" fill="hsl(0, 100%, 50%)"/>"#)),
        [255, 0, 0, 255],
    );
    assert_colour(
        centre(&mut doc(r#"<rect width="10" height="10" style="fill: hwb(240 0% 0%)"/>"#)),
        [0, 0, 255, 255],
    );
    assert_colour(
        centre(&mut doc(r#"<rect width="10" height="10" fill="rgb(0 255 0 / 50%)"/>"#)),
        [0, 255, 0, 128],
    );
}

#[test]
fn gradient_stops() {
    // The shape that drew black: a paint server whose stops are hsl().
    let mut doc = doc(r#"
        <defs><linearGradient id="g">
            <stop offset="0" stop-color="hsl(120, 100%, 25%)"/>
            <stop offset="1" stop-color="hsla(120, 100%, 25%, 1)"/>
        </linearGradient></defs>
        <rect width="10" height="10" fill="url(#g)"/>"#);
    assert_colour(centre(&mut doc), [0, 128, 0, 255]);
}

const KEYFRAMES: &str = "#r { animation: k 1s linear forwards } @keyframes k { from { fill: hsl(0, 100%, 50%) } to { fill: hsl(240 100% 50%) } }";

// Skia applies no `<style>` rules itself; what reads them is the `@keyframes` support.
#[test]
fn keyframes_in_the_document() {
    let mut doc = doc(&format!(r#"<style>{KEYFRAMES}</style><rect id="r" width="10" height="10" fill="black"/>"#));
    doc.advance(0.0);
    assert_colour(centre(&mut doc), [255, 0, 0, 255]);
    doc.advance(2.0);
    assert_colour(centre(&mut doc), [0, 0, 255, 255]);
}

#[test]
fn keyframes_from_add_stylesheet() {
    let mut doc = doc(r#"<rect id="r" width="10" height="10" fill="black"/>"#);
    doc.add_stylesheet(KEYFRAMES);
    doc.advance(0.0);
    assert_colour(centre(&mut doc), [255, 0, 0, 255]);
    doc.advance(2.0);
    assert_colour(centre(&mut doc), [0, 0, 255, 255]);
}

#[test]
fn set_attribute_at_runtime() {
    let mut doc = doc(r#"<rect id="r" width="10" height="10" fill="red"/>"#);
    let mut rect = doc.get_element_by_id("r").expect("r").typed();
    canvas_svg::set_attribute(&mut rect, "fill", "hsl(240 100% 50%)");
    assert_colour(centre(&mut doc), [0, 0, 255, 255]);
}

#[test]
fn smil_animation_values() {
    let mut doc = doc(r#"
        <rect width="10" height="10" fill="black">
            <animate attributeName="fill" from="hsl(0, 100%, 50%)" to="hsl(240, 100%, 50%)"
                     dur="1s" fill="freeze"/>
        </rect>"#);
    doc.advance(0.0);
    assert_colour(centre(&mut doc), [255, 0, 0, 255]);
    // Interpolated, not a discrete jump: both channels are partway.
    doc.advance(0.5);
    let [r, _, b, _] = centre(&mut doc);
    assert!(r > 64 && r < 192 && b > 64 && b < 192, "midpoint {r} / {b}");
    doc.advance(2.0);
    assert_colour(centre(&mut doc), [0, 0, 255, 255]);
}
