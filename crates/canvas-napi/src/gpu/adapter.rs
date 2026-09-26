use std::ffi::{c_char, c_void, CString};
use std::sync::Arc;

use canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter;
use canvas_c::webgpu::gpu_device::CanvasGPUDevice;
use napi::bindgen_prelude::{FnArgs, FromNapiValue, Function, JsObjectValue, Object, Unknown};
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::{Env, Error, Result};
use napi_derive::napi;

use crate::gpu::adapter_info::g_p_u_adapter_info;
use crate::gpu::callback;
use crate::gpu::device::g_p_u_device;
use crate::gpu::limits::{self, g_p_u_supported_limits};
use crate::gpu::parse::{array_field, as_string, c_str, field, is_object, label, take_string};

#[napi(js_name = "GPUAdapter")]
pub struct g_p_u_adapter {
  pub(crate) adapter: Arc<CanvasGPUAdapter>,
}

/// A canvas-c string list as a JS `Set` (what the V8 bindings return for `features`).
pub(crate) fn string_set<'env>(env: &'env Env, values: Vec<String>) -> Result<Unknown<'env>> {
  let set_constructor = env
    .get_global()?
    .get_named_property::<Function<(), Unknown>>("Set")?;
  let set = set_constructor.new_instance(())?;
  let set_object = Object::from_unknown(set)?;
  let add = set_object.get_named_property::<Function<String, Unknown>>("add")?;
  for value in values {
    add.apply(&set_object, value)?;
  }
  Ok(set)
}

type DeviceCallback<'a> =
  Function<'a, FnArgs<(Option<Error>, Option<g_p_u_device>)>, UnknownReturnValue>;

/// What canvas-c's `requestDevice` callback reports: an error or a device pointer.
type DeviceResult = std::result::Result<usize, String>;

extern "C" fn on_device(error: *mut c_char, device: *const CanvasGPUDevice, data: *mut c_void) {
  let error = unsafe { take_string(error) };
  let result = match error {
    Some(error) => Err(error),
    None if device.is_null() => Err("requestDevice failed".to_owned()),
    None => Ok(device as usize),
  };
  unsafe { callback::deliver::<DeviceResult>(data, result) };
}

#[napi]
impl g_p_u_adapter {
  pub(crate) fn ptr(&self) -> *const CanvasGPUAdapter {
    Arc::as_ptr(&self.adapter)
  }

  #[napi(getter, ts_return_type = "Set<string>")]
  pub fn get_features<'env>(&self, env: &'env Env) -> Result<Unknown<'env>> {
    let features =
      canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_get_features(self.ptr());
    let features: Vec<String> = if features.is_null() {
      Vec::new()
    } else {
      unsafe { *Box::from_raw(features) }.into()
    };
    string_set(env, features)
  }

  #[napi(getter)]
  pub fn get_is_fallback_adapter(&self) -> bool {
    canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_is_fallback_adapter(self.ptr())
  }

  #[napi(getter)]
  pub fn get_limits(&self) -> g_p_u_supported_limits {
    let limits = canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_get_limits(self.ptr());
    if limits.is_null() {
      return g_p_u_supported_limits::new();
    }
    unsafe { *Box::from_raw(limits) }.into()
  }

  /// Synchronous, as in the V8 bindings (packages/canvas wraps it in a promise itself).
  #[napi]
  pub fn request_adapter_info(&self) -> Option<g_p_u_adapter_info> {
    let info =
      canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_request_adapter_info(self.ptr());
    (!info.is_null()).then(|| g_p_u_adapter_info {
      info: unsafe { Arc::from_raw(info) },
    })
  }

  /// `requestDevice({ label?, requiredFeatures?, requiredLimits? }, callback(error, device))`.
  /// `requiredLimits` is a `GPUSupportedLimits` (or a plain object of limits); without it the
  /// device gets the adapter's limits.
  #[napi(
    ts_args_type = "options: { label?: string, requiredFeatures?: string[], requiredLimits?: GPUSupportedLimits | Record<string, number> } | null | undefined, callback: (error: Error | null, device?: GPUDevice) => void"
  )]
  pub fn request_device(&self, options: Option<Unknown>, callback: DeviceCallback) -> Result<()> {
    let options = options.filter(is_object);
    let label = options.as_ref().and_then(label);
    let features: Vec<CString> = options
      .as_ref()
      .and_then(|options| array_field(options, c"requiredFeatures"))
      .unwrap_or_default()
      .iter()
      .filter_map(as_string)
      .filter_map(|feature| CString::new(feature).ok())
      .collect();
    let feature_ptrs: Vec<*const c_char> =
      features.iter().map(|feature| feature.as_ptr()).collect();
    let required_limits = options
      .as_ref()
      .and_then(|options| field(options, c"requiredLimits"))
      .and_then(|value| limits::required_limits(&value));

    let adapter = Arc::clone(&self.adapter);
    let tsfn = callback
      .build_threadsafe_function::<DeviceResult>()
      .build_callback(move |ctx| {
        Ok(FnArgs::from(match ctx.value {
          Ok(device) => {
            let device = unsafe { Arc::from_raw(device as *const CanvasGPUDevice) };
            (None, Some(g_p_u_device::new(device, Arc::clone(&adapter))))
          }
          Err(error) => (Some(Error::from_reason(error)), None),
        }))
      })?;
    let data = callback::into_userdata::<DeviceResult>(Box::new(move |result| {
      tsfn.call(result, ThreadsafeFunctionCallMode::NonBlocking);
    }));
    canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_request_device(
      self.ptr(),
      c_str(&label),
      if feature_ptrs.is_empty() {
        std::ptr::null()
      } else {
        feature_ptrs.as_ptr()
      },
      feature_ptrs.len(),
      required_limits
        .as_ref()
        .map_or(std::ptr::null(), |limits| limits as *const _),
      on_device,
      data,
    );
    Ok(())
  }
}
