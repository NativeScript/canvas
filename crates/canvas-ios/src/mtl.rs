use canvas_c::webgpu::gpu::CanvasWebGPUInstance;
use canvas_c::{webgpu, CanvasColorSpace, CanvasRenderingContext2D};
use std::ffi::{c_longlong, c_void};
use std::ptr::NonNull;
use skia_safe::gpu;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub(crate) struct iOSWebGPUContext {
    ios_view: NonNull<c_void>,

}

#[no_mangle]
pub extern "C" fn canvas_native_ios_create_2d_context_metal(
    view: *mut c_void,
    alpha: bool,
    density: f32,
    samples: usize,
    font_color: i32,
    ppi: f32,
    direction: i32,
    color_space: CanvasColorSpace,
) -> i64 {
    let ctx_2d = CanvasRenderingContext2D::new_metal(
        canvas_2d::context::Context::new_metal(
            view,
            density,
            samples,
            alpha,
            font_color,
            ppi,
            canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
            color_space.into()
        ),
        alpha,
    );

    Box::into_raw(Box::new(ctx_2d)) as i64
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_create_2d_context_metal_offscreen(
    width: f32,
    height: f32,
    alpha: bool,
    density: f32,
    samples: usize,
    font_color: i32,
    ppi: f32,
    direction: i32,
    color_space: CanvasColorSpace,
) -> i64 {
    let ctx_2d = CanvasRenderingContext2D::new_metal(
        canvas_2d::context::Context::new_metal_offscreen(
            width,
            height,
            density,
            samples,
            alpha,
            font_color,
            ppi,
            canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
            color_space.into()
        ),
        alpha,
    );

    Box::into_raw(Box::new(ctx_2d)) as i64
}


#[no_mangle]
pub extern "C" fn canvas_native_ios_create_2d_context_metal_device_queue(
    view: *mut c_void,
    device: *mut c_void,
    queue: *mut c_void,
    alpha: bool,
    density: f32,
    samples: usize,
    font_color: i32,
    ppi: f32,
    direction: i32,
    color_space: CanvasColorSpace
) -> i64 {
    let ctx_2d = CanvasRenderingContext2D::new_metal(
        canvas_2d::context::Context::new_metal_device_queue(
            view,
            device,
            queue,
            density,
            samples,
            alpha,
            font_color,
            ppi,
            canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
            color_space.into()
        ),
        alpha,
    );

    Box::into_raw(Box::new(ctx_2d)) as i64
}


/// Retained by the view until the context is released.
struct MetalHandles(*mut c_void, *mut c_void, *mut c_void);
unsafe impl Send for MetalHandles {}

/// Call on the main thread, which reads `layer` and its drawable size: UIKit is main-thread only.
#[no_mangle]
pub extern "C" fn canvas_native_ios_create_2d_context_metal_device_queue_threaded(
    layer: *mut c_void,
    device: *mut c_void,
    queue: *mut c_void,
    width: f32,
    height: f32,
    alpha: bool,
    density: f32,
    samples: usize,
    font_color: i32,
    ppi: f32,
    direction: i32,
    color_space: CanvasColorSpace,
) -> i64 {
    if layer.is_null() || device.is_null() || queue.is_null() {
        return 0;
    }
    let handles = MetalHandles(layer, device, queue);
    let direction =
        canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32);
    let color_space: canvas_core::context_attributes::ColorSpace = color_space.into();
    let context = CanvasRenderingContext2D::new_threaded_with(
        width, height, density, alpha, font_color, ppi, direction, color_space,
        move || {
            // Moved whole: capturing its fields would capture bare pointers, which are not Send.
            let handles = handles;
            let MetalHandles(layer, device, queue) = handles;
            let context = canvas_2d::context::Context::new_metal_layer_device_queue(
                layer, device, queue, density, samples, alpha, font_color, ppi, direction,
                color_space,
            )?;
            Some(CanvasRenderingContext2D::new_metal(context, alpha))
        },
    );
    match context {
        Some(context) => Box::into_raw(Box::new(context)) as i64,
        None => 0,
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRetain(value: *const c_void) -> *const c_void;
    fn CFRelease(value: *const c_void);
}

/// `owner` (the CVMetalTexture) keeps the pixel buffer out of the decoder's pool.
struct MetalFrame {
    texture: *mut c_void,
    owner: *mut c_void,
}

unsafe impl Send for MetalFrame {}

impl Drop for MetalFrame {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.texture);
            if !self.owner.is_null() {
                CFRelease(self.owner);
            }
        }
    }
}

static PENDING_FRAMES: std::sync::Mutex<
    std::collections::VecDeque<(canvas_core::gpu::metal::MetalMarker, MetalFrame)>,
> = std::sync::Mutex::new(std::collections::VecDeque::new());

/// The decoder only has a few buffers.
const MAX_HELD_FRAMES: usize = 2;

fn release_finished_frames() {
    let Ok(mut pending) = PENDING_FRAMES.lock() else { return };
    while let Some((marker, _)) = pending.front() {
        if !marker.is_done() {
            if pending.len() <= MAX_HELD_FRAMES {
                break;
            }
            marker.wait();
        }
        pending.pop_front();
    }
}

/// Takes over `owner` (+1). A threaded context keeps it until the GPU has read the texture.
#[no_mangle]
pub extern "C" fn canvas_native_ios_context_draw_external_metal_texture(
    context: i64,
    mtl_texture: *mut c_void,
    owner: *mut c_void,
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
    if context == 0 || mtl_texture.is_null() {
        if !owner.is_null() {
            unsafe { CFRelease(owner) };
        }
        return false;
    }
    let ctx = unsafe { &mut *(context as *mut CanvasRenderingContext2D) };
    if !ctx.is_threaded() {
        let drawn = draw_image_metal_texture(
            context, mtl_texture, width, height, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
        );
        if !owner.is_null() {
            unsafe { CFRelease(owner) };
        }
        return drawn;
    }

    unsafe { CFRetain(mtl_texture) };
    let frame = MetalFrame { texture: mtl_texture, owner };
    let (w, h) = (width as i32, height as i32);
    let make: canvas_2d::context::recording::ExternalImage =
        Box::new(move |context: &mut canvas_2d::context::Context| {
            let frame = frame;
            let info = unsafe { gpu::mtl::TextureInfo::new(frame.texture as gpu::mtl::Handle) };
            let texture = unsafe {
                gpu::backend_textures::make_mtl((w, h), gpu::Mipmapped::No, &info, "")
            };
            let image = skia_safe::Image::from_texture(
                context.gpu_context()?,
                &texture,
                gpu::SurfaceOrigin::TopLeft,
                skia_safe::ColorType::BGRA8888,
                skia_safe::AlphaType::Premul,
                None,
            )?;
            let release: canvas_2d::context::recording::ExternalRelease =
                Box::new(move |context: &mut canvas_2d::context::Context| {
                    if let Some(ctx) = context.gpu_context() {
                        ctx.flush_and_submit();
                    }
                    let marker = context.metal_context.as_ref().and_then(|m| m.commit_marker());
                    match (marker, PENDING_FRAMES.lock()) {
                        (Some(marker), Ok(mut pending)) => pending.push_back((marker, frame)),
                        _ => {
                            if let Some(ctx) = context.gpu_context() {
                                ctx.flush_submit_and_sync_cpu();
                            }
                            drop(frame);
                        }
                    }
                    release_finished_frames();
                });
            Some((image, release))
        });
    ctx.get_context_mut().draw_external_image(
        w,
        h,
        skia_safe::Rect::from_xywh(sx, sy, s_width, s_height),
        skia_safe::Rect::from_xywh(dx, dy, d_width, d_height),
        make,
    );
    true
}

#[no_mangle]
pub extern "C" fn canvas_native_init_ios_webgpu(
    instance: i64,
    view: *mut c_void,
    width: u32,
    height: u32,
) -> c_longlong {
    // let _ = env_logger::try_init();

    if instance == 0 {
        return 0;
    }

    if let Some(view) = NonNull::new(view) {
        let instance = instance as *mut CanvasWebGPUInstance;
        return unsafe {
            webgpu::gpu_canvas_context::canvas_native_webgpu_context_create(
                instance,
                view.as_ptr(),
                width,
                height,
            ) as i64
        };
    }

    0
}


#[cfg(any(target_os = "macos"))]
#[no_mangle]
pub extern "C" fn canvas_native_init_ios_webgpu_nsview(
    instance: i64,
    view: *mut c_void,
    width: u32,
    height: u32,
) -> c_longlong {
    // let _ = env_logger::try_init();

    if instance == 0 {
        return 0;
    }

    if let Some(view) = NonNull::new(view) {
        let instance = instance as *mut CanvasWebGPUInstance;
        return unsafe {
            webgpu::gpu_canvas_context::canvas_native_webgpu_context_create_nsview(
                instance,
                view.as_ptr(),
                width,
                height,
            ) as i64
        };
    }

    0
}

#[cfg(any(target_os = "ios", target_os = "visionos", target_os = "tvos"))]
#[no_mangle]
pub extern "C" fn canvas_native_resize_ios_webgpu_uiview(
    context: i64,
    view: *mut c_void,
    width: u32,
    height: u32,
) {
    if context == 0 {
        return;
    }

    if let Some(view) = NonNull::new(view) {
        let context = context as *mut webgpu::gpu_canvas_context::CanvasGPUCanvasContext;
        unsafe {
            webgpu::gpu_canvas_context::canvas_native_webgpu_context_resize_uiview(
                context,
                view.as_ptr(),
                width,
                height,
            );
        };
    }
}

fn draw_image_metal_texture(
    context: i64,
    mtl_texture: *mut c_void,
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
    if context == 0 || mtl_texture.is_null() {
        return false;
    }

    let context = context as *mut CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    let info = unsafe { gpu::mtl::TextureInfo::new(mtl_texture as gpu::mtl::Handle) };
    let bt = unsafe {
        gpu::backend_textures::make_mtl(
            (width as i32, height as i32),
            gpu::Mipmapped::No,
            &info,
            "",
        )
    };

    let image_info = skia_safe::ImageInfo::new(
        skia_safe::ISize::new(width as i32, height as i32),
        skia_safe::ColorType::BGRA8888,
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
pub extern "C" fn canvas_native_ios_context_draw_image_dx_dy_with_metal_texture(
    context: i64,
    mtl_texture: *mut c_void,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
) -> bool {
    draw_image_metal_texture(
        context, mtl_texture, width, height, 0.0, 0.0, width, height, dx, dy, width, height,
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_context_draw_image_dx_dy_dw_dh_with_metal_texture(
    context: i64,
    mtl_texture: *mut c_void,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
    d_width: f32,
    d_height: f32,
) -> bool {
    draw_image_metal_texture(
        context, mtl_texture, width, height, 0.0, 0.0, width, height, dx, dy, d_width, d_height,
    )
}

#[no_mangle]
pub extern "C" fn canvas_native_ios_context_draw_image_with_metal_texture(
    context: i64,
    mtl_texture: *mut c_void,
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
    draw_image_metal_texture(
        context, mtl_texture, width, height, sx, sy, s_width, s_height, dx, dy, d_width, d_height,
    )
}
