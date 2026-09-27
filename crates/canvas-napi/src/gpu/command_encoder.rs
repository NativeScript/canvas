use std::ffi::CString;
use std::ptr;
use std::sync::Arc;

use canvas_c::webgpu::gpu_command_encoder::{
  CanvasGPUCommandEncoder, CanvasImageCopyBuffer, CanvasImageCopyTexture,
};
use canvas_c::webgpu::gpu_query_set::CanvasGPUQuerySet;
use canvas_c::webgpu::structs::{
  CanvasLoadOp, CanvasOptionF32, CanvasOptionalLoadOp, CanvasOptionalStoreOp,
  CanvasPassChannelColor, CanvasRenderPassColorAttachment, CanvasRenderPassDepthStencilAttachment,
  CanvasStoreOp,
};
use napi::bindgen_prelude::Unknown;
use napi::Result;
use napi_derive::napi;

use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::command_buffer::g_p_u_command_buffer;
use crate::gpu::compute_pass_encoder::g_p_u_compute_pass_encoder;
use crate::gpu::handle::Handle;
use crate::gpu::parse::{
  array_field, as_string, aspect, boolean, c_str, class, color, downcast, extent3d, field, int32,
  is_object, label, number, origin3d, string, take_string, type_error, uint32, uint32_value,
};
use crate::gpu::query_set::g_p_u_query_set;
use crate::gpu::render_pass_encoder::g_p_u_render_pass_encoder;
use crate::gpu::texture::g_p_u_texture;
use crate::gpu::texture_view::g_p_u_texture_view;

#[napi(js_name = "GPUCommandEncoder")]
pub struct g_p_u_command_encoder {
  pub(crate) encoder: Handle<CanvasGPUCommandEncoder>,
}

/// `loadOp`: `ParseCanvasLoadOp` (int or string, default clear).
fn load_op(value: Option<Unknown>) -> CanvasLoadOp {
  let Some(value) = value else {
    return CanvasLoadOp::Clear;
  };
  match (uint32_value(&value), as_string(&value).as_deref()) {
    (Some(1), _) | (_, Some("load")) => CanvasLoadOp::Load,
    _ => CanvasLoadOp::Clear,
  }
}

/// `storeOp`: `ParseCanvasStoreOp` (int or string, default store).
fn store_op(value: Option<Unknown>) -> CanvasStoreOp {
  let Some(value) = value else {
    return CanvasStoreOp::Store;
  };
  match (uint32_value(&value), as_string(&value).as_deref()) {
    (Some(1), _) | (_, Some("discard")) => CanvasStoreOp::Discard,
    _ => CanvasStoreOp::Store,
  }
}

fn optional_load_op(value: Option<String>) -> CanvasOptionalLoadOp {
  match value.as_deref() {
    Some("load") => CanvasOptionalLoadOp::Some(CanvasLoadOp::Load),
    Some("clear") => CanvasOptionalLoadOp::Some(CanvasLoadOp::Clear),
    _ => CanvasOptionalLoadOp::None,
  }
}

fn optional_store_op(value: Option<String>) -> CanvasOptionalStoreOp {
  match value.as_deref() {
    Some("store") => CanvasOptionalStoreOp::Some(CanvasStoreOp::Store),
    Some("discard") => CanvasOptionalStoreOp::Some(CanvasStoreOp::Discard),
    _ => CanvasOptionalStoreOp::None,
  }
}

/// An attachment `view`: a `GPUTextureView`, or (as packages/canvas's `parseRenderPassDescriptor`
/// may hand over) a `GPUTexture`, which gets its default view for the pass.
fn attachment_view(
  value: Option<Unknown>,
  keep: &mut Vec<g_p_u_texture_view>,
) -> *const canvas_c::webgpu::gpu_texture_view::CanvasGPUTextureView {
  let Some(value) = value else {
    return ptr::null();
  };
  if let Some(view) = downcast::<g_p_u_texture_view>(&value) {
    return view.texture_view.ptr();
  }
  if let Some(texture) = downcast::<g_p_u_texture>(&value) {
    if let Some(view) = g_p_u_texture::create_view_raw(texture.texture.ptr(), None) {
      let ptr = view.texture_view.ptr();
      keep.push(view);
      return ptr;
    }
  }
  ptr::null()
}

/// `timestampWrites`: `(querySet, beginningOfPassWriteIndex, endOfPassWriteIndex)`, -1 for an
/// index left out.
fn timestamp_writes(descriptor: &Unknown) -> (*const CanvasGPUQuerySet, i32, i32) {
  let Some(writes) = field(descriptor, c"timestampWrites").filter(is_object) else {
    return (ptr::null(), -1, -1);
  };
  let query_set = class::<g_p_u_query_set>(&writes, c"querySet")
    .map_or(ptr::null(), |set| Arc::as_ptr(&set.query));
  (
    query_set,
    int32(&writes, c"beginningOfPassWriteIndex").unwrap_or(-1),
    int32(&writes, c"endOfPassWriteIndex").unwrap_or(-1),
  )
}

/// `GPUImageCopyBuffer`: `{ buffer, offset?, bytesPerRow?, rowsPerImage? }`.
fn image_copy_buffer(value: &Unknown, what: &str) -> Result<CanvasImageCopyBuffer> {
  if !is_object(value) {
    return Err(type_error(format!("{what} is not an object")));
  }
  let buffer = class::<g_p_u_buffer>(value, c"buffer")
    .ok_or_else(|| type_error(format!("{what}.buffer is not a GPUBuffer")))?;
  Ok(CanvasImageCopyBuffer {
    buffer: Arc::as_ptr(&buffer.buffer),
    offset: number(value, c"offset").map_or(0, |n| n.max(0.) as u64),
    // Non-positive or absent: unspecified.
    bytes_per_row: int32(value, c"bytesPerRow")
      .filter(|n| *n > 0)
      .unwrap_or(-1),
    rows_per_image: int32(value, c"rowsPerImage").unwrap_or(-1),
  })
}

/// `GPUImageCopyTexture`: `{ texture, mipLevel?, origin?, aspect? }`.
fn image_copy_texture(value: &Unknown, what: &str) -> Result<CanvasImageCopyTexture> {
  if !is_object(value) {
    return Err(type_error(format!("{what} is not an object")));
  }
  let texture = class::<g_p_u_texture>(value, c"texture")
    .map(|texture| texture.texture.ptr())
    .filter(|texture| !texture.is_null())
    .ok_or_else(|| type_error(format!("{what}.texture is not a GPUTexture")))?;
  Ok(CanvasImageCopyTexture {
    texture,
    mip_level: uint32(value, c"mipLevel").unwrap_or(0),
    origin: origin3d(field(value, c"origin").as_ref()),
    aspect: aspect(string(value, c"aspect")),
  })
}

fn size_arg(value: Option<f64>) -> i64 {
  value.map_or(-1, |n| if n < 0. { -1 } else { n as i64 })
}

impl g_p_u_command_encoder {
  pub(crate) unsafe fn from_raw(encoder: *const CanvasGPUCommandEncoder) -> Option<Self> {
    unsafe { Handle::from_raw(encoder) }.map(|encoder| Self { encoder })
  }

  fn ptr(&self) -> *const CanvasGPUCommandEncoder {
    self.encoder.ptr()
  }
}

#[napi]
impl g_p_u_command_encoder {
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(
        canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_get_label(
          self.ptr(),
        ),
      )
    }
    .unwrap_or_default()
  }

  #[napi(ts_args_type = "descriptor?: { label?: string, timestampWrites?: object }")]
  pub fn begin_compute_pass(
    &self,
    descriptor: Option<Unknown>,
  ) -> Option<g_p_u_compute_pass_encoder> {
    let descriptor = descriptor.filter(is_object);
    let label = descriptor.as_ref().and_then(label);
    let (query_set, beginning, end) = descriptor
      .as_ref()
      .map_or((ptr::null(), -1, -1), timestamp_writes);
    let pass = canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_begin_compute_pass(
      self.ptr(),
      query_set,
      c_str(&label),
      beginning,
      end,
    );
    unsafe { Handle::from_raw(pass) }.map(|encoder| g_p_u_compute_pass_encoder { encoder })
  }

  /// `beginRenderPass(descriptor)`. Attachments without a usable view, and `null` color
  /// attachments, are left out (canvas-c has no sparse attachments).
  #[napi(ts_args_type = "descriptor: object")]
  pub fn begin_render_pass(&self, descriptor: Unknown) -> Option<g_p_u_render_pass_encoder> {
    if !is_object(&descriptor) {
      return None;
    }
    let label = label(&descriptor);
    // Default views made for textures given as attachments, alive until the pass is begun.
    let mut views = Vec::new();

    let mut color_attachments = Vec::new();
    for attachment in array_field(&descriptor, c"colorAttachments").unwrap_or_default() {
      if !is_object(&attachment) {
        continue;
      }
      let view = attachment_view(field(&attachment, c"view"), &mut views);
      if view.is_null() {
        continue;
      }
      let resolve_target = attachment_view(field(&attachment, c"resolveTarget"), &mut views);
      color_attachments.push(CanvasRenderPassColorAttachment {
        view,
        resolve_target,
        channel: CanvasPassChannelColor {
          load_op: load_op(field(&attachment, c"loadOp")),
          store_op: store_op(field(&attachment, c"storeOp")),
          clear_value: color(field(&attachment, c"clearValue").as_ref()),
          read_only: false,
        },
      });
    }

    let depth_stencil = field(&descriptor, c"depthStencilAttachment")
      .filter(is_object)
      .and_then(|attachment| {
        let view = attachment_view(field(&attachment, c"view"), &mut views);
        (!view.is_null()).then(|| CanvasRenderPassDepthStencilAttachment {
          view,
          depth_clear_value: match number(&attachment, c"depthClearValue") {
            Some(value) => CanvasOptionF32::Some(value as f32),
            None => CanvasOptionF32::None,
          },
          depth_load_op: optional_load_op(string(&attachment, c"depthLoadOp")),
          depth_store_op: optional_store_op(string(&attachment, c"depthStoreOp")),
          depth_read_only: boolean(&attachment, c"depthReadOnly").unwrap_or(false),
          stencil_clear_value: uint32(&attachment, c"stencilClearValue").unwrap_or(0),
          stencil_load_op: optional_load_op(string(&attachment, c"stencilLoadOp")),
          stencil_store_op: optional_store_op(string(&attachment, c"stencilStoreOp")),
          stencil_read_only: boolean(&attachment, c"stencilReadOnly").unwrap_or(false),
        })
      });

    let occlusion_query_set = class::<g_p_u_query_set>(&descriptor, c"occlusionQuerySet")
      .map_or(ptr::null(), |set| Arc::as_ptr(&set.query));
    let (query_set, beginning, end) = timestamp_writes(&descriptor);

    let pass = unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_begin_render_pass(
        self.ptr(),
        c_str(&label),
        if color_attachments.is_empty() {
          ptr::null()
        } else {
          color_attachments.as_ptr()
        },
        color_attachments.len(),
        depth_stencil
          .as_ref()
          .map_or(ptr::null(), |attachment| attachment as *const _),
        occlusion_query_set,
        query_set,
        beginning,
        end,
      )
    };
    drop(views);
    unsafe { Handle::from_raw(pass) }.map(|encoder| g_p_u_render_pass_encoder { encoder })
  }

  /// `clearBuffer(buffer, offset?, size?)`; -1 (what packages/canvas passes for "absent") or
  /// undefined means the default.
  #[napi]
  pub fn clear_buffer(&self, buffer: &g_p_u_buffer, offset: Option<f64>, size: Option<f64>) {
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_clear_buffer(
        self.ptr(),
        Arc::as_ptr(&buffer.buffer),
        size_arg(offset),
        size_arg(size),
      )
    }
  }

  #[napi]
  pub fn copy_buffer_to_buffer(
    &self,
    source: &g_p_u_buffer,
    source_offset: f64,
    destination: &g_p_u_buffer,
    destination_offset: f64,
    size: Option<f64>,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_copy_buffer_to_buffer(
        self.ptr(),
        Arc::as_ptr(&source.buffer),
        source_offset.max(0.) as i64,
        Arc::as_ptr(&destination.buffer),
        destination_offset.max(0.) as i64,
        size_arg(size),
      )
    }
  }

  #[napi(ts_args_type = "source: object, destination: object, copySize: object")]
  pub fn copy_buffer_to_texture(
    &self,
    source: Unknown,
    destination: Unknown,
    copy_size: Unknown,
  ) -> Result<()> {
    let src = image_copy_buffer(&source, "source")?;
    let dst = image_copy_texture(&destination, "destination")?;
    let size = extent3d(Some(&copy_size));
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_copy_buffer_to_texture(
        self.ptr(),
        &src,
        &dst,
        &size,
      )
    }
    Ok(())
  }

  #[napi(ts_args_type = "source: object, destination: object, copySize: object")]
  pub fn copy_texture_to_buffer(
    &self,
    source: Unknown,
    destination: Unknown,
    copy_size: Unknown,
  ) -> Result<()> {
    let src = image_copy_texture(&source, "source")?;
    let dst = image_copy_buffer(&destination, "destination")?;
    let size = extent3d(Some(&copy_size));
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_copy_texture_to_buffer(
        self.ptr(),
        &src,
        &dst,
        &size,
      )
    }
    Ok(())
  }

  #[napi(ts_args_type = "source: object, destination: object, copySize: object")]
  pub fn copy_texture_to_texture(
    &self,
    source: Unknown,
    destination: Unknown,
    copy_size: Unknown,
  ) -> Result<()> {
    let src = image_copy_texture(&source, "source")?;
    let dst = image_copy_texture(&destination, "destination")?;
    let size = extent3d(Some(&copy_size));
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_copy_texture_to_texture(
        self.ptr(),
        &src,
        &dst,
        &size,
      )
    }
    Ok(())
  }

  #[napi(ts_args_type = "descriptor?: { label?: string }")]
  pub fn finish(&self, descriptor: Option<Unknown>) -> Option<g_p_u_command_buffer> {
    let label = descriptor.as_ref().and_then(label);
    let buffer = unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_finish(
        self.ptr(),
        c_str(&label),
      )
    };
    unsafe { Handle::from_raw(buffer) }.map(|buffer| g_p_u_command_buffer { buffer })
  }

  #[napi]
  pub fn insert_debug_marker(&self, marker_label: String) {
    if let Ok(label) = CString::new(marker_label) {
      unsafe {
        canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_insert_debug_marker(
          self.ptr(),
          label.as_ptr(),
        )
      }
    }
  }

  #[napi]
  pub fn pop_debug_group(&self) {
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_pop_debug_group(
        self.ptr(),
      )
    }
  }

  #[napi]
  pub fn push_debug_group(&self, group_label: String) {
    if let Ok(label) = CString::new(group_label) {
      unsafe {
        canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_push_debug_group(
          self.ptr(),
          label.as_ptr(),
        )
      }
    }
  }

  #[napi]
  pub fn resolve_query_set(
    &self,
    query_set: &g_p_u_query_set,
    first_query: u32,
    query_count: u32,
    destination: &g_p_u_buffer,
    destination_offset: f64,
  ) {
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_resolve_query_set(
        self.ptr(),
        Arc::as_ptr(&query_set.query),
        first_query,
        query_count,
        Arc::as_ptr(&destination.buffer),
        destination_offset.max(0.) as u64,
      )
    }
  }

  #[napi]
  pub fn write_timestamp(&self, query_set: &g_p_u_query_set, query_index: u32) {
    unsafe {
      canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_write_timestamp(
        self.ptr(),
        Arc::as_ptr(&query_set.query),
        query_index,
      )
    }
  }

  /// Releases the encoder now (packages/canvas does right after `finish`).
  #[napi]
  pub fn destroy(&self) {
    self.encoder.release();
  }
}
