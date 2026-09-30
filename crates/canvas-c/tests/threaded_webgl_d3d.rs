//! Threaded WebGL on ANGLE (Windows): contexts on the WebGL thread, shown in a (headless) panel
//! from this thread, next to an unthreaded one on this thread.
//!
//!     cargo test -p canvas-c --features 2d,webgl,gl,d3d --test threaded_webgl_d3d
#![cfg(all(target_os = "windows", feature = "webgl", feature = "gl"))]

use std::ffi::CString;

use canvas_c::WebGLState;
use windows::core::Interface;

const W: i32 = 64;
const H: i32 = 48;

const COLOR_BUFFER_BIT: u32 = 0x4000;
const RGBA: u32 = 0x1908;
const UNSIGNED_BYTE: u32 = 0x1401;
const VERTEX_SHADER: u32 = 0x8B31;
const FRAGMENT_SHADER: u32 = 0x8B30;
const LINK_STATUS: u32 = 0x8B82;

fn create(threaded: bool) -> Option<*mut WebGLState> {
    let create = if threaded {
        canvas_c::canvas_native_webgl_create_d3d_threaded
    } else {
        canvas_c::canvas_native_webgl_create_d3d
    };
    let state = create(W, H, 2, true, false, true, false, 0, true, false, false, false, false);
    if state.is_null() {
        eprintln!("ANGLE unavailable: skipped");
        return None;
    }
    Some(state)
}

fn clear(state: *mut WebGLState, n: usize) {
    let [r, g, b] = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]][n % 3];
    canvas_c::canvas_native_webgl_clear_color(r, g, b, 1., state);
    canvas_c::canvas_native_webgl_clear(COLOR_BUFFER_BIT, state);
}

fn pixel(state: *mut WebGLState) -> [u8; 4] {
    let mut out = [0u8; 4];
    canvas_c::canvas_native_webgl_read_pixels_u8(W / 2, H / 2, 1, 1, RGBA, UNSIGNED_BYTE, out.as_mut_ptr(), 4, state);
    out
}

fn link(state: *mut WebGLState) -> bool {
    let vertex = CString::new("#version 300 es\nin vec4 p; void main() { gl_Position = p; }").unwrap();
    let fragment = CString::new("#version 300 es\nprecision mediump float; out vec4 c; uniform vec4 u; void main() { c = u; }").unwrap();
    let program = canvas_c::canvas_native_webgl_create_program(state);
    for (kind, source) in [(VERTEX_SHADER, &vertex), (FRAGMENT_SHADER, &fragment)] {
        let shader = canvas_c::canvas_native_webgl_create_shader(kind, state);
        canvas_c::canvas_native_webgl_shader_source(shader, source.as_ptr(), state);
        canvas_c::canvas_native_webgl_compile_shader(shader, state);
        canvas_c::canvas_native_webgl_attach_shader(program, shader, state);
    }
    canvas_c::canvas_native_webgl_link_program(program, state);
    let result = canvas_c::canvas_native_webgl_get_program_parameter(program, LINK_STATUS, state);
    let linked = canvas_c::canvas_native_webgl_result_get_bool(result);
    canvas_c::canvas_native_webgl_WebGLResult_destroy(result);
    linked
}

#[test]
fn threaded_frames_read_back_like_direct_ones() {
    let (Some(direct), Some(threaded)) = (create(false), create(true)) else { return };
    assert!(unsafe { &*threaded }.is_threaded());
    for n in 0..6 {
        clear(direct, n);
        clear(threaded, n);
        assert!(canvas_c::canvas_native_webgl_present(direct));
        assert!(canvas_c::canvas_native_webgl_present(threaded));
        assert_eq!(pixel(direct), pixel(threaded), "frame {n}");
    }
    assert_eq!(canvas_c::canvas_native_webgl_get_error(threaded), 0);
    canvas_c::canvas_native_webgl_state_destroy(direct);
    canvas_c::canvas_native_webgl_state_destroy(threaded);
}

#[test]
fn presents_into_a_panel_from_the_webgl_thread_and_resizes() {
    let Some(state) = create(true) else { return };
    let panel = canvas_core::gpu::dxgi::headless_panel();
    assert!(canvas_c::canvas_native_webgl_attach_swap_chain_panel(state, panel.as_raw()));
    assert!(canvas_c::canvas_native_webgl_set_swap_chain_transform(state, 1., 1., 0., 0.));
    // Nothing takes the headless panel's frames: they are held, then forced out.
    for n in 0..8 {
        clear(state, n);
        assert!(canvas_c::canvas_native_webgl_present(state));
    }
    // The last frame's green, and its present queued nothing that stays behind.
    assert_eq!(pixel(state), [0, 255, 0, 255]);
    assert!(!canvas_c::canvas_native_webgl_get_is_context_lost(state));

    assert!(canvas_c::canvas_native_webgl_resize_d3d(state, W * 2, H * 2));
    assert_eq!(canvas_c::canvas_native_webgl_state_get_drawing_buffer_width(state), W * 2);
    clear(state, 2);
    assert!(canvas_c::canvas_native_webgl_present(state));
    assert_eq!(pixel(state), [0, 0, 255, 255]);
    // Dropped with a frame still held.
    canvas_c::canvas_native_webgl_state_destroy(state);

    let Some(next) = create(true) else { return };
    clear(next, 0);
    assert_eq!(pixel(next), [255, 0, 0, 255]);
    canvas_c::canvas_native_webgl_state_destroy(next);
}

#[test]
fn links_programs_on_threaded_contexts_beside_one_on_this_thread() {
    let Some(direct) = create(false) else { return };
    let threaded: Vec<_> = (0..4).filter_map(|_| create(true)).collect();
    assert!(link(direct));
    for &state in &threaded {
        assert!(link(state));
    }
    for state in threaded {
        canvas_c::canvas_native_webgl_state_destroy(state);
    }
    canvas_c::canvas_native_webgl_state_destroy(direct);
}
