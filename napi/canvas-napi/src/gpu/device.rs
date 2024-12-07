use crate::gpu::bind_group::g_p_u_bind_group;
use crate::gpu::bind_group_layout::g_p_u_bind_group_layout;
use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::command_encoder::GPUCommandEncoder;
use crate::gpu::compute_pipeline::g_p_u_compute_pipeline;
use crate::gpu::enums::{
  GPUBufferBindingType, GPUErrorFilter, GPUSamplerBindingType,
  GPUStorageTextureAccess, GPUTextureViewDimension,
};
use crate::gpu::objects::{
  GPUBindGroupDescriptor, GPUBindGroupLayoutDescriptor, GPUBufferDescriptor,
  GPUCommandEncoderDescriptor, GPUComputePipelineDescriptor, GPURenderPipelineDescriptor,
  GPUShaderModuleDescriptor, GPUSupportedLimits,
};
use crate::gpu::render_pipeline::g_p_u_render_pipeline;
use crate::gpu::shader_module::g_p_u_shader_module;
use canvas_c::webgpu::enums::{
  CanvasBindGroupEntry, CanvasBindGroupEntryResource, CanvasBindGroupLayoutEntry,
  CanvasBindingType, CanvasBufferBinding, CanvasBufferBindingLayout, CanvasSamplerBindingLayout,
  CanvasStorageTextureBindingLayout, CanvasTextureBindingLayout, CanvasTextureSampleType,
  CanvasTextureViewDimension, CanvasVertexStepMode,
};
use canvas_c::webgpu::gpu_device::{
  CanvasConstants, CanvasGPUAutoLayoutMode, CanvasGPUPipelineLayoutOrGPUAutoLayoutMode,
  CanvasProgrammableStage,
};
use canvas_c::webgpu::gpu_supported_limits::CanvasGPUSupportedLimits;
use canvas_c::webgpu::structs::CanvasVertexAttribute;
use napi::bindgen_prelude::{ClassInstance, Either3, ObjectFinalize};
use napi::*;
use napi_derive::napi;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::sync::mpsc::channel;
use std::sync::Arc;

#[napi]
pub struct g_p_u_device {
  pub(crate) device: Arc<canvas_c::webgpu::gpu_device::CanvasGPUDevice>,
}

struct CanvasGPUVertexBufferLayout {
  pub array_stride: u64,
  pub attributes: Vec<CanvasVertexAttribute>,
  pub step_mode: Option<CanvasVertexStepMode>,
}

#[napi]
impl g_p_u_device {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_get_label(Arc::as_ptr(&self.device))
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }

  // #[napi(getter)]
  pub fn get_lost(&self, env: Env) {}

  #[napi(getter)]
  pub fn get_limits(&self) -> GPUSupportedLimits {
    let limits = unsafe {
      *Box::from_raw(
        canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_get_limits(Arc::as_ptr(
          &self.device,
        )),
      )
    };

    GPUSupportedLimits::from(limits)
  }

  #[napi(getter)]
  pub fn get_features(&self) -> Vec<String> {
    // todo use hashset after updating
    unsafe {
      *Box::from_raw(
        canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_get_features(Arc::as_ptr(
          &self.device,
        )),
      )
    }
    .into()
  }

  #[napi]
  pub fn create_bind_group(&self, descriptor: GPUBindGroupDescriptor) -> g_p_u_bind_group {
    let label = descriptor.label.map(|l| CString::new(l).unwrap());
    let label_ptr = match label.as_ref() {
      None => std::ptr::null(),
      Some(l) => l.as_ptr(),
    };

    let entries = descriptor
      .entries
      .iter()
      .map(|entry| match &entry.resource {
        Either3::A(sampler) => CanvasBindGroupEntry {
          binding: entry.binding,
          resource: CanvasBindGroupEntryResource::Sampler(Arc::as_ptr(&sampler.sampler)),
        },
        Either3::B(texture_view) => CanvasBindGroupEntry {
          binding: entry.binding,
          resource: CanvasBindGroupEntryResource::TextureView(Arc::as_ptr(
            &texture_view.texture_view,
          )),
        },
        Either3::C(buffer) => CanvasBindGroupEntry {
          binding: entry.binding,
          resource: CanvasBindGroupEntryResource::Buffer(CanvasBufferBinding {
            buffer: Arc::as_ptr(&buffer.buffer.buffer),
            offset: buffer.offset.unwrap_or(-1),
            size: buffer.size.unwrap_or(-1),
          }),
        },
      })
      .collect::<Vec<_>>();

    let bind_group = unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_bind_group(
        Arc::as_ptr(&self.device),
        label_ptr,
        Arc::as_ptr(&descriptor.layout.layout),
        entries.as_ptr(),
        entries.len(),
      )
    };

    g_p_u_bind_group {
      group: unsafe { Arc::from_raw(bind_group) },
    }
  }

  #[napi]
  pub fn create_bind_group_layout(
    &self,
    descriptor: GPUBindGroupLayoutDescriptor,
  ) -> g_p_u_bind_group_layout {
    let label = descriptor.label.map(|l| CString::new(l).unwrap());
    let label_ptr = if let Some(label) = label.as_ref() {
      label.as_ptr()
    } else {
      std::ptr::null()
    };
    let mut has_external_texture = false;
    let entries = descriptor
      .entries
      .iter()
      .filter(|v| {
        if has_external_texture == v.external_texture.is_some() {
          has_external_texture = true;
        }
        v.external_texture.is_none()
      })
      .map(|entry| {
        if let Some(buffer) = &entry.buffer {
          CanvasBindGroupLayoutEntry {
            binding: entry.binding,
            visibility: entry.visibility,
            binding_type: CanvasBindingType::Buffer(CanvasBufferBindingLayout {
              type_: buffer.type_.unwrap_or(GPUBufferBindingType::uniform).into(),
              has_dynamic_offset: buffer.has_dynamic_offset.unwrap_or(false),
              min_binding_size: buffer.min_binding_size.unwrap_or(-1),
            }),
          }
        } else if let Some(texture) = &entry.texture {
          CanvasBindGroupLayoutEntry {
            binding: entry.binding,
            visibility: entry.visibility,
            binding_type: CanvasBindingType::Texture(CanvasTextureBindingLayout {
              sample_type: CanvasTextureSampleType::Float,
              view_dimension: CanvasTextureViewDimension::D2,
              multisampled: texture.multisampled.unwrap_or_default(),
            }),
          }
        } else if let Some(sampler) = &entry.sampler {
          CanvasBindGroupLayoutEntry {
            binding: entry.binding,
            visibility: entry.visibility,
            binding_type: CanvasBindingType::Sampler(CanvasSamplerBindingLayout {
              type_: sampler
                .type_
                .unwrap_or(GPUSamplerBindingType::filtering)
                .into(),
            }),
          }
        } else if let Some(storage) = &entry.storage_texture {
          CanvasBindGroupLayoutEntry {
            binding: entry.binding,
            visibility: entry.visibility,
            binding_type: CanvasBindingType::StorageTexture(CanvasStorageTextureBindingLayout {
              access: storage
                .access
                .unwrap_or(GPUStorageTextureAccess::writeOnly)
                .into(),
              format: storage.format.into(),
              view_dimension: storage
                .view_dimension
                .unwrap_or(GPUTextureViewDimension::d2)
                .into(),
            }),
          }
        } else {
          unreachable!()
        }
      })
      .collect::<Vec<_>>();

    // todo
    if has_external_texture {
      println!("GPUExternalTexture is currently unsupported");
    }
    let layout = unsafe {
      Arc::from_raw(
        canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_bind_group_layout(
          Arc::as_ptr(&self.device),
          label_ptr,
          entries.as_ptr(),
          entries.len(),
        ),
      )
    };

    g_p_u_bind_group_layout { layout }
  }

  #[napi]
  pub fn create_buffer(&self, descriptor: GPUBufferDescriptor) -> g_p_u_buffer {
    let label = descriptor.label.map(|l| CString::new(l).unwrap());
    let label_ptr = if let Some(label) = label.as_ref() {
      label.as_ptr()
    } else {
      std::ptr::null()
    };
    let buffer = unsafe {
      Arc::from_raw(
        canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_buffer(
          Arc::as_ptr(&self.device),
          label_ptr,
          descriptor.size as u64,
          descriptor.usage,
          descriptor.mapped_at_creation.unwrap_or_default(),
        ),
      )
    };
    g_p_u_buffer::new(buffer)
  }

  #[napi]
  pub fn create_command_encoder(
    &self,
    descriptor: GPUCommandEncoderDescriptor,
  ) -> GPUCommandEncoder {
    let mut label_ptr: *const c_char = std::ptr::null();
    let label = descriptor.label.map(|label| CString::new(label).unwrap());
    if let Some(label) = &label {
      label_ptr = label.as_ptr();
    }
    let encoder = unsafe {
      Arc::from_raw(
        canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_command_encoder(
          Arc::as_ptr(&self.device),
          label_ptr,
        ),
      )
    };
    GPUCommandEncoder { encoder }
  }

  #[napi]
  pub fn create_compute_pipeline(
    &self,
    descriptor: GPUComputePipelineDescriptor,
  ) -> g_p_u_compute_pipeline {
    let label = descriptor.label.map(|l| CString::new(l).unwrap());
    let label_ptr = if let Some(label) = label.as_ref() {
      label.as_ptr()
    } else {
      std::ptr::null()
    };
    let layout = match descriptor.layout {
      Either::A(layout) => {
        CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Layout(Arc::as_ptr(&layout.layout))
      }
      Either::B(_) => {
        CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Auto(CanvasGPUAutoLayoutMode::Auto)
      }
    };

    let constants = descriptor
      .compute
      .constants
      .map(|c| CanvasConstants::from(c));
    let constants_ptr = if let Some(constants) = constants.as_ref() {
      constants
    } else {
      std::ptr::null()
    };
    let entry_point = descriptor
      .compute
      .entry_point
      .map(|l| CString::new(l).unwrap());
    let entry_point_ptr = if let Some(entry_point) = entry_point.as_ref() {
      entry_point.as_ptr()
    } else {
      std::ptr::null()
    };

    let compute = CanvasProgrammableStage {
      module: Arc::as_ptr(&descriptor.compute.module.module),
      entry_point: entry_point_ptr,
      constants: constants_ptr,
    };

    let pipeline = unsafe {
      Arc::from_raw(
        canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_compute_pipeline(
          Arc::as_ptr(&self.device),
          label_ptr,
          layout,
          &compute,
        ),
      )
    };

    g_p_u_compute_pipeline { pipeline }
  }

  #[napi]
  pub fn create_render_pipeline(
    &self,
    descriptor: GPURenderPipelineDescriptor,
  ) -> g_p_u_render_pipeline {
    let label = descriptor.label.map(|s| CString::new(s).unwrap());
    let mut label_ptr = std::ptr::null();
    if let Some(label) = &label {
      label_ptr = label.as_ptr();
    }
    let layout = match descriptor.layout {
      None => canvas_c::webgpu::gpu_device::CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Auto(
        CanvasGPUAutoLayoutMode::Auto,
      ),
      Some(layout) => match layout {
        Either::A(layout) => {
          CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Layout(layout.layout.as_ref())
        }
        Either::B(_) => {
          CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Auto(CanvasGPUAutoLayoutMode::Auto)
        }
      },
    };
    let mut entry_point_ptr = std::ptr::null();
    let entry_point = descriptor
      .vertex
      .entry_point
      .map(|v| CString::new(v).unwrap());

    if let Some(entry_point) = &entry_point {
      entry_point_ptr = entry_point.as_ptr();
    }

    let mut constants_ptr = std::ptr::null();
    let constants = descriptor
      .vertex
      .constants
      .map(|c| canvas_c::webgpu::gpu_device::CanvasConstants::from(c));

    if let Some(constants) = &constants {
      constants_ptr = constants;
    }

    let mut buffers_ptr = std::ptr::null();
    let mut buffers_size = 0;
    let buffers = descriptor.vertex.buffers.map(|c| {
      c.into_iter()
        .map(|v| CanvasGPUVertexBufferLayout {
          array_stride: v.array_stride as _,
          attributes: v
            .attributes
            .into_iter()
            .map(|v| CanvasVertexAttribute {
              format: v.format.into(),
              offset: v.offset as u64,
              shader_location: v.shader_location,
            })
            .collect::<Vec<CanvasVertexAttribute>>(),
          step_mode: match v.step_mode {
            None => None,
            Some(value) => Some(value.into()),
          },
        })
        .collect::<Vec<_>>()
    });

    let buffers = buffers.map(|v| {
      v.into_iter()
        .map(|v| canvas_c::webgpu::gpu_device::CanvasVertexBufferLayout {
          array_stride: v.array_stride,
          step_mode: match v.step_mode {
            None => CanvasVertexStepMode::Vertex,
            Some(v) => v.into(),
          },
          attributes: v.attributes.as_ptr(),
          attributes_size: v.attributes.len(),
        })
        .collect::<Vec<canvas_c::webgpu::gpu_device::CanvasVertexBufferLayout>>()
    });

    if let Some(buffers) = &buffers {
      buffers_ptr = buffers.as_ptr();
      buffers_size = buffers.len();
    }

    let vertex = canvas_c::webgpu::gpu_device::CanvasVertexState {
      module: Arc::as_ptr(&descriptor.vertex.module.module),
      entry_point: entry_point_ptr,
      constants: constants_ptr,
      buffers: buffers_ptr,
      buffers_size,
    };

    let mut primitive_ptr = std::ptr::null();

    let primitive: Option<canvas_c::webgpu::gpu_device::CanvasPrimitiveState> =
      descriptor.primitive.map(|primitive| primitive.into());

    if let Some(prim) = &primitive {
      primitive_ptr = prim
    }

    let mut depth_stencil_ptr = std::ptr::null();
    let mut multisample_ptr = std::ptr::null();

    let mut fragment_constants = None;
    let mut fragment_constants_ptr = std::ptr::null();
    let mut fragment_targets: Option<Vec<canvas_c::webgpu::structs::CanvasColorTargetState>> = None;
    let mut fragment_targets_ptr = std::ptr::null();
    let mut fragment_targets_size = 0;
    let mut fragment_state_ptr = std::ptr::null();
    let mut fragment_entry_point_ptr = std::ptr::null();
    let mut fragment_entry_point = None;

    let fragment_state = descriptor.fragment.map(|state| {
      fragment_constants = state
        .constants
        .map(|constants| canvas_c::webgpu::gpu_device::CanvasConstants::from(constants));

      if let Some(constants) = &fragment_constants {
        fragment_constants_ptr = constants;
      }

      fragment_targets = Some(
        state
          .targets
          .into_iter()
          .map(|targets| targets.into())
          .collect::<Vec<canvas_c::webgpu::structs::CanvasColorTargetState>>(),
      );

      if let Some(targets) = &fragment_targets {
        fragment_targets_ptr = targets.as_ptr();
        fragment_targets_size = targets.len();
      }

      fragment_entry_point = state.entry_point.map(|v| CString::new(v).unwrap());

      if let Some(entry_point) = &fragment_entry_point {
        fragment_entry_point_ptr = entry_point.as_ptr();
      }

      canvas_c::webgpu::gpu_device::CanvasFragmentState {
        targets: fragment_targets_ptr,
        targets_size: fragment_targets_size,
        module: state.module.module.as_ref(),
        entry_point: fragment_entry_point_ptr,
        constants: fragment_constants_ptr,
      }
    });

    if let Some(fragment) = &fragment_state {
      fragment_state_ptr = fragment;
    }

    let desc = canvas_c::webgpu::gpu_device::CanvasCreateRenderPipelineDescriptor {
      label: label_ptr,
      layout,
      vertex: &vertex,
      primitive: primitive_ptr,
      depth_stencil: depth_stencil_ptr,
      multisample: multisample_ptr,
      fragment: fragment_state_ptr,
    };
    let pipeline = unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_render_pipeline(
        Arc::as_ptr(&self.device),
        &desc,
      )
    };

    g_p_u_render_pipeline {
      pipeline: unsafe { Arc::from_raw(pipeline) },
    }
  }

  #[napi]
  pub fn create_shader_module(&self, descriptor: GPUShaderModuleDescriptor) -> g_p_u_shader_module {
    let mut label_ptr: *const c_char = std::ptr::null();
    let label = descriptor.label.map(|label| CString::new(label).unwrap());

    if let Some(label) = &label {
      label_ptr = label.as_ptr();
    }

    let source = CString::new(descriptor.code).unwrap();
    let module = unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_shader_module(
        Arc::as_ptr(&self.device),
        label_ptr,
        source.as_ptr(),
      )
    };

    g_p_u_shader_module {
      module: unsafe { Arc::from_raw(module) },
    }
  }
  #[napi]
  pub fn destroy(&self) {
    canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_destroy(Arc::as_ptr(&self.device))
  }

  #[napi]
  pub fn push_error_scope(&self, filter: GPUErrorFilter) {
    unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_push_error_scope(
        Arc::as_ptr(&self.device),
        filter.into(),
      )
    }
  }
}

struct Response {
  device: Option<g_p_u_device>,
  error: Option<String>,
}

struct Sender {
  tx: std::sync::mpsc::Sender<Response>,
}

extern "C" fn request_device(
  error: *mut c_char,
  device: *const canvas_c::webgpu::gpu_device::CanvasGPUDevice,
  data: *mut c_void,
) {
  if data.is_null() {
    return;
  }
  let data = unsafe { data as *mut Sender };
  let data = unsafe { *Box::from_raw(data) };

  if !error.is_null() {
    let error = unsafe { CString::from_raw(error).into_string().unwrap() };
    data
      .tx
      .send(Response {
        device: None,
        error: Some(error),
      })
      .unwrap();
    return;
  }

  let device = unsafe { Arc::from_raw(device) };

  data
    .tx
    .send(Response {
      device: Some(g_p_u_device { device }),
      error: None,
    })
    .unwrap();
}

#[napi(object)]
pub struct default_queue {
  pub label: Option<String>,
}

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUFeatureName", string_enum = "kebab-case")]
pub enum GPUFeatureName {
  depthClipControl,
  #[napi(value = "depth32float-stencil8")]
  depth32floatStencil8,
  textureCompressionBc,
  #[napi(value = "texture-compression-etc2")]
  textureCompressionEtc2,
  textureCompressionAstc,
  timestampQuery,
  indirectFirstInstance,
  #[napi(value = "shader-f16")]
  shaderF16,
  #[napi(value = "rg11b10ufloat-renderable")]
  rg11b10ufloatRenderable,
  #[napi(value = "bgra8unorm-storage")]
  bgra8unormStorage,
  #[napi(value = "float32-filterable")]
  float32Filterable,
}

impl Into<&CStr> for GPUFeatureName {
  fn into(self) -> &'static CStr {
    match self {
      GPUFeatureName::depthClipControl => c"depth-clip-control",
      GPUFeatureName::depth32floatStencil8 => c"depth32float-stencil8",
      GPUFeatureName::textureCompressionBc => c"texture-compression-bc",
      GPUFeatureName::textureCompressionEtc2 => c"texture-compression-etc2",
      GPUFeatureName::textureCompressionAstc => c"texture-compression-astc",
      GPUFeatureName::timestampQuery => c"timestamp-query",
      GPUFeatureName::indirectFirstInstance => c"indirect-first-instance",
      GPUFeatureName::shaderF16 => c"shader-f16",
      GPUFeatureName::rg11b10ufloatRenderable => c"rg11b10ufloat-renderable",
      GPUFeatureName::bgra8unormStorage => c"bgra8unorm-storage",
      GPUFeatureName::float32Filterable => c"float32-filterable",
    }
  }
}

#[napi(js_name = "GPUDeviceDescriptor", object)]
pub struct GPUDeviceDescriptor {
  pub default_queue: Option<default_queue>,
  pub label: Option<String>,
  pub required_features: Option<Vec<GPUFeatureName>>,
  pub required_limits: Option<GPUSupportedLimits>,
}

pub struct AsyncAdapterDevice {
  adapter: Arc<canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter>,
  descriptor: Option<GPUDeviceDescriptor>,
}

impl AsyncAdapterDevice {
  pub fn new(
    adapter: Arc<canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter>,
    descriptor: Option<GPUDeviceDescriptor>,
  ) -> Self {
    Self {
      adapter,
      descriptor,
    }
  }
}

impl Task for AsyncAdapterDevice {
  type Output = i64;
  type JsValue = ClassInstance<g_p_u_device>;

  fn compute(&mut self) -> Result<Self::Output> {
    unsafe {
      let (tx, rx) = channel();
      let data = Box::into_raw(Box::new(Sender { tx }));
      let required_feature = c"texture-adapter-specific-format-features";
      let required_features = vec![required_feature.as_ptr()];
      match self.descriptor.take() {
        None => {
          canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_request_device(
            Arc::as_ptr(&self.adapter),
            std::ptr::null_mut(),
            required_features.as_ptr(),
            required_features.len(),
            std::ptr::null(),
            request_device,
            data as _,
          );
        }
        Some(descriptor) => {
          let mut label_ptr: *const c_char = std::ptr::null_mut();
          let label = descriptor.label.map(|label| CString::new(label).unwrap());
          if let Some(label) = label.as_ref() {
            label_ptr = label.as_ptr();
          }

          let required_features = match descriptor.required_features {
            Some(required_features) => required_features
              .into_iter()
              .map(|feature| {
                let c_str: &CStr = feature.into();
                c_str.as_ptr()
              })
              .collect::<Vec<_>>(),
            _ => required_features,
          };

          let required_limits: Option<CanvasGPUSupportedLimits> =
            descriptor.required_limits.map(|v| {
              let limits: CanvasGPUSupportedLimits = v.into();
              limits
            });

          let mut required_limits_ptr: *const CanvasGPUSupportedLimits = std::ptr::null();

          if let Some(required_limits) = required_limits.as_ref() {
            required_limits_ptr = required_limits;
          };

          canvas_c::webgpu::gpu_adapter::canvas_native_webgpu_adapter_request_device(
            Arc::as_ptr(&self.adapter),
            label_ptr,
            required_features.as_ptr(),
            required_features.len(),
            required_limits_ptr,
            request_device,
            data as _,
          );
        }
      }

      match rx.recv() {
        Ok(ret) => match ret.device {
          Some(device) => Ok(Box::into_raw(Box::new(device)) as i64),
          None => {
            if let Some(error) = ret.error {
              Err(Error::from_reason(error))
            } else {
              Err(Error::from_status(Status::GenericFailure))
            }
          }
        },
        Err(error) => Err(Error::new(Status::Unknown, error.to_string())),
      }
    }
  }

  fn resolve(&mut self, env: Env, output: i64) -> Result<Self::JsValue> {
    Ok(unsafe { *Box::from_raw(output as *mut g_p_u_device) }.into_instance(env)?)
  }
}
