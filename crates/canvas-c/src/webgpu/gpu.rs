use std::{os::raw::c_void, sync::Arc};

use wgpu_core::global::Global;

use super::gpu_adapter::CanvasGPUAdapter;

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
    pub power_preference: CanvasGPUPowerPreference,
    pub force_fallback_adapter: bool,
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
struct CanvasWebGPUInstanceInner {
    pub(crate) global: Arc<Global>,
}

#[derive(Clone)]
pub struct CanvasWebGPUInstance(CanvasWebGPUInstanceInner);

impl CanvasWebGPUInstance {
    pub(crate) fn global(&self) -> &Arc<Global> {
        &self.0.global
    }
}

unsafe impl Send for CanvasWebGPUInstance {}
pub type WebGPUInstance = Arc<CanvasWebGPUInstance>;

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_instance_create() -> *const CanvasWebGPUInstance {
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

    let inner = CanvasWebGPUInstanceInner {
        global: Arc::new(instance),
    };

    Arc::into_raw(Arc::new(CanvasWebGPUInstance(inner)))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_instance_release(instance: *const CanvasWebGPUInstance) {
    if instance.is_null() {
        return;
    }
    let _ = unsafe { Arc::decrement_strong_count(instance) };
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_request_adapter(
    instance: *const CanvasWebGPUInstance,
    options: *const CanvasGPURequestAdapterOptions,
    callback: extern "C" fn(*const CanvasGPUAdapter, *mut c_void),
    callback_data: *mut c_void,
) {
    use super::prelude::build_features;
    Arc::increment_strong_count(instance);
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
    let instance = Arc::from_raw(instance);
    let global = Arc::clone(instance.global());
    std::thread::spawn(move || {
        #[cfg(not(target_os = "android"))]
        let backends = wgpu_types::Backends::METAL;

        #[cfg(target_os = "android")]
        let backends = wgpu_types::Backends::VULKAN | wgpu_types::Backends::GL;

        #[cfg(target_os = "windows")]
        let backends = wgpu_types::Backends::DX12;

        let adapter = global.request_adapter(
            &opts,
            wgpu_core::instance::AdapterInputs::Mask(backends, |_b| None),
        );

        let adapter = adapter.map(|adapter_id| {
            let features = gfx_select!(adapter_id => global.adapter_features(adapter_id))
                .map(build_features)
                .unwrap_or_default();

            let limits =
                gfx_select!(adapter_id => global.adapter_limits(adapter_id)).unwrap_or_default();

            let ret = CanvasGPUAdapter {
                instance,
                adapter: adapter_id,
                is_fallback_adapter: options.force_fallback_adapter,
                features,
                limits,
            };

            Arc::into_raw(Arc::new(ret))
        });
        let callback = unsafe {
            std::mem::transmute::<*const i64, fn(*const CanvasGPUAdapter, *mut c_void)>(
                callback as _,
            )
        };
        let callback_data = callback_data as *mut c_void;
        callback(adapter.unwrap_or(std::ptr::null()), callback_data);
    });
}
