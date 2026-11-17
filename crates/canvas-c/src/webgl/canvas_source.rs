//! A 2D canvas as a WebGL texture source.

use crate::c2d::CanvasRenderingContext2D;
use crate::webgl::WebGLState;

/// For the readback, which is unpremultiplied, when `UNPACK_PREMULTIPLY_ALPHA_WEBGL` is set.
pub(crate) fn premultiply_in_place(pixels: &mut [u8]) {
    for pixel in pixels.chunks_exact_mut(4) {
        let alpha = pixel[3] as u32;
        if alpha != 255 {
            for channel in &mut pixel[..3] {
                *channel = ((*channel as u32 * alpha + 127) / 255) as u8;
            }
        }
    }
}

/// Copies the canvas into the texture bound to `target` with no readback: the canvas draws itself
/// into a texture that this context reads through an EGLImage. `allocate` (an internal format)
/// first allocates `level` at the canvas size, as texImage2D does. `false`: use the readback.
#[cfg(target_os = "linux")]
#[allow(clippy::too_many_arguments)]
pub(crate) fn copy_on_gpu(
    canvas: &mut CanvasRenderingContext2D,
    state: &WebGLState,
    target: u32,
    level: i32,
    allocate: Option<i32>,
    x: i32,
    y: i32,
    format: u32,
    type_: u32,
) -> bool {
    use canvas_webgl::prelude::WebGLVersion;

    if canvas.engine() != crate::c2d::Engine::GL
        || format != gl_bindings::RGBA
        || type_ != gl_bindings::UNSIGNED_BYTE
    {
        return false;
    }
    // The copy is premultiplied, like the canvas.
    let (flip_y, premultiply, version) =
        state.sync(|state| (state.get_flip_y(), state.get_unpack_premultiply_alpha(), state.get_webgl_version()));
    let webgl2 = matches!(version, WebGLVersion::V2);
    // Sized formats are WebGL 2 only; anything else is validated by the readback path.
    let allowed = |internal: i32| internal == gl_bindings::RGBA as i32 || (webgl2 && internal == gl_bindings::RGBA8 as i32);
    if !premultiply || allocate.is_some_and(|internal| !allowed(internal)) {
        return false;
    }
    let Some(shared) = canvas
        .with_pixels_source(move |context| context.share_gl_texture(flip_y))
        .flatten()
    else {
        return false;
    };
    let (width, height) = canvas.get_context().dimensions();
    let (width, height) = (width as i32, height as i32);

    state.post(move |state| unsafe {
        state.make_current();
        let mut source = 0;
        gl_bindings::GenTextures(1, &mut source);
        if shared.attach(source) {
            // WebGL 1 has one framebuffer binding; WebGL 2 keeps its draw binding untouched.
            let (bind, binding) = if webgl2 {
                (gl_bindings::READ_FRAMEBUFFER, gl_bindings::READ_FRAMEBUFFER_BINDING)
            } else {
                (gl_bindings::FRAMEBUFFER, gl_bindings::FRAMEBUFFER_BINDING)
            };
            let mut framebuffer = 0;
            let mut previous = 0;
            gl_bindings::GetIntegerv(binding, &mut previous);
            gl_bindings::GenFramebuffers(1, &mut framebuffer);
            gl_bindings::BindFramebuffer(bind, framebuffer);
            gl_bindings::FramebufferTexture2D(bind, gl_bindings::COLOR_ATTACHMENT0, gl_bindings::TEXTURE_2D, source, 0);
            if let Some(internal) = allocate {
                let mut unpack_buffer = 0;
                if webgl2 {
                    gl_bindings::GetIntegerv(gl_bindings::PIXEL_UNPACK_BUFFER_BINDING, &mut unpack_buffer);
                    gl_bindings::BindBuffer(gl_bindings::PIXEL_UNPACK_BUFFER, 0);
                }
                gl_bindings::TexImage2D(target, level, internal, width, height, 0, format, type_, std::ptr::null());
                if unpack_buffer != 0 {
                    gl_bindings::BindBuffer(gl_bindings::PIXEL_UNPACK_BUFFER, unpack_buffer as u32);
                }
            }
            gl_bindings::CopyTexSubImage2D(target, level, x, y, 0, 0, width, height);
            gl_bindings::BindFramebuffer(bind, previous as u32);
            gl_bindings::DeleteFramebuffers(1, &framebuffer);
        }
        gl_bindings::DeleteTextures(1, &source);
        drop(shared);
    });
    true
}

#[cfg(not(target_os = "linux"))]
#[allow(clippy::too_many_arguments)]
pub(crate) fn copy_on_gpu(
    _canvas: &mut CanvasRenderingContext2D,
    _state: &WebGLState,
    _target: u32,
    _level: i32,
    _allocate: Option<i32>,
    _x: i32,
    _y: i32,
    _format: u32,
    _type: u32,
) -> bool {
    false
}
