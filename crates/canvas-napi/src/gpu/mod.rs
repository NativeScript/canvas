//! WebGPU: `#[napi]` classes over canvas-c's `webgpu` module, with the surface the V8 bindings
//! (`packages/canvas/platforms/ios/src/cpp/webgpu`) register, since that is what
//! `packages/canvas/WebGPU` is written against: member names, accessors vs methods, option bags
//! read leniently (`parse`), int or string enums, callbacks where the TS passes callbacks
//! (`requestAdapter`, `requestDevice`, `popErrorScope`, `onSubmittedWorkDone`,
//! `create*PipelineAsync`) and promises where it expects them (`mapAsync`, `lost`,
//! `getCompilationInfo`).

// Class and enum names mirror the WebGPU JS names (`g_p_u_buffer` -> `GPUBuffer`,
// `GPUTextureFormat::rgba8unorm` -> "rgba8unorm"), so Rust naming lints do not apply.
#![allow(non_camel_case_types, non_snake_case)]

use std::ffi::c_void;
use std::sync::Arc;

use canvas_c::webgpu::gpu::{
  CanvasGPUFeatureLevel, CanvasGPUPowerPreference, CanvasGPURequestAdapterOptions,
  CanvasWebGPUInstance,
};
use napi::bindgen_prelude::{FnArgs, Function, Null, Unknown};
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::Result;
use napi_derive::napi;

use crate::gpu::adapter::g_p_u_adapter;
use crate::gpu::parse::{as_bool, as_number, as_string, field, is_object, uint32_value};

pub mod adapter;
pub mod adapter_info;
mod bind_group;
mod bind_group_layout;
mod buffer;
mod callback;
mod command_buffer;
mod command_encoder;
mod compute_pass_encoder;
mod compute_pipeline;
pub mod context;
mod device;
mod enums;
mod external_texture;
mod handle;
mod limits;
mod parse;
mod pipeline;
mod pipeline_layout;
mod query_set;
mod queue;
mod render_bundle;
mod render_bundle_encoder;
mod render_pass_encoder;
mod render_pipeline;
mod sampler;
mod shader_module;
mod texture;
mod texture_view;

// One wgpu instance per process: adapters, devices and the surfaces host views create
// (`NSCCanvas.initWebGPUContext(BigInt(gpu.__getPointer()))`) must all come from the same one.
// A `static`, not a `const`: a `const` is inlined at every use, so each `GPU` would get a fresh
// (empty) OnceLock and create a new wgpu Instance.
static GPU_INSTANCE: std::sync::OnceLock<Arc<CanvasWebGPUInstance>> = std::sync::OnceLock::new();

#[napi(js_name = "GPU")]
pub struct g_p_u {
  instance: Arc<CanvasWebGPUInstance>,
}

type AdapterCallback<'a> = Function<'a, FnArgs<(Null, Option<g_p_u_adapter>)>, UnknownReturnValue>;

extern "C" fn on_adapter(
  adapter: *const canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter,
  data: *mut c_void,
) {
  unsafe { callback::deliver::<usize>(data, adapter as usize) };
}

/// `requestAdapter` options: `powerPreference` as the WebGPU string (what packages/canvas
/// passes) or the V8 bindings' int (1 low-power, 2 high-performance); `forceFallbackAdapter`
/// (or packages/canvas's `isFallbackAdapter`); `featureLevel`.
fn adapter_options(options: Option<&Unknown>) -> CanvasGPURequestAdapterOptions {
  let mut ret = CanvasGPURequestAdapterOptions::default();
  let Some(options) = options.filter(|options| is_object(options)) else {
    return ret;
  };
  if let Some(power) = field(options, c"powerPreference") {
    ret.power_preference = match (uint32_value(&power), as_string(&power).as_deref()) {
      (Some(1), _) | (_, Some("low-power")) => CanvasGPUPowerPreference::LowPower,
      (Some(2), _) | (_, Some("high-performance")) => CanvasGPUPowerPreference::HighPerformance,
      _ => CanvasGPUPowerPreference::None,
    };
  }
  let fallback = field(options, c"forceFallbackAdapter").or_else(|| field(options, c"isFallbackAdapter"));
  if let Some(fallback) = fallback {
    ret.force_fallback_adapter =
      as_bool(&fallback).unwrap_or_else(|| as_number(&fallback).is_some_and(|n| n != 0.));
  }
  if field(options, c"featureLevel").and_then(|v| as_string(&v)).as_deref() == Some("compatibility") {
    ret.feature_level = CanvasGPUFeatureLevel::Compatibility;
  }
  ret
}

#[napi]
impl g_p_u {
  /// `new CanvasModule.GPU()` (what packages/canvas does): the process's wgpu instance.
  #[napi(constructor)]
  pub fn new() -> Self {
    Self::get_instance()
  }

  #[napi(factory)]
  pub fn get_instance() -> Self {
    let instance = GPU_INSTANCE.get_or_init(|| unsafe {
      Arc::from_raw(canvas_c::webgpu::gpu::canvas_native_webgpu_instance_create())
    });
    Self {
      instance: Arc::clone(instance),
    }
  }

  /// The canvas-c instance pointer as a decimal string, for host views creating surfaces on it.
  #[napi(js_name = "__getPointer")]
  pub fn get_pointer(&self) -> String {
    unsafe { canvas_c::webgpu::gpu::canvas_native_webgpu_get_pointer_addr(Arc::as_ptr(&self.instance)) }
      .to_string()
  }

  #[napi(getter, js_name = "wgslLanguageFeatures")]
  pub fn get_wgsl_language_features(&self) -> Vec<String> {
    vec![]
  }

  /// BGRA is the native swapchain format on Apple and on Windows (DXGI), as packages/canvas's
  /// `GPU.getPreferredCanvasFormat` says.
  #[napi]
  pub fn get_preferred_canvas_format(&self) -> &str {
    if cfg!(any(target_os = "ios", target_os = "macos", target_os = "windows")) {
      "bgra8unorm"
    } else {
      "rgba8unorm"
    }
  }

  /// `requestAdapter(options, callback(error, adapter))`: `adapter` is null when there is none.
  #[napi(
    ts_args_type = "options: { powerPreference?: string | number, forceFallbackAdapter?: boolean, isFallbackAdapter?: boolean, featureLevel?: string } | null | undefined, callback: (error: Error | null, adapter: GPUAdapter | null) => void"
  )]
  pub fn request_adapter(&self, options: Option<Unknown>, callback: AdapterCallback) -> Result<()> {
    let options = adapter_options(options.as_ref());
    let tsfn = callback
      .build_threadsafe_function::<usize>()
      .build_callback(|ctx| {
        let adapter = ctx.value as *const canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter;
        let adapter = (!adapter.is_null()).then(|| g_p_u_adapter {
          adapter: unsafe { Arc::from_raw(adapter) },
        });
        Ok(FnArgs::from((Null, adapter)))
      })?;
    let data = callback::into_userdata::<usize>(Box::new(move |adapter| {
      tsfn.call(adapter, ThreadsafeFunctionCallMode::NonBlocking);
    }));
    unsafe {
      canvas_c::webgpu::gpu::canvas_native_webgpu_request_adapter(
        Arc::as_ptr(&self.instance),
        &options,
        on_adapter,
        data,
      );
    }
    Ok(())
  }
}
