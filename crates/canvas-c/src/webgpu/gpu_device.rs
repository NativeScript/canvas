use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
};

use crate::buffers::StringBuffer;

use super::{
    gpu_buffer::CanvasGPUBuffer, gpu_command_encoder::CanvasGPUCommandEncoder, gpu_queue::CanvasGPUQueue, gpu_shader_module::CanvasGPUShaderModule, gpu_supported_limits::CanvasGPUSupportedLimits, prelude::build_features
};

pub struct CanvasGPUDevice {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: CanvasGPUQueue,
}

impl CanvasGPUDevice {
    // pub fn createBindGroup(&self, descriptor) {
    //     self.device.create_bind_group(desc)
    // }

    // pub fn createBindGroupLayout(&self, descriptor){}

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
        match wgpu::BufferUsages::from_bits(usage) {
            Some(usage) => {
                let desc = wgpu::BufferDescriptor {
                    label: label.as_deref(),
                    size: size,
                    usage: usage,
                    mapped_at_creation: mapped_at_creation,
                };
                let buffer = self.device.create_buffer(&desc);

                Box::into_raw(Box::new(CanvasGPUBuffer { label: label.unwrap_or_default(), buffer }))
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
    let features = build_features(device.device.features());
    let buffer = StringBuffer::from(features);
    Box::into_raw(Box::new(buffer))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_get_limits(
    device: *const CanvasGPUDevice,
) -> *mut CanvasGPUSupportedLimits {
    if device.is_null() {
        return Box::into_raw(Box::new(wgpu::Limits::default().into()));
    }
    let device = unsafe { &*device };
    let limits: CanvasGPUSupportedLimits = device.device.limits().into();
    Box::into_raw(Box::new(limits))
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
    device.device.set_device_lost_callback(callback);
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_destroy(device: *const CanvasGPUDevice) {
    if device.is_null() {
        return;
    }

    let device = unsafe { &*device };
    device.device.destroy();
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
    let desc = wgpu::CommandEncoderDescriptor {
        label: label.as_deref(),
    };
    // let encoder = CanvasGPUCommandEncoder(
    //     Arc::new(parking_lot::RwLock::new(device.device.create_command_encoder(&desc)))
    // );
    let encoder = CanvasGPUCommandEncoder(device.device.create_command_encoder(&desc));
    Box::into_raw(Box::new(encoder))
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
    let source = wgpu::ShaderSource::Wgsl(src);

    let device = unsafe { &*device };
    let desc = wgpu::ShaderModuleDescriptor {
        label: label.as_deref(),
        source,
    };
    let encoder = CanvasGPUShaderModule(device.device.create_shader_module(desc));
    Box::into_raw(Box::new(encoder))
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
