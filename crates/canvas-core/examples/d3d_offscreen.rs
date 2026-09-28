//! Renders into an offscreen Skia surface on the shared Direct3D 12 device and checks the result
//! by reading it back. `CANVAS_FORCE_WARP=1` runs it on the WARP software rasterizer.
//!
//!     cargo run -p canvas-core --features d3d --example d3d_offscreen [out.png]

use canvas_core::gpu::d3d::{D3D12Context, PowerPreference};
use skia_safe::gpu::{self, Budgeted};
use skia_safe::{AlphaType, Color, ColorType, EncodedImageFormat, ImageInfo, Paint};

fn main() {
    let device = D3D12Context::shared(PowerPreference::Default).expect("no Direct3D 12 device");
    let desc = unsafe { device.adapter().GetDesc1() }.expect("adapter description");
    let name = String::from_utf16_lossy(&desc.Description);
    println!("adapter: {} (warp: {})", name.trim_end_matches('\0'), device.is_warp());

    let mut context = device.make_direct_context().expect("Skia D3D context");
    let info = ImageInfo::new((256, 256), ColorType::RGBA8888, AlphaType::Premul, None);
    let mut surface = gpu::surfaces::render_target(
        &mut context,
        Budgeted::Yes,
        &info,
        None,
        gpu::SurfaceOrigin::TopLeft,
        None,
        false,
        None,
    )
    .expect("D3D render target");

    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let mut paint = Paint::default();
    paint.set_anti_alias(true).set_color(Color::from_rgb(0, 128, 255));
    canvas.draw_circle((128.0, 128.0), 96.0, &paint);
    context.flush_and_submit();

    let mut pixels = vec![0u8; 256 * 256 * 4];
    assert!(
        surface.read_pixels(&info, &mut pixels, 256 * 4, (0, 0)),
        "readback failed"
    );
    let at = |x: usize, y: usize| {
        let i = (y * 256 + x) * 4;
        [pixels[i], pixels[i + 1], pixels[i + 2], pixels[i + 3]]
    };
    assert_eq!(at(128, 128), [0, 128, 255, 255], "centre should be the circle colour");
    assert_eq!(at(2, 2), [255, 255, 255, 255], "corner should be the clear colour");
    println!("readback ok");

    if let Some(path) = std::env::args().nth(1) {
        let image = surface.image_snapshot();
        let data = image
            .encode(&mut context, EncodedImageFormat::PNG, None)
            .expect("PNG encode");
        std::fs::write(&path, data.as_bytes()).expect("write PNG");
        println!("wrote {path}");
    }
}
