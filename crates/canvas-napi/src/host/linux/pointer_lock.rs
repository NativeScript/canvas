//! Pointer lock under Wayland, as Firefox and Chromium do it: `zwp_pointer_constraints_v1` locks
//! the pointer on the window's `wl_surface` and `zwp_relative_pointer_v1` reports its travel,
//! unbounded by the screen edges. GTK 4 binds neither protocol, so they are bound here on GTK's
//! own `wl_display` and dispatched on its default queue, in order with GTK's events. They reach
//! the view as its `pointer-lock-state` and `pointer-lock-motion` signals.
//!
//! libwayland-client (loaded with GTK) is called directly; the two protocols are described below
//! as its `wl_interface` tables.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;
use std::sync::OnceLock;

use gtk::prelude::*;
use gtk::glib;

use super::CanvasView;

#[repr(C)]
struct Message {
  name: *const c_char,
  signature: *const c_char,
  types: *const *const Interface,
}

#[repr(C)]
struct Interface {
  name: *const c_char,
  version: c_int,
  method_count: c_int,
  methods: *const Message,
  event_count: c_int,
  events: *const Message,
}

type Proxy = c_void;

/// libwayland-client's calls (all present since 1.10) and the interfaces used.
struct Wayland {
  _lib: libloading::Library,
  marshal: unsafe extern "C" fn(*mut Proxy, u32, ...),
  marshal_constructor: unsafe extern "C" fn(*mut Proxy, u32, *const Interface, ...) -> *mut Proxy,
  marshal_constructor_versioned:
    unsafe extern "C" fn(*mut Proxy, u32, *const Interface, u32, ...) -> *mut Proxy,
  add_listener: unsafe extern "C" fn(*mut Proxy, *const c_void, *mut c_void) -> c_int,
  destroy: unsafe extern "C" fn(*mut Proxy),
  create_wrapper: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
  wrapper_destroy: unsafe extern "C" fn(*mut c_void),
  set_queue: unsafe extern "C" fn(*mut Proxy, *mut c_void),
  create_queue: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
  queue_destroy: unsafe extern "C" fn(*mut c_void),
  roundtrip_queue: unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int,
  flush: unsafe extern "C" fn(*mut c_void) -> c_int,
  registry: *const Interface,
  constraints: *const Interface,
  locked_pointer: *const Interface,
  relative_manager: *const Interface,
  relative_pointer: *const Interface,
  /// GDK's Wayland accessors (libgtk-4 built with Wayland).
  gdk_display: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
  gdk_surface: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
  gdk_pointer: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
}

// Only used on GTK's thread; the tables are immutable.
unsafe impl Send for Wayland {}
unsafe impl Sync for Wayland {}

fn leak<T>(items: Vec<T>) -> *const T {
  Box::leak(items.into_boxed_slice()).as_ptr()
}

fn message(name: &'static CStr, signature: &'static CStr, types: Vec<*const Interface>) -> Message {
  Message {
    name: name.as_ptr(),
    signature: signature.as_ptr(),
    // libwayland reads one entry per argument; argument-less messages still get a valid pointer.
    types: leak(if types.is_empty() { vec![ptr::null()] } else { types }),
  }
}

fn interface(name: &'static CStr, methods: Vec<Message>, events: Vec<Message>) -> *const Interface {
  Box::leak(Box::new(Interface {
    name: name.as_ptr(),
    version: 1,
    method_count: methods.len() as c_int,
    methods: leak(methods),
    event_count: events.len() as c_int,
    events: leak(events),
  }))
}

fn wayland() -> Option<&'static Wayland> {
  static WAYLAND: OnceLock<Option<Wayland>> = OnceLock::new();
  WAYLAND
    .get_or_init(|| unsafe {
      let lib = libloading::Library::new("libwayland-client.so.0").ok()?;
      let this = libloading::os::unix::Library::this();
      macro_rules! sym {
        ($lib:expr, $name:literal) => {
          *$lib.get($name).ok()?
        };
      }
      // Data symbols: their address is the interface.
      let data = |name: &[u8]| -> Option<*const Interface> {
        let symbol = lib.get::<*const Interface>(name).ok()?;
        symbol.try_as_raw_ptr().map(|address| address as *const Interface)
      };
      let registry = data(b"wl_registry_interface\0")?;
      let surface = data(b"wl_surface_interface\0")?;
      let pointer = data(b"wl_pointer_interface\0")?;
      let region = data(b"wl_region_interface\0")?;
      let null = ptr::null::<Interface>();
      let locked_pointer = interface(
        c"zwp_locked_pointer_v1",
        vec![
          message(c"destroy", c"", vec![]),
          message(c"set_cursor_position_hint", c"ff", vec![null, null]),
          message(c"set_region", c"?o", vec![region]),
        ],
        vec![message(c"locked", c"", vec![]), message(c"unlocked", c"", vec![])],
      );
      let confined_pointer = interface(
        c"zwp_confined_pointer_v1",
        vec![message(c"destroy", c"", vec![]), message(c"set_region", c"?o", vec![region])],
        vec![message(c"confined", c"", vec![]), message(c"unconfined", c"", vec![])],
      );
      let constraints = interface(
        c"zwp_pointer_constraints_v1",
        vec![
          message(c"destroy", c"", vec![]),
          message(c"lock_pointer", c"noo?ou", vec![locked_pointer, surface, pointer, region, null]),
          message(c"confine_pointer", c"noo?ou", vec![confined_pointer, surface, pointer, region, null]),
        ],
        vec![],
      );
      let relative_pointer = interface(
        c"zwp_relative_pointer_v1",
        vec![message(c"destroy", c"", vec![])],
        vec![message(c"relative_motion", c"uuffff", vec![null; 6])],
      );
      let relative_manager = interface(
        c"zwp_relative_pointer_manager_v1",
        vec![
          message(c"destroy", c"", vec![]),
          message(c"get_relative_pointer", c"no", vec![relative_pointer, pointer]),
        ],
        vec![],
      );
      Some(Wayland {
        marshal: sym!(lib, b"wl_proxy_marshal\0"),
        marshal_constructor: sym!(lib, b"wl_proxy_marshal_constructor\0"),
        marshal_constructor_versioned: sym!(lib, b"wl_proxy_marshal_constructor_versioned\0"),
        add_listener: sym!(lib, b"wl_proxy_add_listener\0"),
        destroy: sym!(lib, b"wl_proxy_destroy\0"),
        create_wrapper: sym!(lib, b"wl_proxy_create_wrapper\0"),
        wrapper_destroy: sym!(lib, b"wl_proxy_wrapper_destroy\0"),
        set_queue: sym!(lib, b"wl_proxy_set_queue\0"),
        create_queue: sym!(lib, b"wl_display_create_queue\0"),
        queue_destroy: sym!(lib, b"wl_event_queue_destroy\0"),
        roundtrip_queue: sym!(lib, b"wl_display_roundtrip_queue\0"),
        flush: sym!(lib, b"wl_display_flush\0"),
        gdk_display: sym!(this, b"gdk_wayland_display_get_wl_display\0"),
        gdk_surface: sym!(this, b"gdk_wayland_surface_get_wl_surface\0"),
        gdk_pointer: sym!(this, b"gdk_wayland_device_get_wl_pointer\0"),
        registry,
        constraints,
        locked_pointer,
        relative_manager,
        relative_pointer,
        _lib: lib,
      })
    })
    .as_ref()
}

/// The two globals on one display.
#[derive(Clone, Copy)]
struct Globals {
  constraints: *mut Proxy,
  relative_manager: *mut Proxy,
}

thread_local! {
  /// Globals by `wl_display`; None if the compositor lacks either.
  static GLOBALS: RefCell<HashMap<usize, Option<Globals>>> = RefCell::new(HashMap::new());
  /// `wl_surface`s with a lock: a second one is a protocol error (`already_constrained`), which
  /// would end GTK's connection.
  static LOCKED_SURFACES: RefCell<HashSet<usize>> = RefCell::new(HashSet::new());
}

#[derive(Default)]
struct Found {
  constraints: Option<u32>,
  relative_manager: Option<u32>,
}

unsafe extern "C" fn registry_global(
  data: *mut c_void,
  _: *mut Proxy,
  name: u32,
  interface: *const c_char,
  _version: u32,
) {
  let found = &mut *(data as *mut Found);
  match CStr::from_ptr(interface).to_bytes() {
    b"zwp_pointer_constraints_v1" => found.constraints = Some(name),
    b"zwp_relative_pointer_manager_v1" => found.relative_manager = Some(name),
    _ => {}
  }
}

unsafe extern "C" fn registry_global_remove(_: *mut c_void, _: *mut Proxy, _: u32) {}

#[repr(C)]
struct RegistryListener {
  global: unsafe extern "C" fn(*mut c_void, *mut Proxy, u32, *const c_char, u32),
  global_remove: unsafe extern "C" fn(*mut c_void, *mut Proxy, u32),
}

static REGISTRY_LISTENER: RegistryListener = RegistryListener {
  global: registry_global,
  global_remove: registry_global_remove,
};

/// Binds the globals with a roundtrip on a private queue (GTK's queue never sees the registry),
/// then moves them to GTK's default queue, which the objects made from them inherit.
unsafe fn bind_globals(wl: &Wayland, display: *mut c_void) -> Option<Globals> {
  let queue = (wl.create_queue)(display);
  if queue.is_null() {
    return None;
  }
  let wrapper = (wl.create_wrapper)(display);
  if wrapper.is_null() {
    (wl.queue_destroy)(queue);
    return None;
  }
  (wl.set_queue)(wrapper, queue);
  // wl_display.get_registry
  let registry = (wl.marshal_constructor)(wrapper, 1, wl.registry, ptr::null_mut::<c_void>());
  (wl.wrapper_destroy)(wrapper);
  if registry.is_null() {
    (wl.queue_destroy)(queue);
    return None;
  }
  let mut found = Found::default();
  (wl.add_listener)(
    registry,
    &REGISTRY_LISTENER as *const RegistryListener as *const c_void,
    &mut found as *mut Found as *mut c_void,
  );
  let ok = (wl.roundtrip_queue)(display, queue) >= 0;
  let bind = |name: u32, interface: *const Interface| {
    // wl_registry.bind, version 1
    let proxy = (wl.marshal_constructor_versioned)(
      registry,
      0,
      interface,
      1,
      name,
      (*interface).name,
      1u32,
      ptr::null_mut::<c_void>(),
    );
    if !proxy.is_null() {
      (wl.set_queue)(proxy, ptr::null_mut());
    }
    proxy
  };
  let globals = match (ok, found.constraints, found.relative_manager) {
    (true, Some(constraints), Some(relative_manager)) => {
      let constraints = bind(constraints, wl.constraints);
      let relative_manager = bind(relative_manager, wl.relative_manager);
      if constraints.is_null() || relative_manager.is_null() {
        // One without the other is no use: destroy whichever bound.
        for proxy in [constraints, relative_manager] {
          if !proxy.is_null() {
            (wl.marshal)(proxy, 0);
            (wl.destroy)(proxy);
          }
        }
        None
      } else {
        Some(Globals {
          constraints,
          relative_manager,
        })
      }
    }
    _ => None,
  };
  (wl.destroy)(registry);
  (wl.queue_destroy)(queue);
  (wl.flush)(display);
  match globals {
    Some(_) => log::info!(
      "canvas: pointer lock on Wayland: zwp_pointer_constraints_v1 and zwp_relative_pointer_manager_v1 bound"
    ),
    None => log::warn!(
      "canvas: the Wayland compositor lacks {}{}{}: pointer lock is unavailable",
      if found.constraints.is_none() { "zwp_pointer_constraints_v1" } else { "" },
      if found.constraints.is_none() && found.relative_manager.is_none() { " and " } else { "" },
      if found.relative_manager.is_none() { "zwp_relative_pointer_manager_v1" } else { "" },
    ),
  }
  globals
}

fn globals(wl: &Wayland, display: *mut c_void) -> Option<Globals> {
  GLOBALS.with(|globals| {
    *globals
      .borrow_mut()
      .entry(display as usize)
      .or_insert_with(|| unsafe { bind_globals(wl, display) })
  })
}

/// Listener data: the view, held weakly so the lock never keeps it alive.
struct Target {
  view: glib::WeakRef<CanvasView>,
}

/// A pointer lock on a view's window, released when dropped.
pub struct Lock {
  display: *mut c_void,
  surface: *mut c_void,
  locked_pointer: *mut Proxy,
  relative_pointer: *mut Proxy,
  target: *mut Target,
}

unsafe fn emit_state(data: *mut c_void, locked: i32) {
  // Copied out first: a handler may release the lock, and the target with it.
  let view = (*(data as *const Target)).view.upgrade();
  if let Some(view) = view {
    view.emit_by_name::<()>("pointer-lock-state", &[&locked]);
  }
}

unsafe extern "C" fn on_locked(data: *mut c_void, _: *mut Proxy) {
  emit_state(data, 1);
}

unsafe extern "C" fn on_unlocked(data: *mut c_void, _: *mut Proxy) {
  emit_state(data, 0);
}

unsafe extern "C" fn on_relative_motion(
  data: *mut c_void,
  _: *mut Proxy,
  _utime_hi: u32,
  _utime_lo: u32,
  dx: i32,
  dy: i32,
  dx_unaccel: i32,
  dy_unaccel: i32,
) {
  let view = (*(data as *const Target)).view.upgrade();
  if let Some(view) = view {
    // wl_fixed_t: 24.8 fixed point, in surface-local (logical) pixels.
    let fixed = |v: i32| v as f64 / 256.;
    view.emit_by_name::<()>(
      "pointer-lock-motion",
      &[&fixed(dx), &fixed(dy), &fixed(dx_unaccel), &fixed(dy_unaccel)],
    );
  }
}

static LOCKED_POINTER_LISTENER: [unsafe extern "C" fn(*mut c_void, *mut Proxy); 2] = [on_locked, on_unlocked];

static RELATIVE_POINTER_LISTENER: [unsafe extern "C" fn(
  *mut c_void,
  *mut Proxy,
  u32,
  u32,
  i32,
  i32,
  i32,
  i32,
); 1] = [on_relative_motion];

pub enum LockResult {
  /// Requested: `pointer-lock-state` 1 follows once the compositor locks the pointer (at once if
  /// it is over the window), 0 when it unlocks it.
  Locked(Lock),
  /// The display is not Wayland.
  NotWayland,
  /// Wayland, but the compositor lacks the protocols or the view has no window.
  Unavailable,
}

/// Locks the pointer in place over `view`'s window until the lock is dropped or the compositor
/// unlocks it (the window lost focus). One-shot, as in browsers.
pub fn lock(view: &CanvasView) -> LockResult {
  let Some(native) = view.native() else {
    return LockResult::Unavailable;
  };
  let Some(surface) = native.surface() else {
    return LockResult::Unavailable;
  };
  let display = surface.display();
  if display.type_().name() != "GdkWaylandDisplay" {
    return LockResult::NotWayland;
  }
  let Some(wl) = wayland() else {
    log::warn!("canvas: libwayland-client or GDK's Wayland calls not found: pointer lock is unavailable");
    return LockResult::Unavailable;
  };
  let Some(device) = display.default_seat().and_then(|seat| seat.pointer()) else {
    return LockResult::Unavailable;
  };
  unsafe {
    let wl_display = (wl.gdk_display)(display.as_ptr() as *mut c_void);
    let wl_surface = (wl.gdk_surface)(surface.as_ptr() as *mut c_void);
    let wl_pointer = (wl.gdk_pointer)(device.as_ptr() as *mut c_void);
    if wl_display.is_null() || wl_surface.is_null() || wl_pointer.is_null() {
      return LockResult::Unavailable;
    }
    let Some(globals) = globals(wl, wl_display) else {
      return LockResult::Unavailable;
    };
    if LOCKED_SURFACES.with(|surfaces| surfaces.borrow().contains(&(wl_surface as usize))) {
      log::warn!("canvas: the window already has a pointer lock");
      return LockResult::Unavailable;
    }
    // lock_pointer(id, surface, pointer, region: whole surface, lifetime: oneshot)
    let locked_pointer = (wl.marshal_constructor)(
      globals.constraints,
      1,
      wl.locked_pointer,
      ptr::null_mut::<c_void>(),
      wl_surface,
      wl_pointer,
      ptr::null_mut::<c_void>(),
      1u32,
    );
    if locked_pointer.is_null() {
      return LockResult::Unavailable;
    }
    // get_relative_pointer(id, pointer)
    let relative_pointer = (wl.marshal_constructor)(
      globals.relative_manager,
      1,
      wl.relative_pointer,
      ptr::null_mut::<c_void>(),
      wl_pointer,
    );
    if relative_pointer.is_null() {
      (wl.marshal)(locked_pointer, 0);
      (wl.destroy)(locked_pointer);
      (wl.flush)(wl_display);
      return LockResult::Unavailable;
    }
    let target = Box::into_raw(Box::new(Target { view: view.downgrade() }));
    (wl.add_listener)(locked_pointer, LOCKED_POINTER_LISTENER.as_ptr() as *const c_void, target as *mut c_void);
    (wl.add_listener)(relative_pointer, RELATIVE_POINTER_LISTENER.as_ptr() as *const c_void, target as *mut c_void);
    (wl.flush)(wl_display);
    LOCKED_SURFACES.with(|surfaces| surfaces.borrow_mut().insert(wl_surface as usize));
    LockResult::Locked(Lock {
      display: wl_display,
      surface: wl_surface,
      locked_pointer,
      relative_pointer,
      target,
    })
  }
}

impl Drop for Lock {
  fn drop(&mut self) {
    let Some(wl) = wayland() else {
      return;
    };
    unsafe {
      // destroy; libwayland drops events still queued for destroyed proxies.
      (wl.marshal)(self.locked_pointer, 0);
      (wl.destroy)(self.locked_pointer);
      (wl.marshal)(self.relative_pointer, 0);
      (wl.destroy)(self.relative_pointer);
      (wl.flush)(self.display);
      drop(Box::from_raw(self.target));
    }
    let _ = LOCKED_SURFACES.try_with(|surfaces| surfaces.borrow_mut().remove(&(self.surface as usize)));
  }
}
