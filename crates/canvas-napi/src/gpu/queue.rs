use std::ffi::{c_char, c_void};
use std::sync::Arc;

use canvas_c::webgpu::gpu_command_encoder::CanvasImageCopyTexture;
use canvas_c::webgpu::structs::{
  CanvasExtent3d, CanvasImageCopyCanvasRenderingContext2D, CanvasImageCopyExternalImage,
  CanvasImageCopyGPUContext, CanvasImageCopyImageAsset, CanvasImageCopyWebGL,
  CanvasImageDataLayout, CanvasOrigin2d,
};
use napi::bindgen_prelude::{Function, Unknown};
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::{Error, Result, Status};
use napi_derive::napi;

use crate::c2d::image_data::ImageData;
use crate::c2d::CanvasRenderingContext2D;
use crate::gl::web_g_l_rendering_context;
use crate::gl2::web_g_l_2_rendering_context;
use crate::gpu::buffer::g_p_u_buffer;
use crate::gpu::callback;
use crate::gpu::command_buffer::g_p_u_command_buffer;
use crate::gpu::context::g_p_u_canvas_context;
use crate::gpu::parse::{
  array, aspect, boolean, class, downcast, extent3d, field, int32, is_object, number, origin2d,
  origin3d, string, take_string, type_error, uint32,
};
use crate::gpu::texture::g_p_u_texture;
use crate::image_asset::ImageAsset;
use crate::image_bitmap::ImageBitmap;
use crate::module::JsBytes;

#[napi(js_name = "GPUQueue")]
pub struct g_p_u_queue {
  pub(crate) queue: Arc<canvas_c::webgpu::gpu_queue::CanvasGPUQueue>,
}

type DoneCallback<'a> = Function<'a, (), UnknownReturnValue>;

extern "C" fn on_work_done(error: *mut c_char, data: *mut c_void) {
  drop(unsafe { take_string(error) });
  unsafe { callback::deliver::<()>(data, ()) };
}

/// `GPUImageCopyTexture(Tagged)`: `{ texture, mipLevel?, origin?, aspect? }`. The texture is
/// required (canvas-c dereferences it).
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

#[napi]
impl g_p_u_queue {
  fn ptr(&self) -> *const canvas_c::webgpu::gpu_queue::CanvasGPUQueue {
    Arc::as_ptr(&self.queue)
  }

  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_get_label(self.ptr()))
    }
    .unwrap_or_default()
  }

  /// `copyExternalImageToTexture({ source, origin?, flipY? }, destination, size)`. `source` is
  /// the native object packages/canvas resolved (`ImageBitmap`, `ImageData`, `ImageAsset`, a 2D,
  /// WebGL or WebGPU context), or a decoded video frame as `{ nativeTexture, width, height }`.
  #[napi(ts_args_type = "source: object, destination: object, copySize: object")]
  pub fn copy_external_image_to_texture(
    &self,
    source: Unknown,
    destination: Unknown,
    copy_size: Unknown,
  ) -> Result<()> {
    if !is_object(&source) {
      return Ok(());
    }
    let dst = image_copy_texture(&destination, "destination")?;
    let size = extent3d(Some(&copy_size));
    let origin: CanvasOrigin2d = origin2d(field(&source, c"origin").as_ref());
    let flip_y = boolean(&source, c"flipY").unwrap_or(false);
    let queue = self.ptr();

    if let Some(texture) = number(&source, c"nativeTexture") {
      // A frame already on the GPU: nothing to upload, and nothing to fall back on.
      unsafe {
        canvas_c::webgpu::gpu_native_texture::canvas_native_webgpu_queue_copy_native_texture_to_texture(
          queue,
          texture as usize as *mut c_void,
          uint32(&source, c"width").unwrap_or(0),
          uint32(&source, c"height").unwrap_or(0),
          origin.x,
          origin.y,
          flip_y,
          &dst,
          &size,
        );
      }
      return Ok(());
    }

    let Some(image) = field(&source, c"source") else {
      return Ok(());
    };
    let copy_asset = |asset: *const canvas_c::ImageAsset| unsafe {
      canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_image_asset_to_texture(
        queue,
        &CanvasImageCopyImageAsset {
          source: asset,
          origin,
          flip_y,
        },
        &dst,
        &size,
      )
    };
    let copy_webgl = |state: *mut canvas_c::WebGLState| unsafe {
      canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_webgl_to_texture(
        queue,
        &CanvasImageCopyWebGL {
          source: state,
          origin,
          flip_y,
        },
        &dst,
        &size,
      )
    };
    if let Some(bitmap) = downcast::<ImageBitmap>(&image) {
      copy_asset(Arc::as_ptr(&bitmap.asset));
    } else if let Some(asset) = downcast::<ImageAsset>(&image) {
      copy_asset(Arc::as_ptr(&asset.asset));
    } else if let Some(data) = downcast::<ImageData>(&image) {
      let inner = data.data.inner();
      let (width, height) = (inner.width() as u32, inner.height() as u32);
      let pixels = inner.data();
      if pixels.is_empty() {
        return Ok(());
      }
      unsafe {
        canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_external_image_to_texture(
          queue,
          &CanvasImageCopyExternalImage {
            source: pixels.as_ptr(),
            source_size: pixels.len(),
            origin,
            flip_y,
            width,
            height,
          },
          &dst,
          &size,
        )
      }
    } else if let Some(context) = downcast::<CanvasRenderingContext2D>(&image) {
      // Pending drawing first: the copy reads the surface.
      context.flush_pending();
      unsafe {
        canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_context_to_texture(
          queue,
          &CanvasImageCopyCanvasRenderingContext2D {
            source: context.context,
            origin,
            flip_y,
          },
          &dst,
          &size,
        )
      }
    } else if let Some(gl) = downcast::<web_g_l_rendering_context>(&image) {
      copy_webgl(gl.state);
    } else if let Some(gl) = downcast::<web_g_l_2_rendering_context>(&image) {
      copy_webgl(gl.state);
    } else if let Some(gpu) = downcast::<g_p_u_canvas_context>(&image) {
      unsafe {
        canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_gpu_context_to_texture(
          queue,
          &CanvasImageCopyGPUContext {
            source: gpu.context,
            origin,
            flip_y,
          },
          &dst,
          &size,
        )
      }
    }
    Ok(())
  }

  /// `submit([commandBuffer, ...])`; entries that are not (live) command buffers are skipped.
  #[napi(ts_args_type = "commandBuffers: GPUCommandBuffer[]")]
  pub fn submit(&self, command_buffers: Unknown) {
    let Some(items) = array(&command_buffers) else {
      return;
    };
    let buffers: Vec<_> = items
      .iter()
      .filter_map(downcast::<g_p_u_command_buffer>)
      .map(|buffer| buffer.buffer.ptr())
      .filter(|buffer| !buffer.is_null())
      .collect();
    if buffers.is_empty() {
      return;
    }
    unsafe {
      canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_submit(
        self.ptr(),
        buffers.as_ptr(),
        buffers.len(),
      );
    }
  }

  /// `onSubmittedWorkDone(callback)`: called once the work submitted so far has finished.
  #[napi(ts_args_type = "callback: () => void")]
  pub fn on_submitted_work_done(&self, callback: DoneCallback) -> Result<()> {
    let tsfn = callback
      .build_threadsafe_function::<()>()
      .build_callback(|_| Ok(()))?;
    let data = callback::into_userdata::<()>(Box::new(move |()| {
      tsfn.call((), ThreadsafeFunctionCallMode::NonBlocking);
    }));
    unsafe {
      canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_on_submitted_work_done(
        self.ptr(),
        on_work_done,
        data,
      )
    };
    Ok(())
  }

  /// `writeBuffer(buffer, bufferOffset, data, dataOffset?, size?)`: `data` an `ArrayBuffer` or
  /// any view (its bytes, read in place); `dataOffset` and `size` in bytes (packages/canvas
  /// converts element counts), `size` omitted or negative meaning "to the end".
  #[napi(
    ts_args_type = "buffer: GPUBuffer, bufferOffset: number, data: ArrayBuffer | ArrayBufferView, dataOffset?: number, size?: number"
  )]
  pub fn write_buffer(
    &self,
    buffer: &g_p_u_buffer,
    buffer_offset: f64,
    data: JsBytes,
    data_offset: Option<f64>,
    size: Option<f64>,
  ) -> Result<()> {
    let bytes = data.as_slice();
    let data_offset = data_offset.unwrap_or(0.).max(0.) as usize;
    let size = size.filter(|size| *size >= 0.).map(|size| size as usize);
    let end = match size {
      Some(size) => data_offset.checked_add(size),
      None => Some(bytes.len()),
    };
    if data_offset > bytes.len() || end.is_none_or(|end| end > bytes.len()) {
      return Err(Error::new(
        Status::GenericFailure,
        "Failed to execute 'writeBuffer' on 'GPUQueue': the range is outside the data",
      ));
    }
    if bytes.is_empty() || end == Some(data_offset) {
      return Ok(());
    }
    unsafe {
      match size {
        Some(size) => canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_write_buffer_size(
          self.ptr(),
          Arc::as_ptr(&buffer.buffer),
          buffer_offset.max(0.) as u64,
          bytes.as_ptr(),
          bytes.len(),
          data_offset,
          size,
        ),
        None => canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_write_buffer(
          self.ptr(),
          Arc::as_ptr(&buffer.buffer),
          buffer_offset.max(0.) as u64,
          bytes.as_ptr(),
          bytes.len(),
          data_offset,
        ),
      }
    }
    Ok(())
  }

  /// `writeTexture(destination, data, dataLayout, size)`.
  #[napi(
    ts_args_type = "destination: object, data: ArrayBuffer | ArrayBufferView, dataLayout: { offset?: number, bytesPerRow?: number, rowsPerImage?: number }, size: object"
  )]
  pub fn write_texture(
    &self,
    destination: Unknown,
    data: JsBytes,
    data_layout: Unknown,
    size: Unknown,
  ) -> Result<()> {
    let destination = image_copy_texture(&destination, "destination")?;
    let layout = CanvasImageDataLayout {
      offset: number(&data_layout, c"offset").map_or(0, |n| n.max(0.) as u64),
      bytes_per_row: int32(&data_layout, c"bytesPerRow").unwrap_or(-1),
      rows_per_image: int32(&data_layout, c"rowsPerImage").unwrap_or(-1),
    };
    let size: CanvasExtent3d = extent3d(Some(&size));
    let bytes = data.as_slice();
    if bytes.is_empty() {
      return Ok(());
    }
    unsafe {
      canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_write_texture(
        self.ptr(),
        &destination,
        &layout,
        &size,
        bytes.as_ptr(),
        bytes.len(),
      )
    }
    Ok(())
  }
}
