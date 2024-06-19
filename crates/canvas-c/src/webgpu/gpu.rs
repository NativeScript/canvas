use super::gpu_adapter::CanvasGPUAdapter;
use std::os::raw::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasGPUPowerPreference {
    None,
    LowPower,
    HighPerformance,
}

impl Into<wgpu::PowerPreference> for CanvasGPUPowerPreference {
    fn into(self) -> wgpu::PowerPreference {
        match self {
            CanvasGPUPowerPreference::LowPower => wgpu::PowerPreference::LowPower,
            CanvasGPUPowerPreference::HighPerformance => wgpu::PowerPreference::HighPerformance,
            CanvasGPUPowerPreference::None => wgpu::PowerPreference::None,
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

#[cfg(not(target_os = "android"))]
#[no_mangle]
pub extern "C" fn canvas_native_webgpu_request_adapter(
    options: *const CanvasGPURequestAdapterOptions,
    callback: extern "C" fn(*mut CanvasGPUAdapter, *mut c_void),
    callback_data: *mut c_void,
) {
    use super::prelude::build_features;

 

    let backends = wgpu::Backends::METAL;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..Default::default()
    });

    let options = if options.is_null() {
        CanvasGPURequestAdapterOptions::default()
    } else {
        unsafe { *options }
    };

    let opts = wgpu::RequestAdapterOptions {
        power_preference: options.power_preference.into(),
        force_fallback_adapter: options.force_fallback_adapter,
        compatible_surface: None,
    };

    let request = instance.request_adapter(&opts);
    let callback = callback as i64;
    let callback_data = callback_data as i64;
    std::thread::spawn(move || {
        let adapter = futures::executor::block_on(request);
        let adapter = adapter.map(|adapter| {
            let features = build_features(adapter.features());
            let limits = adapter.limits();
            let ret = CanvasGPUAdapter {
                adapter,
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

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn canvas_native_webgpu_request_adapter(
    options: *const CanvasGPURequestAdapterOptions,
    callback: extern "C" fn(*mut CanvasGPUAdapter, *mut c_void),
    callback_data: *mut c_void,
) {
    let backends = wgpu::Backends::VULKAN | wgpu::Backends::GL;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..Default::default()
    });

    let options = if options.is_null() {
        CanvasGPURequestAdapterOptions::default()
    } else {
        unsafe { *options }
    };

    let opts = wgpu::RequestAdapterOptions {
        power_preference: options.power_preference.into(),
        force_fallback_adapter: options.force_fallback_adapter,
        compatible_surface: None,
    };

    let request = instance.request_adapter(&opts);

    let callback = callback as i64;
    let callback_data = callback_data as i64;

    std::thread::spawn(move || {
        let adapter = futures::executor::block_on(request);
        let adapter = adapter.map(|adapter| {
            let features = build_features(adapter.features());
            let limits = adapter.limits();
            let ret = CanvasGPUAdapter {
                adapter,
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
