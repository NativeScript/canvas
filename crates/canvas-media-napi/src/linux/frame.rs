//! Frames handed to canvas natively, as `nativeTexture` / `__texImageVideoFrame` addresses.
//!
//! GL frames are GStreamer textures: GL consumers (canvas WebGL, on GDK's EGL display) read them
//! through an EGLImage, WebGPU (Vulkan) consumers through `copy_to`, which blits the frame into
//! their exportable image here (`GL_EXT_memory_object_fd`), synchronized with their semaphores
//! (`GL_EXT_semaphore_fd`). Other frames are RGBA rows in mapped memory, uploaded once.

use std::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

use gst_gl::prelude::*;
use gtk::gdk::prelude::GLContextExt as _;
use gtk::{gdk, glib};
use napi::bindgen_prelude::ObjectFinalize;
use napi::{Env, Result};
use napi_derive::napi;

use super::gl::{self, gl, Gl};
use super::Frames;

/// Same layout as `CanvasLinuxVideoFrame` in canvas-c.
#[repr(C)]
pub struct LinuxVideoFrame {
  pub size: u32,
  /// `KIND_GL` or `KIND_MEMORY`.
  pub kind: u32,
  pub width: u32,
  pub height: u32,
  /// The player: consumers key their per-producer caches with it.
  pub producer: u64,
  pub egl_display: *mut c_void,
  /// GL: the frame texture, owned by the producer.
  pub egl_image: *mut c_void,
  /// Changes only when `egl_image` does.
  pub image_id: u64,
  /// Memory: RGBA rows.
  pub data: *const u8,
  pub stride: u32,
  pub reserved: u32,
  /// GL: the GPU's (GL_EXT_memory_object), zeros if unknown.
  pub device_uuid: [u8; 16],
  /// GL: blits the frame into a consumer's Vulkan image.
  pub copy_to: Option<unsafe extern "C" fn(*mut LinuxVideoFrame, *const LinuxFrameTarget) -> bool>,
  /// GL: an EGLSync a GL consumer sets after its reads; the texture is reused once it signals.
  pub release_sync: *mut c_void,
  pub user: *mut c_void,
}

pub const KIND_GL: u32 = 1;
pub const KIND_MEMORY: u32 = 2;

/// Same layout as `CanvasLinuxFrameTarget` in canvas-c: an exportable RGBA8 Vulkan image.
#[repr(C)]
pub struct LinuxFrameTarget {
  pub size: u32,
  /// OPAQUE_FDs, sent the first time an id is used (the producer owns them), -1 after.
  pub memory_fd: i32,
  pub id: u64,
  pub allocation_size: u64,
  pub width: u32,
  pub height: u32,
  /// Signalled here once the image is written; -1 without semaphores (then finished here).
  pub ready_fd: i32,
  /// Signalled by the consumer once its reads are done.
  pub release_fd: i32,
  /// The release semaphore has a signal pending: wait for it before writing.
  pub wait_release: u32,
  pub reserved: u32,
}

/// A Vulkan image imported into the GL share group (GDK's and GStreamer's contexts).
pub(super) struct Import {
  id: u64,
  memory: u32,
  texture: u32,
  ready: u32,
  release: u32,
  /// Framebuffers are per context: by EGL context.
  framebuffers: Vec<(usize, u32)>,
}

const MAX_IMPORTS: usize = 6;

/// What a GL context offers consumers.
#[derive(Clone, Copy, Default)]
pub(super) struct Caps {
  pub uuid: Option<[u8; 16]>,
  pub semaphores: bool,
}

static NEXT_IMAGE: AtomicU64 = AtomicU64::new(1);

struct Image {
  display: usize,
  image: usize,
  id: u64,
}

unsafe extern "C" fn drop_image(data: *mut c_void) {
  let image = unsafe { Box::from_raw(data as *mut Image) };
  if let Some(gl) = gl() {
    unsafe { (gl.DestroyImageKHR)(image.display as *mut c_void, image.image as *mut c_void) };
  }
}

fn image_quark() -> glib::ffi::GQuark {
  static QUARK: OnceLock<u32> = OnceLock::new();
  *QUARK.get_or_init(|| unsafe { glib::ffi::g_quark_from_static_string(c"nsc-egl-image".as_ptr()) })
}

/// Sends what `thread_add` runs: it runs before returning, on GStreamer's GL thread.
struct OnGl<T>(T);
unsafe impl<T> Send for OnGl<T> {}

/// The memory's texture as an EGLImage, made once per pool texture and freed with it.
fn egl_image(memory: &gst_gl::GLMemoryRef, display: usize) -> Option<(usize, u64)> {
  let object = memory.as_ptr() as *mut gst::ffi::GstMiniObject;
  let existing = unsafe { gst::ffi::gst_mini_object_get_qdata(object, image_quark()) };
  if !existing.is_null() {
    let image = unsafe { &*(existing as *const Image) };
    return Some((image.image, image.id));
  }
  let gl = gl()?;
  let texture = memory.texture_id();
  let mut image = 0usize;
  let out = OnGl(&mut image as *mut usize);
  memory.context().thread_add(move |_| {
    let out = out;
    unsafe { *out.0 = gl.texture_image(display as *mut c_void, texture) as usize };
  });
  if image == 0 {
    return None;
  }
  let id = NEXT_IMAGE.fetch_add(1, Ordering::Relaxed);
  let data = Box::into_raw(Box::new(Image { display, image, id }));
  unsafe {
    gst::ffi::gst_mini_object_set_qdata(
      object,
      image_quark(),
      data as *mut c_void,
      Some(drop_image),
    )
  };
  Some((image, id))
}

/// GL thread: the context's interop support.
unsafe fn query_caps(gl: &Gl) -> Caps {
  unsafe {
    if !gl.has_extension("GL_EXT_memory_object_fd") {
      return Caps::default();
    }
    let mut uuid = [0u8; 16];
    while (gl.GetError)() != 0 {}
    (gl.GetUnsignedBytevEXT)(gl::GL_DEVICE_UUID_EXT, uuid.as_mut_ptr());
    Caps {
      uuid: ((gl.GetError)() == 0).then_some(uuid),
      semaphores: gl.has_extension("GL_EXT_semaphore_fd"),
    }
  }
}

pub(super) fn caps(frames: &Frames, context: &gst_gl::GLContext) -> Caps {
  *frames.caps.get_or_init(|| {
    let Some(gl) = gl() else {
      return Caps::default();
    };
    let mut caps = Caps::default();
    let out = OnGl(&mut caps as *mut Caps);
    context.thread_add(move |_| {
      let out = out;
      unsafe { *out.0 = query_caps(gl) };
    });
    caps
  })
}

/// 53 bits of the device UUID, for JS (`supportsGPUFrames(device)`); 0 if unknown.
pub fn device_key(uuid: &[u8; 16]) -> f64 {
  let mut key = 0u64;
  for (i, byte) in uuid.iter().enumerate() {
    key ^= (*byte as u64) << ((i % 7) * 8);
  }
  (key & ((1 << 53) - 1)) as f64
}

struct Held {
  desc: LinuxVideoFrame,
  sample: Option<gst::Sample>,
  _mapped: Option<gst_video::VideoFrame<gst_video::video_frame::Readable>>,
  frames: Arc<Frames>,
}

/// A frame handed out: keep it until the consumer's call returned, then `close()`.
#[napi(js_name = "NSCVideoFrame", custom_finalize)]
pub struct NSCVideoFrame {
  held: Option<Box<Held>>,
}

impl ObjectFinalize for NSCVideoFrame {
  fn finalize(mut self, _: Env) -> Result<()> {
    self.close();
    Ok(())
  }
}

#[napi]
impl NSCVideoFrame {
  /// The `CanvasLinuxVideoFrame`'s address.
  #[napi(getter)]
  pub fn address(&self) -> f64 {
    self.held.as_ref().map_or(0., |held| {
      &held.desc as *const LinuxVideoFrame as usize as f64
    })
  }

  #[napi(getter)]
  pub fn width(&self) -> u32 {
    self.held.as_ref().map_or(0, |held| held.desc.width)
  }

  #[napi(getter)]
  pub fn height(&self) -> u32 {
    self.held.as_ref().map_or(0, |held| held.desc.height)
  }

  /// `true` for a GL texture, `false` for mapped memory.
  #[napi(getter)]
  pub fn gpu(&self) -> bool {
    self
      .held
      .as_ref()
      .is_some_and(|held| held.desc.kind == KIND_GL)
  }

  #[napi]
  pub fn close(&mut self) {
    let Some(held) = self.held.take() else {
      return;
    };
    let Held {
      desc,
      sample,
      frames,
      ..
    } = *held;
    match sample {
      Some(sample) if !desc.release_sync.is_null() => {
        frames.retire(sample, desc.release_sync as usize)
      }
      _ => {}
    }
    frames.reap();
  }
}

impl NSCVideoFrame {
  /// The latest frame: a GL texture when `gpu` allows and the frame is one, else mapped memory
  /// (a GL frame is downloaded).
  pub(super) fn latest(producer: u64, frames: &Arc<Frames>, gpu: bool) -> Option<Self> {
    let sample = frames
      .sample
      .lock()
      .unwrap_or_else(|e| e.into_inner())
      .clone()?;
    let info = sample
      .caps()
      .and_then(|caps| gst_video::VideoInfo::from_caps(caps).ok())?;
    let mut desc = LinuxVideoFrame {
      size: std::mem::size_of::<LinuxVideoFrame>() as u32,
      kind: KIND_MEMORY,
      width: info.width(),
      height: info.height(),
      producer,
      egl_display: std::ptr::null_mut(),
      egl_image: std::ptr::null_mut(),
      image_id: 0,
      data: std::ptr::null(),
      stride: 0,
      reserved: 0,
      device_uuid: [0; 16],
      copy_to: None,
      release_sync: std::ptr::null_mut(),
      user: std::ptr::null_mut(),
    };
    let buffer = sample.buffer_owned()?;
    if gpu {
      let memory = buffer
        .peek_memory(0)
        .downcast_memory_ref::<gst_gl::GLMemory>();
      if let (Some(memory), Some(share)) = (memory, gl::share()) {
        if let Some((image, image_id)) = egl_image(memory, share.egl_display) {
          let caps = caps(frames, memory.context());
          desc.kind = KIND_GL;
          desc.egl_display = share.egl_display as *mut c_void;
          desc.egl_image = image as *mut c_void;
          desc.image_id = image_id;
          if let Some(uuid) = caps.uuid {
            desc.device_uuid = uuid;
            desc.copy_to = Some(copy_to);
          }
          let mut held = Box::new(Held {
            desc,
            sample: Some(sample),
            _mapped: None,
            frames: Arc::clone(frames),
          });
          held.desc.user = &*held as *const Held as *mut c_void;
          return Some(Self { held: Some(held) });
        }
      }
    }
    let mapped = gst_video::VideoFrame::from_buffer_readable(buffer, &info).ok()?;
    desc.data = mapped.plane_data(0).ok()?.as_ptr();
    desc.stride = mapped.plane_stride()[0] as u32;
    let mut held = Box::new(Held {
      desc,
      sample: None,
      _mapped: Some(mapped),
      frames: Arc::clone(frames),
    });
    held.desc.user = &*held as *const Held as *mut c_void;
    Some(Self { held: Some(held) })
  }
}

impl Frames {
  /// Keeps a sample a GL consumer read until its fence signals.
  fn retire(&self, sample: gst::Sample, sync: usize) {
    self
      .retiring
      .lock()
      .unwrap_or_else(|e| e.into_inner())
      .push((sample, sync));
  }

  pub(super) fn reap(&self) {
    let (Some(gl), Some(share)) = (gl(), gl::share_ready()) else {
      return;
    };
    let display = share.egl_display as *mut c_void;
    self
      .retiring
      .lock()
      .unwrap_or_else(|e| e.into_inner())
      .retain(|(_, sync)| !unsafe { gl.sync_done(display, *sync as *mut c_void) });
  }

  /// Frees the Vulkan images imported for consumers (UI thread: in GDK's context, which shares
  /// them; framebuffers of GStreamer's context go with it).
  pub(super) fn drop_imports(&self) {
    let imports = std::mem::take(&mut *self.imports.lock().unwrap_or_else(|e| e.into_inner()));
    let sources = std::mem::take(
      &mut *self
        .source_framebuffer
        .lock()
        .unwrap_or_else(|e| e.into_inner()),
    );
    let (Some(gl), Some(context)) = (gl(), gl::gdk_context()) else {
      return;
    };
    if imports.is_empty() && sources.is_empty() {
      return;
    }
    context.make_current();
    let current = unsafe { (gl.GetCurrentContext)() } as usize;
    unsafe {
      for import in &imports {
        delete_import(gl, import, current);
      }
      for (owner, framebuffer) in sources {
        if owner == current {
          (gl.DeleteFramebuffers)(1, &framebuffer);
        }
      }
    }
    gdk::GLContext::clear_current();
  }
}

/// With a context of the share group current; only its framebuffers can go.
unsafe fn delete_import(gl: &Gl, import: &Import, current: usize) {
  unsafe {
    for (owner, framebuffer) in &import.framebuffers {
      if *owner == current {
        (gl.DeleteFramebuffers)(1, framebuffer);
      }
    }
    (gl.DeleteTextures)(1, &import.texture);
    (gl.DeleteMemoryObjectsEXT)(1, &import.memory);
    for semaphore in [import.ready, import.release] {
      if semaphore != 0 {
        (gl.DeleteSemaphoresEXT)(1, &semaphore);
      }
    }
  }
}

/// A share-group context current: the consumer's image, imported the first time (taking its fds).
unsafe fn import(gl: &Gl, target: &LinuxFrameTarget, semaphores: bool) -> Option<Import> {
  if target.memory_fd < 0 {
    return None;
  }
  unsafe {
    while (gl.GetError)() != 0 {}
    let mut memory = 0;
    (gl.CreateMemoryObjectsEXT)(1, &mut memory);
    (gl.MemoryObjectParameterivEXT)(memory, gl::GL_DEDICATED_MEMORY_OBJECT_EXT, &1);
    (gl.ImportMemoryFdEXT)(
      memory,
      target.allocation_size,
      gl::GL_HANDLE_TYPE_OPAQUE_FD_EXT,
      target.memory_fd,
    );
    let mut texture = 0;
    (gl.GenTextures)(1, &mut texture);
    (gl.BindTexture)(gl::GL_TEXTURE_2D, texture);
    (gl.TexParameteri)(
      gl::GL_TEXTURE_2D,
      gl::GL_TEXTURE_TILING_EXT,
      gl::GL_OPTIMAL_TILING_EXT,
    );
    (gl.TexStorageMem2DEXT)(
      gl::GL_TEXTURE_2D,
      1,
      gl::GL_RGBA8,
      target.width as i32,
      target.height as i32,
      memory,
      0,
    );
    (gl.BindTexture)(gl::GL_TEXTURE_2D, 0);
    let (mut ready, mut release) = (0, 0);
    if semaphores && target.ready_fd >= 0 && target.release_fd >= 0 {
      (gl.GenSemaphoresEXT)(1, &mut ready);
      (gl.ImportSemaphoreFdEXT)(ready, gl::GL_HANDLE_TYPE_OPAQUE_FD_EXT, target.ready_fd);
      (gl.GenSemaphoresEXT)(1, &mut release);
      (gl.ImportSemaphoreFdEXT)(release, gl::GL_HANDLE_TYPE_OPAQUE_FD_EXT, target.release_fd);
    } else {
      for fd in [target.ready_fd, target.release_fd] {
        if fd >= 0 {
          libc_close(fd);
        }
      }
    }
    let import = Import {
      id: target.id,
      memory,
      texture,
      ready,
      release,
      framebuffers: Vec::new(),
    };
    if (gl.GetError)() != 0 {
      delete_import(gl, &import, 0);
      eprintln!("canvas-media: could not import a WebGPU image into GStreamer's GL");
      return None;
    }
    Some(import)
  }
}

fn libc_close(fd: i32) {
  extern "C" {
    fn close(fd: i32) -> i32;
  }
  unsafe { close(fd) };
}

/// `texture` as a framebuffer of the current context, made once; 0 if incomplete.
unsafe fn framebuffer(
  gl: &Gl,
  framebuffers: &mut Vec<(usize, u32)>,
  context: usize,
  texture: u32,
) -> u32 {
  if let Some((_, framebuffer)) = framebuffers.iter().find(|(owner, _)| *owner == context) {
    return *framebuffer;
  }
  let mut framebuffer = 0;
  unsafe {
    (gl.GenFramebuffers)(1, &mut framebuffer);
    (gl.BindFramebuffer)(gl::GL_DRAW_FRAMEBUFFER, framebuffer);
    (gl.FramebufferTexture2D)(
      gl::GL_DRAW_FRAMEBUFFER,
      gl::GL_COLOR_ATTACHMENT0,
      gl::GL_TEXTURE_2D,
      texture,
      0,
    );
    if (gl.CheckFramebufferStatus)(gl::GL_DRAW_FRAMEBUFFER) != gl::GL_FRAMEBUFFER_COMPLETE {
      (gl.DeleteFramebuffers)(1, &framebuffer);
      return 0;
    }
  }
  framebuffers.push((context, framebuffer));
  framebuffer
}

/// A share-group context current: the frame texture into the consumer's image.
unsafe fn blit(gl: &Gl, frames: &Frames, texture: u32, target: &LinuxFrameTarget) -> bool {
  let context = unsafe { (gl.GetCurrentContext)() } as usize;
  let semaphores = frames.caps.get().is_some_and(|caps| caps.semaphores);
  let mut imports = frames.imports.lock().unwrap_or_else(|e| e.into_inner());
  let index = match imports.iter().position(|import| import.id == target.id) {
    Some(index) => index,
    None => {
      let Some(import) = (unsafe { import(gl, target, semaphores) }) else {
        return false;
      };
      if imports.len() >= MAX_IMPORTS {
        let old = imports.remove(0);
        unsafe { delete_import(gl, &old, context) };
      }
      imports.push(import);
      imports.len() - 1
    }
  };
  let import = &mut imports[index];
  unsafe {
    while (gl.GetError)() != 0 {}
    let (mut read, mut draw) = (0, 0);
    (gl.GetIntegerv)(gl::GL_READ_FRAMEBUFFER_BINDING, &mut read);
    (gl.GetIntegerv)(gl::GL_DRAW_FRAMEBUFFER_BINDING, &mut draw);
    let scissor = (gl.IsEnabled)(gl::GL_SCISSOR_TEST) != 0;
    if import.release != 0 && target.wait_release != 0 {
      let layout = gl::GL_LAYOUT_SHADER_READ_ONLY_EXT;
      (gl.WaitSemaphoreEXT)(
        import.release,
        0,
        std::ptr::null(),
        1,
        &import.texture,
        &layout,
      );
    }
    let destination = framebuffer(gl, &mut import.framebuffers, context, import.texture);
    if destination == 0 {
      (gl.BindFramebuffer)(gl::GL_DRAW_FRAMEBUFFER, draw as u32);
      eprintln!("canvas-media: a WebGPU image is not renderable in GL");
      return false;
    }
    let source = {
      let mut sources = frames
        .source_framebuffer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
      match sources.iter().find(|(owner, _)| *owner == context) {
        Some((_, source)) => *source,
        None => {
          let mut source = 0;
          (gl.GenFramebuffers)(1, &mut source);
          sources.push((context, source));
          source
        }
      }
    };
    (gl.BindFramebuffer)(gl::GL_READ_FRAMEBUFFER, source);
    (gl.FramebufferTexture2D)(
      gl::GL_READ_FRAMEBUFFER,
      gl::GL_COLOR_ATTACHMENT0,
      gl::GL_TEXTURE_2D,
      texture,
      0,
    );
    (gl.BindFramebuffer)(gl::GL_DRAW_FRAMEBUFFER, destination);
    (gl.Disable)(gl::GL_SCISSOR_TEST);
    let (width, height) = (target.width as i32, target.height as i32);
    (gl.BlitFramebuffer)(
      0,
      0,
      width,
      height,
      0,
      0,
      width,
      height,
      gl::GL_COLOR_BUFFER_BIT,
      gl::GL_NEAREST,
    );
    (gl.FramebufferTexture2D)(
      gl::GL_READ_FRAMEBUFFER,
      gl::GL_COLOR_ATTACHMENT0,
      gl::GL_TEXTURE_2D,
      0,
      0,
    );
    if scissor {
      (gl.Enable)(gl::GL_SCISSOR_TEST);
    }
    (gl.BindFramebuffer)(gl::GL_READ_FRAMEBUFFER, read as u32);
    (gl.BindFramebuffer)(gl::GL_DRAW_FRAMEBUFFER, draw as u32);
    if import.ready != 0 {
      let layout = gl::GL_LAYOUT_SHADER_READ_ONLY_EXT;
      (gl.SignalSemaphoreEXT)(
        import.ready,
        0,
        std::ptr::null(),
        1,
        &import.texture,
        &layout,
      );
      (gl.Flush)();
    } else {
      // No semaphores: Vulkan reads it after this returns.
      (gl.Finish)();
    }
    (gl.GetError)() == 0
  }
}

unsafe extern "C" fn copy_to(frame: *mut LinuxVideoFrame, target: *const LinuxFrameTarget) -> bool {
  let (Some(frame), Some(target)) = (unsafe { frame.as_ref() }, unsafe { target.as_ref() }) else {
    return false;
  };
  if target.size as usize != std::mem::size_of::<LinuxFrameTarget>()
    || (target.width, target.height) != (frame.width, frame.height)
    || frame.user.is_null()
  {
    return false;
  }
  let held = unsafe { &*(frame.user as *const Held) };
  let (Some(sample), Some(gl)) = (held.sample.as_ref(), gl()) else {
    return false;
  };
  let Some(memory) = sample.buffer().and_then(|buffer| {
    buffer
      .peek_memory(0)
      .downcast_memory_ref::<gst_gl::GLMemory>()
  }) else {
    return false;
  };
  let texture = memory.texture_id();
  // UI thread: GDK's context shares the texture, without waiting for GStreamer's GL thread.
  if let Some(gdk) = gl::gdk_context() {
    gdk.make_current();
    let done = unsafe { blit(gl, &held.frames, texture, target) };
    gdk::GLContext::clear_current();
    return done;
  }
  let mut done = false;
  let job = OnGl((
    &mut done as *mut bool,
    &*held.frames as *const Frames,
    target as *const LinuxFrameTarget,
  ));
  memory.context().thread_add(move |_| {
    let job = job;
    let (done, frames, target) = job.0;
    unsafe { *done = blit(gl, &*frames, texture, &*target) };
  });
  done
}
