//! Expensive static content plus a small element that animates every frame.

use std::time::Instant;

const SRC: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="300" height="300" viewBox="0 0 300 300">
  <defs>
    <filter id="f"><feColorMatrix type="saturate" values="0.3"/></filter>
  </defs>
  <g filter="url(#f)">
    <rect x="0" y="0" width="300" height="300" fill="teal"/>
    <circle cx="80" cy="80" r="60" fill="orange"/>
    <rect x="150" y="150" width="120" height="120" fill="purple"/>
  </g>
  <circle id="ball" cx="40" cy="260" r="14" fill="red"/>
</svg>"##;

struct Target { surface: skia_safe::Surface, info: skia_safe::ImageInfo }
impl Target {
    fn new(w: i32, h: i32) -> Self {
        let info = skia_safe::ImageInfo::new_n32_premul(skia_safe::ISize::new(w, h), None);
        Self { surface: skia_safe::surfaces::raster(&info, None, None).unwrap(), info }
    }
    fn draw(&mut self, doc: &mut canvas_svg::SvgDocument, scale: f32) {
        let (w, h) = (self.info.width(), self.info.height());
        let canvas = self.surface.canvas();
        canvas.clear(skia_safe::Color::TRANSPARENT);
        let r = canvas.save();
        doc.render_frame(canvas, w, h, scale);
        canvas.restore_to_count(r);
    }
    fn pixels(&mut self) -> Vec<u8> {
        let (w, h) = (self.info.width(), self.info.height());
        let image = self.surface.image_snapshot();
        let mut out = vec![0u8; (w * h * 4) as usize];
        assert!(image.read_pixels(&self.info, &mut out, (w * 4) as usize, skia_safe::IPoint::new(0, 0), skia_safe::image::CachingHint::Allow));
        out
    }
}

fn bench(label: &str, doc: &mut canvas_svg::SvgDocument, t: &mut Target, scale: f32) -> Vec<u8> {
    t.draw(doc, scale);
    let runs = 20;
    let start = Instant::now();
    for i in 0..runs {
        let mut ball = doc.get_element_by_id("ball").unwrap().typed();
        canvas_svg::set_attribute(&mut ball, "cx", &format!("{}", 40 + i * 5));
        t.draw(doc, scale);
    }
    println!("{label}: {:.2} ms/frame", start.elapsed().as_secs_f64() * 1000.0 / runs as f64);
    t.pixels()
}

#[test]
fn promoting_the_animated_element() {
    let (w, h, scale) = (900, 900, 3.0);
    let mut t = Target::new(w, h);

    let mut plain = canvas_svg::SvgDocument::from_bytes(SRC.as_bytes()).unwrap();
    plain.set_container_size(300.0, 300.0);
    let full = bench("whole document", &mut plain, &mut t, scale);

    let mut layered = canvas_svg::SvgDocument::from_bytes(SRC.as_bytes()).unwrap();
    layered.set_container_size(300.0, 300.0);
    layered.set_layer(Some("ball"));
    let composed = bench("promoted #ball", &mut layered, &mut t, scale);

    let differing = full.chunks(4).zip(composed.chunks(4)).filter(|(a, b)| a != b).count();
    println!("pixels differing after animating: {differing}/{}", (w * h) as usize);
}
