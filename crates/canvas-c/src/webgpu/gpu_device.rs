use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
};

use crate::buffers::StringBuffer;

use super::{
    enums::{
        CanvasCompareFunction, CanvasCullMode, CanvasFrontFace, CanvasGPUTextureFormat,
        CanvasIndexFormat, CanvasOptionalFrontFace, CanvasOptionalIndexFormat,
        CanvasOptionalPrimitiveTopology, CanvasPrimitiveTopology, CanvasStencilFaceState,
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
        println!("canvas_native_webgpu_device_create_command_encoder: error {:?}", error.to_string());
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

impl From<wgpu_types::DepthStencilState> for CanvasDepthStencilState {
    fn from(value: wgpu_types::DepthStencilState) -> Self {
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

impl Into<wgpu_types::DepthStencilState> for CanvasDepthStencilState {
    fn into(self) -> wgpu_types::DepthStencilState {
        wgpu_types::DepthStencilState {
            format: self.format.into(),
            depth_write_enabled: self.depth_write_enabled,
            depth_compare: self.depth_compare.into(),
            stencil: wgpu_types::StencilState {
                front: self.stencil_front.into(),
                back: self.stencil_back.into(),
                read_mask: self.stencil_read_mask,
                write_mask: self.stencil_write_mask,
            },
            bias: wgpu_types::DepthBiasState {
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

impl From<wgpu_types::PrimitiveState> for CanvasPrimitiveState {
    fn from(value: wgpu_types::PrimitiveState) -> Self {
        Self {
            topology: value.topology.into(),
            strip_index_format: value.strip_index_format.into(),
            front_face: value.front_face.into(),
            cull_mode: value.cull_mode.into(),
            unclipped_depth: value.unclipped_depth,
        }
    }
}

impl Into<wgpu_types::PrimitiveState> for CanvasPrimitiveState {
    fn into(self) -> wgpu_types::PrimitiveState {
        wgpu_types::PrimitiveState {
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
    pub  constants: *const CanvasConstants,
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
    pub  label: *const c_char,
    pub  layout: CanvasGPUPipelineLayoutOrGPUAutoLayoutMode,
    pub vertex: *const CanvasVertexState,
    pub primitive: *const CanvasPrimitiveState,
    pub depth_stencil: *const CanvasDepthStencilState,
    pub multisample: *const CanvasMultisampleState,
    pub  fragment: *const CanvasFragmentState,
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_create_render_pipeline(
    device: *const CanvasGPUDevice,
    descriptor: *const CanvasCreateRenderPipelineDescriptor,
) -> *mut CanvasGPURenderPipeline {
    if device.is_null() || descriptor.is_null() {
        return std::ptr::null_mut();
    }

    let device = unsafe { &*device };

    let device_id = device.device;

    let global = &device.instance.0;

    let descriptor = &*descriptor;

    let label = if !descriptor.label.is_null() {
        Some(unsafe { CStr::from_ptr(descriptor.label).to_string_lossy() })
    } else {
        None
    };

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
                .collect::<Vec<Option<wgpu_types::ColorTargetState>>>()
        } else {
            vec![]
        };

        if (frag.constants.is_null()) {
            let constants = HashMap::default();
            Some(wgpu_core::pipeline::FragmentState {
                stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                    module: module_id,
                    entry_point: entry_point,
                    constants: Cow::Owned(constants),
                    // Required to be true for WebGPU
                    zero_initialize_workgroup_memory: true,
                },
                targets: Cow::Owned(targets),
            })
        } else {
            let constants = &*frag.constants;
            let constants = Cow::Borrowed(&constants.0);
            Some(wgpu_core::pipeline::FragmentState {
                stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                    module: module_id,
                    entry_point: entry_point,
                    constants: constants,
                    // Required to be true for WebGPU
                    zero_initialize_workgroup_memory: true,
                },
                targets: Cow::Owned(targets),
            })
        }
    } else {
        None
    };

    let primitive: wgpu_types::PrimitiveState = if !descriptor.primitive.is_null() {
        let primitive = *descriptor.primitive;
        primitive.into()
    } else {
        wgpu_types::PrimitiveState::default()
    };

    let depth_stencil = if !descriptor.depth_stencil.is_null() {
        let depth_stencil = *descriptor.depth_stencil;
        let depth_stencil: wgpu_types::DepthStencilState = depth_stencil.into();
        Some(depth_stencil)
    } else {
        None
    };

    let multisample: wgpu_types::MultisampleState = if !descriptor.multisample.is_null() {
        let multisample = *descriptor.multisample;
        multisample.into()
    } else {
        wgpu_types::MultisampleState::default()
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
                        .collect::<Vec<wgpu_types::VertexAttribute>>()
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

    let entry_point = if vertex.entry_point.is_null() {
        None
    } else {
        Some(CStr::from_ptr(vertex.entry_point).to_string_lossy())
    };

    let constants = if !vertex.constants.is_null() {
        let constants = &*vertex.constants;
        Cow::Borrowed(&constants.0)
    } else {
        Cow::Owned(HashMap::default())
    };

    let vertex = wgpu_core::pipeline::VertexState {
        stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
            module: vertex_shader_module_id,
            entry_point: entry_point,
            constants: constants,
            // Required to be true for WebGPU
            zero_initialize_workgroup_memory: true,
        },
        buffers: Cow::Owned(vertex_buffers),
    };

    let desc = wgpu_core::pipeline::RenderPipelineDescriptor {
        label: label,
        layout: layout,
        vertex: vertex,
        primitive: primitive,
        depth_stencil: depth_stencil,
        multisample: multisample,
        fragment: fragment,
        multiview: None,
    };

    let implicit_pipelines = match descriptor.layout {
        CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Layout(_) => None,
        CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Auto(CanvasGPUAutoLayoutMode::Auto) => {
            Some(wgpu_core::device::ImplicitPipelineIds {
                root_id: None,
                group_ids: &[None; MAX_BIND_GROUPS],
            })
        }
    };

    let (pipeline, error) = gfx_select!(device_id => global.device_create_render_pipeline(device_id, &desc,None, implicit_pipelines));

    if let Some(error) = error {
        // todo handle error
        return std::ptr::null_mut();
    }

    Box::into_raw(Box::new(CanvasGPURenderPipeline {
        instance: device.instance.clone(),
        pipeline: pipeline,
    }))
}

#[repr(C)]
pub struct CanvasCreateTextureDescriptor {
    label: *const c_char,
    dimension: CanvasTextureDimension,
    format: CanvasGPUTextureFormat,
    mipLevelCount: u32,
    sampleCount: u32,
    width: u32,
    height: u32,
    depthOrArrayLayers: u32,
    usage: u32,
    view_formats: *const CanvasGPUTextureFormat,
    view_formats_size: usize,
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_create_texture(
    device: *const CanvasGPUDevice,
    descriptor: *const CanvasCreateTextureDescriptor,
    mut error: *mut c_char,
) -> *mut CanvasGPUTexture {
    if device.is_null() || descriptor.is_null() {
        return std::ptr::null_mut();
    }

    let device = unsafe { &*device };
    let descriptor = unsafe { &*descriptor };
    let device_id = device.device;

    let global = &device.instance.0;

    let label = if !descriptor.label.is_null() {
        Some(unsafe { CStr::from_ptr(descriptor.label).to_string_lossy() })
    } else {
        None
    };

    let view_formats = if !descriptor.view_formats.is_null() && descriptor.view_formats_size > 0 {
        unsafe {
            std::slice::from_raw_parts(descriptor.view_formats, descriptor.view_formats_size)
                .to_vec()
                .into_iter()
                .map(|v| v.into())
                .collect::<Vec<wgpu_types::TextureFormat>>()
        }
    } else {
        vec![]
    };

    let desc = wgpu_core::resource::TextureDescriptor {
        label,
        format: descriptor.format.into(),
        size: wgpu_types::Extent3d {
            width: descriptor.width,
            height: descriptor.height,
            depth_or_array_layers: descriptor.depthOrArrayLayers,
        },
        mip_level_count: descriptor.mipLevelCount,
        sample_count: descriptor.sampleCount,
        dimension: descriptor.dimension.into(),
        usage: wgpu_types::TextureUsages::from_bits_truncate(descriptor.usage),
        view_formats,
    };

    let (texture_id, err) =
        gfx_select!(device_id => global.device_create_texture(device_id, &desc, None));

    if let Some(err) = err {
        let err = err.to_string();
        error = CString::new(err).unwrap().into_raw();
        return std::ptr::null_mut();
    }

    Box::into_raw(Box::new(CanvasGPUTexture {
        instance: device.instance.clone(),
        texture: texture_id,
        owned: true,
        depth_or_array_layers: descriptor.depthOrArrayLayers,
        dimension: descriptor.dimension,
        format: descriptor.format,
        mipLevelCount: descriptor.mipLevelCount,
        sampleCount: descriptor.sampleCount,
        width: descriptor.width,
        height: descriptor.height,
        usage: descriptor.usage,
    }))
}