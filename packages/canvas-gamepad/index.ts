import { Gamepads } from './common';
import { createBackend } from './backend';

export { Gamepad, GamepadButton, GamepadEvent, Gamepads, MAX_GAMEPADS } from './common';
export type { GamepadBackend, GamepadListener, GamepadMappingType } from './common';

export const gamepads = new Gamepads(createBackend());

export function getGamepads() {
	return gamepads.getGamepads();
}
