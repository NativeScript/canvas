use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};

use windows::core::{implement, IInspectable, Interface, Ref, HRESULT, HSTRING};
use windows::Foundation::Collections::IPropertySet;
use windows::Foundation::{IMemoryBufferReference, IPropertyValue};
use windows::Media::Effects::{IBasicAudioEffect, IBasicAudioEffect_Impl, MediaEffectClosedReason, ProcessAudioFrameContext};
use windows::Media::MediaProperties::{AudioEncodingProperties, MediaEncodingSubtypes};
use windows::Media::{AudioBufferAccessMode, IMediaExtension, IMediaExtension_Impl};
use windows::Win32::Foundation::{CLASS_E_CLASSNOTAVAILABLE, E_POINTER, S_FALSE, S_OK};
use windows::Win32::System::WinRT::{IActivationFactory, IActivationFactory_Impl, IMemoryBufferByteAccess};
use windows_collections::IVectorView;

/// Registered in the app manifest by platforms/windows/plugin.targets, this module its server.
pub const TAP_CLASS: &str = "NativeScript.CanvasMedia.AudioTap";
pub const TAP_KEY: &str = "tap";
/// Beyond this the oldest samples go: the graph has fallen behind.
const MAX_BUFFERED_SECONDS: usize = 1;

pub struct Tap {
  pub id: u64,
  ring: Mutex<Ring>,
  routed: AtomicBool,
  /// f32 bits: the element's volume, 0 when muted.
  gain: AtomicU32,
  pub frames: AtomicU64,
}

#[derive(Default)]
struct Ring {
  samples: VecDeque<f32>,
  channels: u32,
  sample_rate: u32,
}

static TAPS: Mutex<Vec<Weak<Tap>>> = Mutex::new(Vec::new());
static NEXT_TAP: AtomicU64 = AtomicU64::new(1);

impl Tap {
  pub fn new() -> Arc<Tap> {
    let tap = Arc::new(Tap {
      id: NEXT_TAP.fetch_add(1, Ordering::Relaxed),
      ring: Mutex::new(Ring::default()),
      routed: AtomicBool::new(false),
      gain: AtomicU32::new(1f32.to_bits()),
      frames: AtomicU64::new(0),
    });
    let mut taps = TAPS.lock().unwrap();
    taps.retain(|tap| tap.strong_count() > 0);
    taps.push(Arc::downgrade(&tap));
    tap
  }

  fn find(id: u64) -> Option<Arc<Tap>> {
    TAPS.lock().unwrap().iter().filter_map(Weak::upgrade).find(|tap| tap.id == id)
  }

  pub fn set_gain(&self, gain: f32) {
    self.gain.store(gain.max(0.).to_bits(), Ordering::Relaxed);
  }

  pub fn set_routed(&self, routed: bool) {
    self.routed.store(routed, Ordering::Release);
    if !routed {
      self.clear();
    }
  }

  fn clear(&self) {
    self.ring.lock().unwrap().samples.clear();
  }

  /// Media Foundation's thread.
  fn push(&self, samples: &[f32], channels: u32, sample_rate: u32) {
    let gain = f32::from_bits(self.gain.load(Ordering::Relaxed));
    let mut ring = self.ring.lock().unwrap();
    if ring.channels != channels || ring.sample_rate != sample_rate {
      ring.samples.clear();
      ring.channels = channels;
      ring.sample_rate = sample_rate;
    }
    ring.samples.extend(samples.iter().map(|sample| sample * gain));
    self.frames.fetch_add((samples.len() / channels.max(1) as usize) as u64, Ordering::Relaxed);
    let max = sample_rate as usize * channels as usize * MAX_BUFFERED_SECONDS;
    let excess = ring.samples.len().saturating_sub(max);
    ring.samples.drain(..excess - excess % channels.max(1) as usize);
  }

  /// The graph's render thread; whole frames only.
  fn read(&self, out: &mut [f32], channels: &mut u32, sample_rate: &mut u32) -> usize {
    let mut ring = self.ring.lock().unwrap();
    *channels = ring.channels;
    *sample_rate = ring.sample_rate;
    if ring.channels == 0 {
      return 0;
    }
    let count = ring.samples.len().min(out.len());
    let count = count - count % ring.channels as usize;
    for (out, sample) in out.iter_mut().zip(ring.samples.drain(..count)) {
      *out = sample;
    }
    count / ring.channels as usize
  }
}

/// Read by audiocontext.node through `NSCAudioTap.address`; `tap` lives while it holds a `retain`ed
/// reference.
#[repr(C)]
pub struct AudioTapSource {
  pub size: u32,
  pub reserved: u32,
  pub tap: *const c_void,
  /// Reports the format too (0 channels: nothing decoded yet).
  pub read: unsafe extern "C" fn(tap: *const c_void, out: *mut f32, capacity: usize, channels: *mut u32, sample_rate: *mut u32) -> usize,
  pub retain: unsafe extern "C" fn(tap: *const c_void),
  pub release: unsafe extern "C" fn(tap: *const c_void),
}

unsafe extern "C" fn source_read(tap: *const c_void, out: *mut f32, capacity: usize, channels: *mut u32, sample_rate: *mut u32) -> usize {
  let tap = &*(tap as *const Tap);
  tap.read(std::slice::from_raw_parts_mut(out, capacity), &mut *channels, &mut *sample_rate)
}

unsafe extern "C" fn source_retain(tap: *const c_void) {
  Arc::increment_strong_count(tap as *const Tap);
}

unsafe extern "C" fn source_release(tap: *const c_void) {
  Arc::decrement_strong_count(tap as *const Tap);
}

impl AudioTapSource {
  pub fn new(tap: &Arc<Tap>) -> Self {
    Self {
      size: std::mem::size_of::<AudioTapSource>() as u32,
      reserved: 0,
      tap: Arc::as_ptr(tap) as *const c_void,
      read: source_read,
      retain: source_retain,
      release: source_release,
    }
  }
}

fn bytes(reference: &IMemoryBufferReference) -> windows::core::Result<(*mut u8, usize)> {
  let access: IMemoryBufferByteAccess = reference.cast()?;
  let (mut data, mut capacity) = (std::ptr::null_mut(), 0u32);
  unsafe { access.GetBuffer(&mut data, &mut capacity)? };
  Ok((data, capacity as usize))
}

#[implement(IBasicAudioEffect, IMediaExtension)]
struct AudioTapEffect {
  tap: Mutex<Option<Arc<Tap>>>,
  /// (sample rate, channels) of the float frames it is given.
  format: Mutex<(u32, u32)>,
}

impl IMediaExtension_Impl for AudioTapEffect_Impl {
  fn SetProperties(&self, configuration: Ref<IPropertySet>) -> windows::core::Result<()> {
    let id = configuration
      .ok()
      .ok()
      .and_then(|configuration| configuration.Lookup(&HSTRING::from(TAP_KEY)).ok())
      .and_then(|value| value.cast::<IPropertyValue>().ok())
      .and_then(|value| value.GetUInt64().ok());
    *self.tap.lock().unwrap() = id.and_then(Tap::find);
    Ok(())
  }
}

impl IBasicAudioEffect_Impl for AudioTapEffect_Impl {
  fn UseInputFrameForOutput(&self) -> windows::core::Result<bool> {
    Ok(false)
  }

  fn SupportedEncodingProperties(&self) -> windows::core::Result<IVectorView<AudioEncodingProperties>> {
    let float = MediaEncodingSubtypes::Float()?;
    let mut formats = Vec::new();
    for sample_rate in [48000, 44100] {
      for channels in [2, 1] {
        let format = AudioEncodingProperties::CreatePcm(sample_rate, channels, 32)?;
        format.SetSubtype(&float)?;
        formats.push(Some(format));
      }
    }
    Ok(IVectorView::from(formats))
  }

  fn SetEncodingProperties(&self, encoding: Ref<AudioEncodingProperties>) -> windows::core::Result<()> {
    let encoding = encoding.ok()?;
    *self.format.lock().unwrap() = (encoding.SampleRate()?, encoding.ChannelCount()?);
    Ok(())
  }

  fn ProcessFrame(&self, context: Ref<ProcessAudioFrameContext>) -> windows::core::Result<()> {
    let context = context.ok()?;
    let (input, output) = (context.InputFrame()?, context.OutputFrame()?);
    let input = input.LockBuffer(AudioBufferAccessMode::Read)?;
    let output = output.LockBuffer(AudioBufferAccessMode::Write)?;
    let (input_ref, output_ref) = (input.CreateReference()?, output.CreateReference()?);
    let ((input_data, input_capacity), (output_data, output_capacity)) = (bytes(&input_ref)?, bytes(&output_ref)?);
    let length = (input.Length()? as usize).min(input_capacity).min(output_capacity) / 4 * 4;
    let samples = unsafe { std::slice::from_raw_parts(input_data as *const f32, length / 4) };
    let out = unsafe { std::slice::from_raw_parts_mut(output_data as *mut f32, length / 4) };
    let tap = self.tap.lock().unwrap().clone();
    match tap.filter(|tap| tap.routed.load(Ordering::Acquire)) {
      Some(tap) => {
        let (sample_rate, channels) = *self.format.lock().unwrap();
        tap.push(samples, channels, sample_rate);
        out.fill(0.);
      }
      None => out.copy_from_slice(samples),
    }
    output.SetLength(length as u32)?;
    input_ref.Close()?;
    output_ref.Close()?;
    Ok(())
  }

  fn Close(&self, _reason: MediaEffectClosedReason) -> windows::core::Result<()> {
    self.tap.lock().unwrap().take();
    Ok(())
  }

  fn DiscardQueuedFrames(&self) -> windows::core::Result<()> {
    if let Some(tap) = self.tap.lock().unwrap().as_ref() {
      tap.clear();
    }
    Ok(())
  }
}

#[implement(IActivationFactory)]
struct AudioTapFactory;

impl IActivationFactory_Impl for AudioTapFactory_Impl {
  fn ActivateInstance(&self) -> windows::core::Result<IInspectable> {
    Ok(AudioTapEffect { tap: Mutex::new(None), format: Mutex::new((48000, 2)) }.into())
  }
}

#[no_mangle]
pub unsafe extern "system" fn DllGetActivationFactory(class_id: *mut c_void, factory: *mut *mut c_void) -> HRESULT {
  if factory.is_null() {
    return E_POINTER;
  }
  // Borrowed: the caller keeps the string.
  let class_id = &*(&class_id as *const *mut c_void as *const HSTRING);
  if *class_id == TAP_CLASS {
    *factory = IActivationFactory::from(AudioTapFactory).into_raw();
    S_OK
  } else {
    *factory = std::ptr::null_mut();
    CLASS_E_CLASSNOTAVAILABLE
  }
}

#[no_mangle]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
  S_FALSE
}
