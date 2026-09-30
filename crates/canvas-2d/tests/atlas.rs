use canvas_2d::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::context::Context;
use canvas_core::context_attributes::ColorSpace;
use skia_safe::Color;

const W: i32 = 100;
const H: i32 = 40;

fn context() -> Context {
    Context::new(W as f32, H as f32, 1.0, true, 0, 96., TextDirection::LTR, ColorSpace::Srgb)
}

/// 20x10: red on the left half, blue on the right.
fn sheet() -> skia_safe::Image {
    let mut surface = skia_safe::surfaces::raster_n32_premul((20, 10)).expect("surface");
    let mut paint = skia_safe::Paint::default();
    paint.set_color(Color::RED);
    surface.canvas().draw_rect(skia_safe::Rect::from_xywh(0., 0., 10., 10.), &paint);
    paint.set_color(Color::BLUE);
    surface.canvas().draw_rect(skia_safe::Rect::from_xywh(10., 0., 10., 10.), &paint);
    surface.image_snapshot()
}

fn pixel(ctx: &mut Context, x: i32, y: i32) -> [u8; 4] {
    let mut out = vec![0u8; (W * H * 4) as usize];
    ctx.get_pixels(&mut out, (0, 0), (W, H));
    let at = ((y * W + x) * 4) as usize;
    [out[at], out[at + 1], out[at + 2], out[at + 3]]
}

fn blend() -> CompositeOperationType {
    CompositeOperationType::from_str("destination-over").unwrap()
}

// Two sprites: the red one at (10, 10), the blue one at (50, 10).
const XFORM: [f32; 8] = [1., 0., 10., 10., 1., 0., 50., 10.];
const TEX: [f32; 8] = [0., 0., 10., 10., 10., 0., 10., 10.];

#[test]
fn draws_each_sprite_where_its_transform_puts_it() {
    let mut ctx = context();
    ctx.draw_atlas(&sheet(), &XFORM, &TEX, None, blend());
    assert_eq!(pixel(&mut ctx, 15, 15), [255, 0, 0, 255]);
    assert_eq!(pixel(&mut ctx, 55, 15), [0, 0, 255, 255]);
    assert_eq!(pixel(&mut ctx, 35, 15)[3], 0);
}

#[test]
fn more_transforms_than_sprites_draws_the_pairs_it_has() {
    let mut ctx = context();
    ctx.draw_atlas(&sheet(), &XFORM, &TEX[..4], None, blend());
    assert_eq!(pixel(&mut ctx, 15, 15), [255, 0, 0, 255]);
    assert_eq!(pixel(&mut ctx, 55, 15)[3], 0);
}

#[test]
fn colors_for_only_some_sprites_are_ignored_rather_than_aborting() {
    let mut ctx = context();
    ctx.draw_atlas(&sheet(), &XFORM, &TEX, Some(&[Color::GREEN]), blend());
    assert_eq!(pixel(&mut ctx, 15, 15), [255, 0, 0, 255]);
    assert_eq!(pixel(&mut ctx, 55, 15), [0, 0, 255, 255]);
}

#[test]
fn global_alpha_applies() {
    let mut ctx = context();
    ctx.set_global_alpha(0.5);
    ctx.draw_atlas(&sheet(), &XFORM, &TEX, None, blend());
    let alpha = pixel(&mut ctx, 15, 15)[3];
    assert!((120..=135).contains(&alpha), "alpha {alpha}");
}
