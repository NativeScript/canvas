use crate::webgpu::gpu_supported_limits::CanvasGPUSupportedLimits;

use super::gpu_adapter::CanvasGPUAdapter;
use std::{os::raw::c_void, sync::Arc};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasGPUPowerPreference {
    None,
    LowPower,
    HighPerformance,
}

impl Into<wgpu_types::PowerPreference> for CanvasGPUPowerPreference {
    fn into(self) -> wgpu_types::PowerPreference {
        match self {
            CanvasGPUPowerPreference::LowPower => wgpu_types::PowerPreference::LowPower,
            CanvasGPUPowerPreference::HighPerformance => {
                wgpu_types::PowerPreference::HighPerformance
            }
            CanvasGPUPowerPreference::None => wgpu_types::PowerPreference::None,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CanvasGPURequestAdapterOptions {
    power_preference: CanvasGPUPowerPreference,
    force_fallback_adapter: bool,
}

impl Default for CanvasGPURequestAdapterOptions {
    fn default() -> Self {
        Self {
            power_preference: CanvasGPUPowerPreference::None,
            force_fallback_adapter: false,
        }
    }
}

#[derive(Clone)]
pub struct CanvasWebGPUInstance(pub(crate) Arc<wgpu_core::global::Global>);

unsafe impl Send for CanvasWebGPUInstance {}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_instance_create() -> *mut CanvasWebGPUInstance {
    #[cfg(not(target_os = "android"))]
    let backends = wgpu_types::Backends::METAL;

    #[cfg(target_os = "android")]
    let backends = wgpu_types::Backends::VULKAN | wgpu_types::Backends::GL;

    #[cfg(target_os = "windows")]
    let backends = wgpu_types::Backends::DX12;

    let instance = wgpu_core::global::Global::new(
        "webgpu",
        wgpu_types::InstanceDescriptor {
            backends,
            ..Default::default()
        },
    );

    Box::into_raw(Box::new(CanvasWebGPUInstance(Arc::new(instance))))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_instance_destroy(instance: *mut CanvasWebGPUInstance) {
    if instance.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(instance) };
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_request_adapter(
    instance: *const CanvasWebGPUInstance,
    options: *const CanvasGPURequestAdapterOptions,
    callback: extern "C" fn(*mut CanvasGPUAdapter, *mut c_void),
    callback_data: *mut c_void,
) {
    use super::prelude::build_features;

    let instance = (&*instance).clone();
    let options = if options.is_null() {
        CanvasGPURequestAdapterOptions::default()
    } else {
        *options
    };

    let opts = wgpu_types::RequestAdapterOptions {
        power_preference: options.power_preference.into(),
        force_fallback_adapter: options.force_fallback_adapter,
        compatible_surface: None,
    };

    let callback = callback as i64;
    let callback_data = callback_data as i64;
    std::thread::spawn(move || {
        #[cfg(not(target_os = "android"))]
        let backends = wgpu_types::Backends::METAL;

        #[cfg(target_os = "android")]
        let backends = wgpu_types::Backends::VULKAN | wgpu_types::Backends::GL;

        #[cfg(target_os = "windows")]
        let backends = wgpu_types::Backends::DX12;

        println!("spawn");

        let instance = instance;

        let global = &instance.0;

        let adapter = global.request_adapter(
            &opts,
            wgpu_core::instance::AdapterInputs::Mask(backends, |_| None),
        );

        println!("adapter {:?}", &adapter);

        let adapter = adapter.map(|adapter_id| {
            let features = gfx_select!(adapter_id => global.adapter_features(adapter_id))
                .map(build_features)
                .unwrap_or_default();

                println!("features {:?}", &features);

            let limits =
                gfx_select!(adapter_id => global.adapter_limits(adapter_id)).unwrap_or_default();

                println!("limits {:?}", &limits);

            let ret = CanvasGPUAdapter {
                instance: instance.clone(),
                adapter: adapter_id,
                is_fallback_adapter: options.force_fallback_adapter,
                features: features,
                limits: limits,
            };

            Box::into_raw(Box::new(ret))
        });
        let callback = unsafe {
            std::mem::transmute::<*const i64, fn(*mut CanvasGPUAdapter, *mut c_void)>(callback as _)
        };
        let callback_data = callback_data as *mut c_void;
        callback(adapter.unwrap_or(std::ptr::null_mut()), callback_data);
    });
}
