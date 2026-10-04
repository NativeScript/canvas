use canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle;
use canvas_2d::context::paths::path::Path;
use canvas_c::webgpu::gpu::CanvasWebGPUInstance;
use canvas_c::WebGLState;
use canvas_core::context_attributes::{ColorSpace, PowerPreference};
use canvas_core::gpu::gl::GLContext;
use jni::objects::{JClass, JIntArray, JObject};
use jni::sys::{jboolean, jfloat, jint, jlong, jobject, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use ndk::native_window::NativeWindow;
use raw_window_handle::RawWindowHandle;
use skia_safe::{AlphaType, ColorType, ISize, ImageInfo, Rect};
use std::ffi::c_void;
use std::ptr;
use std::ptr::NonNull;

use canvas_c::offscreen::android::{detach_2d_surface, update_2d_surface};

fn to_raw_window_handler(window: &NativeWindow) -> RawWindowHandle {
    let handle = raw_window_handle::AndroidNdkWindowHandle::new(
        ptr::NonNull::new(window.ptr().as_ptr() as *mut c_void).unwrap(),
    );
    RawWindowHandle::AndroidNdk(handle)
}

#[no_mangle]
pub extern "system" fn nativeGetVulkanVersion(mut env: JNIEnv, _: JClass, array: JIntArray) {
    #[cfg(feature = "vulkan")]
    {
        unsafe {
            if let Some(version) = VulkanContext::version() {
                if let Ok(elements) =
                    env.get_array_elements_critical(&array, jni::objects::ReleaseMode::CopyBack)
                {
                    let size = elements.len();
                    if size >= 3 {
                        // Length is in elements, not bytes.
                        let buf = std::slice::from_raw_parts_mut(
                            elements.as_ptr() as *mut jint,
                            size,
                        );
                        buf[0] = version.0 as jint;
                        buf[1] = version.1 as jint;
                        buf[2] = version.2 as jint;
                    }
                    drop(elements);
                    return;
                }

                let _ = env.set_int_array_region(
                    &array,
                    0,
                    &[version.0 as jint, version.1 as jint, version.2 as jint],
                );
            }
        }
    }
}

#[no_mangle]
pub extern "system" fn nativeInitWebGPU(
    env: JNIEnv,
    _: JClass,
    instance: jlong,
    surface: jobject,
    width: jint,
    height: jint,
) -> jlong {
    if instance == 0 || width <= 0 || height <= 0 {
        return 0;
    }

    unsafe {
        let window = if surface.is_null() {
            None
        } else {
            NativeWindow::from_surface(env.get_native_interface(), surface)
        };
        let ptr = window
            .as_ref()
            .map_or(ptr::null_mut(), |window| window.ptr().as_ptr() as *mut c_void);
        canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_create(
            instance as *mut CanvasWebGPUInstance,
            ptr,
            width as u32,
            height as u32,
        ) as jlong
    }
}

#[no_mangle]
pub extern "system" fn nativeResizeWebGPU(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    surface: jobject,
    width: jint,
    height: jint,
) {
    if context == 0 || width <= 0 || height <= 0 {
        return;
    }

    unsafe {
        let interface = env.get_native_interface();
        if let Some(window) = NativeWindow::from_surface(interface, surface) {
            let Some(ptr) = NonNull::new(window.ptr().as_ptr() as *mut c_void) else {
                return;
            };
            let context: *mut canvas_c::webgpu::gpu_canvas_context::CanvasGPUCanvasContext =
                context as _;
            #[cfg(any(target_os = "android"))]
            canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_resize(
                context,
                ptr.as_ptr(),
                width as u32,
                height as u32,
            );
        }
    }
}

#[no_mangle]
pub extern "system" fn nativeReleaseWebGPU(_: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }

    unsafe {
        let context: *const canvas_c::webgpu::gpu_canvas_context::CanvasGPUCanvasContext =
            context as _;
        canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_release(context);
    }
}

#[no_mangle]
pub extern "system" fn nativeDetachWebGPUSurface(_: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }

    unsafe {
        canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_detach_surface(
            context as _,
        );
    }
}

#[no_mangle]
pub extern "system" fn nativeDetach2DSurface(_: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }
    let context = unsafe { &mut *(context as *mut canvas_c::CanvasRenderingContext2D) };
    if let Some(target) = context.render_target() {
        // The window goes away when this returns.
        target.sync(detach_2d_surface);
        return;
    }
    detach_2d_surface(context);
}

// #[cfg(feature = "vulkan")]
#[no_mangle]
pub extern "system" fn nativeCreate2dContextVulkan(
    env: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
    surface: jobject,
    alpha: jboolean,
    density: jfloat,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
    color_space: jint,
) -> jlong {
    unsafe {
        let interface = env.get_native_interface();
        if let Some(window) = NativeWindow::from_surface(interface, surface) {
            let color_space = match color_space {
                1 => ColorSpace::P3,
                _ => ColorSpace::Srgb
            };
            let ctx_2d = canvas_c::CanvasRenderingContext2D::new_vulkan(
                canvas_2d::context::Context::new_vulkan(
                    width as f32,
                    height as f32,
                    window.ptr().as_ptr() as *mut c_void,
                    density,
                    alpha == JNI_TRUE,
                    font_color,
                    ppi,
                    direction as u8,
                    color_space
                ),
                alpha == JNI_TRUE,
            );
            return Box::into_raw(Box::new(ctx_2d)) as jlong;
        }
    }
    0
}

#[no_mangle]
pub extern "system" fn nativeInitWebGL(
    env: JNIEnv,
    _: JClass,
    surface: jobject,
    alpha: jboolean,
    antialias: jboolean,
    depth: jboolean,
    fail_if_major_performance_caveat: jboolean,
    power_preference: jint,
    premultiplied_alpha: jboolean,
    preserve_drawing_buffer: jboolean,
    stencil: jboolean,
    desynchronized: jboolean,
    xr_compatible: jboolean,
    version: jint,
    threaded: jboolean,
) -> jlong {
    unsafe {
        let interface = env.get_native_interface();
        if let Some(window) = NativeWindow::from_surface(interface, surface) {
            if window.width() <= 0 || window.height() <= 0 {
                return 0;
            }
            if version == 2 && !GLContext::has_gl2support() {
                return 0;
            }
            if let Ok(power_preference) = PowerPreference::try_from(power_preference) {
                let create = if threaded == JNI_TRUE {
                    canvas_c::canvas_native_webgl_create_threaded
                } else {
                    canvas_c::canvas_native_webgl_create
                };
                let context = create(
                    window.ptr().as_ptr() as _,
                    window.width(),
                    window.height(),
                    version as i32,
                    alpha == JNI_TRUE,
                    antialias == JNI_TRUE,
                    depth == JNI_TRUE,
                    fail_if_major_performance_caveat == JNI_TRUE,
                    power_preference.into(),
                    premultiplied_alpha == JNI_TRUE,
                    preserve_drawing_buffer == JNI_TRUE,
                    stencil == JNI_TRUE,
                    desynchronized == JNI_TRUE,
                    xr_compatible == JNI_TRUE,
                );

                drop(env);

                if context.is_null() {
                    return 0;
                }

                return context as jlong;
            }
        }
    }
    0
}

#[no_mangle]
pub extern "system" fn nativeInitWebGLNoSurface(
    _: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
    alpha: jboolean,
    antialias: jboolean,
    depth: jboolean,
    fail_if_major_performance_caveat: jboolean,
    power_preference: jint,
    premultiplied_alpha: jboolean,
    preserve_drawing_buffer: jboolean,
    stencil: jboolean,
    desynchronized: jboolean,
    xr_compatible: jboolean,
    version: jint,
    threaded: jboolean,
) -> jlong {
    if version == 2 && !GLContext::has_gl2support() {
        return 0;
    }

    let width = width.max(1);
    let height = height.max(1);

    if let Ok(power_preference) = PowerPreference::try_from(power_preference) {
        let create = if threaded == JNI_TRUE {
            canvas_c::canvas_native_webgl_create_no_window_threaded
        } else {
            canvas_c::canvas_native_webgl_create_no_window
        };
        let context = create(
            width,
            height,
            version as i32,
            alpha == JNI_TRUE,
            antialias == JNI_TRUE,
            depth == JNI_TRUE,
            fail_if_major_performance_caveat == JNI_TRUE,
            power_preference.into(),
            premultiplied_alpha == JNI_TRUE,
            preserve_drawing_buffer == JNI_TRUE,
            stencil == JNI_TRUE,
            desynchronized == JNI_TRUE,
            xr_compatible == JNI_TRUE,
            false,
        );

        if context.is_null() {
            return 0;
        }
        return context as jlong;
    }
    0
}

#[no_mangle]
pub extern "system" fn nativeCreate2DContext(
    env: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
    surface: jobject,
    alpha: jboolean,
    density: jfloat,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
    color_space: jint,
) -> jlong {
    unsafe {
        let (view_ptr, w, h, cs) = if surface.is_null() {
            (ptr::null_mut(), width as f32, height as f32, ColorSpace::Srgb)
        } else if let Some(window) = NativeWindow::from_surface(env.get_native_interface(), surface) {
            let cs = match color_space { 1 => ColorSpace::P3, _ => ColorSpace::Srgb };
            (window.ptr().as_ptr() as *mut c_void, window.width() as f32, window.height() as f32, cs)
        } else {
            (ptr::null_mut(), width as f32, height as f32, ColorSpace::Srgb)
        };

        let context = match canvas_2d::context::Context::new_gl(
            view_ptr,
            w,
            h,
            density,
            alpha == JNI_TRUE,
            font_color,
            ppi,
            canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
            cs,
        ) {
            Some(ctx) => ctx,
            None => {
                drop(env);
                return 0;
            }
        };

        let ctx_2d = canvas_c::CanvasRenderingContext2D::new_gl(context, alpha == JNI_TRUE);
        drop(env);
        Box::into_raw(Box::new(ctx_2d)) as jlong
    }
}

#[no_mangle]
pub extern "system" fn nativeCreate2DContextThreaded(
    env: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
    surface: jobject,
    alpha: jboolean,
    density: jfloat,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
    color_space: jint,
) -> jlong {
    let window = if surface.is_null() {
        None
    } else {
        unsafe { NativeWindow::from_surface(env.get_native_interface(), surface) }
    };
    drop(env);
    let (w, h, cs) = match window.as_ref() {
        Some(window) => (
            window.width() as f32,
            window.height() as f32,
            match color_space {
                1 => ColorSpace::P3,
                _ => ColorSpace::Srgb,
            },
        ),
        None => (width as f32, height as f32, ColorSpace::Srgb),
    };
    let window = window.map(SendWindow);
    let alpha = alpha == JNI_TRUE;
    let direction =
        canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32);

    let context = canvas_c::CanvasRenderingContext2D::new_threaded_with(
        w, h, density, alpha, font_color, ppi, direction, cs,
        move || {
            let view = window
                .as_ref()
                .map(|window| window.0.ptr().as_ptr() as *mut c_void)
                .unwrap_or(ptr::null_mut());
            let context = canvas_2d::context::Context::new_gl(
                view, w, h, density, alpha, font_color, ppi, direction, cs,
            )?;
            // EGL holds its own reference to the window from here on.
            drop(window);
            Some(canvas_c::CanvasRenderingContext2D::new_gl(context, alpha))
        },
    );
    match context {
        Some(context) => Box::into_raw(Box::new(context)) as jlong,
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn nativeUpdateWebGLSurface(
    env: JNIEnv,
    _: JClass,
    surface: jobject,
    context: jlong,
) {
    if context == 0 {
        return;
    }
    let context = context as *mut WebGLState;
    let context = unsafe { &*context };
    unsafe {
        if let Some(window) = NativeWindow::from_surface(env.get_native_interface(), surface) {
            drop(env);
            let (width, height) = (window.width(), window.height());
            // Queued, not waited on: this runs from the view's surface callbacks, and blocking them
            // on a thread that may be waiting for a buffer from this window can deadlock its queue.
            // `window` holds a reference until the job is done with it.
            context.post(move |state| {
                // NativeWindow::ptr() is NonNull<ANativeWindow>; the as *mut c_void cast
                // preserves non-nullness, but guard defensively to avoid a panic.
                let Some(nn_ptr) = NonNull::new(window.ptr().as_ptr() as _) else {
                    return;
                };
                state.set_window_surface(width, height, nn_ptr);
                state.make_current();
            });
        }
    }
}

#[no_mangle]
pub extern "system" fn nativeUpdate2DSurface(
    env: JNIEnv,
    _: JClass,
    surface: jobject,
    width: jint,
    height: jint,
    context: jlong,
) {
    if context == 0 {
        return;
    }
    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    let Some(window) = (unsafe { NativeWindow::from_surface(env.get_native_interface(), surface) })
    else {
        return;
    };
    drop(env);

    if let Some(target) = context.render_target() {
        // Not waited on: blocking this callback on the render thread can deadlock the buffer queue.
        let window = SendWindow(window);
        target.post(move |real| {
            let window = window;
            update_2d_surface(real, window.0.ptr().cast(), width, height, true)
        });
        context.resize(width as f32, height as f32);
        return;
    }
    update_2d_surface(context, window.ptr().cast(), width, height, true);
}

struct SendWindow(NativeWindow);
unsafe impl Send for SendWindow {}

fn native_update_2d_surface_no_surface(width: jint, height: jint, context: jlong) {
    if context == 0 {
        return;
    }
    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();

    context.resize(width as f32, height as f32)
}

#[no_mangle]
pub extern "system" fn nativeUpdate2DSurfaceNoSurface(width: jint, height: jint, context: jlong) {
    native_update_2d_surface_no_surface(width, height, context)
}

#[no_mangle]
pub extern "system" fn nativeUpdate2DSurfaceNoSurfaceNormal(
    _env: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
    context: jlong,
) {
    native_update_2d_surface_no_surface(width, height, context)
}

fn native_update_gl_no_surface(width: jint, height: jint, context: jlong) {
    if context == 0 {
        return;
    }
    let context = context as *mut WebGLState;
    let context = unsafe { &*context };
    // Waited on: when the window is going away this is how the context lets go of it.
    context.detach(|state| {
        state.make_current();
        state.resize_pbuffer(width, height);
    });
}

#[no_mangle]
pub extern "system" fn nativeUpdateGLNoSurface(width: jint, height: jint, context: jlong) {
    native_update_gl_no_surface(width, height, context)
}

#[no_mangle]
pub extern "system" fn nativeUpdateWebGLNoSurfaceNormal(
    _env: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
    context: jlong,
) {
    native_update_gl_no_surface(width, height, context)
}

#[no_mangle]
pub extern "system" fn nativeReleaseWebGL(context: jlong) {
    if context == 0 {
        return;
    }
    canvas_c::canvas_native_webgl_state_destroy(context as *mut WebGLState);
}

#[no_mangle]
pub extern "system" fn nativeReleaseWebGLNormal(_env: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }
    canvas_c::canvas_native_webgl_state_destroy(context as *mut WebGLState);
}

#[no_mangle]
pub extern "system" fn nativeRelease2DContext(context: jlong) {
    if context == 0 {
        return;
    }
    canvas_c::canvas_native_context_release(context as *mut canvas_c::CanvasRenderingContext2D);
}

#[no_mangle]
pub extern "system" fn nativeRelease2DContextNormal(_env: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }
    canvas_c::canvas_native_context_release(context as *mut canvas_c::CanvasRenderingContext2D);
}

#[no_mangle]
pub extern "system" fn nativeMakeWebGLCurrent(gl_context: jlong) -> jboolean {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *mut WebGLState;
    let gl_context = unsafe { &*gl_context };
    // A threaded context is made current on its own thread, where everything that uses it runs.
    if gl_context.sync(|state| state.make_current()) {
        return JNI_TRUE;
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn nativeMakeWebGLCurrentNormal(
    _env: JNIEnv,
    _: JClass,
    gl_context: jlong,
) -> jboolean {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *mut WebGLState;
    let gl_context = unsafe { &*gl_context };
    // A threaded context is made current on its own thread, where everything that uses it runs.
    if gl_context.sync(|state| state.make_current()) {
        return JNI_TRUE;
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn nativeContext2DTest(context: jlong) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    //  context.make_current();
    {
        let ctx = context.get_context_mut();
        ctx.set_fill_style_with_color("red");
        ctx.fill_rect_xywh(0., 0., 300., 300.);
    }
    context.render();
}

#[no_mangle]
pub extern "system" fn nativeContext2DTestNormal(_env: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    {
        let ctx = context.get_context_mut();
        ctx.set_fill_style_with_color("red");
        ctx.fill_rect_xywh(0., 0., 300., 300.);
    }
    context.render();
}

#[no_mangle]
pub extern "system" fn nativeContext2DPathTest(context: jlong) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    {
        let ctx = context.get_context_mut();

        // Create path
        let mut region = Path::default();
        region.move_to(30f32, 90f32);
        region.line_to(110f32, 20f32);
        region.line_to(240f32, 130f32);
        region.line_to(60f32, 130f32);
        region.line_to(190f32, 20f32);
        region.line_to(270f32, 90f32);
        region.close_path();

        // Fill path
        ctx.set_fill_style_with_color("green");
        ctx.fill_rule(
            Some(&mut region),
            canvas_2d::context::drawing_paths::fill_rule::FillRule::EvenOdd,
        );
    }
    context.render();
}

#[no_mangle]
pub extern "system" fn nativeContext2DPathTestNormal(_env: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    {
        let ctx = context.get_context_mut();

        // Create path
        let mut region = Path::default();
        region.move_to(30f32, 90f32);
        region.line_to(110f32, 20f32);
        region.line_to(240f32, 130f32);
        region.line_to(60f32, 130f32);
        region.line_to(190f32, 20f32);
        region.line_to(270f32, 90f32);
        region.close_path();

        // Fill path
        ctx.set_fill_style_with_color("green");
        ctx.fill_rule(
            Some(&mut region),
            canvas_2d::context::drawing_paths::fill_rule::FillRule::EvenOdd,
        );
    }
    context.render();
}

#[no_mangle]
pub extern "system" fn nativeContext2DConicTest(_env: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    {
        let ctx = context.get_context_mut();

        let (width, height) = ctx.dimensions();
        let mut gradient = ctx.create_conic_gradient(90., width / 2., height / 2.);
        gradient.add_color_stop_str(0., "red");
        gradient.add_color_stop_str(0.25, "orange");
        gradient.add_color_stop_str(0.5, "yellow");
        gradient.add_color_stop_str(0.75, "green");
        gradient.add_color_stop_str(1., "blue");
        ctx.set_fill_style(PaintStyle::Gradient(gradient));
        ctx.fill_rect_xywh(20., 20., width, height);
    }
    context.render();
}

#[no_mangle]
pub extern "system" fn nativeContext2DRender(context: jlong) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.render();
}

#[no_mangle]
pub extern "system" fn nativeWriteCurrentWebGLContextToBitmap(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
) {
    if context == 0 {
        return;
    }

    let context = context as *mut WebGLState;
    let context = unsafe { &*context };

    unsafe {
        crate::utils::image::bitmap_handler(
            &env,
            bitmap,
            Box::new(move |cb| {
                if let Some((image_data, info)) = cb {
                    // The bitmap stays locked while this waits for the read.
                    context.sync(|state| {
                        state.make_current();
                        // Use checked arithmetic — width/height are u32 and can overflow on multiply.
                        let buf_size = (info.width() as usize)
                            .checked_mul(info.height() as usize)
                            .and_then(|n| n.checked_mul(4));
                        let Some(buf_size) = buf_size else { return };
                        let mut buf = vec![0u8; buf_size];
                        gl_bindings::Flush();
                        gl_bindings::ReadPixels(
                            0,
                            0,
                            info.width() as i32,
                            info.height() as i32,
                            gl_bindings::RGBA as std::os::raw::c_uint,
                            gl_bindings::UNSIGNED_BYTE as std::os::raw::c_uint,
                            buf.as_mut_ptr() as *mut c_void,
                        );
                        image_data.copy_from_slice(buf.as_slice());
                    });
                }
            }),
        )
    }

    drop(env);
}

#[no_mangle]
pub extern "system" fn nativeCustomWithBitmapFlush(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
) {
    if context == 0 {
        return;
    }
    crate::utils::image::bitmap_handler(
        &env,
        bitmap,
        Box::new(move |cb| {
            if let Some((image_data, image_info)) = cb {
                let mut ct = ColorType::RGBA8888;
                #[allow(deprecated)]
                use ndk::bitmap::BitmapFormat::RGBA_4444;

                match image_info.format() {
                    ndk::bitmap::BitmapFormat::RGB_565 => {
                        ct = ColorType::RGB565;
                    }
                    RGBA_4444 => {
                        ct = ColorType::ARGB4444;
                    }
                    _ => {}
                }

                let info = ImageInfo::new(
                    ISize::new(image_info.width() as i32, image_info.height() as i32),
                    ct,
                    AlphaType::Premul,
                    None,
                );
                let context = context as *mut canvas_c::CanvasRenderingContext2D;
                let context = unsafe { &mut *context };

                let Some(mut surface) = skia_safe::surfaces::wrap_pixels(&info, image_data, None, None) else {
                    return;
                };
                let canvas = surface.canvas();
                let mut paint = skia_safe::Paint::default();
                paint.set_anti_alias(true);
                paint.set_style(skia_safe::PaintStyle::Fill);
                paint.set_blend_mode(skia_safe::BlendMode::Clear);
                canvas.draw_rect(
                    Rect::from_xywh(
                        0f32,
                        0f32,
                        image_info.width() as f32,
                        image_info.height() as f32,
                    ),
                    &paint,
                );
                if context.is_threaded() {
                    if let Some(image) = context.image() {
                        surface.canvas().draw_image(&image, (0., 0.), None);
                    }
                } else {
                    context.get_context_mut().draw_on_surface(&mut surface);
                }
            }
        }),
    );

    drop(env);
}

#[no_mangle]
pub extern "system" fn nativeWebGLC2DRender(
    _env: JNIEnv,
    _: JClass,
    gl_context: jlong,
    context: jlong,
    internal_format: jint,
    format: jint,
) {
    if gl_context == 0 || context == 0 {
        return;
    }

    let state = gl_context as *mut WebGLState;

    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    {
        let state = unsafe { &mut *state };
        canvas_c::impl_test::draw_image_space_test(state, context, internal_format, format);
        state.get_inner_mut().swap_buffers();
    }

    let _ = unsafe { Box::from_raw(state) };
}

#[no_mangle]
pub extern "system" fn nativeContext2DSetRenderFunc(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    render: JObject,
) {
    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };
    let context = context.get_context_mut();
    if let (Ok(jvm), Ok(render)) = (env.get_java_vm(), env.new_global_ref(render)) {
        context.cpu_context = Some(canvas_core::cpu::CPUContext::new(jvm, render));
    }
}

#[no_mangle]
pub extern "system" fn nativeContext2DClearRenderFunc(_env: JNIEnv, _: JClass, context: jlong) {
    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };
    let context = context.get_context_mut();
    context.cpu_context = None;
}
