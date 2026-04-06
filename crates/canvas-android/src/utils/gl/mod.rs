use canvas_c::WebGLState;
use jni::sys::{jboolean, jfloat, jint, jlong, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;

pub mod st;
pub(crate) mod surface_texture;
pub mod texture_render;
pub mod webgl2_rendering_context;
pub mod webgl_rendering_context;

pub const TEXTURE_EXTERNAL_OES: u32 = 0x00008d65;

#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_Utils_nativeMakeStateContextCurrent(
    _: JNIEnv,
    _: jni::objects::JClass,
    state: jlong,
) -> jboolean {
    if state == 0 {
        return JNI_FALSE;
    }

    let state = state as *mut WebGLState;

    if state.is_null() {
        return JNI_FALSE;
    }
    let state = &mut *state;

    if state.get_inner().make_current() {
        return JNI_TRUE;
    }
    JNI_FALSE
}

/// Makes the EGL context of a 2D canvas context current on the calling thread.
/// `context` is a pointer to `CanvasRenderingContext2D`.
#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_Utils_nativeMake2DContextCurrent(
    _: JNIEnv,
    _: jni::objects::JClass,
    context: jlong,
) -> jboolean {
    if context == 0 {
        return JNI_FALSE;
    }
    let ctx = &*(context as *mut canvas_c::CanvasRenderingContext2D);
    if ctx.make_current() {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

/// Draw an OES external texture into a 2D canvas context using Skia's GPU path.
///
/// The EGL context that owns the OES texture **must already be current** on the
/// calling thread (call `nativeMake2DContextCurrent` first and call
/// `SurfaceTexture.updateTexImage()` before this function).
///
/// Parameters
/// - `context`     : `CanvasRenderingContext2D*`
/// - `texture_id`  : OES texture name (from `glGenTextures` in the same EGL context)
/// - `video_width` / `video_height` : dimensions of the decoded video frame
/// - `sx,sy,sw,sh` : source rectangle within the video frame
/// - `dx,dy,dw,dh` : destination rectangle in canvas coordinates
///
/// Returns `JNI_TRUE` when the image was drawn successfully.
#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_Utils_nativeContext2DDrawOESTexture(
    _: JNIEnv,
    _: jni::objects::JClass,
    context: jlong,
    texture_id: jint,
    video_width: jint,
    video_height: jint,
    sx: jfloat,
    sy: jfloat,
    sw: jfloat,
    sh: jfloat,
    dx: jfloat,
    dy: jfloat,
    dw: jfloat,
    dh: jfloat,
) -> jboolean {
    use skia_safe::gpu;

    if context == 0 || texture_id <= 0 || video_width <= 0 || video_height <= 0 {
        return JNI_FALSE;
    }

    let ctx = &mut *(context as *mut canvas_c::CanvasRenderingContext2D);

    // Ensure the 2D context's EGL context is current so Skia can reach the GPU.
    ctx.make_current();

    let gl_info = gpu::gl::TextureInfo {
        target: TEXTURE_EXTERNAL_OES,
        id: texture_id as u32,
        // GL_RGBA8 — the colour format Skia should treat this texture as.
        format: 0x8058,
        protected: gpu::Protected::No,
    };

    let bt = gpu::backend_textures::make_gl(
        (video_width, video_height),
        gpu::Mipmapped::No,
        gl_info,
        "",
    );

    let image_info = skia_safe::ImageInfo::new(
        skia_safe::ISize::new(video_width, video_height),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );

    let inner = ctx.get_context_mut();
    

    // `recording_context()` returns an owned value — the borrow on `inner.surface`
    // ends before the subsequent draw call, which is safe.
    if let Some(mut recording_ctx) = inner.get_recording_context() {
        if let Some(image) = skia_safe::Image::from_texture(
            &mut recording_ctx,
            &bt,
            gpu::SurfaceOrigin::TopLeft,
            image_info.color_type(),
            image_info.alpha_type(),
            image_info.color_space(),
        ) {
            inner.draw_image_src_xywh_dst_xywh(&image, sx, sy, sw, sh, dx, dy, dw, dh);
            return JNI_TRUE;
        }
    }

    JNI_FALSE
}
