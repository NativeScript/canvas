use std::sync::Arc;
use std::{ffi::CString, os::raw::c_char};

pub struct CanvasGPUAdapterInfo(wgt::AdapterInfo);

impl CanvasGPUAdapterInfo {
    pub fn new(types: wgt::AdapterInfo) -> Self {
        Self(types)
    }

    pub fn vendor(&self) -> &str {
        vendor_name(self.0.vendor, self.0.backend, &self.0.driver)
    }

    pub fn architecture(&self) -> &str {
        ""
    }
    pub fn device(&self) -> &str {
        self.0.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.0.driver_info.as_str()
    }
    pub fn vendor_id(&self) -> u32 {
        self.0.vendor
    }

    pub fn device_id(&self) -> u32 {
        self.0.device
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
    CString::new(info.vendor()).unwrap().into_raw()
}

/// The vendor as browsers report it: a lowercase name for the PCI vendor id, the driver name when unknown.
fn vendor_name(vendor_id: u32, backend: wgt::Backend, driver: &str) -> &str {
    match vendor_id {
        0x1002 => "amd",
        0x10DE => "nvidia",
        0x8086 => "intel",
        0x13B5 => "arm",
        0x5143 => "qualcomm",
        0x1010 => "img-tec",
        0x106B => "apple",
        0x144D => "samsung",
        0x1AE0 => "google",
        0x1414 => "microsoft",
        _ if backend == wgt::Backend::Metal => "apple",
        _ => driver,
    }
}

#[cfg(test)]
mod tests {
    use super::vendor_name;

    #[test]
    fn names_vendors_by_pci_id() {
        assert_eq!(vendor_name(0x13B5, wgt::Backend::Vulkan, "Mali"), "arm");
        assert_eq!(vendor_name(0x5143, wgt::Backend::Vulkan, "Adreno"), "qualcomm");
    }

    #[test]
    fn metal_is_apple_and_unknown_keeps_the_driver() {
        assert_eq!(vendor_name(0, wgt::Backend::Metal, ""), "apple");
        assert_eq!(vendor_name(0xFFFF, wgt::Backend::Vulkan, "SwiftShader"), "SwiftShader");
    }
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
    CString::new(info.0.name.clone()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_info_description(
    info: *const CanvasGPUAdapterInfo,
) -> *mut c_char {
    if info.is_null() {
        return std::ptr::null_mut();
    }
    let info = unsafe { &*info };
    CString::new(info.0.driver_info.clone()).unwrap().into_raw()
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
