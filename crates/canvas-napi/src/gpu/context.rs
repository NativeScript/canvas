use std::cell::Cell;
use std::ffi::{c_void, CString};
use std::rc::Rc;
use std::sync::Arc;

use canvas_c::webgpu::enums::{
  CanvasGPUTextureUsageCopyDst, CanvasGPUTextureUsageCopySrc,
  CanvasGPUTextureUsageRenderAttachment, CanvasOptionalGPUTextureFormat,
  SurfaceGetCurrentTextureStatus,
};
use canvas_c::webgpu::gpu_canvas_context::{
  CanvasGPUCanvasContext, CanvasGPUPresentMode, CanvasGPUSurfaceAlphaMode,
  CanvasGPUSurfaceConfiguration,
};
use canvas_c::webgpu::wgt::{CompositeAlphaMode, PresentMode};
use canvas_c::StringBuffer;
use napi::bindgen_prelude::{BigInt, ObjectFinalize, ToNapiValue, Unknown};
use napi::{Env, Result};
use napi_derive::napi;

use crate::frame::FrameSlot;
use crate::gpu::adapter::g_p_u_adapter;
use crate::gpu::callback::{null, undefined};
use crate::gpu::device::g_p_u_device;
use crate::gpu::parse::{
  as_string, class, extent3d, field, int32_value, is_object, string, take_string, texture_format,
  texture_formats, uint32,
};
use crate::gpu::texture::g_p_u_texture;
use crate::module::JsRaw;

/// `GPUCanvasContext` over a canvas-c context. `createWebGPUContextWithPointer` wraps one a host
/// view owns (and releases); the wrapper then must not release it.
#[napi(js_name = "GPUCanvasContext", custom_finalize)]
pub struct g_p_u_canvas_context {
  pub(crate) context: *const CanvasGPUCanvasContext,
  owns_context: bool,
  /// A frame whose texture `getCurrentTexture()` handed out is presented at frame end, as on
  /// the web, unless `presentSurface()` already did.
  pub(crate) frame: Rc<FrameSlot>,
  continuous_render: Cell<bool>,
}

impl ObjectFinalize for g_p_u_canvas_context {
  fn finalize(self, _: Env) -> Result<()> {
    if self.owns_context {
      unsafe {
        canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_release(self.context)
      };
    }
    Ok(())
  }
}

/// Presents the current texture if nothing has presented it yet (the V8 bindings'
/// `GPUCanvasContextImpl::Flush`).
fn present_pending(context: *const CanvasGPUCanvasContext) {
  let texture =
    canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_has_current_texture(context);
  if texture.is_null() {
    return;
  }
  unsafe {
    if !canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_has_surface_presented(
      context,
    ) {
      // Takes over the reference `has_current_texture` returned.
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_present_surface(
        context, texture,
      );
    } else {
      canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_release(texture);
    }
  }
}

/// Frame-end flush for WebGPU contexts.
unsafe fn present_webgpu(context: *mut c_void) {
  present_pending(context as *const CanvasGPUCanvasContext);
}

/// `getCapabilities` result, with the member names packages/canvas reads (`format`, not
/// `formats`).
#[napi(object, object_to_js = true, object_from_js = false)]
pub struct GPUCanvasCapabilities {
  pub format: Vec<String>,
  pub present_modes: Vec<String>,
  pub alpha_modes: Vec<String>,
  pub usages: u32,
}

fn string_buffer(buffer: *const StringBuffer) -> Vec<String> {
  if buffer.is_null() {
    return Vec::new();
  }
  unsafe { *Box::from_raw(buffer as *mut StringBuffer) }.into()
}

fn alpha_mode(value: Option<Unknown>) -> CanvasGPUSurfaceAlphaMode {
  let Some(value) = value else {
    return CanvasGPUSurfaceAlphaMode::Opaque;
  };
  if let Some(mode) = int32_value(&value) {
    return match mode {
      0 => CanvasGPUSurfaceAlphaMode::Auto,
      2 => CanvasGPUSurfaceAlphaMode::PreMultiplied,
      3 => CanvasGPUSurfaceAlphaMode::PostMultiplied,
      4 => CanvasGPUSurfaceAlphaMode::Inherit,
      _ => CanvasGPUSurfaceAlphaMode::Opaque,
    };
  }
  match as_string(&value).as_deref() {
    Some("premultiplied") => CanvasGPUSurfaceAlphaMode::PreMultiplied,
    Some("postmultiplied") => CanvasGPUSurfaceAlphaMode::PostMultiplied,
    Some("inherit") => CanvasGPUSurfaceAlphaMode::Inherit,
    Some("auto") => CanvasGPUSurfaceAlphaMode::Auto,
    _ => CanvasGPUSurfaceAlphaMode::Opaque,
  }
}

fn present_mode(value: Option<Unknown>) -> Option<CanvasGPUPresentMode> {
  let value = value?;
  if let Some(mode) = int32_value(&value) {
    return Some(match mode {
      0 => CanvasGPUPresentMode::AutoVsync,
      1 => CanvasGPUPresentMode::AutoNoVsync,
      3 => CanvasGPUPresentMode::FifoRelaxed,
      4 => CanvasGPUPresentMode::Immediate,
      5 => CanvasGPUPresentMode::Mailbox,
      _ => CanvasGPUPresentMode::Fifo,
    });
  }
  Some(match as_string(&value)?.as_str() {
    "autoVsync" => CanvasGPUPresentMode::AutoVsync,
    "autoNoVsync" => CanvasGPUPresentMode::AutoNoVsync,
    "fifoRelaxed" => CanvasGPUPresentMode::FifoRelaxed,
    "immediate" => CanvasGPUPresentMode::Immediate,
    "mailbox" => CanvasGPUPresentMode::Mailbox,
    _ => CanvasGPUPresentMode::Fifo,
  })
}

fn composite_to_alpha_mode(mode: &CompositeAlphaMode) -> CanvasGPUSurfaceAlphaMode {
  match mode {
    CompositeAlphaMode::Auto => CanvasGPUSurfaceAlphaMode::Auto,
    CompositeAlphaMode::Opaque => CanvasGPUSurfaceAlphaMode::Opaque,
    CompositeAlphaMode::PreMultiplied => CanvasGPUSurfaceAlphaMode::PreMultiplied,
    CompositeAlphaMode::PostMultiplied => CanvasGPUSurfaceAlphaMode::PostMultiplied,
    CompositeAlphaMode::Inherit => CanvasGPUSurfaceAlphaMode::Inherit,
  }
}

fn present_to_present_mode(mode: &PresentMode) -> CanvasGPUPresentMode {
  match mode {
    PresentMode::AutoVsync => CanvasGPUPresentMode::AutoVsync,
    PresentMode::AutoNoVsync => CanvasGPUPresentMode::AutoNoVsync,
    PresentMode::Fifo => CanvasGPUPresentMode::Fifo,
    PresentMode::FifoRelaxed => CanvasGPUPresentMode::FifoRelaxed,
    PresentMode::Immediate => CanvasGPUPresentMode::Immediate,
    PresentMode::Mailbox => CanvasGPUPresentMode::Mailbox,
  }
}

impl g_p_u_canvas_context {
  pub(crate) fn from_raw(context: *const CanvasGPUCanvasContext, owns_context: bool) -> Self {
    Self {
      context,
      owns_context,
      frame: FrameSlot::new(context as *mut c_void, present_webgpu),
      continuous_render: Cell::new(false),
    }
  }
}

#[napi]
impl g_p_u_canvas_context {
  /// `configure({ device, format, usage?, viewFormats?, colorSpace?, alphaMode?, presentMode?,
  /// size? })`. An alpha mode the surface cannot do falls back to its first one.
  #[napi(ts_args_type = "options: object")]
  pub fn configure(&self, options: Unknown) {
    if !is_object(&options) {
      return;
    }
    let Some(device) = class::<g_p_u_device>(&options, c"device") else {
      return;
    };
    let mut alpha_mode = alpha_mode(field(&options, c"alphaMode"));
    let mut present_mode = present_mode(field(&options, c"presentMode"));

    let capabilities =
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_get_capabilities_rust(
        // A borrowed Arc: the context is kept alive by its owner, not released here.
        &std::mem::ManuallyDrop::new(unsafe { Arc::from_raw(self.context) }),
        &device.adapter,
      );
    if let Some(capabilities) = capabilities.as_ref() {
      let mode: CompositeAlphaMode = alpha_mode.into();
      if !capabilities.alpha_modes.contains(&mode) {
        if let Some(first) = capabilities.alpha_modes.first() {
          alpha_mode = composite_to_alpha_mode(first);
        }
      }
      if present_mode.is_none() {
        present_mode = capabilities
          .present_modes
          .first()
          .map(present_to_present_mode);
      }
    }

    let view_formats = texture_formats(field(&options, c"viewFormats").as_ref());
    let size = field(&options, c"size").map(|size| extent3d(Some(&size)));
    let size = size.filter(|size| size.width > 0);
    let format = match string(&options, c"format").and_then(|f| texture_format(&f)) {
      Some(format) => CanvasOptionalGPUTextureFormat::Some(format),
      None => CanvasOptionalGPUTextureFormat::None,
    };
    let config = CanvasGPUSurfaceConfiguration {
      alphaMode: alpha_mode,
      usage: uint32(&options, c"usage").unwrap_or(
        CanvasGPUTextureUsageRenderAttachment
          | CanvasGPUTextureUsageCopySrc
          | CanvasGPUTextureUsageCopyDst,
      ),
      presentMode: present_mode.unwrap_or(CanvasGPUPresentMode::Fifo),
      view_formats: if view_formats.is_empty() {
        std::ptr::null()
      } else {
        view_formats.as_ptr()
      },
      view_formats_size: view_formats.len(),
      size: size
        .as_ref()
        .map_or(std::ptr::null(), |size| size as *const _),
      format,
    };
    unsafe {
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_configure(
        self.context,
        Arc::as_ptr(&device.device),
        &config,
      );
    }
  }

  #[napi]
  pub fn unconfigure(&self) {
    unsafe {
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_unconfigure(self.context)
    }
  }

  /// The frame's texture; null when the surface has none to give (lost, outdated, ...),
  /// undefined when the context is not configured. Handing one out schedules the frame's
  /// present at frame end.
  #[napi(ts_return_type = "GPUTexture | null | undefined")]
  pub fn get_current_texture(&self, env: Env) -> Result<JsRaw> {
    let texture =
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_get_current_texture(
        self.context,
      );
    if texture.is_null() {
      return Ok(JsRaw(undefined(env.raw())));
    }
    let status =
      unsafe { canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_get_status(texture) };
    if status != SurfaceGetCurrentTextureStatus::Success {
      unsafe { canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_release(texture) };
      return Ok(JsRaw(null(env.raw())));
    }
    crate::frame::mark_dirty(&self.frame);
    match unsafe { g_p_u_texture::from_raw(texture) } {
      Some(texture) => Ok(JsRaw(unsafe {
        g_p_u_texture::to_napi_value(env.raw(), texture)
      }?)),
      None => Ok(JsRaw(undefined(env.raw()))),
    }
  }

  /// Presents the current texture now, if nothing presented it yet.
  #[napi]
  pub fn present_surface(&self) {
    present_pending(self.context);
  }

  #[napi(getter)]
  pub fn get_has_current_texture(&self) -> bool {
    let texture =
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_has_current_texture(
        self.context,
      );
    if texture.is_null() {
      return false;
    }
    unsafe { canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_release(texture) };
    true
  }

  #[napi(getter)]
  pub fn get_has_surface_presented(&self) -> bool {
    canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_has_surface_presented(
      self.context,
    )
  }

  /// `{ format, presentModes, alphaModes, usages }` of the surface on `adapter`; empty lists
  /// (and usages 0) when it cannot say.
  #[napi]
  pub fn get_capabilities(&self, adapter: Option<&g_p_u_adapter>) -> GPUCanvasCapabilities {
    let capabilities = adapter.map_or(std::ptr::null_mut(), |adapter| {
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_get_capabilities(
        self.context,
        adapter.ptr(),
      )
    });
    if capabilities.is_null() {
      return GPUCanvasCapabilities {
        format: Vec::new(),
        present_modes: Vec::new(),
        alpha_modes: Vec::new(),
        usages: 0,
      };
    }
    let capabilities = unsafe { Box::from_raw(capabilities) };
    GPUCanvasCapabilities {
      format: string_buffer(capabilities.formats),
      present_modes: string_buffer(capabilities.present_modes),
      alpha_modes: string_buffer(capabilities.alpha_modes),
      usages: capabilities.usages,
    }
  }

  /// `quality` 0..1 (default 0.92): the current frame, or the last presented one.
  #[napi(js_name = "__toDataURL")]
  pub fn __to_data_url(&self, format: Option<String>, quality: Option<f64>) -> String {
    let format = CString::new(format.unwrap_or_else(|| "image/png".to_owned())).unwrap_or_default();
    let data = unsafe {
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_to_data_url_with_fallback(
        self.context,
        format.as_ptr(),
        quality.unwrap_or(0.92) as f32,
      )
    };
    unsafe { take_string(data) }.unwrap_or_else(|| "data:,".to_owned())
  }

  #[napi(js_name = "toDataURL")]
  pub fn to_data_url(&self, format: Option<String>, quality: Option<f64>) -> String {
    self.__to_data_url(format, quality)
  }

  /// The canvas-c context pointer, as a decimal string.
  #[napi(js_name = "__getPointer")]
  pub fn get_pointer(&self) -> String {
    (self.context as usize).to_string()
  }

  /// `__startRaf` / `__stopRaf`: a paused context keeps its pending frame but is not presented.
  #[napi(js_name = "__startRaf")]
  pub fn start_raf(&self) {
    self.frame.set_paused(false);
  }

  #[napi(js_name = "__stopRaf")]
  pub fn stop_raf(&self) {
    self.frame.set_paused(true);
  }

  #[napi(getter)]
  pub fn continuous_render_mode(&self) -> bool {
    self.continuous_render.get()
  }

  #[napi(setter)]
  pub fn set_continuous_render_mode(&self, value: bool) {
    self.continuous_render.set(value);
  }
}

/// `CanvasModule.createWebGPUContextWithPointer(pointer)`: wraps a `CanvasGPUCanvasContext` the
/// host view owns (it stays the host's to release).
#[napi(js_name = "createWebGPUContextWithPointer")]
pub fn create_webgpu_context_with_pointer(pointer: BigInt) -> Option<g_p_u_canvas_context> {
  let (pointer, _) = pointer.get_i64();
  (pointer != 0)
    .then(|| g_p_u_canvas_context::from_raw(pointer as *const CanvasGPUCanvasContext, false))
}
