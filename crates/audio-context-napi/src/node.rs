use napi::bindgen_prelude::{Float32ArraySlice, Function, Uint8ArraySlice};
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::{Error, Result};
use napi_derive::napi;
use web_audio_api::node::{
  self, AudioScheduledSourceNode as _, BiquadFilterType, ChannelCountMode, ChannelInterpretation,
  DistanceModelType, OscillatorType, OverSampleType, PanningModelType,
};

use crate::buffer::{AudioBuffer, PeriodicWave};
use crate::param::AudioParam;
use crate::{error, guard};

pub(crate) enum Kind {
  Destination(node::AudioDestinationNode),
  Gain(node::GainNode),
  BiquadFilter(node::BiquadFilterNode),
  IirFilter(node::IIRFilterNode),
  Panner(node::PannerNode),
  StereoPanner(node::StereoPannerNode),
  Delay(node::DelayNode),
  Oscillator(node::OscillatorNode),
  BufferSource(node::AudioBufferSourceNode),
  ConstantSource(node::ConstantSourceNode),
  Analyser(node::AnalyserNode),
  WaveShaper(node::WaveShaperNode),
  Convolver(node::ConvolverNode),
  DynamicsCompressor(node::DynamicsCompressorNode),
  ChannelSplitter(node::ChannelSplitterNode),
  ChannelMerger(node::ChannelMergerNode),
  /// A media element's audio, from canvas-media's tap.
  MediaElementSource(node::MediaStreamTrackAudioSourceNode),
}

/// Evaluates `$body` with `$node` bound to whichever node `$kind` holds.
macro_rules! any_node {
  ($kind:expr, $node:ident => $body:expr) => {
    match $kind {
      Kind::Destination($node) => $body,
      Kind::Gain($node) => $body,
      Kind::BiquadFilter($node) => $body,
      Kind::IirFilter($node) => $body,
      Kind::Panner($node) => $body,
      Kind::StereoPanner($node) => $body,
      Kind::Delay($node) => $body,
      Kind::Oscillator($node) => $body,
      Kind::BufferSource($node) => $body,
      Kind::ConstantSource($node) => $body,
      Kind::Analyser($node) => $body,
      Kind::WaveShaper($node) => $body,
      Kind::Convolver($node) => $body,
      Kind::DynamicsCompressor($node) => $body,
      Kind::ChannelSplitter($node) => $body,
      Kind::ChannelMerger($node) => $body,
      Kind::MediaElementSource($node) => $body,
    }
  };
}

/// As `any_node!`, over the scheduled sources only.
macro_rules! scheduled {
  ($self:ident, $member:literal, $node:ident => $body:expr) => {
    match &mut $self.kind {
      Kind::Oscillator($node) => $body,
      Kind::BufferSource($node) => $body,
      Kind::ConstantSource($node) => $body,
      _ => return Err($self.unsupported($member)),
    }
  };
}

type EndedCallback<'a> = Function<'a, (), UnknownReturnValue>;

/// Any node of a graph. One class for every kind keeps `connect` and `disconnect` simple; the
/// members a kind does not have throw.
#[napi]
pub struct AudioNode {
  pub(crate) kind: Kind,
}

impl AudioNode {
  pub(crate) fn new(kind: Kind) -> Self {
    Self { kind }
  }

  fn node(&self) -> &dyn node::AudioNode {
    any_node!(&self.kind, node => node as &dyn node::AudioNode)
  }

  fn name(&self) -> &'static str {
    match self.kind {
      Kind::Destination(_) => "AudioDestinationNode",
      Kind::Gain(_) => "GainNode",
      Kind::BiquadFilter(_) => "BiquadFilterNode",
      Kind::IirFilter(_) => "IIRFilterNode",
      Kind::Panner(_) => "PannerNode",
      Kind::StereoPanner(_) => "StereoPannerNode",
      Kind::Delay(_) => "DelayNode",
      Kind::Oscillator(_) => "OscillatorNode",
      Kind::BufferSource(_) => "AudioBufferSourceNode",
      Kind::ConstantSource(_) => "ConstantSourceNode",
      Kind::Analyser(_) => "AnalyserNode",
      Kind::WaveShaper(_) => "WaveShaperNode",
      Kind::Convolver(_) => "ConvolverNode",
      Kind::DynamicsCompressor(_) => "DynamicsCompressorNode",
      Kind::ChannelSplitter(_) => "ChannelSplitterNode",
      Kind::ChannelMerger(_) => "ChannelMergerNode",
      Kind::MediaElementSource(_) => "MediaElementAudioSourceNode",
    }
  }

  fn unsupported(&self, member: &str) -> Error {
    error(format!("TypeError: {} has no member '{member}'", self.name()))
  }
}

fn oscillator_type(value: OscillatorType) -> &'static str {
  match value {
    OscillatorType::Sine => "sine",
    OscillatorType::Square => "square",
    OscillatorType::Sawtooth => "sawtooth",
    OscillatorType::Triangle => "triangle",
    OscillatorType::Custom => "custom",
  }
}

fn biquad_type(value: BiquadFilterType) -> &'static str {
  match value {
    BiquadFilterType::Lowpass => "lowpass",
    BiquadFilterType::Highpass => "highpass",
    BiquadFilterType::Bandpass => "bandpass",
    BiquadFilterType::Notch => "notch",
    BiquadFilterType::Allpass => "allpass",
    BiquadFilterType::Peaking => "peaking",
    BiquadFilterType::Lowshelf => "lowshelf",
    BiquadFilterType::Highshelf => "highshelf",
  }
}

fn invalid_enum(value: &str, name: &str) -> Error {
  error(format!("TypeError: '{value}' is not a valid value for enumeration {name}"))
}

#[napi]
impl AudioNode {
  #[napi(getter)]
  pub fn number_of_inputs(&self) -> u32 {
    self.node().number_of_inputs() as u32
  }

  #[napi(getter)]
  pub fn number_of_outputs(&self) -> u32 {
    self.node().number_of_outputs() as u32
  }

  #[napi(getter)]
  pub fn channel_count(&self) -> u32 {
    self.node().channel_count() as u32
  }

  #[napi(setter, js_name = "channelCount")]
  pub fn set_channel_count(&self, value: u32) -> Result<()> {
    guard(|| self.node().set_channel_count(value as usize))
  }

  #[napi(getter)]
  pub fn channel_count_mode(&self) -> &'static str {
    match self.node().channel_count_mode() {
      ChannelCountMode::Max => "max",
      ChannelCountMode::ClampedMax => "clamped-max",
      ChannelCountMode::Explicit => "explicit",
    }
  }

  #[napi(setter, js_name = "channelCountMode")]
  pub fn set_channel_count_mode(&self, value: String) -> Result<()> {
    let mode = match value.as_str() {
      "max" => ChannelCountMode::Max,
      "clamped-max" => ChannelCountMode::ClampedMax,
      "explicit" => ChannelCountMode::Explicit,
      _ => return Err(invalid_enum(&value, "ChannelCountMode")),
    };
    guard(|| self.node().set_channel_count_mode(mode))
  }

  #[napi(getter)]
  pub fn channel_interpretation(&self) -> &'static str {
    match self.node().channel_interpretation() {
      ChannelInterpretation::Speakers => "speakers",
      ChannelInterpretation::Discrete => "discrete",
    }
  }

  #[napi(setter, js_name = "channelInterpretation")]
  pub fn set_channel_interpretation(&self, value: String) -> Result<()> {
    let interpretation = match value.as_str() {
      "speakers" => ChannelInterpretation::Speakers,
      "discrete" => ChannelInterpretation::Discrete,
      _ => return Err(invalid_enum(&value, "ChannelInterpretation")),
    };
    guard(|| self.node().set_channel_interpretation(interpretation))
  }

  #[napi]
  pub fn connect(&self, destination: &AudioNode, output: Option<u32>, input: Option<u32>) -> Result<()> {
    guard(|| {
      self.node().connect_from_output_to_input(
        destination.node(),
        output.unwrap_or(0) as usize,
        input.unwrap_or(0) as usize,
      );
    })
  }

  #[napi]
  pub fn connect_param(&self, destination: &AudioParam, output: Option<u32>) -> Result<()> {
    guard(|| {
      self
        .node()
        .connect_from_output_to_input(&destination.inner, output.unwrap_or(0) as usize, 0);
    })
  }

  /// Every outgoing connection.
  #[napi]
  pub fn disconnect(&self) -> Result<()> {
    guard(|| self.node().disconnect())
  }

  /// Every connection from one output.
  #[napi]
  pub fn disconnect_output(&self, output: u32) -> Result<()> {
    guard(|| self.node().disconnect_output(output as usize))
  }

  #[napi]
  pub fn disconnect_node(&self, destination: &AudioNode, output: Option<u32>, input: Option<u32>) -> Result<()> {
    let node = self.node();
    guard(|| match (output, input) {
      (Some(output), Some(input)) => {
        node.disconnect_dest_from_output_to_input(destination.node(), output as usize, input as usize)
      }
      (Some(output), None) => node.disconnect_dest_from_output(destination.node(), output as usize),
      _ => node.disconnect_dest(destination.node()),
    })
  }

  #[napi]
  pub fn disconnect_param(&self, destination: &AudioParam, output: Option<u32>) -> Result<()> {
    let node = self.node();
    guard(|| match output {
      Some(output) => node.disconnect_dest_from_output(&destination.inner, output as usize),
      None => node.disconnect_dest(&destination.inner),
    })
  }

  /// One of the node's `AudioParam`s by its JS name (`gain`, `frequency`, `Q`, `positionX`, ...).
  #[napi]
  pub fn param(&self, name: String) -> Option<AudioParam> {
    let param = match (&self.kind, name.as_str()) {
      (Kind::Gain(node), "gain") => node.gain(),
      (Kind::BiquadFilter(node), "frequency") => node.frequency(),
      (Kind::BiquadFilter(node), "detune") => node.detune(),
      (Kind::BiquadFilter(node), "Q") => node.q(),
      (Kind::BiquadFilter(node), "gain") => node.gain(),
      (Kind::Panner(node), "positionX") => node.position_x(),
      (Kind::Panner(node), "positionY") => node.position_y(),
      (Kind::Panner(node), "positionZ") => node.position_z(),
      (Kind::Panner(node), "orientationX") => node.orientation_x(),
      (Kind::Panner(node), "orientationY") => node.orientation_y(),
      (Kind::Panner(node), "orientationZ") => node.orientation_z(),
      (Kind::StereoPanner(node), "pan") => node.pan(),
      (Kind::Delay(node), "delayTime") => node.delay_time(),
      (Kind::Oscillator(node), "frequency") => node.frequency(),
      (Kind::Oscillator(node), "detune") => node.detune(),
      (Kind::BufferSource(node), "playbackRate") => node.playback_rate(),
      (Kind::BufferSource(node), "detune") => node.detune(),
      (Kind::ConstantSource(node), "offset") => node.offset(),
      (Kind::DynamicsCompressor(node), "threshold") => node.threshold(),
      (Kind::DynamicsCompressor(node), "knee") => node.knee(),
      (Kind::DynamicsCompressor(node), "ratio") => node.ratio(),
      (Kind::DynamicsCompressor(node), "attack") => node.attack(),
      (Kind::DynamicsCompressor(node), "release") => node.release(),
      _ => return None,
    };
    Some(AudioParam::wrap(param))
  }

  // Scheduled sources.

  /// `start(when, offset, duration)`; `offset` and `duration` apply to buffer sources only.
  #[napi]
  pub fn start(&mut self, when: Option<f64>, offset: Option<f64>, duration: Option<f64>) -> Result<()> {
    let when = when.unwrap_or(0.0);
    if let Kind::BufferSource(node) = &mut self.kind {
      return guard(|| match (offset, duration) {
        (offset, Some(duration)) => node.start_at_with_offset_and_duration(when, offset.unwrap_or(0.0), duration),
        (Some(offset), None) => node.start_at_with_offset(when, offset),
        (None, None) => node.start_at(when),
      });
    }
    scheduled!(self, "start", node => guard(|| node.start_at(when)))
  }

  #[napi]
  pub fn stop(&mut self, when: Option<f64>) -> Result<()> {
    let when = when.unwrap_or(0.0);
    scheduled!(self, "stop", node => guard(|| node.stop_at(when)))
  }

  /// `callback` runs on the JS thread once the source has ended; `null` clears it.
  #[napi(ts_args_type = "callback: (() => void) | null")]
  pub fn set_onended(&mut self, callback: Option<EndedCallback>) -> Result<()> {
    let Some(callback) = callback else {
      scheduled!(self, "onended", node => node.clear_onended());
      return Ok(());
    };
    // Weak: a source that never ends must not keep the host's event loop alive.
    let tsfn = callback
      .build_threadsafe_function::<()>()
      .weak::<true>()
      .build_callback(|_| Ok(()))?;
    scheduled!(self, "onended", node => node.set_onended(move |_| {
      tsfn.call((), ThreadsafeFunctionCallMode::NonBlocking);
    }));
    Ok(())
  }

  // OscillatorNode, BiquadFilterNode.

  #[napi(getter, js_name = "type")]
  pub fn get_type(&self) -> Result<&'static str> {
    match &self.kind {
      Kind::Oscillator(node) => Ok(oscillator_type(node.type_())),
      Kind::BiquadFilter(node) => Ok(biquad_type(node.type_())),
      _ => Err(self.unsupported("type")),
    }
  }

  #[napi(setter, js_name = "type")]
  pub fn set_type(&mut self, value: String) -> Result<()> {
    match &mut self.kind {
      Kind::Oscillator(node) => {
        let type_ = match value.as_str() {
          "sine" => OscillatorType::Sine,
          "square" => OscillatorType::Square,
          "sawtooth" => OscillatorType::Sawtooth,
          "triangle" => OscillatorType::Triangle,
          "custom" => OscillatorType::Custom,
          _ => return Err(invalid_enum(&value, "OscillatorType")),
        };
        guard(|| node.set_type(type_))
      }
      Kind::BiquadFilter(node) => {
        let type_ = match value.as_str() {
          "lowpass" => BiquadFilterType::Lowpass,
          "highpass" => BiquadFilterType::Highpass,
          "bandpass" => BiquadFilterType::Bandpass,
          "notch" => BiquadFilterType::Notch,
          "allpass" => BiquadFilterType::Allpass,
          "peaking" => BiquadFilterType::Peaking,
          "lowshelf" => BiquadFilterType::Lowshelf,
          "highshelf" => BiquadFilterType::Highshelf,
          _ => return Err(invalid_enum(&value, "BiquadFilterType")),
        };
        guard(|| node.set_type(type_))
      }
      _ => Err(self.unsupported("type")),
    }
  }

  #[napi]
  pub fn set_periodic_wave(&mut self, wave: &PeriodicWave) -> Result<()> {
    match &mut self.kind {
      Kind::Oscillator(node) => guard(|| node.set_periodic_wave(wave.inner.clone())),
      _ => Err(self.unsupported("setPeriodicWave")),
    }
  }

  /// BiquadFilterNode and IIRFilterNode.
  #[napi]
  pub fn get_frequency_response(
    &self,
    frequency_hz: Float32ArraySlice,
    mut mag_response: Float32ArraySlice,
    mut phase_response: Float32ArraySlice,
  ) -> Result<()> {
    let (mag, phase) = unsafe { (mag_response.as_mut(), phase_response.as_mut()) };
    match &self.kind {
      Kind::BiquadFilter(node) => guard(|| node.get_frequency_response(&frequency_hz, mag, phase)),
      Kind::IirFilter(node) => guard(|| node.get_frequency_response(&frequency_hz, mag, phase)),
      _ => Err(self.unsupported("getFrequencyResponse")),
    }
  }

  // AudioBufferSourceNode, ConvolverNode.

  /// Buffer sources and convolvers take a buffer once.
  #[napi]
  pub fn set_buffer(&mut self, buffer: &AudioBuffer) -> Result<()> {
    let buffer = buffer.inner.clone();
    match &mut self.kind {
      Kind::BufferSource(node) => guard(|| node.set_buffer(buffer)),
      Kind::Convolver(node) => guard(|| node.set_buffer(buffer)),
      _ => Err(self.unsupported("buffer")),
    }
  }

  #[napi(getter, js_name = "loop")]
  pub fn get_loop(&self) -> Result<bool> {
    match &self.kind {
      Kind::BufferSource(node) => Ok(node.loop_()),
      _ => Err(self.unsupported("loop")),
    }
  }

  #[napi(setter, js_name = "loop")]
  pub fn set_loop(&mut self, value: bool) -> Result<()> {
    match &mut self.kind {
      Kind::BufferSource(node) => guard(|| node.set_loop(value)),
      _ => Err(self.unsupported("loop")),
    }
  }

  #[napi(getter)]
  pub fn loop_start(&self) -> Result<f64> {
    match &self.kind {
      Kind::BufferSource(node) => Ok(node.loop_start()),
      _ => Err(self.unsupported("loopStart")),
    }
  }

  #[napi(setter, js_name = "loopStart")]
  pub fn set_loop_start(&mut self, value: f64) -> Result<()> {
    match &mut self.kind {
      Kind::BufferSource(node) => guard(|| node.set_loop_start(value)),
      _ => Err(self.unsupported("loopStart")),
    }
  }

  #[napi(getter)]
  pub fn loop_end(&self) -> Result<f64> {
    match &self.kind {
      Kind::BufferSource(node) => Ok(node.loop_end()),
      _ => Err(self.unsupported("loopEnd")),
    }
  }

  #[napi(setter, js_name = "loopEnd")]
  pub fn set_loop_end(&mut self, value: f64) -> Result<()> {
    match &mut self.kind {
      Kind::BufferSource(node) => guard(|| node.set_loop_end(value)),
      _ => Err(self.unsupported("loopEnd")),
    }
  }

  #[napi(getter)]
  pub fn normalize(&self) -> Result<bool> {
    match &self.kind {
      Kind::Convolver(node) => Ok(node.normalize()),
      _ => Err(self.unsupported("normalize")),
    }
  }

  #[napi(setter, js_name = "normalize")]
  pub fn set_normalize(&mut self, value: bool) -> Result<()> {
    match &mut self.kind {
      Kind::Convolver(node) => guard(|| node.set_normalize(value)),
      _ => Err(self.unsupported("normalize")),
    }
  }

  // PannerNode.

  #[napi(getter)]
  pub fn panning_model(&self) -> Result<&'static str> {
    match &self.kind {
      Kind::Panner(node) => Ok(match node.panning_model() {
        PanningModelType::EqualPower => "equalpower",
        PanningModelType::HRTF => "HRTF",
      }),
      _ => Err(self.unsupported("panningModel")),
    }
  }

  #[napi(setter, js_name = "panningModel")]
  pub fn set_panning_model(&mut self, value: String) -> Result<()> {
    let model = match value.as_str() {
      "equalpower" => PanningModelType::EqualPower,
      "HRTF" => PanningModelType::HRTF,
      _ => return Err(invalid_enum(&value, "PanningModelType")),
    };
    match &mut self.kind {
      Kind::Panner(node) => guard(|| node.set_panning_model(model)),
      _ => Err(self.unsupported("panningModel")),
    }
  }

  #[napi(getter)]
  pub fn distance_model(&self) -> Result<&'static str> {
    match &self.kind {
      Kind::Panner(node) => Ok(match node.distance_model() {
        DistanceModelType::Linear => "linear",
        DistanceModelType::Inverse => "inverse",
        DistanceModelType::Exponential => "exponential",
      }),
      _ => Err(self.unsupported("distanceModel")),
    }
  }

  #[napi(setter, js_name = "distanceModel")]
  pub fn set_distance_model(&mut self, value: String) -> Result<()> {
    let model = match value.as_str() {
      "linear" => DistanceModelType::Linear,
      "inverse" => DistanceModelType::Inverse,
      "exponential" => DistanceModelType::Exponential,
      _ => return Err(invalid_enum(&value, "DistanceModelType")),
    };
    match &mut self.kind {
      Kind::Panner(node) => guard(|| node.set_distance_model(model)),
      _ => Err(self.unsupported("distanceModel")),
    }
  }

  #[napi(getter)]
  pub fn ref_distance(&self) -> Result<f64> {
    match &self.kind {
      Kind::Panner(node) => Ok(node.ref_distance()),
      _ => Err(self.unsupported("refDistance")),
    }
  }

  #[napi(setter, js_name = "refDistance")]
  pub fn set_ref_distance(&mut self, value: f64) -> Result<()> {
    match &mut self.kind {
      Kind::Panner(node) => guard(|| node.set_ref_distance(value)),
      _ => Err(self.unsupported("refDistance")),
    }
  }

  #[napi(getter)]
  pub fn max_distance(&self) -> Result<f64> {
    match &self.kind {
      Kind::Panner(node) => Ok(node.max_distance()),
      _ => Err(self.unsupported("maxDistance")),
    }
  }

  #[napi(setter, js_name = "maxDistance")]
  pub fn set_max_distance(&mut self, value: f64) -> Result<()> {
    match &mut self.kind {
      Kind::Panner(node) => guard(|| node.set_max_distance(value)),
      _ => Err(self.unsupported("maxDistance")),
    }
  }

  #[napi(getter)]
  pub fn rolloff_factor(&self) -> Result<f64> {
    match &self.kind {
      Kind::Panner(node) => Ok(node.rolloff_factor()),
      _ => Err(self.unsupported("rolloffFactor")),
    }
  }

  #[napi(setter, js_name = "rolloffFactor")]
  pub fn set_rolloff_factor(&mut self, value: f64) -> Result<()> {
    match &mut self.kind {
      Kind::Panner(node) => guard(|| node.set_rolloff_factor(value)),
      _ => Err(self.unsupported("rolloffFactor")),
    }
  }

  #[napi(getter)]
  pub fn cone_inner_angle(&self) -> Result<f64> {
    match &self.kind {
      Kind::Panner(node) => Ok(node.cone_inner_angle()),
      _ => Err(self.unsupported("coneInnerAngle")),
    }
  }

  #[napi(setter, js_name = "coneInnerAngle")]
  pub fn set_cone_inner_angle(&mut self, value: f64) -> Result<()> {
    match &mut self.kind {
      Kind::Panner(node) => guard(|| node.set_cone_inner_angle(value)),
      _ => Err(self.unsupported("coneInnerAngle")),
    }
  }

  #[napi(getter)]
  pub fn cone_outer_angle(&self) -> Result<f64> {
    match &self.kind {
      Kind::Panner(node) => Ok(node.cone_outer_angle()),
      _ => Err(self.unsupported("coneOuterAngle")),
    }
  }

  #[napi(setter, js_name = "coneOuterAngle")]
  pub fn set_cone_outer_angle(&mut self, value: f64) -> Result<()> {
    match &mut self.kind {
      Kind::Panner(node) => guard(|| node.set_cone_outer_angle(value)),
      _ => Err(self.unsupported("coneOuterAngle")),
    }
  }

  #[napi(getter)]
  pub fn cone_outer_gain(&self) -> Result<f64> {
    match &self.kind {
      Kind::Panner(node) => Ok(node.cone_outer_gain()),
      _ => Err(self.unsupported("coneOuterGain")),
    }
  }

  #[napi(setter, js_name = "coneOuterGain")]
  pub fn set_cone_outer_gain(&mut self, value: f64) -> Result<()> {
    match &mut self.kind {
      Kind::Panner(node) => guard(|| node.set_cone_outer_gain(value)),
      _ => Err(self.unsupported("coneOuterGain")),
    }
  }

  // AnalyserNode.

  #[napi(getter)]
  pub fn fft_size(&self) -> Result<u32> {
    match &self.kind {
      Kind::Analyser(node) => Ok(node.fft_size() as u32),
      _ => Err(self.unsupported("fftSize")),
    }
  }

  #[napi(setter, js_name = "fftSize")]
  pub fn set_fft_size(&mut self, value: u32) -> Result<()> {
    match &mut self.kind {
      Kind::Analyser(node) => guard(|| node.set_fft_size(value as usize)),
      _ => Err(self.unsupported("fftSize")),
    }
  }

  #[napi(getter)]
  pub fn frequency_bin_count(&self) -> Result<u32> {
    match &self.kind {
      Kind::Analyser(node) => Ok(node.frequency_bin_count() as u32),
      _ => Err(self.unsupported("frequencyBinCount")),
    }
  }

  #[napi(getter)]
  pub fn smoothing_time_constant(&self) -> Result<f64> {
    match &self.kind {
      Kind::Analyser(node) => Ok(node.smoothing_time_constant()),
      _ => Err(self.unsupported("smoothingTimeConstant")),
    }
  }

  #[napi(setter, js_name = "smoothingTimeConstant")]
  pub fn set_smoothing_time_constant(&mut self, value: f64) -> Result<()> {
    match &mut self.kind {
      Kind::Analyser(node) => guard(|| node.set_smoothing_time_constant(value)),
      _ => Err(self.unsupported("smoothingTimeConstant")),
    }
  }

  #[napi(getter)]
  pub fn min_decibels(&self) -> Result<f64> {
    match &self.kind {
      Kind::Analyser(node) => Ok(node.min_decibels()),
      _ => Err(self.unsupported("minDecibels")),
    }
  }

  #[napi(setter, js_name = "minDecibels")]
  pub fn set_min_decibels(&mut self, value: f64) -> Result<()> {
    match &mut self.kind {
      Kind::Analyser(node) => guard(|| node.set_min_decibels(value)),
      _ => Err(self.unsupported("minDecibels")),
    }
  }

  #[napi(getter)]
  pub fn max_decibels(&self) -> Result<f64> {
    match &self.kind {
      Kind::Analyser(node) => Ok(node.max_decibels()),
      _ => Err(self.unsupported("maxDecibels")),
    }
  }

  #[napi(setter, js_name = "maxDecibels")]
  pub fn set_max_decibels(&mut self, value: f64) -> Result<()> {
    match &mut self.kind {
      Kind::Analyser(node) => guard(|| node.set_max_decibels(value)),
      _ => Err(self.unsupported("maxDecibels")),
    }
  }

  #[napi]
  pub fn get_float_time_domain_data(&mut self, mut destination: Float32ArraySlice) -> Result<()> {
    let destination = unsafe { destination.as_mut() };
    match &mut self.kind {
      Kind::Analyser(node) => guard(|| node.get_float_time_domain_data(destination)),
      _ => Err(self.unsupported("getFloatTimeDomainData")),
    }
  }

  #[napi]
  pub fn get_byte_time_domain_data(&mut self, mut destination: Uint8ArraySlice) -> Result<()> {
    let destination = unsafe { destination.as_mut() };
    match &mut self.kind {
      Kind::Analyser(node) => guard(|| node.get_byte_time_domain_data(destination)),
      _ => Err(self.unsupported("getByteTimeDomainData")),
    }
  }

  #[napi]
  pub fn get_float_frequency_data(&mut self, mut destination: Float32ArraySlice) -> Result<()> {
    let destination = unsafe { destination.as_mut() };
    match &mut self.kind {
      Kind::Analyser(node) => guard(|| node.get_float_frequency_data(destination)),
      _ => Err(self.unsupported("getFloatFrequencyData")),
    }
  }

  #[napi]
  pub fn get_byte_frequency_data(&mut self, mut destination: Uint8ArraySlice) -> Result<()> {
    let destination = unsafe { destination.as_mut() };
    match &mut self.kind {
      Kind::Analyser(node) => guard(|| node.get_byte_frequency_data(destination)),
      _ => Err(self.unsupported("getByteFrequencyData")),
    }
  }

  // WaveShaperNode.

  /// The shaping curve (the engine takes one curve per node).
  #[napi]
  pub fn set_curve(&mut self, curve: Float32ArraySlice) -> Result<()> {
    let curve = curve.to_vec();
    match &mut self.kind {
      Kind::WaveShaper(node) => guard(|| node.set_curve(curve)),
      _ => Err(self.unsupported("curve")),
    }
  }

  #[napi(getter)]
  pub fn oversample(&self) -> Result<&'static str> {
    match &self.kind {
      Kind::WaveShaper(node) => Ok(match node.oversample() {
        OverSampleType::None => "none",
        OverSampleType::X2 => "2x",
        OverSampleType::X4 => "4x",
      }),
      _ => Err(self.unsupported("oversample")),
    }
  }

  #[napi(setter, js_name = "oversample")]
  pub fn set_oversample(&mut self, value: String) -> Result<()> {
    let oversample = match value.as_str() {
      "none" => OverSampleType::None,
      "2x" => OverSampleType::X2,
      "4x" => OverSampleType::X4,
      _ => return Err(invalid_enum(&value, "OverSampleType")),
    };
    match &mut self.kind {
      Kind::WaveShaper(node) => guard(|| node.set_oversample(oversample)),
      _ => Err(self.unsupported("oversample")),
    }
  }

  // DynamicsCompressorNode.

  /// The current gain reduction, in dB.
  #[napi(getter)]
  pub fn reduction(&self) -> Result<f64> {
    match &self.kind {
      Kind::DynamicsCompressor(node) => Ok(node.reduction() as f64),
      _ => Err(self.unsupported("reduction")),
    }
  }

  // AudioDestinationNode.

  #[napi(getter)]
  pub fn max_channel_count(&self) -> Result<u32> {
    match &self.kind {
      Kind::Destination(node) => Ok(node.max_channel_count() as u32),
      _ => Err(self.unsupported("maxChannelCount")),
    }
  }
}
