//! `CanvasModule.OffscreenSurface`: viewless, or a transferred canvas's panel
//! (`NSCCanvas.transferToOffscreenSurface`).

use std::cell::Cell;
use std::ffi::{CStr, CString};

use canvas_c::offscreen::*;
use canvas_c::CanvasColorSpace;
use napi::bindgen_prelude::{Either, ObjectFinalize, Unknown};
use napi::{Env, Result};
use napi_derive::napi;

use crate::c2d::CanvasRenderingContext2D;
use crate::gl::web_g_l_rendering_context;
use crate::gl2::web_g_l_2_rendering_context;
use crate::gpu::context::g_p_u_canvas_context;

#[napi(js_name = "OffscreenSurface", custom_finalize)]
pub struct OffscreenSurface {
  /// Null after `dispose()`.
  surface: Cell<*const CanvasOffscreenSurface>,
}

impl ObjectFinalize for OffscreenSurface {
  fn finalize(self, _: Env) -> Result<()> {
    unsafe { canvas_native_offscreen_surface_release(self.surface.get()) };
    Ok(())
  }
}

impl OffscreenSurface {
  pub(crate) fn from_raw(surface: *const CanvasOffscreenSurface) -> Option<Self> {
    (!surface.is_null()).then(|| Self { surface: Cell::new(surface) })
  }

  fn ptr(&self) -> *const CanvasOffscreenSurface {
    self.surface.get()
  }
}

#[napi]
impl OffscreenSurface {
  #[napi]
  pub fn create(width: u32, height: u32, density: f64, ppi: f64, direction: u32, color_space: u32) -> Option<OffscreenSurface> {
    let color_space = if color_space == 1 { CanvasColorSpace::P3 } else { CanvasColorSpace::Srgb };
    Self::from_raw(canvas_native_offscreen_surface_create(width, height, density as f32, ppi as f32, direction, color_space))
  }

  #[napi]
  pub fn adopt(handle: u32) -> Option<OffscreenSurface> {
    Self::from_raw(canvas_native_offscreen_surface_adopt(handle))
  }

  #[napi(js_name = "releaseHandle")]
  pub fn release_handle(handle: u32) -> bool {
    canvas_native_offscreen_surface_release_handle(handle)
  }

  #[napi(getter)]
  pub fn width(&self) -> u32 {
    unsafe { canvas_native_offscreen_surface_get_width(self.ptr()) }
  }

  #[napi(getter)]
  pub fn height(&self) -> u32 {
    unsafe { canvas_native_offscreen_surface_get_height(self.ptr()) }
  }

  #[napi(getter)]
  pub fn density(&self) -> f64 {
    unsafe { canvas_native_offscreen_surface_get_density(self.ptr()) as f64 }
  }

  #[napi(getter)]
  pub fn ppi(&self) -> f64 {
    unsafe { canvas_native_offscreen_surface_get_ppi(self.ptr()) as f64 }
  }

  #[napi(getter)]
  pub fn direction(&self) -> u32 {
    unsafe { canvas_native_offscreen_surface_get_direction(self.ptr()) }
  }

  #[napi(getter, js_name = "hasView")]
  pub fn has_view(&self) -> bool {
    unsafe { canvas_native_offscreen_surface_has_view(self.ptr()) }
  }

  #[napi]
  pub fn resize(&self, width: u32, height: u32) {
    unsafe { canvas_native_offscreen_surface_resize(self.ptr(), width, height) };
  }

  #[napi(js_name = "toHandle")]
  pub fn to_handle(&self) -> u32 {
    unsafe { canvas_native_offscreen_surface_to_handle(self.ptr()) }
  }

  #[napi]
  pub fn dispose(&self) {
    let surface = self.surface.replace(std::ptr::null());
    unsafe { canvas_native_offscreen_surface_release(surface) };
  }

  #[napi(js_name = "create2D")]
  pub fn create_2d(&self, alpha: bool, font_color: i32, threaded: bool) -> Option<CanvasRenderingContext2D> {
    let context = unsafe { canvas_native_offscreen_surface_create_2d(self.ptr(), alpha, font_color, threaded) };
    (!context.is_null()).then(|| CanvasRenderingContext2D::from_raw(context))
  }

  #[allow(clippy::too_many_arguments)]
  #[napi(js_name = "createWebGL")]
  pub fn create_webgl(
    &self,
    version: i32,
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
    threaded: bool,
  ) -> Option<Either<web_g_l_rendering_context, web_g_l_2_rendering_context>> {
    let attributes = WebGLAttributes {
      version,
      alpha,
      antialias,
      depth,
      fail_if_major_performance_caveat,
      power_preference,
      premultiplied_alpha,
      preserve_drawing_buffer,
      stencil,
      desynchronized,
      xr_compatible,
    };
    let state = unsafe { canvas_native_offscreen_surface_create_webgl(self.ptr(), &attributes, threaded) };
    if state.is_null() {
      return None;
    }
    Some(if version == 2 {
      Either::B(web_g_l_2_rendering_context::from_raw(state))
    } else {
      Either::A(web_g_l_rendering_context::from_raw(state))
    })
  }

  /// One wgpu instance per process here, so `gpu` is unused.
  #[napi(js_name = "createWebGPU")]
  pub fn create_webgpu(&self, _gpu: Unknown) -> Option<g_p_u_canvas_context> {
    let instance = crate::gpu::gpu_instance();
    let context = unsafe { canvas_native_offscreen_surface_create_webgpu(self.ptr(), std::sync::Arc::as_ptr(&instance)) };
    (!context.is_null()).then(|| g_p_u_canvas_context::from_raw(context))
  }

  /// `quality` is 0..1.
  #[napi(js_name = "toDataURL")]
  pub fn to_data_url(&self, format: Option<String>, quality: Option<f64>) -> Option<String> {
    let format = CString::new(format.unwrap_or_else(|| "image/png".into())).ok()?;
    let quality = quality.map_or(92, |q| (q * 100.) as u32);
    let url = unsafe { canvas_native_offscreen_surface_to_data_url(self.ptr(), format.as_ptr(), quality) };
    if url.is_null() {
      return None;
    }
    let string = unsafe { CStr::from_ptr(url) }.to_string_lossy().into_owned();
    canvas_c::canvas_native_string_destroy(url);
    Some(string)
  }
}
