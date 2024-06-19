use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
    sync::Arc,
};

use crate::buffers::StringBuffer;

use super::{
    gpu_adapter_info::CanvasGPUAdapterInfo, gpu_device::CanvasGPUDevice, gpu_queue::CanvasGPUQueue, gpu_supported_limits::CanvasGPUSupportedLimits, prelude::parse_required_features
};

pub struct CanvasGPUAdapter {
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) is_fallback_adapter: bool,
    pub(crate) features: Vec<&'static str>,
    pub(crate) limits: wgpu::Limits,
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_get_features(
    adapter: *const CanvasGPUAdapter,
) -> *mut StringBuffer {
    if adapter.is_null() {
        return std::ptr::null_mut();
    }
    let adapter = unsafe { &*adapter };
    let buffer = StringBuffer::from(adapter.features.clone());
    Box::into_raw(Box::new(buffer))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_is_fallback_adapter(
    adapter: *const CanvasGPUAdapter,
) -> bool {
    if adapter.is_null() {
        return false;
    }
    let adapter = unsafe { &*adapter };
    adapter.is_fallback_adapter
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_get_limits(
    adapter: *const CanvasGPUAdapter,
) -> *mut CanvasGPUSupportedLimits {
    if adapter.is_null() {
        return Box::into_raw(Box::new(wgpu::Limits::default().into()));
    }
    let adapter = unsafe { &*adapter };
    Box::into_raw(Box::new(adapter.limits.clone().into()))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_request_adapter_info(
    adapter: *mut CanvasGPUAdapter,
) -> *mut CanvasGPUAdapterInfo {
    if adapter.is_null() {
        return std::ptr::null_mut();
    }
    let adapter = unsafe { &mut *adapter };
    let info = adapter.adapter.get_info();

    Box::into_raw(Box::new(CanvasGPUAdapterInfo(info)))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_request_device(
    adapter: *mut CanvasGPUAdapter,
    label: *const c_char,
    required_features: *const *const c_char,
    required_features_length: usize,
    required_limits: *const CanvasGPUSupportedLimits,
    callback: extern "C" fn(*mut c_char, *mut CanvasGPUDevice, *mut c_void),
    callback_data: *mut c_void,
) {
    if adapter.is_null() {
        callback(std::ptr::null_mut(), std::ptr::null_mut(), callback_data);
        return;
    }

    let adapter = unsafe { &mut *adapter };
    let features = parse_required_features(required_features, required_features_length);

    let mut limits = if required_limits.is_null() {
        wgpu::Limits::default()
    } else {
        unsafe { *required_limits }.into()
    };

    let label = if !label.is_null() {
        Some(unsafe { CStr::from_ptr(label).to_string_lossy() })
    } else {
        None
    };

    let descriptor = wgpu::DeviceDescriptor {
        label: label.as_deref(),
        required_features: features,
        required_limits: limits,
    };
    let request = adapter.adapter.request_device(&descriptor, None);

    let callback = callback as i64;
    let callback_data = callback_data as i64;

    std::thread::spawn(move || {
        let adapter = futures::executor::block_on(request);
        let callback = unsafe {
            std::mem::transmute::<*const i64, fn(*mut c_char, *mut CanvasGPUDevice, *mut c_void)>(
                callback as _,
            )
        };
        let callback_data = callback_data as *mut c_void;

        match adapter {
            Ok((device, queue)) => {
                let queue = CanvasGPUQueue(Arc::new(parking_lot::RwLock::new(queue)));
                let ret = Box::into_raw(Box::new(CanvasGPUDevice { device, queue }));
                callback(std::ptr::null_mut(), ret, callback_data);
            }
            Err(error) => {
                let error = error.to_string();
                let ret = CString::new(error).unwrap().into_raw();
                callback(ret, std::ptr::null_mut(), callback_data);
            }
        }
    });
}
