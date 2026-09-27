use std::io::Cursor;

use base64::Engine as _;
use std::sync::{Arc, Mutex};

use napi::bindgen_prelude::{AsyncTask, Either, Float32ArraySlice, Function, Uint8ArraySlice};
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::{Env, Result, Task};
use napi_derive::napi;
use web_audio_api::context::{
  AudioContextLatencyCategory, AudioContextOptions, AudioContextState, BaseAudioContext, ConcreteBaseAudioContext,
  OfflineAudioContext,
};
use web_audio_api::PeriodicWaveOptions;

use crate::buffer::{AudioBuffer, PeriodicWave};
use crate::node::{AudioNode, Kind};
use crate::param::AudioParam;
use crate::{error, guard};

#[napi(object)]
#[derive(Default)]
pub struct ContextOptions {
  /// Hz; the output device's rate when unset.
  pub sample_rate: Option<f64>,
  /// `'interactive' | 'balanced' | 'playback'` or seconds.
  pub latency_hint: Option<Either<String, f64>>,
  /// An output device id, `''` for the default device or `'none'` to render without one.
  pub sink_id: Option<String>,
}

enum Backing {
  Online(Arc<web_audio_api::context::AudioContext>),
  /// Taken by `startRendering`.
  Offline(Mutex<Option<OfflineAudioContext>>, usize),
}

type StateChangeCallback<'a> = Function<'a, (), UnknownReturnValue>;

/// An `AudioContext` or an `OfflineAudioContext`. Nodes are made on the shared base, so both
/// kinds build graphs the same way.
#[napi]
pub struct AudioContext {
  base: ConcreteBaseAudioContext,
  backing: Backing,
}

impl AudioContext {
  fn online(&self) -> Result<&Arc<web_audio_api::context::AudioContext>> {
    match &self.backing {
      Backing::Online(context) => Ok(context),
      Backing::Offline(..) => Err(error("InvalidStateError: not supported by an OfflineAudioContext")),
    }
  }

  fn node(&self, make: impl FnOnce(&ConcreteBaseAudioContext) -> Kind) -> Result<AudioNode> {
    guard(|| AudioNode::new(make(&self.base)))
  }
}

#[napi]
impl AudioContext {
  /// A realtime context on an output device (WASAPI).
  #[napi(constructor)]
  pub fn new(options: Option<ContextOptions>) -> Result<Self> {
    let options = options.unwrap_or_default();
    let latency_hint = match options.latency_hint {
      Some(Either::A(hint)) => match hint.as_str() {
        "balanced" => AudioContextLatencyCategory::Balanced,
        "playback" => AudioContextLatencyCategory::Playback,
        _ => AudioContextLatencyCategory::Interactive,
      },
      Some(Either::B(seconds)) if seconds > 0.0 => AudioContextLatencyCategory::Custom(seconds),
      _ => AudioContextLatencyCategory::Interactive,
    };
    let options = AudioContextOptions {
      latency_hint,
      sample_rate: options.sample_rate.filter(|rate| *rate > 0.0).map(|rate| rate as f32),
      sink_id: options.sink_id.unwrap_or_default(),
      ..Default::default()
    };
    let context = guard(|| web_audio_api::context::AudioContext::try_new(options))?
      .map_err(|e| error(format!("NotSupportedError: no audio output ({e})")))?;
    Ok(Self {
      base: context.base().clone(),
      backing: Backing::Online(Arc::new(context)),
    })
  }

  #[napi(factory)]
  pub fn offline(number_of_channels: u32, length: u32, sample_rate: f64) -> Result<Self> {
    let context = guard(|| OfflineAudioContext::new(number_of_channels as usize, length as usize, sample_rate as f32))?;
    Ok(Self {
      base: context.base().clone(),
      backing: Backing::Offline(Mutex::new(Some(context)), length as usize),
    })
  }

  #[napi(getter)]
  pub fn sample_rate(&self) -> f64 {
    self.base.sample_rate() as f64
  }

  #[napi(getter)]
  pub fn current_time(&self) -> f64 {
    self.base.current_time()
  }

  #[napi(getter)]
  pub fn state(&self) -> &'static str {
    match self.base.state() {
      AudioContextState::Suspended => "suspended",
      AudioContextState::Running => "running",
      AudioContextState::Closed => "closed",
    }
  }

  #[napi(getter)]
  pub fn base_latency(&self) -> f64 {
    self.online().map(|context| context.base_latency()).unwrap_or(0.0)
  }

  #[napi(getter)]
  pub fn output_latency(&self) -> f64 {
    self.online().map(|context| context.output_latency()).unwrap_or(0.0)
  }

  #[napi(getter)]
  pub fn sink_id(&self) -> String {
    self.online().map(|context| context.sink_id()).unwrap_or_default()
  }

  /// Offline contexts: the length of the rendered buffer, in frames.
  #[napi(getter)]
  pub fn length(&self) -> u32 {
    match &self.backing {
      Backing::Offline(_, length) => *length as u32,
      Backing::Online(_) => 0,
    }
  }

  /// `callback` runs on the JS thread after each state change; `null` clears it.
  #[napi(ts_args_type = "callback: (() => void) | null")]
  pub fn set_onstatechange(&self, callback: Option<StateChangeCallback>) -> Result<()> {
    match callback {
      Some(callback) => {
        let tsfn = callback
          .build_threadsafe_function::<()>()
          .weak::<true>()
          .build_callback(|_| Ok(()))?;
        self.base.set_onstatechange(move |_| {
          tsfn.call((), ThreadsafeFunctionCallMode::NonBlocking);
        });
      }
      None => self.base.clear_onstatechange(),
    }
    Ok(())
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn resume(&self) -> Result<AsyncTask<StateTask>> {
    Ok(AsyncTask::new(StateTask { context: self.online()?.clone(), change: StateChange::Resume }))
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn suspend(&self) -> Result<AsyncTask<StateTask>> {
    Ok(AsyncTask::new(StateTask { context: self.online()?.clone(), change: StateChange::Suspend }))
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn close(&self) -> Result<AsyncTask<StateTask>> {
    Ok(AsyncTask::new(StateTask { context: self.online()?.clone(), change: StateChange::Close }))
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn set_sink_id(&self, sink_id: String) -> Result<AsyncTask<StateTask>> {
    Ok(AsyncTask::new(StateTask { context: self.online()?.clone(), change: StateChange::Sink(sink_id) }))
  }

  /// Renders the graph once, off the JS thread.
  #[napi(ts_return_type = "Promise<AudioBuffer>")]
  pub fn start_rendering(&self) -> Result<AsyncTask<RenderTask>> {
    let Backing::Offline(context, _) = &self.backing else {
      return Err(error("InvalidStateError: only an OfflineAudioContext renders"));
    };
    let context = context.lock().ok().and_then(|mut context| context.take());
    match context {
      Some(context) => Ok(AsyncTask::new(RenderTask { context: Some(context) })),
      None => Err(error("InvalidStateError: startRendering can only be called once")),
    }
  }

  /// Decodes a complete encoded file (wav, mp3, ogg/vorbis, flac, aac, ...) to the context's
  /// sample rate, off the JS thread.
  #[napi(ts_return_type = "Promise<AudioBuffer>")]
  pub fn decode_audio_data(&self, data: Uint8ArraySlice) -> AsyncTask<DecodeTask> {
    AsyncTask::new(DecodeTask { base: self.base.clone(), source: DecodeSource::Bytes(data.to_vec()) })
  }

  /// As `decodeAudioData`, from base64 or a base64 `data:` URL.
  #[napi(ts_return_type = "Promise<AudioBuffer>")]
  pub fn decode_audio_base64(&self, data: String) -> AsyncTask<DecodeTask> {
    AsyncTask::new(DecodeTask { base: self.base.clone(), source: DecodeSource::Base64(data) })
  }

  #[napi(ts_return_type = "Promise<AudioBuffer>")]
  pub fn decode_audio_file(&self, path: String) -> AsyncTask<DecodeTask> {
    AsyncTask::new(DecodeTask { base: self.base.clone(), source: DecodeSource::File(path) })
  }

  #[napi]
  pub fn destination(&self) -> Result<AudioNode> {
    self.node(|base| Kind::Destination(base.destination()))
  }

  /// One of the listener's `AudioParam`s by its JS name (`positionX`, `forwardY`, `upZ`, ...).
  #[napi]
  pub fn listener_param(&self, name: String) -> Option<AudioParam> {
    let listener = self.base.listener();
    let param = match name.as_str() {
      "positionX" => listener.position_x(),
      "positionY" => listener.position_y(),
      "positionZ" => listener.position_z(),
      "forwardX" => listener.forward_x(),
      "forwardY" => listener.forward_y(),
      "forwardZ" => listener.forward_z(),
      "upX" => listener.up_x(),
      "upY" => listener.up_y(),
      "upZ" => listener.up_z(),
      _ => return None,
    };
    Some(AudioParam::wrap(param))
  }

  #[napi]
  pub fn create_gain(&self) -> Result<AudioNode> {
    self.node(|base| Kind::Gain(base.create_gain()))
  }

  #[napi]
  pub fn create_biquad_filter(&self) -> Result<AudioNode> {
    self.node(|base| Kind::BiquadFilter(base.create_biquad_filter()))
  }

  #[napi]
  pub fn create_iir_filter(&self, feedforward: Vec<f64>, feedback: Vec<f64>) -> Result<AudioNode> {
    self.node(|base| Kind::IirFilter(base.create_iir_filter(feedforward, feedback)))
  }

  #[napi]
  pub fn create_panner(&self) -> Result<AudioNode> {
    self.node(|base| Kind::Panner(base.create_panner()))
  }

  #[napi]
  pub fn create_stereo_panner(&self) -> Result<AudioNode> {
    self.node(|base| Kind::StereoPanner(base.create_stereo_panner()))
  }

  #[napi]
  pub fn create_delay(&self, max_delay_time: f64) -> Result<AudioNode> {
    self.node(|base| Kind::Delay(base.create_delay(max_delay_time)))
  }

  #[napi]
  pub fn create_oscillator(&self) -> Result<AudioNode> {
    self.node(|base| Kind::Oscillator(base.create_oscillator()))
  }

  #[napi]
  pub fn create_buffer_source(&self) -> Result<AudioNode> {
    self.node(|base| Kind::BufferSource(base.create_buffer_source()))
  }

  #[napi]
  pub fn create_constant_source(&self) -> Result<AudioNode> {
    self.node(|base| Kind::ConstantSource(base.create_constant_source()))
  }

  #[napi]
  pub fn create_analyser(&self) -> Result<AudioNode> {
    self.node(|base| Kind::Analyser(base.create_analyser()))
  }

  #[napi]
  pub fn create_wave_shaper(&self) -> Result<AudioNode> {
    self.node(|base| Kind::WaveShaper(base.create_wave_shaper()))
  }

  #[napi]
  pub fn create_convolver(&self) -> Result<AudioNode> {
    self.node(|base| Kind::Convolver(base.create_convolver()))
  }

  #[napi]
  pub fn create_dynamics_compressor(&self) -> Result<AudioNode> {
    self.node(|base| Kind::DynamicsCompressor(base.create_dynamics_compressor()))
  }

  #[napi]
  pub fn create_channel_splitter(&self, number_of_outputs: u32) -> Result<AudioNode> {
    self.node(|base| Kind::ChannelSplitter(base.create_channel_splitter(number_of_outputs as usize)))
  }

  #[napi]
  pub fn create_channel_merger(&self, number_of_inputs: u32) -> Result<AudioNode> {
    self.node(|base| Kind::ChannelMerger(base.create_channel_merger(number_of_inputs as usize)))
  }

  #[napi]
  pub fn create_periodic_wave(
    &self,
    real: Float32ArraySlice,
    imag: Float32ArraySlice,
    disable_normalization: bool,
  ) -> Result<PeriodicWave> {
    let options = PeriodicWaveOptions {
      real: Some(real.to_vec()),
      imag: Some(imag.to_vec()),
      disable_normalization,
    };
    guard(|| PeriodicWave { inner: self.base.create_periodic_wave(options) })
  }
}

pub enum StateChange {
  Resume,
  Suspend,
  Close,
  Sink(String),
}

/// A realtime context's state change; web-audio-api blocks until the render thread has made it.
pub struct StateTask {
  context: Arc<web_audio_api::context::AudioContext>,
  change: StateChange,
}

impl Task for StateTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<()> {
    let context = &self.context;
    match &self.change {
      StateChange::Resume => guard(|| context.resume_sync()),
      StateChange::Suspend => guard(|| context.suspend_sync()),
      StateChange::Close => guard(|| context.close_sync()),
      StateChange::Sink(sink_id) => guard(|| context.set_sink_id_sync(sink_id.clone()).map_err(|e| e.to_string()))?
        .map_err(|e| error(format!("NotFoundError: {e}"))),
    }
  }

  fn resolve(&mut self, _: Env, _: ()) -> Result<()> {
    Ok(())
  }
}

pub struct RenderTask {
  context: Option<OfflineAudioContext>,
}

impl Task for RenderTask {
  type Output = web_audio_api::AudioBuffer;
  type JsValue = AudioBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut context = self.context.take().ok_or_else(|| error("InvalidStateError: already rendered"))?;
    guard(|| context.start_rendering_sync())
  }

  fn resolve(&mut self, _: Env, buffer: Self::Output) -> Result<AudioBuffer> {
    Ok(AudioBuffer { inner: buffer })
  }
}

pub enum DecodeSource {
  Bytes(Vec<u8>),
  Base64(String),
  File(String),
}

pub struct DecodeTask {
  base: ConcreteBaseAudioContext,
  source: DecodeSource,
}

impl Task for DecodeTask {
  type Output = web_audio_api::AudioBuffer;
  type JsValue = AudioBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let base = &self.base;
    let decoded = match std::mem::replace(&mut self.source, DecodeSource::Bytes(Vec::new())) {
      DecodeSource::Bytes(bytes) => guard(|| base.decode_audio_data_sync(Cursor::new(bytes)))?,
      DecodeSource::Base64(data) => {
        let data = data.split_once(";base64,").map_or(data.as_str(), |(_, data)| data);
        let bytes = base64::engine::general_purpose::STANDARD
          .decode(data.trim())
          .map_err(|e| error(format!("EncodingError: invalid base64 ({e})")))?;
        guard(|| base.decode_audio_data_sync(Cursor::new(bytes)))?
      }
      DecodeSource::File(path) => {
        let file = std::fs::File::open(&path).map_err(|e| error(format!("NotFoundError: {path}: {e}")))?;
        guard(|| base.decode_audio_data_sync(file))?
      }
    };
    decoded.map_err(|e| error(format!("EncodingError: unable to decode audio data ({e})")))
  }

  fn resolve(&mut self, _: Env, buffer: Self::Output) -> Result<AudioBuffer> {
    Ok(AudioBuffer { inner: buffer })
  }
}
