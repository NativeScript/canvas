use canvas_c::webgpu::gpu::CanvasWebGPUInstance;
use canvas_c::{webgpu, CanvasRenderingContext2D};
use std::ffi::{c_longlong, c_void};
use std::ptr::NonNull;

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
        ),
        alpha,
    );

    Box::into_raw(Box::new(ctx_2d)) as i64
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

#[cfg(any(target_os = "ios"))]
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