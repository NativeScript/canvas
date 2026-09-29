//! Threaded 2D on Direct3D 12: the render thread's device, a (headless) panel shown from this
//! thread and video frames shared by a D3D11 device. Device loss: threaded_d3d_lost.rs.
//!
//!     cargo test -p canvas-c --features d3d --test threaded_d3d
#![cfg(all(target_os = "windows", feature = "d3d"))]

use std::ffi::c_void;
use std::time::{Duration, Instant};

use canvas_c::webgpu::gpu_shared_frame::CanvasD3DSharedFrame;
use canvas_c::{CanvasColorSpace, CanvasRenderingContext2D};
use canvas_core::gpu::d3d::{D3D12Context, PowerPreference};
use windows::core::{Interface, PCWSTR};

const W: f32 = 64.;
const H: f32 = 48.;

fn create(threaded: bool) -> Option<*mut CanvasRenderingContext2D> {
    let create = if threaded {
        canvas_c::canvas_native_context_create_d3d_threaded
    } else {
        canvas_c::canvas_native_context_create_d3d
    };
    let context = create(W, H, 1., true, 0, 96., 0, CanvasColorSpace::Srgb);
    if context.is_null() {
        eprintln!("no Direct3D 12 device: skipped");
        return None;
    }
    Some(context)
}

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
fn threaded_frames_read_back_like_direct_ones() {
    let (Some(direct), Some(threaded)) = (create(false), create(true)) else { return };
    assert!(unsafe { &*threaded }.is_threaded());
    for n in 0..6 {
        draw(direct, n);
        draw(threaded, n);
        assert_eq!(pixels(direct), pixels(threaded), "frame {n}");
    }
    canvas_c::canvas_native_context_release(direct);
    canvas_c::canvas_native_context_release(threaded);
}

#[test]
fn presents_into_a_panel_from_the_render_thread_and_resizes() {
    let Some(context) = create(true) else { return };
    let panel = canvas_core::gpu::dxgi::headless_panel();
    assert!(canvas_c::canvas_native_context_attach_swap_chain_panel(context, panel.as_raw()));
    assert!(canvas_c::canvas_native_context_set_swap_chain_transform(context, 1., 1., 0., 0.));
    // Nothing takes the headless panel's frames: presents are deferred, then forced.
    for n in 0..8 {
        draw(context, n);
    }
    // The last frame's green.
    assert_eq!(pixel(&pixels(context), 12, 10), [0, 128, 0, 255]);
    assert!(!canvas_c::canvas_native_context_is_lost(context));

    canvas_c::canvas_native_context_resize(context, W * 2., H * 2.);
    let mut out = vec![0u8; (W * H * 16.) as usize];
    unsafe { &mut *context }.read_pixels_into(&mut out, (0, 0), (W as i32 * 2, H as i32 * 2));
    assert!(out.iter().all(|&v| v == 0), "resizing clears");
    canvas_c::canvas_native_context_resize(context, W, H);
    draw(context, 1);
    assert_eq!(pixel(&pixels(context), 10, 10), [0, 128, 0, 255]);
    canvas_c::canvas_native_context_release(context);
}

struct Producer {
    frame: CanvasD3DSharedFrame,
    release: windows::Win32::Graphics::Direct3D11::ID3D11Fence,
}

/// A green BGRA texture and its fences, shared the way canvas-media shares video frames.
fn producer(size: u32) -> Option<Producer> {
    use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_UNKNOWN;
    use windows::Win32::Graphics::Direct3D11::*;
    use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};
    use windows::Win32::Graphics::Dxgi::{IDXGIAdapter, IDXGIResource1, DXGI_SHARED_RESOURCE_READ, DXGI_SHARED_RESOURCE_WRITE};

    // The adapter the render thread picks too.
    let d3d12 = D3D12Context::shared(PowerPreference::Default)?;
    let adapter: IDXGIAdapter = d3d12.adapter().cast().ok()?;
    let (mut device, mut context) = (None, None);
    unsafe {
        D3D11CreateDevice(
            &adapter,
            D3D_DRIVER_TYPE_UNKNOWN,
            windows::Win32::Foundation::HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            Some(&mut context),
        )
    }
    .ok()?;
    let device: ID3D11Device5 = device?.cast().ok()?;
    let context: ID3D11DeviceContext4 = context?.cast().ok()?;

    let green: Vec<u8> = [0u8, 255, 0, 255].repeat((size * size) as usize);
    let desc = D3D11_TEXTURE2D_DESC {
        Width: size,
        Height: size,
        MipLevels: 1,
        ArraySize: 1,
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: (D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
        MiscFlags: (D3D11_RESOURCE_MISC_SHARED_NTHANDLE.0 | D3D11_RESOURCE_MISC_SHARED.0) as u32,
        ..Default::default()
    };
    let data = D3D11_SUBRESOURCE_DATA {
        pSysMem: green.as_ptr() as *const c_void,
        SysMemPitch: size * 4,
        SysMemSlicePitch: 0,
    };
    let mut texture: Option<ID3D11Texture2D> = None;
    unsafe { device.CreateTexture2D(&desc, Some(&data), Some(&mut texture)) }.ok()?;
    let texture = texture?;
    let access = DXGI_SHARED_RESOURCE_READ.0 | DXGI_SHARED_RESOURCE_WRITE.0;
    let texture_handle = unsafe { texture.cast::<IDXGIResource1>().ok()?.CreateSharedHandle(None, access, PCWSTR::null()) }.ok()?;

    let fence = || -> Option<(ID3D11Fence, windows::Win32::Foundation::HANDLE)> {
        let mut fence: Option<ID3D11Fence> = None;
        unsafe { device.CreateFence(0, D3D11_FENCE_FLAG_SHARED, &mut fence) }.ok()?;
        let fence = fence?;
        let handle = unsafe { fence.CreateSharedHandle(None, windows::Win32::Foundation::GENERIC_ALL.0, PCWSTR::null()) }.ok()?;
        Some((fence, handle))
    };
    let (ready, ready_handle) = fence()?;
    let (release, release_handle) = fence()?;
    unsafe {
        context.Signal(&ready, 1).ok()?;
        context.Flush();
    }
    Some(Producer {
        frame: CanvasD3DSharedFrame {
            size: std::mem::size_of::<CanvasD3DSharedFrame>() as u32,
            consumed: 0,
            adapter_luid: d3d12.adapter_luid(),
            texture_id: 1,
            texture: texture_handle.0,
            ready_fence_id: 2,
            ready_fence: ready_handle.0,
            ready_value: 1,
            release_fence_id: 3,
            release_fence: release_handle.0,
            release_value: 1,
        },
        release,
    })
}

#[test]
fn a_shared_video_frame_is_drawn_on_the_render_thread_and_given_back() {
    let Some(context) = create(true) else { return };
    let Some(mut producer) = producer(16) else {
        eprintln!("no D3D11 device with shared fences: skipped");
        return;
    };
    let frame = &mut producer.frame as *mut CanvasD3DSharedFrame as *mut c_void;
    let drawn = unsafe {
        canvas_c::d3d::canvas_native_context_draw_d3d_shared_frame(context, frame, 16, 16, 0., 0., 16., 16., 20., 10., 16., 16.)
    };
    assert!(drawn);
    assert_eq!(producer.frame.consumed, 1);
    unsafe { &mut *context }.render();
    let out = pixels(context);
    assert_eq!(pixel(&out, 27, 17), [0, 255, 0, 255]);
    assert_eq!(pixel(&out, 10, 17), [0, 0, 0, 0]);

    let deadline = Instant::now() + Duration::from_secs(5);
    while unsafe { producer.release.GetCompletedValue() } < 1 {
        assert!(Instant::now() < deadline, "the frame was not given back");
        std::thread::sleep(Duration::from_millis(5));
    }

    // Another adapter's frame is left to the caller.
    producer.frame.adapter_luid ^= 1;
    producer.frame.consumed = 0;
    let drawn = unsafe {
        canvas_c::d3d::canvas_native_context_draw_d3d_shared_frame(context, frame, 16, 16, 0., 0., 16., 16., 0., 0., 16., 16.)
    };
    assert!(!drawn);
    assert_eq!(producer.frame.consumed, 0);
    canvas_c::canvas_native_context_release(context);
}
