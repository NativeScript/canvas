//! Linux: the player behind @nativescript/canvas-media's Video and Audio views, on GStreamer.
//!
//! `playbin` picks the decoders (VA-API on Intel/AMD when gstreamer-vaapi is installed) and the
//! audio sink. Video is converted to RGBA in system memory (`videoconvert`, one copy out of a
//! hardware decoder's surfaces) and shown by a GtkPicture as a GdkMemoryTexture that wraps the
//! decoded buffer itself; GTK uploads it. Canvas reads (`readPixels`) copy the latest frame.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use gst::prelude::*;
use gst_video::prelude::*;
use gtk::{gdk, glib};
use napi::bindgen_prelude::{FnArgs, Function, ObjectFinalize, Uint8Array};
use napi::threadsafe_function::{
  ThreadsafeFunction, ThreadsafeFunctionCallMode, UnknownReturnValue,
};
use napi::{Env, Error, Result};
use napi_derive::napi;

type EventCallback<'a> = Function<'a, FnArgs<(String, Option<String>)>, UnknownReturnValue>;
type Emitter = ThreadsafeFunction<
  (String, Option<String>),
  UnknownReturnValue,
  FnArgs<(String, Option<String>)>,
  napi::Status,
  false,
  true,
>;

/// `MediaPlaybackState` values, as the Windows bridge reports them.
const STATE_PLAYING: &str = "3";
const STATE_PAUSED: &str = "4";

/// What the streaming thread shares with the UI thread.
#[derive(Default)]
struct Frames {
  sample: Mutex<Option<gst::Sample>>,
  id: AtomicU64,
  width: AtomicU32,
  height: AtomicU32,
  /// A present is queued on the UI thread.
  queued: AtomicBool,
  /// The video size changed since the last present.
  resized: AtomicBool,
}

static NEXT_PLAYER: AtomicU64 = AtomicU64::new(1);

thread_local! {
  /// Pictures by player (UI thread): frames arrive from the streaming thread by id.
  static PICTURES: RefCell<HashMap<u64, (glib::WeakRef<gtk::Picture>, Arc<Emitter>)>> = RefCell::new(HashMap::new());
}

/// GTK-rs wants to have initialized GTK itself; the app already has.
fn gtk_ready() -> bool {
  if !gtk::is_initialized() && unsafe { gtk::ffi::gtk_is_initialized() } != 0 {
    unsafe { gtk::set_initialized() };
  }
  gtk::is_initialized_main_thread()
}

/// UI thread: the latest frame as a texture wrapping the decoded buffer.
fn present(id: u64, frames: &Frames) {
  frames.queued.store(false, Ordering::Release);
  let Some(sample) = frames
    .sample
    .lock()
    .unwrap_or_else(|e| e.into_inner())
    .clone()
  else {
    return;
  };
  let Some(info) = sample
    .caps()
    .and_then(|caps| gst_video::VideoInfo::from_caps(caps).ok())
  else {
    return;
  };
  let Some(buffer) = sample.buffer_owned() else {
    return;
  };
  let Ok(mapped) = buffer.into_mapped_buffer_readable() else {
    return;
  };
  let bytes = glib::Bytes::from_owned(mapped);
  let texture = gdk::MemoryTexture::new(
    info.width() as i32,
    info.height() as i32,
    gdk::MemoryFormat::R8g8b8a8,
    &bytes,
    info.stride()[0] as usize,
  );
  PICTURES.with(|pictures| {
    if let Some((picture, emit)) = pictures.borrow().get(&id) {
      if let Some(picture) = picture.upgrade() {
        picture.set_paintable(Some(&texture));
      }
      if frames.resized.swap(false, Ordering::AcqRel) {
        emit.call(
          (String::from("resize"), None),
          ThreadsafeFunctionCallMode::NonBlocking,
        );
      }
      emit.call(
        (String::from("frame"), None),
        ThreadsafeFunctionCallMode::NonBlocking,
      );
    }
  });
}

/// `videoconvert ! RGBA appsink`: the latest frame, dropping late ones.
fn video_sink(id: u64, frames: &Arc<Frames>) -> Result<gst::Element> {
  let bin = gst::parse::bin_from_description(
    "videoconvert ! video/x-raw,format=RGBA ! appsink name=sink sync=true max-buffers=1 drop=true",
    true,
  )
  .map_err(|e| Error::from_reason(e.to_string()))?;
  let sink = bin
    .by_name("sink")
    .and_then(|sink| sink.downcast::<gst_app::AppSink>().ok())
    .ok_or_else(|| Error::from_reason("no appsink"))?;
  let frames = Arc::clone(frames);
  sink.set_callbacks(
    gst_app::AppSinkCallbacks::builder()
      .new_sample(move |sink| {
        let sample = sink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
        if let Some(info) = sample
          .caps()
          .and_then(|caps| gst_video::VideoInfo::from_caps(caps).ok())
        {
          let width = frames.width.swap(info.width(), Ordering::Relaxed);
          let height = frames.height.swap(info.height(), Ordering::Relaxed);
          if (width, height) != (info.width(), info.height()) {
            frames.resized.store(true, Ordering::Release);
          }
        }
        *frames.sample.lock().unwrap_or_else(|e| e.into_inner()) = Some(sample);
        frames.id.fetch_add(1, Ordering::AcqRel);
        if !frames.queued.swap(true, Ordering::AcqRel) {
          let frames = Arc::clone(&frames);
          glib::MainContext::default().invoke(move || present(id, &frames));
        }
        Ok(gst::FlowSuccess::Ok)
      })
      .build(),
  );
  Ok(bin.upcast())
}

/// `CanvasModule`-free media player: a `playbin`, its events on the JS thread and, for video, a
/// GtkPicture showing it.
#[napi(js_name = "NSCMediaPlayer", custom_finalize)]
pub struct NSCMediaPlayer {
  id: u64,
  playbin: gst::Element,
  picture: Option<gtk::Picture>,
  frames: Arc<Frames>,
  looping: Arc<AtomicBool>,
  _bus: Option<gst::bus::BusWatchGuard>,
}

impl ObjectFinalize for NSCMediaPlayer {
  fn finalize(mut self, _: Env) -> Result<()> {
    self.close();
    Ok(())
  }
}

#[napi]
impl NSCMediaPlayer {
  /// `onEvent(type, detail)`: `opened`, `ended`, `error` (detail: the message), `state` (detail:
  /// 3 playing, 4 paused), `seeked`, `durationchange`, `resize` and, with `video`, `frame`.
  #[napi(
    constructor,
    ts_args_type = "onEvent: (type: string, detail?: string) => void, video?: boolean"
  )]
  pub fn new(on_event: EventCallback, video: Option<bool>) -> Result<Self> {
    gst::init().map_err(|e| Error::from_reason(format!("GStreamer: {e}")))?;
    let playbin = gst::ElementFactory::make("playbin").build().map_err(|_| {
      Error::from_reason("GStreamer's playbin is missing (gstreamer1.0-plugins-base)")
    })?;
    let emit: Arc<Emitter> = Arc::new(
      on_event
        .build_threadsafe_function::<(String, Option<String>)>()
        .weak::<true>()
        .build_callback(|ctx| Ok(FnArgs::from(ctx.value)))?,
    );
    let id = NEXT_PLAYER.fetch_add(1, Ordering::Relaxed);
    let frames = Arc::new(Frames::default());
    let video = video.unwrap_or(false) && gtk_ready();
    let picture = if video {
      playbin.set_property("video-sink", video_sink(id, &frames)?);
      let picture = gtk::Picture::new();
      picture.set_can_shrink(true);
      PICTURES.with(|pictures| {
        pictures
          .borrow_mut()
          .insert(id, (picture.downgrade(), emit.clone()))
      });
      Some(picture)
    } else {
      // Audio only: no video decoding at all.
      playbin.set_property(
        "video-sink",
        gst::ElementFactory::make("fakesink").build().ok(),
      );
      None
    };
    let looping = Arc::new(AtomicBool::new(false));
    let bus = playbin.bus().and_then(|bus| {
      let weak = playbin.downgrade();
      let looping = Arc::clone(&looping);
      let mut opened = false;
      bus
        .add_watch_local(move |_, message| {
          let Some(playbin) = weak.upgrade() else {
            return glib::ControlFlow::Break;
          };
          let send = |kind: &str, detail: Option<String>| {
            emit.call(
              (kind.to_string(), detail),
              ThreadsafeFunctionCallMode::NonBlocking,
            );
          };
          use gst::MessageView;
          match message.view() {
            MessageView::Eos(_) => {
              if looping.load(Ordering::Relaxed) {
                let _ = playbin.seek_simple(
                  gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                  gst::ClockTime::ZERO,
                );
              } else {
                send("ended", None);
              }
            }
            MessageView::Error(error) => send("error", Some(error.error().to_string())),
            MessageView::StateChanged(change) if change.src() == Some(playbin.upcast_ref()) => {
              match change.current() {
                gst::State::Playing => send("state", Some(STATE_PLAYING.into())),
                gst::State::Paused if change.old() == gst::State::Playing => {
                  send("state", Some(STATE_PAUSED.into()))
                }
                _ => {}
              }
            }
            MessageView::AsyncDone(_) => {
              if opened {
                send("seeked", None);
              } else {
                opened = true;
                send("opened", None);
              }
            }
            MessageView::DurationChanged(_) => send("durationchange", None),
            MessageView::StreamStart(_) => opened = false,
            _ => {}
          }
          glib::ControlFlow::Continue
        })
        .ok()
    });
    Ok(Self {
      id,
      playbin,
      picture,
      frames,
      looping,
      _bus: bus,
    })
  }

  /// The GtkPicture's address (video players), for `new interop.Pointer(widget)`.
  #[napi(getter)]
  pub fn widget(&self) -> f64 {
    self
      .picture
      .as_ref()
      .map_or(0., |picture| picture.as_ptr() as usize as f64)
  }

  /// A path or URI; empty unloads.
  #[napi]
  pub fn set_source(&self, source: String) {
    let _ = self.playbin.set_state(gst::State::Null);
    self.frames.id.store(0, Ordering::Release);
    *self.frames.sample.lock().unwrap_or_else(|e| e.into_inner()) = None;
    if source.is_empty() {
      return;
    }
    let uri = if source.contains("://") {
      source
    } else {
      match glib::filename_to_uri(&source, None) {
        Ok(uri) => uri.to_string(),
        Err(_) => return,
      }
    };
    self.playbin.set_property("uri", uri);
    // Prerolls: `opened` follows with the first frame decoded.
    let _ = self.playbin.set_state(gst::State::Paused);
  }

  #[napi]
  pub fn play(&self) -> bool {
    self.playbin.set_state(gst::State::Playing).is_ok()
  }

  #[napi]
  pub fn pause(&self) {
    let _ = self.playbin.set_state(gst::State::Paused);
  }

  /// Seconds.
  #[napi(getter)]
  pub fn current_time(&self) -> f64 {
    self
      .playbin
      .query_position::<gst::ClockTime>()
      .map_or(0., |t| t.nseconds() as f64 / 1e9)
  }

  #[napi(setter)]
  pub fn set_current_time(&self, seconds: f64) {
    if seconds.is_finite() && seconds >= 0. {
      let _ = self.playbin.seek_simple(
        gst::SeekFlags::FLUSH | gst::SeekFlags::ACCURATE,
        gst::ClockTime::from_nseconds((seconds * 1e9) as u64),
      );
    }
  }

  /// Seconds, NaN until known.
  #[napi(getter)]
  pub fn duration(&self) -> f64 {
    self
      .playbin
      .query_duration::<gst::ClockTime>()
      .map_or(f64::NAN, |t| t.nseconds() as f64 / 1e9)
  }

  #[napi(getter)]
  pub fn volume(&self) -> f64 {
    self.playbin.property::<f64>("volume")
  }

  #[napi(setter)]
  pub fn set_volume(&self, volume: f64) {
    self.playbin.set_property("volume", volume.clamp(0., 1.));
  }

  #[napi(getter)]
  pub fn muted(&self) -> bool {
    self.playbin.property::<bool>("mute")
  }

  #[napi(setter)]
  pub fn set_muted(&self, muted: bool) {
    self.playbin.set_property("mute", muted);
  }

  #[napi(getter, js_name = "loop")]
  pub fn looping(&self) -> bool {
    self.looping.load(Ordering::Relaxed)
  }

  #[napi(setter, js_name = "loop")]
  pub fn set_looping(&self, looping: bool) {
    self.looping.store(looping, Ordering::Relaxed);
  }

  #[napi(getter)]
  pub fn video_width(&self) -> u32 {
    self.frames.width.load(Ordering::Relaxed)
  }

  #[napi(getter)]
  pub fn video_height(&self) -> u32 {
    self.frames.height.load(Ordering::Relaxed)
  }

  /// Changes with every decoded frame; 0 before the first.
  #[napi(getter)]
  pub fn frame_id(&self) -> f64 {
    self.frames.id.load(Ordering::Acquire) as f64
  }

  /// Copies the latest frame into `out` (RGBA rows of `videoWidth * 4` bytes, the page's
  /// buffer, reused across frames). `false` without a frame or when `out` is too small.
  #[napi]
  pub fn read_pixels_into(&self, mut out: Uint8Array) -> bool {
    let Some(sample) = self
      .frames
      .sample
      .lock()
      .unwrap_or_else(|e| e.into_inner())
      .clone()
    else {
      return false;
    };
    let Some(info) = sample
      .caps()
      .and_then(|caps| gst_video::VideoInfo::from_caps(caps).ok())
    else {
      return false;
    };
    let Some(buffer) = sample.buffer() else {
      return false;
    };
    let Ok(frame) = gst_video::VideoFrameRef::from_buffer_ref_readable(buffer, &info) else {
      return false;
    };
    let (width, height) = (info.width() as usize, info.height() as usize);
    let row = width * 4;
    let stride = frame.plane_stride()[0] as usize;
    let (Ok(data), out) = (frame.plane_data(0), unsafe { out.as_mut() }) else {
      return false;
    };
    if out.len() < row * height {
      return false;
    }
    if stride == row {
      out[..row * height].copy_from_slice(&data[..row * height]);
    } else {
      for y in 0..height {
        out[y * row..(y + 1) * row].copy_from_slice(&data[y * stride..y * stride + row]);
      }
    }
    true
  }

  #[napi]
  pub fn close(&mut self) {
    let _ = self.playbin.set_state(gst::State::Null);
    self._bus = None;
    PICTURES.with(|pictures| pictures.borrow_mut().remove(&self.id));
    self.picture = None;
  }
}
