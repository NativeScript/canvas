//! The shared buffer's layout. Keep in sync with packages/canvas-gamepad/common.ts.

pub const MAX_GAMEPADS: usize = 4;
pub const MAX_AXES: usize = 8;
pub const SLOT_CONNECTED: usize = 0;
pub const SLOT_SEQUENCE: usize = 1;
pub const SLOT_MAPPING: usize = 2;
pub const SLOT_AXIS_COUNT: usize = 3;
pub const SLOT_BUTTON_COUNT: usize = 4;
pub const SLOT_AXES: usize = 5;
pub const SLOT_BUTTONS: usize = SLOT_AXES + MAX_AXES;
pub const SLOT_STRIDE: usize = 80;
pub const BUTTON_PRESSED: u32 = 1;
pub const BUTTON_TOUCHED: u32 = 2;
pub const MAPPING_STANDARD: f32 = 1.0;
pub const STANDARD_AXES: f32 = 4.0;
// Float32 holds integers exactly up to 2^24.
pub const SEQUENCE_WRAP: f32 = 16_777_216.0;
pub const TRIGGER_PRESSED: f32 = 0.1;

pub fn write_button(slot: &mut [f32], index: usize, value: f32, pressed: bool) {
  let offset = SLOT_BUTTONS + index * 2;
  let touched = pressed || value > 0.0;
  slot[offset] = value;
  slot[offset + 1] = ((if pressed { BUTTON_PRESSED } else { 0 })
    | (if touched { BUTTON_TOUCHED } else { 0 })) as f32;
}
