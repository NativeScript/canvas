use napi::bindgen_prelude::ObjectFinalize;
use napi::*;
use napi_derive::napi;
use std::os::raw::c_void;

mod context;

pub struct GPURequestAdapterOptions {
    power_preference: String,
    is_fallback_adapter: bool,
}

#[napi(custom_finalize)]
pub struct g_p_u {
    instance: *const canvas_c::webgpu::gpu::CanvasWebGPUInstance,
}

impl ObjectFinalize for g_p_u {
    fn finalize(self, _: Env) -> Result<()> {
        unsafe { canvas_c::webgpu::gpu::canvas_native_webgpu_instance_release(self.instance); }
        Ok(())
    }
}

pub struct AsyncRequestAdapterTask {}

extern "C" fn request_adapter(adapter: *const canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter, data: *mut c_void) {}

#[napi]
impl g_p_u {

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

    #[napi]
    pub fn request_adapter(&self) {
        unsafe {
            let data = std::ptr::null_mut();
            canvas_c::webgpu::gpu::canvas_native_webgpu_request_adapter(self.instance, std::ptr::null(), request_adapter, data);
        }
    }
}