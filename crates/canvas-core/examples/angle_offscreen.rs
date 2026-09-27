//! Creates offscreen WebGL-style and canvas-style `GLContext`s on ANGLE and checks a clear by
//! reading it back. Needs the ANGLE DLLs: set `CANVAS_ANGLE_DIR` (e.g. `.angle-prebuilt/angle-x64/bin`).
//!
//!     cargo run -p canvas-core --features gl --example angle_offscreen

use canvas_core::context_attributes::{ColorSpace, ContextAttributes, PowerPreference};
use canvas_core::gpu::gl::GLContext;

fn attributes(is_canvas: bool) -> ContextAttributes {
    ContextAttributes::new(
        true,
        !is_canvas,
        false,
        false,
        PowerPreference::Default,
        true,
        false,
        false,
        false,
        false,
        is_canvas,
        false,
        ColorSpace::Srgb,
    )
}

fn main() {
    for is_canvas in [false, true] {
        let mut attrs = attributes(is_canvas);
        let context = GLContext::create_offscreen_context(&mut attrs, 64, 32).expect("ANGLE context");
        assert!(context.make_current(), "make_current");
        assert_eq!(context.get_surface_dimensions(), (64, 32));

        let mut pixel = [0u8; 4];
        unsafe {
            gl_bindings::ClearColor(0.0, 0.5, 1.0, 1.0);
            gl_bindings::Clear(gl_bindings::COLOR_BUFFER_BIT);
            gl_bindings::ReadPixels(
                0,
                0,
                1,
                1,
                gl_bindings::RGBA,
                gl_bindings::UNSIGNED_BYTE,
                pixel.as_mut_ptr() as *mut _,
            );
        }
        let version = unsafe { std::ffi::CStr::from_ptr(gl_bindings::GetString(gl_bindings::VERSION) as *const _) };
        println!(
            "{}: {} pixel={pixel:?} gl2={}",
            if is_canvas { "canvas" } else { "webgl " },
            version.to_string_lossy(),
            GLContext::has_gl2support()
        );
        assert_eq!(pixel[0], 0);
        assert!((127..=128).contains(&pixel[1]));
        assert_eq!(pixel[2], 255);
        assert!(context.remove_if_current());
    }
    println!("ok");
}
