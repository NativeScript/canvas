use canvas_2d::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_2d::context::pixel_manipulation::ImageData;
use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::context::Context;
use canvas_core::context_attributes::ColorSpace;

const W: f32 = 200.;
const H: f32 = 120.;

fn context() -> Context {
    Context::new(W, H, 1.0, true, 0, 96., TextDirection::LTR, ColorSpace::Srgb)
}

fn pixels(ctx: &mut Context) -> Vec<u8> {
    let mut out = vec![0u8; (W * H * 4.) as usize];
    ctx.get_pixels(&mut out, (0, 0), (W as i32, H as i32));
    out
}

fn clip_rect(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32) {
    ctx.begin_path();
    ctx.rect(x, y, w, h);
    ctx.clip(None, None);
}

fn fill(ctx: &mut Context, color: &str, x: f32, y: f32, w: f32, h: f32) {
    ctx.set_fill_style_with_color(color);
    ctx.fill_rect_xywh(x, y, w, h);
}

const FRAMES: [fn(&mut Context); 9] = [
    |ctx| {
        ctx.translate(20., 10.);
        ctx.save();
        clip_rect(ctx, 0., 0., 60., 60.);
        fill(ctx, "red", -50., -50., 300., 300.);
    },
    |ctx| {
        // Still translated and clipped from the previous frame.
        fill(ctx, "blue", 30., 30., 100., 100.);
        ctx.restore();
        fill(ctx, "green", 100., 0., 40., 40.);
    },
    |ctx| {
        ctx.rotate(0.3);
        ctx.set_line_width(3.);
        ctx.set_stroke_style_with_color("black");
        ctx.stroke_rect_xywh(10., 10., 50., 30.);
        ctx.set_font("16px sans-serif");
        fill(ctx, "purple", 0., 0., 0., 0.);
        ctx.fill_text("recorded", 5., 70., None);
        ctx.set_global_composite_operation(CompositeOperationType::from_str("destination-out").unwrap());
        fill(ctx, "black", 40., 20., 30., 30.);
        ctx.set_global_composite_operation(CompositeOperationType::from_str("source-over").unwrap());
    },
    |ctx| {
        ctx.save();
        clip_rect(ctx, 0., 0., 120., 80.);
        ctx.save();
        ctx.translate(10., 10.);
        clip_rect(ctx, 0., 0., 40., 200.);
        fill(ctx, "orange", -100., -100., 400., 400.);
        ctx.restore();
        // One save level and its clip left open for the next frame.
    },
    |ctx| {
        // Ends with clips on two saved levels below the current one.
        ctx.save();
        ctx.translate(30., 0.);
        clip_rect(ctx, 0., 0., 150., 40.);
        ctx.save();
        ctx.rotate(0.2);
    },
    |ctx| {
        fill(ctx, "navy", -300., -300., 900., 900.);
        ctx.restore();
        fill(ctx, "gold", 60., 10., 200., 200.);
        ctx.restore();
        fill(ctx, "pink", 100., 50., 200., 200.);
        ctx.restore();
        fill(ctx, "gray", 170., 0., 30., 30.);
    },
    |ctx| {
        // putImageData ignores the leftover clip and transform; later draws still see them.
        let mut data = ImageData::new(30, 20);
        for px in data.data_mut().chunks_mut(4) {
            px.copy_from_slice(&[255, 0, 128, 255]);
        }
        ctx.put_image_data(&data, 150., 90., 0., 0., 30., 20.);
        fill(ctx, "lime", 0., 0., 20., 20.);
    },
    |ctx| {
        fill(ctx, "teal", 0., 0., 300., 300.);
        ctx.reset();
        // reset() drops the clip, the stack and the transform.
        fill(ctx, "rgba(0, 0, 255, 0.5)", 150., 60., 50., 60.);
    },
    |ctx| {
        ctx.translate(5., 5.);
        clip_rect(ctx, 0., 0., 20., 20.);
        fill(ctx, "maroon", 0., 0., 100., 100.);
    },
];

#[test]
fn recorded_frames_match_direct_drawing() {
    let mut direct = context();
    let mut recorded = context();
    recorded.begin_recording();
    assert!(recorded.is_recording());

    for (i, frame) in FRAMES.iter().enumerate() {
        frame(&mut direct);
        frame(&mut recorded);
        let frame = recorded.take_frame();
        recorded.replay(frame);
        let expected = pixels(&mut direct);
        let actual = pixels(&mut recorded);
        let differing = expected
            .chunks(4)
            .zip(actual.chunks(4))
            .filter(|(a, b)| a != b)
            .count();
        assert_eq!(differing, 0, "frame {i}: {differing} pixels differ");
    }
}

#[test]
fn ending_a_recording_hands_the_state_back_to_the_surface() {
    let mut direct = context();
    let mut recorded = context();
    recorded.begin_recording();

    FRAMES[0](&mut direct);
    FRAMES[0](&mut recorded);
    let frame = recorded.end_recording();
    assert!(!frame.is_empty());
    recorded.replay(frame);
    assert!(!recorded.is_recording());

    // Direct from here on: the transform and clip from frame 0 must still apply.
    FRAMES[1](&mut direct);
    FRAMES[1](&mut recorded);
    assert_eq!(pixels(&mut direct), pixels(&mut recorded));
}

fn same_as_direct(frames: &[fn(&mut Context)]) {
    let mut direct = context();
    let mut recorded = context();
    recorded.begin_recording();
    for (i, frame) in frames.iter().enumerate() {
        frame(&mut direct);
        frame(&mut recorded);
        let taken = recorded.take_frame();
        recorded.replay(taken);
        assert_eq!(pixels(&mut direct), pixels(&mut recorded), "frame {i}");
    }
}

/// Merged before one replay, as the render thread does when it falls behind.
fn same_as_direct_merged(frames: &[fn(&mut Context)]) {
    let mut direct = context();
    let mut recorded = context();
    recorded.begin_recording();
    let mut merged = recorded.take_frame();
    for frame in frames {
        frame(&mut direct);
        frame(&mut recorded);
        merged.append(recorded.take_frame());
    }
    recorded.replay(merged);
    let differing = differing(&pixels(&mut direct), &pixels(&mut recorded));
    assert_eq!(differing, 0, "{} merged frames: {differing} pixels differ", frames.len());
}

fn differing(a: &[u8], b: &[u8]) -> usize {
    a.chunks(4).zip(b.chunks(4)).filter(|(a, b)| a != b).count()
}

const PARTIAL_CLEARS: [fn(&mut Context); 4] = [
        |ctx| fill(ctx, "red", 0., 0., W, H),
        |ctx| {
            // Clipped: only part of the canvas is cleared.
            ctx.save();
            clip_rect(ctx, 0., 0., 50., 50.);
            ctx.clear_rect(0., 0., W, H);
            ctx.restore();
        },
        |ctx| {
            // Rotated: the full-size rect no longer covers the corners.
            ctx.rotate(0.2);
            ctx.clear_rect(0., 0., W, H);
            ctx.reset();
        },
        |ctx| {
            // A full clear, then more drawing, in one frame.
            fill(ctx, "green", 10., 10., 50., 50.);
            ctx.clear_rect(0., 0., W, H);
            fill(ctx, "blue", 30., 30., 50., 50.);
        },
];

#[test]
fn clears_that_do_not_cover_the_canvas_keep_what_came_before() {
    same_as_direct(&PARTIAL_CLEARS);
    for n in 2..=PARTIAL_CLEARS.len() {
        same_as_direct_merged(&PARTIAL_CLEARS[..n]);
    }
}

#[test]
fn a_frame_that_clears_everything_replaces_earlier_ones_when_merged() {
    let draw_a: fn(&mut Context) = |ctx| fill(ctx, "red", 0., 0., W, H);
    let draw_b: fn(&mut Context) = |ctx| {
        ctx.translate(10., 0.);
        ctx.clear_rect(-10., 0., W, H);
        fill(ctx, "blue", 20., 20., 40., 40.);
    };

    let mut direct = context();
    draw_a(&mut direct);
    draw_b(&mut direct);

    let mut recorded = context();
    recorded.begin_recording();
    draw_a(&mut recorded);
    let mut merged = recorded.take_frame();
    draw_b(&mut recorded);
    let b = recorded.take_frame();
    let b_len = b.len();
    merged.append(b);
    assert_eq!(merged.len(), b_len, "the covered frame was dropped");
    recorded.replay(merged);

    assert_eq!(pixels(&mut direct), pixels(&mut recorded));
}

fn numbered(width: i32, height: i32) -> ImageData {
    let mut data = ImageData::new(width, height);
    for (i, px) in data.data_mut().chunks_mut(4).enumerate() {
        px.copy_from_slice(&[((i % width as usize) as u8).wrapping_mul(20), ((i / width as usize) as u8).wrapping_mul(20), 200, 255]);
    }
    data
}

#[test]
fn put_image_data_copies_only_the_dirty_rect_from_its_place_in_the_source() {
    let data = numbered(10, 10);
    // (dirty x, y, width, height) that all mean columns 2..6, rows 3..8.
    for (dirty, recording) in [
        ((2., 3., 4., 5.), false),
        ((6., 8., -4., -5.), false),
        ((2., 3., 4., 5.), true),
    ] {
        let mut ctx = context();
        if recording {
            ctx.begin_recording();
        }
        ctx.put_image_data(&data, 20., 30., dirty.0, dirty.1, dirty.2, dirty.3);
        if recording {
            let frame = ctx.take_frame();
            ctx.replay(frame);
        }
        let out = pixels(&mut ctx);
        for y in 0..H as usize {
            for x in 0..W as usize {
                let got = &out[(y * W as usize + x) * 4..][..4];
                let (sx, sy) = (x as i32 - 20, y as i32 - 30);
                let inside = (2..6).contains(&sx) && (3..8).contains(&sy);
                let want: [u8; 4] = if inside {
                    [sx as u8 * 20, sy as u8 * 20, 200, 255]
                } else {
                    [0, 0, 0, 0]
                };
                assert_eq!(got, want, "dirty {dirty:?} recording {recording} at ({x}, {y})");
            }
        }
    }
}

#[test]
fn put_image_data_clamps_the_dirty_rect_and_skips_an_empty_one() {
    let data = numbered(10, 10);
    let mut clamped = context();
    clamped.put_image_data(&data, 0., 0., -5., -5., 100., 100.);
    let mut whole = context();
    whole.put_image_data(&data, 0., 0., 0., 0., 10., 10.);
    assert_eq!(pixels(&mut clamped), pixels(&mut whole));

    let mut empty = context();
    empty.put_image_data(&data, 0., 0., 3., 3., 0., 4.);
    assert!(pixels(&mut empty).iter().all(|&v| v == 0));
}

fn checker() -> canvas_2d::context::Image {
    let data = numbered(16, 12);
    canvas_2d::utils::image::from_image_slice(data.data(), 16, 12).expect("image")
}

fn external_setup(ctx: &mut Context) {
    ctx.translate(15., 10.);
    ctx.rotate(0.1);
    ctx.save();
    clip_rect(ctx, 0., 0., 70., 50.);
    ctx.set_global_alpha(0.6);
}

#[test]
fn an_external_image_draws_like_draw_image_in_both_modes() {
    use canvas_2d::context::recording::{ExternalImage, ExternalRelease};
    use skia_safe::Rect;
    let (src, dst) = (Rect::from_xywh(2., 1., 12., 10.), Rect::from_xywh(5., 5., 90., 60.));

    let mut expected = context();
    external_setup(&mut expected);
    expected.draw_image_src_xywh_dst_xywh(&checker(), 2., 1., 12., 10., 5., 5., 90., 60.);

    for recording in [false, true] {
        let mut ctx = context();
        if recording {
            ctx.begin_recording();
        }
        external_setup(&mut ctx);
        if recording {
            let frame = ctx.take_frame();
            ctx.replay(frame);
        }
        let released = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let flag = released.clone();
        let make: ExternalImage = Box::new(move |_: &mut Context| {
            let release: ExternalRelease = Box::new(move |_: &mut Context| {
                flag.store(true, std::sync::atomic::Ordering::SeqCst)
            });
            Some((checker(), release))
        });
        ctx.draw_external_image(16, 12, src, dst, make);
        if recording {
            assert!(!released.load(std::sync::atomic::Ordering::SeqCst), "not drawn before replay");
            let frame = ctx.take_frame();
            ctx.replay(frame);
        }
        assert!(released.load(std::sync::atomic::Ordering::SeqCst), "release ran after the draw");
        let d = differing(&pixels(&mut expected), &pixels(&mut ctx));
        assert_eq!(d, 0, "recording {recording}: {d} pixels differ");
    }
}
