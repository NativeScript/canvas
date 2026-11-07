//! Linux: the player behind @nativescript/canvas-media's Video and Audio views, on GStreamer.
//!
//! `playbin` picks the decoders (VA-API on Intel/AMD with gstreamer-vaapi) and the audio sink.
//! Video takes the GL path when GDK is on EGL (`gl`): `glupload ! glcolorconvert` to RGBA
//! textures in a context sharing GDK's, shown as GdkGLTextures and handed to canvas on the GPU
//! (`frame`). Otherwise (`NS_CANVAS_MEDIA_CPU=1`, GLX, missing GL elements, a GL error before the
//! first frame) `videoconvert` makes RGBA in memory, shown as GdkMemoryTextures.
//!
//! Decoders offering DMABuf (GStreamer >= 1.24 `va`) are imported by `glupload` without a copy.
//! TODO: GdkDmabufTexture (GTK >= 4.14) display and NV12 DMABuf import into Vulkan.

mod frame;
mod gl;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use gst::prelude::*;
use gst_video::prelude::*;
use gtk::{gdk, glib};
use napi::bindgen_prelude::{FnArgs, Function, ObjectFinalize, Uint8Array};
use napi::threadsafe_function::{
  ThreadsafeFunction, ThreadsafeFunctionCallMode, UnknownReturnValue,
};
use napi::{Env, Error, Result};
use napi_derive::napi;

pub use frame::NSCVideoFrame;

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
  /// Samples GL consumers read, with their EGL fences.
  retiring: Mutex<Vec<(gst::Sample, usize)>>,
  imports: Mutex<Vec<frame::Import>>,
  /// Framebuffers reading frames, by EGL context.
  source_framebuffer: Mutex<Vec<(usize, u32)>>,
  caps: OnceLock<frame::Caps>,
}

static NEXT_PLAYER: AtomicU64 = AtomicU64::new(1);

/// `GST_DEBUG=canvasmedia:4` logs the frame path.
static CAT: std::sync::LazyLock<gst::DebugCategory> = std::sync::LazyLock::new(|| {
  gst::DebugCategory::new(
    "canvasmedia",
    gst::DebugColorFlags::empty(),
    Some("canvas-media"),
  )
});

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
  let Some(texture) = gl_texture(&sample, &info).or_else(|| memory_texture(sample, &info)) else {
    return;
  };
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

/// A GL frame as a texture of GDK's, holding the sample until GDK lets go.
fn gl_texture(sample: &gst::Sample, info: &gst_video::VideoInfo) -> Option<gdk::Texture> {
  let memory = sample
    .buffer()?
    .peek_memory(0)
    .downcast_memory_ref::<gst_gl::GLMemory>()?;
  let context = gl::gdk_context()?;
  let texture = memory.texture_id();
  let sample = sample.clone();
  let texture = unsafe {
    gdk::GLTexture::with_release_func(
      &context,
      texture,
      info.width() as i32,
      info.height() as i32,
      move || drop(sample),
    )
  };
  Some(texture.upcast())
}

/// A system-memory frame, wrapped without a copy (GDK uploads it).
fn memory_texture(sample: gst::Sample, info: &gst_video::VideoInfo) -> Option<gdk::Texture> {
  let mapped = sample.buffer_owned()?.into_mapped_buffer_readable().ok()?;
  let bytes = glib::Bytes::from_owned(mapped);
  Some(
    gdk::MemoryTexture::new(
      info.width() as i32,
      info.height() as i32,
      gdk::MemoryFormat::R8g8b8a8,
      &bytes,
      info.stride()[0] as usize,
    )
    .upcast(),
  )
}

/// GL: RGBA textures; system memory and DMABuf inputs (VA-API's GL texture upload meta fails here).
const GL_SINK: &str = "capsfilter caps=\"video/x-raw(memory:GLMemory);video/x-raw(memory:DMABuf);video/x-raw\" ! glupload ! glcolorconvert ! video/x-raw(memory:GLMemory),format=RGBA,texture-target=2D ! appsink name=sink sync=true max-buffers=1 drop=true enable-last-sample=false";
const CPU_SINK: &str = "videoconvert ! video/x-raw,format=RGBA ! appsink name=sink sync=true max-buffers=1 drop=true enable-last-sample=false";

/// The video sink bin: the latest frame, dropping late ones.
fn video_sink(id: u64, frames: &Arc<Frames>, gpu: bool) -> Result<gst::Element> {
  let bin = gst::parse::bin_from_description(if gpu { GL_SINK } else { CPU_SINK }, true)
    .map_err(|e| Error::from_reason(e.to_string()))?;
  let sink = bin
    .by_name("sink")
    .and_then(|sink| sink.downcast::<gst_app::AppSink>().ok())
    .ok_or_else(|| Error::from_reason("no appsink"))?;
  let frames = Arc::clone(frames);
  let take = Arc::new(move |sample: gst::Sample| {
    // Consumers (GDK, canvas) read it in other contexts: it must be complete.
    if let Some(meta) = sample
      .buffer()
      .and_then(|buffer| buffer.meta::<gst_gl::GLSyncMeta>())
    {
      meta.wait_cpu(meta.context());
    }
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
    *frames.sample.lock().unwrap_or_else(|e| e.into_inner()) = Some(sample.clone());
    if frames.id.fetch_add(1, Ordering::AcqRel) == 0 {
      gst::info!(
        CAT,
        "player {id}: {} frames, {:?}",
        if sample
          .buffer()
          .is_some_and(|b| b.peek_memory(0).is_memory_type::<gst_gl::GLMemory>())
        {
          "GL texture"
        } else {
          "system memory"
        },
        sample.caps()
      );
    }
    frames.reap();
    if !frames.queued.swap(true, Ordering::AcqRel) {
      let frames = Arc::clone(&frames);
      glib::MainContext::default().invoke(move || present(id, &frames));
    }
  });
  let preroll = Arc::clone(&take);
  sink.set_callbacks(
    gst_app::AppSinkCallbacks::builder()
      // The first frame shows (and is readable) before playing.
      .new_preroll(move |sink| {
        preroll(sink.pull_preroll().map_err(|_| gst::FlowError::Eos)?);
        Ok(gst::FlowSuccess::Ok)
      })
      .new_sample(move |sink| {
        take(sink.pull_sample().map_err(|_| gst::FlowError::Eos)?);
        Ok(gst::FlowSuccess::Ok)
      })
      .build(),
  );
  Ok(bin.upcast())
}

/// After a GL error before any frame: the same source again, with the system-memory sink.
fn fall_back_to_memory(playbin: &gst::Element, id: u64, frames: &Arc<Frames>) {
  let (_, current, pending) = playbin.state(gst::ClockTime::ZERO);
  let target = if pending == gst::State::VoidPending {
    current
  } else {
    pending
  };
  let _ = playbin.set_state(gst::State::Null);
  let Ok(sink) = video_sink(id, frames, false) else {
    return;
  };
  playbin.set_property("video-sink", sink);
  let _ = playbin.set_state(target.max(gst::State::Paused));
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
  /// Video frames are GL textures.
  gpu: Rc<Cell<bool>>,
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
    // gstreamer-vaapi's postprocessor hands out NV12 that cannot be mapped (iHD): decode only.
    playbin.connect("element-setup", false, |values| {
      if let Some(element) = values.get(1).and_then(|v| v.get::<gst::Element>().ok()) {
        if element
          .factory()
          .is_some_and(|factory| factory.name() == "vaapidecodebin")
        {
          element.set_property("disable-vpp", true);
        }
      }
      None
    });
    let emit: Arc<Emitter> = Arc::new(
      on_event
        .build_threadsafe_function::<(String, Option<String>)>()
        .weak::<true>()
        .build_callback(|ctx| Ok(FnArgs::from(ctx.value)))?,
    );
    let id = NEXT_PLAYER.fetch_add(1, Ordering::Relaxed);
    let frames = Arc::new(Frames::default());
    let video = video.unwrap_or(false) && gtk_ready();
    let gpu = Rc::new(Cell::new(video && gl::share().is_some()));
    let picture = if video {
      let sink = video_sink(id, &frames, gpu.get()).or_else(|error| {
        if !gpu.replace(false) {
          return Err(error);
        }
        eprintln!("canvas-media: no GStreamer GL elements ({error}); video frames stay in memory");
        video_sink(id, &frames, false)
      })?;
      playbin.set_property("video-sink", sink);
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
    if gpu.get() {
      if let Some(bus) = playbin.bus() {
        // Synchronous: GL elements wait for the answer.
        bus.set_sync_handler(|_, message| {
          if let Some(share) = gl::share_ready() {
            gl::answer_need_context(share, message);
          }
          gst::BusSyncReply::Pass
        });
      }
    }
    let bus = playbin.bus().and_then(|bus| {
      let weak = playbin.downgrade();
      let looping = Arc::clone(&looping);
      let gpu = Rc::clone(&gpu);
      let frames = Arc::clone(&frames);
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
            MessageView::Error(error) => {
              let in_sink = playbin
                .property::<Option<gst::Element>>("video-sink")
                .zip(error.src())
                .is_some_and(|(sink, src)| {
                  src == sink.upcast_ref::<gst::Object>() || src.has_as_ancestor(&sink)
                });
              if gpu.get() && in_sink && frames.id.load(Ordering::Acquire) == 0 {
                eprintln!(
                  "canvas-media: GL video failed ({}); video frames stay in memory",
                  error.error()
                );
                gpu.set(false);
                fall_back_to_memory(&playbin, id, &frames);
              } else {
                send("error", Some(error.error().to_string()))
              }
            }
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
      gpu,
      _bus: bus,
    })
  }

  /// The latest frame for canvas (`NSCVideoFrame`): a GL texture when `gpu` (default true) and
  /// the frame is one, else mapped RGBA rows. Null before the first frame.
  #[napi(ts_return_type = "NSCVideoFrame | null")]
  pub fn gpu_frame(&self, gpu: Option<bool>) -> Option<NSCVideoFrame> {
    NSCVideoFrame::latest(self.id, &self.frames, gpu.unwrap_or(true) && self.gpu.get())
  }

  /// Frames are GL textures.
  #[napi(getter)]
  pub fn gpu_frames(&self) -> bool {
    self.gpu.get()
  }

  /// 53 bits of the GL device UUID GL frames are on (`supportsGPUFrames`); 0 if unknown.
  #[napi(getter)]
  pub fn device_key(&self) -> f64 {
    if !self.gpu.get() {
      return 0.;
    }
    let caps = match self.frames.caps.get() {
      Some(caps) => *caps,
      None => {
        let sample = self
          .frames
          .sample
          .lock()
          .unwrap_or_else(|e| e.into_inner())
          .clone();
        let Some(context) = sample.as_ref().and_then(|sample| {
          Some(
            sample
              .buffer()?
              .peek_memory(0)
              .downcast_memory_ref::<gst_gl::GLMemory>()?
              .context()
              .clone(),
          )
        }) else {
          return 0.;
        };
        frame::caps(&self.frames, &context)
      }
    };
    caps.uuid.as_ref().map_or(0., frame::device_key)
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
    self.frames.drop_imports();
    self
      .frames
      .retiring
      .lock()
      .unwrap_or_else(|e| e.into_inner())
      .clear();
    PICTURES.with(|pictures| pictures.borrow_mut().remove(&self.id));
    self.picture = None;
  }
}
