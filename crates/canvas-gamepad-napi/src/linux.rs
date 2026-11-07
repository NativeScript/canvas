//! Linux: pads read through gilrs (evdev, hot-plug via udev) and mapped to the standard layout
//! with SDL's controller database. Connections are watched on the GLib main loop between polls.

use std::cell::RefCell;
use std::rc::Rc;

use gilrs::{Axis, Button, EventType, GamepadId, Gilrs};
use napi::bindgen_prelude::{Float32Array, Function, ObjectFinalize};
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::{Env, Error, Result};
use napi_derive::napi;

use crate::layout::*;

// With the guide (home) button.
const STANDARD_BUTTONS: f32 = 17.0;
const WATCH_INTERVAL_MS: u64 = 250;

const DIGITAL: [(usize, Button); 15] = [
  (0, Button::South),
  (1, Button::East),
  (2, Button::West),
  (3, Button::North),
  (4, Button::LeftTrigger),
  (5, Button::RightTrigger),
  (8, Button::Select),
  (9, Button::Start),
  (10, Button::LeftThumb),
  (11, Button::RightThumb),
  (12, Button::DPadUp),
  (13, Button::DPadDown),
  (14, Button::DPadLeft),
  (15, Button::DPadRight),
  (16, Button::Mode),
];

struct State {
  gilrs: Gilrs,
  slots: [Option<GamepadId>; MAX_GAMEPADS],
  ids: [String; MAX_GAMEPADS],
  /// A pad came or went since the last poll.
  dirty: bool,
}

impl State {
  /// Reads pending events (gilrs updates pad state as they are read); `true` on a connection change.
  fn pump(&mut self) -> bool {
    let mut changed = false;
    while let Some(event) = self.gilrs.next_event() {
      if matches!(event.event, EventType::Connected | EventType::Disconnected) {
        changed = true;
      }
    }
    self.dirty |= changed;
    changed
  }

  fn sync(&mut self, buffer: &mut [f32]) -> u32 {
    let mut changed = 0u32;
    for index in 0..MAX_GAMEPADS {
      let gone = self.slots[index].is_some_and(|id| !self.gilrs.connected_gamepad(id).is_some());
      if gone {
        self.slots[index] = None;
        self.ids[index].clear();
        buffer[index * SLOT_STRIDE..(index + 1) * SLOT_STRIDE].fill(0.0);
        changed |= 1 << index;
      }
    }
    let connected: Vec<GamepadId> = self.gilrs.gamepads().map(|(id, _)| id).collect();
    for id in connected {
      if self.slots.contains(&Some(id)) {
        continue;
      }
      let Some(index) = self.slots.iter().position(Option::is_none) else {
        break;
      };
      let slot = &mut buffer[index * SLOT_STRIDE..(index + 1) * SLOT_STRIDE];
      slot.fill(0.0);
      slot[SLOT_CONNECTED] = 1.0;
      slot[SLOT_MAPPING] = MAPPING_STANDARD;
      slot[SLOT_AXIS_COUNT] = STANDARD_AXES;
      slot[SLOT_BUTTON_COUNT] = STANDARD_BUTTONS;
      let pad = self.gilrs.gamepad(id);
      let vendor = pad.vendor_id().unwrap_or(0);
      let product = pad.product_id().unwrap_or(0);
      // As Chrome reports a standard-mapped pad.
      self.ids[index] = format!(
        "{} (STANDARD GAMEPAD Vendor: {vendor:04x} Product: {product:04x})",
        pad.name()
      );
      self.slots[index] = Some(id);
      changed |= 1 << index;
    }
    changed
  }
}

type ChangeCallback<'a> = Function<'a, (), UnknownReturnValue>;

#[napi(js_name = "NSCGamepadPoller", custom_finalize)]
pub struct NSCGamepadPoller {
  state: Rc<RefCell<State>>,
  watch: Option<glib::SourceId>,
}

impl ObjectFinalize for NSCGamepadPoller {
  fn finalize(mut self, _: Env) -> Result<()> {
    self.close();
    Ok(())
  }
}

#[napi]
impl NSCGamepadPoller {
  /// `onConnectionChanged()` runs when a pad comes or goes; `poll` then reports which.
  #[napi(constructor, ts_args_type = "onConnectionChanged: () => void")]
  pub fn new(on_change: ChangeCallback) -> Result<Self> {
    let gilrs = match Gilrs::new() {
      Ok(gilrs) => gilrs,
      // No udev: an instance that never reports a pad.
      Err(gilrs::Error::NotImplemented(dummy)) => dummy,
      Err(e) => return Err(Error::from_reason(format!("gamepads: {e}"))),
    };
    let state = Rc::new(RefCell::new(State {
      gilrs,
      slots: Default::default(),
      ids: Default::default(),
      dirty: true,
    }));
    let tsfn = on_change
      .build_threadsafe_function::<()>()
      .weak::<true>()
      .build()?;
    let watched = Rc::clone(&state);
    let watch = glib::timeout_add_local(
      std::time::Duration::from_millis(WATCH_INTERVAL_MS),
      move || {
        if watched.borrow_mut().pump() {
          tsfn.call((), ThreadsafeFunctionCallMode::NonBlocking);
        }
        glib::ControlFlow::Continue
      },
    );
    Ok(Self {
      state,
      watch: Some(watch),
    })
  }

  /// Writes every pad's reading into `buffer`; returns a bit per slot whose connection changed.
  #[napi]
  pub fn poll(&mut self, mut buffer: Float32Array) -> u32 {
    // SAFETY: the JS caller owns the array and is blocked for the duration of the call.
    let buffer: &mut [f32] = unsafe { buffer.as_mut() };
    if buffer.len() < MAX_GAMEPADS * SLOT_STRIDE {
      return 0;
    }
    let mut state = self.state.borrow_mut();
    state.pump();
    let changed = if std::mem::take(&mut state.dirty) {
      state.sync(buffer)
    } else {
      0
    };

    for index in 0..MAX_GAMEPADS {
      let Some(id) = state.slots[index] else {
        continue;
      };
      let Some(pad) = state.gilrs.connected_gamepad(id) else {
        continue;
      };
      let slot = &mut buffer[index * SLOT_STRIDE..(index + 1) * SLOT_STRIDE];
      let axes = &mut slot[SLOT_AXES..SLOT_AXES + 4];
      // gilrs's Y axes point up; the web's down.
      axes[0] = pad.value(Axis::LeftStickX);
      axes[1] = -pad.value(Axis::LeftStickY);
      axes[2] = pad.value(Axis::RightStickX);
      axes[3] = -pad.value(Axis::RightStickY);
      for (button, which) in DIGITAL {
        let pressed = pad.is_pressed(which);
        write_button(slot, button, if pressed { 1.0 } else { 0.0 }, pressed);
      }
      for (button, which) in [(6, Button::LeftTrigger2), (7, Button::RightTrigger2)] {
        let value = pad.button_data(which).map_or(0.0, |data| data.value());
        write_button(slot, button, value, value > TRIGGER_PRESSED);
      }
      slot[SLOT_SEQUENCE] = (slot[SLOT_SEQUENCE] + 1.0) % SEQUENCE_WRAP;
    }
    changed
  }

  #[napi]
  pub fn id(&self, index: u32) -> String {
    self
      .state
      .borrow()
      .ids
      .get(index as usize)
      .cloned()
      .unwrap_or_default()
  }

  #[napi]
  pub fn close(&mut self) {
    if let Some(watch) = self.watch.take() {
      watch.remove();
    }
    self.state.borrow_mut().slots = Default::default();
  }
}
