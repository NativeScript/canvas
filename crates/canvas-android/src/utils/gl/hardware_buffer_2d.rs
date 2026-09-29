//! Video frames imported as EGLImages on the thread that rasterizes the canvas, and handed back to
//! the decoder only once the GPU has read them.

use std::collections::VecDeque;
use std::ffi::{c_char, c_void, CString};
use std::sync::{Arc, Mutex, OnceLock};

use canvas_2d::context::recording::{ExternalImage, ExternalRelease};
use jni::objects::{GlobalRef, JClass, JObject};
use jni::sys::{jboolean, jfloat, jint, jlong, JNI_FALSE, JNI_TRUE};
use jni::{JNIEnv, JavaVM};
use skia_safe::gpu;

use super::TEXTURE_EXTERNAL_OES;

type EGLDisplay = *mut c_void;
type EGLContext = *mut c_void;
type EGLImage = *mut c_void;
type EGLSync = *mut c_void;

const EGL_NATIVE_BUFFER_ANDROID: u32 = 0x3140;
const EGL_IMAGE_PRESERVED_KHR: i32 = 0x30D2;
const EGL_SYNC_FENCE_KHR: u32 = 0x30F9;
const EGL_CONDITION_SATISFIED_KHR: i32 = 0x30F6;
const EGL_TRUE: i32 = 1;
const EGL_NONE: i32 = 0x3038;
/// The decoder only has a few buffers.
const MAX_HELD: usize = 2;
const WAIT_NS: u64 = 16_000_000;

#[link(name = "EGL")]
extern "C" {
    fn eglGetProcAddress(name: *const c_char) -> *mut c_void;
    fn eglGetCurrentDisplay() -> EGLDisplay;
    fn eglGetCurrentContext() -> EGLContext;
}

extern "C" {
    fn AHardwareBuffer_fromHardwareBuffer(
        env: *mut jni::sys::JNIEnv,
        hardware_buffer: jni::sys::jobject,
    ) -> *mut c_void;
    fn AHardwareBuffer_acquire(buffer: *mut c_void);
    fn AHardwareBuffer_release(buffer: *mut c_void);
}

struct Egl {
    native_client_buffer: unsafe extern "C" fn(*const c_void) -> *mut c_void,
    create_image: unsafe extern "C" fn(EGLDisplay, EGLContext, u32, *mut c_void, *const i32) -> EGLImage,
    destroy_image: unsafe extern "C" fn(EGLDisplay, EGLImage) -> u32,
    image_target_texture: unsafe extern "C" fn(u32, EGLImage),
    create_sync: unsafe extern "C" fn(EGLDisplay, u32, *const i32) -> EGLSync,
    client_wait_sync: unsafe extern "C" fn(EGLDisplay, EGLSync, i32, u64) -> i32,
    destroy_sync: unsafe extern "C" fn(EGLDisplay, EGLSync) -> u32,
}

fn egl() -> Option<&'static Egl> {
    static EGL: OnceLock<Option<Egl>> = OnceLock::new();
    EGL.get_or_init(|| unsafe {
        fn load(name: &str) -> Option<*mut c_void> {
            let name = CString::new(name).ok()?;
            let ptr = unsafe { eglGetProcAddress(name.as_ptr()) };
            (!ptr.is_null()).then_some(ptr)
        }
        Some(Egl {
            native_client_buffer: std::mem::transmute(load("eglGetNativeClientBufferANDROID")?),
            create_image: std::mem::transmute(load("eglCreateImageKHR")?),
            destroy_image: std::mem::transmute(load("eglDestroyImageKHR")?),
            image_target_texture: std::mem::transmute(load("glEGLImageTargetTexture2DOES")?),
            create_sync: std::mem::transmute(load("eglCreateSyncKHR")?),
            client_wait_sync: std::mem::transmute(load("eglClientWaitSyncKHR")?),
            destroy_sync: std::mem::transmute(load("eglDestroySyncKHR")?),
        })
    })
    .as_ref()
}

/// `owner.close()` returns the frame to the decoder.
struct Frame {
    buffer: *mut c_void,
    owner: Option<GlobalRef>,
    vm: Arc<JavaVM>,
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe { AHardwareBuffer_release(self.buffer) };
        if let Some(owner) = self.owner.take() {
            if let Ok(mut env) = self.vm.attach_current_thread_permanently() {
                let _ = env.call_method(owner.as_obj(), "close", "()V", &[]);
                let _ = env.exception_clear();
            }
        }
    }
}

struct Imported {
    display: EGLDisplay,
    context: EGLContext,
    image: EGLImage,
    texture: u32,
    _frame: Frame,
}

impl Drop for Imported {
    fn drop(&mut self) {
        let Some(egl) = egl() else { return };
        unsafe {
            // With another context current this would delete its texture; leaking the name is safer.
            if eglGetCurrentContext() == self.context {
                gl_bindings::DeleteTextures(1, &self.texture);
            }
            (egl.destroy_image)(self.display, self.image);
        }
    }
}

unsafe impl Send for Frame {}
unsafe impl Send for Imported {}

struct Pending {
    sync: EGLSync,
    imported: Imported,
}
unsafe impl Send for Pending {}

static PENDING: Mutex<VecDeque<Pending>> = Mutex::new(VecDeque::new());

fn release_finished() {
    let Some(egl) = egl() else { return };
    let Ok(mut pending) = PENDING.lock() else { return };
    while let Some(front) = pending.front() {
        let timeout = if pending.len() > MAX_HELD { WAIT_NS } else { 0 };
        let status = unsafe { (egl.client_wait_sync)(front.imported.display, front.sync, 0, timeout) };
        if status != EGL_CONDITION_SATISFIED_KHR && pending.len() <= MAX_HELD {
            break;
        }
        if let Some(done) = pending.pop_front() {
            unsafe { (egl.destroy_sync)(done.imported.display, done.sync) };
            drop(done.imported);
        }
    }
}

fn import(frame: Frame, ctx: &mut gpu::DirectContext, width: i32, height: i32) -> Option<(skia_safe::Image, ExternalRelease)> {
    let ext = egl()?;
    let display = unsafe { eglGetCurrentDisplay() };
    let context = unsafe { eglGetCurrentContext() };
    if display.is_null() || context.is_null() {
        return None;
    }
    let client = unsafe { (ext.native_client_buffer)(frame.buffer) };
    let attrs = [EGL_IMAGE_PRESERVED_KHR, EGL_TRUE, EGL_NONE];
    let image = unsafe {
        (ext.create_image)(display, std::ptr::null_mut(), EGL_NATIVE_BUFFER_ANDROID, client, attrs.as_ptr())
    };
    if image.is_null() {
        return None;
    }
    let mut texture = 0u32;
    unsafe {
        gl_bindings::GenTextures(1, &mut texture);
        gl_bindings::BindTexture(TEXTURE_EXTERNAL_OES, texture);
        gl_bindings::TexParameteri(TEXTURE_EXTERNAL_OES, gl_bindings::TEXTURE_MIN_FILTER, gl_bindings::LINEAR as _);
        gl_bindings::TexParameteri(TEXTURE_EXTERNAL_OES, gl_bindings::TEXTURE_MAG_FILTER, gl_bindings::LINEAR as _);
        gl_bindings::TexParameteri(TEXTURE_EXTERNAL_OES, gl_bindings::TEXTURE_WRAP_S, gl_bindings::CLAMP_TO_EDGE as _);
        gl_bindings::TexParameteri(TEXTURE_EXTERNAL_OES, gl_bindings::TEXTURE_WRAP_T, gl_bindings::CLAMP_TO_EDGE as _);
        (ext.image_target_texture)(TEXTURE_EXTERNAL_OES, image);
        gl_bindings::BindTexture(TEXTURE_EXTERNAL_OES, 0);
    }
    let imported = Imported { display, context, image, texture, _frame: frame };
    // Skia caches GL state, and the texture binding just changed under it.
    ctx.reset(None);

    let info = gpu::gl::TextureInfo {
        target: TEXTURE_EXTERNAL_OES,
        id: texture,
        format: 0x8058, // GL_RGBA8: external textures sample as RGBA whatever the buffer format.
        protected: gpu::Protected::No,
    };
    let texture =
        unsafe { gpu::backend_textures::make_gl((width, height), gpu::Mipmapped::No, info, "") };
    let image = skia_safe::Image::from_texture(
        ctx,
        &texture,
        gpu::SurfaceOrigin::TopLeft,
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    )?;

    let release: ExternalRelease = Box::new(move |context: &mut canvas_2d::context::Context| {
        if let Some(ctx) = context.gpu_context() {
            ctx.flush_and_submit();
        }
        let Some(egl) = egl() else { return };
        let sync = unsafe { (egl.create_sync)(imported.display, EGL_SYNC_FENCE_KHR, [EGL_NONE].as_ptr()) };
        if sync.is_null() {
            unsafe { gl_bindings::Finish() };
            drop(imported);
        } else if let Ok(mut pending) = PENDING.lock() {
            pending.push_back(Pending { sync, imported });
        }
        release_finished();
    });
    Some((image, release))
}

/// On true, takes over `owner` and closes it once the GPU is done with the frame.
#[no_mangle]
pub unsafe extern "system" fn Java_org_nativescript_canvas_Utils_nativeContext2DDrawHardwareBuffer(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    buffer: JObject,
    owner: JObject,
    width: jint,
    height: jint,
    sx: jfloat,
    sy: jfloat,
    sw: jfloat,
    sh: jfloat,
    dx: jfloat,
    dy: jfloat,
    dw: jfloat,
    dh: jfloat,
) -> jboolean {
    if context == 0 || buffer.is_null() || width <= 0 || height <= 0 || egl().is_none() {
        return JNI_FALSE;
    }
    let ahb = AHardwareBuffer_fromHardwareBuffer(env.get_raw(), buffer.as_raw());
    if ahb.is_null() {
        return JNI_FALSE;
    }
    let (Ok(vm), Ok(owner)) = (env.get_java_vm(), env.new_global_ref(owner)) else {
        return JNI_FALSE;
    };
    AHardwareBuffer_acquire(ahb);
    let frame = Frame {
        buffer: ahb,
        owner: Some(owner),
        vm: Arc::new(vm),
    };

    let make: ExternalImage = Box::new(move |context: &mut canvas_2d::context::Context| {
        import(frame, context.gpu_context()?, width, height)
    });
    let ctx = &mut *(context as *mut canvas_c::CanvasRenderingContext2D);
    ctx.get_context_mut().draw_external_image(
        width,
        height,
        skia_safe::Rect::from_xywh(sx, sy, sw, sh),
        skia_safe::Rect::from_xywh(dx, dy, dw, dh),
        make,
    );
    JNI_TRUE
}
