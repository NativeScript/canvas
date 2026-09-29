//! `<style>` rules apply, in cascade order, though Skia's SVG module ignores them itself.

use canvas_svg::SvgDocument;

const SIZE: i32 = 10;

/// Straight RGBA of the centre pixel.
fn centre(body: &str) -> [u8; 4] {
    let source = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{SIZE}" height="{SIZE}">{body}</svg>"#
    );
    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
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

const RED: [u8; 4] = [255, 0, 0, 255];
const GREEN: [u8; 4] = [0, 128, 0, 255];
const BLUE: [u8; 4] = [0, 0, 255, 255];

const RECT: &str = r#"width="10" height="10""#;

#[test]
fn a_rule_beats_a_presentation_attribute() {
    assert_eq!(centre(&format!(r#"<style>.a {{ fill: blue }}</style><rect class="a" fill="red" {RECT}/>"#)), BLUE);
    // Whichever side of the element the attribute is on.
    assert_eq!(centre(&format!(r#"<rect class="a" {RECT} fill="red"/><style>.a {{ fill: blue }}</style>"#)), BLUE);
}

#[test]
fn inline_style_beats_a_rule_unless_important() {
    assert_eq!(centre(&format!(r#"<style>rect {{ fill: blue }}</style><rect style="fill: green" {RECT}/>"#)), GREEN);
    assert_eq!(centre(&format!(r#"<style>rect {{ fill: blue !important }}</style><rect style="fill: green" {RECT}/>"#)), BLUE);
}

#[test]
fn specificity_wins_over_order() {
    assert_eq!(centre(&format!(r#"<style>#r {{ fill: red }} .a {{ fill: blue }}</style><rect id="r" class="a" {RECT}/>"#)), RED);
}

#[test]
fn a_rule_on_a_group_is_inherited() {
    assert_eq!(centre(&format!(r#"<style>g.tint {{ fill: blue }}</style><g class="tint"><rect {RECT}/></g>"#)), BLUE);
}

#[test]
fn an_illustrator_style_export() {
    // Class rules in a CDATA block inside <defs>, the shape design tools write.
    let body = format!(
        r#"<defs><style><![CDATA[.cls-1{{fill:#00f;}}.cls-2{{fill:none;stroke:red;}}]]></style></defs><rect class="cls-1" {RECT}/>"#
    );
    assert_eq!(centre(&body), BLUE);
}

#[test]
fn rule_colours_go_through_the_colour_rewrite() {
    assert_eq!(centre(&format!(r#"<style>#r {{ fill: hsl(240, 100%, 50%) }}</style><rect id="r" {RECT}/>"#)), BLUE);
}

#[test]
fn keyframes_still_animate_a_styled_element() {
    let source = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{SIZE}" height="{SIZE}"><style>.a {{ fill: green }} #r {{ animation: k 1s linear forwards }} @keyframes k {{ from {{ fill: red }} to {{ fill: blue }} }}</style><rect id="r" class="a" {RECT}/></svg>"#
    );
    let mut doc = SvgDocument::from_bytes(source.as_bytes()).expect("parse");
    doc.set_container_size(SIZE as f32, SIZE as f32);
    doc.advance(2.0);
    let mut surface = skia_safe::surfaces::raster_n32_premul((SIZE, SIZE)).expect("surface");
    doc.render_frame(surface.canvas(), SIZE, SIZE, 1.0);
    let pixel = surface.image_snapshot().peek_pixels().map(|p| p.get_color((5, 5)));
    assert_eq!(pixel, Some(skia_safe::Color::BLUE));
}
