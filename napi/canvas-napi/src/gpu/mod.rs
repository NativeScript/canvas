use crate::gpu::adapter::g_p_u_adapter;
use canvas_c::webgpu::gpu::CanvasGPURequestAdapterOptions;
use napi::bindgen_prelude::{AsyncTask, ClassInstance, ObjectFinalize};
use napi::*;
use napi_derive::napi;
use std::error::Error;
use std::ops::Deref;
use std::os::raw::c_void;
use std::sync::mpsc::channel;
use std::sync::Arc;

pub mod context;
pub mod adapter;
pub mod adapter_info;
mod bind_group;
mod device;
mod texture;
mod enums;
mod objects;
mod shader_module;
mod pipeline_layout;
mod render_pipeline;
mod bind_group_layout;
mod command_encoder;
mod queue;
mod command_buffer;
mod texture_view;
mod query_set;
mod render_pass_encoder;

mod buffer;
mod compute_pass_encoder;
mod compute_pipeline;
mod render_bundle;
mod render_bundle_encoder;
mod sampler;
mod external_texture;
// const GPU: std::sync::LazyLock<Arc<canvas_c::webgpu::gpu::CanvasWebGPUInstance>> = std::sync::LazyLock::new(|| {
//     unsafe {
//         let instance = canvas_c::webgpu::gpu::canvas_native_webgpu_instance_create();
//         Arc::increment_strong_count(instance);
//         Arc::from_raw(instance)
//     }
// });

const GPU_INSTANCE: std::sync::OnceLock<Arc<canvas_c::webgpu::gpu::CanvasWebGPUInstance>> = std::sync::OnceLock::new();

#[napi(object)]
pub struct GPURequestAdapterOptions {
    pub power_preference: Option<String>,
    pub is_fallback_adapter: Option<bool>,
}
#[napi(js_name = "GPU")]
pub struct g_p_u {
    instance: Arc<canvas_c::webgpu::gpu::CanvasWebGPUInstance>,
}

extern "C" fn request_adapter(adapter: *const canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter, data: *mut c_void) {
    if adapter.is_null() || data.is_null() { return; }
    let data = unsafe { data as *mut Sender };
    let data = unsafe { *Box::from_raw(data) };

    data.tx.send(g_p_u_adapter { adapter: unsafe { Arc::from_raw(adapter) } }).unwrap()
}

pub struct AsyncRequestAdapterTask {
    instance: Arc<canvas_c::webgpu::gpu::CanvasWebGPUInstance>,
    options: Option<GPURequestAdapterOptions>,
}

impl AsyncRequestAdapterTask {
    pub fn new(instance: &Arc<canvas_c::webgpu::gpu::CanvasWebGPUInstance>, options: Option<GPURequestAdapterOptions>) -> Self {
        Self { instance: Arc::clone(instance), options }
    }
}

struct Sender {
    tx: std::sync::mpsc::Sender<g_p_u_adapter>,
}

impl Task for AsyncRequestAdapterTask {
    type Output = g_p_u_adapter;
    type JsValue = ClassInstance<g_p_u_adapter>;

    fn compute(&mut self) -> Result<Self::Output> {
        let (tx, rx) = channel();
        let mut options = CanvasGPURequestAdapterOptions::default();
        if let Some(opts) = self.options.as_ref() {
            if let Some(power_preference) = opts.power_preference.as_deref() {
                options.power_preference = match power_preference {
                    "low-power" => {
                        canvas_c::webgpu::gpu::CanvasGPUPowerPreference::LowPower
                    }
                    "high-performance" => {
                        canvas_c::webgpu::gpu::CanvasGPUPowerPreference::HighPerformance
                    }
                    _ => {
                        canvas_c::webgpu::gpu::CanvasGPUPowerPreference::None
                    }
                };

                options.force_fallback_adapter = opts.is_fallback_adapter.unwrap_or_default();
            }
        }
        let data = Box::into_raw(Box::new(Sender { tx }));
        unsafe {
            canvas_c::webgpu::gpu::canvas_native_webgpu_request_adapter(Arc::as_ptr(&self.instance), &options, request_adapter, data as *mut c_void);
        }

        match rx.recv() {
            Ok(ret) => {
                Ok(ret)
            }
            Err(error) => { Err(napi::Error::new(Status::Unknown, error.to_string())) }
        }
    }

    fn resolve(&mut self, env: Env, output: g_p_u_adapter) -> Result<Self::JsValue> {
        output.into_instance(env)
    }
}


#[napi]
impl g_p_u {
    #[napi(factory)]
    pub fn get_instance() -> Self {
        let binding = GPU_INSTANCE;
        let instance = binding.get_or_init(|| {
            unsafe {
                let instance = canvas_c::webgpu::gpu::canvas_native_webgpu_instance_create();
                Arc::from_raw(instance)
            }
        });
        Self {
            instance: Arc::clone(instance),
        }
    }
    #[napi(js_name = "wgslLanguageFeatures")]
    pub fn get_wgsl_language_features(&self) -> Vec<String> {
        vec![]
    }

    #[napi]
    pub fn get_preferred_canvas_format(&self) -> &str {
        #[cfg(any(target_os = "ios", target_os = "macos"))]{
            return "bgra8unorm";
        }
        "rgba8unorm"
    }

    #[napi(ts_return_type = "Promise<GPUAdapter>")]
    pub fn request_adapter(&self, options: Option<GPURequestAdapterOptions>) -> AsyncTask<AsyncRequestAdapterTask> {
        AsyncTask::new(AsyncRequestAdapterTask::new(&self.instance, options))
    }
}