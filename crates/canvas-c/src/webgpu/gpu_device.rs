use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
};


use crate::buffers::StringBuffer;

use super::{
    gpu::CanvasWebGPUInstance, gpu_buffer::CanvasGPUBuffer,
    gpu_command_encoder::CanvasGPUCommandEncoder, gpu_queue::CanvasGPUQueue,
    gpu_shader_module::CanvasGPUShaderModule, gpu_supported_limits::CanvasGPUSupportedLimits,
    prelude::*,
};

pub struct CanvasGPUDevice {
    pub(crate) instance: CanvasWebGPUInstance,
    pub(crate) device: wgpu_core::id::DeviceId,
    pub(crate) queue: CanvasGPUQueue,
}

impl CanvasGPUDevice {
    // pub fn createBindGroup(&self, descriptor) {
    //     self.device.create_bind_group(desc)
    // }

    // pub fn createBindGroupLayout(&self, descriptor){}

    pub fn features(&self) -> Result<wgpu_types::Features, wgpu_core::device::InvalidDevice> {
        let device_id = self.device;
        let global = &self.instance.0;
        gfx_select!(device_id => global.device_features(device_id))
    }

    pub fn destroy(&self) {
        let device_id = self.device;
        let global = &self.instance.0;
        gfx_select!(device_id => global.device_destroy(device_id));
    }

    pub fn create_buffer(
        &self,
        label: *const c_char,
        size: u64,
        usage: u32,
        mapped_at_creation: bool,
        mut error: *mut c_char,
    ) -> *mut CanvasGPUBuffer {
        let label = if !label.is_null() {
            Some(unsafe { CStr::from_ptr(label).to_string_lossy() })
        } else {
            None
        };
        match wgpu_types::BufferUsages::from_bits(usage) {
            Some(usage) => {
                let desc = wgpu_types::BufferDescriptor {
                    label: label.clone(),
                    size: size,
                    usage: usage,
                    mapped_at_creation: mapped_at_creation,
                };

                let device_id = self.device;
                let global = &self.instance.0;
                let (buffer, err) =
                    gfx_select!(device_id => global.device_create_buffer(device_id, &desc, None));

                    // todo handle error
                if let Some(_) = err {
                    error = CString::new("usage is not valid").unwrap().into_raw();
                    std::ptr::null_mut()
                } else {
                    Box::into_raw(Box::new(CanvasGPUBuffer {
                        instance: self.instance.clone(),
                        label: label.unwrap_or_default(),
                        buffer,
                        usage: usage.bits(),
                        size,
                    }))
                }
            }
            None => {
                error = CString::new("usage is not valid").unwrap().into_raw();
                std::ptr::null_mut()
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_get_features(
    device: *const CanvasGPUDevice,
) -> *mut StringBuffer {
    if device.is_null() {
        return std::ptr::null_mut();
    }
    let device = unsafe { &*device };
    let features = build_features(device.features().unwrap_or_default());
    let buffer = StringBuffer::from(features);
    Box::into_raw(Box::new(buffer))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_get_limits(
    device: *const CanvasGPUDevice,
) -> *mut CanvasGPUSupportedLimits {
    if device.is_null() {
        return Box::into_raw(Box::new(wgpu_types::Limits::default().into()));
    }
    let device = unsafe { &*device };
    let device_id = device.device;
    let global = &device.instance.0;
    match gfx_select!(device_id => global.device_limits(device_id)) {
        Ok(limits) => {
            let limits: CanvasGPUSupportedLimits = limits.into();
            Box::into_raw(Box::new(limits))
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_get_queue(
    device: *const CanvasGPUDevice,
) -> *mut CanvasGPUQueue {
    if device.is_null() {
        return std::ptr::null_mut();
    }
    let device = unsafe { &*device };
    Box::into_raw(Box::new(device.queue.clone()))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_set_lost_callback(
    device: *const CanvasGPUDevice,
    callback: extern "C" fn(i32, *mut c_char, *mut c_void),
    callback_data: *mut c_void,
) {
    if device.is_null() {
        return;
    }

    let device = unsafe { &*device };
    let callback = callback as i64;
    let callback_data = callback_data as i64;
    let callback = Box::new(move |reason, message| {
        let callback = unsafe {
            std::mem::transmute::<*const i64, extern "C" fn(i32, *mut c_char, *mut c_void)>(
                callback as _,
            )
        };
        let callback_data = callback_data as *mut c_void;
        callback(
            reason as i32,
            CString::new(message).unwrap().into_raw(),
            callback_data,
        );
    });

    let device_id = device.device;
    let global = &device.instance.0;

    gfx_select!(device_id => global.device_set_device_lost_closure(
        device_id,
        wgpu_core::device::DeviceLostClosure::from_rust(callback),
    ));
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_destroy(device: *const CanvasGPUDevice) {
    if device.is_null() {
        return;
    }

    let device = unsafe { &*device };
    device.destroy();
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_destroy_destory(device: *mut CanvasGPUDevice) {
    if device.is_null() {
        return;
    }

    let _ = unsafe { Box::from_raw(device) };
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_create_command_encoder(
    device: *const CanvasGPUDevice,
    label: *const c_char,
) -> *mut CanvasGPUCommandEncoder {
    if device.is_null() {
        return std::ptr::null_mut();
    }
    let label = if !label.is_null() {
        Some(unsafe { CStr::from_ptr(label).to_string_lossy() })
    } else {
        None
    };

    let device = unsafe { &*device };
    let desc = wgpu_types::CommandEncoderDescriptor { label: label };
    // let encoder = CanvasGPUCommandEncoder(
    //     Arc::new(parking_lot::RwLock::new(device.device.create_command_encoder(&desc)))
    // );

    let device_id = device.device;
    let global = &device.instance.0;
    let (encoder, error) =
        gfx_select!(device_id => global.device_create_command_encoder(device_id, &desc, None));

    // todo handle error
    if let Some(error) = error {
        std::ptr::null_mut()
    } else {
        let encoder = CanvasGPUCommandEncoder {
            instance: device.instance.clone(),
            encoder,
        };
        Box::into_raw(Box::new(encoder))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_create_shader_module(
    device: *const CanvasGPUDevice,
    label: *const c_char,
    source: *const c_char,
) -> *mut CanvasGPUShaderModule {
    if device.is_null() {
        return std::ptr::null_mut();
    }
    let label = if !label.is_null() {
        Some(unsafe { CStr::from_ptr(label).to_string_lossy() })
    } else {
        None
    };
    let src = unsafe { CStr::from_ptr(source) };
    let src = src.to_string_lossy();
    let source = wgpu_core::pipeline::ShaderModuleSource::Wgsl(src);

    let device = unsafe { &*device };
    let desc = wgpu_core::pipeline::ShaderModuleDescriptor {
        label: label,
        shader_bound_checks: wgpu_types::ShaderBoundChecks::default(),
    };

    let device_id = device.device;
    let global = &device.instance.0;

    let (module, error) = gfx_select!(device_id => global.device_create_shader_module(device_id, &desc, source, None));

    // todo handle error
    if let Some(error) = error {
        std::ptr::null_mut()
    } else {
        let shader = CanvasGPUShaderModule {
            module,
            instance: device.instance.clone(),
        };
        Box::into_raw(Box::new(shader))
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_create_buffer(
    device: *const CanvasGPUDevice,
    label: *const c_char,
    size: u64,
    usage: u32,
    mapped_at_creation: bool,
    mut error: *mut c_char,
) -> *mut CanvasGPUBuffer {
    if device.is_null() {
        return std::ptr::null_mut();
    }

    let device = unsafe { &*device };

    device.create_buffer(label, size, usage, mapped_at_creation, error)
}
