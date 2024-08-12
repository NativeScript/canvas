use std::{
    borrow::Cow,
    collections::HashMap,
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
};
use std::cmp::PartialEq;
use std::num::NonZeroU64;
use std::sync::Arc;

use wgpu_core::binding_model::BufferBinding;
use wgpu_core::command::RenderBundleEncoder;
use wgpu_core::device::DeviceError;
use wgpu_core::id::PipelineLayoutId;
use wgpu_core::pipeline::{CreateRenderPipelineError, RenderPipelineDescriptor};
//use wgpu_core::gfx_select;
use wgpu_core::resource::CreateBufferError;
use wgt::Features;

use crate::buffers::StringBuffer;
use crate::webgpu::enums::{
    CanvasAddressMode, CanvasBindGroupEntry, CanvasBindGroupEntryResource,
    CanvasBindGroupLayoutEntry, CanvasFilterMode, CanvasOptionalCompareFunction,
    CanvasOptionalGPUTextureFormat, CanvasQueryType,
};
use crate::webgpu::error::{CanvasGPUError, CanvasGPUErrorType, handle_error, handle_error_fatal};
use crate::webgpu::gpu_bind_group::CanvasGPUBindGroup;
use crate::webgpu::gpu_bind_group_layout::CanvasGPUBindGroupLayout;
use crate::webgpu::gpu_compute_pipeline::CanvasGPUComputePipeline;
use crate::webgpu::gpu_query_set::CanvasGPUQuerySet;
use crate::webgpu::gpu_render_bundle_encoder::CanvasGPURenderBundleEncoder;
use crate::webgpu::gpu_sampler::CanvasGPUSampler;

use super::{
    enums::{
        CanvasCompareFunction, CanvasCullMode, CanvasFrontFace, CanvasGPUTextureFormat,
        CanvasOptionalIndexFormat, CanvasPrimitiveTopology, CanvasStencilFaceState,
        CanvasTextureDimension, CanvasVertexStepMode,
    },
    gpu::CanvasWebGPUInstance,
    gpu_buffer::CanvasGPUBuffer,
    gpu_command_encoder::CanvasGPUCommandEncoder,
    gpu_pipeline_layout::CanvasGPUPipelineLayout,
    gpu_queue::CanvasGPUQueue,
    gpu_render_pipeline::CanvasGPURenderPipeline,
    gpu_shader_module::CanvasGPUShaderModule,
    gpu_supported_limits::CanvasGPUSupportedLimits,
    gpu_texture::CanvasGPUTexture,
    prelude::*,
    structs::{CanvasColorTargetState, CanvasMultisampleState, CanvasVertexAttribute},
};

const MAX_BIND_GROUPS: usize = 8;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum CanvasGPUErrorFilter {
    /// Catch only out-of-memory errors.
    OutOfMemory,
    /// Catch only validation errors.
    Validation,
    /// Catch only internal errors.
    Internal,
}

pub(crate) type ErrorSink = Arc<parking_lot::Mutex<ErrorSinkRaw>>;

struct ErrorScope {
    error: Option<CanvasGPUError>,
    filter: CanvasGPUErrorFilter,
}

pub struct DeviceCallback<T> {
    callback: T,
    userdata: *mut c_void,
}
unsafe impl<T> Send for DeviceCallback<T> {}

pub type GPUErrorCallback =
Option<unsafe extern "C" fn(CanvasGPUErrorType, *mut c_char, *mut c_void)>;
pub type GPUDeviceLostCallback = Option<unsafe extern "C" fn(i32, *mut c_char, *mut c_void)>;
pub type UncapturedErrorCallback = DeviceCallback<GPUErrorCallback>;
pub(crate) type DeviceLostCallback = DeviceCallback<GPUDeviceLostCallback>;

unsafe extern "C" fn default_uncaptured_error_handler(
    _typ: super::error::CanvasGPUErrorType,
    message: *mut ::std::os::raw::c_char,
    _userdata: *mut ::std::os::raw::c_void,
) {
    let message = unsafe { CStr::from_ptr(message) }.to_str().unwrap();

    #[cfg(target_os = "android")]
    log::warn!("Handling webgpu uncaptured errors as fatal by default");
    #[cfg(target_os = "android")]
    log::error!("webgpu uncaptured error:\n{message}\n");

    #[cfg(not(target_os = "android"))]
    println!("Handling webgpu uncaptured errors as fatal by default");
    #[cfg(not(target_os = "android"))]
    println!("webgpu uncaptured error:\n{message}\n");
}

const DEFAULT_UNCAPTURED_ERROR_HANDLER: UncapturedErrorCallback = UncapturedErrorCallback {
    callback: Some(default_uncaptured_error_handler),
    userdata: std::ptr::null_mut(),
};

pub(crate) unsafe extern "C" fn default_device_lost_handler(
    _reason: i32,
    message: *mut c_char,
    _userdata: *mut c_void,
) {
    let message = unsafe { CStr::from_ptr(message) }.to_str().unwrap();

    #[cfg(target_os = "android")]
    log::warn!("Handling webgpu device lost errors as fatal by default");
    #[cfg(target_os = "android")]
    log::error!("webgpu device lost error:\n{message}\n");

    #[cfg(not(target_os = "android"))]
    println!("Handling webgpu device lost errors as fatal by default");
    #[cfg(not(target_os = "android"))]
    println!("webgpu device lost error:\n{message}\n");
}
pub const DEFAULT_DEVICE_LOST_HANDLER: DeviceLostCallback = DeviceLostCallback {
    callback: Some(default_device_lost_handler),
    userdata: std::ptr::null_mut(),
};

pub struct ErrorSinkRaw {
    scopes: Vec<ErrorScope>,
    uncaptured_handler: UncapturedErrorCallback,
    device_lost_handler: DeviceLostCallback,
}

impl PartialEq for CanvasGPUError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CanvasGPUError::Internal, CanvasGPUError::Internal) => true,
            (CanvasGPUError::OutOfMemory, CanvasGPUError::OutOfMemory) => true,
            (CanvasGPUError::Lost, CanvasGPUError::Lost) => true,
            (CanvasGPUError::Validation(_), CanvasGPUError::Validation(_)) => true,
            _ => false,
        }
    }
}

impl ErrorSinkRaw {
    pub(crate) fn new(device_lost_handler: DeviceLostCallback) -> ErrorSinkRaw {
        ErrorSinkRaw {
            scopes: Vec::new(),
            uncaptured_handler: DEFAULT_UNCAPTURED_ERROR_HANDLER,
            device_lost_handler,
        }
    }

    pub(crate) fn handle_error(&mut self, err: CanvasGPUError) {
        let (typ, filter) = match err {
            super::error::CanvasGPUError::Lost { .. } => {
                // handle device lost error early
                if let Some(callback) = self.device_lost_handler.callback {
                    let userdata = self.device_lost_handler.userdata;
                    let msg = CString::new(err.to_string()).unwrap();
                    unsafe {
                        callback(
                            wgt::DeviceLostReason::Destroyed as i32,
                            msg.into_raw(),
                            userdata,
                        );
                    };
                }
                return;
            }
            CanvasGPUError::OutOfMemory { .. } => (
                super::error::CanvasGPUErrorType::OutOfMemory,
                CanvasGPUErrorFilter::OutOfMemory,
            ),
            super::error::CanvasGPUError::Validation { .. } => (
                super::error::CanvasGPUErrorType::Validation,
                CanvasGPUErrorFilter::Validation,
            ),
            super::error::CanvasGPUError::Internal { .. } => (
                super::error::CanvasGPUErrorType::Internal,
                CanvasGPUErrorFilter::Internal,
            ),
            super::error::CanvasGPUError::None { .. } => (
                super::error::CanvasGPUErrorType::None,
                CanvasGPUErrorFilter::Internal,
            ),
        };

        if typ == CanvasGPUErrorType::None {
            return;
        }

        match self
            .scopes
            .iter_mut()
            .rev()
            .find(|scope| scope.filter == filter)
        {
            Some(scope) => {
                if scope.error.is_none() {
                    scope.error = Some(err);
                }
            }
            None => {
                if let Some(callback) = self.uncaptured_handler.callback {
                    let userdata = self.uncaptured_handler.userdata;
                    let msg = CString::new(err.to_string()).unwrap();
                    unsafe { callback(typ, msg.into_raw(), userdata) };
                }
            }
        }
    }
}

pub struct CanvasGPUDevice {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) device: wgpu_core::id::DeviceId,
    pub(crate) queue: Arc<CanvasGPUQueue>,
    pub(crate) user_data: *mut c_void,
    pub(crate) error_sink: ErrorSink,
}

impl CanvasGPUDevice {
    pub fn features(&self) -> Result<Features, DeviceError> {
        let device_id = self.device;
        let global = self.instance.global();
        gfx_select!(device_id => global.device_features(device_id))
    }

    pub fn destroy(&self) {
        let device_id = self.device;
        let global = self.instance.global();
        gfx_select!(device_id => global.device_destroy(device_id));
    }

    pub fn create_bind_group(
        &self,
        label: Option<Cow<'static, str>>,
        layout: &CanvasGPUBindGroupLayout,
        entries: &[CanvasBindGroupEntry],
    ) -> *const CanvasGPUBindGroup {
        let global = self.instance.global();
        let layout_id = layout.group_layout;
        let entries = entries
            .iter()
            .map(|entry| wgpu_core::binding_model::BindGroupEntry {
                binding: entry.binding,
                resource: match &entry.resource {
                    CanvasBindGroupEntryResource::Buffer(buffer) => {
                        let buf = unsafe { &*buffer.buffer };
                        wgpu_core::binding_model::BindingResource::Buffer(BufferBinding {
                            buffer_id: buf.buffer,
                            offset: buffer.offset.try_into().ok().unwrap_or_default(),
                            size: buffer
                                .size
                                .try_into()
                                .map(|value: u64| NonZeroU64::new(value))
                                .ok()
                                .flatten(),
                        })
                    }
                    CanvasBindGroupEntryResource::Sampler(sampler) => {
                        let sampler = unsafe { &**sampler };
                        wgpu_core::binding_model::BindingResource::Sampler(sampler.sampler)
                    }
                    CanvasBindGroupEntryResource::TextureView(view) => {
                        let view = unsafe { &**view };
                        wgpu_core::binding_model::BindingResource::TextureView(view.texture_view)
                    }
                },
            })
            .collect::<Vec<_>>();
        let desc = wgpu_core::binding_model::BindGroupDescriptor {
            label: label.clone(),
            layout: layout_id,
            entries: Cow::from(entries),
        };
        let device_id = self.device;
        let (group, error) =
            gfx_select!(device_id => global.device_create_bind_group(device_id, &desc, None));

        let error_sink = self.error_sink.as_ref();
        if let Some(cause) = error {
            handle_error(
                global,
                error_sink,
                cause,
                "label",
                desc.label,
                "canvas_native_webgpu_device_create_bind_group",
            );
        };

        Arc::into_raw(Arc::new(CanvasGPUBindGroup {
            label,
            instance: self.instance.clone(),
            group,
        }))
    }

    pub fn create_bind_group_layout(
        &self,
        label: Option<Cow<'static, str>>,
        entries: &[CanvasBindGroupLayoutEntry],
    ) -> *const CanvasGPUBindGroupLayout {
        let global = self.instance.global();
        let entries = entries
            .iter()
            .map(|entry| (*entry).into())
            .collect::<Vec<_>>();
        let desc = wgpu_core::binding_model::BindGroupLayoutDescriptor {
            label: label.clone(),
            entries: Cow::from(entries),
        };

        let device_id = self.device;
        let (group_layout_id, error) = gfx_select!(device_id => global.device_create_bind_group_layout(device_id, &desc, None));

        let error_sink = self.error_sink.as_ref();
        if let Some(cause) = error {
            handle_error(
                global,
                error_sink,
                cause,
                "label",
                desc.label,
                "canvas_native_webgpu_device_create_bind_group_layout",
            );
        }
        Arc::into_raw(Arc::new(CanvasGPUBindGroupLayout {
            instance: self.instance.clone(),
            group_layout: group_layout_id,
        }))
    }

    pub fn create_buffer(
        &self,
        label: *const c_char,
        size: u64,
        usage: u32,
        mapped_at_creation: bool,
    ) -> *const CanvasGPUBuffer {
        let label = ptr_into_label(label);
        let global = self.instance.global();

        match wgt::BufferUsages::from_bits(usage) {
            Some(usage) => {
                let desc = wgt::BufferDescriptor {
                    label: label.clone(),
                    size,
                    usage,
                    mapped_at_creation,
                };

                let device_id = self.device;

                let (buffer, err) =
                    gfx_select!(device_id => global.device_create_buffer(device_id, &desc, None));

                let error_sink = self.error_sink.as_ref();
                if let Some(cause) = err {
                    handle_error(
                        global,
                        error_sink,
                        cause,
                        "label",
                        desc.label,
                        "canvas_native_webgpu_device_create_buffer",
                    );
                }

                Arc::into_raw(Arc::new(CanvasGPUBuffer {
                    instance: self.instance.clone(),
                    label: label.unwrap_or_default(),
                    buffer,
                    usage: usage.bits(),
                    size,
                    error_sink: self.error_sink.clone(),
                }))
            }
            None => {
                let err = CreateBufferError::InvalidUsage(
                    wgt::BufferUsages::from_bits_truncate(usage),
                );

                handle_error(
                    global,
                    self.error_sink.as_ref(),
                    err,
                    "label",
                    label,
                    "canvas_native_webgpu_device_create_buffer",
                );

                std::ptr::null_mut()
            }
        }
    }
}

unsafe impl Send for CanvasGPUDevice {}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_get_features(
    device: *const CanvasGPUDevice,
) -> *mut StringBuffer {
    if device.is_null() {
        return std::ptr::null_mut();
    }
    let device = unsafe { &*device };
    let features = device.features().unwrap_or_default();
    let features = build_features(features);
    let buffer = StringBuffer::from(features);
    Box::into_raw(Box::new(buffer))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_get_limits(
    device: *const CanvasGPUDevice,
) -> *mut CanvasGPUSupportedLimits {
    if device.is_null() {
        return Box::into_raw(Box::new(wgt::Limits::default().into()));
    }
    let device = unsafe { &*device };
    let device_id = device.device;
    let global = device.instance.global();
    match gfx_select!(device_id => global.device_limits(device_id)) {
        Ok(limits) => {
            let limits: CanvasGPUSupportedLimits = limits.into();
            Box::into_raw(Box::new(limits))
        }
        Err(err) => {
            handle_error_fatal(global, err, "canvas_native_webgpu_device_get_limits");
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_get_queue(
    device: *const CanvasGPUDevice,
) -> *const CanvasGPUQueue {
    if device.is_null() {
        return std::ptr::null();
    }
    let device = unsafe { &*device };
    Arc::into_raw(device.queue.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_set_lost_callback(
    device: *const CanvasGPUDevice,
    callback: Option<unsafe extern "C" fn(i32, *mut c_char, *mut c_void)>,
    userdata: *mut c_void,
) {
    if device.is_null() {
        return;
    }

    let device = &*device;

    let mut error_sink = device.error_sink.lock();
    error_sink.device_lost_handler = DeviceLostCallback { callback, userdata };
    // let callback = callback as i64;
    // let callback_data = callback_data as i64;
    // let callback = Box::new(move |reason, message| {
    //     let callback = unsafe {
    //         std::mem::transmute::<*const i64, extern "C" fn(i32, *mut c_char, *mut c_void)>(
    //             callback as _,
    //         )
    //     };
    //     let callback_data = callback_data as *mut c_void;
    //     callback(
    //         reason as i32,
    //         CString::new(message).unwrap().into_raw(),
    //         callback_data,
    //     );
    // });
    //
    // let device_id = device.device;
    // let global = device.instance.global();
    //
    // gfx_select!(device_id => global.device_set_device_lost_closure(
    //     device_id,
    //     wgpu_core::device::DeviceLostClosure::from_rust(callback),
    // ));
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_pop_error_scope(
    device: *const CanvasGPUDevice,
    callback: Option<unsafe extern "C" fn(CanvasGPUErrorType, *mut c_char, *mut c_void)>,
    userdata: *mut ::std::os::raw::c_void,
) {
    let device = &*device;
    let callback = callback;
    let mut error_sink = device.error_sink.lock();
    let scope = error_sink.scopes.pop().unwrap();

    match scope.error {
        Some(error) => {
            let typ = match error {
                CanvasGPUError::OutOfMemory { .. } => CanvasGPUErrorType::OutOfMemory,
                CanvasGPUError::Validation { .. } => CanvasGPUErrorType::Validation,
                // We handle device lost error early in ErrorSinkRaw::handle_error
                // so we should never get device lost error here.
                CanvasGPUError::Lost { .. } => unreachable!(),
                CanvasGPUError::None => CanvasGPUErrorType::None,
                CanvasGPUError::Internal => CanvasGPUErrorType::Internal,
            };

            let msg = CString::new(error.to_string()).unwrap();
            unsafe {
                if let Some(callback) = callback {
                    callback(typ, msg.into_raw(), userdata);
                }
            };
        }
        None => {
            unsafe {
                if let Some(callback) = callback {
                    callback(CanvasGPUErrorType::None, std::ptr::null_mut(), userdata);
                }
            };
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_push_error_scope(
    device: *const CanvasGPUDevice,
    filter: CanvasGPUErrorFilter,
) {
    let device = &*device;
    let mut error_sink = device.error_sink.lock();
    error_sink.scopes.push(ErrorScope {
        error: None,
        filter,
    });
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_set_uncaptured_error_callback(
    device: *const CanvasGPUDevice,
    callback: Option<unsafe extern "C" fn(CanvasGPUErrorType, *mut c_char, *mut c_void)>,
    userdata: *mut std::os::raw::c_void,
) {
    let device = &*device;
    let mut error_sink = device.error_sink.lock();
    error_sink.uncaptured_handler = UncapturedErrorCallback { callback, userdata };
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
pub unsafe extern "C" fn canvas_native_webgpu_device_reference(device: *const CanvasGPUDevice) {
    if device.is_null() {
        return;
    }
    Arc::increment_strong_count(device);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_release(device: *const CanvasGPUDevice) {
    if device.is_null() {
        return;
    }

    Arc::decrement_strong_count(device);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_create_bind_group_layout(
    device: *const CanvasGPUDevice,
    label: *const c_char,
    entries: *const CanvasBindGroupLayoutEntry,
    size: usize,
) -> *const CanvasGPUBindGroupLayout {
    if device.is_null() || entries.is_null() || size == 0 {
        return std::ptr::null();
    }

    let device = unsafe { &*device };

    let entries = std::slice::from_raw_parts(entries, size);

    let label = ptr_into_label(label);

    device.create_bind_group_layout(label, entries)
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_create_bind_group(
    device: *const CanvasGPUDevice,
    label: *const c_char,
    layout: *const CanvasGPUBindGroupLayout,
    entries: *const CanvasBindGroupEntry,
    size: usize,
) -> *const CanvasGPUBindGroup {
    if device.is_null() || layout.is_null() || entries.is_null() || size == 0 {
        return std::ptr::null();
    }

    let device = unsafe { &*device };

    let entries = std::slice::from_raw_parts(entries, size);

    let label = ptr_into_label(label);

    let layout = &*layout;

    device.create_bind_group(label, layout, entries)
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_create_command_encoder(
    device: *const CanvasGPUDevice,
    label: *const c_char,
) -> *const CanvasGPUCommandEncoder {
    if device.is_null() {
        return std::ptr::null_mut();
    }
    let label = ptr_into_label(label);

    let device = unsafe { &*device };
    let desc = wgt::CommandEncoderDescriptor { label };

    let device_id = device.device;
    let global = &device.instance.global();

    let (encoder, error) =
        gfx_select!(device_id => global.device_create_command_encoder(device_id, &desc, None));
    let error_sink = device.error_sink.as_ref();
    if let Some(cause) = error {
        handle_error(
            global,
            error_sink,
            cause,
            "label",
            desc.label,
            "canvas_native_webgpu_device_create_command_encoder",
        );
    }

    let encoder = CanvasGPUCommandEncoder {
        instance: device.instance.clone(),
        encoder,
        error_sink: device.error_sink.clone(),
        open: std::sync::atomic::AtomicBool::new(true),
    };

    Arc::into_raw(Arc::new(encoder))
}

#[repr(C)]
pub struct CanvasProgrammableStage {
    pub module: *const CanvasGPUShaderModule,
    pub entry_point: *const c_char,
    pub constants: *const CanvasConstants,
}

struct ProgrammableStage {
    pub module: CanvasGPUShaderModule,
    pub entry_point: Option<Cow<'static, str>>,
    pub constants: CanvasConstants,
}

unsafe fn parse_compute_pipeline_descriptor(label: *const c_char,
                                            layout: CanvasGPUPipelineLayoutOrGPUAutoLayoutMode,
                                            compute: *const CanvasProgrammableStage) -> (Option<Cow<'static, str>>, Option<PipelineLayoutId>, ProgrammableStage) {
    let label = ptr_into_string(label);

    let pipeline_layout = match layout {
        CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Layout(layout) => {
            let layout = &*layout;
            Some(layout.layout)
        }
        CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Auto(CanvasGPUAutoLayoutMode::Auto) => None,
    };

    let compute = &*compute;
    let module = &*compute.module;

    let stage = ProgrammableStage {
        module: CanvasGPUShaderModule { instance: Arc::clone(&module.instance), module: module.module },
        entry_point: ptr_into_string(compute.entry_point),
        constants: if !compute.constants.is_null() {
            let constants = &*compute.constants;
            constants.clone()
        } else {
            CanvasConstants::default()
        },
    };

    (label, pipeline_layout, stage)
}

unsafe fn create_compute_pipeline(
    device: *const CanvasGPUDevice,
    descriptor: wgpu_core::pipeline::ComputePipelineDescriptor,
) -> (CanvasGPUComputePipeline, Option<wgpu_core::pipeline::CreateComputePipelineError>) {
    assert!(!device.is_null());

    let device = &*device;
    let device_id = device.device;

    let global = device.instance.global();

    let (pipeline, error) = gfx_select!(device_id => global.device_create_compute_pipeline(device_id, &descriptor, None, None));

    let pipeline = CanvasGPUComputePipeline {
        instance: device.instance.clone(),
        pipeline,
        error_sink: device.error_sink.clone(),
    };

    (pipeline, error)
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_create_compute_pipeline(
    device: *const CanvasGPUDevice,
    label: *const c_char,
    layout: CanvasGPUPipelineLayoutOrGPUAutoLayoutMode,
    compute: *const CanvasProgrammableStage,
) -> *const CanvasGPUComputePipeline {
    assert!(!compute.is_null());
    let (label, layout, stage) = parse_compute_pipeline_descriptor(label, layout, compute);

    let descriptor = wgpu_core::pipeline::ComputePipelineDescriptor {
        label: label.clone(),
        layout,
        stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
            module: stage.module.module,
            entry_point: stage.entry_point,
            constants: Cow::Borrowed(&stage.constants.0),
            zero_initialize_workgroup_memory: true,
            vertex_pulling_transform: false,
        },
        cache: None,
    };

    let (pipeline, error) = create_compute_pipeline(device, descriptor);


    let global = pipeline.instance.global();
    let error_sink = pipeline.error_sink.as_ref();

    if let Some(cause) = error {
        if let wgpu_core::pipeline::CreateComputePipelineError::Internal(ref error) = cause {
            #[cfg(target_os = "android")]
            log::warn!(
                "Shader translation error for stage {:?}: {}",
                wgt::ShaderStages::COMPUTE,
                error
            );
            #[cfg(target_os = "android")]
            log::warn!("Please report it to https://github.com/gfx-rs/wgpu");


            #[cfg(not(target_os = "android"))]
            println!(
                "Shader translation error for stage {:?}: {}",
                wgt::ShaderStages::COMPUTE,
                error
            );
            #[cfg(not(target_os = "android"))]
            println!("Please report it to https://github.com/gfx-rs/wgpu");
        }

        handle_error(
            global,
            error_sink,
            cause,
            "label",
            label,
            "canvas_native_webgpu_device_create_compute_pipeline",
        );
    }

    Arc::into_raw(Arc::new(pipeline))
}

// todo replace with create_compute_pipeline_async once implemented in wgpu-core
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_create_compute_pipeline_async(
    device: *const CanvasGPUDevice,
    label: *const c_char,
    layout: CanvasGPUPipelineLayoutOrGPUAutoLayoutMode,
    compute: *const CanvasProgrammableStage,
    callback: unsafe extern "C" fn(
        *const CanvasGPUComputePipeline,
        CanvasGPUErrorType,
        *mut c_char,
        *mut c_void,
    ),
    callback_data: *mut c_void,
) {
    if device.is_null() || compute.is_null() {
        let error = CString::new("invalid device").unwrap();
        callback(
            std::ptr::null(),
            CanvasGPUErrorType::Internal,
            error.into_raw(),
            callback_data,
        );
        return;
    }

    let callback_data = callback_data as i64;
    let device = device as i64;
    let (label, layout, stage) = parse_compute_pipeline_descriptor(label, layout, compute);

    std::thread::scope(|s| {
        s.spawn(move || {
            let device = device as *const CanvasGPUDevice;

            let descriptor = wgpu_core::pipeline::ComputePipelineDescriptor {
                label: label.clone(),
                layout,
                stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                    module: stage.module.module,
                    entry_point: stage.entry_point,
                    constants: Cow::Borrowed(&stage.constants.0),
                    zero_initialize_workgroup_memory: true,
                    vertex_pulling_transform: false,
                },
                cache: None,
            };


            let (pipeline, error) = create_compute_pipeline(device, descriptor);

            let callback = unsafe {
                std::mem::transmute::<
                    *const i64,
                    extern "C" fn(
                        *const CanvasGPUComputePipeline,
                        CanvasGPUErrorType,
                        *mut c_char,
                        *mut c_void,
                    ),
                >(callback as _)
            };

            let callback_data = callback_data as *mut c_void;

            if let Some(error) = error {
                let error: CanvasGPUError = error.into();
                let error_type = match &error {
                    CanvasGPUError::Lost => super::error::CanvasGPUErrorType::Lost,
                    CanvasGPUError::OutOfMemory => super::error::CanvasGPUErrorType::OutOfMemory,
                    CanvasGPUError::Validation(_) => super::error::CanvasGPUErrorType::Validation,
                    CanvasGPUError::Internal => super::error::CanvasGPUErrorType::Internal,
                    CanvasGPUError::None => CanvasGPUErrorType::None,
                };

                let error_value = match error {
                    CanvasGPUError::Lost => std::ptr::null_mut(),
                    CanvasGPUError::OutOfMemory => std::ptr::null_mut(),
                    CanvasGPUError::Validation(value) => CString::new(value).unwrap().into_raw(),
                    CanvasGPUError::Internal => std::ptr::null_mut(),
                    CanvasGPUError::None => std::ptr::null_mut(),
                };
                callback(std::ptr::null(), error_type, error_value, callback_data);
            } else {
                let ret = Arc::into_raw(Arc::new(pipeline));

                callback(
                    ret,
                    CanvasGPUErrorType::None,
                    std::ptr::null_mut(),
                    callback_data,
                );
            }
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_create_pipeline_layout(
    device: *const CanvasGPUDevice,
    label: *const c_char,
    group_layouts: *const *const CanvasGPUBindGroupLayout,
    size: usize,
) -> *const CanvasGPUPipelineLayout {
    if device.is_null() || group_layouts.is_null() || size == 0 {
        return std::ptr::null_mut();
    }
    let label = ptr_into_label(label);

    let device = &*device;

    let global = device.instance.global();

    let device_id = device.device;

    let group_layouts = std::slice::from_raw_parts(group_layouts, size);

    let group_layouts = group_layouts
        .iter()
        .map(|group| {
            let group = &**group;
            group.group_layout
        })
        .collect::<Vec<_>>();

    let desc = wgpu_core::binding_model::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: Cow::Owned(group_layouts),
        push_constant_ranges: Default::default(),
    };

    let (pipeline_layout, error) =
        gfx_select!(device_id => global.device_create_pipeline_layout(device_id, &desc, None));

    let error_sink = device.error_sink.as_ref();
    if let Some(cause) = error {
        handle_error(
            global,
            error_sink,
            cause,
            "label",
            desc.label,
            "canvas_native_webgpu_device_create_pipeline_layout",
        );
    }

    Arc::into_raw(Arc::new(CanvasGPUPipelineLayout {
        instance: device.instance.clone(),
        layout: pipeline_layout,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_create_query_set(
    device: *const CanvasGPUDevice,
    label: *const c_char,
    type_: CanvasQueryType,
    count: u32,
) -> *const CanvasGPUQuerySet {
    if device.is_null() {
        return std::ptr::null_mut();
    }
    let label = ptr_into_label(label);

    let device = &*device;

    let global = device.instance.global();

    let device_id = device.device;

    let desc = wgt::QuerySetDescriptor {
        label: label.clone(),
        ty: type_.into(),
        count,
    };

    let (query, error) =
        gfx_select!(device_id => global.device_create_query_set(device_id, &desc, None));

    let error_sink = device.error_sink.as_ref();
    if let Some(cause) = error {
        handle_error(
            global,
            error_sink,
            cause,
            "label",
            desc.label,
            "canvas_native_webgpu_device_create_query_set",
        );
    }

    Arc::into_raw(Arc::new(CanvasGPUQuerySet {
        instance: device.instance.clone(),
        query,
        count,
        type_,
        label,
    }))
}

#[repr(C)]
pub struct CanvasCreateRenderBundleEncoderDescriptor {
    label: *const c_char,
    color_formats: *const CanvasGPUTextureFormat,
    color_formats_size: usize,
    depth_stencil_format: CanvasOptionalGPUTextureFormat,
    sample_count: u32,
    depth_read_only: bool,
    stencil_read_only: bool,
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_create_render_bundle_encoder(
    device: *const CanvasGPUDevice,
    descriptor: *const CanvasCreateRenderBundleEncoderDescriptor,
) -> *const CanvasGPURenderBundleEncoder {
    if device.is_null() || descriptor.is_null() {
        return std::ptr::null_mut();
    }

    let descriptor = &*descriptor;

    let label = ptr_into_label(descriptor.label);

    let device = &*device;

    let global = device.instance.global();

    let device_id = device.device;

    let depth_stencil = match descriptor.depth_stencil_format {
        CanvasOptionalGPUTextureFormat::None => None,
        CanvasOptionalGPUTextureFormat::Some(format) => {
            Some(wgt::RenderBundleDepthStencil {
                format: format.into(),
                depth_read_only: descriptor.depth_read_only,
                stencil_read_only: descriptor.stencil_read_only,
            })
        }
    };

    let color_formats = if !descriptor.color_formats.is_null() && descriptor.color_formats_size > 0
    {
        unsafe {
            std::slice::from_raw_parts(descriptor.color_formats, descriptor.color_formats_size)
                .to_vec()
                .into_iter()
                .map(|v| Some(v.into()))
                .collect::<Vec<_>>()
        }
    } else {
        vec![]
    };

    let desc = wgpu_core::command::RenderBundleEncoderDescriptor {
        label,
        color_formats: Cow::Owned(color_formats),
        depth_stencil,
        sample_count: descriptor.sample_count,
        multiview: None,
    };

    let bundle = RenderBundleEncoder::new(&desc, device_id, None);

    match bundle {
        Ok(bundle) => Arc::into_raw(Arc::new(CanvasGPURenderBundleEncoder {
            instance: device.instance.clone(),
            encoder: Box::into_raw(Box::new(Some(bundle))),
        })),
        Err(cause) => {
            handle_error_fatal(
                global,
                cause,
                "canvas_native_webgpu_device_create_render_bundle_encoder",
            );
            Arc::into_raw(Arc::new(CanvasGPURenderBundleEncoder {
                instance: device.instance.clone(),
                encoder: Box::into_raw(Box::new(Some(RenderBundleEncoder::dummy(device_id)))),
            }))
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_create_shader_module(
    device: *const CanvasGPUDevice,
    label: *const c_char,
    source: *const c_char,
) -> *const CanvasGPUShaderModule {
    if device.is_null() {
        return std::ptr::null_mut();
    }
    let label = ptr_into_label(label);

    let src = unsafe { CStr::from_ptr(source) };
    let src = Cow::Borrowed(src.to_str().unwrap());
    let source = wgpu_core::pipeline::ShaderModuleSource::Wgsl(src);

    let device = unsafe { &*device };
    let desc = wgpu_core::pipeline::ShaderModuleDescriptor {
        label,
        shader_bound_checks: wgt::ShaderBoundChecks::default(),
    };

    let device_id = device.device;
    let global = &device.instance.global();

    let (module, error) = gfx_select!(device_id => global.device_create_shader_module(device_id, &desc, source, None));

    if let Some(cause) = error {
        handle_error(
            global,
            device.error_sink.as_ref(),
            cause,
            "label",
            desc.label,
            "canvas_native_webgpu_device_create_shader_module",
        );
    }

    let shader = CanvasGPUShaderModule {
        module,
        instance: device.instance.clone(),
    };
    Arc::into_raw(Arc::new(shader))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_create_buffer(
    device: *const CanvasGPUDevice,
    label: *const c_char,
    size: u64,
    usage: u32,
    mapped_at_creation: bool,
) -> *const CanvasGPUBuffer {
    if device.is_null() {
        return std::ptr::null_mut();
    }

    let device = unsafe { &*device };

    device.create_buffer(label, size, usage, mapped_at_creation)
}

#[derive(Clone, Debug, Default)]
pub struct CanvasConstants(HashMap<String, f64>);

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_constants_create() -> *mut CanvasConstants {
    Box::into_raw(Box::new(CanvasConstants(HashMap::new())))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_constants_insert(
    constants: *mut CanvasConstants,
    key: *const c_char,
    value: f64,
) {
    if constants.is_null() {
        return;
    }

    let constants = &mut *constants;
    let key = CStr::from_ptr(key);
    let key = key.to_string_lossy();
    let key = key.to_string();
    constants.0.insert(key, value);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_constants_destroy(constants: *mut CanvasConstants) {
    if constants.is_null() {
        return;
    }

    let _ = Box::from_raw(constants);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CanvasDepthStencilState {
    format: CanvasGPUTextureFormat,
    depth_write_enabled: bool,
    depth_compare: CanvasCompareFunction,
    stencil_front: CanvasStencilFaceState,
    stencil_back: CanvasStencilFaceState,
    stencil_read_mask: u32,
    stencil_write_mask: u32,
    depth_bias: i32,
    depth_bias_slope_scale: f32,
    depth_bias_clamp: f32,
}

impl From<wgt::DepthStencilState> for CanvasDepthStencilState {
    fn from(value: wgt::DepthStencilState) -> Self {
        Self {
            format: value.format.into(),
            depth_write_enabled: value.depth_write_enabled,
            depth_compare: value.depth_compare.into(),
            stencil_front: value.stencil.front.into(),
            stencil_back: value.stencil.back.into(),
            stencil_read_mask: value.stencil.read_mask,
            stencil_write_mask: value.stencil.write_mask,
            depth_bias: value.bias.constant,
            depth_bias_slope_scale: value.bias.slope_scale,
            depth_bias_clamp: value.bias.clamp,
        }
    }
}

impl Into<wgt::DepthStencilState> for CanvasDepthStencilState {
    fn into(self) -> wgt::DepthStencilState {
        wgt::DepthStencilState {
            format: self.format.into(),
            depth_write_enabled: self.depth_write_enabled,
            depth_compare: self.depth_compare.into(),
            stencil: wgt::StencilState {
                front: self.stencil_front.into(),
                back: self.stencil_back.into(),
                read_mask: self.stencil_read_mask,
                write_mask: self.stencil_write_mask,
            },
            bias: wgt::DepthBiasState {
                constant: self.depth_bias,
                slope_scale: self.depth_bias_slope_scale,
                clamp: self.depth_bias_clamp,
            },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct CanvasPrimitiveState {
    pub topology: CanvasPrimitiveTopology,
    pub strip_index_format: CanvasOptionalIndexFormat,
    pub front_face: CanvasFrontFace,
    pub cull_mode: CanvasCullMode,
    pub unclipped_depth: bool,
}

impl From<wgt::PrimitiveState> for CanvasPrimitiveState {
    fn from(value: wgt::PrimitiveState) -> Self {
        Self {
            topology: value.topology.into(),
            strip_index_format: value.strip_index_format.into(),
            front_face: value.front_face.into(),
            cull_mode: value.cull_mode.into(),
            unclipped_depth: value.unclipped_depth,
        }
    }
}

impl Into<wgt::PrimitiveState> for CanvasPrimitiveState {
    fn into(self) -> wgt::PrimitiveState {
        wgt::PrimitiveState {
            topology: self.topology.into(),
            strip_index_format: self.strip_index_format.into(),
            front_face: self.front_face.into(),
            cull_mode: self.cull_mode.into(),
            unclipped_depth: self.unclipped_depth,
            polygon_mode: Default::default(), // native
            conservative: false,              // native
        }
    }
}

#[repr(C)]
pub struct CanvasVertexBufferLayout {
    pub array_stride: u64,
    pub step_mode: CanvasVertexStepMode,
    pub attributes: *const CanvasVertexAttribute,
    pub attributes_size: usize,
}

#[repr(C)]
pub struct CanvasVertexState {
    pub module: *const CanvasGPUShaderModule,
    pub entry_point: *const c_char,
    pub constants: *const CanvasConstants,
    pub buffers: *const CanvasVertexBufferLayout,
    pub buffers_size: usize,
}

#[repr(C)]
pub struct CanvasFragmentState {
    pub targets: *const CanvasColorTargetState,
    pub targets_size: usize,
    pub module: *const CanvasGPUShaderModule,
    pub entry_point: *const c_char,
    pub constants: *const CanvasConstants,
}

#[repr(C)]
pub enum CanvasGPUAutoLayoutMode {
    Auto,
}

#[repr(C)]
pub enum CanvasGPUPipelineLayoutOrGPUAutoLayoutMode {
    Layout(*const CanvasGPUPipelineLayout),
    Auto(CanvasGPUAutoLayoutMode),
}

#[repr(C)]
pub struct CanvasCreateRenderPipelineDescriptor {
    pub label: *const c_char,
    pub layout: CanvasGPUPipelineLayoutOrGPUAutoLayoutMode,
    pub vertex: *const CanvasVertexState,
    pub primitive: *const CanvasPrimitiveState,
    pub depth_stencil: *const CanvasDepthStencilState,
    pub multisample: *const CanvasMultisampleState,
    pub fragment: *const CanvasFragmentState,
}

unsafe fn parse_render_pipeline_descriptor<'a>(
    descriptor: *const CanvasCreateRenderPipelineDescriptor
) -> RenderPipelineDescriptor<'a> {
    let descriptor = &*descriptor;

    let label = ptr_into_label(descriptor.label);

    let layout = match descriptor.layout {
        CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Layout(value) => {
            if value.is_null() {
                None
            } else {
                let value = &*value;
                Some(value.layout)
            }
        }
        CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Auto(CanvasGPUAutoLayoutMode::Auto) => None,
    };

    let vertex = &*descriptor.vertex;

    let vertex_shader_module = &*vertex.module;

    let vertex_shader_module_id = vertex_shader_module.module;

    let fragment = if !descriptor.fragment.is_null() {
        let frag = &*descriptor.fragment;
        let module = &*frag.module;
        let module_id = module.module;

        let entry_point = if frag.entry_point.is_null() {
            None
        } else {
            Some(CStr::from_ptr(frag.entry_point).to_string_lossy())
        };

        let targets = if !frag.targets.is_null() {
            std::slice::from_raw_parts(frag.targets, frag.targets_size)
                .iter()
                .map(|target| Some((*target).into()))
                .collect::<Vec<Option<wgt::ColorTargetState>>>()
        } else {
            vec![]
        };

        if frag.constants.is_null() {
            let constants = HashMap::default();
            Some(wgpu_core::pipeline::FragmentState {
                stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                    module: module_id,
                    entry_point,
                    constants: Cow::Owned(constants),
                    // Required to be true for WebGPU
                    zero_initialize_workgroup_memory: true,
                    vertex_pulling_transform: false,
                },
                targets: Cow::Owned(targets),
            })
        } else {
            let constants = &*frag.constants;
            let constants = Cow::Borrowed(&constants.0);
            Some(wgpu_core::pipeline::FragmentState {
                stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                    module: module_id,
                    entry_point,
                    constants,
                    // Required to be true for WebGPU
                    zero_initialize_workgroup_memory: true,
                    vertex_pulling_transform: false,
                },
                targets: Cow::Owned(targets),
            })
        }
    } else {
        None
    };
    let primitive: wgt::PrimitiveState = if !descriptor.primitive.is_null() {
        let primitive = *descriptor.primitive;
        primitive.into()
    } else {
        wgt::PrimitiveState::default()
    };

    let depth_stencil = if !descriptor.depth_stencil.is_null() {
        let depth_stencil = *descriptor.depth_stencil;
        let depth_stencil: wgt::DepthStencilState = depth_stencil.into();
        Some(depth_stencil)
    } else {
        None
    };

    let multisample: wgt::MultisampleState = if !descriptor.multisample.is_null() {
        let multisample = *descriptor.multisample;
        multisample.into()
    } else {
        wgt::MultisampleState::default()
    };

    let vertex_buffers = if !vertex.buffers.is_null() {
        let buffers = std::slice::from_raw_parts(vertex.buffers, vertex.buffers_size);
        buffers
            .iter()
            .map(|layout| {
                let attributes = if !layout.attributes.is_null() {
                    std::slice::from_raw_parts(layout.attributes, layout.attributes_size)
                        .iter()
                        .map(|attr| (*attr).into())
                        .collect::<Vec<wgt::VertexAttribute>>()
                } else {
                    vec![]
                };
                wgpu_core::pipeline::VertexBufferLayout {
                    array_stride: layout.array_stride,
                    attributes: Cow::Owned(attributes),
                    step_mode: layout.step_mode.into(),
                }
            })
            .collect::<Vec<wgpu_core::pipeline::VertexBufferLayout>>()
    } else {
        vec![]
    };

    let entry_point = ptr_into_label(vertex.entry_point);

    let constants = if !vertex.constants.is_null() {
        let constants = &*vertex.constants;
        Cow::Borrowed(&constants.0)
    } else {
        Cow::Owned(HashMap::default())
    };

    let vertex = wgpu_core::pipeline::VertexState {
        stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
            module: vertex_shader_module_id,
            entry_point,
            constants,
            // Required to be true for WebGPU
            zero_initialize_workgroup_memory: true,
            vertex_pulling_transform: false,
        },
        buffers: Cow::Owned(vertex_buffers),
    };

    RenderPipelineDescriptor {
        label: label.clone(),
        layout,
        vertex,
        primitive,
        depth_stencil,
        multisample,
        fragment,
        multiview: None,
        cache: None,
    }
}

unsafe fn create_render_pipeline(
    global: Arc<CanvasWebGPUInstance>,
    device_id: wgpu_core::id::DeviceId,
    descriptor: RenderPipelineDescriptor,
) -> (wgpu_core::id::RenderPipelineId, Option<CreateRenderPipelineError>) {
    let global = global.global();
    gfx_select!(device_id => global.device_create_render_pipeline(device_id, &descriptor,None, None))
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_create_render_pipeline(
    device: *const CanvasGPUDevice,
    descriptor: *const CanvasCreateRenderPipelineDescriptor,
) -> *const CanvasGPURenderPipeline {
    assert!(!device.is_null() && !descriptor.is_null());
    let descriptor = parse_render_pipeline_descriptor(descriptor);

    let label = descriptor.label.clone();

    let device = &*device;

    let (pipeline, error) = create_render_pipeline(device.instance.clone(), device.device, descriptor);

    let error_sink = device.error_sink.as_ref();
    let global = device.instance.global();

    if let Some(cause) = error {
        if let wgpu_core::pipeline::CreateRenderPipelineError::Internal { stage, ref error } = cause
        {
            #[cfg(target_os = "android")]
            log::error!("Shader translation error for stage {:?}: {}", stage, error);
            #[cfg(target_os = "android")]
            log::error!("Please report it to https://github.com/gfx-rs/wgpu");

            #[cfg(not(target_os = "android"))]
            println!("Shader translation error for stage {:?}: {}", stage, error);
            #[cfg(not(target_os = "android"))]
            println!("Please report it to https://github.com/gfx-rs/wgpu");
        }
        handle_error(
            global,
            error_sink,
            cause,
            "label",
            label.clone(),
            "canvas_native_webgpu_device_create_render_pipeline",
        );
    }

    Arc::into_raw(Arc::new(CanvasGPURenderPipeline {
        label,
        instance: device.instance.clone(),
        pipeline,
        error_sink: device.error_sink.clone(),
    }
    ))
}


// todo replace with create_render_pipeline_async once implemented in wgpu-core
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_create_render_pipeline_async(
    device: *const CanvasGPUDevice,
    descriptor: *const CanvasCreateRenderPipelineDescriptor,
    callback: unsafe extern "C" fn(
        *const CanvasGPURenderPipeline,
        CanvasGPUErrorType,
        *mut c_char,
        *mut c_void,
    ),
    callback_data: *mut c_void,
) {
    if device.is_null() || descriptor.is_null() {
        let error = CString::new("invalid device").unwrap();
        callback(
            std::ptr::null(),
            CanvasGPUErrorType::Internal,
            error.into_raw(),
            callback_data,
        );
        return;
    }

    let callback_data = callback_data as i64;
    let descriptor = parse_render_pipeline_descriptor(descriptor);
    let device = &*device;
    let device_id = device.device;
    let instance = Arc::clone(&device.instance);
    let error_sink = Arc::clone(&device.error_sink);
    let label = descriptor.label.as_ref().map(|value|{
        Cow::Owned(value.to_string())
    });
    std::thread::spawn(move || {
        let (pipeline, error) = create_render_pipeline(instance.clone(), device_id, descriptor);

        let callback = unsafe {
            std::mem::transmute::<
                *const i64,
                extern "C" fn(
                    *const CanvasGPURenderPipeline,
                    CanvasGPUErrorType,
                    *mut c_char,
                    *mut c_void,
                ),
            >(callback as _)
        };

        let callback_data = callback_data as *mut c_void;

        if let Some(error) = error {
            let error: CanvasGPUError = error.into();
            let error_type = match &error {
                CanvasGPUError::Lost => super::error::CanvasGPUErrorType::Lost,
                CanvasGPUError::OutOfMemory => super::error::CanvasGPUErrorType::OutOfMemory,
                CanvasGPUError::Validation(_) => super::error::CanvasGPUErrorType::Validation,
                CanvasGPUError::Internal => super::error::CanvasGPUErrorType::Internal,
                CanvasGPUError::None => CanvasGPUErrorType::None,
            };

            let error_value = match error {
                CanvasGPUError::Lost => std::ptr::null_mut(),
                CanvasGPUError::OutOfMemory => std::ptr::null_mut(),
                CanvasGPUError::Validation(value) => CString::new(value).unwrap().into_raw(),
                CanvasGPUError::Internal => std::ptr::null_mut(),
                CanvasGPUError::None => std::ptr::null_mut(),
            };
            callback(std::ptr::null(), error_type, error_value, callback_data);
        } else {
            let pipeline = CanvasGPURenderPipeline {
                label,
                instance,
                pipeline,
                error_sink,
            };
            let ret = Arc::into_raw(Arc::new(pipeline));
            callback(
                ret,
                CanvasGPUErrorType::None,
                std::ptr::null_mut(),
                callback_data,
            );
        }
    });
}

#[repr(C)]
pub struct CanvasCreateTextureDescriptor {
    pub label: *const c_char,
    pub dimension: CanvasTextureDimension,
    pub format: CanvasGPUTextureFormat,
    pub mipLevelCount: u32,
    pub sampleCount: u32,
    pub width: u32,
    pub height: u32,
    pub depthOrArrayLayers: u32,
    pub usage: u32,
    pub view_formats: *const CanvasGPUTextureFormat,
    pub view_formats_size: usize,
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_create_texture(
    device: *const CanvasGPUDevice,
    descriptor: *const CanvasCreateTextureDescriptor,
) -> *const CanvasGPUTexture {
    if device.is_null() || descriptor.is_null() {
        return std::ptr::null_mut();
    }

    let device = unsafe { &*device };
    let descriptor = unsafe { &*descriptor };
    let device_id = device.device;

    let global = device.instance.global();

    let label = ptr_into_label(descriptor.label);

    let view_formats = if !descriptor.view_formats.is_null() && descriptor.view_formats_size > 0 {
        unsafe {
            std::slice::from_raw_parts(descriptor.view_formats, descriptor.view_formats_size)
                .to_vec()
                .into_iter()
                .map(|v| v.into())
                .collect::<Vec<wgt::TextureFormat>>()
        }
    } else {
        vec![]
    };

    let desc = wgpu_core::resource::TextureDescriptor {
        label,
        format: descriptor.format.into(),
        size: wgt::Extent3d {
            width: descriptor.width,
            height: descriptor.height,
            depth_or_array_layers: descriptor.depthOrArrayLayers,
        },
        mip_level_count: descriptor.mipLevelCount,
        sample_count: descriptor.sampleCount,
        dimension: descriptor.dimension.into(),
        usage: wgt::TextureUsages::from_bits_truncate(descriptor.usage),
        view_formats,
    };

    let (texture_id, err) =
        gfx_select!(device_id => global.device_create_texture(device_id, &desc, None));

    if let Some(cause) = err {
        handle_error(
            global,
            device.error_sink.as_ref(),
            cause,
            "",
            desc.label,
            "canvas_native_webgpu_device_create_texture",
        );
    }

    Arc::into_raw(Arc::new(CanvasGPUTexture {
        instance: device.instance.clone(),
        texture: texture_id,
        surface_id: None,
        owned: true,
        depth_or_array_layers: descriptor.depthOrArrayLayers,
        dimension: descriptor.dimension,
        format: descriptor.format,
        mipLevelCount: descriptor.mipLevelCount,
        sampleCount: descriptor.sampleCount,
        width: descriptor.width,
        height: descriptor.height,
        usage: descriptor.usage,
        error_sink: device.error_sink.clone(),
        has_surface_presented: Arc::default(),
    }))
}

#[repr(C)]
pub struct CanvasCreateSamplerDescriptor {
    pub label: *const c_char,
    pub address_mode_u: CanvasAddressMode,
    pub address_mode_v: CanvasAddressMode,
    pub address_mode_w: CanvasAddressMode,
    pub mag_filter: CanvasFilterMode,
    pub min_filter: CanvasFilterMode,
    pub mipmap_filter: CanvasFilterMode, // TODO: GPUMipmapFilterMode
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: CanvasOptionalCompareFunction,
    pub max_anisotropy: u16,
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_create_sampler(
    device: *const CanvasGPUDevice,
    descriptor: *const CanvasCreateSamplerDescriptor,
) -> *const CanvasGPUSampler {
    if device.is_null() {
        return std::ptr::null_mut();
    }

    let device = unsafe { &*device };
    let device_id = device.device;

    let global = device.instance.global();

    let mut label: wgpu_core::Label = None;

    let desc = if !descriptor.is_null() {
        let descriptor = unsafe { &*descriptor };

        label = ptr_into_label(descriptor.label);

        wgpu_core::resource::SamplerDescriptor {
            label: label.clone(),
            address_modes: [
                descriptor.address_mode_u.into(),
                descriptor.address_mode_v.into(),
                descriptor.address_mode_w.into(),
            ],
            mag_filter: descriptor.mag_filter.into(),
            min_filter: descriptor.min_filter.into(),
            mipmap_filter: descriptor.mipmap_filter.into(),
            lod_min_clamp: descriptor.lod_min_clamp,
            lod_max_clamp: descriptor.lod_max_clamp,
            compare: descriptor.compare.into(),
            anisotropy_clamp: descriptor.max_anisotropy,
            border_color: None, // native-only
        }
    } else {
        wgpu_core::resource::SamplerDescriptor {
            label: None,
            address_modes: [
                wgt::AddressMode::ClampToEdge,
                wgt::AddressMode::ClampToEdge,
                wgt::AddressMode::ClampToEdge,
            ],
            mag_filter: wgt::FilterMode::Nearest,
            min_filter: wgt::FilterMode::Nearest,
            mipmap_filter: wgt::FilterMode::Nearest,
            lod_min_clamp: 0f32,
            lod_max_clamp: 32f32,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        }
    };

    let (sampler_id, error) =
        gfx_select!(device_id => global.device_create_sampler(device_id, &desc,None));

    let error_sink = device.error_sink.as_ref();
    if let Some(cause) = error {
        handle_error(
            global,
            error_sink,
            cause,
            "label",
            desc.label,
            "canvas_native_webgpu_device_create_sampler",
        );
    }

    Arc::into_raw(Arc::new(CanvasGPUSampler {
        instance: device.instance.clone(),
        sampler: sampler_id,
        label,
    }))
}
