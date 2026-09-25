//! The document's thread records a display list; another thread rasterizes it.

use canvas_svg::{FrameSlot, SvgDocument};
use std::sync::Arc;

const SOURCE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="120" viewBox="0 0 120 120">
    <rect width="120" height="120" fill="#202040"/>
    <circle cx="30" cy="60" r="20" fill="orange">
        <animate attributeName="cx" values="30;90;30" dur="2s" repeatCount="indefinite"/>
    </circle>
</svg>"##;

fn surface(size: i32) -> skia_safe::Surface {
    skia_safe::surfaces::raster_n32_premul((size, size)).expect("surface")
}

fn read(surface: &mut skia_safe::Surface, size: i32) -> Vec<u8> {
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

fn direct(doc: &mut SvgDocument, size: i32) -> Vec<u8> {
    let mut s = surface(size);
    s.canvas().clear(skia_safe::Color::TRANSPARENT);
    doc.render_frame(s.canvas(), size, size, 1.0);
    read(&mut s, size)
}

fn replayed(frame: &canvas_svg::RecordedFrame, size: i32) -> Vec<u8> {
    let mut s = surface(size);
    s.canvas().clear(skia_safe::Color::TRANSPARENT);
    frame.replay(s.canvas());
    read(&mut s, size)
}

#[test]
fn a_replayed_frame_matches_rendering_directly() {
    let mut doc = SvgDocument::from_bytes(SOURCE.as_bytes()).expect("parse");
    doc.set_container_size(120.0, 120.0);

    for step in 0..8 {
        let t = step as f64 / 4.0;
        doc.advance(t);
        let recorded = doc.record(120, 120, 1.0).expect("record");
        assert_eq!(
            direct(&mut doc, 120),
            replayed(&recorded, 120),
            "replaying the display list differed from rendering at t={t}s"
        );
    }
}

#[test]
fn a_frame_recorded_here_rasterizes_on_another_thread() {
    let mut doc = SvgDocument::from_bytes(SOURCE.as_bytes()).expect("parse");
    doc.set_container_size(120.0, 120.0);
    doc.advance(0.5);

    let expected = direct(&mut doc, 120);
    let recorded = doc.record(120, 120, 1.0).expect("record");

    // `doc` stays here; only the display list crosses.
    let handle = std::thread::spawn(move || replayed(&recorded, 120));
    let produced = handle.join().expect("render thread panicked");

    assert_eq!(expected, produced, "off-thread raster differed from on-thread render");
}

#[test]
fn the_slot_keeps_only_the_newest_frame() {
    let mut doc = SvgDocument::from_bytes(SOURCE.as_bytes()).expect("parse");
    doc.set_container_size(120.0, 120.0);
    let slot = Arc::new(FrameSlot::new());

    // Three frames produced before the renderer gets a turn: the first two are already stale.
    doc.advance(0.0);
    slot.commit(doc.record(120, 120, 1.0).unwrap());
    doc.advance(0.5);
    slot.commit(doc.record(120, 120, 1.0).unwrap());
    doc.advance(1.0);
    slot.commit(doc.record(120, 120, 1.0).unwrap());

    doc.advance(1.0);
    let newest = direct(&mut doc, 120);

    let taken = slot.take().expect("a frame was committed");
    assert_eq!(newest, replayed(&taken, 120), "the slot handed back a stale frame");
    assert!(!slot.has_frame(), "taking a frame should empty the slot");
}

#[test]
fn a_render_thread_consumes_frames_as_they_are_committed() {
    let slot = Arc::new(FrameSlot::new());
    let done = Arc::new(std::sync::atomic::AtomicBool::new(false));

    let renderer = {
        let slot = Arc::clone(&slot);
        let done = Arc::clone(&done);
        std::thread::spawn(move || {
            let mut rendered = 0usize;
            while !done.load(std::sync::atomic::Ordering::Acquire) || slot.has_frame() {
                if let Some(frame) = slot.take() {
                    let mut s = surface(120);
                    frame.replay(s.canvas());
                    rendered += 1;
                }
                std::thread::yield_now();
            }
            rendered
        })
    };

    let mut doc = SvgDocument::from_bytes(SOURCE.as_bytes()).expect("parse");
    doc.set_container_size(120.0, 120.0);
    for step in 0..30 {
        doc.advance(step as f64 / 30.0);
        slot.commit(doc.record(120, 120, 1.0).expect("record"));
    }
    done.store(true, std::sync::atomic::Ordering::Release);

    let rendered = renderer.join().expect("render thread panicked");
    assert!(rendered > 0, "the render thread never picked up a frame");
}
