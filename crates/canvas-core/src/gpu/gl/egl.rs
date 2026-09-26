//! `GLContext` over a dynamically loaded EGL, for desktop platforms.
//!
//! On Windows the EGL/GLES implementation is ANGLE (`libEGL.dll` + `libGLESv2.dll`, shipped next
//! to the canvas binary) running on Direct3D 11. Other desktop EGL implementations can reuse this
//! file by adding a `platform_display` for their native display type.
//!
//! Surfaces are pbuffers. Presenting to the screen goes through a D3D11 texture shared with
//! ANGLE (see `create_texture_surface`); window surfaces are not used because the app's native
//! view is a WinUI `SwapChainPanel`, not an HWND.

use std::cell::Cell;
use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};

use khronos_egl as egl;
use parking_lot::RwLock;

use crate::context_attributes::ContextAttributes;

pub static IS_GL_SYMBOLS_LOADED: OnceLock<bool> = OnceLock::new();

type EglInstance = egl::DynamicInstance<egl::EGL1_5>;

// EGL_ANGLE_platform_angle / EGL_ANGLE_device_d3d / EGL_ANGLE_d3d_texture_client_buffer /
// EGL_ANGLE_create_context_webgl_compatibility / EGL_ANGLE_robust_resource_initialization.
const EGL_PLATFORM_ANGLE_ANGLE: egl::Enum = 0x3202;
const EGL_PLATFORM_ANGLE_TYPE_ANGLE: egl::Attrib = 0x3203;
const EGL_PLATFORM_ANGLE_TYPE_D3D11_ANGLE: egl::Attrib = 0x3208;
const EGL_PLATFORM_ANGLE_DEVICE_TYPE_ANGLE: egl::Attrib = 0x3209;
const EGL_PLATFORM_ANGLE_DEVICE_TYPE_HARDWARE_ANGLE: egl::Attrib = 0x320A;
const EGL_PLATFORM_ANGLE_DEVICE_TYPE_D3D_WARP_ANGLE: egl::Attrib = 0x320B;
const EGL_DEVICE_EXT: egl::Int = 0x322C;
const EGL_D3D11_DEVICE_ANGLE: egl::Int = 0x33A1;
const EGL_D3D_TEXTURE_ANGLE: egl::Enum = 0x33A3;
const EGL_CONTEXT_WEBGL_COMPATIBILITY_ANGLE: egl::Int = 0x33AC;
const EGL_ROBUST_RESOURCE_INITIALIZATION_ANGLE: egl::Int = 0x3453;

type QueryDisplayAttribExt =
    unsafe extern "system" fn(egl::EGLDisplay, egl::Int, *mut egl::Attrib) -> egl::Boolean;
type QueryDeviceAttribExt =
    unsafe extern "system" fn(*mut c_void, egl::Int, *mut egl::Attrib) -> egl::Boolean;

/// The process-wide EGL display. Like Android's shared display, there is exactly one owner so
/// tearing down one canvas can never terminate the display under the others.
struct Egl {
    instance: EglInstance,
    display: egl::Display,
    extensions: String,
    // Keeps libGLESv2 mapped for as long as the display lives.
    _gles: libloading::Library,
}

// The display handle is process-wide in EGL; contexts are what is thread-affine.
unsafe impl Send for Egl {}
unsafe impl Sync for Egl {}

static EGL: OnceLock<Option<Egl>> = OnceLock::new();

fn env_flag(name: &str) -> bool {
    std::env::var_os(name).is_some_and(|v| !v.is_empty() && v != "0")
}

/// Where the ANGLE DLLs live: `CANVAS_ANGLE_DIR`, else the directory of the module containing
/// this code (the canvas addon ships them side by side), else the default DLL search order.
fn angle_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("CANVAS_ANGLE_DIR") {
        return Some(PathBuf::from(dir));
    }
    own_module_dir()
}

#[cfg(target_os = "windows")]
fn own_module_dir() -> Option<PathBuf> {
    use std::os::windows::ffi::OsStringExt;
    type Hmodule = *mut c_void;
    const FROM_ADDRESS: u32 = 0x4;
    const UNCHANGED_REFCOUNT: u32 = 0x2;
    #[link(name = "kernel32")]
    extern "system" {
        fn GetModuleHandleExW(flags: u32, name: *const u16, module: *mut Hmodule) -> i32;
        fn GetModuleFileNameW(module: Hmodule, name: *mut u16, size: u32) -> u32;
    }
    unsafe {
        let mut module: Hmodule = std::ptr::null_mut();
        let anchor = own_module_dir as *const u16;
        if GetModuleHandleExW(FROM_ADDRESS | UNCHANGED_REFCOUNT, anchor, &mut module) == 0 {
            return None;
        }
        let mut path = vec![0u16; 32768];
        let len = GetModuleFileNameW(module, path.as_mut_ptr(), path.len() as u32) as usize;
        if len == 0 {
            return None;
        }
        let path = PathBuf::from(std::ffi::OsString::from_wide(&path[..len]));
        path.parent().map(Path::to_path_buf)
    }
}

#[cfg(not(target_os = "windows"))]
fn own_module_dir() -> Option<PathBuf> {
    None
}

#[cfg(target_os = "windows")]
const LIB_EGL: &str = "libEGL.dll";
#[cfg(target_os = "windows")]
const LIB_GLES: &str = "libGLESv2.dll";
#[cfg(not(target_os = "windows"))]
const LIB_EGL: &str = "libEGL.so.1";
#[cfg(not(target_os = "windows"))]
const LIB_GLES: &str = "libGLESv2.so.2";

fn resolve(dir: Option<&Path>, name: &str) -> PathBuf {
    match dir {
        Some(dir) if dir.join(name).exists() => dir.join(name),
        _ => PathBuf::from(name),
    }
}

fn load() -> Option<Egl> {
    let dir = angle_dir();
    // libEGL resolves libGLESv2 by module name, so load it first by full path: the default
    // search order could otherwise pick up an unrelated copy (a GPU vendor's or a browser's).
    let gles = match unsafe { libloading::Library::new(resolve(dir.as_deref(), LIB_GLES)) } {
        Ok(lib) => lib,
        Err(error) => {
            log::error!("canvas: could not load {LIB_GLES}: {error}");
            return None;
        }
    };
    let instance = match unsafe {
        EglInstance::load_required_from_filename(resolve(dir.as_deref(), LIB_EGL))
    } {
        Ok(instance) => instance,
        Err(error) => {
            log::error!("canvas: could not load {LIB_EGL}: {error}");
            return None;
        }
    };

    let display = platform_display(&instance)?;
    if let Err(error) = instance.initialize(display) {
        log::error!("canvas: eglInitialize failed: {error}");
        return None;
    }
    let extensions = instance
        .query_string(Some(display), egl::EXTENSIONS)
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();

    Some(Egl {
        instance,
        display,
        extensions,
        _gles: gles,
    })
}

#[cfg(target_os = "windows")]
fn platform_display(instance: &EglInstance) -> Option<egl::Display> {
    let device = if env_flag("CANVAS_FORCE_WARP") {
        EGL_PLATFORM_ANGLE_DEVICE_TYPE_D3D_WARP_ANGLE
    } else {
        EGL_PLATFORM_ANGLE_DEVICE_TYPE_HARDWARE_ANGLE
    };
    let attribs = [
        EGL_PLATFORM_ANGLE_TYPE_ANGLE,
        EGL_PLATFORM_ANGLE_TYPE_D3D11_ANGLE,
        EGL_PLATFORM_ANGLE_DEVICE_TYPE_ANGLE,
        device,
        egl::NONE as egl::Attrib,
    ];
    match unsafe {
        instance.get_platform_display(EGL_PLATFORM_ANGLE_ANGLE, egl::DEFAULT_DISPLAY, &attribs)
    } {
        Ok(display) => Some(display),
        Err(error) => {
            log::error!("canvas: eglGetPlatformDisplay(ANGLE, D3D11) failed: {error}");
            None
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn platform_display(instance: &EglInstance) -> Option<egl::Display> {
    unsafe { instance.get_display(egl::DEFAULT_DISPLAY) }
}

fn shared() -> Option<&'static Egl> {
    let egl = EGL.get_or_init(load).as_ref()?;
    IS_GL_SYMBOLS_LOADED.get_or_init(|| {
        gl_bindings::load_with(|symbol| {
            egl.instance
                .get_proc_address(symbol)
                .map_or(std::ptr::null(), |f| f as *const c_void)
        });
        true
    });
    Some(egl)
}

impl Egl {
    fn has_extension(&self, name: &str) -> bool {
        self.extensions.split(' ').any(|e| e == name)
    }
}

/// ANGLE's own D3D11 device. Textures that ANGLE renders into must be created on it.
#[cfg(target_os = "windows")]
pub fn angle_d3d11_device() -> Option<windows::Win32::Graphics::Direct3D11::ID3D11Device> {
    use windows::core::Interface;
    let egl = shared()?;
    let query_display: QueryDisplayAttribExt =
        unsafe { std::mem::transmute(egl.instance.get_proc_address("eglQueryDisplayAttribEXT")?) };
    let query_device: QueryDeviceAttribExt =
        unsafe { std::mem::transmute(egl.instance.get_proc_address("eglQueryDeviceAttribEXT")?) };
    let mut device: egl::Attrib = 0;
    if unsafe { query_display(egl.display.as_ptr(), EGL_DEVICE_EXT, &mut device) } != egl::TRUE {
        return None;
    }
    let mut d3d11: egl::Attrib = 0;
    if unsafe { query_device(device as *mut c_void, EGL_D3D11_DEVICE_ANGLE, &mut d3d11) } != egl::TRUE {
        return None;
    }
    let raw = d3d11 as *mut c_void;
    unsafe { windows::Win32::Graphics::Direct3D11::ID3D11Device::from_raw_borrowed(&raw) }.cloned()
}

thread_local! {
    /// Mirrors what this thread last bound so `make_current` can skip the call. Keyed on the
    /// surface as well as the context, since a context is rebound on resize.
    static CURRENT_EGL_BINDING: Cell<Binding> = const { Cell::new(UNBOUND) };
}

/// (context, draw surface, epoch). The epoch changes whenever any surface is replaced, so a
/// recycled `EGLSurface` address can never match a stale entry.
type Binding = (usize, usize, usize);
const UNBOUND: Binding = (0, 0, 0);
static SURFACE_EPOCH: AtomicUsize = AtomicUsize::new(1);

fn binding(context: egl::Context, surface: egl::Surface) -> Binding {
    (
        context.as_ptr() as usize,
        surface.as_ptr() as usize,
        SURFACE_EPOCH.load(Ordering::Relaxed),
    )
}

fn invalidate_surface_bindings() {
    SURFACE_EPOCH.fetch_add(1, Ordering::Relaxed);
}

fn bind(context: egl::Context, surface: egl::Surface) -> bool {
    let target = binding(context, surface);
    if CURRENT_EGL_BINDING.with(|c| c.get()) == target {
        return true;
    }
    let Some(egl) = shared() else { return false };
    let bound = egl
        .instance
        .make_current(egl.display, Some(surface), Some(surface), Some(context))
        .is_ok();
    CURRENT_EGL_BINDING.with(|c| c.set(if bound { target } else { UNBOUND }));
    bound
}

fn unbind_if_current(context: egl::Context) -> bool {
    let Some(egl) = shared() else { return false };
    if egl.instance.get_current_context() != Some(context) {
        return false;
    }
    let unbound = egl.instance.make_current(egl.display, None, None, None).is_ok();
    if unbound {
        CURRENT_EGL_BINDING.with(|c| c.set(UNBOUND));
    }
    unbound
}

#[derive(Debug, Copy, Clone, Default)]
struct Dimensions {
    width: i32,
    height: i32,
}

#[derive(Debug, Clone)]
pub struct GLContextRaw {
    surface: Option<egl::Surface>,
    context: Option<egl::Context>,
}

impl GLContextRaw {
    pub fn make_current(&self) -> bool {
        match (self.context, self.surface) {
            (Some(context), Some(surface)) => bind(context, surface),
            _ => false,
        }
    }

    pub fn remove_if_current(&self) -> bool {
        self.context.is_some_and(unbind_if_current)
    }
}

#[derive(Default)]
pub(crate) struct GLContextInner {
    context: Option<egl::Context>,
    surface: Option<egl::Surface>,
    config: Option<egl::Config>,
    #[cfg(target_os = "windows")]
    texture: Option<windows::Win32::Graphics::Direct3D11::ID3D11Texture2D>,
    dimensions: Arc<RwLock<Dimensions>>,
}

#[derive(Default)]
pub struct GLContext(GLContextInner);

impl Debug for GLContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GLContext")
            .field("context", &self.0.context)
            .field("surface", &self.0.surface)
            .finish()
    }
}

impl Drop for GLContext {
    fn drop(&mut self) {
        let Some(egl) = EGL.get().and_then(Option::as_ref) else { return };
        if let Some(context) = self.0.context {
            unbind_if_current(context);
        }
        if let Some(surface) = self.0.surface.take() {
            invalidate_surface_bindings();
            let _ = egl.instance.destroy_surface(egl.display, surface);
        }
        if let Some(context) = self.0.context.take() {
            let _ = egl.instance.destroy_context(egl.display, context);
        }
    }
}

fn wants_gles3(attrs: &ContextAttributes) -> bool {
    !(attrs.get_is_canvas() || attrs.get_gl_legacy())
}

fn choose_config(egl: &Egl, attrs: &ContextAttributes) -> Option<egl::Config> {
    let renderable = if wants_gles3(attrs) {
        egl::OPENGL_ES3_BIT
    } else {
        egl::OPENGL_ES2_BIT
    };
    let multisample = !attrs.get_is_canvas() && attrs.get_antialias();
    let mut candidates = Vec::with_capacity(3);
    for (depth, samples) in [
        (if attrs.get_depth() { 24 } else { 0 }, if multisample { 4 } else { 0 }),
        (if attrs.get_depth() { 24 } else { 0 }, 0),
        (if attrs.get_depth() { 16 } else { 0 }, 0),
    ] {
        candidates.push([
            egl::RED_SIZE,
            8,
            egl::GREEN_SIZE,
            8,
            egl::BLUE_SIZE,
            8,
            egl::ALPHA_SIZE,
            if attrs.get_alpha() { 8 } else { 0 },
            egl::DEPTH_SIZE,
            depth,
            egl::STENCIL_SIZE,
            if attrs.get_stencil() { 8 } else { 0 },
            egl::SAMPLE_BUFFERS,
            (samples > 0) as i32,
            egl::SAMPLES,
            samples,
            egl::SURFACE_TYPE,
            egl::PBUFFER_BIT,
            egl::RENDERABLE_TYPE,
            renderable,
            egl::NONE,
        ]);
    }
    candidates.iter().find_map(|attribs| {
        egl.instance
            .choose_first_config(egl.display, attribs)
            .ok()
            .flatten()
    })
}

fn create_context(egl: &Egl, config: egl::Config, attrs: &ContextAttributes) -> Option<egl::Context> {
    egl.instance.bind_api(egl::OPENGL_ES_API).ok()?;
    let mut context_attribs = vec![
        egl::CONTEXT_MAJOR_VERSION,
        if wants_gles3(attrs) { 3 } else { 2 },
    ];
    // WebGL contexts get ANGLE's WebGL validation and zero-initialised resources; the 2D
    // canvas's Skia context must not (Skia relies on extensions WebGL mode hides).
    if !attrs.get_is_canvas() {
        if egl.has_extension("EGL_ANGLE_create_context_webgl_compatibility") {
            context_attribs.extend([EGL_CONTEXT_WEBGL_COMPATIBILITY_ANGLE, egl::TRUE as i32]);
        }
        if egl.has_extension("EGL_ANGLE_robust_resource_initialization") {
            context_attribs.extend([EGL_ROBUST_RESOURCE_INITIALIZATION_ANGLE, egl::TRUE as i32]);
        }
    }
    context_attribs.push(egl::NONE);
    match egl.instance.create_context(egl.display, config, None, &context_attribs) {
        Ok(context) => Some(context),
        Err(error) => {
            log::error!("canvas: eglCreateContext failed: {error}");
            None
        }
    }
}

fn create_pbuffer_surface(egl: &Egl, config: egl::Config, width: i32, height: i32) -> Option<egl::Surface> {
    egl.instance
        .create_pbuffer_surface(
            egl.display,
            config,
            &[egl::WIDTH, width.max(1), egl::HEIGHT, height.max(1), egl::NONE],
        )
        .ok()
}

impl GLContext {
    pub fn as_raw(&self) -> GLContextRaw {
        GLContextRaw {
            surface: self.0.surface,
            context: self.0.context,
        }
    }

    pub fn has_gl2support() -> bool {
        let Some(egl) = shared() else { return false };
        egl.instance
            .choose_first_config(
                egl.display,
                &[egl::RENDERABLE_TYPE, egl::OPENGL_ES3_BIT, egl::NONE],
            )
            .ok()
            .flatten()
            .is_some()
    }

    pub fn create_offscreen_context(
        attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) -> Option<Self> {
        let egl = shared()?;
        let config = choose_config(egl, attrs)?;
        let context = create_context(egl, config, attrs)?;
        let Some(surface) = create_pbuffer_surface(egl, config, width, height) else {
            let _ = egl.instance.destroy_context(egl.display, context);
            return None;
        };
        Some(GLContext(GLContextInner {
            context: Some(context),
            surface: Some(surface),
            config: Some(config),
            #[cfg(target_os = "windows")]
            texture: None,
            dimensions: Arc::new(RwLock::new(Dimensions {
                width: width.max(1),
                height: height.max(1),
            })),
        }))
    }

    /// Replaces the pbuffer with one of the new size. The context (and every GL object in it)
    /// is kept.
    pub fn resize_pbuffer(&mut self, _attrs: &mut ContextAttributes, width: i32, height: i32) {
        let (Some(egl), Some(config)) = (shared(), self.0.config) else { return };
        let Some(surface) = create_pbuffer_surface(egl, config, width, height) else { return };
        self.replace_surface(egl, surface);
        #[cfg(target_os = "windows")]
        {
            self.0.texture = None;
        }
        *self.0.dimensions.write() = Dimensions {
            width: width.max(1),
            height: height.max(1),
        };
    }

    /// Makes the context render into `texture`, a D3D11 texture created on
    /// [`angle_d3d11_device`] with render-target and shader-resource bind flags. The GL
    /// framebuffer is bottom-up relative to the texture; the presenter flips it.
    #[cfg(target_os = "windows")]
    pub fn set_texture_surface(
        &mut self,
        texture: windows::Win32::Graphics::Direct3D11::ID3D11Texture2D,
        width: i32,
        height: i32,
    ) -> bool {
        use windows::core::Interface;
        let (Some(egl), Some(config)) = (shared(), self.0.config) else { return false };
        let buffer = unsafe { egl::ClientBuffer::from_ptr(texture.as_raw()) };
        let attribs = [
            egl::TEXTURE_FORMAT,
            egl::TEXTURE_RGBA,
            egl::TEXTURE_TARGET,
            egl::TEXTURE_2D,
            egl::NONE,
        ];
        let surface = match egl.instance.create_pbuffer_from_client_buffer(
            egl.display,
            EGL_D3D_TEXTURE_ANGLE,
            buffer,
            config,
            &attribs,
        ) {
            Ok(surface) => surface,
            Err(error) => {
                log::error!("canvas: eglCreatePbufferFromClientBuffer failed: {error}");
                return false;
            }
        };
        self.replace_surface(egl, surface);
        self.0.texture = Some(texture);
        *self.0.dimensions.write() = Dimensions {
            width: width.max(1),
            height: height.max(1),
        };
        true
    }

    #[cfg(target_os = "windows")]
    pub fn texture(&self) -> Option<&windows::Win32::Graphics::Direct3D11::ID3D11Texture2D> {
        self.0.texture.as_ref()
    }

    fn replace_surface(&mut self, egl: &Egl, surface: egl::Surface) {
        let rebind = self
            .0
            .context
            .is_some_and(|context| egl.instance.get_current_context() == Some(context));
        if let Some(old) = self.0.surface.replace(surface) {
            invalidate_surface_bindings();
            if rebind {
                let _ = egl.instance.make_current(egl.display, None, None, None);
                CURRENT_EGL_BINDING.with(|c| c.set(UNBOUND));
            }
            let _ = egl.instance.destroy_surface(egl.display, old);
        }
        if rebind {
            self.make_current();
        }
    }

    pub fn set_vsync(&self, sync: bool) -> bool {
        // Pbuffers are never presented through EGL; pacing comes from the swapchain.
        let Some(egl) = shared() else { return false };
        self.make_current() && egl.instance.swap_interval(egl.display, sync as i32).is_ok()
    }

    pub fn make_current(&self) -> bool {
        match (self.0.context, self.0.surface) {
            (Some(context), Some(surface)) => bind(context, surface),
            _ => false,
        }
    }

    pub fn remove_if_current(&self) -> bool {
        self.0.context.is_some_and(unbind_if_current)
    }

    /// Pbuffers have nothing to swap; this flushes so the texture/pbuffer contents are complete
    /// before they are read or copied by the presenter.
    pub fn swap_buffers(&self) -> bool {
        if !self.make_current() {
            return false;
        }
        unsafe { gl_bindings::Flush() };
        true
    }

    pub fn get_surface_width(&self) -> i32 {
        self.0.dimensions.read().width
    }

    pub fn get_surface_height(&self) -> i32 {
        self.0.dimensions.read().height
    }

    pub fn get_surface_dimensions(&self) -> (i32, i32) {
        let lock = self.0.dimensions.read();
        (lock.width, lock.height)
    }
}
