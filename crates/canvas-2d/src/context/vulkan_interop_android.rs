//! Android: the canvas drawn into an AHardwareBuffer that WebGPU (Vulkan) samples, ordered with the
//! reader through native fence fds (`EGL_ANDROID_native_fence_sync`).

use std::ffi::{c_char, c_void, CStr, CString};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

use crate::context::Context;

type EGLDisplay = *mut c_void;
type EGLImage = *mut c_void;
type EGLSync = *mut c_void;

const EGL_EXTENSIONS: i32 = 0x3055;
const EGL_NONE: i32 = 0x3038;
const EGL_TRUE: i32 = 1;
const EGL_IMAGE_PRESERVED_KHR: i32 = 0x30D2;
const EGL_NATIVE_BUFFER_ANDROID: u32 = 0x3140;
const EGL_SYNC_NATIVE_FENCE_ANDROID: u32 = 0x3144;
const EGL_SYNC_NATIVE_FENCE_FD_ANDROID: i32 = 0x3145;

/// The image to draw into, and the reader's fence.
pub struct HardwareBufferTarget {
    pub id: u64,
    /// `AHardwareBuffer *`, kept alive by the caller until its copy has been submitted.
    pub buffer: *mut c_void,
    pub width: i32,
    pub height: i32,
    /// Signalled once the reader's last copy is done, owned by the callee; -1 for none.
    pub release_fd: i32,
}

unsafe impl Send for HardwareBufferTarget {}

#[link(name = "EGL")]
extern "C" {
    fn eglGetProcAddress(name: *const c_char) -> *mut c_void;
    fn eglGetCurrentDisplay() -> EGLDisplay;
    fn eglQueryString(display: EGLDisplay, name: i32) -> *const c_char;
}

#[repr(C)]
struct PollFd {
    fd: i32,
    events: i16,
    revents: i16,
}

const POLLIN: i16 = 1;

extern "C" {
    fn close(fd: i32) -> i32;
    fn poll(fds: *mut PollFd, count: u32, timeout: i32) -> i32;
}

struct Egl {
    native_client_buffer: unsafe extern "C" fn(*const c_void) -> *mut c_void,
    create_image: unsafe extern "C" fn(EGLDisplay, *mut c_void, u32, *mut c_void, *const i32) -> EGLImage,
    destroy_image: unsafe extern "C" fn(EGLDisplay, EGLImage) -> u32,
    image_target_texture: unsafe extern "C" fn(u32, EGLImage),
    create_sync: unsafe extern "C" fn(EGLDisplay, u32, *const i32) -> EGLSync,
    destroy_sync: unsafe extern "C" fn(EGLDisplay, EGLSync) -> u32,
    wait_sync: unsafe extern "C" fn(EGLDisplay, EGLSync, i32) -> i32,
    dup_native_fence_fd: unsafe extern "C" fn(EGLDisplay, EGLSync) -> i32,
}

/// The current display has every extension the copy needs.
fn egl() -> Option<&'static Egl> {
    static EGL: OnceLock<Option<Egl>> = OnceLock::new();
    EGL.get_or_init(|| unsafe {
        let display = eglGetCurrentDisplay();
        let extensions = eglQueryString(display, EGL_EXTENSIONS);
        if display.is_null() || extensions.is_null() {
            return None;
        }
        let extensions = CStr::from_ptr(extensions).to_string_lossy();
        let required = [
            "EGL_ANDROID_get_native_client_buffer",
            "EGL_ANDROID_image_native_buffer",
            "EGL_ANDROID_native_fence_sync",
            "EGL_KHR_wait_sync",
        ];
        if !required.iter().all(|name| extensions.split(' ').any(|e| e == *name)) {
            return None;
        }
        let load = |name: &str| {
            let name = CString::new(name).ok()?;
            let f = eglGetProcAddress(name.as_ptr());
            (!f.is_null()).then_some(f)
        };
        Some(Egl {
            native_client_buffer: std::mem::transmute(load("eglGetNativeClientBufferANDROID")?),
            create_image: std::mem::transmute(load("eglCreateImageKHR")?),
            destroy_image: std::mem::transmute(load("eglDestroyImageKHR")?),
            image_target_texture: std::mem::transmute(load("glEGLImageTargetTexture2DOES")?),
            create_sync: std::mem::transmute(load("eglCreateSyncKHR")?),
            destroy_sync: std::mem::transmute(load("eglDestroySyncKHR")?),
            wait_sync: std::mem::transmute(load("eglWaitSyncKHR")?),
            dup_native_fence_fd: std::mem::transmute(load("eglDupNativeFenceFDANDROID")?),
        })
    })
    .as_ref()
}

/// The buffer as a texture of the canvas's context.
struct Import {
    id: u64,
    display: EGLDisplay,
    image: EGLImage,
    texture: u32,
}

impl Import {
    /// The context current.
    unsafe fn new(egl: &Egl, target: &HardwareBufferTarget) -> Option<Self> {
        let display = eglGetCurrentDisplay();
        let client = (egl.native_client_buffer)(target.buffer);
        if client.is_null() {
            return None;
        }
        let attributes = [EGL_IMAGE_PRESERVED_KHR, EGL_TRUE, EGL_NONE];
        let image = (egl.create_image)(
            display,
            std::ptr::null_mut(),
            EGL_NATIVE_BUFFER_ANDROID,
            client,
            attributes.as_ptr(),
        );
        if image.is_null() {
            return None;
        }
        let mut bound = 0;
        gl_bindings::GetIntegerv(gl_bindings::TEXTURE_BINDING_2D, &mut bound);
        let mut texture = 0;
        gl_bindings::GenTextures(1, &mut texture);
        gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, texture);
        (egl.image_target_texture)(gl_bindings::TEXTURE_2D, image);
        gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, bound as u32);
        Some(Self {
            id: target.id,
            display,
            image,
            texture,
        })
    }

    /// The context current.
    unsafe fn delete(&self, egl: &Egl) {
        gl_bindings::DeleteTextures(1, &self.texture);
        (egl.destroy_image)(self.display, self.image);
    }
}

/// Per canvas: the buffer it last drew into, and the id the reader caches buffers under.
pub(crate) struct VulkanInterop {
    producer: u64,
    import: Option<Import>,
}

impl VulkanInterop {
    /// The context current.
    pub(crate) fn release(&mut self) {
        if let (Some(import), Some(egl)) = (self.import.take(), egl()) {
            unsafe { import.delete(egl) };
        }
    }
}

impl Default for VulkanInterop {
    fn default() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Self {
            producer: NEXT.fetch_add(1, Ordering::Relaxed),
            import: None,
        }
    }
}

impl Context {
    /// Identifies this canvas to a reader that caches the buffers it hands out.
    pub fn vulkan_producer(&self) -> u64 {
        self.vulkan_interop.producer
    }

    /// Draws the canvas, premultiplied and top row first, into `target` once the reader's
    /// `release_fd` has signalled. Returns a fence fd signalled when the drawing is done (-1 if
    /// it already is); `None` when GL cannot draw into the buffer.
    pub fn draw_into_hardware_buffer(&mut self, target: &HardwareBufferTarget) -> Option<i32> {
        let release_fd = target.release_fd;
        let current = self.gl_context.as_ref().is_some_and(|gl| gl.make_current());
        let egl = current.then(egl).flatten();
        let Some(egl) = egl else {
            if release_fd >= 0 {
                unsafe { close(release_fd) };
            }
            return None;
        };
        if self.vulkan_interop.import.as_ref().map(|import| import.id) != Some(target.id) {
            if let Some(old) = self.vulkan_interop.import.take() {
                unsafe { old.delete(egl) };
            }
            self.vulkan_interop.import = unsafe { Import::new(egl, target) };
            if let Some(ctx) = self.direct_context.as_mut() {
                ctx.reset(None);
            }
        }
        let Some((display, texture)) = self
            .vulkan_interop
            .import
            .as_ref()
            .map(|import| (import.display, import.texture))
        else {
            if release_fd >= 0 {
                unsafe { close(release_fd) };
            }
            return None;
        };
        self.flush();
        unsafe {
            if release_fd >= 0 {
                let attributes = [EGL_SYNC_NATIVE_FENCE_FD_ANDROID, release_fd, EGL_NONE];
                let sync = (egl.create_sync)(display, EGL_SYNC_NATIVE_FENCE_ANDROID, attributes.as_ptr());
                if sync.is_null() {
                    // EGL did not take the fd: wait for it here.
                    let mut fd = PollFd {
                        fd: release_fd,
                        events: POLLIN,
                        revents: 0,
                    };
                    poll(&mut fd, 1, -1);
                    close(release_fd);
                } else {
                    (egl.wait_sync)(display, sync, 0);
                    (egl.destroy_sync)(display, sync);
                }
            }
        }
        // The reader waits on the returned fence from here on, so it is made even if drawing failed.
        if !self.draw_into_gl_texture(texture, target.width, target.height, false, None) {
            log::warn!("canvas: could not draw a 2D canvas into a WebGPU buffer");
        }
        unsafe {
            let sync = (egl.create_sync)(display, EGL_SYNC_NATIVE_FENCE_ANDROID, [EGL_NONE].as_ptr());
            gl_bindings::Flush();
            let fd = if sync.is_null() {
                -1
            } else {
                let fd = (egl.dup_native_fence_fd)(display, sync);
                (egl.destroy_sync)(display, sync);
                fd
            };
            if fd < 0 {
                gl_bindings::Finish();
            }
            Some(fd)
        }
    }
}
