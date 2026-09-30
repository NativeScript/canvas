//! `GLContext` over a dynamically loaded EGL, for desktop platforms.
//!
//! On Windows the EGL/GLES implementation is ANGLE (`libEGL.dll` + `libGLESv2.dll`, shipped next
//! to the canvas binary) running on Direct3D 11. Other desktop EGL implementations can reuse this
//! file by adding a `platform_display` for their native display type.
//!
//! Surfaces are pbuffers. Presenting to the screen goes through a D3D11 texture shared with
//! ANGLE (see `create_texture_context`) that is copied into a composition swapchain; window
//! surfaces are not used because the app's native view is a WinUI `SwapChainPanel`, not an HWND.

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
// EGL_EXT_create_context_robustness: a device loss (driver reset, TDR) is reported to the
// context (glGetGraphicsResetStatusEXT) instead of going unnoticed.
const EGL_CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY_EXT: egl::Int = 0x3138;
const EGL_LOSE_CONTEXT_ON_RESET_EXT: egl::Int = 0x31BF;

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

    let egl = Egl {
        instance,
        display,
        extensions,
        _gles: gles,
    };
    #[cfg(target_os = "windows")]
    protect_immediate_context(&egl);
    Some(egl)
}

/// Threaded WebGL contexts copy frames with ANGLE's immediate context outside ANGLE's lock while
/// contexts on other threads call into ANGLE, so D3D has to serialize the two.
#[cfg(target_os = "windows")]
fn protect_immediate_context(egl: &Egl) {
    use windows::core::Interface;
    use windows::Win32::Graphics::Direct3D11::ID3D11Multithread;
    let Some(device) = d3d11_device(egl) else { return };
    if let Ok(multithread) = unsafe { device.GetImmediateContext() }.and_then(|context| context.cast::<ID3D11Multithread>()) {
        let _ = unsafe { multithread.SetMultithreadProtected(true) };
    }
}

/// ANGLE on a D3D11 device of ours, made with BGRA support: XAML SurfaceImageSources (what
/// transparent canvases present into) only take such a device, and ANGLE's own lacks it.
#[cfg(target_os = "windows")]
fn device_display(instance: &EglInstance) -> Option<egl::Display> {
    use windows::Win32::Foundation::HMODULE;
    use windows::Win32::Graphics::Direct3D::{
        D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1,
        D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1,
    };
    use windows::Win32::Graphics::Direct3D11::{D3D11CreateDevice, ID3D11Device, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION};
    const EGL_PLATFORM_DEVICE_EXT: egl::Enum = 0x313F;

    let client = instance
        .query_string(None, egl::EXTENSIONS)
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let has = |name: &str| client.split(' ').any(|e| e == name);
    if !has("EGL_ANGLE_device_creation") || !has("EGL_ANGLE_device_creation_d3d11") || !has("EGL_EXT_platform_device") {
        return None;
    }
    let levels = [D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_10_0];
    let create = |driver: D3D_DRIVER_TYPE| {
        let mut device: Option<ID3D11Device> = None;
        unsafe {
            D3D11CreateDevice(
                None,
                driver,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                Some(&levels),
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                None,
            )
        }
        .ok()
        .and(device)
    };
    let device = if env_flag("CANVAS_FORCE_WARP") {
        create(D3D_DRIVER_TYPE_WARP)
    } else {
        create(D3D_DRIVER_TYPE_HARDWARE).or_else(|| create(D3D_DRIVER_TYPE_WARP))
    }?;

    type CreateDeviceAngle = unsafe extern "system" fn(egl::Int, *mut c_void, *const egl::Attrib) -> *mut c_void;
    let create_device: CreateDeviceAngle =
        unsafe { std::mem::transmute(instance.get_proc_address("eglCreateDeviceANGLE")?) };
    let egl_device = unsafe { create_device(EGL_D3D11_DEVICE_ANGLE, windows::core::Interface::as_raw(&device), std::ptr::null()) };
    if egl_device.is_null() {
        log::warn!("canvas: eglCreateDeviceANGLE failed; ANGLE makes its own D3D11 device");
        return None;
    }
    // The display lives as long as the process; keep the device with it.
    std::mem::forget(device);
    match unsafe { instance.get_platform_display(EGL_PLATFORM_DEVICE_EXT, egl_device, &[egl::NONE as egl::Attrib]) } {
        Ok(display) => Some(display),
        Err(error) => {
            log::warn!("canvas: eglGetPlatformDisplay(device) failed: {error}");
            None
        }
    }
}

#[cfg(target_os = "windows")]
fn platform_display(instance: &EglInstance) -> Option<egl::Display> {
    if let Some(display) = device_display(instance) {
        return Some(display);
    }
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

/// A GL/EGL entry point from the loaded EGL implementation (null if unavailable).
pub fn get_proc_address(name: &str) -> *const c_void {
    shared()
        .and_then(|egl| egl.instance.get_proc_address(name))
        .map_or(std::ptr::null(), |f| f as *const c_void)
}

impl Egl {
    fn has_extension(&self, name: &str) -> bool {
        self.extensions.split(' ').any(|e| e == name)
    }
}

/// ANGLE's own D3D11 device. Textures that ANGLE renders into must be created on it.
#[cfg(target_os = "windows")]
pub fn angle_d3d11_device() -> Option<windows::Win32::Graphics::Direct3D11::ID3D11Device> {
    d3d11_device(shared()?)
}

#[cfg(target_os = "windows")]
fn d3d11_device(egl: &Egl) -> Option<windows::Win32::Graphics::Direct3D11::ID3D11Device> {
    use windows::core::Interface;
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
    /// On screen: the panel's swapchain and ANGLE's immediate context that copies into it.
    #[cfg(target_os = "windows")]
    presenter: Option<Presenter>,
    /// On screen and blending with the page: a XAML SurfaceImageSource on ANGLE's device.
    #[cfg(target_os = "windows")]
    xaml: Option<crate::gpu::dxgi::XamlSurface>,
    dimensions: Arc<RwLock<Dimensions>>,
    /// Set once the context reports a reset: a lost context stays lost.
    #[cfg(target_os = "windows")]
    lost: std::sync::atomic::AtomicBool,
}

#[cfg(target_os = "windows")]
pub(crate) struct Presenter {
    swap_chain: crate::gpu::dxgi::CompositionSwapChain,
    context: windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext,
    /// A frame the display wasn't ready for, copied out of the drawing buffer so it can be shown
    /// later without what's drawn next (`present_or_hold`).
    held: parking_lot::Mutex<Option<windows::Win32::Graphics::Direct3D11::ID3D11Texture2D>>,
}

#[cfg(target_os = "windows")]
impl Presenter {
    fn new(swap_chain: crate::gpu::dxgi::CompositionSwapChain, context: windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext) -> Self {
        Self {
            swap_chain,
            context,
            held: parking_lot::Mutex::new(None),
        }
    }

    fn present_from(&self, texture: &windows::Win32::Graphics::Direct3D11::ID3D11Texture2D) -> bool {
        let Ok(back_buffer) = self.swap_chain.buffer::<windows::Win32::Graphics::Direct3D11::ID3D11Texture2D>(0) else {
            return false;
        };
        unsafe { self.context.CopyResource(&back_buffer, texture) };
        drop(back_buffer);
        self.swap_chain.present(true).is_ok()
    }
}

#[cfg(target_os = "windows")]
fn hold_frame(
    presenter: &Presenter,
    texture: &windows::Win32::Graphics::Direct3D11::ID3D11Texture2D,
    reuse: Option<windows::Win32::Graphics::Direct3D11::ID3D11Texture2D>,
) -> Option<windows::Win32::Graphics::Direct3D11::ID3D11Texture2D> {
    use windows::Win32::Graphics::Direct3D11::D3D11_TEXTURE2D_DESC;
    let mut desc = D3D11_TEXTURE2D_DESC::default();
    unsafe { texture.GetDesc(&mut desc) };
    let fits = |held: &windows::Win32::Graphics::Direct3D11::ID3D11Texture2D| {
        let mut held_desc = D3D11_TEXTURE2D_DESC::default();
        unsafe { held.GetDesc(&mut held_desc) };
        (held_desc.Width, held_desc.Height) == (desc.Width, desc.Height)
    };
    let held = match reuse.filter(fits) {
        Some(held) => held,
        None => create_render_texture(desc.Width as i32, desc.Height as i32)?,
    };
    unsafe { presenter.context.CopyResource(&held, texture) };
    Some(held)
}

/// A BGRA texture on ANGLE's device that a pbuffer can wrap and a swapchain buffer can be
/// copied from.
#[cfg(target_os = "windows")]
fn create_render_texture(width: i32, height: i32) -> Option<windows::Win32::Graphics::Direct3D11::ID3D11Texture2D> {
    use windows::Win32::Graphics::Direct3D11::*;
    use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};
    let device = angle_d3d11_device()?;
    let desc = D3D11_TEXTURE2D_DESC {
        Width: width.max(1) as u32,
        Height: height.max(1) as u32,
        MipLevels: 1,
        ArraySize: 1,
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: (D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
        ..Default::default()
    };
    let mut texture = None;
    match unsafe { device.CreateTexture2D(&desc, None, Some(&mut texture)) } {
        Ok(()) => texture,
        Err(error) => {
            log::error!("canvas: could not create the WebGL render texture: {error}");
            None
        }
    }
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
        if egl.has_extension("EGL_EXT_create_context_robustness") {
            context_attribs.extend([
                EGL_CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY_EXT,
                EGL_LOSE_CONTEXT_ON_RESET_EXT,
            ]);
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
            #[cfg(target_os = "windows")]
            presenter: None,
            #[cfg(target_os = "windows")]
            xaml: None,
            dimensions: Arc::new(RwLock::new(Dimensions {
                width: width.max(1),
                height: height.max(1),
            })),
            #[cfg(target_os = "windows")]
            lost: Default::default(),
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

    /// A context whose default framebuffer is a D3D11 texture, so it can be presented in a
    /// `SwapChainPanel` (`attach_swap_chain_panel`). Single-sampled: ANGLE cannot wrap a
    /// multisampled texture, so `antialias` is reported as false.
    #[cfg(target_os = "windows")]
    pub fn create_texture_context(attrs: &mut ContextAttributes, width: i32, height: i32) -> Option<Self> {
        let egl = shared()?;
        attrs.set_antialias(false);
        let config = choose_config(egl, attrs)?;
        let context = create_context(egl, config, attrs)?;
        let mut gl = GLContext(GLContextInner {
            context: Some(context),
            config: Some(config),
            dimensions: Arc::new(RwLock::new(Dimensions {
                width: width.max(1),
                height: height.max(1),
            })),
            ..Default::default()
        });
        let texture = create_render_texture(width, height)?;
        gl.set_texture_surface(texture, width, height).then_some(gl)
    }

    /// Resizes a texture context (its GL objects are kept) and its swapchain. The contents are
    /// cleared, as resizing a canvas does.
    #[cfg(target_os = "windows")]
    pub fn resize_texture_surface(&mut self, width: i32, height: i32) -> bool {
        if self.0.texture.is_none() {
            return false;
        }
        let Some(texture) = create_render_texture(width, height) else { return false };
        if !self.set_texture_surface(texture, width, height) {
            return false;
        }
        // A SurfaceImageSource has a fixed size: the host attaches one of the new size.
        self.0.xaml = None;
        match self.0.presenter.as_mut() {
            Some(presenter) => {
                *presenter.held.lock() = None;
                presenter.swap_chain.resize(width.max(1) as u32, height.max(1) as u32).is_ok()
            }
            None => true,
        }
    }

    /// Presents into a XAML `SurfaceImageSource` through `handoff` (made on the UI thread with
    /// [`angle_d3d11_device`] at the context's size), from the thread that owns the context.
    #[cfg(target_os = "windows")]
    pub fn attach_xaml_handoff(&mut self, handoff: Arc<crate::gpu::dxgi::XamlHandoff>) -> bool {
        if self.0.texture.is_none() {
            return false;
        }
        let Some(device) = angle_d3d11_device() else { return false };
        self.0.presenter = None;
        self.0.xaml = match crate::gpu::dxgi::XamlSurface::with_handoff(handoff, &device) {
            Ok(surface) => Some(surface),
            Err(error) => {
                log::error!("canvas: could not use the XAML surface for WebGL: {error}");
                None
            }
        };
        self.0.xaml.is_some()
    }

    /// Presents a texture context into a XAML `SurfaceImageSource` (any COM pointer to it, made
    /// at the context's size) instead of a swapchain, so it blends with the page. The rows stay
    /// bottom-up: the host flips the image. UI thread.
    #[cfg(target_os = "windows")]
    pub unsafe fn attach_xaml_surface(&mut self, source: *mut c_void) -> bool {
        if self.0.texture.is_none() {
            return false;
        }
        let Some(device) = angle_d3d11_device() else { return false };
        let (width, height) = self.get_surface_dimensions();
        self.0.presenter = None;
        self.0.xaml = match unsafe { crate::gpu::dxgi::XamlSurface::new(source, &device, width.max(1) as u32, height.max(1) as u32) } {
            Ok(surface) => Some(surface),
            Err(error) => {
                log::error!("canvas: could not use the XAML surface for WebGL: {error}");
                None
            }
        };
        self.0.xaml.is_some()
    }

    /// Shows a texture context in a WinUI `SwapChainPanel` (any COM pointer to it). UI thread.
    #[cfg(target_os = "windows")]
    pub unsafe fn attach_swap_chain_panel(&mut self, panel: *mut c_void, alpha: bool) -> bool {
        use windows::core::Interface;
        let Some(swap_chain) = self.create_panel_swap_chain(alpha) else { return false };
        if let Err(error) = unsafe { crate::gpu::dxgi::bind_swap_chain(panel, swap_chain.as_raw()) } {
            log::error!("canvas: could not show the WebGL swapchain in its panel: {error}");
            self.0.presenter = None;
            return false;
        }
        true
    }

    /// The swapchain a texture context now presents into, for the UI thread to show in a
    /// `SwapChainPanel` (`dxgi::bind_swap_chain`). The thread that owns the context.
    #[cfg(target_os = "windows")]
    pub fn create_panel_swap_chain(&mut self, alpha: bool) -> Option<crate::gpu::dxgi::SwapChainRef> {
        self.0.texture.as_ref()?;
        let device = angle_d3d11_device()?;
        let context = unsafe { device.GetImmediateContext() }.ok()?;
        let (width, height) = self.get_surface_dimensions();
        self.0.xaml = None;
        let swap_chain = match crate::gpu::dxgi::CompositionSwapChain::new_d3d11(&device, width as u32, height as u32, alpha) {
            Ok(swap_chain) => swap_chain,
            Err(error) => {
                log::error!("canvas: could not create a WebGL swapchain: {error}");
                return None;
            }
        };
        let unknown = swap_chain.as_unknown();
        self.0.presenter = Some(Presenter::new(swap_chain, context));
        Some(unknown)
    }

    /// Maps the swapchain into its panel: DIPs = pixels * scale + offset. The texture holds GL's
    /// bottom-up rows, so the swapchain is flipped vertically here.
    #[cfg(target_os = "windows")]
    pub fn set_swap_chain_transform(&self, scale_x: f32, scale_y: f32, offset_x: f32, offset_y: f32) -> bool {
        let Some(presenter) = self.0.presenter.as_ref() else { return false };
        let height = self.get_surface_height() as f32;
        presenter
            .swap_chain
            .set_transform(scale_x, -scale_y, offset_x, offset_y + height * scale_y)
            .is_ok()
    }

    /// The context was lost with its device (a driver reset or update, the GPU gone). It stays
    /// lost; WebGL reports it through `isContextLost()` / `webglcontextlost`.
    #[cfg(target_os = "windows")]
    pub fn is_lost(&self) -> bool {
        use std::sync::atomic::Ordering;
        if self.0.lost.load(Ordering::Relaxed) {
            return true;
        }
        type GetGraphicsResetStatus = unsafe extern "system" fn() -> u32;
        static RESET_STATUS: OnceLock<Option<GetGraphicsResetStatus>> = OnceLock::new();
        let reset_status = RESET_STATUS.get_or_init(|| {
            let proc = get_proc_address("glGetGraphicsResetStatusEXT");
            (!proc.is_null()).then(|| unsafe { std::mem::transmute::<_, GetGraphicsResetStatus>(proc) })
        });
        let lost = if self.make_current() {
            reset_status.is_some_and(|status| unsafe { status() } != gl_bindings::NO_ERROR)
        } else {
            // EGL_CONTEXT_LOST
            shared().is_some_and(|egl| egl.instance.get_error() == Some(egl::Error::ContextLost))
        };
        if lost {
            self.0.lost.store(true, Ordering::Relaxed);
        }
        lost
    }

    /// Finishes the frame and, on screen, copies it into the swapchain and presents it. A copy
    /// leaves ANGLE's cached D3D11 pipeline state untouched, unlike a draw would.
    #[cfg(target_os = "windows")]
    pub fn present(&self) -> bool {
        self.present_frame(false)
    }

    /// `present` for a context on a thread of its own, with no frame loop to try again from: a
    /// frame the display isn't ready for is copied aside and shown by `present_held` once it is,
    /// unless a newer frame comes first.
    #[cfg(target_os = "windows")]
    pub fn present_or_hold(&self) -> bool {
        self.present_frame(true)
    }

    /// Shows the frame `present_or_hold` held if the display takes it now. `true` while one is
    /// still held.
    #[cfg(target_os = "windows")]
    pub fn present_held(&self) -> bool {
        let Some(presenter) = self.0.presenter.as_ref() else { return false };
        let mut held = presenter.held.lock();
        let Some(texture) = held.as_ref() else { return false };
        if !presenter.swap_chain.acquire_frame() {
            return true;
        }
        presenter.present_from(texture);
        *held = None;
        false
    }

    #[cfg(target_os = "windows")]
    fn present_frame(&self, hold: bool) -> bool {
        if self.0.lost.load(std::sync::atomic::Ordering::Relaxed) || !self.make_current() {
            return false;
        }
        unsafe { gl_bindings::Flush() };
        if let (Some(xaml), Some(texture)) = (self.0.xaml.as_ref(), self.0.texture.as_ref()) {
            let Ok(texture) = windows::core::Interface::cast::<windows::Win32::Graphics::Direct3D11::ID3D11Resource>(texture) else {
                return false;
            };
            return match xaml.present(&texture) {
                Ok(()) => true,
                Err(error) => {
                    log::warn!("canvas: presenting WebGL into the XAML surface failed: {error}");
                    false
                }
            };
        }
        let (Some(presenter), Some(texture)) = (self.0.presenter.as_ref(), self.0.texture.as_ref()) else {
            return true;
        };
        // A held frame is superseded by this one either way.
        let superseded = if hold { presenter.held.lock().take() } else { None };
        if !presenter.swap_chain.acquire_frame() {
            if hold {
                *presenter.held.lock() = hold_frame(presenter, texture, superseded);
            }
            return true;
        }
        presenter.present_from(texture)
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
