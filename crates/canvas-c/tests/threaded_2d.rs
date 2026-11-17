use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::context::Context;
use canvas_c::CanvasRenderingContext2D;
use canvas_core::context_attributes::ColorSpace;

const W: f32 = 160.;
const H: f32 = 100.;

fn cpu_context() -> Context {
    Context::new(W, H, 1.0, true, 0, 96., TextDirection::LTR, ColorSpace::Srgb)
}

fn direct() -> CanvasRenderingContext2D {
    CanvasRenderingContext2D::new(cpu_context(), true)
}

fn threaded() -> CanvasRenderingContext2D {
    CanvasRenderingContext2D::new_threaded_with(
        W,
        H,
        1.0,
        true,
        0,
        96.,
        TextDirection::LTR,
        ColorSpace::Srgb,
        || Some(CanvasRenderingContext2D::new(cpu_context(), true)),
    )
    .expect("render thread")
}

fn pixels(ctx: &mut CanvasRenderingContext2D) -> Vec<u8> {
    let mut out = vec![0u8; (W * H * 4.) as usize];
    ctx.read_pixels_into(&mut out, (0, 0), (W as i32, H as i32));
    out
}

fn frame(ctx: &mut CanvasRenderingContext2D, n: usize) {
    let c = ctx.get_context_mut();
    let colors = ["red", "green", "blue", "orange", "purple"];
    c.set_fill_style_with_color(colors[n % colors.len()]);
    c.translate(3., 2.);
    c.save();
    c.rotate(0.1 * n as f32);
    c.fill_rect_xywh(10. + n as f32, 10., 30., 20.);
    c.restore();
}

#[test]
fn threaded_frames_read_back_like_direct_ones() {
    let mut a = direct();
    let mut b = threaded();
    assert!(b.is_threaded());
    for n in 0..12 {
        frame(&mut a, n);
        frame(&mut b, n);
        a.render();
        b.render();
        assert_eq!(pixels(&mut a), pixels(&mut b), "frame {n}");
    }
}

#[test]
fn a_read_mid_frame_sees_everything_drawn_so_far() {
    let mut a = direct();
    let mut b = threaded();
    frame(&mut a, 0);
    frame(&mut b, 0);
    // No render(): the read itself must hand the pending draws over first.
    let (x, y, w, h) = (0., 0., W, H);
    let expected = a.image_data(x, y, w, h);
    let actual = b.image_data(x, y, w, h);
    assert_eq!(expected.data(), actual.data());
    assert!(actual.data().iter().any(|&v| v != 0), "something was drawn");
}

#[test]
fn a_threaded_canvas_draws_into_a_direct_one() {
    let mut source = threaded();
    frame(&mut source, 3);
    let image = source.image().expect("an image of the threaded canvas");

    let mut reference_source = direct();
    frame(&mut reference_source, 3);
    let reference = reference_source.image().expect("an image");

    let mut a = direct();
    let mut b = direct();
    a.get_context_mut().draw_image_dx_dy(&reference, 5., 5.);
    b.get_context_mut().draw_image_dx_dy(&image, 5., 5.);
    assert_eq!(pixels(&mut a), pixels(&mut b));
}

#[test]
fn frames_merged_while_the_render_thread_is_behind_still_match() {
    for clear_each_frame in [false, true] {
        let mut a = direct();
        let mut b = threaded();
        // No reads in between, so commits outpace replays and merge.
        for n in 0..200 {
            for ctx in [&mut a, &mut b] {
                if clear_each_frame {
                    ctx.get_context_mut().clear_rect(0., 0., W, H);
                }
                frame(ctx, n);
                ctx.render();
            }
        }
        assert_eq!(pixels(&mut a), pixels(&mut b), "clear each frame: {clear_each_frame}");
        drop(b);
    }
}

fn fill(ctx: &mut CanvasRenderingContext2D, color: &str) {
    let c = ctx.get_context_mut();
    c.set_fill_style_with_color(color);
    c.fill_rect_xywh(0., 0., W, H);
}

/// Holds the render thread until the returned sender goes, so work queued meanwhile waits behind it.
fn hold(ctx: &CanvasRenderingContext2D) -> mpsc::Sender<()> {
    let (release, held) = mpsc::channel::<()>();
    ctx.render_target().expect("a threaded canvas").post(move |_| {
        let _ = held.recv();
    });
    release
}

/// Lets the render thread go after a while; the flag says whether it has.
fn release_later(release: mpsc::Sender<()>) -> Arc<AtomicBool> {
    let released = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&released);
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(200));
        flag.store(true, Ordering::Release);
        drop(release);
    });
    released
}

#[test]
fn a_later_read_sees_the_canvas_as_it_was_at_the_call() {
    let mut ctx = threaded();
    let release = hold(&ctx);
    fill(&mut ctx, "red");
    let (width, height, red) = ctx.read_all_later();
    // Redrawn before the render thread has read it, as a text renderer reusing one canvas does.
    fill(&mut ctx, "blue");
    ctx.render();
    drop(release);
    assert_eq!((width, height), (W as i32, H as i32));
    assert_eq!(red.wait()[..4], [255u8, 0, 0, 255], "the read saw the later drawing");
    assert_eq!(pixels(&mut ctx)[..4], [0u8, 0, 255, 255]);
}

#[test]
fn reads_past_the_limit_wait_for_the_render_thread() {
    let mut ctx = threaded();
    let released = release_later(hold(&ctx));
    let first = ctx.read_all_later().2;
    let second = ctx.read_all_later().2;
    assert!(!released.load(Ordering::Acquire), "the first two reads waited");
    let third = ctx.read_all_later().2;
    assert!(released.load(Ordering::Acquire), "a third read was queued without waiting");
    for read in [first, second, third] {
        read.wait();
    }

    let released = release_later(hold(&ctx));
    let fourth = ctx.read_all_later().2;
    assert!(!released.load(Ordering::Acquire), "reads already taken still counted");
    fourth.wait();
}
