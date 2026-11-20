//! Hit testing for views drawn with a 3D matrix. XAML input follows a RenderTransform (2D affine)
//! but neither a Projection nor TransformMatrix, so each such view's slot gets the matrix's affine
//! approximation at the point under the pointer, which is exact there. The canvas view watches
//! PointerEntered and PointerMoved: XAML has hit-tested that event already, so the press, release
//! or next move at that point is what lands exactly (a touch enters before it presses).

use std::cell::RefCell;
use std::rc::Rc;

use ::windows::Foundation::Point;

use super::xaml::{Matrix, MatrixTransform, PointerWatch, UIElement};

/// A view's slot and its matrix, in the canvas view's DIP.
pub struct Perspective {
  pub slot: UIElement,
  transform: MatrixTransform,
  /// What `transform` holds: setting it, even to the same value, re-renders the view.
  current: Option<Matrix>,
  /// The DOMMatrix with its translation moved to the slot origin; z is not needed for picking.
  t: [f64; 16],
  /// The view's border box in the slot.
  view: (f64, f64, f64, f64),
}

impl Perspective {
  /// `t` maps slot coordinates to the canvas view; the view sits at `view` (x, y, w, h) in the slot.
  pub fn new(
    slot: UIElement,
    t: [f64; 16],
    view: (f64, f64, f64, f64),
  ) -> windows_core::Result<Self> {
    let transform = MatrixTransform::new()?;
    slot.SetRenderTransform(Some(&transform))?;
    Ok(Perspective {
      slot,
      transform,
      current: None,
      t,
      view,
    })
  }

  pub fn update(&mut self, t: [f64; 16], view: (f64, f64, f64, f64)) {
    self.t = t;
    self.view = view;
  }

  /// (X, Y, W) of slot point (u, v).
  fn project(&self, u: f64, v: f64) -> (f64, f64, f64) {
    let t = &self.t;
    let w = t[3] * u + t[7] * v + t[15];
    (
      (t[0] * u + t[4] * v + t[12]) / w,
      (t[1] * u + t[5] * v + t[13]) / w,
      w,
    )
  }

  /// The slot point drawn at (x, y); None when (x, y) is not on the view's plane.
  fn unproject(&self, x: f64, y: f64) -> Option<(f64, f64)> {
    let t = &self.t;
    let (a, b) = (t[0] - x * t[3], t[4] - x * t[7]);
    let (c, d) = (t[1] - y * t[3], t[5] - y * t[7]);
    let (e, f) = (x * t[15] - t[12], y * t[15] - t[13]);
    let det = a * d - b * c;
    if det.abs() < 1e-12 {
      return None;
    }
    let (u, v) = ((e * d - b * f) / det, (a * f - e * c) / det);
    (self.project(u, v).2 > 0.).then_some((u, v))
  }

  fn clamp(&self, (u, v): (f64, f64)) -> (f64, f64) {
    let (x0, y0, w, h) = self.view;
    (u.clamp(x0, x0 + w), v.clamp(y0, y0 + h))
  }

  fn contains(&self, (u, v): (f64, f64)) -> bool {
    let (x0, y0, w, h) = self.view;
    u >= x0 && u <= x0 + w && v >= y0 && v <= y0 + h
  }

  /// Whether XAML, with the transform as set, would hit the view at (x, y).
  fn hit_as_set(&self, x: f64, y: f64) -> bool {
    let Some(m) = self.current else {
      return false;
    };
    let det = m.M11 * m.M22 - m.M12 * m.M21;
    if det.abs() < 1e-12 {
      return false;
    }
    let (dx, dy) = (x - m.OffsetX, y - m.OffsetY);
    self.contains((
      (dx * m.M22 - dy * m.M21) / det,
      (dy * m.M11 - dx * m.M12) / det,
    ))
  }

  /// Sets the slot's transform to the matrix's tangent at slot point `at`.
  fn aim_at(&mut self, at: (f64, f64)) -> windows_core::Result<()> {
    let (u, v) = at;
    let (x, y, w) = self.project(u, v);
    if !(w > 0.) {
      return Ok(());
    }
    let t = &self.t;
    let m11 = (t[0] - x * t[3]) / w;
    let m12 = (t[1] - y * t[3]) / w;
    let m21 = (t[4] - x * t[7]) / w;
    let m22 = (t[5] - y * t[7]) / w;
    let matrix = Matrix {
      M11: m11,
      M12: m12,
      M21: m21,
      M22: m22,
      OffsetX: x - (u * m11 + v * m21),
      OffsetY: y - (u * m12 + v * m22),
    };
    if self.current != Some(matrix) {
      self.transform.SetMatrix(matrix)?;
      self.current = Some(matrix);
    }
    Ok(())
  }

  fn center(&self) -> (f64, f64) {
    (
      self.view.0 + self.view.2 / 2.,
      self.view.1 + self.view.3 / 2.,
    )
  }

  /// A point neither on the view nor under the transform as set misses either way; keeping the
  /// transform spares a re-render.
  fn aim(&mut self, x: f64, y: f64) {
    let at = self.unproject(x, y);
    let on_view = at.is_some_and(|at| self.contains(at));
    if self.current.is_some() && !on_view && !self.hit_as_set(x, y) {
      return;
    }
    let at = at.map_or_else(|| self.center(), |at| self.clamp(at));
    let _ = self.aim_at(at);
  }
}

/// One canvas's 3D-drawn views.
pub struct Picking {
  pub views: Vec<Perspective>,
  /// The last pointer position, in the canvas view's DIP.
  last: Option<(f64, f64)>,
}

impl Picking {
  /// Aims every view at (x, y) in the canvas view (DIP), as a pointer there does.
  pub fn aim(&mut self, x: f64, y: f64) {
    self.last = Some((x, y));
    for view in &mut self.views {
      view.aim(x, y);
    }
  }

  /// Aims view `index` at the last pointer position, or at its center before any.
  pub fn refresh(&mut self, index: usize) {
    let Some(view) = self.views.get_mut(index) else {
      return;
    };
    match self.last {
      Some((x, y)) => view.aim(x, y),
      None => {
        let center = view.center();
        let _ = view.aim_at(center);
      }
    }
  }
}

/// Starts aiming the views of `panel` (the canvas view) at the pointer.
pub fn watch(panel: &UIElement) -> windows_core::Result<(Rc<RefCell<Picking>>, PointerWatch)> {
  let picking = Rc::new(RefCell::new(Picking {
    views: Vec::new(),
    last: None,
  }));
  let weak = Rc::downgrade(&picking);
  let watch = panel.watch_pointer(move |point: Point| {
    if let Some(picking) = weak.upgrade() {
      if let Ok(mut picking) = picking.try_borrow_mut() {
        picking.aim(point.X as f64, point.Y as f64);
      }
    }
  })?;
  Ok((picking, watch))
}
