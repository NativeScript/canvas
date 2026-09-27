use crate::gpu::callback::PendingPromise;
use crate::gpu::parse::{as_number, take_string};
use crate::module::JsRaw;
use canvas_c::webgpu::error::CanvasGPUErrorType;
use canvas_c::webgpu::gpu_buffer::GPUMapMode as CGPUMapMode;
use napi::bindgen_prelude::{ArrayBuffer, ObjectFinalize, Unknown};
use napi::*;
use napi_derive::napi;
use std::cell::RefCell;
use std::os::raw::{c_char, c_void};
use std::sync::{Arc, Mutex, MutexGuard};

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUMapState", string_enum)]
#[derive(Debug, Clone, Copy)]
pub enum GPUMapState {
  unmapped,
  mapped,
  pending,
}

#[napi(js_name = "GPUBuffer", custom_finalize)]
#[derive(Debug)]
pub struct g_p_u_buffer {
  pub(crate) buffer: Arc<canvas_c::webgpu::gpu_buffer::CanvasGPUBuffer>,
  pub(crate) state: Arc<Mutex<GPUMapState>>,
  /// Weak references to the ArrayBuffers `getMappedRange` handed out. They alias the
  /// mapped memory, so `unmap()` / `destroy()` detach them before that memory goes away.
  mapped_ranges: RefCell<Vec<sys::napi_ref>>,
}

impl ObjectFinalize for g_p_u_buffer {
  fn finalize(self, env: Env) -> Result<()> {
    // Each mapped-range ArrayBuffer holds its own strong reference to the canvas-c
    // buffer (its finalize hint), so the memory it aliases outlives this wrapper;
    // only the tracking references are released here.
    for reference in self.mapped_ranges.take() {
      unsafe { sys::napi_delete_reference(env.raw(), reference) };
    }
    Ok(())
  }
}

/// What `mapAsync` needs back from canvas-c's mapping callback (the mapping poller's thread).
struct MapRequest {
  pending: PendingPromise,
  state: Arc<Mutex<GPUMapState>>,
  previous_state: GPUMapState,
}

extern "C" fn map_async(kind: CanvasGPUErrorType, error_message: *mut c_char, data: *mut c_void) {
  let message = unsafe { take_string(error_message) };
  if data.is_null() {
    return;
  }
  let request = unsafe { *Box::from_raw(data as *mut MapRequest) };
  let failed = kind != CanvasGPUErrorType::None;
  {
    let mut state = request.state.lock().unwrap_or_else(|e| {
      request.state.clear_poison();
      e.into_inner()
    });
    *state = if failed {
      request.previous_state
    } else {
      GPUMapState::mapped
    };
  }
  if failed {
    let message = message.unwrap_or_else(|| match kind {
      CanvasGPUErrorType::Lost => {
        "Failed to execute 'mapAsync' on 'GPUBuffer': the device is lost".to_owned()
      }
      CanvasGPUErrorType::OutOfMemory => {
        "Failed to execute 'mapAsync' on 'GPUBuffer': out of memory".to_owned()
      }
      _ => "Failed to execute 'mapAsync' on 'GPUBuffer'".to_owned(),
    });
    request.pending.reject_message(message);
  } else {
    request.pending.resolve_undefined();
  }
}

#[napi]
impl g_p_u_buffer {
  pub fn new(
    buffer: Arc<canvas_c::webgpu::gpu_buffer::CanvasGPUBuffer>,
    mapped_at_creation: bool,
  ) -> Self {
    Self {
      buffer,
      state: Arc::new(Mutex::new(if mapped_at_creation {
        GPUMapState::mapped
      } else {
        GPUMapState::unmapped
      })),
      mapped_ranges: RefCell::new(Vec::new()),
    }
  }

  fn lock_state(&self) -> MutexGuard<'_, GPUMapState> {
    self.state.lock().unwrap_or_else(|e| {
      self.state.clear_poison();
      e.into_inner()
    })
  }

  /// Detaches every ArrayBuffer `getMappedRange` returned, so JS can no longer reach the
  /// mapped memory once the buffer is unmapped or destroyed.
  fn detach_mapped_ranges(&self, env: &Env) {
    for reference in self.mapped_ranges.take() {
      unsafe {
        let mut value = std::ptr::null_mut();
        if sys::napi_get_reference_value(env.raw(), reference, &mut value) == sys::Status::napi_ok
          && !value.is_null()
        {
          sys::napi_detach_arraybuffer(env.raw(), value);
        }
        sys::napi_delete_reference(env.raw(), reference);
      }
    }
  }

  #[napi(getter)]
  pub fn get_map_state(&self) -> GPUMapState {
    *self.lock_state()
  }

  #[napi(getter)]
  pub fn get_label(&self) -> String {
    unsafe {
      take_string(
        canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_get_label(Arc::as_ptr(
          &self.buffer,
        )),
      )
    }
    .unwrap_or_default()
  }

  #[napi(getter)]
  pub fn get_usage(&self) -> u32 {
    canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_usage(Arc::as_ptr(&self.buffer))
  }

  #[napi(ts_return_type = "ArrayBuffer")]
  pub fn get_mapped_range<'env>(
    &self,
    env: &'env Env,
    offset: Option<i64>,
    size: Option<i64>,
  ) -> Result<ArrayBuffer<'env>> {
    let mapped = unsafe {
      canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_get_mapped_range_size(
        Arc::as_ptr(&self.buffer),
        offset.unwrap_or(-1),
        size.unwrap_or(-1),
      )
    };
    if mapped.0.is_null() {
      return ArrayBuffer::copy_from(env, [0u8; 0]);
    }
    // The ArrayBuffer aliases the mapped memory. Its finalize hint keeps the canvas-c
    // buffer alive for as long as the ArrayBuffer is, and `unmap()` / `destroy()`
    // detach it (see `detach_mapped_ranges`) before the mapping is released.
    let arraybuffer = unsafe {
      ArrayBuffer::from_external(
        env,
        mapped.0 as *mut u8,
        mapped.1 as usize,
        Arc::clone(&self.buffer),
        |_, buffer| drop(buffer),
      )
    }?;
    let mut reference = std::ptr::null_mut();
    check_status!(
      unsafe { sys::napi_create_reference(env.raw(), arraybuffer.raw(), 0, &mut reference) },
      "Failed to track the mapped range"
    )?;
    self.mapped_ranges.borrow_mut().push(reference);
    Ok(arraybuffer)
  }

  /// `mapAsync(mode, offset?, size?)`: a promise, resolved once the range is mapped (mode 1 is
  /// read, anything else write, as in the V8 bindings).
  #[napi(ts_return_type = "Promise<void>")]
  pub fn map_async(
    &self,
    env: Env,
    mode: Unknown,
    offset: Option<Unknown>,
    size: Option<Unknown>,
  ) -> Result<JsRaw> {
    let mode = match as_number(&mode) {
      Some(mode) if mode as u32 == 1 => CGPUMapMode::Read,
      _ => CGPUMapMode::Write,
    };
    let range = |value: Option<Unknown>| value.and_then(|v| as_number(&v)).map_or(-1, |n| n as i64);
    let (offset, size) = (range(offset), range(size));
    let (pending, promise) = PendingPromise::new(&env, false)?;
    let previous_state = {
      let mut state = self.lock_state();
      std::mem::replace(&mut *state, GPUMapState::pending)
    };
    let request = Box::into_raw(Box::new(MapRequest {
      pending,
      state: Arc::clone(&self.state),
      previous_state,
    }));
    canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_map_async(
      Arc::as_ptr(&self.buffer),
      mode,
      offset,
      size,
      map_async,
      request as *mut c_void,
    );
    Ok(JsRaw(promise))
  }

  #[napi]
  pub fn destroy(&self, env: Env) {
    self.detach_mapped_ranges(&env);
    canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_destroy(Arc::as_ptr(&self.buffer));
    *self.lock_state() = GPUMapState::unmapped;
  }

  #[napi(getter)]
  pub fn size(&self) -> i64 {
    canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_size(Arc::as_ptr(&self.buffer)) as i64
  }

  #[napi]
  pub fn unmap(&self, env: Env) {
    self.detach_mapped_ranges(&env);
    canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_unmap(Arc::as_ptr(&self.buffer));
    *self.lock_state() = GPUMapState::unmapped;
  }
}
