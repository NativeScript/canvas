//! GL frames: GStreamer's GL context shares GDK's (one EGL display), so a decoded frame stays a
//! texture from upload to display, and other GL / Vulkan users get it on the GPU.

use std::cell::OnceCell;
use std::ffi::{c_char, c_void, CStr};
use std::sync::OnceLock;

use gst_gl::prelude::*;
use gtk::gdk;
use gtk::gdk::prelude::{DisplayExt as _, GLContextExt as _};

pub const GL_TEXTURE_2D: u32 = 0x0DE1;
pub const GL_RGBA8: u32 = 0x8058;
pub const GL_FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;
pub const GL_READ_FRAMEBUFFER: u32 = 0x8CA8;
pub const GL_DRAW_FRAMEBUFFER: u32 = 0x8CA9;
pub const GL_READ_FRAMEBUFFER_BINDING: u32 = 0x8CAA;
pub const GL_DRAW_FRAMEBUFFER_BINDING: u32 = 0x8CA6;
pub const GL_COLOR_ATTACHMENT0: u32 = 0x8CE0;
pub const GL_COLOR_BUFFER_BIT: u32 = 0x4000;
pub const GL_NEAREST: u32 = 0x2600;
pub const GL_SCISSOR_TEST: u32 = 0x0C11;
pub const GL_EXTENSIONS: u32 = 0x1F03;
pub const GL_NUM_EXTENSIONS: u32 = 0x821D;
pub const GL_TEXTURE_TILING_EXT: u32 = 0x9580;
pub const GL_DEDICATED_MEMORY_OBJECT_EXT: u32 = 0x9581;
pub const GL_OPTIMAL_TILING_EXT: i32 = 0x9584;
pub const GL_HANDLE_TYPE_OPAQUE_FD_EXT: u32 = 0x9586;
pub const GL_DEVICE_UUID_EXT: u32 = 0x9597;
pub const GL_LAYOUT_SHADER_READ_ONLY_EXT: u32 = 0x9591;

const EGL_GL_TEXTURE_2D_KHR: u32 = 0x30B1;
const EGL_GL_TEXTURE_LEVEL_KHR: i32 = 0x30BC;
const EGL_IMAGE_PRESERVED_KHR: i32 = 0x30D2;
const EGL_NONE: i32 = 0x3038;
const EGL_SYNC_STATUS_KHR: i32 = 0x30F1;
const EGL_SIGNALED_KHR: i32 = 0x30F2;

/// EGL and GL entry points (any current context of the driver).
#[allow(non_snake_case)]
pub struct Gl {
  pub GetCurrentDisplay: unsafe extern "C" fn() -> *mut c_void,
  pub GetCurrentContext: unsafe extern "C" fn() -> *mut c_void,
  pub CreateImageKHR:
    unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *mut c_void, *const i32) -> *mut c_void,
  pub DestroyImageKHR: unsafe extern "C" fn(*mut c_void, *mut c_void) -> u32,
  pub GetSyncAttribKHR: unsafe extern "C" fn(*mut c_void, *mut c_void, i32, *mut i32) -> u32,
  pub DestroySyncKHR: unsafe extern "C" fn(*mut c_void, *mut c_void) -> u32,

  pub GetError: unsafe extern "C" fn() -> u32,
  pub GetIntegerv: unsafe extern "C" fn(u32, *mut i32),
  pub GetStringi: unsafe extern "C" fn(u32, u32) -> *const c_char,
  pub IsEnabled: unsafe extern "C" fn(u32) -> u8,
  pub Enable: unsafe extern "C" fn(u32),
  pub Disable: unsafe extern "C" fn(u32),
  pub Flush: unsafe extern "C" fn(),
  pub Finish: unsafe extern "C" fn(),
  pub GenTextures: unsafe extern "C" fn(i32, *mut u32),
  pub DeleteTextures: unsafe extern "C" fn(i32, *const u32),
  pub BindTexture: unsafe extern "C" fn(u32, u32),
  pub TexParameteri: unsafe extern "C" fn(u32, u32, i32),
  pub GenFramebuffers: unsafe extern "C" fn(i32, *mut u32),
  pub DeleteFramebuffers: unsafe extern "C" fn(i32, *const u32),
  pub BindFramebuffer: unsafe extern "C" fn(u32, u32),
  pub FramebufferTexture2D: unsafe extern "C" fn(u32, u32, u32, u32, i32),
  pub CheckFramebufferStatus: unsafe extern "C" fn(u32) -> u32,
  pub BlitFramebuffer: unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32, u32, u32),

  pub CreateMemoryObjectsEXT: unsafe extern "C" fn(i32, *mut u32),
  pub DeleteMemoryObjectsEXT: unsafe extern "C" fn(i32, *const u32),
  pub MemoryObjectParameterivEXT: unsafe extern "C" fn(u32, u32, *const i32),
  pub ImportMemoryFdEXT: unsafe extern "C" fn(u32, u64, u32, i32),
  pub TexStorageMem2DEXT: unsafe extern "C" fn(u32, i32, u32, i32, i32, u32, u64),
  pub GetUnsignedBytevEXT: unsafe extern "C" fn(u32, *mut u8),
  pub GenSemaphoresEXT: unsafe extern "C" fn(i32, *mut u32),
  pub DeleteSemaphoresEXT: unsafe extern "C" fn(i32, *const u32),
  pub ImportSemaphoreFdEXT: unsafe extern "C" fn(u32, u32, i32),
  pub WaitSemaphoreEXT: unsafe extern "C" fn(u32, u32, *const u32, u32, *const u32, *const u32),
  pub SignalSemaphoreEXT: unsafe extern "C" fn(u32, u32, *const u32, u32, *const u32, *const u32),
}

static GL: OnceLock<Option<Gl>> = OnceLock::new();

pub fn gl() -> Option<&'static Gl> {
  GL.get_or_init(|| unsafe { load() }).as_ref()
}

unsafe fn load() -> Option<Gl> {
  let lib = unsafe { libloading::Library::new("libEGL.so.1") }.ok()?;
  let get: libloading::Symbol<unsafe extern "C" fn(*const c_char) -> *mut c_void> =
    unsafe { lib.get(b"eglGetProcAddress\0") }.ok()?;
  let get = *get;
  // Stays loaded: the entry points live in it.
  std::mem::forget(lib);
  macro_rules! f {
    ($name:literal) => {{
      let address = unsafe { get(concat!($name, "\0").as_ptr() as *const c_char) };
      if address.is_null() {
        return None;
      }
      unsafe { std::mem::transmute::<*mut c_void, _>(address) }
    }};
  }
  Some(Gl {
    GetCurrentDisplay: f!("eglGetCurrentDisplay"),
    GetCurrentContext: f!("eglGetCurrentContext"),
    CreateImageKHR: f!("eglCreateImageKHR"),
    DestroyImageKHR: f!("eglDestroyImageKHR"),
    GetSyncAttribKHR: f!("eglGetSyncAttribKHR"),
    DestroySyncKHR: f!("eglDestroySyncKHR"),
    GetError: f!("glGetError"),
    GetIntegerv: f!("glGetIntegerv"),
    GetStringi: f!("glGetStringi"),
    IsEnabled: f!("glIsEnabled"),
    Enable: f!("glEnable"),
    Disable: f!("glDisable"),
    Flush: f!("glFlush"),
    Finish: f!("glFinish"),
    GenTextures: f!("glGenTextures"),
    DeleteTextures: f!("glDeleteTextures"),
    BindTexture: f!("glBindTexture"),
    TexParameteri: f!("glTexParameteri"),
    GenFramebuffers: f!("glGenFramebuffers"),
    DeleteFramebuffers: f!("glDeleteFramebuffers"),
    BindFramebuffer: f!("glBindFramebuffer"),
    FramebufferTexture2D: f!("glFramebufferTexture2D"),
    CheckFramebufferStatus: f!("glCheckFramebufferStatus"),
    BlitFramebuffer: f!("glBlitFramebuffer"),
    CreateMemoryObjectsEXT: f!("glCreateMemoryObjectsEXT"),
    DeleteMemoryObjectsEXT: f!("glDeleteMemoryObjectsEXT"),
    MemoryObjectParameterivEXT: f!("glMemoryObjectParameterivEXT"),
    ImportMemoryFdEXT: f!("glImportMemoryFdEXT"),
    TexStorageMem2DEXT: f!("glTexStorageMem2DEXT"),
    GetUnsignedBytevEXT: f!("glGetUnsignedBytevEXT"),
    GenSemaphoresEXT: f!("glGenSemaphoresEXT"),
    DeleteSemaphoresEXT: f!("glDeleteSemaphoresEXT"),
    ImportSemaphoreFdEXT: f!("glImportSemaphoreFdEXT"),
    WaitSemaphoreEXT: f!("glWaitSemaphoreEXT"),
    SignalSemaphoreEXT: f!("glSignalSemaphoreEXT"),
  })
}

impl Gl {
  /// The current context's extensions (GL 3 / GLES 3).
  pub unsafe fn has_extension(&self, name: &str) -> bool {
    let mut count = 0;
    unsafe { (self.GetIntegerv)(GL_NUM_EXTENSIONS, &mut count) };
    (0..count.max(0) as u32).any(|i| {
      let extension = unsafe { (self.GetStringi)(GL_EXTENSIONS, i) };
      !extension.is_null() && unsafe { CStr::from_ptr(extension) }.to_bytes() == name.as_bytes()
    })
  }

  /// `texture` (of the current context's share group) as an EGLImage.
  pub unsafe fn texture_image(&self, display: *mut c_void, texture: u32) -> *mut c_void {
    let attributes = [
      EGL_GL_TEXTURE_LEVEL_KHR,
      0,
      EGL_IMAGE_PRESERVED_KHR,
      1,
      EGL_NONE,
    ];
    unsafe {
      (self.CreateImageKHR)(
        display,
        (self.GetCurrentContext)(),
        EGL_GL_TEXTURE_2D_KHR,
        texture as usize as *mut c_void,
        attributes.as_ptr(),
      )
    }
  }

  /// A consumer's EGL fence signalled (or gone); destroys it then.
  pub unsafe fn sync_done(&self, display: *mut c_void, sync: *mut c_void) -> bool {
    let mut status = 0;
    let known = unsafe { (self.GetSyncAttribKHR)(display, sync, EGL_SYNC_STATUS_KHR, &mut status) };
    let done = known == 0 || status == EGL_SIGNALED_KHR;
    if done {
      unsafe { (self.DestroySyncKHR)(display, sync) };
    }
    done
  }
}

/// What the streaming threads need to put GStreamer's GL on GDK's display.
pub struct Share {
  pub display: gst_gl::GLDisplay,
  /// GDK's context, wrapped: GStreamer's shares with it.
  pub app: gst_gl::GLContext,
  pub egl_display: usize,
}

static SHARE: OnceLock<Option<Share>> = OnceLock::new();

thread_local! {
  /// The GDK context frames are shown with (UI thread).
  static GDK: OnceCell<gdk::GLContext> = const { OnceCell::new() };
}

/// `NS_CANVAS_MEDIA_CPU=1`: system-memory frames only.
pub fn forced_cpu() -> bool {
  std::env::var_os("NS_CANVAS_MEDIA_CPU").is_some_and(|v| !v.is_empty() && v != "0")
}

/// UI thread: the GL share, set up once. None without GDK EGL (GLX, no GL) or when forced off.
pub fn share() -> Option<&'static Share> {
  SHARE
    .get_or_init(|| {
      if forced_cpu() {
        return None;
      }
      let share = unsafe { init() };
      if share.is_none() {
        eprintln!("canvas-media: GDK has no shareable EGL context; video frames stay in memory");
      }
      share
    })
    .as_ref()
}

/// The share if it was set up, without setting it up (any thread).
pub fn share_ready() -> Option<&'static Share> {
  SHARE.get().and_then(Option::as_ref)
}

pub fn gdk_context() -> Option<gdk::GLContext> {
  GDK.with(|gdk| gdk.get().cloned())
}

unsafe fn init() -> Option<Share> {
  let gl = gl()?;
  let context = gdk::Display::default()?.create_gl_context().ok()?;
  context.realize().ok()?;
  context.make_current();
  let egl_display = unsafe { (gl.GetCurrentDisplay)() };
  let egl_context = unsafe { (gl.GetCurrentContext)() };
  let share = (|| {
    if egl_display.is_null() || egl_context.is_null() {
      return None;
    }
    let api = if context.uses_es() {
      gst_gl::GLAPI::GLES2
    } else {
      gst_gl::GLAPI::OPENGL3
    };
    let display = unsafe { gst_gl_egl::GLDisplayEGL::with_egl_display(egl_display as usize) }
      .ok()?
      .upcast::<gst_gl::GLDisplay>();
    let app = unsafe {
      gst_gl::GLContext::new_wrapped(&display, egl_context as usize, gst_gl::GLPlatform::EGL, api)
    }?;
    app.activate(true).ok()?;
    let filled = app.fill_info();
    let _ = app.activate(false);
    filled.ok()?;
    Some(Share {
      display,
      app,
      egl_display: egl_display as usize,
    })
  })();
  gdk::GLContext::clear_current();
  if share.is_some() {
    GDK.with(|gdk| {
      let _ = gdk.set(context);
    });
  }
  share
}

/// Answers GStreamer GL's `need-context` with GDK's display and context.
pub fn answer_need_context(share: &Share, message: &gst::Message) {
  let gst::MessageView::NeedContext(need) = message.view() else {
    return;
  };
  let Some(element) = message
    .src()
    .and_then(|src| src.downcast_ref::<gst::Element>())
  else {
    return;
  };
  let context = match need.context_type() {
    t if t == gst_gl::GL_DISPLAY_CONTEXT_TYPE.as_str() => {
      let context = gst::Context::new(t, true);
      context.set_gl_display(&share.display);
      context
    }
    "gst.gl.app_context" => {
      let mut context = gst::Context::new("gst.gl.app_context", true);
      context
        .get_mut()
        .unwrap()
        .structure_mut()
        .set("context", &share.app);
      context
    }
    _ => return,
  };
  element.set_context(&context);
}
