use std::{os::raw::c_void, sync::Arc};


////use wgpu_core::gfx_select;
use wgpu_core::global::Global;

use super::gpu_adapter::CanvasGPUAdapter;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasGPUPowerPreference {
    None,
    LowPower,
    HighPerformance,
}

impl Into<wgt::PowerPreference> for CanvasGPUPowerPreference {
    fn into(self) -> wgt::PowerPreference {
        match self {
            CanvasGPUPowerPreference::LowPower => wgt::PowerPreference::LowPower,
            CanvasGPUPowerPreference::HighPerformance => {
                wgt::PowerPreference::HighPerformance
            }
            CanvasGPUPowerPreference::None => wgt::PowerPreference::None,
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

pub struct CanvasWebGPUInstance(CanvasWebGPUInstanceInner);
impl Clone for CanvasWebGPUInstance {
    fn clone(&self) -> Self {
        Self(CanvasWebGPUInstanceInner { global: Arc::clone(&self.0.global) })
    }

    fn clone_from(&mut self, source: &Self) {
        self.0.global = Arc::clone(&source.0.global)
    }
}
impl CanvasWebGPUInstance {
    pub(crate) fn global(&self) -> &Arc<Global> {
        &self.0.global
    }
}

unsafe impl Send for CanvasWebGPUInstance {}
pub type WebGPUInstance = Arc<CanvasWebGPUInstance>;

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_instance_create() -> *const CanvasWebGPUInstance {
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    let backends = wgt::Backends::METAL;

   #[cfg(target_os = "android")]
    let backends = wgt::Backends::from_bits(wgt::Backends::VULKAN.bits() | wgt::Backends::GL.bits()).unwrap();

    #[cfg(target_os = "windows")]
    let backends = wgt::Backends::DX12;

    let instance = Global::new(
        "webgpu",
        wgt::InstanceDescriptor {
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
pub unsafe extern "C" fn canvas_native_webgpu_get_pointer_addr(
    instance: *const CanvasWebGPUInstance,
) -> i64 {
    if instance.is_null() {
        return 0;
    }

    instance as i64
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

    let opts = wgt::RequestAdapterOptions {
        power_preference: options.power_preference.into(),
        force_fallback_adapter: options.force_fallback_adapter,
        compatible_surface: None,
    };

    let callback = callback as i64;
    let callback_data = callback_data as i64;
    let instance = Arc::from_raw(instance);
    let global = Arc::clone(instance.global());
    std::thread::spawn(move || {
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        let backends = wgt::Backends::METAL;

        #[cfg(target_os = "android")]
        let backends = wgt::Backends::VULKAN | wgt::Backends::GL;

        #[cfg(target_os = "windows")]
        let backends = wgt::Backends::DX12;

        let adapter = global.request_adapter(
            &opts,
            backends,
            None,
        );

        let adapter = adapter.map(|adapter_id| {
            let features = build_features(global.adapter_features(adapter_id));

            let limits = global.adapter_limits(adapter_id);

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
