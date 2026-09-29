//! A lost threaded 2D context. Its own test binary: removing a device removes it for every
//! thread in the process (one device per adapter).
//!
//!     cargo test -p canvas-c --features d3d --test threaded_d3d_lost
#![cfg(all(target_os = "windows", feature = "d3d"))]

use std::time::{Duration, Instant};

use canvas_c::{CanvasColorSpace, CanvasRenderingContext2D};
use windows::core::Interface;

const W: f32 = 64.;
const H: f32 = 48.;

fn draw(context: *mut CanvasRenderingContext2D, n: usize) {
    let context = unsafe { &mut *context };
    let c = context.get_context_mut();
    c.set_fill_style_with_color(["red", "green", "blue"][n % 3]);
    c.fill_rect_xywh(4. + n as f32, 6., 20., 12.);
    context.render();
}

fn pixels(context: *mut CanvasRenderingContext2D) -> Vec<u8> {
    let mut out = vec![0u8; (W * H * 4.) as usize];
    unsafe { &mut *context }.read_pixels_into(&mut out, (0, 0), (W as i32, H as i32));
    out
}

fn pixel(pixels: &[u8], x: usize, y: usize) -> [u8; 4] {
    let at = (y * W as usize + x) * 4;
    pixels[at..at + 4].try_into().unwrap()
}

#[test]
fn a_lost_threaded_context_is_restored_into_its_panel() {
    let context = canvas_c::canvas_native_context_create_d3d_threaded(W, H, 1., true, 0, 96., 0, CanvasColorSpace::Srgb);
    if context.is_null() {
        eprintln!("no Direct3D 12 device: skipped");
        return;
    }
    let panel = canvas_core::gpu::dxgi::headless_panel();
    assert!(canvas_c::canvas_native_context_attach_swap_chain_panel(context, panel.as_raw()));
    draw(context, 0);
    unsafe { &mut *context }.get_context_mut().set_line_width(7.);
    if !canvas_c::canvas_native_d3d_simulate_device_removal() {
        eprintln!("RemoveDevice needs Windows 10 1809: skipped");
        return;
    }
    assert!(canvas_c::canvas_native_context_is_lost(context));
    draw(context, 0);

    let deadline = Instant::now() + Duration::from_secs(15);
    while !unsafe { canvas_c::canvas_native_context_restore_d3d(context, std::ptr::null_mut()) } {
        assert!(Instant::now() < deadline, "no new device");
        std::thread::sleep(Duration::from_millis(250));
    }
    assert!(!canvas_c::canvas_native_context_is_lost(context));
    assert!(pixels(context).iter().all(|&v| v == 0), "restored cleared");
    assert_eq!(unsafe { &*context }.get_context().line_width(), 1., "restored in the default state");
    draw(context, 2);
    assert_eq!(pixel(&pixels(context), 12, 10), [0, 0, 255, 255]);
    canvas_c::canvas_native_context_release(context);
}
