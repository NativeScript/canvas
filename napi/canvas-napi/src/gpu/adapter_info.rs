use napi::*;
use napi_derive::napi;
use std::sync::Arc;

#[napi]
pub struct g_p_u_adapter_info {
    info: Arc<canvas_c::webgpu::gpu_adapter_info::CanvasGPUAdapterInfo>,
}

#[napi]
impl g_p_u_adapter_info {
    #[napi(getter)]
    pub fn get_architecture(&self) -> &str {
        self.info.architecture()
    }

    #[napi(getter)]
    pub fn get_description(&self) -> &str {
        self.info.description()
    }

    #[napi(getter)]
    pub fn get_device(&self) -> &str {
        self.info.device()
    }

    #[napi(getter)]
    pub fn get_vendor(&self) -> &str {
        self.info.vendor()
    }
}

pub struct AsyncAdapterInfo {
    adapter: Arc<canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter>,
}

impl AsyncAdapterInfo {
    pub fn new(adapter: Arc<canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter>) -> Self {
        Self { adapter }
    }
}

impl Task for AsyncAdapterInfo {
    type Output = g_p_u_adapter_info;
    type JsValue = g_p_u_adapter_info;

    fn compute(&mut self) -> Result<Self::Output> {
        let info = unsafe { Arc::from_raw(canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_request_adapter_info(Arc::as_ptr(&self.adapter))) };
        Ok(g_p_u_adapter_info { info })
    }

    fn resolve(&mut self, env: Env, output: g_p_u_adapter_info) -> Result<Self::JsValue> {
        Ok(output)
    }
}