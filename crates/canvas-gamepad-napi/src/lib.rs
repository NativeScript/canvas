#![cfg(target_os = "windows")]
#![deny(clippy::all)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use napi::bindgen_prelude::{Float32Array, Function, ObjectFinalize};
use napi::threadsafe_function::{ThreadsafeFunctionCallMode, UnknownReturnValue};
use napi::{Env, Result};
use napi_derive::napi;
use windows::Foundation::EventHandler;
use windows::Gaming::Input::{Gamepad, GamepadButtons, RawGameController};

// Keep in sync with packages/canvas-gamepad/common.ts.
const MAX_GAMEPADS: usize = 4;
const MAX_AXES: usize = 8;
const SLOT_CONNECTED: usize = 0;
const SLOT_SEQUENCE: usize = 1;
const SLOT_MAPPING: usize = 2;
const SLOT_AXIS_COUNT: usize = 3;
const SLOT_BUTTON_COUNT: usize = 4;
const SLOT_AXES: usize = 5;
const SLOT_BUTTONS: usize = SLOT_AXES + MAX_AXES;
const SLOT_STRIDE: usize = 80;
const BUTTON_PRESSED: u32 = 1;
const BUTTON_TOUCHED: u32 = 2;
const MAPPING_STANDARD: f32 = 1.0;
const STANDARD_AXES: f32 = 4.0;
// Windows.Gaming.Input.Gamepad has no guide button.
const STANDARD_BUTTONS: f32 = 16.0;
// Float32 holds integers exactly up to 2^24.
const SEQUENCE_WRAP: f32 = 16_777_216.0;
const TRIGGER_PRESSED: f32 = 0.1;

const DIGITAL: [(usize, GamepadButtons); 14] = [
  (0, GamepadButtons::A),
  (1, GamepadButtons::B),
  (2, GamepadButtons::X),
  (3, GamepadButtons::Y),
  (4, GamepadButtons::LeftShoulder),
  (5, GamepadButtons::RightShoulder),
  (8, GamepadButtons::View),
  (9, GamepadButtons::Menu),
  (10, GamepadButtons::LeftThumbstick),
  (11, GamepadButtons::RightThumbstick),
  (12, GamepadButtons::DPadUp),
  (13, GamepadButtons::DPadDown),
  (14, GamepadButtons::DPadLeft),
  (15, GamepadButtons::DPadRight),
];

fn write_button(slot: &mut [f32], index: usize, value: f32, pressed: bool) {
  let offset = SLOT_BUTTONS + index * 2;
  let touched = pressed || value > 0.0;
  slot[offset] = value;
  slot[offset + 1] = ((if pressed { BUTTON_PRESSED } else { 0 }) | (if touched { BUTTON_TOUCHED } else { 0 })) as f32;
}

type ChangeCallback<'a> = Function<'a, (), UnknownReturnValue>;

#[napi(js_name = "NSCGamepadPoller", custom_finalize)]
pub struct NSCGamepadPoller {
  slots: [Option<Gamepad>; MAX_GAMEPADS],
  ids: [String; MAX_GAMEPADS],
  dirty: Arc<AtomicBool>,
  tokens: Option<(i64, i64)>,
}

impl ObjectFinalize for NSCGamepadPoller {
  fn finalize(mut self, _: Env) -> Result<()> {
    self.close();
    Ok(())
  }
}

#[napi]
impl NSCGamepadPoller {
  #[napi(constructor, ts_args_type = "onConnectionChanged: () => void")]
  pub fn new(on_change: ChangeCallback) -> Result<Self> {
    let dirty = Arc::new(AtomicBool::new(true));
    let tsfn = on_change.build_threadsafe_function::<()>().weak::<true>().build()?;
    let tsfn = Arc::new(tsfn);

    let handler = {
      let dirty = Arc::clone(&dirty);
      let tsfn = Arc::clone(&tsfn);
      EventHandler::<Gamepad>::new(move |_, _| {
        dirty.store(true, Ordering::Release);
        tsfn.call((), ThreadsafeFunctionCallMode::NonBlocking);
        Ok(())
      })
    };
    let tokens = match (Gamepad::GamepadAdded(&handler), Gamepad::GamepadRemoved(&handler)) {
      (Ok(added), Ok(removed)) => Some((added, removed)),
      _ => None,
    };

    Ok(Self {
      slots: Default::default(),
      ids: Default::default(),
      dirty,
      tokens,
    })
  }

  #[napi]
  pub fn poll(&mut self, mut buffer: Float32Array) -> u32 {
    // SAFETY: the JS caller owns the array and is blocked for the duration of the call.
    let buffer: &mut [f32] = unsafe { buffer.as_mut() };
    if buffer.len() < MAX_GAMEPADS * SLOT_STRIDE {
      return 0;
    }
    let changed = if self.dirty.swap(false, Ordering::AcqRel) {
      self.sync(buffer)
    } else {
      0
    };

    for (index, pad) in self.slots.iter().enumerate() {
      let Some(pad) = pad else { continue };
      let Ok(reading) = pad.GetCurrentReading() else { continue };
      let slot = &mut buffer[index * SLOT_STRIDE..(index + 1) * SLOT_STRIDE];

      let axes = &mut slot[SLOT_AXES..SLOT_AXES + 4];
      axes[0] = reading.LeftThumbstickX as f32;
      axes[1] = -reading.LeftThumbstickY as f32;
      axes[2] = reading.RightThumbstickX as f32;
      axes[3] = -reading.RightThumbstickY as f32;

      for (button, flag) in DIGITAL {
        let pressed = reading.Buttons.contains(flag);
        write_button(slot, button, if pressed { 1.0 } else { 0.0 }, pressed);
      }
      let left = reading.LeftTrigger as f32;
      let right = reading.RightTrigger as f32;
      write_button(slot, 6, left, left > TRIGGER_PRESSED);
      write_button(slot, 7, right, right > TRIGGER_PRESSED);

      slot[SLOT_SEQUENCE] = (slot[SLOT_SEQUENCE] + 1.0) % SEQUENCE_WRAP;
    }
    changed
  }

  #[napi]
  pub fn id(&self, index: u32) -> String {
    self.ids.get(index as usize).cloned().unwrap_or_default()
  }

  #[napi]
  pub fn close(&mut self) {
    if let Some((added, removed)) = self.tokens.take() {
      let _ = Gamepad::RemoveGamepadAdded(added);
      let _ = Gamepad::RemoveGamepadRemoved(removed);
    }
    self.slots = Default::default();
  }
}

impl NSCGamepadPoller {
  fn sync(&mut self, buffer: &mut [f32]) -> u32 {
    let current: Vec<Gamepad> = Gamepad::Gamepads().map(|list| list.into_iter().collect()).unwrap_or_default();
    let mut changed = 0u32;

    for index in 0..MAX_GAMEPADS {
      let gone = matches!(&self.slots[index], Some(pad) if !current.contains(pad));
      if gone {
        self.slots[index] = None;
        self.ids[index].clear();
        buffer[index * SLOT_STRIDE..(index + 1) * SLOT_STRIDE].fill(0.0);
        changed |= 1 << index;
      }
    }

    for pad in current {
      if self.slots.iter().any(|slot| slot.as_ref() == Some(&pad)) {
        continue;
      }
      let Some(index) = self.slots.iter().position(Option::is_none) else { break };
      let slot = &mut buffer[index * SLOT_STRIDE..(index + 1) * SLOT_STRIDE];
      slot.fill(0.0);
      slot[SLOT_CONNECTED] = 1.0;
      slot[SLOT_MAPPING] = MAPPING_STANDARD;
      slot[SLOT_AXIS_COUNT] = STANDARD_AXES;
      slot[SLOT_BUTTON_COUNT] = STANDARD_BUTTONS;
      self.ids[index] = describe(&pad);
      self.slots[index] = Some(pad);
      changed |= 1 << index;
    }
    changed
  }
}

fn describe(pad: &Gamepad) -> String {
  match RawGameController::FromGameController(pad) {
    Ok(raw) => {
      let name = raw.DisplayName().map(|n| n.to_string()).unwrap_or_default();
      let name = if name.is_empty() { "Xbox Controller".to_string() } else { name };
      let vendor = raw.HardwareVendorId().unwrap_or(0);
      let product = raw.HardwareProductId().unwrap_or(0);
      format!("{name} (STANDARD GAMEPAD Vendor: {vendor:04x} Product: {product:04x})")
    }
    Err(_) => "Xbox Controller (STANDARD GAMEPAD)".to_string(),
  }
}
