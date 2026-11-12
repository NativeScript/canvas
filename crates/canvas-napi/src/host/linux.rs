//! `CanvasModule.NSCCanvas` on Linux: the native side of a canvas view, a GTK 4 (>= 4.6) widget.
//!
//! Contexts are GLES on the system EGL (canvas-core's `egl`), rendering into pbuffers on GDK's
//! EGL display. At frame end the pbuffer is blitted into a texture GDK sees through an EGLImage
//! and shown as a `GdkGLTexture`, so frames never leave the GPU. When GDK is not on EGL (GLX) or
//! lacks EGLImage support, frames are read back into memory textures instead.
//!
//! The drawing buffer is sized in physical pixels (300x150 until set, as on the web); the widget
//! places it with the `fit` mode (`canvas_core::fit`) at its scale factor.

use std::cell::{Cell, OnceCell, RefCell};
use std::collections::HashMap;
use std::ffi::c_void;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use canvas_c::{CanvasRenderingContext2D as CCanvasRenderingContext2D, WebGLState};
use canvas_core::fit::{surface_transform, CanvasFit};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene};
use napi::bindgen_prelude::ObjectFinalize;
use napi::{Env, Error, Result};
use napi_derive::napi;

mod imp {
  use super::*;

  #[derive(Default)]
  pub struct CanvasView {
    pub frame: RefCell<Option<gdk::Texture>>,
    /// GL frames are bottom-up.
    pub flipped: Cell<bool>,
    pub fit: Cell<CanvasFit>,
    /// The drawing buffer size its owner last set.
    pub size: Cell<(i32, i32)>,
    /// Buffer pixels per DIP, as the TS view reports it.
    pub density: Cell<f32>,
  }

  #[glib::object_subclass]
  impl ObjectSubclass for CanvasView {
    const NAME: &'static str = "NSCCanvasView";
    type Type = super::CanvasView;
    type ParentType = gtk::Widget;
  }

  impl ObjectImpl for CanvasView {}

  impl WidgetImpl for CanvasView {
    fn measure(&self, _: gtk::Orientation, _: i32) -> (i32, i32, i32, i32) {
      (0, 0, -1, -1)
    }

    fn snapshot(&self, snapshot: &gtk::Snapshot) {
      let Some(frame) = self.frame.borrow().clone() else {
        return;
      };
      let widget = self.obj();
      let (width, height) = (
        WidgetExt::width(&*widget) as f32,
        WidgetExt::height(&*widget) as f32,
      );
      let scale = self.density.get();
      let (frame_width, frame_height) = (
        TextureExt::width(&frame) as f32,
        TextureExt::height(&frame) as f32,
      );
      let t = surface_transform(
        self.fit.get(),
        (frame_width, frame_height),
        (scale, scale),
        (width, height),
      );
      let rect = graphene::Rect::new(
        t.offset_x,
        t.offset_y,
        frame_width * t.scale_x,
        frame_height * t.scale_y,
      );
      snapshot.push_clip(&graphene::Rect::new(0., 0., width, height));
      if self.flipped.get() {
        snapshot.save();
        snapshot.translate(&graphene::Point::new(0., 2. * rect.y() + rect.height()));
        snapshot.scale(1., -1.);
        snapshot.append_texture(&frame, &rect);
        snapshot.restore();
      } else {
        snapshot.append_texture(&frame, &rect);
      }
      snapshot.pop();
    }
  }
}

glib::wrapper! {
  pub struct CanvasView(ObjectSubclass<imp::CanvasView>)
    @extends gtk::Widget,
    @implements gtk::Buildable, gtk::ConstraintTarget;
}

impl CanvasView {
  fn show(&self, frame: gdk::Texture, flipped: bool) {
    let imp = self.imp();
    imp.flipped.set(flipped);
    imp.frame.replace(Some(frame));
    self.queue_draw();
  }

  fn set_fit(&self, fit: CanvasFit) {
    self.imp().fit.set(fit);
    self.queue_draw();
  }

  fn set_size(&self, width: u32, height: u32) {
    self.imp().size.set((width as i32, height as i32));
  }

  fn set_density(&self, density: f32) {
    self.imp().density.set(density);
    self.queue_draw();
  }
}

const EGL_GL_TEXTURE_2D_KHR: u32 = 0x30B1;
const EGL_GL_TEXTURE_LEVEL_KHR: i32 = 0x30BC;
const EGL_IMAGE_PRESERVED_KHR: i32 = 0x30D2;
const EGL_NONE: i32 = 0x3038;
const EGL_EXTENSIONS: i32 = 0x3055;

type CreateImage =
  unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *mut c_void, *const i32) -> *mut c_void;
type DestroyImage = unsafe extern "C" fn(*mut c_void, *mut c_void) -> u32;
type ImageTargetTexture = unsafe extern "C" fn(u32, *mut c_void);
type QueryString = unsafe extern "C" fn(*mut c_void, i32) -> *const std::ffi::c_char;

type CreateSync = unsafe extern "C" fn(*mut c_void, u32, *const i32) -> *mut c_void;
type ClientWaitSync = unsafe extern "C" fn(*mut c_void, *mut c_void, i32, u64) -> i32;
type DestroySync = unsafe extern "C" fn(*mut c_void, *mut c_void) -> u32;

const EGL_SYNC_FENCE_KHR: u32 = 0x30F9;
const EGL_SYNC_FLUSH_COMMANDS_BIT_KHR: i32 = 0x0001;
const EGL_FOREVER_KHR: u64 = u64::MAX;

/// EGLImage and fence entry points, when canvases and GDK share an EGL display.
#[derive(Clone, Copy)]
struct Images {
  display: usize,
  create: CreateImage,
  destroy: DestroyImage,
  target_texture: ImageTargetTexture,
  fence: Option<(CreateSync, ClientWaitSync, DestroySync)>,
}

impl Images {
  fn display(&self) -> *mut c_void {
    self.display as *mut c_void
  }

  /// Blocks this thread until the GPU finished what the current context submitted, so another
  /// context (GDK's, on the UI thread) sees it complete.
  unsafe fn finish(&self) {
    match self.fence {
      Some((create, wait, destroy)) => unsafe {
        let sync = create(self.display(), EGL_SYNC_FENCE_KHR, std::ptr::null());
        if sync.is_null() {
          gl_bindings::Finish();
          return;
        }
        wait(
          self.display(),
          sync,
          EGL_SYNC_FLUSH_COMMANDS_BIT_KHR,
          EGL_FOREVER_KHR,
        );
        destroy(self.display(), sync);
      },
      None => unsafe { gl_bindings::Finish() },
    }
  }
}

/// GDK's side: a GL context of its own (whose textures GSK's renderer shares) and, with a shared
/// display, the EGLImage entry points.
struct Gdk {
  context: Option<gdk::GLContext>,
  images: Option<Images>,
}

thread_local! {
  static GDK: OnceCell<Gdk> = const { OnceCell::new() };
  /// Presenters by the canvas-c context they show.
  static PRESENTERS: RefCell<HashMap<usize, Weak<Presenter>>> = RefCell::new(HashMap::new());
}

/// GTK-rs wants to have initialized GTK itself; the app already has.
fn gtk_ready() -> bool {
  if !gtk::is_initialized() && unsafe { gtk::ffi::gtk_is_initialized() } != 0 {
    unsafe { gtk::set_initialized() };
  }
  gtk::is_initialized_main_thread()
}

fn proc_address<T>(name: &str) -> Option<T> {
  let address = canvas_core::gpu::gl::get_proc_address(name);
  (!address.is_null()).then(|| unsafe { std::mem::transmute_copy::<*const c_void, T>(&address) })
}

/// `eglGetCurrentDisplay` without bringing canvas EGL up (it must start on GDK's display).
fn current_egl_display() -> *mut c_void {
  static LIB: std::sync::OnceLock<Option<libloading::Library>> = std::sync::OnceLock::new();
  let Some(lib) = LIB.get_or_init(|| unsafe { libloading::Library::new("libEGL.so.1") }.ok())
  else {
    return std::ptr::null_mut();
  };
  unsafe {
    lib
      .get::<unsafe extern "C" fn() -> *mut c_void>(b"eglGetCurrentDisplay\0")
      .map_or(std::ptr::null_mut(), |f| f())
  }
}

fn clear_gdk_current() {
  unsafe { gdk::ffi::gdk_gl_context_clear_current() };
}

fn init_gdk() -> Gdk {
  // GDK caches its current context: it must forget it whenever a canvas binds its own.
  canvas_core::gpu::gl::set_before_bind(clear_gdk_current);
  let context = gdk::Display::default().and_then(|display| {
    let context = display.create_gl_context().ok()?;
    context.realize().ok()?;
    Some(context)
  });
  let Some(context) = context else {
    log::warn!("canvas: GDK has no GL context; canvases are read back into memory textures");
    return Gdk {
      context: None,
      images: None,
    };
  };
  context.make_current();
  let display = current_egl_display();
  clear_gdk_current();
  // EGL may still settle elsewhere (e.g. Wayland, which has no pbuffers).
  let shared = !display.is_null()
    && canvas_core::gpu::gl::set_host_display(display)
    && canvas_core::gpu::gl::init_egl_display() == display;
  let images = shared.then(|| load_images(display)).flatten();
  if images.is_none() {
    log::warn!(
      "canvas: GDK's EGL display is not shareable; canvases are read back into memory textures"
    );
  }
  Gdk {
    context: Some(context),
    images,
  }
}

fn load_images(display: *mut c_void) -> Option<Images> {
  let query: QueryString = proc_address("eglQueryString")?;
  let extensions = unsafe { query(display, EGL_EXTENSIONS) };
  if extensions.is_null() {
    return None;
  }
  let extensions = unsafe { std::ffi::CStr::from_ptr(extensions) }.to_string_lossy();
  let has = |name: &str| extensions.split(' ').any(|e| e == name);
  if !has("EGL_KHR_image_base") || !has("EGL_KHR_gl_texture_2D_image") {
    return None;
  }
  let fence = has("EGL_KHR_fence_sync")
    .then(|| {
      Some((
        proc_address("eglCreateSyncKHR")?,
        proc_address("eglClientWaitSyncKHR")?,
        proc_address("eglDestroySyncKHR")?,
      ))
    })
    .flatten();
  Some(Images {
    display: display as usize,
    create: proc_address("eglCreateImageKHR")?,
    destroy: proc_address("eglDestroyImageKHR")?,
    target_texture: proc_address("glEGLImageTargetTexture2DOES")?,
    fence,
  })
}

fn with_gdk<R>(f: impl FnOnce(&Gdk) -> R) -> R {
  GDK.with(|gdk| f(gdk.get_or_init(init_gdk)))
}

/// Brings canvas EGL up on GDK's display before anything else makes a context.
pub fn init() {
  if gtk_ready() {
    with_gdk(|_| ());
  }
}

/// A texture shared with GDK, and the framebuffer that fills it.
struct Slot {
  texture: u32,
  framebuffer: u32,
  image: *mut c_void,
  gdk_texture: u32,
  width: i32,
  height: i32,
  format: u32,
  /// GDK still holds a `GdkGLTexture` of it.
  busy: Rc<Cell<bool>>,
  /// The order it was last shown in.
  shown: u64,
}

/// Shows a context's frames in its view.
struct Presenter {
  view: glib::WeakRef<CanvasView>,
  egl_context: *mut c_void,
  flipped: bool,
  slots: RefCell<Vec<Slot>>,
  retired: RefCell<Vec<Slot>>,
  frames: Cell<u64>,
}

const MAX_SLOTS: usize = 4;

/// Restores the GL bindings a present touches, so Skia's and WebGL's tracked state stays true.
struct SavedBindings {
  read: i32,
  draw: i32,
  texture: i32,
  scissor: bool,
}

impl SavedBindings {
  unsafe fn save() -> Self {
    let mut saved = SavedBindings {
      read: 0,
      draw: 0,
      texture: 0,
      scissor: false,
    };
    unsafe {
      gl_bindings::GetIntegerv(gl_bindings::READ_FRAMEBUFFER_BINDING, &mut saved.read);
      gl_bindings::GetIntegerv(gl_bindings::DRAW_FRAMEBUFFER_BINDING, &mut saved.draw);
      gl_bindings::GetIntegerv(gl_bindings::TEXTURE_BINDING_2D, &mut saved.texture);
      saved.scissor = gl_bindings::IsEnabled(gl_bindings::SCISSOR_TEST) != 0;
    }
    saved
  }

  unsafe fn restore(&self) {
    unsafe {
      gl_bindings::BindFramebuffer(gl_bindings::READ_FRAMEBUFFER, self.read as u32);
      gl_bindings::BindFramebuffer(gl_bindings::DRAW_FRAMEBUFFER, self.draw as u32);
      gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, self.texture as u32);
      if self.scissor {
        gl_bindings::Enable(gl_bindings::SCISSOR_TEST);
      }
    }
  }
}

fn current_egl_context() -> *mut c_void {
  type GetCurrentContext = unsafe extern "C" fn() -> *mut c_void;
  thread_local! {
    static GET: Option<GetCurrentContext> = proc_address("eglGetCurrentContext");
  }
  GET.with(|get| get.map_or(std::ptr::null_mut(), |f| unsafe { f() }))
}

impl Presenter {
  fn new(view: &CanvasView, egl_context: *mut c_void, flipped: bool) -> Rc<Self> {
    Rc::new(Presenter {
      view: view.downgrade(),
      egl_context,
      flipped,
      slots: RefCell::new(Vec::new()),
      retired: RefCell::new(Vec::new()),
      frames: Cell::new(0),
    })
  }

  /// Called right after the context rendered a frame, with it current.
  fn present(&self, width: i32, height: i32) {
    let Some(view) = self.view.upgrade() else {
      return;
    };
    if self.egl_context.is_null()
      || current_egl_context() != self.egl_context
      || width <= 0
      || height <= 0
    {
      return;
    }
    let frame = with_gdk(|gdk| match (gdk.context.as_ref(), gdk.images.as_ref()) {
      (Some(context), Some(images)) => self.share(context, images, width, height),
      _ => None,
    });
    let frame = frame.or_else(|| unsafe { read_back(width, height) });
    if let Some(frame) = frame {
      view.show(frame, self.flipped);
    }
  }

  /// Blits the frame into a free shared texture and wraps it for GDK.
  fn share(
    &self,
    context: &gdk::GLContext,
    images: &Images,
    width: i32,
    height: i32,
  ) -> Option<gdk::Texture> {
    let saved = unsafe { SavedBindings::save() };
    let format = unsafe { frame_format() };
    let mut slots = self.slots.borrow_mut();
    let mut retired = self.retired.borrow_mut();
    let (stale, kept): (Vec<Slot>, Vec<Slot>) = slots
      .drain(..)
      .partition(|s| (s.width, s.height, s.format) != (width, height, format));
    *slots = kept;
    retired.extend(stale);

    let mut released = Vec::new();
    retired.retain(|slot| {
      if slot.busy.get() {
        return true;
      }
      unsafe {
        gl_bindings::DeleteFramebuffers(1, &slot.framebuffer);
        gl_bindings::DeleteTextures(1, &slot.texture);
        (images.destroy)(images.display(), slot.image);
      }
      released.push(slot.gdk_texture);
      false
    });

    let index = match slots.iter().position(|s| !s.busy.get()) {
      Some(index) => index,
      None if slots.len() < MAX_SLOTS => {
        match unsafe { create_slot(images, self.egl_context, width, height, format) } {
          Some(slot) => {
            slots.push(slot);
            slots.len() - 1
          }
          None => {
            unsafe { saved.restore() };
            return None;
          }
        }
      }
      // GDK holds them all: reuse the oldest.
      None => (0..slots.len()).min_by_key(|&i| slots[i].shown)?,
    };
    unsafe {
      gl_bindings::BindFramebuffer(gl_bindings::READ_FRAMEBUFFER, 0);
      gl_bindings::BindFramebuffer(gl_bindings::DRAW_FRAMEBUFFER, slots[index].framebuffer);
      gl_bindings::Disable(gl_bindings::SCISSOR_TEST);
      gl_bindings::BlitFramebuffer(
        0,
        0,
        width,
        height,
        0,
        0,
        width,
        height,
        gl_bindings::COLOR_BUFFER_BIT,
        gl_bindings::NEAREST,
      );
      saved.restore();
      gl_bindings::Flush();
    }

    context.make_current();
    unsafe {
      for id in released.into_iter().filter(|id| *id != 0) {
        gl_bindings::DeleteTextures(1, &id);
      }
    }
    let slot = &mut slots[index];
    if slot.gdk_texture == 0 {
      slot.gdk_texture = unsafe { import_image(images, slot.image) }?;
    }
    let frames = self.frames.get() + 1;
    self.frames.set(frames);
    slot.shown = frames;
    slot.busy.set(true);
    let busy = slot.busy.clone();
    let texture = unsafe {
      gdk::GLTexture::with_release_func(context, slot.gdk_texture, width, height, move || {
        busy.set(false)
      })
    };
    Some(texture.upcast())
  }
}

/// The sized format a frame texture needs: a multisampled default framebuffer only blits into the
/// same format (RGB8 without alpha). Leaves the default framebuffer bound for reading.
unsafe fn frame_format() -> u32 {
  let mut alpha = 0;
  unsafe {
    gl_bindings::BindFramebuffer(gl_bindings::READ_FRAMEBUFFER, 0);
    gl_bindings::GetFramebufferAttachmentParameteriv(
      gl_bindings::READ_FRAMEBUFFER,
      gl_bindings::BACK,
      gl_bindings::FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE,
      &mut alpha,
    );
  }
  if alpha > 0 {
    gl_bindings::RGBA8
  } else {
    gl_bindings::RGB8
  }
}

/// A texture of the frame's format in the canvas context, its framebuffer and its EGLImage.
unsafe fn create_slot(
  images: &Images,
  egl_context: *mut c_void,
  width: i32,
  height: i32,
  format: u32,
) -> Option<Slot> {
  unsafe {
    let mut texture = 0;
    gl_bindings::GenTextures(1, &mut texture);
    gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, texture);
    gl_bindings::TexParameteri(
      gl_bindings::TEXTURE_2D,
      gl_bindings::TEXTURE_MIN_FILTER,
      gl_bindings::LINEAR as i32,
    );
    gl_bindings::TexParameteri(
      gl_bindings::TEXTURE_2D,
      gl_bindings::TEXTURE_MAG_FILTER,
      gl_bindings::LINEAR as i32,
    );
    gl_bindings::TexStorage2D(gl_bindings::TEXTURE_2D, 1, format, width, height);
    let mut framebuffer = 0;
    gl_bindings::GenFramebuffers(1, &mut framebuffer);
    gl_bindings::BindFramebuffer(gl_bindings::DRAW_FRAMEBUFFER, framebuffer);
    gl_bindings::FramebufferTexture2D(
      gl_bindings::DRAW_FRAMEBUFFER,
      gl_bindings::COLOR_ATTACHMENT0,
      gl_bindings::TEXTURE_2D,
      texture,
      0,
    );
    let complete = gl_bindings::CheckFramebufferStatus(gl_bindings::DRAW_FRAMEBUFFER)
      == gl_bindings::FRAMEBUFFER_COMPLETE;
    let attributes = [
      EGL_GL_TEXTURE_LEVEL_KHR,
      0,
      EGL_IMAGE_PRESERVED_KHR,
      1,
      EGL_NONE,
    ];
    let image = if complete {
      (images.create)(
        images.display(),
        egl_context,
        EGL_GL_TEXTURE_2D_KHR,
        texture as usize as *mut c_void,
        attributes.as_ptr(),
      )
    } else {
      std::ptr::null_mut()
    };
    if image.is_null() {
      log::error!("canvas: could not share a {width}x{height} frame texture with GDK");
      gl_bindings::DeleteFramebuffers(1, &framebuffer);
      gl_bindings::DeleteTextures(1, &texture);
      return None;
    }
    Some(Slot {
      texture,
      framebuffer,
      image,
      gdk_texture: 0,
      width,
      height,
      format,
      busy: Rc::new(Cell::new(false)),
      shown: 0,
    })
  }
}

/// The EGLImage as a texture of GDK's (current) context.
unsafe fn import_image(images: &Images, image: *mut c_void) -> Option<u32> {
  unsafe {
    while gl_bindings::GetError() != gl_bindings::NO_ERROR {}
    let mut texture = 0;
    gl_bindings::GenTextures(1, &mut texture);
    gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, texture);
    (images.target_texture)(gl_bindings::TEXTURE_2D, image);
    gl_bindings::TexParameteri(
      gl_bindings::TEXTURE_2D,
      gl_bindings::TEXTURE_MIN_FILTER,
      gl_bindings::LINEAR as i32,
    );
    gl_bindings::TexParameteri(
      gl_bindings::TEXTURE_2D,
      gl_bindings::TEXTURE_MAG_FILTER,
      gl_bindings::LINEAR as i32,
    );
    gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, 0);
    if gl_bindings::GetError() != gl_bindings::NO_ERROR {
      gl_bindings::DeleteTextures(1, &texture);
      log::error!("canvas: GDK's GL context cannot use EGLImages");
      return None;
    }
    Some(texture)
  }
}

/// The current context's default framebuffer, copied into `pixels` (RGBA8, bottom-up).
unsafe fn read_pixels(width: i32, height: i32, pixels: &mut [u8]) {
  unsafe {
    let (mut read, mut pack_buffer, mut alignment, mut row_length) = (0, 0, 0, 0);
    gl_bindings::GetIntegerv(gl_bindings::READ_FRAMEBUFFER_BINDING, &mut read);
    gl_bindings::GetIntegerv(gl_bindings::PIXEL_PACK_BUFFER_BINDING, &mut pack_buffer);
    gl_bindings::GetIntegerv(gl_bindings::PACK_ALIGNMENT, &mut alignment);
    gl_bindings::GetIntegerv(gl_bindings::PACK_ROW_LENGTH, &mut row_length);
    gl_bindings::BindFramebuffer(gl_bindings::READ_FRAMEBUFFER, 0);
    gl_bindings::BindBuffer(gl_bindings::PIXEL_PACK_BUFFER, 0);
    gl_bindings::PixelStorei(gl_bindings::PACK_ALIGNMENT, 4);
    gl_bindings::PixelStorei(gl_bindings::PACK_ROW_LENGTH, 0);
    gl_bindings::ReadPixels(
      0,
      0,
      width,
      height,
      gl_bindings::RGBA,
      gl_bindings::UNSIGNED_BYTE,
      pixels.as_mut_ptr() as *mut c_void,
    );
    gl_bindings::BindFramebuffer(gl_bindings::READ_FRAMEBUFFER, read as u32);
    gl_bindings::BindBuffer(gl_bindings::PIXEL_PACK_BUFFER, pack_buffer as u32);
    gl_bindings::PixelStorei(gl_bindings::PACK_ALIGNMENT, alignment);
    gl_bindings::PixelStorei(gl_bindings::PACK_ROW_LENGTH, row_length);
  }
}

/// The current context's default framebuffer as a memory texture.
unsafe fn read_back(width: i32, height: i32) -> Option<gdk::Texture> {
  let mut pixels = vec![0u8; width as usize * height as usize * 4];
  unsafe { read_pixels(width, height, &mut pixels) };
  let bytes = glib::Bytes::from_owned(pixels);
  Some(
    gdk::MemoryTexture::new(
      width,
      height,
      gdk::MemoryFormat::R8g8b8a8Premultiplied,
      &bytes,
      width as usize * 4,
    )
    .upcast(),
  )
}

/// Frame end for a context (`render_2d` / `present_webgl`): shows it if a view owns it.
pub(crate) fn presented(context: *mut c_void) {
  let Some(presenter) =
    PRESENTERS.with(|p| p.borrow().get(&(context as usize)).and_then(Weak::upgrade))
  else {
    return;
  };
  let Some((width, height)) = presenter.view.upgrade().map(|view| view.imp().size.get()) else {
    return;
  };
  presenter.present(width, height);
}

/// The context the view owns; a view gets at most one, as on the web.
enum Context {
  None,
  TwoD(*mut CCanvasRenderingContext2D),
  WebGL(*mut WebGLState),
  WebGPU(*const canvas_c::webgpu::gpu_canvas_context::CanvasGPUCanvasContext),
}

impl Context {
  fn pointer(&self) -> usize {
    match *self {
      Context::None => 0,
      Context::TwoD(context) => context as usize,
      Context::WebGL(state) => state as usize,
      Context::WebGPU(context) => context as usize,
    }
  }
}

#[napi(js_name = "NSCCanvas", custom_finalize)]
pub struct NSCCanvas {
  view: CanvasView,
  surface_width: u32,
  surface_height: u32,
  /// The view's scale factor: physical pixels per DIP.
  scale_x: f32,
  scale_y: f32,
  /// The view's size in DIPs, 0 until laid out.
  view_width: f32,
  view_height: f32,
  fit: CanvasFit,
  context: Context,
  presenter: Option<Rc<Presenter>>,
  /// Set once transferred to an OffscreenCanvas (`transferToOffscreenSurface`).
  offscreen: *const canvas_c::offscreen::CanvasOffscreenSurface,
  offscreen_view: u64,
}

type OffscreenListener<'a> = napi::bindgen_prelude::Function<
  'a,
  napi::bindgen_prelude::FnArgs<(u32, u32, u32)>,
  napi::threadsafe_function::UnknownReturnValue,
>;

struct OffscreenSink(Box<dyn Fn(u32, u32, u32) + Send + Sync>);

extern "C" fn offscreen_event(
  data: *mut c_void,
  event: canvas_c::offscreen::CanvasOffscreenEvent,
  a: u32,
  b: u32,
) {
  let sink = unsafe { &*(data as *const OffscreenSink) };
  (sink.0)(event as u32, a, b);
}

extern "C" fn offscreen_release(data: *mut c_void) {
  drop(unsafe { Box::from_raw(data as *mut OffscreenSink) });
}

impl ObjectFinalize for NSCCanvas {
  fn finalize(mut self, _: Env) -> Result<()> {
    self.detach_offscreen_surface();
    let key = self.context.pointer();
    PRESENTERS.with(|p| p.borrow_mut().remove(&key));
    match self.context {
      Context::None => {}
      Context::TwoD(context) => canvas_c::canvas_native_context_release(context),
      Context::WebGL(state) => canvas_c::canvas_native_webgl_state_destroy(state),
      Context::WebGPU(context) => unsafe {
        canvas_c::webgpu::linux_frames::canvas_native_webgpu_context_set_linux_sink(context, None);
        canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_release(context)
      },
    }
    if self.offscreen_view != 0 {
      drop_offscreen_view(self.offscreen_view);
    }
    Ok(())
  }
}

impl NSCCanvas {
  fn density(&self) -> f32 {
    self.scale_x.max(1.)
  }

  fn attach(&mut self, context: Context, egl_context: *mut c_void, flipped: bool) {
    let key = context.pointer();
    self.context = context;
    let presenter = Presenter::new(&self.view, egl_context, flipped);
    PRESENTERS.with(|p| p.borrow_mut().insert(key, Rc::downgrade(&presenter)));
    self.presenter = Some(presenter);
  }
}

#[napi]
impl NSCCanvas {
  /// Makes the view (a GtkWidget, see `widget`). Needs GTK initialized on this thread.
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    if !gtk_ready() {
      return Err(Error::from_reason(
        "NSCCanvas needs GTK 4 initialized on this thread",
      ));
    }
    with_gdk(|_| ());
    let view: CanvasView = glib::Object::new();
    view.set_size(300, 150);
    view.set_density(1.);
    Ok(Self {
      view,
      surface_width: 300,
      surface_height: 150,
      scale_x: 1.,
      scale_y: 1.,
      view_width: 0.,
      view_height: 0.,
      fit: CanvasFit::default(),
      context: Context::None,
      presenter: None,
      offscreen: std::ptr::null(),
      offscreen_view: 0,
    })
  }

  /// The GtkWidget's address, for `new interop.Pointer(widget)`.
  #[napi(getter)]
  pub fn widget(&self) -> f64 {
    self.view.as_ptr() as usize as f64
  }

  #[napi(getter)]
  pub fn surface_width(&self) -> u32 {
    if !self.offscreen.is_null() {
      return unsafe {
        canvas_c::offscreen::canvas_native_offscreen_surface_get_width(self.offscreen)
      };
    }
    self.surface_width
  }

  #[napi(setter)]
  pub fn set_surface_width(&mut self, width: f64) {
    self.set_surface_size(width, self.surface_height as f64);
  }

  #[napi(getter)]
  pub fn surface_height(&self) -> u32 {
    if !self.offscreen.is_null() {
      return unsafe {
        canvas_c::offscreen::canvas_native_offscreen_surface_get_height(self.offscreen)
      };
    }
    self.surface_height
  }

  #[napi(setter)]
  pub fn set_surface_height(&mut self, height: f64) {
    self.set_surface_size(self.surface_width as f64, height);
  }

  /// `CanvasFit` as an int (0 none, 1 fill, 2 fitX, 3 fitY, 4 scaleDown), as on iOS.
  #[napi(getter)]
  pub fn fit(&self) -> i32 {
    self.fit as i32
  }

  #[napi(setter)]
  pub fn set_fit(&mut self, fit: i32) {
    if let Some(fit) = CanvasFit::from_i32(fit) {
      self.fit = fit;
      self.view.set_fit(fit);
    }
  }

  #[napi(getter)]
  pub fn drawing_buffer_width(&self) -> u32 {
    self.surface_width()
  }

  #[napi(getter)]
  pub fn drawing_buffer_height(&self) -> u32 {
    self.surface_height()
  }

  /// Hands the view to an OffscreenCanvas: whatever thread renders its context shows frames
  /// here. `listener(event, a, b)` gets `CanvasOffscreenEvent`s on this thread.
  #[napi(
    js_name = "transferToOffscreenSurface",
    ts_args_type = "listener: (event: number, a: number, b: number) => void"
  )]
  pub fn transfer_to_offscreen_surface(
    &mut self,
    listener: OffscreenListener,
  ) -> Result<Option<crate::offscreen::OffscreenSurface>> {
    if !matches!(self.context, Context::None) || !self.offscreen.is_null() {
      return Ok(None);
    }
    let (id, surface) = create_offscreen_surface(
      &self.view,
      self.surface_width,
      self.surface_height,
      self.density(),
    );
    if surface.is_null() {
      drop_offscreen_view(id);
      return Ok(None);
    }
    // Weak: a view's events don't keep the env alive.
    let tsfn = listener
      .build_threadsafe_function::<(u32, u32, u32)>()
      .weak::<true>()
      .build_callback(|ctx| Ok(napi::bindgen_prelude::FnArgs::from(ctx.value)))?;
    let sink = Box::new(OffscreenSink(Box::new(move |event, a, b| {
      tsfn.call(
        (event, a, b),
        napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
      );
    })));
    unsafe {
      canvas_c::offscreen::canvas_native_offscreen_surface_set_ui_sink(
        surface,
        canvas_c::offscreen::CanvasOffscreenUiSink {
          data: Box::into_raw(sink) as *mut c_void,
          call: Some(offscreen_event),
          release: Some(offscreen_release),
        },
      );
      canvas_c::offscreen::canvas_native_offscreen_surface_reference(surface);
    }
    self.offscreen = surface;
    self.offscreen_view = id;
    Ok(crate::offscreen::OffscreenSurface::from_raw(surface))
  }

  /// `quality` is 0..1.
  #[napi(js_name = "offscreenToDataURL")]
  pub fn offscreen_to_data_url(&self, format: String, quality: f64) -> Option<String> {
    if self.offscreen.is_null() {
      return None;
    }
    let format = std::ffi::CString::new(format).ok()?;
    let url = unsafe {
      canvas_c::offscreen::canvas_native_offscreen_surface_to_data_url(
        self.offscreen,
        format.as_ptr(),
        (quality * 100.) as u32,
      )
    };
    if url.is_null() {
      return None;
    }
    let string = unsafe { std::ffi::CStr::from_ptr(url) }
      .to_string_lossy()
      .into_owned();
    canvas_c::canvas_native_string_destroy(url);
    Some(string)
  }

  #[napi]
  pub fn detach_offscreen_surface(&mut self) {
    let surface = std::mem::replace(&mut self.offscreen, std::ptr::null());
    if !surface.is_null() {
      unsafe {
        canvas_c::offscreen::canvas_native_offscreen_surface_detach_view(surface);
        canvas_c::offscreen::canvas_native_offscreen_surface_release(surface);
      }
      drop_offscreen_view(self.offscreen_view);
    }
  }

  /// The context's pointer as a decimal string (0 without one), as the iOS view's `nativeContext`.
  #[napi(getter)]
  pub fn native_context(&self) -> String {
    self.context.pointer().to_string()
  }

  /// The drawing buffer size in physical pixels; resizes (and clears) an existing context.
  #[napi]
  pub fn set_surface_size(&mut self, width: f64, height: f64) {
    // `max` also maps NaN to 1.
    let (width, height) = (width.max(1.) as u32, height.max(1.) as u32);
    // A transferred canvas's bitmap is its OffscreenCanvas's.
    if !self.offscreen.is_null() || (width, height) == (self.surface_width, self.surface_height) {
      return;
    }
    self.surface_width = width;
    self.surface_height = height;
    self.view.set_size(width, height);
    match self.context {
      Context::None => {}
      Context::TwoD(context) => {
        canvas_c::resize(unsafe { &mut *context }, width as f32, height as f32)
      }
      Context::WebGL(state) => {
        canvas_c::canvas_native_webgl_resize_pbuffer(state, width as i32, height as i32);
      }
      Context::WebGPU(context) => unsafe {
        canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_resize_offscreen(
          context, width, height,
        )
      },
    }
  }

  /// The view's laid-out size in DIPs.
  #[napi]
  pub fn set_view_size(&mut self, width: f64, height: f64) {
    self.view_width = width as f32;
    self.view_height = height as f32;
  }

  /// The view's scale factor (physical pixels per DIP).
  #[napi]
  pub fn set_composition_scale(&mut self, scale_x: f64, scale_y: f64) {
    self.scale_x = scale_x as f32;
    self.scale_y = scale_y as f32;
    self.view.set_density(self.density());
  }

  /// Where the drawing buffer sits in the view, in DIPs: `[scaleX, scaleY, offsetX, offsetY]`.
  #[napi(getter)]
  pub fn surface_transform(&self) -> Vec<f64> {
    let t = surface_transform(
      self.fit,
      (self.surface_width as f32, self.surface_height as f32),
      (self.scale_x, self.scale_y),
      (self.view_width, self.view_height),
    );
    vec![
      t.scale_x as f64,
      t.scale_y as f64,
      t.offset_x as f64,
      t.offset_y as f64,
    ]
  }

  /// Creates (once) the 2D context and returns its pointer as a decimal string, like the iOS view.
  /// Arguments mirror `NSCCanvas.create2DContext` on iOS; alpha, fontColor and colorSpace apply.
  #[napi(js_name = "create2DContext")]
  pub fn create_2d_context(
    &mut self,
    alpha: bool,
    _antialias: bool,
    _depth: bool,
    _fail_if_major_performance_caveat: bool,
    _power_preference: i32,
    _premultiplied_alpha: bool,
    _preserve_drawing_buffer: bool,
    _stencil: bool,
    _desynchronized: bool,
    _xr_compatible: bool,
    font_color: i32,
    _will_read_frequently: bool,
    color_space: Option<i32>,
  ) -> Result<String> {
    match self.context {
      Context::TwoD(context) => return Ok((context as usize).to_string()),
      Context::WebGL(_) | Context::WebGPU(_) => {
        return Err(Error::from_reason(
          "The canvas already has a WebGL or WebGPU context",
        ))
      }
      Context::None => {}
    }
    let color_space = match color_space.unwrap_or(0) {
      1 => canvas_c::CanvasColorSpace::P3,
      _ => canvas_c::CanvasColorSpace::Srgb,
    };
    let (width, height) = (self.surface_width as f32, self.surface_height as f32);
    let density = self.density();
    let mut context = canvas_c::canvas_native_context_create_gl(
      std::ptr::null_mut(),
      width,
      height,
      density,
      alpha,
      font_color,
      density * 96.,
      0,
      color_space,
    );
    if context.is_null() {
      log::error!(
        "canvas: no GLES context available; the 2D canvas draws on the CPU and is not shown"
      );
      context = canvas_c::canvas_native_context_create(
        width,
        height,
        density,
        alpha,
        font_color,
        density * 96.,
        0,
        color_space,
      );
    }
    let egl_context = canvas_c::canvas_native_context_get_egl_context(context);
    self.attach(Context::TwoD(context), egl_context, true);
    Ok((context as usize).to_string())
  }

  /// Creates (once) the WebGL (`type` "webgl"/"experimental-webgl") or WebGL 2 context;
  /// `nativeContext` then holds its pointer. Arguments mirror the iOS view's `initContext`.
  #[napi]
  pub fn init_context(
    &mut self,
    context_type: String,
    alpha: bool,
    antialias: bool,
    depth: bool,
    fail_if_major_performance_caveat: bool,
    power_preference: i32,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    stencil: bool,
    desynchronized: bool,
    xr_compatible: bool,
    _is_canvas: Option<bool>,
    _color_space: Option<i32>,
  ) -> Result<()> {
    match self.context {
      Context::WebGL(_) => return Ok(()),
      Context::TwoD(_) | Context::WebGPU(_) => {
        return Err(Error::from_reason(
          "The canvas already has a 2D or WebGPU context",
        ))
      }
      Context::None => {}
    }
    let version = if context_type.contains("webgl2") {
      2
    } else {
      1
    };
    let state = canvas_c::canvas_native_webgl_create_no_window(
      self.surface_width as i32,
      self.surface_height as i32,
      version,
      alpha,
      antialias,
      depth,
      fail_if_major_performance_caveat,
      power_preference.max(0),
      premultiplied_alpha,
      preserve_drawing_buffer,
      stencil,
      desynchronized,
      xr_compatible,
      false,
    );
    if state.is_null() {
      return Err(Error::from_reason(
        "WebGL is unavailable: no GLES context could be created on EGL",
      ));
    }
    let egl_context = canvas_c::canvas_native_webgl_get_egl_context(state);
    self.attach(Context::WebGL(state), egl_context, true);
    Ok(())
  }

  /// Creates (once) the WebGPU context on the `GPU` instance `instance` (its `__getPointer()`);
  /// `nativeContext` then holds its pointer. Frames are Vulkan images shown without copies.
  #[napi(js_name = "initWebGPUContext")]
  pub fn init_webgpu_context(&mut self, instance: napi::bindgen_prelude::BigInt) -> Result<()> {
    match self.context {
      Context::WebGPU(_) => return Ok(()),
      Context::TwoD(_) | Context::WebGL(_) => {
        return Err(Error::from_reason(
          "The canvas already has a 2D or WebGL context",
        ))
      }
      Context::None => {}
    }
    let (instance, _) = instance.get_i64();
    if instance == 0 {
      return Err(Error::from_reason("Invalid GPU instance"));
    }
    let context = unsafe {
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_create_offscreen(
        instance as *const _,
        self.surface_width,
        self.surface_height,
      )
    };
    if context.is_null() {
      return Err(Error::from_reason("Could not create a WebGPU context"));
    }
    let id = NEXT_VIEW.fetch_add(1, Ordering::Relaxed);
    OFFSCREEN_VIEWS.with(|views| views.borrow_mut().insert(id, self.view.downgrade()));
    self.offscreen_view = id;
    let sink: canvas_c::webgpu::linux_frames::LinuxFrameSink =
      Arc::new(move |frame| glib::MainContext::default().invoke(move || show_gpu_frame(id, frame)));
    canvas_c::webgpu::linux_frames::canvas_native_webgpu_context_set_linux_sink(
      context,
      Some(sink),
    );
    self.context = Context::WebGPU(context);
    Ok(())
  }

  /// GL contexts are not lost on Linux (Mesa has no device loss to report here).
  #[napi]
  pub fn is_context_lost(&self) -> bool {
    match self.context {
      Context::WebGL(state) => canvas_c::canvas_native_webgl_get_is_context_lost(state),
      _ => false,
    }
  }

  #[napi]
  pub fn restore_context(&self) -> bool {
    true
  }

  /// Renders pending drawing and shows it now.
  #[napi]
  pub fn present(&self) {
    match self.context {
      Context::None => {}
      Context::TwoD(context) => canvas_c::canvas_native_context_render(context),
      // WebGPU presents through its context (presentSurface).
      Context::WebGPU(_) => {}
      Context::WebGL(state) => {
        canvas_c::canvas_native_webgl_present(state);
      }
    }
    if let Some(presenter) = self.presenter.as_ref() {
      presenter.present(self.surface_width as i32, self.surface_height as i32);
    }
  }
}

/// A GdkTexture's pixels as premultiplied RGBA8.
pub(crate) fn texture_pixels(texture: usize) -> Option<(u32, u32, Vec<u8>)> {
  if texture == 0 || !gtk_ready() {
    return None;
  }
  let texture = texture as *mut gdk::ffi::GdkTexture;
  let (width, height) = unsafe {
    (
      gdk::ffi::gdk_texture_get_width(texture),
      gdk::ffi::gdk_texture_get_height(texture),
    )
  };
  if width <= 0 || height <= 0 {
    return None;
  }
  let stride = width as usize * 4;
  let mut pixels = vec![0u8; stride * height as usize];
  // GDK_MEMORY_DEFAULT: premultiplied BGRA on little-endian machines.
  unsafe { gdk::ffi::gdk_texture_download(texture, pixels.as_mut_ptr(), stride) };
  if cfg!(target_endian = "little") {
    for pixel in pixels.chunks_exact_mut(4) {
      pixel.swap(0, 2);
    }
  } else {
    for pixel in pixels.chunks_exact_mut(4) {
      pixel.rotate_left(1);
    }
  }
  Some((width as u32, height as u32, pixels))
}

/// The caller owns the returned reference.
pub(crate) fn texture_from_pixels(width: u32, height: u32, pixels: Vec<u8>) -> usize {
  if width == 0 || height == 0 || !gtk_ready() {
    return 0;
  }
  let texture = gdk::MemoryTexture::new(
    width as i32,
    height as i32,
    gdk::MemoryFormat::R8g8b8a8Premultiplied,
    &glib::Bytes::from_owned(pixels),
    width as usize * 4,
  );
  use glib::translate::IntoGlibPtr;
  let texture: *mut gdk::ffi::GdkTexture = unsafe { texture.upcast::<gdk::Texture>().into_glib_ptr() };
  texture as usize
}

/// A frame texture an OffscreenCanvas's rendering thread fills and GDK shows.
struct SharedSlot {
  texture: u32,
  framebuffer: u32,
  image: usize,
  width: i32,
  height: i32,
  format: u32,
  /// GDK still holds a `GdkGLTexture` of it.
  busy: Arc<AtomicBool>,
  shown: u64,
}

/// Shows a transferred canvas's frames: runs on the thread that renders its context and hands
/// each frame to the UI thread.
struct OffscreenPresenter {
  view: u64,
  images: Option<Images>,
  slots: Mutex<Vec<SharedSlot>>,
  frames: AtomicU64,
}

static NEXT_VIEW: AtomicU64 = AtomicU64::new(1);

thread_local! {
  /// Views of transferred canvases, by id (UI thread).
  static OFFSCREEN_VIEWS: RefCell<HashMap<u64, glib::WeakRef<CanvasView>>> = RefCell::new(HashMap::new());
  /// GDK textures of offscreen frame images (UI thread).
  static IMPORTED: RefCell<HashMap<usize, u32>> = RefCell::new(HashMap::new());
}

impl OffscreenPresenter {
  fn present(&self, width: i32, height: i32) {
    if width <= 0 || height <= 0 {
      return;
    }
    let view = self.view;
    let Some(images) = self.images else {
      // No shared display: copy the frame out.
      let mut pixels = vec![0u8; width as usize * height as usize * 4];
      unsafe { read_pixels(width, height, &mut pixels) };
      glib::MainContext::default().invoke(move || {
        let bytes = glib::Bytes::from_owned(pixels);
        let frame = gdk::MemoryTexture::new(
          width,
          height,
          gdk::MemoryFormat::R8g8b8a8Premultiplied,
          &bytes,
          width as usize * 4,
        );
        if let Some(view) = offscreen_view(view) {
          view.show(frame.upcast(), true);
        }
      });
      return;
    };
    let egl_context = current_egl_context();
    let mut slots = self.slots.lock().unwrap_or_else(|e| e.into_inner());
    let saved = unsafe { SavedBindings::save() };
    let format = unsafe { frame_format() };
    let mut forgotten = Vec::new();
    slots.retain(|slot| {
      let keep = (slot.width, slot.height, slot.format) == (width, height, format)
        || slot.busy.load(Ordering::Acquire);
      if !keep {
        unsafe {
          gl_bindings::DeleteFramebuffers(1, &slot.framebuffer);
          gl_bindings::DeleteTextures(1, &slot.texture);
          (images.destroy)(images.display(), slot.image as *mut c_void);
        }
        forgotten.push(slot.image);
      }
      keep
    });
    let fits =
      |slot: &SharedSlot| (slot.width, slot.height, slot.format) == (width, height, format);
    let index = match slots
      .iter()
      .position(|s| fits(s) && !s.busy.load(Ordering::Acquire))
    {
      Some(index) => Some(index),
      None if slots.iter().filter(|s| fits(s)).count() < MAX_SLOTS => {
        unsafe { create_slot(&images, egl_context, width, height, format) }.map(|slot| {
          slots.push(SharedSlot {
            texture: slot.texture,
            framebuffer: slot.framebuffer,
            image: slot.image as usize,
            width,
            height,
            format,
            busy: Arc::new(AtomicBool::new(false)),
            shown: 0,
          });
          slots.len() - 1
        })
      }
      // GDK holds them all: skip the frame, the next one comes soon.
      None => None,
    };
    let Some(index) = index else {
      unsafe { saved.restore() };
      return;
    };
    let slot = &mut slots[index];
    unsafe {
      gl_bindings::BindFramebuffer(gl_bindings::READ_FRAMEBUFFER, 0);
      gl_bindings::BindFramebuffer(gl_bindings::DRAW_FRAMEBUFFER, slot.framebuffer);
      gl_bindings::Disable(gl_bindings::SCISSOR_TEST);
      gl_bindings::BlitFramebuffer(
        0,
        0,
        width,
        height,
        0,
        0,
        width,
        height,
        gl_bindings::COLOR_BUFFER_BIT,
        gl_bindings::NEAREST,
      );
      saved.restore();
      images.finish();
    }
    let frames = self.frames.fetch_add(1, Ordering::Relaxed) + 1;
    slot.shown = frames;
    slot.busy.store(true, Ordering::Release);
    let (image, busy) = (slot.image, slot.busy.clone());
    drop(slots);
    glib::MainContext::default().invoke(move || {
      for image in forgotten {
        forget_image(image);
      }
      show_offscreen(view, images, image, width, height, busy);
    });
  }
}

fn offscreen_view(id: u64) -> Option<CanvasView> {
  OFFSCREEN_VIEWS.with(|views| views.borrow().get(&id).and_then(|view| view.upgrade()))
}

/// UI thread: `image` (complete) as a GdkGLTexture in its view.
fn show_offscreen(
  view: u64,
  images: Images,
  image: usize,
  width: i32,
  height: i32,
  busy: Arc<AtomicBool>,
) {
  let shown = with_gdk(|gdk| {
    let context = gdk.context.as_ref()?;
    let view = offscreen_view(view)?;
    context.make_current();
    let texture = IMPORTED.with(|imported| {
      let mut imported = imported.borrow_mut();
      if let Some(texture) = imported.get(&image) {
        return Some(*texture);
      }
      let texture = unsafe { import_image(&images, image as *mut c_void) }?;
      imported.insert(image, texture);
      Some(texture)
    })?;
    let released = busy.clone();
    let frame = unsafe {
      gdk::GLTexture::with_release_func(context, texture, width, height, move || {
        released.store(false, Ordering::Release)
      })
    };
    view.show(frame.upcast(), true);
    Some(())
  });
  if shown.is_none() {
    busy.store(false, Ordering::Release);
  }
}

fn forget_image(image: usize) {
  if let Some(texture) = IMPORTED.with(|imported| imported.borrow_mut().remove(&image)) {
    with_gdk(|gdk| {
      if let Some(context) = gdk.context.as_ref() {
        context.make_current();
        unsafe { gl_bindings::DeleteTextures(1, &texture) };
      }
    });
  }
}

/// UI thread: a surface whose contexts' frames show in `view`.
fn create_offscreen_surface(
  view: &CanvasView,
  width: u32,
  height: u32,
  density: f32,
) -> (u64, *const canvas_c::offscreen::CanvasOffscreenSurface) {
  let id = NEXT_VIEW.fetch_add(1, Ordering::Relaxed);
  OFFSCREEN_VIEWS.with(|views| views.borrow_mut().insert(id, view.downgrade()));
  let presenter = Arc::new(OffscreenPresenter {
    view: id,
    images: with_gdk(|gdk| gdk.images),
    slots: Mutex::new(Vec::new()),
    frames: AtomicU64::new(0),
  });
  let hook: canvas_core::gpu::gl::PresentHook =
    Arc::new(move |width, height| presenter.present(width, height));
  let surface = canvas_c::offscreen::canvas_native_offscreen_surface_create_linux(
    width,
    height,
    density,
    density * 96.,
    0,
    canvas_c::CanvasColorSpace::Srgb,
    hook,
  );
  (id, surface)
}

fn drop_offscreen_view(id: u64) {
  OFFSCREEN_VIEWS.with(|views| views.borrow_mut().remove(&id));
}

const GL_TEXTURE_TILING_EXT: u32 = 0x9580;
const GL_DEDICATED_MEMORY_OBJECT_EXT: u32 = 0x9581;
const GL_OPTIMAL_TILING_EXT: i32 = 0x9584;
const GL_HANDLE_TYPE_OPAQUE_FD_EXT: u32 = 0x9586;
const GL_TEXTURE_SWIZZLE_R: u32 = 0x8E42;
const GL_TEXTURE_SWIZZLE_B: u32 = 0x8E44;

/// `GL_EXT_memory_object(_fd)` entry points, for Vulkan images.
#[derive(Clone, Copy)]
struct MemoryObjects {
  create: unsafe extern "C" fn(i32, *mut u32),
  parameter: unsafe extern "C" fn(u32, u32, *const i32),
  import: unsafe extern "C" fn(u32, u64, u32, i32),
  storage: unsafe extern "C" fn(u32, i32, u32, i32, i32, u32, u64),
  delete: unsafe extern "C" fn(i32, *const u32),
}

thread_local! {
  static MEMORY_OBJECTS: OnceCell<Option<MemoryObjects>> = const { OnceCell::new() };
  /// GDK textures (and their memory objects) of WebGPU frame images (UI thread).
  static GPU_IMPORTED: RefCell<HashMap<u64, (u32, u32)>> = RefCell::new(HashMap::new());
}

fn memory_objects() -> Option<MemoryObjects> {
  MEMORY_OBJECTS.with(|cell| {
    *cell.get_or_init(|| {
      let objects = Some(MemoryObjects {
        create: proc_address("glCreateMemoryObjectsEXT")?,
        parameter: proc_address("glMemoryObjectParameterivEXT")?,
        import: proc_address("glImportMemoryFdEXT")?,
        storage: proc_address("glTexStorageMem2DEXT")?,
        delete: proc_address("glDeleteMemoryObjectsEXT")?,
      });
      if objects.is_none() {
        log::error!(
          "canvas: GDK's GL lacks GL_EXT_memory_object_fd; WebGPU canvases are not shown"
        );
      }
      objects
    })
  })
}

/// UI thread, GDK's context current: the image's memory as a GL texture (taking `fd`).
unsafe fn import_gpu_image(
  objects: &MemoryObjects,
  frame: &canvas_c::webgpu::linux_frames::LinuxGpuFrame,
) -> Option<(u32, u32)> {
  unsafe {
    while gl_bindings::GetError() != gl_bindings::NO_ERROR {}
    let mut memory = 0;
    (objects.create)(1, &mut memory);
    let dedicated = 1;
    (objects.parameter)(memory, GL_DEDICATED_MEMORY_OBJECT_EXT, &dedicated);
    (objects.import)(
      memory,
      frame.allocation_size,
      GL_HANDLE_TYPE_OPAQUE_FD_EXT,
      frame.fd,
    );
    let mut texture = 0;
    gl_bindings::GenTextures(1, &mut texture);
    gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, texture);
    gl_bindings::TexParameteri(
      gl_bindings::TEXTURE_2D,
      GL_TEXTURE_TILING_EXT,
      GL_OPTIMAL_TILING_EXT,
    );
    (objects.storage)(
      gl_bindings::TEXTURE_2D,
      1,
      gl_bindings::RGBA8,
      frame.width as i32,
      frame.height as i32,
      memory,
      0,
    );
    gl_bindings::TexParameteri(
      gl_bindings::TEXTURE_2D,
      gl_bindings::TEXTURE_MIN_FILTER,
      gl_bindings::LINEAR as i32,
    );
    gl_bindings::TexParameteri(
      gl_bindings::TEXTURE_2D,
      gl_bindings::TEXTURE_MAG_FILTER,
      gl_bindings::LINEAR as i32,
    );
    if frame.bgra {
      gl_bindings::TexParameteri(
        gl_bindings::TEXTURE_2D,
        GL_TEXTURE_SWIZZLE_R,
        gl_bindings::BLUE as i32,
      );
      gl_bindings::TexParameteri(
        gl_bindings::TEXTURE_2D,
        GL_TEXTURE_SWIZZLE_B,
        gl_bindings::RED as i32,
      );
    }
    gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, 0);
    if gl_bindings::GetError() != gl_bindings::NO_ERROR {
      gl_bindings::DeleteTextures(1, &texture);
      (objects.delete)(1, &memory);
      log::error!("canvas: GDK's GL could not import a WebGPU frame image");
      return None;
    }
    Some((texture, memory))
  }
}

/// UI thread: a finished WebGPU frame in its view.
fn show_gpu_frame(view: u64, frame: canvas_c::webgpu::linux_frames::LinuxGpuFrame) {
  let busy = frame.busy.clone();
  let shown = with_gdk(|gdk| {
    let context = gdk.context.as_ref()?;
    let objects = memory_objects()?;
    context.make_current();
    GPU_IMPORTED.with(|imported| {
      let mut imported = imported.borrow_mut();
      for image in &frame.forget {
        if let Some((texture, memory)) = imported.remove(image) {
          unsafe {
            gl_bindings::DeleteTextures(1, &texture);
            (objects.delete)(1, &memory);
          }
        }
      }
      if frame.fd >= 0 {
        let entry = unsafe { import_gpu_image(&objects, &frame) }?;
        imported.insert(frame.image, entry);
      }
      let view = offscreen_view(view)?;
      let (texture, _) = *imported.get(&frame.image)?;
      let released = busy.clone();
      let texture = unsafe {
        gdk::GLTexture::with_release_func(
          context,
          texture,
          frame.width as i32,
          frame.height as i32,
          move || released.store(false, Ordering::Release),
        )
      };
      view.show(texture.upcast(), false);
      Some(())
    })
  });
  if shown.is_none() {
    busy.store(false, Ordering::Release);
  }
}
