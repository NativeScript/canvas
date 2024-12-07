use crate::gpu::enums::GPUMapMode;
use canvas_c::webgpu::error::CanvasGPUErrorType;
use napi::bindgen_prelude::{AsyncTask, Unknown};
use napi::*;
use napi_derive::napi;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

#[allow(clippy::enum_variant_names)]
#[napi(js_name = "GPUMapState", string_enum)]
pub enum GPUMapState {
  unmapped,
  mapped,
  pending,
}

#[napi]
pub struct g_p_u_buffer {
  pub(crate) buffer: Arc<canvas_c::webgpu::gpu_buffer::CanvasGPUBuffer>,
  pub(crate) state: Arc<Mutex<GPUMapState>>,
}
struct Sender {
  tx: std::sync::mpsc::Sender<Option<String>>,
  state: Arc<Mutex<GPUMapState>>,
  previous_state: GPUMapState,
}

extern "C" fn map_async(_: CanvasGPUErrorType, error_message: *mut c_char, data: *mut c_void) {
  if data.is_null() {
    return;
  }
  let data = unsafe { data as *mut Sender };
  let data = unsafe { *Box::from_raw(data) };

  let ret = if !error_message.is_null() {
    (
      Some(unsafe { CString::from_raw(error_message).into_string().unwrap() }),
      data.previous_state,
    )
  } else {
    (None, GPUMapState::mapped)
  };

  let mut state = data.state.lock().unwrap_or_else(|mut e| {
    data.state.clear_poison();
    e.into_inner()
  });

  *state = ret.1;

  data.tx.send(ret.0).unwrap()
}

pub struct AsyncMapBufferTask {
  buffer: Arc<canvas_c::webgpu::gpu_buffer::CanvasGPUBuffer>,
  mode: GPUMapMode,
  offset: i64,
  size: i64,
  previous_state: GPUMapState,
  state: Arc<Mutex<GPUMapState>>,
}

impl AsyncMapBufferTask {
  pub fn new(
    buffer: Arc<canvas_c::webgpu::gpu_buffer::CanvasGPUBuffer>,
    mode: GPUMapMode,
    offset: i64,
    size: i64,
    previous_state: GPUMapState,
    state: Arc<Mutex<GPUMapState>>,
  ) -> Self {
    Self {
      buffer,
      mode,
      offset,
      size,
      previous_state,
      state,
    }
  }
}

impl Task for AsyncMapBufferTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    let (tx, rx) = channel();
    let data = Box::into_raw(Box::new(Sender {
      tx,
      state: Arc::clone(&self.state),
      previous_state: self.previous_state,
    }));
    canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_map_async(
      Arc::as_ptr(&self.buffer),
      self.mode.into(),
      self.offset,
      self.size,
      map_async,
      data as _,
    );
    match rx.recv() {
      Ok(error) => match error {
        None => Ok(()),
        Some(error) => Err(Error::new(Status::Unknown, error)),
      },
      Err(error) => Err(Error::new(Status::Unknown, error.to_string())),
    }
  }

  fn resolve(&mut self, env: Env, output: ()) -> Result<Self::JsValue> {
    Ok(())
  }
}

#[napi]
impl g_p_u_buffer {
  #[napi(getter)]
  pub fn get_state(&self) -> GPUMapState {
    *self.state.lock().unwrap_or_else(|mut e| {
      self.state.clear_poison();
      e.into_inner()
    })
  }
  #[napi(getter)]
  pub fn get_label(&self) -> String {
    let label = unsafe {
      canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_get_label(Arc::as_ptr(&self.buffer))
    };
    if label.is_null() {
      return String::new();
    }
    unsafe { CString::from_raw(label).into_string().unwrap() }
  }

  #[napi(getter)]
  pub fn get_usage(&self) -> u32 {
    canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_usage(Arc::as_ptr(&self.buffer))
  }

  #[napi(ts_return_type = "ArrayBuffer")]
  pub fn get_mapped_range(
    &self,
    env: Env,
    offset: Option<i64>,
    size: Option<i64>,
  ) -> Result<Unknown> {
    let mapped = unsafe {
      canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_get_mapped_range_size(
        Arc::as_ptr(&self.buffer),
        offset.unwrap_or(-1),
        size.unwrap_or(-1),
      )
    };
    if mapped.0.is_null() {
      return env.create_arraybuffer(0).map(|v| v.value.into_unknown());
    }
    unsafe {
      env
        .create_arraybuffer_with_borrowed_data(
          mapped.0 as *mut u8,
          mapped.1 as usize,
          (),
          |_, _| {},
        )
        .map(|v| v.value.into_unknown())
    }
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn map_async(
    &self,
    mode: GPUMapMode,
    offset: Option<i64>,
    size: Option<i64>,
  ) -> AsyncTask<AsyncMapBufferTask> {
    let mut state = self.state.lock().unwrap_or_else(|mut e| {
      self.state.clear_poison();
      e.into_inner()
    });
    let previous_state_value = *state;
    *state = GPUMapState::pending;
    let buffer = Arc::clone(&self.buffer);
    drop(state);
    AsyncTask::new(AsyncMapBufferTask::new(
      buffer,
      mode.into(),
      offset.unwrap_or(-1),
      size.unwrap_or(-1),
      previous_state_value,
      Arc::clone(&self.state),
    ))
  }

  #[napi]
  pub fn destroy(&self) {
    canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_destroy(Arc::as_ptr(&self.buffer))
  }

  #[napi(getter)]
  pub fn size(&self) -> i64 {
    canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_size(Arc::as_ptr(&self.buffer)) as i64
  }

  #[napi]
  pub fn unmap(&self) {
    canvas_c::webgpu::gpu_buffer::canvas_native_webgpu_buffer_unmap(Arc::as_ptr(&self.buffer))
  }
}
