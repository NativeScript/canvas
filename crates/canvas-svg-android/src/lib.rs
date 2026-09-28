#![allow(non_snake_case)]

pub use canvas_svg_c::*;

use std::ffi::c_void;

use jni::objects::{JByteBuffer, JClass, JObject, JString};
use jni::sys::{jboolean, jfloat, jint, jlong};
use jni::{JNIEnv, JavaVM};
use ndk::bitmap::AndroidBitmap;

/// This `.so` has no logger of its own; without this, `log::` output is dropped.
#[unsafe(no_mangle)]
pub extern "system" fn JNI_OnLoad(_vm: JavaVM, _reserved: *const c_void) -> jint {
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("canvassvg")
            .with_max_level(log::LevelFilter::Info),
    );
    jni::sys::JNI_VERSION_1_6
}

/// `backend` is a `gpu::Backend` discriminant (0 auto-selects). Returns 0 if no context
/// could be created; the caller then falls back to the bitmap path.
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeGpuCreate(
    env: JNIEnv,
    _: JClass,
    surface: JObject,
    width: jint,
    height: jint,
    backend: jint,
) -> jlong {
    if surface.is_null() {
        return 0;
    }
    let Some(window) = (unsafe {
        ndk::native_window::NativeWindow::from_surface(env.get_native_interface(), surface.as_raw())
    }) else {
        return 0;
    };
    let ptr = window.ptr().as_ptr() as *mut std::ffi::c_void;
    // Must outlive the context, which reuses it after context loss; `nativeGpuDestroy` releases it.
    std::mem::forget(window);
    let gpu = canvas_svg_c::gpu::canvas_native_svg_gpu_create(ptr, width, height, backend);
    if gpu.is_null() {
        release_window(ptr);
    }
    gpu as jlong
}

/// Hands back the `ANativeWindow` reference taken in `nativeGpuCreate`.
fn release_window(ptr: *mut std::ffi::c_void) {
    let Some(ptr) = std::ptr::NonNull::new(ptr.cast()) else {
        return;
    };
    let _ = unsafe { ndk::native_window::NativeWindow::from_ptr(ptr) };
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeGpuBackend(
    _env: JNIEnv,
    _: JClass,
    gpu: jlong,
) -> jint {
    canvas_svg_c::gpu::canvas_native_svg_gpu_backend(gpu as *const _)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeGpuResize(
    _env: JNIEnv,
    _: JClass,
    gpu: jlong,
    width: jint,
    height: jint,
) {
    canvas_svg_c::gpu::canvas_native_svg_gpu_resize(gpu as *mut _, width, height);
}

/// Returns a `gpu::FrameStatus`: 0 presented, 1 skipped, 2 presented after rebuild,
/// 3 lost (fall back to the bitmap).
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeGpuRender(
    _env: JNIEnv,
    _: JClass,
    gpu: jlong,
    document: jlong,
    scale: jfloat,
) -> jint {
    canvas_svg_c::gpu::canvas_native_svg_gpu_render(gpu as *mut _, document as *mut _, scale)
}

/// Simulates a context loss to exercise recovery.
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeGpuDebugLoseContext(
    _env: JNIEnv,
    _: JClass,
    gpu: jlong,
) {
    canvas_svg_c::gpu::canvas_native_svg_gpu_debug_lose_context(gpu as *mut _);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeGpuDestroy(
    _env: JNIEnv,
    _: JClass,
    gpu: jlong,
) {
    // Read the window first: `destroy` frees the surface that holds it.
    let window = canvas_svg_c::gpu::canvas_native_svg_gpu_window(gpu as *const _);
    canvas_svg_c::gpu::canvas_native_svg_gpu_destroy(gpu as *mut _);
    release_window(window);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeRenderDocument(
    env: JNIEnv,
    _: JClass,
    bitmap: JObject,
    document: jlong,
    scale: jfloat,
) {
    if bitmap.is_null() || document == 0 {
        return;
    }
    let document = document as *mut SvgDocument;
    let native_bitmap =
        unsafe { AndroidBitmap::from_jni(env.get_native_interface(), bitmap.as_raw()) };
    if let (Ok(ptr), Ok(info)) = (native_bitmap.lock_pixels(), native_bitmap.get_info()) {
        if !ptr.is_null() {
            let stride = info.stride() as usize;
            let len = stride * info.height() as usize;
            canvas_native_svg_document_render_to_pixels(
                document,
                ptr as *mut u8,
                len,
                info.width() as i32,
                info.height() as i32,
                stride,
                scale,
            );
        }
        let _ = native_bitmap.unlock_pixels();
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeDrawSVG(
    mut env: JNIEnv,
    _: JClass,
    bitmap: JObject,
    _scale: jfloat,
    svg: JString,
) {
    if bitmap.is_null() {
        return;
    }
    if let Ok(svg) = env.get_string(&svg) {
        let svg = svg.to_string_lossy();
        let native_bitmap =
            unsafe { AndroidBitmap::from_jni(env.get_native_interface(), bitmap.as_raw()) };
        if let (Ok(ptr), Ok(info)) = (native_bitmap.lock_pixels(), native_bitmap.get_info()) {
            // Don't `return` here: skipping `unlock_pixels` deadlocks later locks on the main thread.
            if !ptr.is_null() {
                let width = info.width();
                let height = info.height();
                let size = height * info.stride();
                let slice = unsafe {
                    std::slice::from_raw_parts_mut(
                        std::mem::transmute::<*mut c_void, *mut u8>(ptr),
                        size as usize,
                    )
                };

                let info = skia_safe::ImageInfo::new_n32_premul(
                    skia_safe::ISize::new(width as i32, height as i32),
                    None,
                );
                if let Some(mut surface) =
                    skia_safe::surface::surfaces::wrap_pixels(&info, slice, None, None)
                {
                    //surface.canvas().scale((scale, scale));
                    canvas_svg::draw_svg(&mut surface, svg.as_ref())
                }
            }
        }
        let _ = native_bitmap.unlock_pixels();
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeDrawSVGFromPath(
    mut env: JNIEnv,
    _: JClass,
    bitmap: JObject,
    _scale: jfloat,
    path: JString,
) {
    if bitmap.is_null() {
        return;
    }

    if let Ok(path) = env.get_string(&path) {
        let path = path.to_string_lossy();
        let native_bitmap =
            unsafe { AndroidBitmap::from_jni(env.get_native_interface(), bitmap.as_raw()) };
        if let (Ok(ptr), Ok(info)) = (native_bitmap.lock_pixels(), native_bitmap.get_info()) {
            if ptr.is_null() {
                return;
            }
            let width = info.width();
            let height = info.height();
            let size = height * info.stride();
            let slice = unsafe {
                std::slice::from_raw_parts_mut(
                    std::mem::transmute::<*mut c_void, *mut u8>(ptr),
                    size as usize,
                )
            };

            let info = skia_safe::ImageInfo::new_n32_premul(
                skia_safe::ISize::new(width as i32, height as i32),
                None,
            );
            if let Some(mut surface) =
                skia_safe::surface::surfaces::wrap_pixels(&info, slice, None, None)
            {
                // surface.canvas().scale((scale, scale));
                canvas_svg::draw_svg_from_path(&mut surface, path.as_ref())
            }
        }
        let _ = native_bitmap.unlock_pixels();
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeDrawSVGFromBytes(
    mut env: JNIEnv,
    _: JClass,
    bitmap: JObject,
    _scale: jfloat,
    bytes: JByteBuffer,
) {
    if bytes.is_null() {
        return;
    }
    if let (Ok(bytes), Ok(size)) = (
        env.get_direct_buffer_address(&bytes),
        env.get_direct_buffer_capacity(&bytes),
    ) {
        let source = unsafe { std::slice::from_raw_parts_mut(bytes, size) };

        let native_bitmap =
            unsafe { AndroidBitmap::from_jni(env.get_native_interface(), bitmap.as_raw()) };
        if let (Ok(ptr), Ok(info)) = (native_bitmap.lock_pixels(), native_bitmap.get_info()) {
            if ptr.is_null() {
                return;
            }
            let width = info.width();
            let height = info.height();
            let size = height * info.stride();
            let slice = unsafe {
                std::slice::from_raw_parts_mut(
                    std::mem::transmute::<*mut c_void, *mut u8>(ptr),
                    size as usize,
                )
            };

            let info = skia_safe::ImageInfo::new_n32_premul(
                skia_safe::ISize::new(width as i32, height as i32),
                None,
            );
            if let Some(mut surface) =
                skia_safe::surface::surfaces::wrap_pixels(&info, slice, None, None)
            {
                // surface.canvas().scale((scale, scale));
                canvas_svg::draw_svg_from_bytes(&mut surface, source);
            }
        }
        let _ = native_bitmap.unlock_pixels();
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeDrawSVGWithBuffer(
    mut env: JNIEnv,
    _: JClass,
    buffer: JByteBuffer,
    width: jint,
    height: jint,
    _scale: jfloat,
    svg: JString,
) {
    if buffer.is_null() {
        return;
    }
    if let Ok(svg) = env.get_string(&svg) {
        let svg = svg.to_string_lossy();

        if let (Ok(bytes), Ok(size)) = (
            env.get_direct_buffer_address(&buffer),
            env.get_direct_buffer_capacity(&buffer),
        ) {
            let slice = unsafe { std::slice::from_raw_parts_mut(bytes, size) };

            let info = skia_safe::ImageInfo::new_n32_premul(
                skia_safe::ISize::new(width as i32, height as i32),
                None,
            );
            if let Some(mut surface) =
                skia_safe::surface::surfaces::wrap_pixels(&info, slice, None, None)
            {
                canvas_svg::draw_svg(&mut surface, svg.as_ref())
            }
        }
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeDrawSVGFromPathWithBuffer(
    mut env: JNIEnv,
    _: JClass,
    buffer: JByteBuffer,
    width: jint,
    height: jint,
    _scale: jfloat,
    path: JString,
) {
    if buffer.is_null() {
        return;
    }

    if let Ok(path) = env.get_string(&path) {
        let path = path.to_string_lossy();

        if buffer.is_null() {
            return;
        }

        if let (Ok(bytes), Ok(size)) = (
            env.get_direct_buffer_address(&buffer),
            env.get_direct_buffer_capacity(&buffer),
        ) {
            let slice = unsafe { std::slice::from_raw_parts_mut(bytes, size) };

            let info = skia_safe::ImageInfo::new_n32_premul(
                skia_safe::ISize::new(width as i32, height as i32),
                None,
            );
            if let Some(mut surface) =
                skia_safe::surface::surfaces::wrap_pixels(&info, slice, None, None)
            {
                canvas_svg::draw_svg_from_path(&mut surface, path.as_ref())
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeDrawSVGFromBytesWithBuffer(
    mut env: JNIEnv,
    _: JClass,
    buffer: JByteBuffer,
    width: jint,
    height: jint,
    _scale: jfloat,
    bytes: JByteBuffer,
) {
    if bytes.is_null() {
        return;
    }
    if let (Ok(bytes), Ok(size), Ok(dst), Ok(dst_size)) = (
        env.get_direct_buffer_address(&bytes),
        env.get_direct_buffer_capacity(&bytes),
        env.get_direct_buffer_address(&buffer),
        env.get_direct_buffer_capacity(&buffer),
    ) {
        let source = unsafe { std::slice::from_raw_parts_mut(bytes, size) };

        let slice = unsafe { std::slice::from_raw_parts_mut(dst, dst_size) };

        let info = skia_safe::ImageInfo::new_n32_premul(
            skia_safe::ISize::new(width as i32, height as i32),
            None,
        );
        if let Some(mut surface) =
            skia_safe::surface::surfaces::wrap_pixels(&info, slice, None, None)
        {
            // surface.canvas().scale((scale, scale));
            canvas_svg::draw_svg_from_bytes(&mut surface, source);
        }
    }
}

/// The render thread owns the GPU context; the UI thread never touches it. 0 means no GPU
/// path, stay on the bitmap.
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeRenderThreadCreate(
    env: JNIEnv,
    _: JClass,
    surface: JObject,
    width: jint,
    height: jint,
    backend: jint,
) -> jlong {
    if surface.is_null() {
        return 0;
    }
    let Some(window) = (unsafe {
        ndk::native_window::NativeWindow::from_surface(env.get_native_interface(), surface.as_raw())
    }) else {
        return 0;
    };
    let ptr = window.ptr().as_ptr() as *mut std::ffi::c_void;
    // Held until `nativeRenderThreadDestroy` has joined the thread.
    std::mem::forget(window);
    let thread = canvas_svg_c::gpu::canvas_native_svg_render_thread_create(ptr, width, height, backend);
    if thread.is_null() {
        release_window(ptr);
        return 0;
    }
    Box::into_raw(Box::new(ThreadHandle { thread, window: ptr })) as jlong
}

struct ThreadHandle {
    thread: *mut canvas_svg_c::gpu::thread::RenderThread,
    window: *mut std::ffi::c_void,
}

/// Records on the calling thread; rasterizing happens on the render thread without blocking.
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeRenderThreadCommit(
    _env: JNIEnv,
    _: JClass,
    handle: jlong,
    document: jlong,
    width: jint,
    height: jint,
    scale: jfloat,
) -> jboolean {
    if handle == 0 {
        return 0;
    }
    let handle = unsafe { &*(handle as *const ThreadHandle) };
    canvas_svg_c::gpu::canvas_native_svg_render_thread_commit(
        handle.thread,
        document as *mut _,
        width,
        height,
        scale,
    ) as jboolean
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeRenderThreadResize(
    _env: JNIEnv,
    _: JClass,
    handle: jlong,
    width: jint,
    height: jint,
) {
    if handle == 0 {
        return;
    }
    let handle = unsafe { &*(handle as *const ThreadHandle) };
    canvas_svg_c::gpu::canvas_native_svg_render_thread_resize(handle.thread, width, height);
}

/// The `FrameStatus` of the last present, or -1 if nothing has presented since the last call.
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeRenderThreadStatus(
    _env: JNIEnv,
    _: JClass,
    handle: jlong,
) -> jint {
    if handle == 0 {
        return -1;
    }
    let handle = unsafe { &*(handle as *const ThreadHandle) };
    canvas_svg_c::gpu::canvas_native_svg_render_thread_status(handle.thread)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_nativescript_canvas_svg_NSCSVG_nativeRenderThreadDestroy(
    _env: JNIEnv,
    _: JClass,
    handle: jlong,
) {
    if handle == 0 {
        return;
    }
    let handle = unsafe { Box::from_raw(handle as *mut ThreadHandle) };
    // Blocks until joined and the surface is gone; only then may the window be released.
    canvas_svg_c::gpu::canvas_native_svg_render_thread_destroy(handle.thread);
    release_window(handle.window);
}
