use crate::gpu::adapter_info::AsyncAdapterInfo;
use crate::gpu::device::AsyncAdapterDevice;
use crate::gpu::objects::GPUSupportedLimits;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

#[napi]
pub struct g_p_u_adapter {
  pub(crate) adapter: Arc<canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter>,
}

#[napi]
impl g_p_u_adapter {
  pub(crate) fn ptr(&self) -> *const canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter {
    Arc::as_ptr(&self.adapter)
  }

  #[napi(getter)]
  pub fn get_features<'env>(&self, env: &'env Env) -> Result<Unknown<'env>> {
    let features =
      canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_get_features(self.ptr());
    let set_constructor = env
      .get_global()?
      .get_named_property::<Function<(), Unknown>>("Set")?;
    let set = set_constructor.new_instance(())?;
    if features.is_null() {
      return Ok(set);
    }
    let features: Vec<String> = unsafe { (*Box::from_raw(features)).into() };

    let set_object = Object::from_unknown(set)?;
    let add = set_object.get_named_property::<Function<String, Unknown>>("add")?;
    for feature in features.into_iter() {
      add.apply(&set_object, feature)?;
    }

    Ok(set)
  }

  #[napi(getter)]
  pub fn get_is_fallback_adapter(&self) -> bool {
    canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_is_fallback_adapter(self.ptr())
  }

  #[napi(getter)]
  pub fn get_limits(&self) -> GPUSupportedLimits {
    let limits = canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_get_limits(self.ptr());
    if limits.is_null() {
      return GPUSupportedLimits::default();
    }
    unsafe { GPUSupportedLimits::from(*Box::from_raw(limits)) }
  }

  #[napi(ts_return_type = "Promise<GPUAdapterInfo>")]
  pub fn request_adapter_info(&self) -> AsyncTask<AsyncAdapterInfo> {
    AsyncTask::new(AsyncAdapterInfo::new(Arc::clone(&self.adapter)))
  }

  #[napi(ts_return_type = "Promise<GPUDevice>")]
  pub fn request_device(
    &self,
    descriptor: Option<crate::gpu::device::GPUDeviceDescriptor>,
  ) -> AsyncTask<AsyncAdapterDevice> {
    AsyncTask::new(AsyncAdapterDevice::new(
      Arc::clone(&self.adapter),
      descriptor,
    ))
  }
}
