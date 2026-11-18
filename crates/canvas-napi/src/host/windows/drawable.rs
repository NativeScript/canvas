//! `CanvasModule.NSCDrawableHost` on Windows: the children of a `content="drawable"` canvas
//! (HTML-in-Canvas). They live in a transparent XAML Canvas on top of the SwapChainPanel, each in a
//! Grid slot that carries the drawn geometry (transform, clip, hit testability), so XAML's own hit
//! testing finds a view where it was drawn.
//!
//! WinUI 3 has no per-element invalidation and only an async snapshot (RenderTargetBitmap), so
//! views are re-rendered on a backing-off schedule and compared, a small probe first. The async
//! operations are polled from `update()` rather than given completion handlers: delegates must be
//! `Send`, and XAML objects are not.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use ::windows::Foundation::{Point, Rect};
use ::windows::Storage::Streams::IBuffer;
use ::windows::Win32::System::WinRT::IBufferByteAccess;
use napi::bindgen_prelude::Float64ArraySlice;
use napi_derive::napi;
use windows_core::{IUnknown, Interface};
use windows_future::{AsyncStatus, IAsyncAction, IAsyncOperation};

use super::picking::{self, Perspective, Picking};
use super::xaml::{
  Event, IDependencyObject, Matrix, MatrixTransform, PointerWatch, RectangleGeometry,
  RenderTargetBitmap, UIElement,
};
use super::{parse_pointer_key, NSCCanvas};
use crate::image_asset::{load_bgra, ImageAsset};

const PROBE_SCALE: f64 = 0.25;
/// Time, not frames: refresh rates differ.
const HOT: Duration = Duration::from_millis(500);
const MIN_INTERVAL: Duration = Duration::from_millis(66);
const MAX_INTERVAL: Duration = Duration::from_millis(1000);

/// BGRA8, premultiplied.
struct Pixels {
  bytes: Vec<u8>,
  width: u32,
  height: u32,
}

enum Stage {
  Idle,
  Rendering(RenderTargetBitmap, IAsyncAction, Job),
  Reading(RenderTargetBitmap, IAsyncOperation<IBuffer>, Job),
}

#[derive(Clone, Copy)]
struct Job {
  probe: bool,
  /// Snapshot size (pixels) and the view's size (DIP) when it was asked for.
  target: (i32, i32),
  css: (f64, f64),
  scale: (f64, f64),
}

struct Watch {
  /// COM identity of the view.
  key: usize,
  element: UIElement,
  stage: Stage,
  snapshot: Option<Pixels>,
  css: (f64, f64),
  target: (i32, i32),
  probe: Option<Vec<u8>>,
  interval: Duration,
  next_check: Instant,
  hot_until: Instant,
  /// A new snapshot `update()` has not reported yet.
  fresh: bool,
}

struct Slot {
  child: usize,
  element: UIElement,
  slot: UIElement,
  /// The view's own UseLayoutRounding, restored when it leaves.
  layout_rounding: bool,
}

fn element_at(key: &str) -> Option<IUnknown> {
  let raw = parse_pointer_key(key).filter(|a| *a != 0)? as *mut std::ffi::c_void;
  unsafe { IUnknown::from_raw_borrowed(&raw) }.cloned()
}

fn identity(object: &impl Interface) -> usize {
  object
    .cast::<IUnknown>()
    .map_or(0, |unknown| unknown.as_raw() as usize)
}

fn buffer_bytes(buffer: &IBuffer) -> Option<Vec<u8>> {
  let length = buffer.Length().ok()? as usize;
  let access = buffer.cast::<IBufferByteAccess>().ok()?;
  let data = unsafe { access.Buffer() }.ok()?;
  (!data.is_null()).then(|| unsafe { std::slice::from_raw_parts(data, length) }.to_vec())
}

/// Bilinear resample of `rect` (in `source` pixels) to `width` x `height`.
fn resample(source: &Pixels, rect: (f64, f64, f64, f64), width: u32, height: u32) -> Vec<u8> {
  let (sw, sh) = (source.width as usize, source.height as usize);
  let mut out = vec![0u8; width as usize * height as usize * 4];
  let (max_x, max_y) = ((sw - 1) as f64, (sh - 1) as f64);
  let (step_x, step_y) = (rect.2 / width as f64, rect.3 / height as f64);
  for y in 0..height as usize {
    let fy = (rect.1 + (y as f64 + 0.5) * step_y - 0.5).clamp(0., max_y);
    let (y0, wy) = (fy.floor() as usize, fy - fy.floor());
    let y1 = (y0 + 1).min(sh - 1);
    for x in 0..width as usize {
      let fx = (rect.0 + (x as f64 + 0.5) * step_x - 0.5).clamp(0., max_x);
      let (x0, wx) = (fx.floor() as usize, fx - fx.floor());
      let x1 = (x0 + 1).min(sw - 1);
      let at = |px: usize, py: usize| (py * sw + px) * 4;
      let (a, b, c, d) = (at(x0, y0), at(x1, y0), at(x0, y1), at(x1, y1));
      let o = (y * width as usize + x) * 4;
      for k in 0..4 {
        let p = |i: usize| source.bytes[i + k] as f64;
        let top = p(a) + (p(b) - p(a)) * wx;
        let bottom = p(c) + (p(d) - p(c)) * wx;
        out[o + k] = (top + (bottom - top) * wy).round() as u8;
      }
    }
  }
  out
}

#[napi(js_name = "NSCDrawableHost")]
pub struct DrawableHost {
  panel: UIElement,
  host: UIElement,
  slots: Vec<Slot>,
  /// In `setDrawables` order.
  watches: Vec<Watch>,
  hot_all: Instant,
  /// Set by input on the hosted views (delegates must be Send).
  touched: Arc<AtomicBool>,
  handlers: Vec<(Event, i64)>,
  z_index: i32,
  slot_width: f64,
  /// Views drawn with a 3D matrix, aimed at the pointer.
  picking: Option<(Rc<RefCell<Picking>>, PointerWatch)>,
}

#[napi]
impl DrawableHost {
  #[napi(constructor)]
  pub fn new(canvas: &NSCCanvas) -> napi::Result<Self> {
    let reason = |e: windows_core::Error| napi::Error::from_reason(e.message());
    let panel: UIElement = canvas
      .panel
      .as_ref()
      .ok_or_else(|| napi::Error::from_reason("an offscreen canvas has no drawable children"))?
      .cast()
      .map_err(reason)?;
    let host = UIElement::new_canvas().map_err(reason)?;
    // Opacity hides the children but leaves them hit-testable.
    host.SetOpacity(0.).map_err(reason)?;
    host.set_z_index(1_000_000).map_err(reason)?;
    panel
      .Children()
      .and_then(|c| c.Append(&host))
      .map_err(reason)?;
    let touched = Arc::new(AtomicBool::new(false));
    let mut handlers = Vec::new();
    for event in Event::ALL {
      let touched = touched.clone();
      let token = host
        .add_handler(event, move || touched.store(true, Ordering::Relaxed))
        .map_err(reason)?;
      handlers.push((event, token));
    }
    let now = Instant::now();
    Ok(DrawableHost {
      panel,
      host,
      slots: Vec::new(),
      watches: Vec::new(),
      hot_all: now,
      touched,
      handlers,
      z_index: 0,
      slot_width: 0.,
      picking: None,
    })
  }

  /// Puts `child` (a pointer key) in a new slot at `index` among the canvas's children.
  #[napi]
  pub fn attach(&mut self, child: String, index: i32) -> bool {
    let Some(element) = element_at(&child).and_then(|u| u.cast::<UIElement>().ok()) else {
      return false;
    };
    let layout_rounding = element.UseLayoutRounding().unwrap_or(true);
    let attach = || -> windows_core::Result<UIElement> {
      let slot = UIElement::new_grid()?;
      // Pixel-snapped layout makes a 50 DIP view 50.4 at 125%. The view is only ever drawn through
      // the canvas, so its size stays the one it was given.
      slot.SetUseLayoutRounding(false)?;
      element.SetUseLayoutRounding(false)?;
      slot.SetIsHitTestVisible(false)?;
      slot.align_start()?;
      if self.slot_width > 0. {
        slot.SetWidth(self.slot_width)?;
      }
      slot.Children()?.Append(&element)?;
      let children = self.host.Children()?;
      if index >= 0 && (index as u32) < children.Size()? {
        children.InsertAt(index as u32, &slot)?;
      } else {
        children.Append(&slot)?;
      }
      Ok(slot)
    };
    let Ok(slot) = attach() else {
      let _ = element.SetUseLayoutRounding(layout_rounding);
      return false;
    };
    self.slots.push(Slot {
      child: identity(&element),
      element,
      slot,
      layout_rounding,
    });
    self.mark_hot();
    true
  }

  #[napi]
  pub fn detach(&mut self, child: String) {
    let Some(key) = element_at(&child).map(|u| identity(&u)) else {
      return;
    };
    let Some(i) = self.slots.iter().position(|s| s.child == key) else {
      return;
    };
    let slot = self.slots.remove(i);
    self.forget_perspective(&slot.slot);
    let _ = slot.element.SetUseLayoutRounding(slot.layout_rounding);
    let _ = slot.slot.Children().and_then(|c| c.Clear());
    let _ = self.remove_from(&self.host, &slot.slot);
  }

  fn remove_from(&self, parent: &UIElement, element: &UIElement) -> windows_core::Result<()> {
    let children = parent.Children()?;
    let mut index = 0u32;
    if children.IndexOf(element, &mut index)? {
      children.RemoveAt(index)?;
    }
    Ok(())
  }

  /// Something in the children may have changed: look every frame for a while.
  #[napi]
  pub fn mark_hot(&mut self) {
    self.hot_all = Instant::now() + HOT;
  }

  /// `[width, height]` (DIP) of a view.
  #[napi]
  pub fn size(&self, child: String) -> Vec<f64> {
    element_at(&child)
      .and_then(|u| u.cast::<UIElement>().ok())
      .map_or(vec![0., 0.], |e| {
        vec![
          e.ActualWidth().unwrap_or(0.),
          e.ActualHeight().unwrap_or(0.),
        ]
      })
  }

  /// Whether `source` (a pointer key, an event's OriginalSource) is inside the hosted children.
  #[napi]
  pub fn owns(&self, source: String) -> bool {
    let host = identity(&self.host);
    let mut node = element_at(&source).and_then(|u| u.cast::<IDependencyObject>().ok());
    for _ in 0..64 {
      let Some(current) = node else { return false };
      if identity(&current) == host {
        return true;
      }
      node = UIElement::parent_of(&current).ok();
    }
    false
  }

  /// The views to snapshot (pointer keys), in tree order.
  #[napi]
  pub fn set_drawables(&mut self, keys: Vec<String>) {
    let mut old = std::mem::take(&mut self.watches);
    let now = Instant::now();
    for key in keys {
      let Some(element) = element_at(&key).and_then(|u| u.cast::<UIElement>().ok()) else {
        continue;
      };
      let id = identity(&element);
      let watch = match old.iter().position(|w| w.key == id) {
        Some(i) => old.swap_remove(i),
        None => Watch {
          key: id,
          element,
          stage: Stage::Idle,
          snapshot: None,
          css: (0., 0.),
          target: (0, 0),
          probe: None,
          interval: MIN_INTERVAL,
          next_check: now,
          hot_until: now,
          fresh: false,
        },
      };
      self.watches.push(watch);
    }
  }

  /// Advances captures and starts the ones due, at `scale` pixels per DIP; `width` is the canvas's
  /// (DIP), which slots take. Returns `[index, cssWidth, cssHeight, ...]` for each new snapshot,
  /// index in `setDrawables` order.
  #[napi]
  pub fn update(&mut self, scale_x: f64, scale_y: f64, width: f64) -> Option<Vec<f64>> {
    let now = Instant::now();
    if self.touched.swap(false, Ordering::Relaxed) {
      self.hot_all = now + HOT;
    }
    if width > 0. && width != self.slot_width {
      self.slot_width = width;
      for slot in &self.slots {
        let _ = slot.slot.SetWidth(width);
      }
    }
    let hot_all = self.hot_all;
    let mut changed: Option<Vec<f64>> = None;
    for (index, watch) in self.watches.iter_mut().enumerate() {
      advance(watch, now, hot_all);
      if watch.fresh {
        watch.fresh = false;
        if watch.snapshot.is_some() {
          changed
            .get_or_insert_with(Vec::new)
            .extend([index as f64, watch.css.0, watch.css.1]);
        }
      }
      if matches!(watch.stage, Stage::Idle) {
        let hot = now <= watch.hot_until || now <= hot_all;
        if watch.snapshot.is_none() || hot || now >= watch.next_check {
          start(watch, (scale_x, scale_y), now, false);
        }
      }
    }
    changed
  }

  /// Loads the current snapshot of `child` into `asset`.
  #[napi]
  pub fn load_snapshot(&self, child: String, asset: &ImageAsset) -> bool {
    self
      .snapshot_of(&child)
      .is_some_and(|p| load_bgra(asset, p.width, p.height, &p.bytes))
  }

  /// `width` x `height` pixels of the `source` rect (DIP) of `child`'s last snapshot, or all of it
  /// when the rect is empty, into `asset`. RenderTargetBitmap cannot render synchronously.
  #[napi]
  pub fn capture(
    &self,
    child: String,
    width: u32,
    height: u32,
    source_x: f64,
    source_y: f64,
    source_width: f64,
    source_height: f64,
    asset: &ImageAsset,
  ) -> bool {
    let Some(watch) = self.watch_of(&child) else {
      return false;
    };
    let Some(pixels) = &watch.snapshot else {
      return false;
    };
    if width == 0 || height == 0 {
      return false;
    }
    let rx = pixels.width as f64 / watch.css.0.max(1e-6);
    let ry = pixels.height as f64 / watch.css.1.max(1e-6);
    let rect = if source_width > 0. && source_height > 0. {
      (
        source_x * rx,
        source_y * ry,
        source_width * rx,
        source_height * ry,
      )
    } else {
      (0., 0., pixels.width as f64, pixels.height as f64)
    };
    if rect == (0., 0., width as f64, height as f64) {
      return load_bgra(asset, width, height, &pixels.bytes);
    }
    load_bgra(asset, width, height, &resample(pixels, rect, width, height))
  }

  fn watch_of(&self, child: &str) -> Option<&Watch> {
    let id = identity(&element_at(child)?);
    self.watches.iter().find(|w| w.key == id)
  }

  fn snapshot_of(&self, child: &str) -> Option<&Pixels> {
    self.watch_of(child)?.snapshot.as_ref()
  }

  /// `matrix` (DOMMatrix order) maps a child's border box to the canvas view (DIP); the clip is
  /// in its border box. `raise` puts it above its siblings for hit testing.
  #[napi]
  pub fn set_geometry(
    &mut self,
    child: String,
    matrix: Float64ArraySlice,
    has_clip: bool,
    clip_x: f64,
    clip_y: f64,
    clip_width: f64,
    clip_height: f64,
    raise: bool,
  ) -> bool {
    if matrix.len() != 16 {
      return false;
    }
    let Some(id) = element_at(&child).map(|u| identity(&u)) else {
      return false;
    };
    let Some(slot) = self.slots.iter().find(|s| s.child == id) else {
      return false;
    };
    let m: &[f64] = &matrix;
    let panel = &self.panel;
    let picking_ref = &mut self.picking;
    let mut apply = || -> windows_core::Result<()> {
      // The view's offset in its slot (its alignment and margin).
      let origin = slot.element.point_in(&slot.slot, Point { X: 0., Y: 0. })?;
      let (ox, oy) = (origin.X as f64, origin.Y as f64);
      // m · translate(-ox, -oy): the slot's origin maps where the view's border box would.
      let t = |i: usize| m[i];
      let (tx, ty, tz, tw) = (
        t(12) - ox * t(0) - oy * t(4),
        t(13) - ox * t(1) - oy * t(5),
        t(14) - ox * t(2) - oy * t(6),
        t(15) - ox * t(3) - oy * t(7),
      );
      // XAML picks a rasterization scale from a transform's matrix, so every new angle would change
      // the snapshot. Pin it to the root's.
      let raster = slot.slot.root_scale().unwrap_or(1.);
      if slot.slot.RasterizationScale().ok() != Some(raster) {
        slot.slot.SetRasterizationScale(raster)?;
      }
      // Without perspective the view's plane maps affinely, and a RenderTransform is exact.
      let affine = t(3) == 0. && t(7) == 0. && tw == 1.;
      if affine {
        forget(picking_ref, &slot.slot);
        let transform = MatrixTransform::new()?;
        transform.SetMatrix(Matrix {
          M11: t(0),
          M12: t(1),
          M21: t(4),
          M22: t(5),
          OffsetX: tx,
          OffsetY: ty,
        })?;
        slot.slot.SetRenderTransform(Some(&transform))?;
      } else {
        // XAML input follows neither a Projection nor TransformMatrix: picking aims the slot at
        // the pointer instead.
        let mut matrix = [0.; 16];
        matrix[..12].copy_from_slice(&m[..12]);
        matrix[12..].copy_from_slice(&[tx, ty, tz, tw]);
        let view = (
          ox,
          oy,
          slot.element.ActualWidth()?,
          slot.element.ActualHeight()?,
        );
        if picking_ref.is_none() {
          *picking_ref = Some(picking::watch(panel)?);
        }
        if let Some((picking, _)) = picking_ref.as_ref() {
          let mut picking = picking.borrow_mut();
          let index = match picking.views.iter().position(|p| p.slot == slot.slot) {
            Some(i) => {
              picking.views[i].update(matrix, view);
              i
            }
            None => {
              picking
                .views
                .push(Perspective::new(slot.slot.clone(), matrix, view)?);
              picking.views.len() - 1
            }
          };
          picking.refresh(index);
        }
      }
      if has_clip {
        let geometry = RectangleGeometry::new()?;
        geometry.SetRect(Rect {
          X: (clip_x.min(clip_x + clip_width) + ox) as f32,
          Y: (clip_y.min(clip_y + clip_height) + oy) as f32,
          Width: clip_width.abs() as f32,
          Height: clip_height.abs() as f32,
        })?;
        slot.slot.SetClip(Some(&geometry))?;
      } else {
        slot.slot.SetClip(None)?;
      }
      slot.slot.SetIsHitTestVisible(true)
    };
    let applied = apply().is_ok();
    if applied && raise {
      self.z_index += 1;
      let _ = slot.slot.set_z_index(self.z_index);
    }
    applied
  }

  /// Aims the views drawn with a 3D matrix at (x, y) in the canvas view (DIP), as a pointer moving
  /// there does.
  #[napi]
  pub fn aim(&self, x: f64, y: f64) {
    if let Some((picking, _)) = &self.picking {
      if let Ok(mut picking) = picking.try_borrow_mut() {
        picking.aim(x, y);
      }
    }
  }

  fn forget_perspective(&self, slot: &UIElement) {
    forget(&self.picking, slot);
  }

  #[napi]
  pub fn clear_geometry(&self, child: String) {
    let Some(id) = element_at(&child).map(|u| identity(&u)) else {
      return;
    };
    if let Some(slot) = self.slots.iter().find(|s| s.child == id) {
      self.forget_perspective(&slot.slot);
      let _ = slot.slot.SetIsHitTestVisible(false);
      let _ = slot.slot.SetRenderTransform(None);
      let _ = slot.slot.SetClip(None);
    }
  }

  #[napi]
  pub fn dispose(&mut self) {
    for (event, token) in self.handlers.drain(..) {
      let _ = self.host.remove_handler(event, token);
    }
    self.picking = None;
    self.watches.clear();
    for slot in self.slots.drain(..) {
      let _ = slot.element.SetUseLayoutRounding(slot.layout_rounding);
      let _ = slot.slot.Children().and_then(|c| c.Clear());
    }
    let _ = self.host.Children().and_then(|c| c.Clear());
    let _ = self.remove_from(&self.panel, &self.host);
  }
}

/// Stops aiming `slot` at the pointer.
fn forget(picking: &Option<(Rc<RefCell<Picking>>, PointerWatch)>, slot: &UIElement) {
  if let Some((picking, _)) = picking {
    if let Ok(mut picking) = picking.try_borrow_mut() {
      picking.views.retain(|p| &p.slot != slot);
    }
  }
}

/// Starts a probe, or a full render when `full`, there is nothing to compare with or the size
/// changed.
fn start(watch: &mut Watch, scale: (f64, f64), now: Instant, full: bool) {
  let begin = || -> windows_core::Result<Stage> {
    let css = (watch.element.ActualWidth()?, watch.element.ActualHeight()?);
    if css.0 <= 0. || css.1 <= 0. {
      return Ok(Stage::Idle);
    }
    let target = (
      ((css.0 * scale.0 - 1e-3).ceil() as i32).max(1),
      ((css.1 * scale.1 - 1e-3).ceil() as i32).max(1),
    );
    let probe = !full && watch.snapshot.is_some() && watch.target == target;
    let factor = if probe { PROBE_SCALE } else { 1. };
    // RenderTargetBitmap sizes are in DIP; it rasterizes at the XamlRoot's scale.
    let rasterization = watch.element.root_scale().unwrap_or(1.).max(1e-6);
    let size = |pixels: i32| (((pixels as f64 * factor) / rasterization).round() as i32).max(1);
    let bitmap = RenderTargetBitmap::new()?;
    let op = bitmap.RenderToSizeAsync(&watch.element, size(target.0), size(target.1))?;
    Ok(Stage::Rendering(
      bitmap,
      op,
      Job {
        probe,
        target,
        css,
        scale,
      },
    ))
  };
  match begin() {
    Ok(stage) => watch.stage = stage,
    Err(_) => settle(watch, false, now, now),
  }
}

/// Moves a capture on once XAML has finished its current step.
fn advance(watch: &mut Watch, now: Instant, hot_all: Instant) {
  let stage = std::mem::replace(&mut watch.stage, Stage::Idle);
  watch.stage = match stage {
    Stage::Idle => Stage::Idle,
    Stage::Rendering(bitmap, op, job) => match op.Status() {
      Ok(AsyncStatus::Started) => Stage::Rendering(bitmap, op, job),
      Ok(AsyncStatus::Completed) => match bitmap.GetPixelsAsync() {
        Ok(read) => Stage::Reading(bitmap, read, job),
        Err(_) => {
          settle(watch, false, now, hot_all);
          Stage::Idle
        }
      },
      _ => {
        settle(watch, false, now, hot_all);
        Stage::Idle
      }
    },
    Stage::Reading(bitmap, op, job) => match op.Status() {
      Ok(AsyncStatus::Started) => Stage::Reading(bitmap, op, job),
      Ok(AsyncStatus::Completed) => {
        let pixels = op
          .GetResults()
          .ok()
          .and_then(|b| buffer_bytes(&b))
          .map(|bytes| Pixels {
            bytes,
            width: bitmap.PixelWidth().unwrap_or(0).max(0) as u32,
            height: bitmap.PixelHeight().unwrap_or(0).max(0) as u32,
          });
        finish(watch, job, pixels, now, hot_all)
      }
      _ => {
        settle(watch, false, now, hot_all);
        Stage::Idle
      }
    },
  };
}

fn finish(
  watch: &mut Watch,
  job: Job,
  pixels: Option<Pixels>,
  now: Instant,
  hot_all: Instant,
) -> Stage {
  let Some(pixels) = pixels.filter(|p| p.width > 0 && p.height > 0) else {
    settle(watch, false, now, hot_all);
    return Stage::Idle;
  };
  if job.probe {
    if watch
      .probe
      .as_deref()
      .is_some_and(|p| same(p, &pixels.bytes))
    {
      settle(watch, false, now, hot_all);
      return Stage::Idle;
    }
    watch.probe = Some(pixels.bytes);
    // The probe differs: render the full snapshot.
    start(watch, job.scale, now, true);
    return std::mem::replace(&mut watch.stage, Stage::Idle);
  }
  // RenderToSizeAsync takes whole DIP, so at a fractional scale the bitmap misses the target by a
  // pixel (13 DIP at 125% is 17 px for a 16 px target).
  let (width, height) = (job.target.0 as u32, job.target.1 as u32);
  let pixels = if (pixels.width, pixels.height) == (width, height) {
    pixels
  } else {
    let rect = (0., 0., pixels.width as f64, pixels.height as f64);
    Pixels {
      bytes: resample(&pixels, rect, width, height),
      width,
      height,
    }
  };
  let resized = watch.target != job.target;
  if !resized
    && watch
      .snapshot
      .as_ref()
      .is_some_and(|s| same(&s.bytes, &pixels.bytes))
  {
    settle(watch, false, now, hot_all);
    return Stage::Idle;
  }
  if resized {
    watch.probe = None;
  }
  watch.snapshot = Some(pixels);
  watch.target = job.target;
  watch.css = job.css;
  watch.fresh = true;
  settle(watch, true, now, hot_all);
  Stage::Idle
}

/// Equal but for rounding: under a skewed transform the GPU renders an unchanged view with some
/// edge pixels off by one from one render to the next. The earlier snapshot stays the reference.
fn same(a: &[u8], b: &[u8]) -> bool {
  a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.abs_diff(*y) <= 1)
}

fn settle(watch: &mut Watch, changed: bool, now: Instant, hot_all: Instant) {
  if changed {
    watch.interval = MIN_INTERVAL;
    watch.hot_until = now + HOT;
  } else if now > watch.hot_until && now > hot_all {
    watch.interval = (watch.interval * 2).min(MAX_INTERVAL);
  }
  watch.next_check = now + watch.interval;
}
