use crate::buffers::StringBuffer;
use crate::webgpu::gpu_device::{ErrorSinkRaw, DEFAULT_DEVICE_LOST_HANDLER};
use crate::webgpu::gpu_queue::QueueId;
use std::sync::Arc;
use std::{
    ffi::CString,
    os::raw::{c_char, c_void},
};

use super::{
    gpu::CanvasWebGPUInstance, gpu_adapter_info::CanvasGPUAdapterInfo, gpu_device::CanvasGPUDevice,
    gpu_queue::CanvasGPUQueue, gpu_supported_limits::CanvasGPUSupportedLimits, prelude::*,
};

pub struct CanvasGPUAdapter {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) adapter: Arc<wgpu_core::instance::Adapter>,
    pub(crate) is_fallback_adapter: bool,
    pub(crate) feature_level: wgt::FeatureLevel,
    pub(crate) features: Vec<&'static str>,
    pub(crate) limits: wgt::Limits,
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_adapter_reference(adapter: *const CanvasGPUAdapter) {
    if adapter.is_null() {
        return;
    }

    Arc::increment_strong_count(adapter);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_adapter_release(adapter: *const CanvasGPUAdapter) {
    if adapter.is_null() {
        return;
    }

    Arc::decrement_strong_count(adapter);
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
        return Box::into_raw(Box::new(wgt::Limits::default().into()));
    }
    let adapter = unsafe { &*adapter };
    Box::into_raw(Box::new(adapter.limits.clone().into()))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_request_adapter_info(
    adapter: *const CanvasGPUAdapter,
) -> *const CanvasGPUAdapterInfo {
    if adapter.is_null() {
        return std::ptr::null_mut();
    }
    let adapter = unsafe { &*adapter };

    let info = adapter.adapter.get_info();

    Arc::into_raw(Arc::new(CanvasGPUAdapterInfo::new(info)))
}

/// `Adapter::request_device` with VK_KHR_external_semaphore_fd, which video frames shared with
/// GL synchronize with (`linux_video_frame`).
#[cfg(target_os = "linux")]
fn request_device_sharing_semaphores(
    adapter: &Arc<wgpu_core::instance::Adapter>,
    descriptor: &wgpu_core::device::DeviceDescriptor,
) -> Result<
    (Arc<wgpu_core::device::Device>, Arc<wgpu_core::device::queue::Queue>),
    wgpu_core::instance::RequestDeviceError,
> {
    let Some(hal) = (unsafe { Arc::clone(adapter).as_hal::<wgpu_hal::api::Vulkan>() }) else {
        return adapter.request_device(descriptor);
    };
    let mut descriptor = descriptor.clone();
    adapter.validate_device_descriptor(&mut descriptor)?;
    let extension = ash::khr::external_semaphore_fd::NAME;
    let supported = hal.physical_device_capabilities().supports_extension(extension);
    let open = unsafe {
        hal.open_with_callback(
            descriptor.required_features,
            &descriptor.required_limits,
            &descriptor.memory_hints,
            Some(Box::new(move |args: wgpu_hal::vulkan::CreateDeviceCallbackArgs| {
                if supported && !args.extensions.contains(&extension) {
                    args.extensions.push(extension);
                }
            })),
        )
    }
    .map_err(wgpu_core::device::DeviceError::from_hal)?;
    drop(hal);
    unsafe { adapter.create_device_and_queue_from_hal(open.into(), &descriptor) }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_request_device(
    adapter: *const CanvasGPUAdapter,
    label: *const c_char,
    required_features: *const *const c_char,
    required_features_length: usize,
    required_limits: *const CanvasGPUSupportedLimits,
    callback: extern "C" fn(*mut c_char, *const CanvasGPUDevice, *mut c_void),
    callback_data: *mut c_void,
) {
    if adapter.is_null() {
        callback(std::ptr::null_mut(), std::ptr::null_mut(), callback_data);
        return;
    }

    let adapter = unsafe { &*adapter };
    let mut features = parse_required_features(required_features, required_features_length);

    // importExternalTexture is core WebGPU, so enable it wherever the backend can.
    features |= adapter.adapter.features() & wgt::Features::EXTERNAL_TEXTURE;

    let limits = if required_limits.is_null() {
        adapter.limits.clone()
    } else {
        unsafe { *required_limits }.into()
    };

    let label = ptr_into_label(label);

    {
        let adapter_features = adapter.adapter.features();
        if !adapter_features.contains(features) {
            let missing = features.difference(adapter_features);
            let err = format!("Adapter does not support required features: {missing:?}");
            let safe_err = err.replace('\0', "<NUL>");
            let ret = CString::new(safe_err)
                .unwrap_or_else(|_| CString::new("unsupported features").unwrap())
                .into_raw();
            callback(ret, std::ptr::null_mut(), callback_data);
            return;
        }
    }

    let callback = callback as i64;
    let callback_data = callback_data as i64;
    let adapter_inner = Arc::clone(&adapter.adapter);
    let feature_level = adapter.feature_level;
    let instance = Arc::clone(&adapter.instance);
    std::thread::spawn(move || {
        let descriptor = wgt::DeviceDescriptor {
            label: label.clone(),
            required_features: features,
            required_limits: limits,
            memory_hints: Default::default(),
            default_queue: wgt::QueueDescriptor { label: None },
            trace: Default::default(),
            experimental_features: Default::default(),
        };

        let callback = unsafe {
            std::mem::transmute::<*const i64, fn(*mut c_char, *const CanvasGPUDevice, *mut c_void)>(
                callback as _,
            )
        };
        let callback_data = callback_data as *mut c_void;

        #[cfg(target_os = "linux")]
        let device = request_device_sharing_semaphores(&adapter_inner, &descriptor);
        #[cfg(not(target_os = "linux"))]
        let device = adapter_inner.request_device(&descriptor);
        match device {
            Ok((device, queue)) => {
                let error_sink = Arc::new(parking_lot::Mutex::new(ErrorSinkRaw::new(
                    DEFAULT_DEVICE_LOST_HANDLER,
                )));
                // wgpu's own default panics; errors from the queue thread must be routed.
                super::gpu_device::install_uncaptured_handler(
                    &device,
                    &error_sink,
                    None,
                    std::ptr::null_mut(),
                );

                let queue = Arc::new(CanvasGPUQueue {
                    label: descriptor.label,
                    device_id: Arc::clone(&device),
                    queue: Arc::new(QueueId {
                        id: queue,
                        instance: Arc::clone(&instance),
                    }),
                    error_sink: error_sink.clone(),
                    blit: Arc::new(parking_lot::Mutex::new(None)),
                    #[cfg(target_os = "windows")]
                    shared_frames: Default::default(),
                    #[cfg(target_os = "linux")]
                    linux_frames: Default::default(),
                });

                let ret = Arc::into_raw(Arc::new(CanvasGPUDevice {
                    label,
                    device,
                    adapter: Arc::clone(&adapter_inner),
                    queue,
                    user_data: std::ptr::null_mut(),
                    instance,
                    error_sink,
                    feature_level,
                }));
                callback(std::ptr::null_mut(), ret, callback_data);
            }
            Err(error) => {
                let error = error.to_string();
                let safe_error = error.replace('\0', "<NUL>");
                let ret = CString::new(safe_error)
                    .unwrap_or_else(|_| CString::new("requestDevice failed").unwrap())
                    .into_raw();
                callback(ret, std::ptr::null_mut(), callback_data);
            }
        }
    });
}
