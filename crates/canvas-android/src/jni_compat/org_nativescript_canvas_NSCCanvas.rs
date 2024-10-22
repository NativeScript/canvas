use jni::objects::{JClass, JIntArray, JObject};
use jni::sys::{jboolean, jfloat, jint, jintArray, jlong, jobject, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use ndk::native_window::NativeWindow;
use raw_window_handle::RawWindowHandle;
use skia_safe::{AlphaType, ColorType, ISize, ImageInfo, Rect};
use std::ffi::c_void;
use std::ptr;
use std::ptr::NonNull;

use canvas_2d::context::paths::path::Path;
use canvas_c::webgpu::gpu::CanvasWebGPUInstance;
use canvas_c::WebGLState;
use canvas_core::context_attributes::PowerPreference;
use canvas_core::gpu::gl::GLContext;
use canvas_core::gpu::vulkan::VulkanContext;

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
                        let buf = std::slice::from_raw_parts_mut(
                            elements.as_ptr() as *mut jint,
                            size * std::mem::size_of::<i32>(),
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
    unsafe {
        let interface = env.get_native_interface();
        if let Some(window) = NativeWindow::from_surface(interface, surface) {
            let ptr = window.ptr().as_ptr();
            let instance: *mut CanvasWebGPUInstance = instance as _;
            let ret = canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_create(
                instance,
                ptr as *mut c_void,
                width as u32,
                height as u32,
            );

            return ret as jlong;
        }
    }
    0
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
    unsafe {
        let interface = env.get_native_interface();
        if let Some(window) = NativeWindow::from_surface(interface, surface) {
            let ptr = window.ptr().as_ptr();
            let context: *mut canvas_c::webgpu::gpu_canvas_context::CanvasGPUCanvasContext =
                context as _;
            // #[cfg(any(target_os = "android"))]
            canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_resize(
                context,
                ptr as *mut c_void,
                width as u32,
                height as u32,
            );
        }
    }
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
) -> jlong {
    unsafe {
        let interface = env.get_native_interface();
        if let Some(window) = NativeWindow::from_surface(interface, surface) {
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
) -> jlong {
    unsafe {
        let interface = env.get_native_interface();
        if let Some(window) = NativeWindow::from_surface(interface, surface) {
            if version == 2 && !GLContext::has_gl2support() {
                return 0;
            }
            if let Ok(power_preference) = PowerPreference::try_from(power_preference) {
                let context = canvas_c::canvas_native_webgl_create(
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

                return Box::into_raw(Box::new(context)) as jlong;
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
) -> jlong {
    if version == 2 && !GLContext::has_gl2support() {
        return 0;
    }

    if let Ok(power_preference) = PowerPreference::try_from(power_preference) {
        let context = canvas_c::canvas_native_webgl_create_no_window(
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
        return Box::into_raw(Box::new(context)) as jlong;
    }
    0
}

#[no_mangle]
pub extern "system" fn nativeCreate2DContext(
    env: JNIEnv,
    _: JClass,
    surface: jobject,
    alpha: jboolean,
    density: jfloat,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    unsafe {
        if let Some(window) = NativeWindow::from_surface(env.get_native_interface(), surface) {
            let width = window.width();
            let height = window.height();

            let ctx_2d = canvas_c::CanvasRenderingContext2D::new_gl(
                canvas_2d::context::Context::new_gl(
                    window.ptr().as_ptr() as _,
                    width as f32,
                    height as f32,
                    density,
                    alpha == JNI_TRUE,
                    font_color,
                    ppi,
                    canvas_2d::context::text_styles::text_direction::TextDirection::from(
                        direction as u32,
                    ),
                ),
                alpha == JNI_TRUE,
            );

            drop(env);
            return Box::into_raw(Box::new(ctx_2d)) as jlong;
        }
    }

    0
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
    let context = unsafe { &mut *context };
    unsafe {
        if let Some(window) = NativeWindow::from_surface(env.get_native_interface(), surface) {
            context.get_inner_mut().set_window_surface(
                window.width(),
                window.height(),
                NonNull::new(window.ptr().as_ptr() as _).unwrap(),
            );
            context.get_inner().make_current();
            drop(env);
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

    unsafe {
        if let Some(window) = NativeWindow::from_surface(env.get_native_interface(), surface) {
            {
                let context = context.get_context_mut();
                let alpha = !context.surface_data().is_opaque();
                if let Some(context) = context.gl_context.as_mut() {
                    let mut attr = canvas_core::context_attributes::ContextAttributes::new(
                        alpha,
                        false,
                        false,
                        false,
                        PowerPreference::Default,
                        true,
                        false,
                        false,
                        false,
                        false,
                        true,
                    );

                    let handle = raw_window_handle::AndroidNdkWindowHandle::new(
                        NonNull::new(window.ptr().as_ptr() as _).unwrap(),
                    );
                    let handle = RawWindowHandle::AndroidNdk(handle);
                    context.set_window_surface(&mut attr, width, height, handle);
                    context.make_current();
                }
            }

            let width = width as f32;
            let height = height as f32;

            context.resize(width, height)
        }
        drop(env);
    }
}

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
    let context = unsafe { &mut *context };
    context.get_inner().make_current();
    context.get_inner_mut().resize_pbuffer(width, height);
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
    let context = context as *mut WebGLState;
    let _ = unsafe { Box::from_raw(context) };
}

#[no_mangle]
pub extern "system" fn nativeReleaseWebGLNormal(_env: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }
    let context = context as *mut WebGLState;
    let _ = unsafe { Box::from_raw(context) };
}

#[no_mangle]
pub extern "system" fn nativeMakeWebGLCurrent(gl_context: jlong) -> jboolean {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *mut WebGLState;
    let gl_context = unsafe { &*gl_context };
    if gl_context.get_inner().make_current() {
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
    if gl_context.get_inner().make_current() {
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

    log::info!(
        "data {:?}",
        context.get_context_mut().as_data_url("image/png", 100)
    )
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
    let context = unsafe { &mut *context };

    unsafe {
        crate::utils::image::bitmap_handler(
            &env,
            bitmap,
            Box::new(move |cb| {
                if let Some((image_data, info)) = cb {
                    context.get_inner().make_current();
                    let mut buf = vec![0u8; (info.width() * info.height() * 4) as usize];
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

                let mut surface =
                    skia_safe::surfaces::wrap_pixels(&info, image_data, None, None).unwrap();
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
                let context = context.get_context_mut();
                context.draw_on_surface(&mut surface);
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
