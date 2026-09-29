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
