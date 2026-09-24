//! GPU rasterization for the SVG document, rendering straight into the window's framebuffer.

#[cfg(feature = "gl")]
mod gl;
#[cfg(feature = "metal")]
mod metal;
#[cfg(feature = "vulkan")]
mod vulkan;

pub mod thread;

use crate::SvgDocument;

/// `Auto` tries the platform's preferred API and falls back; forcing GL works around bad drivers.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Backend {
    Auto = 0,
    Gl = 1,
    Vulkan = 2,
    Metal = 3,
}

impl Backend {
    fn from_raw(value: i32) -> Self {
        match value {
            1 => Backend::Gl,
            2 => Backend::Vulkan,
            3 => Backend::Metal,
            _ => Backend::Auto,
        }
    }
}

/// Only `Lost` warrants a context rebuild; `Skipped` is a momentarily unavailable image.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Frame {
    Presented,
    Skipped,
    Lost,
}

/// `Recovered` means presented after a context rebuild; `Lost` means fall back to the CPU raster.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FrameStatus {
    Presented = 0,
    Skipped = 1,
    Recovered = 2,
    Lost = 3,
}

/// Consecutive rebuilds before giving up and letting the caller fall back.
const MAX_RECOVERIES: u32 = 3;

enum Inner {
    #[cfg(feature = "gl")]
    Gl(gl::GlSurface),
    #[cfg(feature = "vulkan")]
    Vulkan(vulkan::VulkanSurface),
    #[cfg(feature = "metal")]
    Metal(metal::MetalSurface),
}

fn backend_of(inner: &Inner) -> Backend {
    match inner {
        #[cfg(feature = "gl")]
        Inner::Gl(_) => Backend::Gl,
        #[cfg(feature = "vulkan")]
        Inner::Vulkan(_) => Backend::Vulkan,
        #[cfg(feature = "metal")]
        Inner::Metal(_) => Backend::Metal,
    }
}

/// Keeps the window, backend and size so `render` can rebuild the context in place after a
/// driver reset or GPU reclaim, which can happen while the window is still valid.
pub struct SvgGpuSurface {
    inner: Option<Inner>,
    window: *mut std::ffi::c_void,
    requested: Backend,
    width: i32,
    height: i32,
    /// Cleared by a frame that actually presents.
    recoveries: u32,
}

macro_rules! dispatch {
    ($self:expr, $default:expr, |$s:ident| $body:expr) => {
        match &mut $self.inner {
            None => $default,
            #[cfg(feature = "gl")]
            Some(Inner::Gl($s)) => $body,
            #[cfg(feature = "vulkan")]
            Some(Inner::Vulkan($s)) => $body,
            #[cfg(feature = "metal")]
            Some(Inner::Metal($s)) => $body,
        }
    };
}

impl SvgGpuSurface {
    /// `window` is an `ANativeWindow*` on Android, a `CAMetalLayer`-backed view on Apple.
    pub fn new(
        window: *mut std::ffi::c_void,
        width: i32,
        height: i32,
        backend: Backend,
    ) -> Option<Self> {
        if window.is_null() || width <= 0 || height <= 0 {
            return None;
        }

        let inner = Self::build(window, width, height, backend)?;
        log::info!("svg gpu: {:?} context ready at {width}x{height}", backend_of(&inner));
        Some(Self {
            inner: Some(inner),
            window,
            requested: backend,
            width,
            height,
            recoveries: 0,
        })
    }

    fn build(
        window: *mut std::ffi::c_void,
        width: i32,
        height: i32,
        backend: Backend,
    ) -> Option<Inner> {
        match backend {
            Backend::Auto => Self::auto(window, width, height),
            Backend::Gl => Self::gl(window, width, height),
            Backend::Vulkan => Self::vulkan(window, width, height),
            Backend::Metal => Self::metal(window, width, height),
        }
    }

    /// Metal on Apple, otherwise Vulkan with a GL fallback: Android drivers may expose a
    /// broken Vulkan.
    fn auto(window: *mut std::ffi::c_void, width: i32, height: i32) -> Option<Inner> {
        #[cfg(feature = "metal")]
        if let Some(inner) = Self::metal(window, width, height) {
            return Some(inner);
        }
        #[cfg(feature = "vulkan")]
        if let Some(inner) = Self::vulkan(window, width, height) {
            return Some(inner);
        }
        #[cfg(feature = "vulkan")]
        log::warn!("svg gpu: vulkan unavailable, trying gl");
        #[cfg(feature = "gl")]
        if let Some(inner) = Self::gl(window, width, height) {
            return Some(inner);
        }
        log::warn!("svg gpu: no backend available, falling back to the cpu raster");
        let _ = (window, width, height);
        None
    }

    fn gl(_window: *mut std::ffi::c_void, _width: i32, _height: i32) -> Option<Inner> {
        #[cfg(feature = "gl")]
        return gl::GlSurface::new(_window, _width, _height).map(Inner::Gl);
        #[cfg(not(feature = "gl"))]
        return None;
    }

    fn vulkan(_window: *mut std::ffi::c_void, _width: i32, _height: i32) -> Option<Inner> {
        #[cfg(feature = "vulkan")]
        return vulkan::VulkanSurface::new(_window, _width, _height).map(Inner::Vulkan);
        #[cfg(not(feature = "vulkan"))]
        return None;
    }

    fn metal(_window: *mut std::ffi::c_void, _width: i32, _height: i32) -> Option<Inner> {
        #[cfg(feature = "metal")]
        return metal::MetalSurface::new(_window, _width, _height).map(Inner::Metal);
        #[cfg(not(feature = "metal"))]
        return None;
    }

    /// `Auto` means the context is gone and not yet rebuilt.
    pub fn backend(&self) -> Backend {
        match &self.inner {
            None => Backend::Auto,
            #[cfg(feature = "gl")]
            Some(Inner::Gl(_)) => Backend::Gl,
            #[cfg(feature = "vulkan")]
            Some(Inner::Vulkan(_)) => Backend::Vulkan,
            #[cfg(feature = "metal")]
            Some(Inner::Metal(_)) => Backend::Metal,
        }
    }

    pub fn window(&self) -> *mut std::ffi::c_void {
        self.window
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        if width <= 0 || height <= 0 {
            return;
        }
        // Recorded even with no context, so a rebuild comes back at the current size.
        self.width = width;
        self.height = height;
        dispatch!(self, (), |s| s.resize(width, height))
    }

    pub fn render(&mut self, doc: &mut SvgDocument, scale: f32) -> FrameStatus {
        let doc = std::cell::RefCell::new(doc);
        self.paint(&|canvas, width, height| {
            doc.borrow_mut().0.draw(canvas, width, height, scale)
        })
    }

    /// Render-thread path: replays an immutable display list, never touching the document.
    pub fn present(&mut self, frame: &canvas_svg::RecordedFrame) -> FrameStatus {
        self.paint(&|canvas, _, _| frame.replay(canvas))
    }

    fn paint(&mut self, paint: &dyn Fn(&skia_safe::Canvas, i32, i32)) -> FrameStatus {
        match self.render_once(paint) {
            Frame::Presented => {
                self.recoveries = 0;
                FrameStatus::Presented
            }
            Frame::Skipped => FrameStatus::Skipped,
            Frame::Lost => {
                log::warn!("svg gpu: context lost, rebuilding");
                if !self.rebuild() {
                    return FrameStatus::Lost;
                }
                match self.render_once(paint) {
                    Frame::Presented => FrameStatus::Recovered,
                    // A fresh context that still cannot present won't recover.
                    _ => FrameStatus::Lost,
                }
            }
        }
    }

    fn render_once(&mut self, paint: &dyn Fn(&skia_safe::Canvas, i32, i32)) -> Frame {
        dispatch!(self, Frame::Lost, |s| s.render(paint))
    }

    /// The old context must drop first: EGL rejects a second surface for the same window, and
    /// the old Vulkan swapchain still holds the images.
    fn rebuild(&mut self) -> bool {
        if self.recoveries >= MAX_RECOVERIES {
            log::warn!("svg gpu: giving up after {MAX_RECOVERIES} rebuilds");
            return false;
        }
        self.recoveries += 1;
        self.inner = None;
        self.inner = Self::build(self.window, self.width, self.height, self.requested);
        self.inner.is_some()
    }

    /// Drops the context so the next frame takes the recovery path.
    pub fn debug_lose_context(&mut self) {
        self.inner = None;
        self.recoveries = 0;
    }
}

#[cfg(all(feature = "gl", target_os = "android"))]
fn window_handle(window: *mut std::ffi::c_void) -> Option<raw_window_handle::RawWindowHandle> {
    let ptr = std::ptr::NonNull::new(window)?;
    Some(raw_window_handle::RawWindowHandle::AndroidNdk(
        raw_window_handle::AndroidNdkWindowHandle::new(ptr),
    ))
}

#[cfg(all(feature = "gl", not(target_os = "android")))]
fn window_handle(_window: *mut std::ffi::c_void) -> Option<raw_window_handle::RawWindowHandle> {
    None
}

/// `backend` is a `Backend` discriminant; pass 0 (auto) unless the user forced one.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_gpu_create(
    window: *mut std::ffi::c_void,
    width: i32,
    height: i32,
    backend: i32,
) -> *mut SvgGpuSurface {
    match SvgGpuSurface::new(window, width, height, Backend::from_raw(backend)) {
        Some(surface) => Box::into_raw(Box::new(surface)),
        None => std::ptr::null_mut(),
    }
}

/// Returns the `Backend` discriminant actually in use, or 0 for a null surface.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_gpu_backend(gpu: *const SvgGpuSurface) -> i32 {
    if gpu.is_null() {
        return Backend::Auto as i32;
    }
    unsafe { &*gpu }.backend() as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_gpu_resize(gpu: *mut SvgGpuSurface, width: i32, height: i32) {
    if gpu.is_null() {
        return;
    }
    unsafe { &mut *gpu }.resize(width, height);
}

/// Returns a `FrameStatus` discriminant; only 3 (lost) means the GPU path is unusable.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_gpu_render(
    gpu: *mut SvgGpuSurface,
    doc: *mut SvgDocument,
    scale: f32,
) -> i32 {
    if gpu.is_null() || doc.is_null() {
        return FrameStatus::Lost as i32;
    }
    let gpu = unsafe { &mut *gpu };
    let doc = unsafe { &mut *doc };
    gpu.render(doc, scale) as i32
}

/// The caller still owns the reference it passed to `create` and must release it after `destroy`.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_gpu_window(gpu: *const SvgGpuSurface) -> *mut std::ffi::c_void {
    if gpu.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { &*gpu }.window()
}

/// Simulates a context loss to exercise the recovery path.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_gpu_debug_lose_context(gpu: *mut SvgGpuSurface) {
    if gpu.is_null() {
        return;
    }
    unsafe { &mut *gpu }.debug_lose_context();
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_gpu_destroy(gpu: *mut SvgGpuSurface) {
    if gpu.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(gpu) };
}

use thread::RenderThread;

/// Registers `window` with the shared render thread, which builds its GPU surface there. Null
/// means use the bitmap.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_render_thread_create(
    window: *mut std::ffi::c_void,
    width: i32,
    height: i32,
    backend: i32,
) -> *mut RenderThread {
    match RenderThread::new(window, width, height, Backend::from_raw(backend)) {
        Some(thread) => Box::into_raw(Box::new(thread)),
        None => std::ptr::null_mut(),
    }
}

/// Records `doc` on the calling thread (which owns it); only the display list crosses to the
/// render thread. Returns false when there was nothing to record.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_render_thread_commit(
    render: *mut RenderThread,
    doc: *mut SvgDocument,
    width: i32,
    height: i32,
    scale: f32,
) -> bool {
    if render.is_null() || doc.is_null() {
        return false;
    }
    let render = unsafe { &*render };
    let doc = unsafe { &mut *doc };
    match doc.0.frame(width, height, scale) {
        Some(frame) => {
            render.commit(frame);
            true
        }
        None => false,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_render_thread_resize(
    render: *mut RenderThread,
    width: i32,
    height: i32,
) {
    if render.is_null() || width <= 0 || height <= 0 {
        return;
    }
    unsafe { &*render }.resize(width, height);
}

/// The `FrameStatus` of the last present, or -1 if nothing has presented since the last call.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_render_thread_status(render: *mut RenderThread) -> i32 {
    if render.is_null() {
        return -1;
    }
    match unsafe { &*render }.take_status() {
        Some(status) => status as i32,
        None => -1,
    }
}

/// Blocks until the thread has torn down its surface, since the caller releases the window next.
#[unsafe(no_mangle)]
pub extern "C" fn canvas_native_svg_render_thread_destroy(render: *mut RenderThread) {
    if !render.is_null() {
        let _ = unsafe { Box::from_raw(render) };
    }
}
