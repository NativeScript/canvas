//! AudioWorklet: JS processors run in a Worker, fed through a per-node ring of quanta.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock, Weak};
use std::thread::{self, JoinHandle, Thread};
use std::time::Duration;

use napi::bindgen_prelude::Function;
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::{sys, Env, Result, Unknown};
use napi_derive::napi;
use web_audio_api::worklet::{AudioParamValues, AudioWorkletGlobalScope, AudioWorkletProcessor};
use web_audio_api::{AudioParamDescriptor, AutomationRate};

use crate::error;

pub const QUANTUM: usize = 128;
pub const SLOTS: usize = 64;
pub const MAX_INPUT_CHANNELS: usize = 8;
/// For an output whose channel count follows its input.
pub const MAX_DYNAMIC_OUTPUT_CHANNELS: usize = 8;
const MAX_BACKLOG: u64 = 48;
/// Quanta over which a backlog left by a stall is measured before shrinking it.
const SHRINK_WINDOW: u32 = 128;
const OFFLINE_WAIT: Duration = Duration::from_secs(2);

/// Slot header word offsets; the frame is an f64 in words 0-1.
const FRAME: usize = 0;
const ALIVE: usize = 2;
const COUNTS: usize = 4;

/// Word offsets within one slot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layout {
  pub inputs: usize,
  pub output_capacity: Vec<usize>,
  pub params: usize,
  pub input_counts: usize,
  pub output_counts: usize,
  pub param_lengths: usize,
  pub input_data: usize,
  pub output_data: Vec<usize>,
  pub param_data: usize,
  pub slot_words: usize,
}

const fn align4(words: usize) -> usize {
  (words + 3) & !3
}

impl Layout {
  pub fn new(inputs: usize, output_capacity: Vec<usize>, params: usize) -> Self {
    let input_counts = COUNTS;
    let output_counts = input_counts + inputs;
    let param_lengths = output_counts + output_capacity.len();
    let input_data = align4(param_lengths + params);
    let mut next = input_data + inputs * MAX_INPUT_CHANNELS * QUANTUM;
    let output_data = output_capacity
      .iter()
      .map(|channels| {
        let offset = next;
        next += channels * QUANTUM;
        offset
      })
      .collect();
    let param_data = next;
    let slot_words = align4(param_data + params * QUANTUM);
    Self {
      inputs,
      output_capacity,
      params,
      input_counts,
      output_counts,
      param_lengths,
      input_data,
      output_data,
      param_data,
      slot_words,
    }
  }

  pub fn input_channel(&self, input: usize, channel: usize) -> usize {
    self.input_data + (input * MAX_INPUT_CHANNELS + channel) * QUANTUM
  }

  pub fn output_channel(&self, output: usize, channel: usize) -> usize {
    self.output_data[output] + channel * QUANTUM
  }

  pub fn param(&self, param: usize) -> usize {
    self.param_data + param * QUANTUM
  }
}

/// Quantum slots shared by the render thread and the Worker: `read <= done <= requested <= read + SLOTS`.
pub struct Ring {
  layout: Layout,
  memory: *mut f32,
  words: usize,
  requested: AtomicU64,
  done: AtomicU64,
  read: AtomicU64,
  alive: AtomicBool,
  attached: AtomicBool,
  /// The Worker let go; offline renders stop waiting.
  detached: AtomicBool,
  closed: AtomicBool,
  underruns: AtomicU64,
  offline: Option<(Mutex<()>, Condvar)>,
  gave_up: AtomicBool,
  low_water: AtomicU64,
  window: AtomicU32,
}

// Each slot is only touched by the side the indices give it to.
unsafe impl Send for Ring {}
unsafe impl Sync for Ring {}

impl Drop for Ring {
  fn drop(&mut self) {
    // SAFETY: allocated in `Ring::new` as a boxed `[f64]` of `words / 2`.
    unsafe { drop(Box::from_raw(ptr::slice_from_raw_parts_mut(self.memory.cast::<f64>(), self.words / 2))) };
  }
}

impl Ring {
  pub fn new(layout: Layout, offline: bool) -> Self {
    let words = layout.slot_words * SLOTS;
    // f64-aligned for the frame.
    let memory = Box::into_raw(vec![0f64; words / 2].into_boxed_slice()).cast::<f32>();
    Self {
      layout,
      memory,
      words,
      requested: AtomicU64::new(0),
      done: AtomicU64::new(0),
      read: AtomicU64::new(0),
      alive: AtomicBool::new(true),
      attached: AtomicBool::new(false),
      detached: AtomicBool::new(false),
      closed: AtomicBool::new(false),
      underruns: AtomicU64::new(0),
      offline: offline.then(|| (Mutex::new(()), Condvar::new())),
      gave_up: AtomicBool::new(false),
      low_water: AtomicU64::new(u64::MAX),
      window: AtomicU32::new(0),
    }
  }

  pub fn layout(&self) -> &Layout {
    &self.layout
  }

  fn slot(&self, index: u64) -> *mut f32 {
    unsafe { self.memory.add((index % SLOTS as u64) as usize * self.layout.slot_words) }
  }

  /// `false` when every slot is taken; `params` writes a param's values and returns their count.
  pub fn push_request(
    &self,
    frame: u64,
    inputs: &[&[&[f32]]],
    output_channels: &[usize],
    params: &mut dyn FnMut(usize, &mut [f32]) -> usize,
  ) -> bool {
    let requested = self.requested.load(Ordering::Relaxed);
    let read = self.read.load(Ordering::Acquire);
    if requested - read >= SLOTS as u64 {
      return false;
    }
    let layout = &self.layout;
    let slot = self.slot(requested);
    // SAFETY: the slot is neither pending nor unplayed.
    unsafe {
      slot.add(FRAME).cast::<f64>().write(frame as f64);
      slot.add(ALIVE).cast::<u32>().write(1);
      for input in 0..layout.inputs {
        let channels = inputs.get(input).copied().unwrap_or(&[]);
        let count = channels.len().min(MAX_INPUT_CHANNELS);
        slot.add(layout.input_counts + input).cast::<u32>().write(count as u32);
        for (channel, samples) in channels.iter().take(count).enumerate() {
          let out = std::slice::from_raw_parts_mut(slot.add(layout.input_channel(input, channel)), QUANTUM);
          let n = samples.len().min(QUANTUM);
          out[..n].copy_from_slice(&samples[..n]);
          out[n..].fill(0.);
        }
      }
      for (output, capacity) in layout.output_capacity.iter().enumerate() {
        let count = output_channels.get(output).copied().unwrap_or(0).min(*capacity);
        slot.add(layout.output_counts + output).cast::<u32>().write(count as u32);
      }
      for param in 0..layout.params {
        let values = std::slice::from_raw_parts_mut(slot.add(layout.param(param)), QUANTUM);
        let length = params(param, values).clamp(1, QUANTUM);
        slot.add(layout.param_lengths + param).cast::<u32>().write(length as u32);
      }
    }
    self.requested.store(requested + 1, Ordering::Release);
    true
  }

  pub fn wait_for_responses(&self) {
    let Some((lock, ready)) = &self.offline else {
      return;
    };
    if self.gave_up.load(Ordering::Relaxed) {
      return;
    }
    let target = self.requested.load(Ordering::Relaxed);
    let pending = || self.done.load(Ordering::Acquire) < target && !self.detached.load(Ordering::Acquire);
    let guard = lock.lock().unwrap_or_else(|e| e.into_inner());
    let (_guard, result) =
      ready.wait_timeout_while(guard, OFFLINE_WAIT, |_| pending()).unwrap_or_else(|e| e.into_inner());
    if result.timed_out() && pending() {
      self.gave_up.store(true, Ordering::Relaxed);
    }
  }

  /// Plays the oldest processed response, or silence; returns the last `process()` result.
  pub fn pull_response(&self, outputs: &mut [&mut [&mut [f32]]]) -> bool {
    let mut read = self.read.load(Ordering::Relaxed);
    let done = self.done.load(Ordering::Acquire);
    let backlog = done - read;
    if backlog > MAX_BACKLOG {
      read = done - MAX_BACKLOG;
    } else if self.offline.is_none() {
      // Shrink a backlog left by a stall.
      let low = self.low_water.load(Ordering::Relaxed).min(backlog);
      let window = self.window.load(Ordering::Relaxed) + 1;
      if window >= SHRINK_WINDOW {
        if low >= 2 {
          read += ((low - 1) / 2).max(1);
        }
        self.window.store(0, Ordering::Relaxed);
        self.low_water.store(u64::MAX, Ordering::Relaxed);
      } else {
        self.window.store(window, Ordering::Relaxed);
        self.low_water.store(low, Ordering::Relaxed);
      }
    }
    if done > read {
      let layout = &self.layout;
      let slot = self.slot(read);
      // SAFETY: processed and not yet given back for writing.
      unsafe {
        for (output, channels) in outputs.iter_mut().enumerate() {
          let count = if output < layout.output_capacity.len() {
            slot.add(layout.output_counts + output).cast::<u32>().read() as usize
          } else {
            0
          };
          for (channel, samples) in channels.iter_mut().enumerate() {
            if channel < count {
              let n = samples.len().min(QUANTUM);
              let from = std::slice::from_raw_parts(slot.add(layout.output_channel(output, channel)), n);
              samples[..n].copy_from_slice(from);
            } else {
              samples.fill(0.);
            }
          }
        }
        self.alive.store(slot.add(ALIVE).cast::<u32>().read() != 0, Ordering::Relaxed);
      }
      self.read.store(read + 1, Ordering::Release);
    } else {
      outputs.iter_mut().flat_map(|channels| channels.iter_mut()).for_each(|samples| samples.fill(0.));
      if self.attached.load(Ordering::Relaxed) {
        self.underruns.fetch_add(1, Ordering::Relaxed);
      }
      self.read.store(read, Ordering::Release);
    }
    self.alive.load(Ordering::Relaxed)
  }

  pub fn pending(&self) -> u32 {
    (self.requested.load(Ordering::Acquire) - self.done.load(Ordering::Relaxed)) as u32
  }

  pub fn cursor(&self) -> u32 {
    (self.done.load(Ordering::Relaxed) % SLOTS as u64) as u32
  }

  pub fn commit(&self, count: u32) {
    let pending = self.pending();
    self.done.fetch_add(count.min(pending) as u64, Ordering::Release);
    self.notify_offline();
  }

  pub fn attach(&self) {
    self.attached.store(true, Ordering::Relaxed);
  }

  pub fn detach(&self) {
    self.detached.store(true, Ordering::Release);
    self.notify_offline();
  }

  fn notify_offline(&self) {
    if let Some((lock, ready)) = &self.offline {
      drop(lock.lock().unwrap_or_else(|e| e.into_inner()));
      ready.notify_all();
    }
  }

  pub fn closed(&self) -> bool {
    self.closed.load(Ordering::Acquire)
  }

  pub fn underruns(&self) -> u64 {
    self.underruns.load(Ordering::Relaxed)
  }
}

/// A context's worklet scope: its nodes and the Worker's wakeup.
pub struct Scope {
  id: u32,
  nodes: Mutex<HashMap<u32, Weak<Ring>>>,
  next_node: AtomicU32,
  /// A wakeup is on its way and the Worker hasn't started draining.
  queued: AtomicBool,
  kick: AtomicBool,
  notifier: OnceLock<Thread>,
}

fn scopes() -> &'static Mutex<HashMap<u32, Weak<Scope>>> {
  static SCOPES: OnceLock<Mutex<HashMap<u32, Weak<Scope>>>> = OnceLock::new();
  SCOPES.get_or_init(Default::default)
}

impl Scope {
  pub fn new() -> Arc<Self> {
    static NEXT: AtomicU32 = AtomicU32::new(1);
    let scope = Arc::new(Self {
      id: NEXT.fetch_add(1, Ordering::Relaxed),
      nodes: Mutex::default(),
      next_node: AtomicU32::new(1),
      queued: AtomicBool::new(false),
      kick: AtomicBool::new(false),
      notifier: OnceLock::new(),
    });
    let mut all = scopes().lock().unwrap_or_else(|e| e.into_inner());
    all.retain(|_, scope| scope.strong_count() > 0);
    all.insert(scope.id, Arc::downgrade(&scope));
    scope
  }

  pub fn find(id: u32) -> Option<Arc<Self>> {
    scopes().lock().unwrap_or_else(|e| e.into_inner()).get(&id).and_then(Weak::upgrade)
  }

  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn add(&self, ring: &Arc<Ring>) -> u32 {
    let id = self.next_node.fetch_add(1, Ordering::Relaxed);
    let mut nodes = self.nodes.lock().unwrap_or_else(|e| e.into_inner());
    nodes.retain(|_, ring| ring.strong_count() > 0);
    nodes.insert(id, Arc::downgrade(ring));
    id
  }

  pub fn node(&self, id: u32) -> Option<Arc<Ring>> {
    self.nodes.lock().unwrap_or_else(|e| e.into_inner()).get(&id).and_then(Weak::upgrade)
  }

  fn rings(&self) -> Vec<Arc<Ring>> {
    self.nodes.lock().unwrap_or_else(|e| e.into_inner()).values().filter_map(Weak::upgrade).collect()
  }

  /// Lock-free, for the render thread.
  pub fn wake(&self) {
    if !self.queued.swap(true, Ordering::AcqRel) {
      if let Some(notifier) = self.notifier.get() {
        self.kick.store(true, Ordering::Release);
        notifier.unpark();
      }
    }
  }

  pub fn begin_drain(&self) {
    self.queued.store(false, Ordering::Release);
  }

  /// Runs `call` on each wakeup until `stop`; joining returns `call` so the caller drops it.
  pub fn start_notifier<C: FnMut() + Send + 'static>(
    self: &Arc<Self>,
    stop: Arc<AtomicBool>,
    mut call: C,
  ) -> Option<JoinHandle<C>> {
    let scope = self.clone();
    let thread_stop = stop.clone();
    let handle = thread::Builder::new()
      .name("audio-worklet-notifier".into())
      .spawn(move || {
        loop {
          thread::park();
          if thread_stop.load(Ordering::Acquire) {
            break;
          }
          if scope.kick.swap(false, Ordering::AcqRel) {
            call();
          }
        }
        call
      })
      .ok()?;
    if self.notifier.set(handle.thread().clone()).is_err() {
      stop.store(true, Ordering::Release);
      handle.thread().unpark();
      let _ = handle.join();
      return None;
    }
    // Serve requests made before the Worker attached.
    self.queued.store(false, Ordering::Release);
    self.wake();
    Some(handle)
  }
}

thread_local! {
  /// web-audio-api asks the processor type for its descriptors; these are the node being made's.
  static DESCRIPTORS: RefCell<Vec<AudioParamDescriptor>> = const { RefCell::new(Vec::new()) };
}

pub fn with_descriptors<T>(descriptors: Vec<AudioParamDescriptor>, make: impl FnOnce() -> T) -> T {
  DESCRIPTORS.with(|pending| *pending.borrow_mut() = descriptors);
  let made = make();
  DESCRIPTORS.with(|pending| pending.borrow_mut().clear());
  made
}

pub struct ProcessorOptions {
  pub ring: Arc<Ring>,
  pub scope: Arc<Scope>,
  pub param_names: Vec<String>,
}

pub struct JsProcessor {
  options: ProcessorOptions,
  output_channels: Vec<usize>,
}

impl AudioWorkletProcessor for JsProcessor {
  type ProcessorOptions = ProcessorOptions;

  fn constructor(options: ProcessorOptions) -> Self {
    let outputs = options.ring.layout.output_capacity.len();
    Self { options, output_channels: vec![0; outputs] }
  }

  fn parameter_descriptors() -> Vec<AudioParamDescriptor> {
    DESCRIPTORS.with(|pending| pending.borrow().clone())
  }

  fn process<'a, 'b>(
    &mut self,
    inputs: &'b [&'a [&'a [f32]]],
    outputs: &'b mut [&'a mut [&'a mut [f32]]],
    params: AudioParamValues<'b>,
    scope: &'b AudioWorkletGlobalScope,
  ) -> bool {
    let ProcessorOptions { ring, scope: worklet, param_names } = &self.options;
    for (count, output) in self.output_channels.iter_mut().zip(outputs.iter()) {
      *count = output.len();
    }
    let pushed = ring.push_request(scope.current_frame, inputs, &self.output_channels, &mut |param, values| {
      let computed = params.get(&param_names[param]);
      let n = computed.len().min(values.len());
      values[..n].copy_from_slice(&computed[..n]);
      n
    });
    if pushed {
      worklet.wake();
      ring.wait_for_responses();
    }
    ring.pull_response(outputs)
  }
}

impl Drop for JsProcessor {
  fn drop(&mut self) {
    self.options.ring.closed.store(true, Ordering::Release);
    self.options.scope.wake();
  }
}

pub fn output_capacity(inputs: usize, outputs: usize, output_channel_count: &[usize]) -> Vec<usize> {
  if !output_channel_count.is_empty() {
    output_channel_count.to_vec()
  } else if inputs == 1 && outputs == 1 {
    vec![MAX_DYNAMIC_OUTPUT_CHANNELS]
  } else {
    vec![1; outputs]
  }
}

pub fn automation_rate(value: Option<&str>) -> Result<AutomationRate> {
  match value.unwrap_or("a-rate") {
    "a-rate" => Ok(AutomationRate::A),
    "k-rate" => Ok(AutomationRate::K),
    other => Err(error(format!("TypeError: '{other}' is not a valid AutomationRate"))),
  }
}

#[napi]
pub struct AudioWorkletScope {
  pub(crate) scope: Arc<Scope>,
}

#[napi]
impl AudioWorkletScope {
  #[napi(getter)]
  pub fn id(&self) -> u32 {
    self.scope.id()
  }
}

type WakeCallback<'a> = Function<'a, (), UnknownReturnValue>;

type Stopper = Box<dyn FnOnce() + Send>;

struct Attachment {
  scope: Arc<Scope>,
  stop: Mutex<Option<Stopper>>,
}

impl Attachment {
  /// Idempotent; the runtimes don't finalize live threadsafe functions when a Worker's env goes.
  fn shutdown(&self) {
    let stop = self.stop.lock().unwrap_or_else(|e| e.into_inner()).take();
    if let Some(stop) = stop {
      stop();
    }
    for ring in self.scope.rings() {
      ring.detach();
    }
  }
}

/// No runtime sets Worker priorities; Linux would need privileges.
fn raise_thread_priority() {
  #[cfg(windows)]
  {
    #[link(name = "kernel32")]
    extern "system" {
      fn GetCurrentThread() -> *mut c_void;
      fn SetThreadPriority(thread: *mut c_void, priority: i32) -> i32;
    }
    const THREAD_PRIORITY_HIGHEST: i32 = 2;
    // SAFETY: the pseudo handle of the calling thread.
    unsafe { SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_HIGHEST) };
  }
}

/// The Linux runtime only runs Worker scripts from files.
#[napi]
pub fn audio_worklet_script(source: String) -> Result<String> {
  use std::hash::{Hash, Hasher};
  let mut hasher = std::collections::hash_map::DefaultHasher::new();
  source.hash(&mut hasher);
  let path = std::env::temp_dir().join(format!("nativescript-audioworklet-{:016x}.js", hasher.finish()));
  if std::fs::read_to_string(&path).ok().as_deref() != Some(source.as_str()) {
    let partial = path.with_extension(format!("{}.tmp", std::process::id()));
    std::fs::write(&partial, &source)
      .and_then(|_| std::fs::rename(&partial, &path))
      .map_err(|e| error(format!("NotSupportedError: cannot write the AudioWorklet script ({e})")))?;
  }
  Ok(path.to_string_lossy().into_owned())
}

pub struct ModuleTask {
  source: String,
}

fn percent_decode(text: &str) -> Vec<u8> {
  let bytes = text.as_bytes();
  let mut out = Vec::with_capacity(bytes.len());
  let mut i = 0;
  while i < bytes.len() {
    let hex = |b: u8| (b as char).to_digit(16);
    if bytes[i] == b'%' && i + 2 < bytes.len() {
      if let (Some(hi), Some(lo)) = (hex(bytes[i + 1]), hex(bytes[i + 2])) {
        out.push((hi * 16 + lo) as u8);
        i += 3;
        continue;
      }
    }
    out.push(bytes[i]);
    i += 1;
  }
  out
}

pub fn read_module(source: &str) -> std::result::Result<String, String> {
  use base64::Engine as _;
  if let Some(rest) = source.strip_prefix("data:") {
    let (meta, payload) = rest.split_once(',').ok_or("the data: URL has no comma")?;
    let bytes = if meta.ends_with(";base64") {
      base64::engine::general_purpose::STANDARD
        .decode(percent_decode(payload.trim()))
        .map_err(|e| format!("invalid base64 ({e})"))?
    } else {
      percent_decode(payload)
    };
    return String::from_utf8(bytes).map_err(|e| e.to_string());
  }
  std::fs::read_to_string(source).map_err(|e| format!("{source}: {e}"))
}

impl napi::Task for ModuleTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<String> {
    read_module(&self.source).map_err(|e| error(format!("AbortError: unable to load the AudioWorklet module ({e})")))
  }

  fn resolve(&mut self, _: Env, text: String) -> Result<String> {
    Ok(text)
  }
}

#[napi(ts_return_type = "Promise<string>")]
pub fn read_audio_worklet_module(source: String) -> napi::bindgen_prelude::AsyncTask<ModuleTask> {
  napi::bindgen_prelude::AsyncTask::new(ModuleTask { source })
}

#[napi]
pub struct AudioWorkletBridge {
  attachment: Arc<Attachment>,
}

#[napi]
impl AudioWorkletBridge {
  /// `wake` should call `beginDrain()` before serving the rings.
  #[napi(factory, ts_args_type = "scopeId: number, wake: () => void")]
  pub fn attach(env: Env, scope_id: u32, wake: WakeCallback) -> Result<Self> {
    let scope = Scope::find(scope_id).ok_or_else(|| error("InvalidStateError: the AudioWorklet's context is gone"))?;
    let tsfn = wake.build_threadsafe_function::<()>().weak::<true>().build_callback(|_| Ok(()))?;
    let stop = Arc::new(AtomicBool::new(false));
    let notifier = scope
      .start_notifier(stop.clone(), move || {
        tsfn.call((), ThreadsafeFunctionCallMode::NonBlocking);
      })
      .ok_or_else(|| error("InvalidStateError: the AudioWorklet's scope is attached already"))?;
    raise_thread_priority();
    let stopper: Stopper = Box::new(move || {
      stop.store(true, Ordering::Release);
      notifier.thread().unpark();
      // Drops the threadsafe function on the Worker's thread.
      drop(notifier.join());
    });
    let attachment = Arc::new(Attachment { scope, stop: Mutex::new(Some(stopper)) });
    let hook = attachment.clone();
    env.add_env_cleanup_hook(hook, |attachment| attachment.shutdown())?;
    Ok(Self { attachment })
  }

  #[napi]
  pub fn begin_drain(&self) {
    self.attachment.scope.begin_drain();
  }

  #[napi]
  pub fn node(&self, id: u32) -> Option<AudioWorkletRing> {
    self.attachment.scope.node(id).map(|ring| {
      ring.attach();
      AudioWorkletRing { ring }
    })
  }

  /// Call before the Worker is terminated.
  #[napi]
  pub fn close(&self) {
    self.attachment.shutdown();
  }
}

#[napi(object)]
pub struct RingLayout {
  pub slots: u32,
  pub quantum: u32,
  pub slot_words: u32,
  pub max_input_channels: u32,
  pub inputs: u32,
  pub alive: u32,
  pub input_counts: u32,
  pub output_counts: u32,
  pub param_lengths: u32,
  pub input_data: u32,
  pub output_data: Vec<u32>,
  pub output_capacity: Vec<u32>,
  pub param_data: u32,
}

#[napi]
pub struct AudioWorkletRing {
  ring: Arc<Ring>,
}

unsafe extern "C" fn release_ring(_env: sys::napi_env, _data: *mut c_void, hint: *mut c_void) {
  drop(Box::from_raw(hint.cast::<Arc<Ring>>()));
}

#[napi]
impl AudioWorkletRing {
  #[napi(ts_return_type = "ArrayBuffer")]
  pub fn buffer<'env>(&self, env: &'env Env) -> Result<Unknown<'env>> {
    let hint = Box::into_raw(Box::new(self.ring.clone()));
    let mut value = ptr::null_mut();
    // SAFETY: the finalizer hint keeps the ring alive.
    let status = unsafe {
      sys::napi_create_external_arraybuffer(
        env.raw(),
        self.ring.memory.cast(),
        self.ring.words * 4,
        Some(release_ring),
        hint.cast(),
        &mut value,
      )
    };
    if status != sys::Status::napi_ok {
      drop(unsafe { Box::from_raw(hint) });
      return Err(error(format!("NotSupportedError: no external ArrayBuffer (status {status})")));
    }
    Ok(unsafe { Unknown::from_raw_unchecked(env.raw(), value) })
  }

  #[napi]
  pub fn layout(&self) -> RingLayout {
    let layout = self.ring.layout();
    let word = |value: usize| value as u32;
    RingLayout {
      slots: SLOTS as u32,
      quantum: QUANTUM as u32,
      slot_words: word(layout.slot_words),
      max_input_channels: MAX_INPUT_CHANNELS as u32,
      inputs: word(layout.inputs),
      alive: ALIVE as u32,
      input_counts: word(layout.input_counts),
      output_counts: word(layout.output_counts),
      param_lengths: word(layout.param_lengths),
      input_data: word(layout.input_data),
      output_data: layout.output_data.iter().copied().map(word).collect(),
      output_capacity: layout.output_capacity.iter().copied().map(word).collect(),
      param_data: word(layout.param_data),
    }
  }

  #[napi]
  pub fn pending(&self) -> u32 {
    self.ring.pending()
  }

  #[napi(getter)]
  pub fn cursor(&self) -> u32 {
    self.ring.cursor()
  }

  #[napi]
  pub fn commit(&self, count: u32) {
    self.ring.commit(count);
  }

  #[napi(getter)]
  pub fn closed(&self) -> bool {
    self.ring.closed()
  }

  #[napi(getter)]
  pub fn underruns(&self) -> f64 {
    self.ring.underruns() as f64
  }

  #[napi]
  pub fn release(&self) {
    self.ring.detach();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn ring(inputs: usize, outputs: Vec<usize>, params: usize, offline: bool) -> Ring {
    Ring::new(Layout::new(inputs, outputs, params), offline)
  }

  /// Plays the Worker's part.
  fn serve(ring: &Ring, mut process: impl FnMut(f64, &[&[f32]], &[&[f32]], &mut [&mut [f32]]) -> bool) -> u32 {
    let pending = ring.pending();
    let layout = ring.layout().clone();
    for k in 0..pending {
      let slot = ring.slot(ring.done.load(Ordering::Relaxed) + k as u64);
      unsafe {
        let frame = slot.cast::<f64>().read();
        let counts = slot.add(layout.input_counts).cast::<u32>().read() as usize;
        let inputs: Vec<&[f32]> = (0..if layout.inputs > 0 { counts } else { 0 })
          .map(|c| std::slice::from_raw_parts(slot.add(layout.input_channel(0, c)), QUANTUM))
          .collect();
        let params: Vec<&[f32]> = (0..layout.params)
          .map(|p| {
            let n = slot.add(layout.param_lengths + p).cast::<u32>().read() as usize;
            std::slice::from_raw_parts(slot.add(layout.param(p)), n)
          })
          .collect();
        let out_count = slot.add(layout.output_counts).cast::<u32>().read() as usize;
        let mut outputs: Vec<&mut [f32]> = (0..out_count)
          .map(|c| std::slice::from_raw_parts_mut(slot.add(layout.output_channel(0, c)), QUANTUM))
          .collect();
        let alive = process(frame, &inputs, &params, &mut outputs);
        slot.add(ALIVE).cast::<u32>().write(alive as u32);
      }
    }
    ring.commit(pending);
    pending
  }

  fn quantum(ring: &Ring, frame: u64, input: &[f32], param: f32) -> (bool, Vec<f32>) {
    let input_channels: [&[f32]; 1] = [input];
    let inputs: [&[&[f32]]; 1] = [&input_channels];
    assert!(ring.push_request(frame, &inputs[..ring.layout().inputs], &[1], &mut |_, values| {
      values[0] = param;
      1
    }));
    ring.wait_for_responses();
    let mut channel = vec![9f32; QUANTUM];
    let alive = {
      let mut channels: [&mut [f32]; 1] = [&mut channel];
      let mut outputs: [&mut [&mut [f32]]; 1] = [&mut channels];
      ring.pull_response(&mut outputs)
    };
    (alive, channel)
  }

  #[test]
  fn reads_module_sources() {
    assert_eq!(
      read_module("data:text/javascript,registerProcessor(%22a%22%2C%20A)").unwrap(),
      "registerProcessor(\"a\", A)"
    );
    assert_eq!(read_module("data:text/javascript;base64,cmVnaXN0ZXI=").unwrap(), "register");
    assert_eq!(read_module("data:,100%").unwrap(), "100%");
    assert!(read_module("/no/such/worklet.js").is_err());
    let path = audio_worklet_script("// scope".into()).unwrap();
    assert_eq!(read_module(&path).unwrap(), "// scope");
    assert_eq!(audio_worklet_script("// scope".into()).unwrap(), path);
  }

  #[test]
  fn layout_aligns_slots_for_the_f64_frame() {
    let layout = Layout::new(1, vec![2], 2);
    assert_eq!(layout.input_counts, 4);
    assert_eq!(layout.output_counts, 5);
    assert_eq!(layout.param_lengths, 6);
    assert_eq!(layout.input_data, 8);
    assert_eq!(layout.output_data, vec![8 + MAX_INPUT_CHANNELS * QUANTUM]);
    assert_eq!(layout.param_data, layout.output_data[0] + 2 * QUANTUM);
    assert_eq!(layout.slot_words % 4, 0);
    assert_eq!(Layout::new(0, vec![1], 0).slot_words, 8 + QUANTUM);
  }

  #[test]
  fn plays_silence_until_the_worker_responds_then_one_burst_late() {
    let ring = ring(0, vec![1], 1, false);
    for q in 0..4 {
      let (alive, out) = quantum(&ring, q * 128, &[], q as f32);
      assert!(alive);
      assert!(out.iter().all(|v| *v == 0.));
    }
    assert_eq!(ring.pending(), 4);
    assert_eq!(
      serve(&ring, |frame, _, params, outputs| {
        outputs[0].fill(frame as f32 + params[0][0]);
        true
      }),
      4
    );
    // The next burst plays the previous one.
    for q in 4..8 {
      let (_, out) = quantum(&ring, q * 128, &[], q as f32);
      let request = q - 4;
      assert!(out.iter().all(|v| *v == (request * 128 + request) as f32), "quantum {q}: {}", out[0]);
    }
    assert_eq!(ring.pending(), 4);
    assert_eq!(ring.read.load(Ordering::Relaxed), 4);
  }

  #[test]
  fn passes_inputs_and_a_rate_params() {
    let ring = ring(1, vec![1], 1, true);
    let input: Vec<f32> = (0..QUANTUM).map(|i| i as f32).collect();
    let inputs_channels: [&[f32]; 1] = [&input];
    let inputs: [&[&[f32]]; 1] = [&inputs_channels];
    let ramp: Vec<f32> = (0..QUANTUM).map(|i| 1000. + i as f32).collect();
    assert!(ring.push_request(0, &inputs, &[1], &mut |_, values| {
      values.copy_from_slice(&ramp);
      QUANTUM
    }));
    serve(&ring, |_, inputs, params, outputs| {
      assert_eq!(params[0].len(), QUANTUM);
      for i in 0..QUANTUM {
        outputs[0][i] = inputs[0][i] + params[0][i];
      }
      true
    });
    let mut out = vec![0f32; QUANTUM];
    let mut channels: [&mut [f32]; 1] = [&mut out];
    let mut outputs: [&mut [&mut [f32]]; 1] = [&mut channels];
    ring.pull_response(&mut outputs);
    assert!((0..QUANTUM).all(|i| out[i] == 1000. + 2. * i as f32));
  }

  #[test]
  fn offline_waits_for_each_response() {
    let ring = Arc::new(ring(0, vec![1], 0, true));
    ring.attach();
    let worker = {
      let ring = ring.clone();
      thread::spawn(move || {
        let mut served = 0;
        while served < 10 {
          served += serve(&ring, |frame, _, _, outputs| {
            outputs[0].fill(frame as f32);
            true
          });
          thread::yield_now();
        }
      })
    };
    for q in 0..10u64 {
      let (_, out) = quantum(&ring, q * 128, &[], 0.);
      assert_eq!(out[0], (q * 128) as f32, "no added latency offline");
    }
    worker.join().unwrap();
    assert_eq!(ring.underruns(), 0);
  }

  #[test]
  fn offline_gives_up_on_a_detached_worker() {
    let ring = ring(0, vec![1], 0, true);
    ring.detach();
    let (alive, out) = quantum(&ring, 0, &[], 0.);
    assert!(alive && out[0] == 0.);
  }

  #[test]
  fn process_returning_false_reaches_the_render_thread() {
    let ring = ring(0, vec![1], 0, false);
    quantum(&ring, 0, &[], 0.);
    serve(&ring, |_, _, _, _| false);
    let (alive, _) = quantum(&ring, 128, &[], 0.);
    assert!(!alive);
  }

  #[test]
  fn a_stalled_worker_fills_the_ring_then_requests_stop() {
    let ring = ring(0, vec![1], 0, false);
    for q in 0..SLOTS as u64 {
      quantum(&ring, q * 128, &[], 0.);
    }
    assert_eq!(ring.pending() as usize, SLOTS);
    assert!(!ring.push_request(0, &[], &[1], &mut |_, _| 1));
  }

  #[test]
  fn caps_the_backlog_and_shrinks_it_after_a_stall() {
    let ring = ring(0, vec![1], 0, false);
    ring.attach();
    for q in 0..2000u64 {
      if q == 100 {
        for s in 0..20 {
          quantum(&ring, (q + s) * 128, &[], 0.);
        }
      }
      quantum(&ring, q * 128, &[], 0.);
      serve(&ring, |frame, _, _, outputs| {
        outputs[0].fill(frame as f32 / 128.);
        true
      });
    }
    let backlog = ring.done.load(Ordering::Relaxed) - ring.read.load(Ordering::Relaxed);
    assert!(backlog <= 2, "backlog {backlog}");
    assert!(ring.underruns() >= 19);
  }

  #[test]
  fn param_descriptors_come_from_the_node_being_made() {
    let descriptors = vec![AudioParamDescriptor {
      name: "rpm".into(),
      automation_rate: AutomationRate::K,
      default_value: 0.,
      min_value: 0.,
      max_value: 12000.,
    }];
    let made = with_descriptors(descriptors, JsProcessor::parameter_descriptors);
    assert_eq!(made.len(), 1);
    assert_eq!(made[0].name, "rpm");
    assert!(JsProcessor::parameter_descriptors().is_empty());
  }

  #[test]
  fn wakes_the_notifier_once_per_drain() {
    let scope = Scope::new();
    assert!(Arc::ptr_eq(&Scope::find(scope.id()).unwrap(), &scope));
    let calls = Arc::new(AtomicU32::new(0));
    let stop = Arc::new(AtomicBool::new(false));
    let counter = calls.clone();
    let notifier = scope
      .start_notifier(stop.clone(), move || {
        counter.fetch_add(1, Ordering::SeqCst);
      })
      .unwrap();
    let wait_for = |n: u32| {
      for _ in 0..1000 {
        if calls.load(Ordering::SeqCst) >= n {
          return;
        }
        thread::sleep(Duration::from_millis(1));
      }
      panic!("no wakeup {n}");
    };
    wait_for(1);
    scope.begin_drain();
    scope.wake();
    scope.wake();
    scope.wake();
    wait_for(2);
    thread::sleep(Duration::from_millis(20));
    assert_eq!(calls.load(Ordering::SeqCst), 2, "coalesced until the drain begins");
    scope.begin_drain();
    scope.wake();
    wait_for(3);
    assert!(scope.start_notifier(Arc::new(AtomicBool::new(false)), || {}).is_none());
    stop.store(true, Ordering::Release);
    notifier.thread().unpark();
    drop(notifier.join().unwrap());
  }

  #[test]
  fn bridges_a_processor_in_an_offline_graph() {
    use web_audio_api::context::{BaseAudioContext, OfflineAudioContext};
    use web_audio_api::node::AudioNode;
    use web_audio_api::worklet::{AudioWorkletNode, AudioWorkletNodeOptions};

    let context = OfflineAudioContext::new(2, 128 * 8, 48000.);
    let scope = Scope::new();
    let ring = Arc::new(Ring::new(Layout::new(0, vec![2], 1), true));
    scope.add(&ring);
    let descriptors = vec![AudioParamDescriptor {
      name: "level".into(),
      automation_rate: AutomationRate::K,
      default_value: 0.25,
      min_value: 0.,
      max_value: 1.,
    }];
    let options = AudioWorkletNodeOptions {
      number_of_inputs: 0,
      number_of_outputs: 1,
      output_channel_count: vec![2],
      parameter_data: HashMap::new(),
      processor_options: ProcessorOptions {
        ring: ring.clone(),
        scope: scope.clone(),
        param_names: vec!["level".into()],
      },
      audio_node_options: Default::default(),
    };
    let node = with_descriptors(descriptors, || AudioWorkletNode::new::<JsProcessor>(&context, options));
    node.parameters()["level"].set_value_at_time(0.5, 128. * 4. / 48000.);
    node.connect(&context.destination());

    let stop = Arc::new(AtomicBool::new(false));
    let (wakes, woken) = std::sync::mpsc::channel::<()>();
    let notifier = scope
      .start_notifier(stop.clone(), move || {
        let _ = wakes.send(());
      })
      .unwrap();
    ring.attach();
    let worker = {
      let (ring, scope) = (ring.clone(), scope.clone());
      thread::spawn(move || {
        while woken.recv().is_ok() {
          scope.begin_drain();
          serve(&ring, |frame, _, params, outputs| {
            outputs[0].fill(params[0][0]);
            outputs[1].fill(frame as f32);
            true
          });
        }
      })
    };
    let mut context = context;
    let rendered = context.start_rendering_sync();
    stop.store(true, Ordering::Release);
    notifier.thread().unpark();
    drop(notifier.join());
    worker.join().unwrap();

    let level = rendered.get_channel_data(0);
    let frames = rendered.get_channel_data(1);
    assert_eq!(level[0], 0.25);
    assert_eq!(level[128 * 4], 0.5);
    for q in 0..8 {
      assert_eq!(frames[q * 128], (q * 128) as f32, "quantum {q} plays its own request");
    }
  }
}
