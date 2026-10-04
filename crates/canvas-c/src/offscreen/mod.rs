//! An OffscreenCanvas's surface: a transferred canvas's view, or none. The UI thread reaches the
//! owner thread's context only through the render thread, the WebGL thread or wgpu's locks.

use std::collections::HashMap;
use std::ffi::{c_char, c_void, CStr, CString};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

use canvas_2d::context::text_styles::text_direction::TextDirection;
use canvas_2d::context::Context;
use canvas_core::context_attributes::{ColorSpace, PowerPreference};

use crate::c2d::render_thread::TargetHandle;
use crate::webgpu::gpu::CanvasWebGPUInstance;
use crate::webgpu::gpu_canvas_context::CanvasGPUCanvasContext;
use crate::{CanvasColorSpace, CanvasRenderingContext2D, WebGLState};

#[cfg(target_os = "android")]
pub mod android;
#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
mod apple;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasOffscreenEvent {
    /// `a` x `b`.
    Resize = 1,
    /// `a`: a `CanvasOffscreenEngine`, `b`: 1 with alpha.
    Engine = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasOffscreenEngine {
    Metal = 1,
    GL = 2,
    GPU = 3,
}

/// `call` comes from any thread; it must hop to the UI thread, or run inline when already there.
#[repr(C)]
pub struct CanvasOffscreenUiSink {
    pub data: *mut c_void,
    pub call: Option<extern "C" fn(data: *mut c_void, event: CanvasOffscreenEvent, a: u32, b: u32)>,
    pub release: Option<extern "C" fn(data: *mut c_void)>,
}

struct Sink(CanvasOffscreenUiSink);

unsafe impl Send for Sink {}
unsafe impl Sync for Sink {}

impl Sink {
    fn send(&self, event: CanvasOffscreenEvent, a: u32, b: u32) {
        if let Some(call) = self.0.call {
            call(self.0.data, event, a, b);
        }
    }
}

impl Drop for Sink {
    fn drop(&mut self) {
        if let Some(release) = self.0.release {
            release(self.0.data);
        }
    }
}

struct GlRef(*mut WebGLState);

// A direct (unthreaded) state only lives on a viewless surface, held by its owner thread alone.
unsafe impl Send for GlRef {}

impl GlRef {
    fn state(&self) -> &WebGLState {
        unsafe { &*self.0 }
    }
}

impl Drop for GlRef {
    fn drop(&mut self) {
        crate::canvas_native_webgl_state_destroy(self.0);
    }
}

enum Binding {
    None,
    TwoD(TargetHandle),
    /// Viewless and unthreaded: only the owner thread touches it.
    TwoDDirect,
    WebGL(GlRef),
    WebGPU(Arc<CanvasGPUCanvasContext>),
}

struct Inner {
    #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
    view: Option<apple::View>,
    #[cfg(target_os = "android")]
    view: Option<android::View>,
    binding: Binding,
    sink: Option<Arc<Sink>>,
}

pub struct CanvasOffscreenSurface {
    density: f32,
    ppi: f32,
    direction: u32,
    color_space: ColorSpace,
    /// Only the owner thread changes it.
    size: Mutex<(u32, u32)>,
    inner: Mutex<Inner>,
}

impl CanvasOffscreenSurface {
    fn new(width: u32, height: u32, density: f32, ppi: f32, direction: u32, color_space: CanvasColorSpace) -> Self {
        Self {
            density,
            ppi,
            direction,
            color_space: color_space.into(),
            size: Mutex::new((width, height)),
            inner: Mutex::new(Inner {
                #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos", target_os = "android"))]
                view: None,
                binding: Binding::None,
                sink: None,
            }),
        }
    }

    fn inner(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn size(&self) -> (u32, u32) {
        *self.size.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn direction(&self) -> TextDirection {
        TextDirection::from(self.direction)
    }

    pub fn has_view(&self) -> bool {
        has_view(&self.inner())
    }

    pub fn has_context(&self) -> bool {
        !matches!(self.inner().binding, Binding::None)
    }

    /// Outside the lock: an inline sink calls back in.
    fn notify(&self, event: CanvasOffscreenEvent, a: u32, b: u32) {
        let sink = self.inner().sink.clone();
        if let Some(sink) = sink {
            sink.send(event, a, b);
        }
    }

    fn bind(&self, binding: Binding, engine: Option<CanvasOffscreenEngine>, alpha: bool) {
        self.inner().binding = binding;
        if let Some(engine) = engine {
            self.notify(CanvasOffscreenEvent::Engine, engine as u32, alpha as u32);
        }
    }

    /// Owner thread, after a 2D context resized its own recording.
    pub fn resize(&self, width: u32, height: u32) {
        let (width, height) = (width.max(1), height.max(1));
        {
            let mut size = self.size.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            if *size == (width, height) {
                return;
            }
            *size = (width, height);
        }
        let has_view = {
            let inner = self.inner();
            match &inner.binding {
                Binding::WebGPU(context) => self.resize_webgpu(&inner, context, width, height),
                Binding::WebGL(state) if !self.view_sizes_webgl(&inner) => {
                    resize_webgl_offscreen(state.state(), width, height)
                }
                _ => {}
            }
            has_view(&inner)
        };
        if has_view {
            self.notify(CanvasOffscreenEvent::Resize, width, height);
        }
    }

    /// A view's WebGL drawing buffer is its layer or window, sized by the view's layout.
    fn view_sizes_webgl(&self, inner: &Inner) -> bool {
        #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
        let sizes = inner.view.as_ref().is_some_and(|view| view.gl_layer.is_some());
        #[cfg(target_os = "android")]
        let sizes = inner.view.as_ref().is_some_and(|view| view.window.is_some());
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "visionos", target_os = "android")))]
        let sizes = {
            let _ = inner;
            false
        };
        sizes
    }

    fn resize_webgpu(&self, inner: &Inner, context: &Arc<CanvasGPUCanvasContext>, width: u32, height: u32) {
        let context = Arc::as_ptr(context);
        #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
        if let Some(layer) = inner.view.as_ref().and_then(|view| view.metal.as_ref()) {
            unsafe {
                crate::webgpu::gpu_canvas_context::canvas_native_webgpu_context_resize_layer(
                    context,
                    layer.layer.ptr(),
                    width,
                    height,
                )
            };
            return;
        }
        #[cfg(target_os = "android")]
        if let Some(window) = inner.view.as_ref().and_then(|view| view.window.as_ref()) {
            unsafe {
                crate::webgpu::gpu_canvas_context::canvas_native_webgpu_context_resize(
                    context as *mut _,
                    window.ptr().as_ptr() as *mut c_void,
                    width,
                    height,
                )
            };
            return;
        }
        let _ = inner;
        unsafe {
            crate::webgpu::gpu_canvas_context::canvas_native_webgpu_context_resize_offscreen(context, width, height)
        };
    }

    /// `threaded: false` is ignored with a view: its callbacks arrive on the UI thread.
    pub fn create_2d(&self, alpha: bool, font_color: i32, threaded: bool) -> *mut CanvasRenderingContext2D {
        if self.has_context() {
            return std::ptr::null_mut();
        }
        let (width, height) = self.size();
        let (w, h) = (width as f32, height as f32);
        let (density, ppi, direction, color_space) = (self.density, self.ppi, self.direction(), self.color_space);

        #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
        let metal = self.inner().view.as_ref().and_then(|view| view.metal.clone());
        #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
        if let Some(layer) = metal {
            let context = CanvasRenderingContext2D::new_threaded_with(
                w, h, density, alpha, font_color, ppi, direction, color_space,
                move || {
                    let layer = layer;
                    let context = Context::new_metal_layer_device_queue_sized(
                        layer.layer.ptr(), layer.device.ptr(), layer.queue.ptr(), w, h, density,
                        layer.samples, alpha, font_color, ppi, direction, color_space,
                    )?;
                    Some(CanvasRenderingContext2D::new_metal(context, alpha))
                },
            );
            return match context {
                Some(context) => {
                    let handle = context.render_target().map(|target| target.handle());
                    self.bind(handle.map_or(Binding::TwoDDirect, Binding::TwoD), Some(CanvasOffscreenEngine::Metal), alpha);
                    Box::into_raw(Box::new(context))
                }
                None => std::ptr::null_mut(),
            };
        }

        #[cfg(target_os = "android")]
        let window = self.inner().view.as_ref().and_then(|view| view.window.clone());
        #[cfg(target_os = "android")]
        let threaded = threaded || self.has_view();
        #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
        let threaded = threaded || self.has_view();

        let context = if threaded {
            CanvasRenderingContext2D::new_threaded_with(
                w, h, density, alpha, font_color, ppi, direction, color_space,
                move || Some(viewless_2d(w, h, density, alpha, font_color, ppi, direction, color_space)),
            )
        } else {
            None
        };
        let context = match context {
            Some(context) => {
                let handle = context.render_target().map(|target| target.handle());
                #[cfg(target_os = "android")]
                if let (Some(handle), Some(window)) = (handle.as_ref(), window) {
                    android::attach_2d(handle, window, width, height);
                }
                #[cfg(target_os = "android")]
                let engine = self.has_view().then_some(CanvasOffscreenEngine::GL);
                #[cfg(not(target_os = "android"))]
                let engine = None;
                self.bind(handle.map_or(Binding::TwoDDirect, Binding::TwoD), engine, alpha);
                context
            }
            None => {
                self.bind(Binding::TwoDDirect, None, alpha);
                viewless_2d(w, h, density, alpha, font_color, ppi, direction, color_space)
            }
        };
        Box::into_raw(Box::new(context))
    }

    pub fn create_webgl(&self, attributes: &WebGLAttributes, threaded: bool) -> *mut WebGLState {
        if self.has_context() {
            return std::ptr::null_mut();
        }
        let (width, height) = self.size();
        let a = *attributes;

        #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
        let gl_layer = self.inner().view.as_ref().and_then(|view| view.gl_layer.clone());
        #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
        if let Some(layer) = gl_layer {
            let state = apple::create_webgl(layer, &a);
            return self.bind_webgl(state, true, a.alpha);
        }

        #[cfg(target_os = "android")]
        let window = self.inner().view.as_ref().and_then(|view| view.window.clone());
        let threaded = threaded || self.has_view();
        let create = if threaded {
            crate::canvas_native_webgl_create_no_window_threaded
        } else {
            crate::canvas_native_webgl_create_no_window
        };
        let state = create(
            width as i32, height as i32, a.version, a.alpha, a.antialias, a.depth,
            a.fail_if_major_performance_caveat, a.power_preference, a.premultiplied_alpha,
            a.preserve_drawing_buffer, a.stencil, a.desynchronized, a.xr_compatible, false,
        );
        #[cfg(target_os = "android")]
        if let (false, Some(window)) = (state.is_null(), window) {
            android::attach_webgl(unsafe { &*state }, window);
        }
        let has_view = self.has_view();
        self.bind_webgl(state, has_view, a.alpha)
    }

    fn bind_webgl(&self, state: *mut WebGLState, has_view: bool, alpha: bool) -> *mut WebGLState {
        if state.is_null() {
            return state;
        }
        crate::canvas_native_webgl_state_reference(state);
        let engine = has_view.then_some(CanvasOffscreenEngine::GL);
        self.bind(Binding::WebGL(GlRef(state)), engine, alpha);
        state
    }

    pub fn create_webgpu(&self, instance: *const CanvasWebGPUInstance) -> *const CanvasGPUCanvasContext {
        if instance.is_null() || self.has_context() {
            return std::ptr::null();
        }
        let (width, height) = self.size();
        #[allow(unused_mut)]
        let mut engine = None;
        let mut context: *const CanvasGPUCanvasContext = std::ptr::null();

        #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
        let metal = self.inner().view.as_ref().and_then(|view| view.metal.clone());
        #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
        if let Some(layer) = metal {
            context = unsafe {
                crate::webgpu::gpu_canvas_context::canvas_native_webgpu_context_create(
                    instance,
                    layer.layer.ptr(),
                    width,
                    height,
                )
            };
            engine = Some(CanvasOffscreenEngine::Metal);
        }
        #[cfg(target_os = "android")]
        let window = self.inner().view.as_ref().and_then(|view| view.window.clone());
        #[cfg(target_os = "android")]
        if let Some(window) = window {
            context = unsafe {
                crate::webgpu::gpu_canvas_context::canvas_native_webgpu_context_create(
                    instance as *mut _,
                    window.ptr().as_ptr() as *mut c_void,
                    width,
                    height,
                )
            };
            engine = Some(CanvasOffscreenEngine::GPU);
        }
        if context.is_null() && engine.is_none() {
            context = unsafe {
                crate::webgpu::gpu_canvas_context::canvas_native_webgpu_context_create_offscreen(instance, width, height)
            };
        }
        if context.is_null() {
            return context;
        }
        let shared = unsafe {
            Arc::increment_strong_count(context);
            Arc::from_raw(context)
        };
        self.bind(Binding::WebGPU(shared), engine, true);
        context
    }

    /// Any thread: read where the context lives.
    pub fn data_url(&self, format: &str, quality: u32) -> Option<String> {
        enum Source {
            TwoD(TargetHandle),
            WebGL(*mut WebGLState),
            WebGPU(Arc<CanvasGPUCanvasContext>),
        }
        let source = match &self.inner().binding {
            Binding::TwoD(handle) => Source::TwoD(handle.clone()),
            Binding::WebGL(state) if state.state().is_threaded() => {
                crate::canvas_native_webgl_state_reference(state.0);
                Source::WebGL(state.0)
            }
            Binding::WebGPU(context) => Source::WebGPU(Arc::clone(context)),
            _ => return None,
        };
        let format = CString::new(format).ok()?;
        match source {
            Source::TwoD(handle) => {
                let format = format.to_string_lossy().into_owned();
                handle.sync(move |real| real.data_url(&format, quality))
            }
            Source::WebGL(state) => {
                let url = crate::canvas_native_webgl_to_data_url(state, format.as_ptr(), quality);
                crate::canvas_native_webgl_state_destroy(state);
                take_c_string(url as *mut c_char)
            }
            Source::WebGPU(context) => {
                let url = unsafe {
                    crate::webgpu::gpu_canvas_context::canvas_native_webgpu_to_data_url(
                        Arc::as_ptr(&context),
                        format.as_ptr(),
                        quality,
                    )
                };
                take_c_string(url)
            }
        }
    }

    /// UI thread, synchronously. The context keeps drawing without the view.
    pub fn detach_view(&self) {
        #[cfg(target_os = "android")]
        self.window_destroyed();
        let sink = {
            let mut inner = self.inner();
            #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos", target_os = "android"))]
            {
                inner.view = None;
            }
            inner.sink.take()
        };
        drop(sink);
    }

    fn set_sink(&self, sink: Option<Sink>) {
        let old = std::mem::replace(&mut self.inner().sink, sink.map(Arc::new));
        drop(old);
    }
}

fn take_c_string(value: *mut c_char) -> Option<String> {
    if value.is_null() {
        return None;
    }
    let string = unsafe { CString::from_raw(value) };
    Some(string.to_string_lossy().into_owned())
}

#[allow(clippy::too_many_arguments)]
fn viewless_2d(
    width: f32,
    height: f32,
    density: f32,
    alpha: bool,
    font_color: i32,
    ppi: f32,
    direction: TextDirection,
    color_space: ColorSpace,
) -> CanvasRenderingContext2D {
    #[cfg(all(feature = "metal", any(target_os = "ios", target_os = "tvos", target_os = "visionos")))]
    let gpu = Some(CanvasRenderingContext2D::new_metal(
        Context::new_metal_offscreen(width, height, density, 1, alpha, font_color, ppi, direction, color_space),
        alpha,
    ));
    #[cfg(all(feature = "gl", target_os = "android"))]
    let gpu = Context::new_gl(std::ptr::null_mut(), width, height, density, alpha, font_color, ppi, direction, color_space)
        .map(|context| CanvasRenderingContext2D::new_gl(context, alpha));
    #[cfg(not(any(
        all(feature = "metal", any(target_os = "ios", target_os = "tvos", target_os = "visionos")),
        all(feature = "gl", target_os = "android")
    )))]
    let gpu: Option<CanvasRenderingContext2D> = None;
    gpu.unwrap_or_else(|| {
        CanvasRenderingContext2D::new(
            Context::new(width, height, density, alpha, font_color, ppi, direction, color_space),
            alpha,
        )
    })
}

fn has_view(inner: &Inner) -> bool {
    #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos", target_os = "android"))]
    let has = inner.view.is_some();
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "visionos", target_os = "android")))]
    let has = {
        let _ = inner;
        false
    };
    has
}

fn resize_webgl_offscreen(state: &WebGLState, width: u32, height: u32) {
    let (width, height) = (width as i32, height as i32);
    #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
    state.post(move |state| state.resize_drawable(width, height));
    #[cfg(target_os = "android")]
    state.post(move |state| {
        state.make_current();
        state.resize_pbuffer(width, height);
    });
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "visionos", target_os = "android")))]
    let _ = (state, width, height);
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebGLAttributes {
    pub version: i32,
    pub alpha: bool,
    pub antialias: bool,
    pub depth: bool,
    pub fail_if_major_performance_caveat: bool,
    pub power_preference: i32,
    pub premultiplied_alpha: bool,
    pub preserve_drawing_buffer: bool,
    pub stencil: bool,
    pub desynchronized: bool,
    pub xr_compatible: bool,
}

impl WebGLAttributes {
    #[cfg_attr(not(any(target_os = "ios", target_os = "tvos", target_os = "visionos")), allow(dead_code))]
    fn power_preference(&self) -> PowerPreference {
        PowerPreference::try_from(self.power_preference).unwrap_or(PowerPreference::Default)
    }
}

static HANDLES: LazyLock<Mutex<HashMap<u32, Arc<CanvasOffscreenSurface>>>> = LazyLock::new(Default::default);

/// Never reused, so a stale handle can't name another surface.
static NEXT_HANDLE: AtomicU32 = AtomicU32::new(1);

fn handles() -> std::sync::MutexGuard<'static, HashMap<u32, Arc<CanvasOffscreenSurface>>> {
    HANDLES.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

unsafe fn surface<'a>(surface: *const CanvasOffscreenSurface) -> Option<&'a CanvasOffscreenSurface> {
    surface.as_ref()
}

#[no_mangle]
pub extern "C" fn canvas_native_offscreen_surface_create(
    width: u32,
    height: u32,
    density: f32,
    ppi: f32,
    direction: u32,
    color_space: CanvasColorSpace,
) -> *const CanvasOffscreenSurface {
    Arc::into_raw(Arc::new(CanvasOffscreenSurface::new(width.max(1), height.max(1), density, ppi, direction, color_space)))
}

/// UI thread. Retains the layers, device and queue; any may be null.
#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
#[no_mangle]
pub extern "C" fn canvas_native_offscreen_surface_create_ios(
    width: u32,
    height: u32,
    density: f32,
    ppi: f32,
    direction: u32,
    color_space: CanvasColorSpace,
    metal_layer: *mut c_void,
    device: *mut c_void,
    queue: *mut c_void,
    samples: usize,
    gl_layer: *mut c_void,
) -> *const CanvasOffscreenSurface {
    let surface = CanvasOffscreenSurface::new(width.max(1), height.max(1), density, ppi, direction, color_space);
    surface.inner().view = Some(apple::View::new(metal_layer, device, queue, samples, gl_layer));
    Arc::into_raw(Arc::new(surface))
}

/// UI thread. The window arrives with `canvas_native_offscreen_surface_set_window`.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn canvas_native_offscreen_surface_create_android(
    width: u32,
    height: u32,
    density: f32,
    ppi: f32,
    direction: u32,
    color_space: CanvasColorSpace,
) -> *const CanvasOffscreenSurface {
    let surface = CanvasOffscreenSurface::new(width.max(1), height.max(1), density, ppi, direction, color_space);
    surface.inner().view = Some(android::View::default());
    Arc::into_raw(Arc::new(surface))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_reference(surface: *const CanvasOffscreenSurface) {
    if !surface.is_null() {
        Arc::increment_strong_count(surface);
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_release(surface: *const CanvasOffscreenSurface) {
    if !surface.is_null() {
        Arc::decrement_strong_count(surface);
    }
}

/// UI thread.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_set_ui_sink(
    surface: *const CanvasOffscreenSurface,
    sink: CanvasOffscreenUiSink,
) {
    match self::surface(surface) {
        Some(surface) => surface.set_sink(Some(Sink(sink))),
        None => drop(Sink(sink)),
    }
}

/// UI thread, synchronously.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_detach_view(surface: *const CanvasOffscreenSurface) {
    if let Some(surface) = self::surface(surface) {
        surface.detach_view();
    }
}

/// UI thread, after the view laid out a resize.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_view_resized(surface: *const CanvasOffscreenSurface) {
    let Some(surface) = self::surface(surface) else { return };
    let inner = surface.inner();
    if let Binding::WebGL(state) = &inner.binding {
        if surface.view_sizes_webgl(&inner) {
            #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
            state.state().post(|state| state.resize_drawable(0, 0));
            #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "visionos")))]
            let _ = state;
        }
    }
}

/// Owner thread.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_resize(surface: *const CanvasOffscreenSurface, width: u32, height: u32) {
    if let Some(surface) = self::surface(surface) {
        surface.resize(width, height);
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_get_width(surface: *const CanvasOffscreenSurface) -> u32 {
    self::surface(surface).map_or(0, |surface| surface.size().0)
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_get_height(surface: *const CanvasOffscreenSurface) -> u32 {
    self::surface(surface).map_or(0, |surface| surface.size().1)
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_get_density(surface: *const CanvasOffscreenSurface) -> f32 {
    self::surface(surface).map_or(1., |surface| surface.density)
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_get_ppi(surface: *const CanvasOffscreenSurface) -> f32 {
    self::surface(surface).map_or(160., |surface| surface.ppi)
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_get_direction(surface: *const CanvasOffscreenSurface) -> u32 {
    self::surface(surface).map_or(0, |surface| surface.direction)
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_has_view(surface: *const CanvasOffscreenSurface) -> bool {
    self::surface(surface).is_some_and(|surface| surface.has_view())
}

/// Owner thread. Null if the surface has a context.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_create_2d(
    surface: *const CanvasOffscreenSurface,
    alpha: bool,
    font_color: i32,
    threaded: bool,
) -> *mut CanvasRenderingContext2D {
    match self::surface(surface) {
        Some(surface) => surface.create_2d(alpha, font_color, threaded),
        None => std::ptr::null_mut(),
    }
}

/// Owner thread. Null if the surface has a context.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_create_webgl(
    surface: *const CanvasOffscreenSurface,
    attributes: *const WebGLAttributes,
    threaded: bool,
) -> *mut WebGLState {
    match (self::surface(surface), attributes.as_ref()) {
        (Some(surface), Some(attributes)) => surface.create_webgl(attributes, threaded),
        _ => std::ptr::null_mut(),
    }
}

/// Owner thread. Null if the surface has a context.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_create_webgpu(
    surface: *const CanvasOffscreenSurface,
    instance: *const CanvasWebGPUInstance,
) -> *const CanvasGPUCanvasContext {
    match self::surface(surface) {
        Some(surface) => surface.create_webgpu(instance),
        None => std::ptr::null(),
    }
}

/// Any thread. Null without a context.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_to_data_url(
    surface: *const CanvasOffscreenSurface,
    format: *const c_char,
    quality: u32,
) -> *mut c_char {
    let (Some(surface), false) = (self::surface(surface), format.is_null()) else {
        return std::ptr::null_mut();
    };
    let format = CStr::from_ptr(format).to_string_lossy();
    surface
        .data_url(&format, quality)
        .and_then(|url| CString::new(url).ok())
        .map_or(std::ptr::null_mut(), CString::into_raw)
}

/// 0 if the surface has a context. The handle holds a reference until adopted or released.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_offscreen_surface_to_handle(surface: *const CanvasOffscreenSurface) -> u32 {
    if surface.is_null() || (*surface).has_context() {
        return 0;
    }
    Arc::increment_strong_count(surface);
    let surface = Arc::from_raw(surface);
    let handle = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    handles().insert(handle, surface);
    handle
}

/// Take-once: null after the first adopt or a release.
#[no_mangle]
pub extern "C" fn canvas_native_offscreen_surface_adopt(handle: u32) -> *const CanvasOffscreenSurface {
    handles().remove(&handle).map_or(std::ptr::null(), Arc::into_raw)
}

#[no_mangle]
pub extern "C" fn canvas_native_offscreen_surface_release_handle(handle: u32) -> bool {
    let surface = handles().remove(&handle);
    surface.is_some()
}
