use canvas_c::offscreen::*;
use canvas_c::{CanvasColorSpace, CanvasRenderingContext2D};

fn viewless(width: u32, height: u32) -> *const CanvasOffscreenSurface {
    canvas_native_offscreen_surface_create(width, height, 1., 96., 0, CanvasColorSpace::Srgb)
}

struct Sent<T>(T);
unsafe impl<T> Send for Sent<T> {}

#[test]
fn a_handle_is_adopted_once() {
    let surface = viewless(30, 20);
    let handle = unsafe { canvas_native_offscreen_surface_to_handle(surface) };
    assert_ne!(handle, 0);
    unsafe { canvas_native_offscreen_surface_release(surface) };

    let adopted = canvas_native_offscreen_surface_adopt(handle);
    assert!(!adopted.is_null(), "the handle holds the surface");
    assert_eq!(unsafe { canvas_native_offscreen_surface_get_width(adopted) }, 30);
    assert!(canvas_native_offscreen_surface_adopt(handle).is_null(), "a second adopt");
    assert!(!canvas_native_offscreen_surface_release_handle(handle));
    unsafe { canvas_native_offscreen_surface_release(adopted) };
}

#[test]
fn handles_are_never_reused() {
    let surface = viewless(1, 1);
    let first = unsafe { canvas_native_offscreen_surface_to_handle(surface) };
    assert!(canvas_native_offscreen_surface_release_handle(first));
    assert!(!canvas_native_offscreen_surface_release_handle(first), "released once");
    let second = unsafe { canvas_native_offscreen_surface_to_handle(surface) };
    assert!(second > first);
    assert!(canvas_native_offscreen_surface_release_handle(second));
    unsafe { canvas_native_offscreen_surface_release(surface) };
}

#[test]
fn a_surface_with_a_context_has_no_handle() {
    let surface = viewless(10, 10);
    let context = unsafe { canvas_native_offscreen_surface_create_2d(surface, true, 0, false) };
    assert!(!context.is_null());
    assert_eq!(unsafe { canvas_native_offscreen_surface_to_handle(surface) }, 0);
    assert!(
        unsafe { canvas_native_offscreen_surface_create_2d(surface, true, 0, false) }.is_null(),
        "one context per surface"
    );
    canvas_c::canvas_native_context_release(context);
    unsafe { canvas_native_offscreen_surface_release(surface) };
}

#[test]
fn an_adopted_surface_draws_on_its_new_thread() {
    let surface = viewless(16, 16);
    let handle = unsafe { canvas_native_offscreen_surface_to_handle(surface) };
    unsafe { canvas_native_offscreen_surface_release(surface) };

    let (pixel, size, url) = std::thread::spawn(move || {
        let surface = canvas_native_offscreen_surface_adopt(handle);
        assert!(!surface.is_null());
        let context = unsafe { canvas_native_offscreen_surface_create_2d(surface, true, 0, true) };
        let ctx: &mut CanvasRenderingContext2D = unsafe { &mut *context };
        assert!(ctx.is_threaded());

        canvas_c::resize(ctx, 20., 10.);
        unsafe { canvas_native_offscreen_surface_resize(surface, 20, 10) };

        let c = ctx.get_context_mut();
        c.set_fill_style_with_color("red");
        c.fill_rect_xywh(0., 0., 20., 10.);
        ctx.render();

        let data = ctx.image_data(0., 0., 20., 10.);
        let pixel = data.data()[..4].to_vec();
        let size = unsafe {
            (canvas_native_offscreen_surface_get_width(surface), canvas_native_offscreen_surface_get_height(surface))
        };
        // As the placeholder's toDataURL does.
        let shared = Sent(surface);
        let url = std::thread::spawn(move || {
            let shared = shared;
            let url = unsafe { canvas_native_offscreen_surface_to_data_url(shared.0, c"image/png".as_ptr(), 92) };
            assert!(!url.is_null());
            let string = unsafe { std::ffi::CStr::from_ptr(url) }.to_string_lossy().into_owned();
            canvas_c::canvas_native_string_destroy(url);
            string
        })
        .join()
        .unwrap();

        canvas_c::canvas_native_context_release(context);
        unsafe { canvas_native_offscreen_surface_release(surface) };
        (pixel, size, url)
    })
    .join()
    .unwrap();

    assert_eq!(pixel, vec![255, 0, 0, 255]);
    assert_eq!(size, (20, 10));
    assert!(url.starts_with("data:image/png;base64,"), "{url}");
}

#[test]
fn a_viewless_surface_has_no_view() {
    let surface = viewless(4, 4);
    assert!(!unsafe { canvas_native_offscreen_surface_has_view(surface) });
    unsafe { canvas_native_offscreen_surface_resize(surface, 8, 2) };
    assert_eq!(unsafe { canvas_native_offscreen_surface_get_width(surface) }, 8);
    assert_eq!(unsafe { canvas_native_offscreen_surface_get_height(surface) }, 2);
    unsafe { canvas_native_offscreen_surface_release(surface) };
}
