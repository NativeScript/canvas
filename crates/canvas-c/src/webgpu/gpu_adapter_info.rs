use std::sync::Arc;
use std::{ffi::CString, os::raw::c_char};

pub struct CanvasGPUAdapterInfo(wgt::AdapterInfo);

impl CanvasGPUAdapterInfo {
    pub fn new(types: wgt::AdapterInfo) -> Self {
        Self(types)
    }

    pub fn vendor(&self) -> String {
        self.0.vendor.to_string()
    }

    pub fn architecture(&self) -> &str {
        ""
    }
    pub fn device(&self) -> String {
        self.0.device.to_string()
    }

    pub fn description(&self) -> &str {
        self.0.name.as_str()
    }
}


#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_info_vendor(
    info: *const CanvasGPUAdapterInfo,
) -> *mut c_char {
    if info.is_null() {
        return std::ptr::null_mut();
    }
    let info = unsafe { &*info };
    CString::new(info.0.vendor.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_info_architecture(
    _info: *const CanvasGPUAdapterInfo,
) -> *mut c_char {
    // if info.is_null() {return std::ptr::null()}
    // let info = unsafe {&*info};
    // CString::new().unwrap().into_raw()
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_info_device(
    info: *const CanvasGPUAdapterInfo,
) -> *mut c_char {
    if info.is_null() {
        return std::ptr::null_mut();
    }
    let info = unsafe { &*info };
    CString::new(info.0.device.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_info_description(
    info: *const CanvasGPUAdapterInfo,
) -> *mut c_char {
    if info.is_null() {
        return std::ptr::null_mut();
    }
    let info = unsafe { &*info };
    CString::new(info.0.name.clone()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_adapter_info_reference(
    info: *const CanvasGPUAdapterInfo,
) {
    if info.is_null() {
        return;
    }

    Arc::increment_strong_count(info);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_adapter_info_release(
    info: *const CanvasGPUAdapterInfo,
) {
    if info.is_null() {
        return;
    }

    Arc::decrement_strong_count(info);
}
