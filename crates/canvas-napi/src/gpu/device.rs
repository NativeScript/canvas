use std::cell::Cell;
use std::ffi::{c_char, c_void, CString};
use std::ptr;
use std::sync::Arc;

use canvas_c::webgpu::enums::{
  CanvasAddressMode, CanvasBindGroupEntry, CanvasBindGroupEntryResource,
  CanvasBindGroupLayoutEntry, CanvasBindingType, CanvasBufferBinding, CanvasBufferBindingLayout,
  CanvasBufferBindingType, CanvasFilterMode, CanvasOptionalCompareFunction,
  CanvasOptionalGPUTextureFormat, CanvasQueryType, CanvasSamplerBindingLayout,
  CanvasSamplerBindingType, CanvasStorageTextureAccess, CanvasStorageTextureBindingLayout,
  CanvasTextureBindingLayout, CanvasTextureDimension, CanvasTextureSampleType,
  CanvasTextureViewDimension,
};
use canvas_c::webgpu::error::CanvasGPUErrorType;
use canvas_c::webgpu::gpu_adapter::CanvasGPUAdapter;
use canvas_c::webgpu::gpu_compute_pipeline::CanvasGPUComputePipeline;
use canvas_c::webgpu::gpu_device::{
  CanvasCreateRenderBundleEncoderDescriptor, CanvasCreateSamplerDescriptor,
  CanvasCreateTextureDescriptor, CanvasGPUDevice, CanvasGPUErrorFilter,
};
use canvas_c::webgpu::gpu_render_pipeline::CanvasGPURenderPipeline;
use napi::bindgen_prelude::{FnArgs, Function, Object, ObjectFinalize, This, ToNapiValue, Unknown};
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::{check_status, sys, Env, JsValue, Result, ValueType};
use napi_derive::napi;

use crate::gpu::adapter::string_set;
use crate::gpu::bind_group::g_p_u_bind_group;
use crate::gpu::bind_group_layout::g_p_u_bind_group_layout;
use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::callback::{
  self, js_error, null, undefined, ErrorArgs, PendingPromise, Settled, Slot, WeakCallback,
};
use crate::gpu::command_encoder::g_p_u_command_encoder;
use crate::gpu::compute_pipeline::g_p_u_compute_pipeline;
use crate::gpu::external_texture::g_p_u_external_texture;
use crate::gpu::limits::g_p_u_supported_limits;
use crate::gpu::parse::{
  array_field, as_bool, as_number, as_string, boolean, c_str, class, compare_function, downcast,
  extent3d, field, is_object, label, number, string, take_string, texture_format,
  texture_format_field, texture_formats, type_error, type_of, uint32, view_dimension,
};
use crate::gpu::pipeline::{with_compute_pipeline, with_render_pipeline};
use crate::gpu::pipeline_layout::g_p_u_pipeline_layout;
use crate::gpu::query_set::g_p_u_query_set;
use crate::gpu::queue::g_p_u_queue;
use crate::gpu::render_bundle_encoder::g_p_u_render_bundle_encoder;
use crate::gpu::render_pipeline::g_p_u_render_pipeline;
use crate::gpu::sampler::g_p_u_sampler;
use crate::gpu::shader_module::g_p_u_shader_module;
use crate::gpu::texture::g_p_u_texture;
use crate::gpu::texture_view::g_p_u_texture_view;
use crate::module::JsRaw;

/// What `setuncapturederror` / `popErrorScope` callbacks receive (`ErrorArgs`): the V8 bindings'
/// error type (0 none, 1 lost, 2 out-of-memory, 3 validation, 4 internal) and the message.
type ErrorCallback<'a> = Function<'a, FnArgs<ErrorArgs>, UnknownReturnValue>;
type UncapturedCallback = dyn Fn(ErrorArgs) + Send;

/// `(error, pipeline)` for `create*PipelineAsync`.
type PipelineCallback<'a> = Function<'a, FnArgs<(JsRaw, JsRaw)>, UnknownReturnValue>;
/// What canvas-c's async pipeline callbacks report: the pipeline pointer, error type, message.
type PipelineResult = (usize, u32, Option<String>);

#[napi(js_name = "GPUDevice", custom_finalize)]
pub struct g_p_u_device {
  pub(crate) device: Arc<CanvasGPUDevice>,
  pub(crate) adapter: Arc<CanvasGPUAdapter>,
  /// The `lost` promise (a strong reference), made on first read.
  lost: Cell<sys::napi_ref>,
  lost_slot: Cell<Option<&'static Slot<PendingPromise>>>,
  uncaptured: Cell<Option<&'static Slot<UncapturedCallback>>>,
  destroyed: Cell<bool>,
}

impl ObjectFinalize for g_p_u_device {
  fn finalize(self, env: Env) -> Result<()> {
    let lost = self.lost.get();
    if !lost.is_null() {
      unsafe { sys::napi_delete_reference(env.raw(), lost) };
    }
    // canvas-c may still call into the slots; they just go empty.
    if let Some(slot) = self.lost_slot.get() {
      drop(slot.take());
    }
    if let Some(slot) = self.uncaptured.get() {
      drop(slot.take());
    }
    Ok(())
  }
}

fn error_type(value: CanvasGPUErrorType) -> u32 {
  value as u32
}

/// `{ reason, message }`, what `lost` resolves with (reason 0 unknown, 1 destroyed).
fn lost_info(env: &Env, reason: i32, message: String) -> Result<Settled> {
  let mut info = Object::new(env)?;
  info.set("reason", reason)?;
  info.set("message", message)?;
  Ok(Settled::Resolve(info.raw()))
}

unsafe extern "C" fn on_lost(reason: i32, message: *mut c_char, data: *mut c_void) {
  let message = unsafe { take_string(message) }.unwrap_or_default();
  if data.is_null() {
    return;
  }
  let slot = unsafe { &*(data as *const Slot<PendingPromise>) };
  if let Some(pending) = slot.take() {
    pending.settle(move |env| lost_info(env, reason, message));
  }
}

unsafe extern "C" fn on_uncaptured_error(
  kind: CanvasGPUErrorType,
  message: *mut c_char,
  data: *mut c_void,
) {
  let message = unsafe { take_string(message) };
  if data.is_null() {
    return;
  }
  let slot = unsafe { &*(data as *const Slot<UncapturedCallback>) };
  slot.with(|callback| callback((error_type(kind), message)));
}

unsafe extern "C" fn on_pop_error_scope(
  kind: CanvasGPUErrorType,
  message: *mut c_char,
  data: *mut c_void,
) {
  let message = unsafe { take_string(message) };
  if !data.is_null() {
    unsafe { *(data as *mut Option<ErrorArgs>) = Some((error_type(kind), message)) };
  }
}

unsafe extern "C" fn on_compute_pipeline(
  pipeline: *const CanvasGPUComputePipeline,
  kind: CanvasGPUErrorType,
  message: *mut c_char,
  data: *mut c_void,
) {
  let message = unsafe { take_string(message) };
  unsafe {
    callback::deliver::<PipelineResult>(data, (pipeline as usize, error_type(kind), message))
  };
}

unsafe extern "C" fn on_render_pipeline(
  pipeline: *const CanvasGPURenderPipeline,
  kind: CanvasGPUErrorType,
  message: *mut c_char,
  data: *mut c_void,
) {
  let message = unsafe { take_string(message) };
  unsafe {
    callback::deliver::<PipelineResult>(data, (pipeline as usize, error_type(kind), message))
  };
}

/// The userdata for a `create*PipelineAsync` callback: calls `callback(null, pipeline)` or, as
/// the V8 bindings do, `callback({ error, type })`.
fn pipeline_delivery<T: ToNapiValue + 'static>(
  callback: PipelineCallback,
  wrap: fn(usize) -> T,
) -> Result<callback::Delivery<PipelineResult>> {
  let tsfn = callback
    .build_threadsafe_function::<PipelineResult>()
    .build_callback(move |ctx| {
      let env = ctx.env.raw();
      let (pipeline, kind, message) = ctx.value;
      if kind != 0 || pipeline == 0 {
        if pipeline != 0 {
          drop(wrap(pipeline));
        }
        let mut error = Object::new(&ctx.env)?;
        let message = message.unwrap_or_else(|| "Failed to create the pipeline".to_owned());
        error.set("error", JsRaw(js_error(env, &message)))?;
        error.set("type", kind)?;
        return Ok(FnArgs::from((JsRaw(error.raw()), JsRaw(undefined(env)))));
      }
      let value = unsafe { T::to_napi_value(env, wrap(pipeline)) }?;
      Ok(FnArgs::from((JsRaw(null(env)), JsRaw(value))))
    })?;
  Ok(Box::new(move |result| {
    tsfn.call(result, ThreadsafeFunctionCallMode::NonBlocking);
  }))
}

fn address_mode(value: Option<String>) -> CanvasAddressMode {
  match value.as_deref() {
    Some("repeat") => CanvasAddressMode::Repeat,
    Some("mirror-repeat") => CanvasAddressMode::MirrorRepeat,
    _ => CanvasAddressMode::ClampToEdge,
  }
}

fn filter_mode(value: Option<String>) -> CanvasFilterMode {
  match value.as_deref() {
    Some("linear") => CanvasFilterMode::Linear,
    _ => CanvasFilterMode::Nearest,
  }
}

/// Visibility bits wgpu knows (vertex, fragment, compute, task, mesh): canvas-c unwraps
/// `ShaderStages::from_bits`, so anything else must not reach it.
const SHADER_STAGES: u32 = 0x1F;

/// `ParseBindGroupLayoutEntries`: buffer, externalTexture, sampler, storageTexture, texture, in
/// that order of precedence.
fn bind_group_layout_entry(entry: &Unknown) -> Option<CanvasBindGroupLayoutEntry> {
  let binding = number(entry, c"binding").map_or(0, |n| n.max(0.) as u32);
  let visibility = number(entry, c"visibility").map_or(0, |n| n.max(0.) as u32) & SHADER_STAGES;
  let binding_type = if let Some(buffer) = field(entry, c"buffer").filter(is_object) {
    CanvasBindingType::Buffer(CanvasBufferBindingLayout {
      type_: match string(&buffer, c"type").as_deref() {
        Some("storage") => CanvasBufferBindingType::Storage,
        Some("read-only-storage") => CanvasBufferBindingType::ReadOnlyStorage,
        _ => CanvasBufferBindingType::Uniform,
      },
      has_dynamic_offset: boolean(&buffer, c"hasDynamicOffset").unwrap_or(false),
      min_binding_size: number(&buffer, c"minBindingSize").map_or(-1, |n| n as i64),
    })
  } else if field(entry, c"externalTexture").filter(is_object).is_some() {
    CanvasBindingType::ExternalTexture
  } else if let Some(sampler) = field(entry, c"sampler").filter(is_object) {
    CanvasBindingType::Sampler(CanvasSamplerBindingLayout {
      type_: match string(&sampler, c"type").as_deref() {
        Some("comparison") => CanvasSamplerBindingType::Comparison,
        Some("non-filtering") => CanvasSamplerBindingType::NonFiltering,
        _ => CanvasSamplerBindingType::Filtering,
      },
    })
  } else if let Some(storage) = field(entry, c"storageTexture").filter(is_object) {
    // A storage texture needs a format; the V8 bindings drop the entry without one.
    let format = texture_format_field(&storage, c"format")?;
    CanvasBindingType::StorageTexture(CanvasStorageTextureBindingLayout {
      access: match string(&storage, c"access").as_deref() {
        Some("read-only") => CanvasStorageTextureAccess::ReadOnly,
        Some("read-write") => CanvasStorageTextureAccess::ReadWrite,
        _ => CanvasStorageTextureAccess::WriteOnly,
      },
      format,
      view_dimension: string(&storage, c"viewDimension")
        .and_then(|v| view_dimension(&v))
        .unwrap_or(CanvasTextureViewDimension::D2),
    })
  } else if let Some(texture) = field(entry, c"texture").filter(is_object) {
    CanvasBindingType::Texture(CanvasTextureBindingLayout {
      sample_type: match string(&texture, c"sampleType").as_deref() {
        Some("depth") => CanvasTextureSampleType::Depth,
        Some("sint") => CanvasTextureSampleType::Sint,
        Some("uint") => CanvasTextureSampleType::Uint,
        Some("unfilterable-float") => CanvasTextureSampleType::UnfilterableFloat,
        _ => CanvasTextureSampleType::Float,
      },
      view_dimension: string(&texture, c"viewDimension")
        .and_then(|v| view_dimension(&v))
        .unwrap_or(CanvasTextureViewDimension::D2),
      multisampled: boolean(&texture, c"multisampled").unwrap_or(false),
    })
  } else {
    return None;
  };
  Some(CanvasBindGroupLayoutEntry {
    binding,
    visibility,
    binding_type,
  })
}

/// `ParseBindGroupEntries`: a sampler, texture view or external texture, or
/// `{ buffer, offset?, size? }` (a bare `GPUBuffer` is taken as the whole buffer).
fn bind_group_entry(entry: &Unknown) -> Option<CanvasBindGroupEntry> {
  let binding = number(entry, c"binding").map_or(0, |n| n.max(0.) as u32);
  let resource = field(entry, c"resource")?;
  let resource = if let Some(sampler) = downcast::<g_p_u_sampler>(&resource) {
    CanvasBindGroupEntryResource::Sampler(Arc::as_ptr(&sampler.sampler))
  } else if let Some(view) = downcast::<g_p_u_texture_view>(&resource) {
    let view = view.texture_view.ptr();
    if view.is_null() {
      return None;
    }
    CanvasBindGroupEntryResource::TextureView(view)
  } else if let Some(texture) = downcast::<g_p_u_external_texture>(&resource) {
    CanvasBindGroupEntryResource::ExternalTexture(Arc::as_ptr(&texture.texture))
  } else if let Some(buffer) = downcast::<g_p_u_buffer>(&resource) {
    CanvasBindGroupEntryResource::Buffer(CanvasBufferBinding {
      buffer: Arc::as_ptr(&buffer.buffer),
      offset: 0,
      size: -1,
    })
  } else {
    let buffer = class::<g_p_u_buffer>(&resource, c"buffer")?;
    CanvasBindGroupEntryResource::Buffer(CanvasBufferBinding {
      buffer: Arc::as_ptr(&buffer.buffer),
      offset: number(&resource, c"offset").map_or(-1, |n| n as i64),
      size: number(&resource, c"size").map_or(-1, |n| n as i64),
    })
  };
  Some(CanvasBindGroupEntry { binding, resource })
}

#[napi]
impl g_p_u_device {
  pub(crate) fn new(device: Arc<CanvasGPUDevice>, adapter: Arc<CanvasGPUAdapter>) -> Self {
    Self {
      device,
      adapter,
      lost: Cell::new(ptr::null_mut()),
      lost_slot: Cell::new(None),
      uncaptured: Cell::new(None),
      destroyed: Cell::new(false),
    }
  }

  fn ptr(&self) -> *const CanvasGPUDevice {
    Arc::as_ptr(&self.device)
  }

  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_get_label(self.ptr()))
    }
    .unwrap_or_default()
  }

  #[napi(getter, ts_return_type = "Set<string>")]
  pub fn get_features<'env>(&self, env: &'env Env) -> Result<Unknown<'env>> {
    let features =
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_get_features(self.ptr());
    let features: Vec<String> = if features.is_null() {
      Vec::new()
    } else {
      unsafe { *Box::from_raw(features) }.into()
    };
    string_set(env, features)
  }

  #[napi(getter)]
  pub fn get_limits(&self) -> g_p_u_supported_limits {
    let limits = canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_get_limits(self.ptr());
    if limits.is_null() {
      return g_p_u_supported_limits::new();
    }
    unsafe { *Box::from_raw(limits) }.into()
  }

  #[napi(getter)]
  pub fn get_queue(&self) -> Option<g_p_u_queue> {
    let queue = canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_get_queue(self.ptr());
    (!queue.is_null()).then(|| g_p_u_queue {
      queue: unsafe { Arc::from_raw(queue) },
    })
  }

  /// A promise of `{ reason, message }`, settled when the device is lost or `destroy()`ed. It does
  /// not keep the host alive while pending.
  #[napi(
    getter,
    ts_return_type = "Promise<{ reason: number, message: string }>"
  )]
  pub fn get_lost(&self, env: Env) -> Result<JsRaw> {
    let existing = self.lost.get();
    if !existing.is_null() {
      let mut value = ptr::null_mut();
      check_status!(unsafe { sys::napi_get_reference_value(env.raw(), existing, &mut value) })?;
      return Ok(JsRaw(value));
    }
    let (pending, promise) = PendingPromise::new(&env, true)?;
    if self.destroyed.get() {
      pending.settle(|env| lost_info(env, 1, String::new()));
    } else {
      let slot = Slot::leak();
      slot.set(Some(Box::new(pending)));
      self.lost_slot.set(Some(slot));
      unsafe {
        canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_set_lost_callback(
          self.ptr(),
          Some(on_lost),
          slot as *const Slot<PendingPromise> as *mut c_void,
        )
      };
    }
    let mut reference = ptr::null_mut();
    check_status!(unsafe { sys::napi_create_reference(env.raw(), promise, 1, &mut reference) })?;
    self.lost.set(reference);
    Ok(JsRaw(promise))
  }

  /// `setuncapturederror(callback(type, message))`: errors no error scope captured, from any
  /// thread, delivered on the JS thread. The callback lives as long as this wrapper (it is kept
  /// on it) and keeps neither the wrapper nor the host alive.
  #[napi(
    js_name = "setuncapturederror",
    ts_args_type = "callback: ((type: number, message: string | null) => void) | null"
  )]
  pub fn set_uncaptured_error(
    &self,
    env: Env,
    this: This,
    callback: Option<Unknown>,
  ) -> Result<()> {
    let slot = match self.uncaptured.get() {
      Some(slot) => slot,
      None => {
        let slot = Slot::<UncapturedCallback>::leak();
        self.uncaptured.set(Some(slot));
        unsafe {
          canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_set_uncaptured_error_callback(
            self.ptr(),
            Some(on_uncaptured_error),
            slot as *const Slot<UncapturedCallback> as *mut c_void,
          )
        };
        slot
      }
    };
    let callback = callback.filter(|callback| type_of(callback) == ValueType::Function);
    let Some(callback) = callback else {
      slot.set(None);
      return Ok(());
    };
    let weak = WeakCallback::new(&env, this.raw(), c"__uncapturederror", callback.raw())?;
    slot.set(Some(Box::new(move |args| weak.call(args))));
    Ok(())
  }

  /// `popErrorScope(callback(type, message))`: type 0 when the scope caught nothing.
  #[napi(ts_args_type = "callback: (type: number, message: string | null) => void")]
  pub fn pop_error_scope(&self, callback: ErrorCallback) -> Result<()> {
    // canvas-c reports synchronously.
    let mut result: Option<ErrorArgs> = None;
    unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_pop_error_scope(
        self.ptr(),
        Some(on_pop_error_scope),
        &mut result as *mut Option<ErrorArgs> as *mut c_void,
      )
    };
    callback.call(FnArgs::from(result.unwrap_or((0, None))))?;
    Ok(())
  }

  #[napi(ts_args_type = "filter: 'validation' | 'out-of-memory' | 'internal'")]
  pub fn push_error_scope(&self, filter: Unknown) {
    let filter = match as_string(&filter).as_deref() {
      Some("validation") => CanvasGPUErrorFilter::Validation,
      Some("out-of-memory") => CanvasGPUErrorFilter::OutOfMemory,
      Some("internal") => CanvasGPUErrorFilter::Internal,
      _ => return,
    };
    unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_push_error_scope(self.ptr(), filter)
    }
  }

  /// Destroys the device; `lost` resolves with reason 1 ("destroyed").
  #[napi]
  pub fn destroy(&self) {
    canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_destroy(self.ptr());
    self.destroyed.set(true);
    if let Some(pending) = self.lost_slot.get().and_then(|slot| slot.take()) {
      pending.settle(|env| lost_info(env, 1, String::new()));
    }
  }

  #[napi(ts_args_type = "descriptor: object")]
  pub fn create_bind_group(&self, descriptor: Unknown) -> Option<g_p_u_bind_group> {
    if !is_object(&descriptor) {
      return None;
    }
    let label = label(&descriptor);
    let layout = class::<g_p_u_bind_group_layout>(&descriptor, c"layout");
    let entries: Vec<CanvasBindGroupEntry> = array_field(&descriptor, c"entries")
      .unwrap_or_default()
      .iter()
      .filter(|entry| is_object(entry))
      .filter_map(bind_group_entry)
      .collect();
    let group = unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_bind_group(
        self.ptr(),
        c_str(&label),
        layout
          .as_ref()
          .map_or(ptr::null(), |layout| Arc::as_ptr(&layout.layout)),
        entries.as_ptr(),
        entries.len(),
      )
    };
    (!group.is_null()).then(|| g_p_u_bind_group {
      group: unsafe { Arc::from_raw(group) },
    })
  }

  #[napi(ts_args_type = "descriptor: object")]
  pub fn create_bind_group_layout(&self, descriptor: Unknown) -> Option<g_p_u_bind_group_layout> {
    if !is_object(&descriptor) {
      return None;
    }
    let label = label(&descriptor);
    let entries: Vec<CanvasBindGroupLayoutEntry> = array_field(&descriptor, c"entries")
      .unwrap_or_default()
      .iter()
      .filter(|entry| is_object(entry))
      .filter_map(bind_group_layout_entry)
      .collect();
    let layout = unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_bind_group_layout(
        self.ptr(),
        c_str(&label),
        if entries.is_empty() {
          ptr::null()
        } else {
          entries.as_ptr()
        },
        entries.len(),
      )
    };
    (!layout.is_null()).then(|| g_p_u_bind_group_layout {
      layout: unsafe { Arc::from_raw(layout) },
    })
  }

  /// `createBuffer({ label?, size, usage, mappedAtCreation? })`; undefined for usage bits
  /// canvas-c rejects (reported as a validation error).
  #[napi(
    ts_args_type = "descriptor: { label?: string, size: number, usage: number, mappedAtCreation?: boolean }"
  )]
  pub fn create_buffer(&self, descriptor: Unknown) -> Option<g_p_u_buffer> {
    let (label, size, usage, mapped_at_creation) = if is_object(&descriptor) {
      (
        label(&descriptor),
        number(&descriptor, c"size").map_or(0, |n| n.max(0.) as u64),
        number(&descriptor, c"usage").map_or(0, |n| n.max(0.) as u32),
        field(&descriptor, c"mappedAtCreation")
          .is_some_and(|v| as_bool(&v).unwrap_or_else(|| as_number(&v).is_some_and(|n| n != 0.))),
      )
    } else {
      (None, 0, 0, false)
    };
    let buffer = canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_buffer(
      self.ptr(),
      c_str(&label),
      size,
      usage,
      mapped_at_creation,
    );
    (!buffer.is_null())
      .then(|| g_p_u_buffer::new(unsafe { Arc::from_raw(buffer) }, mapped_at_creation))
  }

  #[napi(ts_args_type = "descriptor?: { label?: string }")]
  pub fn create_command_encoder(
    &self,
    descriptor: Option<Unknown>,
  ) -> Option<g_p_u_command_encoder> {
    let label = descriptor.as_ref().and_then(|descriptor| {
      as_string(descriptor)
        .and_then(|label| CString::new(label).ok())
        .or_else(|| label(descriptor))
    });
    let encoder = canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_command_encoder(
      self.ptr(),
      c_str(&label),
    );
    unsafe { g_p_u_command_encoder::from_raw(encoder) }
  }

  #[napi(ts_args_type = "descriptor: object")]
  pub fn create_compute_pipeline(
    &self,
    descriptor: Unknown,
  ) -> Result<Option<g_p_u_compute_pipeline>> {
    let pipeline = with_compute_pipeline(&descriptor, |label, layout, stage| unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_compute_pipeline(
        self.ptr(),
        label,
        layout,
        stage,
      )
    })?;
    Ok((!pipeline.is_null()).then(|| g_p_u_compute_pipeline {
      pipeline: unsafe { Arc::from_raw(pipeline) },
    }))
  }

  /// `createComputePipelineAsync(descriptor, callback(error, pipeline))`; `error` is
  /// `{ error, type }`.
  #[napi(
    ts_args_type = "descriptor: object, callback: (error: { error: Error, type: number } | null, pipeline?: GPUComputePipeline) => void"
  )]
  pub fn create_compute_pipeline_async(
    &self,
    descriptor: Unknown,
    callback: PipelineCallback,
  ) -> Result<()> {
    let deliver = pipeline_delivery(callback, |pipeline| g_p_u_compute_pipeline {
      pipeline: unsafe { Arc::from_raw(pipeline as *const CanvasGPUComputePipeline) },
    })?;
    with_compute_pipeline(&descriptor, move |label, layout, stage| unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_compute_pipeline_async(
        self.ptr(),
        label,
        layout,
        stage,
        on_compute_pipeline,
        callback::into_userdata(deliver),
      )
    })
  }

  #[napi(ts_args_type = "descriptor: { label?: string, bindGroupLayouts: GPUBindGroupLayout[] }")]
  pub fn create_pipeline_layout(&self, descriptor: Unknown) -> Option<g_p_u_pipeline_layout> {
    if !is_object(&descriptor) {
      return None;
    }
    let label = label(&descriptor);
    let items = array_field(&descriptor, c"bindGroupLayouts").unwrap_or_default();
    let layouts: Vec<_> = items
      .iter()
      .filter_map(downcast::<g_p_u_bind_group_layout>)
      .map(|layout| Arc::as_ptr(&layout.layout))
      .collect();
    let layout = unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_pipeline_layout(
        self.ptr(),
        c_str(&label),
        if layouts.is_empty() {
          ptr::null()
        } else {
          layouts.as_ptr()
        },
        layouts.len(),
      )
    };
    (!layout.is_null()).then(|| g_p_u_pipeline_layout {
      layout: unsafe { Arc::from_raw(layout) },
    })
  }

  #[napi(
    ts_args_type = "descriptor: { label?: string, type: 'occlusion' | 'timestamp', count: number }"
  )]
  pub fn create_query_set(&self, descriptor: Unknown) -> Option<g_p_u_query_set> {
    if !is_object(&descriptor) {
      return None;
    }
    let kind = match string(&descriptor, c"type").as_deref() {
      Some("occlusion") => CanvasQueryType::Occlusion,
      Some("timestamp") => CanvasQueryType::Timestamp,
      _ => return None,
    };
    let label = label(&descriptor);
    let count = number(&descriptor, c"count").map_or(0, |n| n.max(0.) as u32);
    let set = unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_query_set(
        self.ptr(),
        c_str(&label),
        kind,
        count,
      )
    };
    (!set.is_null()).then(|| g_p_u_query_set {
      query: unsafe { Arc::from_raw(set) },
    })
  }

  #[napi(ts_args_type = "descriptor: object")]
  pub fn create_render_bundle_encoder(
    &self,
    descriptor: Unknown,
  ) -> Option<g_p_u_render_bundle_encoder> {
    if !is_object(&descriptor) {
      return None;
    }
    let label = label(&descriptor);
    let color_formats = texture_formats(field(&descriptor, c"colorFormats").as_ref());
    let desc = CanvasCreateRenderBundleEncoderDescriptor {
      label: c_str(&label),
      color_formats: if color_formats.is_empty() {
        ptr::null()
      } else {
        color_formats.as_ptr()
      },
      color_formats_size: color_formats.len(),
      depth_stencil_format: match texture_format_field(&descriptor, c"depthStencilFormat") {
        Some(format) => CanvasOptionalGPUTextureFormat::Some(format),
        None => CanvasOptionalGPUTextureFormat::None,
      },
      sample_count: uint32(&descriptor, c"sampleCount").unwrap_or(1),
      depth_read_only: boolean(&descriptor, c"depthReadOnly").unwrap_or(false),
      stencil_read_only: boolean(&descriptor, c"stencilReadOnly").unwrap_or(false),
    };
    let encoder = unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_render_bundle_encoder(
        self.ptr(),
        &desc,
      )
    };
    (!encoder.is_null()).then(|| g_p_u_render_bundle_encoder {
      encoder: unsafe { Arc::from_raw(encoder) },
    })
  }

  #[napi(ts_args_type = "descriptor: object")]
  pub fn create_render_pipeline(
    &self,
    descriptor: Unknown,
  ) -> Result<Option<g_p_u_render_pipeline>> {
    let pipeline = with_render_pipeline(&descriptor, |desc| unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_render_pipeline(
        self.ptr(),
        desc,
      )
    })?;
    Ok((!pipeline.is_null()).then(|| g_p_u_render_pipeline {
      pipeline: unsafe { Arc::from_raw(pipeline) },
    }))
  }

  /// `createRenderPipelineAsync(descriptor, callback(error, pipeline))`; `error` is
  /// `{ error, type }`.
  #[napi(
    ts_args_type = "descriptor: object, callback: (error: { error: Error, type: number } | null, pipeline?: GPURenderPipeline) => void"
  )]
  pub fn create_render_pipeline_async(
    &self,
    descriptor: Unknown,
    callback: PipelineCallback,
  ) -> Result<()> {
    let deliver = pipeline_delivery(callback, |pipeline| g_p_u_render_pipeline {
      pipeline: unsafe { Arc::from_raw(pipeline as *const CanvasGPURenderPipeline) },
    })?;
    with_render_pipeline(&descriptor, move |desc| unsafe {
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_render_pipeline_async(
        self.ptr(),
        desc,
        on_render_pipeline,
        callback::into_userdata(deliver),
      )
    })
  }

  #[napi(ts_args_type = "descriptor?: object")]
  pub fn create_sampler(&self, descriptor: Option<Unknown>) -> Option<g_p_u_sampler> {
    let descriptor = descriptor.filter(is_object);
    let label = descriptor.as_ref().and_then(label);
    let desc = descriptor.as_ref().map(|d| CanvasCreateSamplerDescriptor {
      label: c_str(&label),
      address_mode_u: address_mode(string(d, c"addressModeU")),
      address_mode_v: address_mode(string(d, c"addressModeV")),
      address_mode_w: address_mode(string(d, c"addressModeW")),
      mag_filter: filter_mode(string(d, c"magFilter")),
      min_filter: filter_mode(string(d, c"minFilter")),
      mipmap_filter: filter_mode(string(d, c"mipmapFilter")),
      lod_min_clamp: number(d, c"lodMinClamp").unwrap_or(0.) as f32,
      lod_max_clamp: number(d, c"lodMaxClamp").unwrap_or(32.) as f32,
      compare: match compare_function(string(d, c"compare")) {
        Some(compare) => CanvasOptionalCompareFunction::Some(compare),
        None => CanvasOptionalCompareFunction::None,
      },
      max_anisotropy: number(d, c"maxAnisotropy")
        .map_or(1, |n| n.clamp(1., u16::MAX as f64) as u16),
    });
    let sampler = canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_sampler(
      self.ptr(),
      desc.as_ref().map_or(ptr::null(), |desc| desc as *const _),
    );
    (!sampler.is_null()).then(|| g_p_u_sampler {
      sampler: unsafe { Arc::from_raw(sampler) },
    })
  }

  #[napi(ts_args_type = "descriptor: { label?: string, code: string }")]
  pub fn create_shader_module(&self, descriptor: Unknown) -> Option<g_p_u_shader_module> {
    if !is_object(&descriptor) {
      return None;
    }
    let label = label(&descriptor);
    let code = CString::new(string(&descriptor, c"code").unwrap_or_default()).unwrap_or_default();
    let module = canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_shader_module(
      self.ptr(),
      c_str(&label),
      code.as_ptr(),
    );
    (!module.is_null()).then(|| g_p_u_shader_module {
      module: unsafe { Arc::from_raw(module) },
    })
  }

  /// `createTexture({ label?, width, height?, depthOrArrayLayers?, format, usage, dimension?,
  /// mipLevelCount?, sampleCount?, viewFormats? })`: the size flattened as packages/canvas
  /// passes it (a `size` extent is read too).
  #[napi(ts_args_type = "descriptor: object")]
  pub fn create_texture(&self, descriptor: Unknown) -> Result<Option<g_p_u_texture>> {
    if !is_object(&descriptor) {
      return Ok(None);
    }
    let label = label(&descriptor);
    let size = extent3d(field(&descriptor, c"size").as_ref());
    let format = string(&descriptor, c"format")
      .and_then(|format| texture_format(&format))
      .ok_or_else(|| {
        type_error("Failed to execute 'createTexture' on 'GPUDevice': invalid format")
      })?;
    let view_formats = texture_formats(field(&descriptor, c"viewFormats").as_ref());
    let desc = CanvasCreateTextureDescriptor {
      label: c_str(&label),
      dimension: match string(&descriptor, c"dimension").as_deref() {
        Some("1d") => CanvasTextureDimension::D1,
        Some("3d") => CanvasTextureDimension::D3,
        _ => CanvasTextureDimension::D2,
      },
      format,
      mipLevelCount: uint32(&descriptor, c"mipLevelCount").unwrap_or(1),
      sampleCount: uint32(&descriptor, c"sampleCount").unwrap_or(1),
      width: uint32(&descriptor, c"width").unwrap_or(size.width),
      height: uint32(&descriptor, c"height").unwrap_or(size.height),
      depthOrArrayLayers: uint32(&descriptor, c"depthOrArrayLayers")
        .unwrap_or(size.depth_or_array_layers),
      usage: number(&descriptor, c"usage").map_or(0, |n| n.max(0.) as u32),
      view_formats: if view_formats.is_empty() {
        ptr::null()
      } else {
        view_formats.as_ptr()
      },
      view_formats_size: view_formats.len(),
    };
    let texture =
      canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_texture(self.ptr(), &desc);
    Ok(unsafe { g_p_u_texture::from_raw(texture) })
  }

  /// `importExternalTexture({ nativeTexture, width, height, label? })`: packages/canvas resolves
  /// the video to a platform texture; undefined where the backend cannot import it.
  #[napi(
    ts_args_type = "descriptor: { nativeTexture: number, width: number, height: number, label?: string }"
  )]
  pub fn import_external_texture(&self, descriptor: Unknown) -> Option<g_p_u_external_texture> {
    if !is_object(&descriptor) {
      return None;
    }
    let label = label(&descriptor);
    let native_texture = number(&descriptor, c"nativeTexture").map_or(0, |n| n as usize);
    let texture = unsafe {
      canvas_c::webgpu::gpu_external_texture::canvas_native_webgpu_device_import_external_texture(
        self.ptr(),
        c_str(&label),
        native_texture as *mut c_void,
        uint32(&descriptor, c"width").unwrap_or(0),
        uint32(&descriptor, c"height").unwrap_or(0),
      )
    };
    (!texture.is_null()).then(|| g_p_u_external_texture {
      texture: unsafe { Arc::from_raw(texture) },
    })
  }
}
