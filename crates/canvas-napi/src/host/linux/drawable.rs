//! `CanvasModule.NSCDrawableHost` on Linux: the children of a `content="drawable"` canvas
//! (HTML-in-Canvas). The canvas widget holds each child in a slot it allocates but never shows.
//! A slot is placed where its child was last drawn and clips to the drawn part, so GTK's own
//! picking sends input there. Snapshots are rendered by the window's GSK renderer from the view's
//! last render node, and a `GtkWidgetPaintable` per view reports when that node changes.

use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene, gsk};
use napi::bindgen_prelude::Float64ArraySlice;
use napi_derive::napi;

use super::{gtk_ready, CanvasView, NSCCanvas};
use crate::image_asset::ImageAsset;

/// Slots with no geometry are allocated this far left of the canvas, where nothing picks them.
const OFFSCREEN: f32 = -1.0e6;

/// Frames a view may go without a render node before its snapshot is taken as empty.
const MAX_WAIT_FRAMES: u32 = 30;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Placement {
  /// Layout size of the child (DIP).
  pub width: i32,
  pub height: i32,
  /// Border box to canvas view (DIP), in DOMMatrix order (graphene's too).
  pub transform: Option<[f32; 16]>,
  /// The drawn part (x, y, width, height) of the border box.
  pub clip: Option<[f32; 4]>,
}

mod imp {
  use super::*;

  #[derive(Default)]
  pub struct DrawableSlot {
    pub placement: Cell<Placement>,
    pub size: Cell<(i32, i32)>,
  }

  #[glib::object_subclass]
  impl ObjectSubclass for DrawableSlot {
    const NAME: &'static str = "NSCDrawableSlot";
    type Type = super::DrawableSlot;
    type ParentType = gtk::Widget;
  }

  impl ObjectImpl for DrawableSlot {
    fn constructed(&self) {
      self.parent_constructed();
      // Picking stops at the padding box: the slot is the clip.
      self.obj().set_overflow(gtk::Overflow::Hidden);
    }

    fn dispose(&self) {
      while let Some(child) = self.obj().first_child() {
        child.unparent();
      }
    }
  }

  impl WidgetImpl for DrawableSlot {
    fn measure(&self, _: gtk::Orientation, _: i32) -> (i32, i32, i32, i32) {
      (0, 0, -1, -1)
    }

    fn size_allocate(&self, _: i32, _: i32, _: i32) {
      let Some(child) = self.obj().first_child() else {
        return;
      };
      let (width, height) = self.size.get();
      let (x, y) = self.placement.get().clip.map_or((0., 0.), |c| (c[0], c[1]));
      let origin = gsk::Transform::new().translate(&graphene::Point::new(-x, -y));
      child.allocate(width, height, -1, Some(origin));
    }
  }
}

glib::wrapper! {
  pub struct DrawableSlot(ObjectSubclass<imp::DrawableSlot>)
    @extends gtk::Widget,
    @implements gtk::Buildable, gtk::ConstraintTarget;
}

pub(super) fn allocate_children(view: &CanvasView) {
  let mut next = view.first_child();
  while let Some(widget) = next {
    next = widget.next_sibling();
    let Ok(slot) = widget.downcast::<DrawableSlot>() else {
      continue;
    };
    let Some(child) = slot.first_child().filter(|c| c.should_layout()) else {
      continue;
    };
    let placement = slot.imp().placement.get();
    // GTK needs at least the minimum size; NativeScript may lay out smaller.
    let (min_width, ..) = child.measure(gtk::Orientation::Horizontal, -1);
    let width = placement.width.max(min_width);
    let (min_height, ..) = child.measure(gtk::Orientation::Vertical, width);
    let height = placement.height.max(min_height);
    slot.imp().size.set((width, height));
    let (transform, clip) = match placement.transform {
      Some(m) => {
        let clip = placement
          .clip
          .unwrap_or([0., 0., width as f32, height as f32]);
        let t = gsk::Transform::new()
          .matrix(&graphene::Matrix::from_float(m))
          .translate(&graphene::Point::new(clip[0], clip[1]));
        (t, clip)
      }
      None => (
        gsk::Transform::new().translate(&graphene::Point::new(OFFSCREEN, 0.)),
        [0., 0., width as f32, height as f32],
      ),
    };
    let size = |v: f32| (v.ceil() as i32).max(1);
    slot.allocate(size(clip[2]), size(clip[3]), -1, Some(transform));
  }
}

/// Keeps each child's render node current without showing it: snapshots come from that node, and
/// a view's paintables only report a change once it is redrawn.
pub(super) fn snapshot_children(view: &CanvasView) {
  let Some(first) = view.first_child() else {
    return;
  };
  let scratch = gtk::Snapshot::new();
  let mut next = Some(first);
  while let Some(child) = next {
    view.snapshot_child(&child, &scratch);
    next = child.next_sibling();
  }
  drop(scratch.to_node());
}

fn widget_at(address: f64) -> Option<gtk::Widget> {
  let pointer = address as usize as *mut gtk::ffi::GtkWidget;
  if pointer.is_null() || !gtk_ready() {
    return None;
  }
  Some(unsafe { glib::translate::from_glib_none(pointer) })
}

fn key(widget: &gtk::Widget) -> usize {
  widget.as_ptr() as usize
}

struct Entry {
  widget: glib::WeakRef<gtk::Widget>,
  paintable: gtk::WidgetPaintable,
  handler: Option<glib::SignalHandlerId>,
  dirty: Rc<Cell<bool>>,
  texture: Option<gdk::Texture>,
  waited: u32,
}

impl Entry {
  fn new(widget: &gtk::Widget) -> Self {
    let paintable = gtk::WidgetPaintable::new(Some(widget));
    let dirty = Rc::new(Cell::new(true));
    let flag = Rc::clone(&dirty);
    let handler = paintable.connect_invalidate_contents(move |_| flag.set(true));
    Entry {
      widget: widget.downgrade(),
      paintable,
      handler: Some(handler),
      dirty,
      texture: None,
      waited: 0,
    }
  }
}

impl Drop for Entry {
  fn drop(&mut self) {
    if let Some(handler) = self.handler.take() {
      self.paintable.disconnect(handler);
    }
    self.paintable.set_widget(None::<&gtk::Widget>);
  }
}

/// `paintable` (the view at its CSS size) scaled by `scale` and offset by `-source`, into a
/// `width` x `height` texture. None while the view has never been drawn.
fn render(
  widget: &gtk::Widget,
  paintable: &impl IsA<gdk::Paintable>,
  width: i32,
  height: i32,
  scale: (f32, f32),
  source: (f32, f32),
) -> Option<gdk::Texture> {
  let renderer = widget.native()?.renderer()?;
  let snapshot = gtk::Snapshot::new();
  snapshot.scale(scale.0, scale.1);
  snapshot.translate(&graphene::Point::new(-source.0, -source.1));
  paintable.snapshot(&snapshot, widget.width() as f64, widget.height() as f64);
  let node = snapshot.to_node()?;
  let viewport = graphene::Rect::new(0., 0., width as f32, height as f32);
  Some(renderer.render_texture(node, Some(&viewport)))
}

fn transparent(width: i32, height: i32) -> gdk::Texture {
  let bytes = glib::Bytes::from_owned(vec![0u8; width as usize * height as usize * 4]);
  gdk::MemoryTexture::new(
    width,
    height,
    gdk::MemoryFormat::R8g8b8a8Premultiplied,
    &bytes,
    width as usize * 4,
  )
  .upcast()
}

fn load(asset: &ImageAsset, texture: &gdk::Texture) -> bool {
  asset.from_gdk_texture_sync(texture.as_ptr() as usize as f64)
}

#[napi(js_name = "NSCDrawableHost")]
pub struct DrawableHost {
  view: glib::WeakRef<CanvasView>,
  /// In `setDrawables` order.
  entries: Vec<Entry>,
}

#[napi]
impl DrawableHost {
  #[napi(constructor)]
  pub fn new(canvas: &NSCCanvas) -> Self {
    DrawableHost {
      view: canvas.view.downgrade(),
      entries: Vec::new(),
    }
  }

  /// Parents `child` (a GtkWidget address), in a slot, at `index` among the canvas's children.
  #[napi]
  pub fn attach(&self, child: f64, index: i32) -> bool {
    let (Some(view), Some(child)) = (self.view.upgrade(), widget_at(child)) else {
      return false;
    };
    if let Some(parent) = child.parent() {
      child.unparent();
      if let Ok(old) = parent.downcast::<DrawableSlot>() {
        old.unparent();
      }
    }
    let slot: DrawableSlot = glib::Object::new();
    child.set_parent(&slot);
    let mut sibling = view.first_child();
    for _ in 0..index.max(0) {
      sibling = sibling.and_then(|s| s.next_sibling());
    }
    match sibling {
      Some(sibling) if index >= 0 => slot.insert_before(&view, Some(&sibling)),
      _ => slot.set_parent(&view),
    }
    true
  }

  #[napi]
  pub fn detach(&self, child: f64) {
    let Some((slot, child)) = self.slot_of(child) else {
      return;
    };
    child.unparent();
    slot.unparent();
  }

  fn slot_of(&self, child: f64) -> Option<(DrawableSlot, gtk::Widget)> {
    let view = self.view.upgrade()?;
    let child = widget_at(child)?;
    let slot = child.parent()?.downcast::<DrawableSlot>().ok()?;
    (slot.parent().as_ref() == Some(view.upcast_ref())).then_some((slot, child))
  }

  /// Layout sizes (DIP) of the canvas's children: `[widget, width, height, ...]`.
  #[napi]
  pub fn place_all(&self, sizes: Vec<f64>) {
    for chunk in sizes.chunks_exact(3) {
      let (width, height) = (chunk[1].round() as i32, chunk[2].round() as i32);
      self.update_placement(chunk[0], |p| {
        p.width = width;
        p.height = height;
      });
    }
  }

  /// `[width, height]` (DIP) of a view.
  #[napi]
  pub fn size(&self, child: f64) -> Vec<f64> {
    widget_at(child).map_or(vec![0., 0.], |w| vec![w.width() as f64, w.height() as f64])
  }

  /// `matrix` (DOMMatrix order) maps a child's border box to the canvas view (DIP); the clip is
  /// in its border box. `raise` puts it above its siblings for picking.
  #[napi]
  pub fn set_geometry(
    &self,
    child: f64,
    matrix: Float64ArraySlice,
    has_clip: bool,
    clip_x: f64,
    clip_y: f64,
    clip_width: f64,
    clip_height: f64,
    raise: bool,
  ) {
    if matrix.len() != 16 {
      return;
    }
    let mut m = [0f32; 16];
    for (out, value) in m.iter_mut().zip(matrix.iter()) {
      *out = *value as f32;
    }
    let clip = has_clip.then(|| {
      let (x, width) = if clip_width < 0. {
        (clip_x + clip_width, -clip_width)
      } else {
        (clip_x, clip_width)
      };
      let (y, height) = if clip_height < 0. {
        (clip_y + clip_height, -clip_height)
      } else {
        (clip_y, clip_height)
      };
      [x as f32, y as f32, width as f32, height as f32]
    });
    self.update_placement(child, |p| {
      p.transform = Some(m);
      p.clip = clip;
    });
    if let (true, Some(view), Some((slot, _))) = (raise, self.view.upgrade(), self.slot_of(child)) {
      if slot.next_sibling().is_some() {
        slot.insert_before(&view, None::<&gtk::Widget>);
      }
    }
  }

  #[napi]
  pub fn clear_geometry(&self, child: f64) {
    self.update_placement(child, |p| {
      p.transform = None;
      p.clip = None;
    });
  }

  fn update_placement(&self, child: f64, f: impl FnOnce(&mut Placement)) {
    let (Some(view), Some((slot, _))) = (self.view.upgrade(), self.slot_of(child)) else {
      return;
    };
    let before = slot.imp().placement.get();
    let mut placement = before;
    f(&mut placement);
    if placement != before {
      slot.imp().placement.set(placement);
      view.queue_allocate();
    }
  }

  /// The views to snapshot (GtkWidget addresses), in tree order.
  #[napi]
  pub fn set_drawables(&mut self, widgets: Vec<f64>) {
    let mut old: HashMap<usize, Entry> = self
      .entries
      .drain(..)
      .filter_map(|e| e.widget.upgrade().map(|w| (key(&w), e)))
      .collect();
    for address in widgets {
      let Some(widget) = widget_at(address) else {
        continue;
      };
      let entry = old
        .remove(&key(&widget))
        .unwrap_or_else(|| Entry::new(&widget));
      self.entries.push(entry);
    }
  }

  /// Snapshots the views whose render node changed since the last call, at `scale` pixels per
  /// DIP; returns `[index, cssWidth, cssHeight, ...]` for each, index in `setDrawables` order.
  #[napi]
  pub fn update(&mut self, scale_x: f64, scale_y: f64) -> Option<Vec<f64>> {
    let mut changed: Option<Vec<f64>> = None;
    for (index, entry) in self.entries.iter_mut().enumerate() {
      let Some(widget) = entry.widget.upgrade() else {
        continue;
      };
      let (css_width, css_height) = (widget.width(), widget.height());
      if !widget.is_mapped() || css_width <= 0 || css_height <= 0 {
        continue;
      }
      let width = ((css_width as f64 * scale_x - 1e-3).ceil() as i32).max(1);
      let height = ((css_height as f64 * scale_y - 1e-3).ceil() as i32).max(1);
      let resized = entry
        .texture
        .as_ref()
        .is_none_or(|t| t.width() != width || t.height() != height);
      if !resized && !entry.dirty.get() {
        continue;
      }
      let scale = (scale_x as f32, scale_y as f32);
      let texture = match render(&widget, &entry.paintable, width, height, scale, (0., 0.)) {
        Some(texture) => texture,
        None if entry.waited >= MAX_WAIT_FRAMES => transparent(width, height),
        None => {
          entry.waited += 1;
          continue;
        }
      };
      entry.waited = 0;
      entry.dirty.set(false);
      entry.texture = Some(texture);
      changed.get_or_insert_with(Vec::new).extend([
        index as f64,
        css_width as f64,
        css_height as f64,
      ]);
    }
    changed
  }

  #[napi]
  pub fn load_snapshot(&self, child: f64, asset: &ImageAsset) -> bool {
    let Some(child) = widget_at(child) else {
      return false;
    };
    self
      .entries
      .iter()
      .find(|e| e.widget.upgrade().as_ref() == Some(&child))
      .and_then(|e| e.texture.as_ref())
      .is_some_and(|texture| load(asset, texture))
  }

  /// A fresh snapshot of `child` into `asset`, not shared with the per-frame one: `width` x
  /// `height` pixels of the `source` rect (DIP) of its border box, or all of it when the rect is
  /// empty.
  #[napi]
  pub fn capture(
    &self,
    child: f64,
    width: i32,
    height: i32,
    source_x: f64,
    source_y: f64,
    source_width: f64,
    source_height: f64,
    asset: &ImageAsset,
  ) -> bool {
    let Some(widget) = widget_at(child) else {
      return false;
    };
    let (source_width, source_height) = if source_width > 0. && source_height > 0. {
      (source_width, source_height)
    } else {
      (widget.width() as f64, widget.height() as f64)
    };
    if width <= 0 || height <= 0 || source_width <= 0. || source_height <= 0. {
      return false;
    }
    let (scale_x, scale_y) = (width as f64 / source_width, height as f64 / source_height);
    let existing = self
      .entries
      .iter()
      .find(|e| e.widget.upgrade().as_ref() == Some(&widget))
      .map(|e| e.paintable.clone());
    let paintable = existing.unwrap_or_else(|| gtk::WidgetPaintable::new(Some(&widget)));
    let scale = (scale_x as f32, scale_y as f32);
    let source = (source_x as f32, source_y as f32);
    let texture = render(&widget, &paintable, width, height, scale, source)
      .unwrap_or_else(|| transparent(width, height));
    load(asset, &texture)
  }

  #[napi]
  pub fn dispose(&mut self) {
    self.entries.clear();
  }
}
