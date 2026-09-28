//! `CanvasModule.NSCCanvas` on Windows: the native side of a canvas view, backed by a WinUI 3
//! `SwapChainPanel` the TS view creates.
//!
//! Like the iOS view object of the same name, it owns the rendering context and hands its pointer
//! to `packages/canvas` (`create2DContext` → `create2DContextWithPointer`, `initContext` +
//! `nativeContext` → `createWebGLContext`). The drawing buffer is sized in physical pixels
//! (300x150 until set, as on the web); the panel's DIP size, its composition scale and the `fit`
//! mode place it in the panel (`canvas_core::fit`).

use std::ffi::c_void;

use canvas_c::webgpu::gpu_canvas_context::CanvasGPUCanvasContext;
use canvas_c::{CanvasRenderingContext2D as CCanvasRenderingContext2D, WebGLState};
use canvas_core::fit::{surface_transform, CanvasFit};
use napi::bindgen_prelude::ObjectFinalize;
use napi::{Env, Error, Result};
use napi_derive::napi;
use windows_core::{IUnknown, Interface};

/// `CanvasModule.__simulateD3DDeviceRemoval()`: removes the thread's Direct3D 12 device (2D
/// canvases), as a driver reset would, to exercise context loss. `false` where unsupported.
#[napi(js_name = "__simulateD3DDeviceRemoval")]
pub fn simulate_d3d_device_removal() -> bool {
  canvas_c::canvas_native_d3d_simulate_device_removal()
}

#[napi(object)]
pub struct D3DAdapterInfo {
  pub description: String,
  pub is_warp: bool,
}

/// `CanvasModule.__d3dAdapterInfo()`: the adapter of the thread's Direct3D 12 device (2D
/// canvases), `null` before the first canvas (tests: a restore stays on the GPU it was on).
#[napi(js_name = "__d3dAdapterInfo")]
pub fn d3d_adapter_info() -> Option<D3DAdapterInfo> {
  canvas_core::gpu::d3d::D3D12Context::current_shared().map(|device| D3DAdapterInfo {
    description: device.adapter_name(),
    is_warp: device.is_warp(),
  })
}

thread_local! {
  static HEADLESS_PANELS: std::cell::RefCell<Vec<IUnknown>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// `CanvasModule.__createHeadlessPanel()`: the pointer key of a stand-in SwapChainPanel (tests):
/// `new NSCCanvas(key)` then takes the on-screen paths, headless. Kept alive for the thread.
#[napi(js_name = "__createHeadlessPanel")]
pub fn create_headless_panel() -> String {
  let panel = canvas_core::gpu::dxgi::headless_panel();
  let key = format!("0x{:x}", panel.as_raw() as usize);
  HEADLESS_PANELS.with(|panels| panels.borrow_mut().push(panel));
  key
}

/// Parses `NSWinRT.interop.pointerKey(...)` output (`"0x…"`) or a decimal address.
fn parse_pointer_key(key: &str) -> Option<usize> {
  let key = key.trim();
  match key.strip_prefix("0x").or_else(|| key.strip_prefix("0X")) {
    Some(hex) => usize::from_str_radix(hex, 16).ok(),
    None => key.parse().ok(),
  }
}

/// The context the view owns; a view gets at most one, as on the web.
enum Context {
  None,
  TwoD(*mut CCanvasRenderingContext2D),
  WebGL(*mut WebGLState),
  WebGPU(*const CanvasGPUCanvasContext),
}

impl Context {
  fn pointer(&self) -> usize {
    match *self {
      Context::None => 0,
      Context::TwoD(context) => context as usize,
      Context::WebGL(state) => state as usize,
      Context::WebGPU(context) => context as usize,
    }
  }
}

#[napi(js_name = "NSCCanvas", custom_finalize)]
pub struct NSCCanvas {
  /// A reference on the panel, so detaching at teardown never touches a released object.
  panel: Option<IUnknown>,
  surface_width: u32,
  surface_height: u32,
  /// `CompositionScaleX/Y`: physical pixels per panel DIP.
  scale_x: f32,
  scale_y: f32,
  /// The panel's size in DIPs, 0 until laid out.
  view_width: f32,
  view_height: f32,
  fit: CanvasFit,
  context: Context,
  /// A XAML `SurfaceImageSource` to present into instead of the panel's swapchain (a canvas
  /// that blends with the page: a SwapChainPanel is external content in WinUI 3). Made by the
  /// view at the drawing buffer's size.
  xaml_source: Option<IUnknown>,
}

impl ObjectFinalize for NSCCanvas {
  fn finalize(self, _: Env) -> Result<()> {
    if let Some(panel) = self.panel.as_ref() {
      unsafe { canvas_core::gpu::dxgi::CompositionSwapChain::unbind_panel(panel.as_raw()) };
    }
    match self.context {
      Context::None => {}
      Context::TwoD(context) => canvas_c::canvas_native_context_release(context),
      Context::WebGL(state) => canvas_c::canvas_native_webgl_state_destroy(state),
      Context::WebGPU(context) => unsafe { canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_release(context) },
    }
    Ok(())
  }
}

impl NSCCanvas {
  fn panel_ptr(&self) -> *mut c_void {
    self.panel.as_ref().map_or(std::ptr::null_mut(), |p| p.as_raw())
  }

  fn density(&self) -> f32 {
    self.scale_x.max(1.)
  }

  fn apply_transform(&self) {
    let t = surface_transform(
      self.fit,
      (self.surface_width as f32, self.surface_height as f32),
      (self.scale_x, self.scale_y),
      (self.view_width, self.view_height),
    );
    match self.context {
      Context::None => {}
      Context::TwoD(context) => {
        canvas_c::canvas_native_context_set_swap_chain_transform(context, t.scale_x, t.scale_y, t.offset_x, t.offset_y);
      }
      Context::WebGL(state) => {
        canvas_c::canvas_native_webgl_set_swap_chain_transform(state, t.scale_x, t.scale_y, t.offset_x, t.offset_y);
      }
      Context::WebGPU(context) => unsafe {
        canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_set_swap_chain_transform(
          context, t.scale_x, t.scale_y, t.offset_x, t.offset_y,
        );
      },
    }
  }
}

#[napi]
impl NSCCanvas {
  /// `panelKey`: the `SwapChainPanel`'s pointer key. Without one the canvas is offscreen.
  #[napi(constructor)]
  pub fn new(panel_key: Option<String>) -> Result<Self> {
    let panel = match panel_key.as_deref() {
      None | Some("") => None,
      Some(key) => {
        let address = parse_pointer_key(key)
          .filter(|a| *a != 0)
          .ok_or_else(|| Error::from_reason(format!("Invalid SwapChainPanel pointer: {key}")))?;
        let raw = address as *mut c_void;
        let panel = unsafe { IUnknown::from_raw_borrowed(&raw) }.cloned();
        Some(panel.ok_or_else(|| Error::from_reason("Invalid SwapChainPanel pointer"))?)
      }
    };
    Ok(Self {
      panel,
      surface_width: 300,
      surface_height: 150,
      scale_x: 1.,
      scale_y: 1.,
      view_width: 0.,
      view_height: 0.,
      fit: CanvasFit::default(),
      context: Context::None,
      xaml_source: None,
    })
  }

  #[napi(getter)]
  pub fn surface_width(&self) -> u32 {
    self.surface_width
  }

  #[napi(setter)]
  pub fn set_surface_width(&mut self, width: f64) {
    self.set_surface_size(width, self.surface_height as f64);
  }

  #[napi(getter)]
  pub fn surface_height(&self) -> u32 {
    self.surface_height
  }

  #[napi(setter)]
  pub fn set_surface_height(&mut self, height: f64) {
    self.set_surface_size(self.surface_width as f64, height);
  }

  /// `CanvasFit` as an int (0 none, 1 fill, 2 fitX, 3 fitY, 4 scaleDown), as on iOS.
  #[napi(getter)]
  pub fn fit(&self) -> i32 {
    self.fit as i32
  }

  #[napi(setter)]
  pub fn set_fit(&mut self, fit: i32) {
    if let Some(fit) = CanvasFit::from_i32(fit) {
      self.fit = fit;
      self.apply_transform();
    }
  }

  #[napi(getter)]
  pub fn drawing_buffer_width(&self) -> u32 {
    self.surface_width
  }

  #[napi(getter)]
  pub fn drawing_buffer_height(&self) -> u32 {
    self.surface_height
  }

  /// The context's pointer as a decimal string (0 without one), as the iOS view's
  /// `nativeContext`: `packages/canvas` wraps it with `createWebGLContext(options, BigInt(...))`.
  #[napi(getter)]
  pub fn native_context(&self) -> String {
    self.context.pointer().to_string()
  }

  /// The drawing buffer size in physical pixels; resizes (and clears) an existing context.
  #[napi]
  pub fn set_surface_size(&mut self, width: f64, height: f64) {
    // `max` also maps NaN to 1.
    let (width, height) = (width.max(1.) as u32, height.max(1.) as u32);
    if (width, height) == (self.surface_width, self.surface_height) {
      return;
    }
    self.surface_width = width;
    self.surface_height = height;
    // A SurfaceImageSource has a fixed size: the view attaches one of the new size.
    self.xaml_source = None;
    match self.context {
      Context::None => return,
      Context::TwoD(context) => canvas_c::resize(unsafe { &mut *context }, width as f32, height as f32),
      Context::WebGL(state) => {
        canvas_c::canvas_native_webgl_resize_d3d(state, width as i32, height as i32);
      }
      Context::WebGPU(context) => unsafe {
        canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_resize_swap_chain_panel(context, width, height);
      },
    }
    self.apply_transform();
  }

  /// The panel's laid-out size in DIPs (`ActualWidth/Height`).
  #[napi]
  pub fn set_view_size(&mut self, width: f64, height: f64) {
    self.view_width = width as f32;
    self.view_height = height as f32;
    self.apply_transform();
  }

  /// The panel's `CompositionScaleX/Y` (DPI scale and any render transform).
  #[napi]
  pub fn set_composition_scale(&mut self, scale_x: f64, scale_y: f64) {
    self.scale_x = scale_x as f32;
    self.scale_y = scale_y as f32;
    self.apply_transform();
  }

  /// Creates (once) the 2D context and returns its pointer as a decimal string, like the iOS view.
  /// Arguments mirror `NSCCanvas.create2DContext` on iOS; only alpha, fontColor and colorSpace
  /// affect a D3D canvas.
  #[napi(js_name = "create2DContext")]
  pub fn create_2d_context(
    &mut self,
    alpha: bool,
    _antialias: bool,
    _depth: bool,
    _fail_if_major_performance_caveat: bool,
    _power_preference: i32,
    _premultiplied_alpha: bool,
    _preserve_drawing_buffer: bool,
    _stencil: bool,
    _desynchronized: bool,
    _xr_compatible: bool,
    font_color: i32,
    _will_read_frequently: bool,
    color_space: Option<i32>,
  ) -> Result<String> {
    match self.context {
      Context::TwoD(context) => return Ok((context as usize).to_string()),
      Context::WebGL(_) | Context::WebGPU(_) => {
        return Err(Error::from_reason("The canvas already has a WebGL or WebGPU context"))
      }
      Context::None => {}
    }
    let color_space = match color_space.unwrap_or(0) {
      1 => canvas_c::CanvasColorSpace::P3,
      _ => canvas_c::CanvasColorSpace::Srgb,
    };
    let (width, height) = (self.surface_width as f32, self.surface_height as f32);
    let density = self.density();
    let mut context =
      canvas_c::canvas_native_context_create_d3d(width, height, density, alpha, font_color, density * 96., 0, color_space);
    if context.is_null() {
      // No usable D3D12 device: a CPU canvas still works offscreen (readback, toDataURL).
      context = canvas_c::canvas_native_context_create(width, height, density, alpha, font_color, density * 96., 0, color_space);
    } else if let Some(source) = self.xaml_source.as_ref() {
      if !canvas_c::canvas_native_context_attach_xaml_surface(context, source.as_raw()) {
        log::error!("canvas: could not attach the canvas to its XAML surface");
      }
    } else if !self.panel_ptr().is_null()
      && !canvas_c::canvas_native_context_attach_swap_chain_panel(context, self.panel_ptr())
    {
      log::error!("canvas: could not attach the canvas to its SwapChainPanel");
    }
    self.context = Context::TwoD(context);
    self.apply_transform();
    Ok((context as usize).to_string())
  }

  /// Creates (once) the WebGL (`type` "webgl"/"experimental-webgl") or WebGL 2 context, on
  /// ANGLE; `nativeContext` then holds its pointer. Arguments mirror the iOS view's `initContext`.
  #[napi]
  pub fn init_context(
    &mut self,
    context_type: String,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: i32,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    _is_canvas: Option<bool>,
    _color_space: Option<i32>,
  ) -> Result<()> {
    match self.context {
      Context::WebGL(_) => return Ok(()),
      Context::TwoD(_) | Context::WebGPU(_) => {
        return Err(Error::from_reason("The canvas already has a 2D or WebGPU context"))
      }
      Context::None => {}
    }
    let version = if context_type.contains("webgl2") { 2 } else { 1 };
    let state = canvas_c::canvas_native_webgl_create_d3d(
      self.surface_width as i32,
      self.surface_height as i32,
      version,
      alpha,
      antialias,
      depth,
      fail_if_major_performance_caveat,
      power_preference.max(0),
      premultiplied_alpha,
      preserve_drawing_buffer,
      stencil,
      desynchronized,
      xr_compatible,
    );
    if state.is_null() {
      return Err(Error::from_reason("WebGL is unavailable: ANGLE (libEGL.dll, libGLESv2.dll) could not be initialised"));
    }
    if let Some(source) = self.xaml_source.as_ref() {
      if !canvas_c::canvas_native_webgl_attach_xaml_surface(state, source.as_raw()) {
        log::error!("canvas: could not attach the WebGL canvas to its XAML surface");
      }
    } else if !self.panel_ptr().is_null() && !canvas_c::canvas_native_webgl_attach_swap_chain_panel(state, self.panel_ptr()) {
      log::error!("canvas: could not attach the WebGL canvas to its SwapChainPanel");
    }
    self.context = Context::WebGL(state);
    self.apply_transform();
    Ok(())
  }

  /// Creates (once) the WebGPU context on the `GPU` instance `instance` (its `__getPointer()`),
  /// presenting in the panel; `nativeContext` then holds its pointer. Needs a SwapChainPanel.
  #[napi(js_name = "initWebGPUContext")]
  pub fn init_webgpu_context(&mut self, instance: napi::bindgen_prelude::BigInt) -> Result<()> {
    match self.context {
      Context::WebGPU(_) => return Ok(()),
      Context::TwoD(_) | Context::WebGL(_) => {
        return Err(Error::from_reason("The canvas already has a 2D or WebGL context"))
      }
      Context::None => {}
    }
    let (instance, _) = instance.get_i64();
    if instance == 0 {
      return Err(Error::from_reason("Invalid GPU instance"));
    }
    if self.panel_ptr().is_null() {
      return Err(Error::from_reason("WebGPU needs an on-screen canvas (a SwapChainPanel)"));
    }
    let context = unsafe {
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_create_swap_chain_panel(
        instance as *const _,
        self.panel_ptr(),
        self.surface_width,
        self.surface_height,
      )
    };
    if context.is_null() {
      return Err(Error::from_reason("Could not create a WebGPU surface for the panel"));
    }
    self.context = Context::WebGPU(context);
    self.apply_transform();
    Ok(())
  }

  /// Presents into a XAML `SurfaceImageSource` (its pointer key; made at the drawing buffer's
  /// size, not opaque) instead of the panel's swapchain, so the canvas blends with the page. Set
  /// before the context is made, or again after a resize. `false` for WebGPU (wgpu owns its
  /// swapchain) or when the source cannot be used.
  #[napi]
  pub fn attach_surface_image_source(&mut self, key: String) -> Result<bool> {
    let address = parse_pointer_key(&key)
      .filter(|a| *a != 0)
      .ok_or_else(|| Error::from_reason(format!("Invalid SurfaceImageSource pointer: {key}")))?;
    let raw = address as *mut c_void;
    let source = unsafe { IUnknown::from_raw_borrowed(&raw) }
      .cloned()
      .ok_or_else(|| Error::from_reason("Invalid SurfaceImageSource pointer"))?;
    let attached = match self.context {
      Context::None => true,
      Context::TwoD(context) => canvas_c::canvas_native_context_attach_xaml_surface(context, source.as_raw()),
      Context::WebGL(state) => canvas_c::canvas_native_webgl_attach_xaml_surface(state, source.as_raw()),
      Context::WebGPU(_) => false,
    };
    if attached {
      self.xaml_source = Some(source);
    }
    Ok(attached)
  }

  /// Where the drawing buffer sits in the view, in DIPs: `[scaleX, scaleY, offsetX, offsetY]`
  /// (DIPs = pixels * scale + offset), from the fit mode, composition scale and view size. The
  /// view places a XAML surface's image with it (a swapchain gets it natively).
  #[napi(getter)]
  pub fn surface_transform(&self) -> Vec<f64> {
    let t = surface_transform(
      self.fit,
      (self.surface_width as f32, self.surface_height as f32),
      (self.scale_x, self.scale_y),
      (self.view_width, self.view_height),
    );
    vec![t.scale_x as f64, t.scale_y as f64, t.offset_x as f64, t.offset_y as f64]
  }

  /// The context's GPU device was lost (driver reset or update, GPU removed). A 2D context can
  /// be `restoreContext()`d; a WebGL one stays lost; WebGPU reports it through `device.lost`.
  #[napi]
  pub fn is_context_lost(&self) -> bool {
    match self.context {
      Context::TwoD(context) => canvas_c::canvas_native_context_is_lost(context),
      Context::WebGL(state) => {
        canvas_webgl::webgl::canvas_native_webgl_get_is_context_lost(unsafe { &mut *state }.get_inner_mut())
      }
      Context::None | Context::WebGPU(_) => false,
    }
  }

  /// Moves a lost 2D context to a new device: cleared, in its default state, shown in the panel
  /// again (the web's `contextrestored`). `false` if it cannot be restored.
  #[napi]
  pub fn restore_context(&self) -> bool {
    let Context::TwoD(context) = self.context else {
      return false;
    };
    if !canvas_c::canvas_native_context_is_lost(context) {
      return true;
    }
    let restored = unsafe { canvas_c::canvas_native_context_restore_d3d(context, self.panel_ptr()) };
    if restored {
      self.apply_transform();
    }
    restored
  }

  /// Renders pending drawing and presents it now.
  #[napi]
  pub fn present(&self) {
    match self.context {
      Context::None => {}
      Context::TwoD(context) => canvas_c::canvas_native_context_render(context),
      Context::WebGL(state) => {
        canvas_c::canvas_native_webgl_present(state);
      }
      // WebGPU presents through its context (presentSurface / at frame end).
      Context::WebGPU(_) => {}
    }
  }
}
