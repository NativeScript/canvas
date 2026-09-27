#![cfg(target_os = "windows")]
#![deny(clippy::all)]

mod tap;

use std::cell::{Cell, RefCell};
use std::mem::ManuallyDrop;
use std::rc::{Rc, Weak};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use napi::bindgen_prelude::{FnArgs, Function, ObjectFinalize, Uint8Array};
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::{Env, Error, Result};
use napi_derive::napi;
use windows::core::{IInspectable, IUnknown, Interface, RuntimeType, HRESULT};
use windows::Foundation::TypedEventHandler;
use windows::Graphics::DirectX::Direct3D11::IDirect3DSurface;
use windows::Media::Playback::{MediaPlaybackSession, MediaPlayer, MediaPlayerFailedEventArgs};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, GENERIC_ALL, HANDLE, HMODULE, POINT, RECT};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP};
use windows::Win32::Graphics::Direct3D11::{
  D3D11CreateDevice, ID3D11Device, ID3D11Device5, ID3D11DeviceContext, ID3D11DeviceContext4, ID3D11Fence,
  ID3D11Multithread, ID3D11Resource, ID3D11Texture2D, D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE,
  D3D11_CPU_ACCESS_READ, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_VIDEO_SUPPORT, D3D11_FENCE_FLAG_SHARED,
  D3D11_MAPPED_SUBRESOURCE, D3D11_MAP_READ, D3D11_RESOURCE_MISC_SHARED, D3D11_RESOURCE_MISC_SHARED_NTHANDLE,
  D3D11_SDK_VERSION, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT, D3D11_USAGE_STAGING,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};
use windows::Win32::Graphics::Dxgi::{
  IDXGIDevice, IDXGIResource1, IDXGISurface, DXGI_SHARED_RESOURCE_READ, DXGI_SHARED_RESOURCE_WRITE,
};
use windows::Win32::System::WinRT::Direct3D11::CreateDirect3D11SurfaceFromDXGISurface;

/// WinUI 3's `ISurfaceImageSourceNative` (microsoft.ui.xaml.media.dxinterop.h; not the UWP IID).
#[windows::core::interface("e4cecd6c-f14b-4f46-83c3-8bbda27c6504")]
unsafe trait ISurfaceImageSourceNative: windows::core::IUnknown {
  fn SetDevice(&self, device: *mut c_void) -> HRESULT;
  fn BeginDraw(&self, update_rect: RECT, surface: *mut *mut c_void, offset: *mut POINT) -> HRESULT;
  fn EndDraw(&self) -> HRESULT;
}

/// Parses `NSWinRT.interop.pointerKey(...)` output (`"0x…"`) or a decimal address.
fn parse_pointer_key(key: &str) -> Option<*mut c_void> {
  let key = key.trim();
  let address = match key.strip_prefix("0x").or_else(|| key.strip_prefix("0X")) {
    Some(hex) => usize::from_str_radix(hex, 16).ok(),
    None => key.parse().ok(),
  }?;
  (address != 0).then_some(address as *mut c_void)
}

fn to_napi(error: windows::core::Error) -> Error {
  Error::from_reason(error.message())
}

struct Device {
  device: ID3D11Device,
  context: ID3D11DeviceContext,
  /// `LowPart | HighPart << 32`: WebGPU opens shared frames only on the same adapter.
  luid: u64,
  /// Fences (Windows 10 1703+): without them frames are not shared with other devices.
  fences: Option<(ID3D11Device5, ID3D11DeviceContext4)>,
}

// Multithread-protected (below), and D3D11 devices are free-threaded.
unsafe impl Send for Device {}
unsafe impl Sync for Device {}

/// One device for every player. MediaPlayer copies frames into it on its own threads while the JS
/// thread reads them back or presents them, so its immediate context is multithread-protected.
fn device() -> Option<&'static Device> {
  static DEVICE: OnceLock<Option<Device>> = OnceLock::new();
  DEVICE
    .get_or_init(|| create_device(D3D_DRIVER_TYPE_HARDWARE).or_else(|| create_device(D3D_DRIVER_TYPE_WARP)))
    .as_ref()
}

fn create_device(driver: D3D_DRIVER_TYPE) -> Option<Device> {
  let (mut device, mut context) = (None, None);
  unsafe {
    D3D11CreateDevice(
      None,
      driver,
      HMODULE::default(),
      // BGRA for XAML SurfaceImageSources; video support for MediaPlayer's video processor.
      D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_VIDEO_SUPPORT,
      None,
      D3D11_SDK_VERSION,
      Some(&mut device),
      None,
      Some(&mut context),
    )
  }
  .ok()?;
  let (device, context) = (device?, context?);
  let multithread: ID3D11Multithread = context.cast().ok()?;
  let _ = unsafe { multithread.SetMultithreadProtected(true) };
  let luid = (|| -> windows::core::Result<u64> {
    let adapter = unsafe { device.cast::<IDXGIDevice>()?.GetAdapter()? };
    let luid = unsafe { adapter.GetDesc()? }.AdapterLuid;
    Ok(luid.LowPart as u64 | ((luid.HighPart as u32 as u64) << 32))
  })()
  .unwrap_or(0);
  let fences = device.cast::<ID3D11Device5>().ok().zip(context.cast::<ID3D11DeviceContext4>().ok());
  Some(Device { device, context, luid, fences })
}

fn texture_desc(width: u32, height: u32, shared: bool) -> D3D11_TEXTURE2D_DESC {
  D3D11_TEXTURE2D_DESC {
    Width: width,
    Height: height,
    MipLevels: 1,
    ArraySize: 1,
    Format: DXGI_FORMAT_B8G8R8A8_UNORM,
    SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
    Usage: D3D11_USAGE_DEFAULT,
    BindFlags: (D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
    CPUAccessFlags: 0,
    // Opened by WebGPU on its D3D12 device (canvas-c's gpu_shared_frame).
    MiscFlags: if shared { (D3D11_RESOURCE_MISC_SHARED_NTHANDLE.0 | D3D11_RESOURCE_MISC_SHARED.0) as u32 } else { 0 },
  }
}

static NEXT_SHARED_ID: AtomicU64 = AtomicU64::new(1);

/// An NT handle to share a texture or fence, closed with it.
struct SharedHandle {
  handle: HANDLE,
  /// Unique for the process's lifetime, unlike handle values: consumers cache by it.
  id: u64,
}

impl SharedHandle {
  fn new(handle: HANDLE) -> Self {
    Self { handle, id: NEXT_SHARED_ID.fetch_add(1, Ordering::Relaxed) }
  }
}

impl Drop for SharedHandle {
  fn drop(&mut self) {
    let _ = unsafe { CloseHandle(self.handle) };
  }
}

/// The fences frames are shared with: `ready` is signalled here after each copy, `release` by the
/// consumer once it has read a frame.
struct Fences {
  ready: ID3D11Fence,
  ready_handle: SharedHandle,
  release: ID3D11Fence,
  release_handle: SharedHandle,
}

impl Fences {
  fn new(device: &ID3D11Device5) -> Option<Fences> {
    let create = || -> windows::core::Result<(ID3D11Fence, SharedHandle)> {
      let mut fence: Option<ID3D11Fence> = None;
      unsafe { device.CreateFence(0, D3D11_FENCE_FLAG_SHARED, &mut fence)? };
      let fence = fence.ok_or_else(windows::core::Error::empty)?;
      let handle = unsafe { fence.CreateSharedHandle(None, GENERIC_ALL.0, PCWSTR::null())? };
      Ok((fence, SharedHandle::new(handle)))
    };
    let (ready, ready_handle) = create().ok()?;
    let (release, release_handle) = create().ok()?;
    Some(Fences { ready, ready_handle, release, release_handle })
  }
}

/// `CanvasD3DSharedFrame` in canvas-c (crates/canvas-c/src/webgpu/gpu_shared_frame.rs): same layout.
#[repr(C)]
struct SharedFrameDesc {
  size: u32,
  consumed: u32,
  adapter_luid: u64,
  texture_id: u64,
  texture: *mut c_void,
  ready_fence_id: u64,
  ready_fence: *mut c_void,
  ready_value: u64,
  release_fence_id: u64,
  release_fence: *mut c_void,
  release_value: u64,
}

/// A decoded frame, BGRA at the video's natural size.
struct Frame {
  texture: ID3D11Texture2D,
  /// `texture` as the WinRT surface `CopyFrameToVideoSurface` takes.
  surface: IDirect3DSurface,
  /// Set when the device has fences: the texture is shareable.
  shared: Option<SharedHandle>,
  /// The release-fence value consumers must reach before the texture is written again.
  busy_until: u64,
}

impl Frame {
  fn new(device: &Device, width: u32, height: u32) -> Option<Frame> {
    let shared = device.fences.is_some();
    let mut texture = None;
    unsafe { device.device.CreateTexture2D(&texture_desc(width, height, shared), None, Some(&mut texture)) }.ok()?;
    let texture = texture?;
    let dxgi: IDXGISurface = texture.cast().ok()?;
    let surface: IDirect3DSurface = unsafe { CreateDirect3D11SurfaceFromDXGISurface(&dxgi) }.ok()?.cast().ok()?;
    let shared = shared
      .then(|| {
        let resource: IDXGIResource1 = texture.cast().ok()?;
        let access = DXGI_SHARED_RESOURCE_READ.0 | DXGI_SHARED_RESOURCE_WRITE.0;
        unsafe { resource.CreateSharedHandle(None, access, PCWSTR::null()) }.ok().map(SharedHandle::new)
      })
      .flatten();
    Some(Frame { texture, surface, shared, busy_until: 0 })
  }
}

/// A texture a consumer may still be reading is not written; with every other one busy, a decoded
/// frame is dropped rather than waited for.
const POOL_SIZE: usize = 3;

#[derive(Default)]
struct Pool {
  frames: Vec<Frame>,
  latest: Option<usize>,
  width: u32,
  height: u32,
  fences: Option<Fences>,
  /// The ready-fence value signalled after the latest copy.
  latest_ready: u64,
  next_ready: u64,
  next_release: u64,
}

unsafe impl Send for Pool {}

impl Pool {
  fn latest(&self) -> Option<&Frame> {
    self.frames.get(self.latest?)
  }
}

#[derive(Default)]
struct Frames {
  pool: Mutex<Pool>,
  /// Bumped per copied frame.
  generation: AtomicU64,
  /// Set under `pool`'s lock: a `VideoFrameAvailable` still running when its handler is removed
  /// copies nothing after the bridge closes.
  closed: AtomicBool,
}

impl Frames {
  /// On the player's thread, from `VideoFrameAvailable`.
  fn copy_from(&self, player: &MediaPlayer) -> bool {
    let Some(device) = device() else { return false };
    let Ok(session) = player.PlaybackSession() else { return false };
    let (width, height) = (session.NaturalVideoWidth().unwrap_or(0), session.NaturalVideoHeight().unwrap_or(0));
    if width == 0 || height == 0 {
      return false;
    }
    let mut pool = self.pool.lock().unwrap();
    if self.closed.load(Ordering::Acquire) {
      return false;
    }
    if pool.width != width || pool.height != height {
      pool.frames.clear();
      pool.latest = None;
      pool.width = width;
      pool.height = height;
    }
    if pool.fences.is_none() {
      pool.fences = device.fences.as_ref().and_then(|(device5, _)| Fences::new(device5));
    }
    let released = pool.fences.as_ref().map_or(u64::MAX, |fences| unsafe { fences.release.GetCompletedValue() });
    let latest = pool.latest;
    let free = |i: usize| Some(i) != latest && pool.frames.get(i).is_none_or(|frame| frame.busy_until <= released);
    let Some(index) = (0..POOL_SIZE).find(|&i| free(i)) else {
      return false;
    };
    if index == pool.frames.len() {
      let Some(frame) = Frame::new(device, width, height) else { return false };
      pool.frames.push(frame);
    }
    if player.CopyFrameToVideoSurface(&pool.frames[index].surface).is_err() {
      return false;
    }
    if let (Some(fences), Some((_, context4))) = (pool.fences.as_ref(), device.fences.as_ref()) {
      let value = pool.next_ready + 1;
      if unsafe { context4.Signal(&fences.ready, value) }.is_ok() {
        pool.next_ready = value;
      }
    }
    pool.latest = Some(index);
    pool.latest_ready = pool.next_ready;
    self.generation.fetch_add(1, Ordering::AcqRel);
    true
  }

  fn close(&self) {
    let _pool = self.pool.lock().unwrap();
    self.closed.store(true, Ordering::Release);
  }

  fn size(&self) -> (u32, u32) {
    let pool = self.pool.lock().unwrap();
    if pool.latest.is_some() {
      (pool.width, pool.height)
    } else {
      (0, 0)
    }
  }
}

/// A XAML `SurfaceImageSource` frames are copied into, made by the view at the video's size.
struct XamlSurface {
  native: ISurfaceImageSourceNative,
  width: u32,
  height: u32,
}

impl XamlSurface {
  /// UI thread.
  fn present(&self, device: &Device, texture: &ID3D11Texture2D) -> windows::core::Result<()> {
    let rect = RECT { left: 0, top: 0, right: self.width as i32, bottom: self.height as i32 };
    let mut surface = std::ptr::null_mut();
    let mut offset = POINT::default();
    unsafe { self.native.BeginDraw(rect, &mut surface, &mut offset) }.ok()?;
    // The update rectangle lives in XAML's atlas at `offset`.
    let copied = unsafe { IDXGISurface::from_raw(surface) }.cast::<ID3D11Resource>().map(|target| unsafe {
      device.context.CopySubresourceRegion(&target, 0, offset.x.max(0) as u32, offset.y.max(0) as u32, 0, texture, 0, None)
    });
    let ended = unsafe { self.native.EndDraw() }.ok();
    copied.and(ended)
  }
}

/// The player's audio routed to a Web Audio graph (`NSCMediaPlayerBridge.createAudioTap()`).
/// `address` is a `tap::AudioTapSource` audiocontext.node reads through.
#[napi(js_name = "NSCAudioTap")]
pub struct NSCAudioTap {
  tap: Arc<tap::Tap>,
  source: Box<tap::AudioTapSource>,
}

#[napi]
impl NSCAudioTap {
  #[napi(getter)]
  pub fn address(&self) -> f64 {
    &*self.source as *const tap::AudioTapSource as usize as f64
  }

  /// The element's volume (0 when muted): the graph gets what the element would have played.
  #[napi]
  pub fn set_gain(&self, gain: f64) {
    self.tap.set_gain(gain as f32);
  }

  /// Routed: the player is silent and its audio goes to the graph. Not routed (initially): it plays
  /// as before.
  #[napi]
  pub fn set_routed(&self, routed: bool) {
    self.tap.set_routed(routed);
  }

  /// Frames handed to the graph so far.
  #[napi(getter)]
  pub fn frames_tapped(&self) -> f64 {
    self.tap.frames.load(std::sync::atomic::Ordering::Relaxed) as f64
  }
}

/// A frame handed to WebGPU (`NSCMediaPlayerBridge.gpuFrame()`); holds the handles it names.
#[napi(js_name = "NSCSharedFrame", custom_finalize)]
pub struct NSCSharedFrame {
  desc: Box<SharedFrameDesc>,
  frames: Arc<Frames>,
  width: u32,
  height: u32,
  released: bool,
}

impl ObjectFinalize for NSCSharedFrame {
  fn finalize(mut self, _: Env) -> Result<()> {
    self.close();
    Ok(())
  }
}

#[napi]
impl NSCSharedFrame {
  /// The descriptor's address, for `nativeTexture`.
  #[napi(getter)]
  pub fn address(&self) -> f64 {
    &*self.desc as *const SharedFrameDesc as usize as f64
  }

  #[napi(getter)]
  pub fn width(&self) -> u32 {
    self.width
  }

  #[napi(getter)]
  pub fn height(&self) -> u32 {
    self.height
  }

  /// Done with the frame. One never consumed (its import failed) frees its texture at once; a
  /// consumed one is freed when the consumer's release signal completes.
  #[napi]
  pub fn close(&mut self) {
    if std::mem::replace(&mut self.released, true) {
      return;
    }
    // Written by the consumer (another module) through the address.
    let consumed = unsafe { std::ptr::read_volatile(&self.desc.consumed) };
    if consumed == 0 {
      let mut pool = self.frames.pool.lock().unwrap();
      let (texture_id, release_value) = (self.desc.texture_id, self.desc.release_value);
      if let Some(frame) = pool.frames.iter_mut().find(|f| f.shared.as_ref().is_some_and(|s| s.id == texture_id)) {
        if frame.busy_until == release_value {
          frame.busy_until = 0;
        }
      }
    }
  }
}

/// What a bridge holds on the player. A MediaPlayer released while the process exits (after
/// ExitProcess has killed Media Foundation's threads, e.g. from a thread-local destructor) spins
/// forever in the graphics driver, so an env cleanup hook closes every live player while its threads
/// still run, and nothing here is released from process shutdown.
struct Live {
  player: MediaPlayer,
  unsubscribe: Vec<Box<dyn FnOnce()>>,
  frames: Option<Arc<Frames>>,
}

impl Live {
  fn close(&mut self) {
    for unsubscribe in self.unsubscribe.drain(..) {
      unsubscribe();
    }
    if let Some(frames) = self.frames.as_ref() {
      frames.close();
    }
  }
}

type LiveCell = RefCell<Option<Live>>;

thread_local! {
  // Weak and ManuallyDrop: dropping these at thread exit releases nothing.
  static LIVE: RefCell<Vec<Weak<LiveCell>>> = const { RefCell::new(Vec::new()) };
  static TEST_PLAYERS: RefCell<Vec<ManuallyDrop<MediaPlayer>>> = const { RefCell::new(Vec::new()) };
  static CLEANUP_HOOKED: Cell<bool> = const { Cell::new(false) };
}

/// Closes every live player (bridges and test players) before the env goes away.
fn close_everything() {
  for live in LIVE.with(|live| std::mem::take(&mut *live.borrow_mut())) {
    if let Some(live) = live.upgrade() {
      if let Some(mut live) = live.borrow_mut().take() {
        live.close();
        let _ = live.player.Close();
      }
    }
  }
  close_test_players();
}

fn ensure_cleanup_hook(env: &Env) -> Result<()> {
  if !CLEANUP_HOOKED.get() {
    env.add_env_cleanup_hook((), |_| close_everything())?;
    CLEANUP_HOOKED.set(true);
  }
  Ok(())
}

type Emit = Arc<dyn Fn(&'static str, Option<String>) + Send + Sync>;
type EventCallback<'a> = Function<'a, FnArgs<(String, Option<String>)>, UnknownReturnValue>;

fn notify<S: RuntimeType + 'static, A: RuntimeType + 'static>(emit: &Emit, kind: &'static str) -> TypedEventHandler<S, A> {
  let emit = Arc::clone(emit);
  TypedEventHandler::new(move |_, _| {
    emit(kind, None);
    Ok(())
  })
}

/// `NSCMediaPlayerBridge`: the native side of canvas-media's Windows `Video` and `Audio`, over a
/// `Windows.Media.Playback.MediaPlayer` the TS creates and drives.
///
/// MediaPlayer raises its events on Media Foundation threads, where the runtime cannot run JS
/// delegates, so they are subscribed here and delivered on the JS thread (`onEvent`). With
/// `frames`, the player is expected in frame-server mode: each decoded frame is copied into a
/// texture as it arrives, for the view's `SurfaceImageSource` and for canvases (`readPixels`).
#[napi(js_name = "NSCMediaPlayerBridge", custom_finalize)]
pub struct NSCMediaPlayerBridge {
  live: Rc<LiveCell>,
  frames: Option<Arc<Frames>>,
  staging: RefCell<Option<(ID3D11Texture2D, u32, u32)>>,
  xaml: RefCell<Option<XamlSurface>>,
}

impl ObjectFinalize for NSCMediaPlayerBridge {
  fn finalize(mut self, _: Env) -> Result<()> {
    self.close();
    Ok(())
  }
}

#[napi]
impl NSCMediaPlayerBridge {
  /// `playerKey`: the MediaPlayer's pointer key. `onEvent(type, detail)`: `opened`, `ended`,
  /// `error` (detail: the message), `state` (detail: the `MediaPlaybackState`), `seeked`,
  /// `durationchange`, `resize`, `waiting`, `buffered` and, with `frames`, `frame` (at most one
  /// queued at a time).
  #[napi(
    constructor,
    ts_args_type = "playerKey: string, onEvent: (type: string, detail?: string) => void, frames?: boolean"
  )]
  pub fn new(env: Env, player_key: String, on_event: EventCallback, frames: Option<bool>) -> Result<Self> {
    ensure_cleanup_hook(&env)?;
    let raw = parse_pointer_key(&player_key)
      .ok_or_else(|| Error::from_reason(format!("Invalid MediaPlayer pointer: {player_key}")))?;
    let player: MediaPlayer = unsafe { IUnknown::from_raw_borrowed(&raw) }
      .ok_or_else(|| Error::from_reason("Invalid MediaPlayer pointer"))?
      .cast()
      .map_err(to_napi)?;
    let session = player.PlaybackSession().map_err(to_napi)?;

    let frame_queued = Arc::new(AtomicBool::new(false));
    let queued = Arc::clone(&frame_queued);
    let tsfn = on_event
      .build_threadsafe_function::<(String, Option<String>)>()
      .weak::<true>()
      .build_callback(move |ctx| {
        if ctx.value.0 == "frame" {
          queued.store(false, Ordering::Release);
        }
        Ok(FnArgs::from(ctx.value))
      })?;
    let emit: Emit = Arc::new(move |kind, detail| {
      tsfn.call((kind.to_string(), detail), ThreadsafeFunctionCallMode::NonBlocking);
    });

    let mut unsubscribe: Vec<Box<dyn FnOnce()>> = Vec::new();
    macro_rules! subscribe {
      ($target:expr, $add:ident, $remove:ident, $handler:expr) => {{
        let target = $target.clone();
        let token = target.$add(&$handler).map_err(to_napi)?;
        unsubscribe.push(Box::new(move || {
          let _ = target.$remove(token);
        }));
      }};
    }
    type OnPlayer = IInspectable;

    subscribe!(player, MediaOpened, RemoveMediaOpened, notify::<MediaPlayer, OnPlayer>(&emit, "opened"));
    subscribe!(player, MediaEnded, RemoveMediaEnded, notify::<MediaPlayer, OnPlayer>(&emit, "ended"));
    let failed = {
      let emit = Arc::clone(&emit);
      TypedEventHandler::<MediaPlayer, MediaPlayerFailedEventArgs>::new(move |_, args| {
        let message = args.ok().ok().map(|args| {
          let message = args.ErrorMessage().map(|m| m.to_string()).unwrap_or_default();
          if message.is_empty() {
            let code = args.ExtendedErrorCode().map(|code| code.0).unwrap_or(0);
            format!("Media playback failed (0x{:08x})", code as u32)
          } else {
            message
          }
        });
        emit("error", message);
        Ok(())
      })
    };
    subscribe!(player, MediaFailed, RemoveMediaFailed, failed);
    let state = {
      let emit = Arc::clone(&emit);
      TypedEventHandler::<MediaPlaybackSession, IInspectable>::new(move |session, _| {
        let state = session.ok().ok().and_then(|s| s.PlaybackState().ok()).map(|s| s.0.to_string());
        emit("state", state);
        Ok(())
      })
    };
    subscribe!(session, PlaybackStateChanged, RemovePlaybackStateChanged, state);
    subscribe!(session, SeekCompleted, RemoveSeekCompleted, notify::<MediaPlaybackSession, IInspectable>(&emit, "seeked"));
    subscribe!(
      session,
      NaturalDurationChanged,
      RemoveNaturalDurationChanged,
      notify::<MediaPlaybackSession, IInspectable>(&emit, "durationchange")
    );
    subscribe!(
      session,
      NaturalVideoSizeChanged,
      RemoveNaturalVideoSizeChanged,
      notify::<MediaPlaybackSession, IInspectable>(&emit, "resize")
    );
    subscribe!(session, BufferingStarted, RemoveBufferingStarted, notify::<MediaPlaybackSession, IInspectable>(&emit, "waiting"));
    subscribe!(session, BufferingEnded, RemoveBufferingEnded, notify::<MediaPlaybackSession, IInspectable>(&emit, "buffered"));

    let frames = frames.unwrap_or(false).then(|| Arc::new(Frames::default()));
    if let Some(frames) = frames.as_ref() {
      let frames = Arc::clone(frames);
      let emit = Arc::clone(&emit);
      let available = TypedEventHandler::<MediaPlayer, IInspectable>::new(move |player, _| {
        if let Ok(player) = player.ok() {
          if frames.copy_from(player) && !frame_queued.swap(true, Ordering::AcqRel) {
            emit("frame", None);
          }
        }
        Ok(())
      });
      subscribe!(player, VideoFrameAvailable, RemoveVideoFrameAvailable, available);
    }

    let live = Rc::new(RefCell::new(Some(Live { player, unsubscribe, frames: frames.clone() })));
    LIVE.with(|all| {
      let mut all = all.borrow_mut();
      all.retain(|live| live.strong_count() > 0);
      all.push(Rc::downgrade(&live));
    });
    Ok(Self { live, frames, staging: RefCell::new(None), xaml: RefCell::new(None) })
  }

  /// Unsubscribes from the player; the last frame stays readable.
  #[napi]
  pub fn close(&mut self) {
    if let Some(mut live) = self.live.borrow_mut().take() {
      live.close();
    }
    self.xaml.borrow_mut().take();
  }

  /// The size of the frames copied so far, 0 before the first.
  #[napi(getter)]
  pub fn video_width(&self) -> u32 {
    self.frames.as_ref().map_or(0, |frames| frames.size().0)
  }

  #[napi(getter)]
  pub fn video_height(&self) -> u32 {
    self.frames.as_ref().map_or(0, |frames| frames.size().1)
  }

  /// The adapter frames are decoded on (`LowPart | HighPart << 32`, 0 if unknown).
  #[napi(getter)]
  pub fn adapter_luid(&self) -> f64 {
    device().map_or(0., |device| device.luid as f64)
  }

  /// Whether frames can be shared with a WebGPU device (on the same adapter).
  #[napi(getter)]
  pub fn shares_frames(&self) -> bool {
    self.frames.is_some() && device().is_some_and(|device| device.fences.is_some())
  }

  /// The latest frame, shared for WebGPU: `address` is a `CanvasD3DSharedFrame` for
  /// `nativeTexture`. The texture is not written again until the consumer has signalled its
  /// release (or `close()` finds it never staged one). `null` before the first frame.
  #[napi]
  pub fn gpu_frame(&self) -> Option<NSCSharedFrame> {
    let frames = self.frames.as_ref()?;
    let device = device()?;
    let mut pool = frames.pool.lock().unwrap();
    let index = pool.latest?;
    let (width, height, ready_value) = (pool.width, pool.height, pool.latest_ready);
    let fences = pool.fences.as_ref()?;
    let (ready_fence_id, ready_fence) = (fences.ready_handle.id, fences.ready_handle.handle.0);
    let (release_fence_id, release_fence) = (fences.release_handle.id, fences.release_handle.handle.0);
    pool.next_release += 1;
    let release_value = pool.next_release;
    let frame = &mut pool.frames[index];
    let shared = frame.shared.as_ref()?;
    let (texture_id, texture) = (shared.id, shared.handle.0);
    frame.busy_until = frame.busy_until.max(release_value);
    Some(NSCSharedFrame {
      desc: Box::new(SharedFrameDesc {
        size: std::mem::size_of::<SharedFrameDesc>() as u32,
        consumed: 0,
        adapter_luid: device.luid,
        texture_id,
        texture,
        ready_fence_id,
        ready_fence,
        ready_value,
        release_fence_id,
        release_fence,
        release_value,
      }),
      frames: Arc::clone(frames),
      width,
      height,
      released: false,
    })
  }

  /// A tap on the player's decoded audio (an audio effect Media Foundation runs on it), passing it
  /// through until routed, for a MediaElementAudioSourceNode. Effects apply to the next source
  /// set, so this is called before any. Optional for the player: where the effect cannot be
  /// activated the element just keeps playing.
  #[napi]
  pub fn create_audio_tap(&self) -> Result<NSCAudioTap> {
    let live = self.live.borrow();
    let player = &live.as_ref().ok_or_else(|| Error::from_reason("The player is closed"))?.player;
    let tap = tap::Tap::new();
    let configuration = windows::Foundation::Collections::PropertySet::new().map_err(to_napi)?;
    configuration
      .Insert(&windows::core::HSTRING::from(tap::TAP_KEY), &windows::Foundation::PropertyValue::CreateUInt64(tap.id).map_err(to_napi)?)
      .map_err(to_napi)?;
    player
      .AddAudioEffect(&windows::core::HSTRING::from(tap::TAP_CLASS), true, &configuration)
      .map_err(to_napi)?;
    let source = Box::new(tap::AudioTapSource::new(&tap));
    Ok(NSCAudioTap { tap, source })
  }

  /// Changes with every frame copied; 0 before the first.
  #[napi(getter)]
  pub fn frame_id(&self) -> f64 {
    self.frames.as_ref().map_or(0., |frames| frames.generation.load(Ordering::Acquire) as f64)
  }

  /// Presents frames into a XAML `SurfaceImageSource` of `width` x `height` (the video's size) from
  /// now on, replacing any previous one. UI thread.
  #[napi]
  pub fn attach_surface_image_source(&self, key: String, width: u32, height: u32) -> bool {
    let (Some(device), Some(raw)) = (device(), parse_pointer_key(&key)) else { return false };
    let attached = (|| -> windows::core::Result<XamlSurface> {
      let unknown = unsafe { IUnknown::from_raw_borrowed(&raw) }.ok_or_else(windows::core::Error::empty)?;
      let native: ISurfaceImageSourceNative = unknown.cast()?;
      let dxgi: IDXGIDevice = device.device.cast()?;
      unsafe { native.SetDevice(dxgi.as_raw()) }.ok()?;
      Ok(XamlSurface { native, width, height })
    })();
    let attached = attached.ok();
    let ok = attached.is_some();
    *self.xaml.borrow_mut() = attached;
    ok
  }

  #[napi]
  pub fn detach_surface_image_source(&self) {
    self.xaml.borrow_mut().take();
  }

  /// Copies the current frame into the attached `SurfaceImageSource`; `false` without one of the
  /// frame's size. UI thread.
  #[napi]
  pub fn present(&self) -> bool {
    let (Some(frames), Some(device)) = (self.frames.as_ref(), device()) else { return false };
    let xaml = self.xaml.borrow();
    let Some(surface) = xaml.as_ref() else { return false };
    let pool = frames.pool.lock().unwrap();
    if pool.width != surface.width || pool.height != surface.height {
      return false;
    }
    let Some(frame) = pool.latest() else { return false };
    surface.present(device, &frame.texture).is_ok()
  }

  /// The current frame's pixels, RGBA and top row first (`videoWidth` x `videoHeight`), read back
  /// from the GPU; `null` before the first frame.
  #[napi]
  pub fn read_pixels(&self) -> Option<Uint8Array> {
    let (frames, device) = (self.frames.as_ref()?, device()?);
    let mut staging = self.staging.borrow_mut();
    let pool = frames.pool.lock().unwrap();
    let frame = pool.latest()?;
    let (width, height) = (pool.width, pool.height);
    if staging.as_ref().is_none_or(|(_, w, h)| *w != width || *h != height) {
      let desc = D3D11_TEXTURE2D_DESC {
        Usage: D3D11_USAGE_STAGING,
        BindFlags: 0,
        CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
        ..texture_desc(width, height, false)
      };
      let mut texture = None;
      unsafe { device.device.CreateTexture2D(&desc, None, Some(&mut texture)) }.ok()?;
      *staging = Some((texture?, width, height));
    }
    let (texture, ..) = staging.as_ref()?;
    unsafe { device.context.CopyResource(texture, &frame.texture) };
    drop(pool);

    let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
    unsafe { device.context.Map(texture, 0, D3D11_MAP_READ, 0, Some(&mut mapped)) }.ok()?;
    let row = width as usize * 4;
    let mut rgba = vec![0u8; row * height as usize];
    for (y, dst) in rgba.chunks_exact_mut(row).enumerate() {
      let src = unsafe { std::slice::from_raw_parts((mapped.pData as *const u8).add(y * mapped.RowPitch as usize), row) };
      for (dst, src) in dst.chunks_exact_mut(4).zip(src.chunks_exact(4)) {
        dst.copy_from_slice(&[src[2], src[1], src[0], 255]);
      }
    }
    unsafe { device.context.Unmap(texture, 0) };
    Some(Uint8Array::new(rgba))
  }
}

/// `__createTestPlayer(uri)`: the pointer key of a muted, autoplaying frame-server MediaPlayer on
/// `uri` (tests: Node has no WinRT projection to make one). Kept until `__closeTestPlayers()` or the
/// env's cleanup.
#[napi(js_name = "__createTestPlayer")]
pub fn create_test_player(env: Env, uri: String) -> Result<String> {
  ensure_cleanup_hook(&env)?;
  let player = MediaPlayer::new().map_err(to_napi)?;
  let setup = || -> windows::core::Result<()> {
    player.SetIsVideoFrameServerEnabled(true)?;
    player.SetIsMuted(true)?;
    player.SetAutoPlay(true)?;
    let uri = windows::Foundation::Uri::CreateUri(&windows::core::HSTRING::from(uri.as_str()))?;
    player.SetSource(&windows::Media::Core::MediaSource::CreateFromUri(&uri)?)
  };
  setup().map_err(to_napi)?;
  let key = format!("0x{:x}", player.as_raw() as usize);
  TEST_PLAYERS.with(|players| players.borrow_mut().push(ManuallyDrop::new(player)));
  Ok(key)
}

#[napi(js_name = "__closeTestPlayers")]
pub fn close_test_players() {
  for player in TEST_PLAYERS.with(|players| std::mem::take(&mut *players.borrow_mut())) {
    let player = ManuallyDrop::into_inner(player);
    let _ = player.Close();
  }
}
