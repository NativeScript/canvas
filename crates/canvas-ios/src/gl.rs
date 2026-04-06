use canvas_c::CanvasRenderingContext2D;
use skia_safe::gpu;

fn draw_image_gl_texture(
    context: i64,
    gl_texture_id: u32,
    gl_target: u32,
    width: f32,
    height: f32,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) -> bool {
    if context == 0 || gl_texture_id == 0 {
        return false;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };
    context.make_current();

    let gl_info = gpu::gl::TextureInfo {
        target: gl_target,
        id: gl_texture_id,
        format: 0x8058, // GL_RGBA8
        protected: gpu::Protected::No,
    };

    let bt = unsafe {
        gpu::backend_textures::make_gl(
            (width as i32, height as i32),
            gpu::Mipmapped::No,
            gl_info,
            "",
        )
    };

    let image_info = skia_safe::ImageInfo::new(
        skia_safe::ISize::new(width as i32, height as i32),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );

    let ctx = context.get_context_mut();
    if let Some(mut recording_ctx) = ctx.get_recording_context() {
        if let Some(image) = skia_safe::Image::from_texture(
            &mut recording_ctx,
            &bt,
            gpu::SurfaceOrigin::TopLeft,
            image_info.color_type(),
            image_info.alpha_type(),
            image_info.color_space(),
        ) {
            ctx.draw_image_src_xywh_dst_xywh(
                &image, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
            );
            return true;
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_context_draw_image_dx_dy_with_gl_texture(
    context: i64,
    gl_texture_id: u32,
    gl_target: u32,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
) -> bool {
    draw_image_gl_texture(
        context,
        gl_texture_id,
        gl_target,
        width,
        height,
        0.0,
        0.0,
        width,
        height,
        dx,
        dy,
        width,
        height,
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_context_draw_image_dx_dy_dw_dh_with_gl_texture(
    context: i64,
    gl_texture_id: u32,
    gl_target: u32,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) -> bool {
    draw_image_gl_texture(
        context,
        gl_texture_id,
        gl_target,
        width,
        height,
        0.0,
        0.0,
        width,
        height,
        dx,
        dy,
        d_width,
        d_height,
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_context_draw_image_with_gl_texture(
    context: i64,
    gl_texture_id: u32,
    gl_target: u32,
    width: f32,
    height: f32,
    sx: f32,
    sy: f32,
    s_width: f32,
    s_height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) -> bool {
    draw_image_gl_texture(
        context,
        gl_texture_id,
        gl_target,
        width,
        height,
        sx,
        sy,
        s_width,
        s_height,
        dx,
        dy,
        d_width,
        d_height,
    )
}
