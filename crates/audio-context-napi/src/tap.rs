use std::ffi::c_void;

use web_audio_api::AudioBuffer;

/// Same layout as `tap::AudioTapSource` in canvas-media's module (`NSCAudioTap.address`).
#[repr(C)]
#[derive(Clone, Copy)]
struct AudioTapSource {
  size: u32,
  reserved: u32,
  tap: *const c_void,
  read: unsafe extern "C" fn(tap: *const c_void, out: *mut f32, capacity: usize, channels: *mut u32, sample_rate: *mut u32) -> usize,
  retain: unsafe extern "C" fn(tap: *const c_void),
  release: unsafe extern "C" fn(tap: *const c_void),
}

const FRAMES: usize = 256;
const MAX_CHANNELS: usize = 8;

/// Pulled on the render thread: a read never waits, and an empty tap is silence.
pub struct TapStream {
  source: AudioTapSource,
  scratch: Vec<f32>,
  channels: usize,
  sample_rate: f32,
}

// The tap is Send + Sync on the producer's side and only reached through `read`.
unsafe impl Send for TapStream {}
unsafe impl Sync for TapStream {}

impl TapStream {
  /// # Safety
  /// `address` must be an `NSCAudioTap.address` whose tap is alive for the duration of the call;
  /// the stream then holds its own reference.
  pub unsafe fn new(address: usize) -> Option<TapStream> {
    let source = (address as *const AudioTapSource).as_ref()?;
    if source.size as usize != std::mem::size_of::<AudioTapSource>() || source.tap.is_null() {
      return None;
    }
    (source.retain)(source.tap);
    Some(TapStream {
      source: *source,
      scratch: vec![0.; FRAMES * MAX_CHANNELS],
      channels: 2,
      sample_rate: 48000.,
    })
  }
}

impl Drop for TapStream {
  fn drop(&mut self) {
    unsafe { (self.source.release)(self.source.tap) };
  }
}

impl Iterator for TapStream {
  type Item = Result<AudioBuffer, Box<dyn std::error::Error + Send + Sync>>;

  fn next(&mut self) -> Option<Self::Item> {
    let (mut channels, mut sample_rate) = (0u32, 0u32);
    let frames = unsafe {
      (self.source.read)(self.source.tap, self.scratch.as_mut_ptr(), self.scratch.len(), &mut channels, &mut sample_rate)
    };
    if channels > 0 && (channels as usize) <= MAX_CHANNELS && sample_rate > 0 {
      self.channels = channels as usize;
      self.sample_rate = sample_rate as f32;
    }
    let channel_count = self.channels;
    let data: Vec<Vec<f32>> = if frames == 0 {
      vec![vec![0.; FRAMES]; channel_count]
    } else {
      (0..channel_count)
        .map(|channel| (0..frames).map(|frame| self.scratch[frame * channel_count + channel]).collect())
        .collect()
    };
    Some(Ok(AudioBuffer::from(data, self.sample_rate)))
  }
}
