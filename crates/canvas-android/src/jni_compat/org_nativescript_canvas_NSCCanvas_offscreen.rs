use std::ffi::{c_void, CString};

use canvas_c::offscreen::android::{canvas_native_offscreen_surface_set_window, canvas_native_offscreen_surface_window_destroyed};
use canvas_c::offscreen::*;
use canvas_c::CanvasColorSpace;
use jni::objects::{GlobalRef, JClass, JObject, JString, JValue};
use jni::sys::{jfloat, jint, jlong, jobject, jstring};
use jni::{JNIEnv, NativeMethod};
use ndk::native_window::NativeWindow;

fn surface(ptr: jlong) -> *const CanvasOffscreenSurface {
    ptr as *const CanvasOffscreenSurface
}

pub extern "system" fn nativeOffscreenSurfaceCreate(
    _: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
    density: jfloat,
    ppi: jfloat,
    direction: jint,
    color_space: jint,
) -> jlong {
    let color_space = if color_space == 1 { CanvasColorSpace::P3 } else { CanvasColorSpace::Srgb };
    canvas_native_offscreen_surface_create_android(
        width.max(1) as u32,
        height.max(1) as u32,
        density,
        ppi,
        direction as u32,
        color_space,
    ) as jlong
}

pub extern "system" fn nativeOffscreenSurfaceSetWindow(env: JNIEnv, _: JClass, ptr: jlong, window: jobject) {
    if ptr == 0 || window.is_null() {
        return;
    }
    if let Some(window) = unsafe { NativeWindow::from_surface(env.get_native_interface(), window) } {
        unsafe { canvas_native_offscreen_surface_set_window(surface(ptr), window.ptr().as_ptr() as *mut c_void) };
    }
}

pub extern "system" fn nativeOffscreenSurfaceWindowDestroyed(_: JNIEnv, _: JClass, ptr: jlong) {
    unsafe { canvas_native_offscreen_surface_window_destroyed(surface(ptr)) };
}

pub extern "system" fn nativeOffscreenSurfaceDetachView(_: JNIEnv, _: JClass, ptr: jlong) {
    unsafe { canvas_native_offscreen_surface_detach_view(surface(ptr)) };
}

pub extern "system" fn nativeOffscreenSurfaceReference(_: JNIEnv, _: JClass, ptr: jlong) -> jlong {
    unsafe { canvas_native_offscreen_surface_reference(surface(ptr)) };
    ptr
}

pub extern "system" fn nativeOffscreenSurfaceRelease(_: JNIEnv, _: JClass, ptr: jlong) {
    unsafe { canvas_native_offscreen_surface_release(surface(ptr)) };
}

pub extern "system" fn nativeOffscreenSurfaceToDataURL(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    format: JString,
    quality: jint,
) -> jstring {
    let format: String = env.get_string(&format).map(Into::into).unwrap_or_else(|_| "image/png".into());
    let Ok(format) = CString::new(format) else {
        return std::ptr::null_mut();
    };
    let url = unsafe { canvas_native_offscreen_surface_to_data_url(surface(ptr), format.as_ptr(), quality.max(0) as u32) };
    if url.is_null() {
        return std::ptr::null_mut();
    }
    let url = unsafe { CString::from_raw(url) };
    env.new_string(url.to_string_lossy()).map_or(std::ptr::null_mut(), |value| value.into_raw())
}

/// Called from the owning JS thread, which is attached.
struct Sink(GlobalRef);

extern "C" fn sink_call(data: *mut c_void, event: CanvasOffscreenEvent, a: u32, b: u32) {
    let sink = unsafe { &*(data as *const Sink) };
    let Some(vm) = crate::JVM.get() else { return };
    let Ok(mut env) = vm.attach_current_thread() else { return };
    let result = env.call_method(
        sink.0.as_obj(),
        "onEvent",
        "(III)V",
        &[JValue::Int(event as jint), JValue::Int(a as jint), JValue::Int(b as jint)],
    );
    if result.is_err() {
        let _ = env.exception_clear();
    }
}

extern "C" fn sink_release(data: *mut c_void) {
    // GlobalRef attaches the thread it is dropped on.
    drop(unsafe { Box::from_raw(data as *mut Sink) });
}

pub extern "system" fn nativeOffscreenSurfaceSetSink(env: JNIEnv, _: JClass, ptr: jlong, sink: JObject) {
    let Ok(global) = env.new_global_ref(sink) else { return };
    let data = Box::into_raw(Box::new(Sink(global))) as *mut c_void;
    unsafe {
        canvas_native_offscreen_surface_set_ui_sink(
            surface(ptr),
            CanvasOffscreenUiSink { data, call: Some(sink_call), release: Some(sink_release) },
        )
    };
}

/// Not @FastNative: these call into Java or wait on a context's thread.
pub fn register(env: &mut JNIEnv, class: &JClass) {
    let methods = [
        ("nativeOffscreenSurfaceCreate", "(IIFFII)J", nativeOffscreenSurfaceCreate as *mut c_void),
        ("nativeOffscreenSurfaceSetWindow", "(JLandroid/view/Surface;)V", nativeOffscreenSurfaceSetWindow as *mut c_void),
        ("nativeOffscreenSurfaceWindowDestroyed", "(J)V", nativeOffscreenSurfaceWindowDestroyed as *mut c_void),
        ("nativeOffscreenSurfaceDetachView", "(J)V", nativeOffscreenSurfaceDetachView as *mut c_void),
        ("nativeOffscreenSurfaceSetSink", "(JLjava/lang/Object;)V", nativeOffscreenSurfaceSetSink as *mut c_void),
        ("nativeOffscreenSurfaceReference", "(J)J", nativeOffscreenSurfaceReference as *mut c_void),
        ("nativeOffscreenSurfaceRelease", "(J)V", nativeOffscreenSurfaceRelease as *mut c_void),
        ("nativeOffscreenSurfaceToDataURL", "(JLjava/lang/String;I)Ljava/lang/String;", nativeOffscreenSurfaceToDataURL as *mut c_void),
    ];
    let methods: Vec<NativeMethod> = methods
        .into_iter()
        .map(|(name, sig, fn_ptr)| NativeMethod { name: name.into(), sig: sig.into(), fn_ptr })
        .collect();
    let _ = env.register_native_methods(class, &methods);
}
