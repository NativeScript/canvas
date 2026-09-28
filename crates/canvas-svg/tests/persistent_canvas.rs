//! GPU backends reuse one surface across frames, so `render_frame` must not leave its matrix
//! changes on the canvas.

use canvas_svg::SvgDocument;

const SOURCE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
    <rect x="10" y="10" width="50" height="50" fill="red"/>
</svg>"##;

fn opaque(surface: &mut skia_safe::Surface, size: i32) -> usize {
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
fn repeated_frames_on_one_canvas_are_stable() {
    let size = 300;
    let scale = 3.0;
    let mut doc = SvgDocument::from_bytes(SOURCE.as_bytes()).expect("parse");
    doc.set_container_size(size as f32 / scale, size as f32 / scale);

    let mut surface = skia_safe::surfaces::raster_n32_premul((size, size)).expect("surface");

    let mut counts = Vec::new();
    for _ in 0..6 {
        surface.canvas().clear(skia_safe::Color::TRANSPARENT);
        doc.render_frame(surface.canvas(), size, size, scale);
        counts.push(opaque(&mut surface, size));
    }

    println!("per-frame opaque pixels: {counts:?}");
    assert!(counts[0] > 0, "nothing drew on the first frame");
    assert!(
        counts.iter().all(|c| *c == counts[0]),
        "frames diverged on a reused canvas, the matrix is leaking between frames: {counts:?}"
    );
}
