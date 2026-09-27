use napi::bindgen_prelude::{Float32Array, Float32ArraySlice};
use napi::Result;
use napi_derive::napi;
use web_audio_api::AudioBufferOptions;

use crate::guard;

/// Non-interleaved f32 samples. Immutable once built: the JS `AudioBuffer` keeps its own
/// channel arrays and builds a new one from them whenever a node acquires the buffer.
#[napi]
pub struct AudioBuffer {
  pub(crate) inner: web_audio_api::AudioBuffer,
}

#[napi]
impl AudioBuffer {
  #[napi(constructor)]
  pub fn new(number_of_channels: u32, length: u32, sample_rate: f64) -> Result<Self> {
    let options = AudioBufferOptions {
      number_of_channels: number_of_channels as usize,
      length: length as usize,
      sample_rate: sample_rate as f32,
    };
    guard(|| Self { inner: web_audio_api::AudioBuffer::new(options) })
  }

  /// A buffer holding a copy of `channels` (all of one length).
  #[napi(factory)]
  pub fn from_channels(channels: Vec<Float32Array>, sample_rate: f64) -> Result<Self> {
    let samples = channels.iter().map(|channel| channel.to_vec()).collect();
    guard(|| Self { inner: web_audio_api::AudioBuffer::from(samples, sample_rate as f32) })
  }

  #[napi(getter)]
  pub fn sample_rate(&self) -> f64 {
    self.inner.sample_rate() as f64
  }

  #[napi(getter)]
  pub fn length(&self) -> u32 {
    self.inner.length() as u32
  }

  #[napi(getter)]
  pub fn duration(&self) -> f64 {
    self.inner.duration()
  }

  #[napi(getter)]
  pub fn number_of_channels(&self) -> u32 {
    self.inner.number_of_channels() as u32
  }

  /// A copy of one channel's samples.
  #[napi]
  pub fn channel_data(&self, channel: u32) -> Result<Float32Array> {
    guard(|| Float32Array::new(self.inner.get_channel_data(channel as usize).to_vec()))
  }

  #[napi]
  pub fn copy_from_channel(&self, mut destination: Float32ArraySlice, channel: u32, start_in_channel: u32) -> Result<()> {
    let destination = unsafe { destination.as_mut() };
    guard(|| {
      self
        .inner
        .copy_from_channel_with_offset(destination, channel as usize, start_in_channel as usize)
    })
  }
}

/// A custom oscillator waveform.
#[napi]
pub struct PeriodicWave {
  pub(crate) inner: web_audio_api::PeriodicWave,
}
