import type { GamepadBackend } from './common';

export function createBackend(): GamepadBackend {
	return {
		start(buffer, onConnection) {
			NSCGamepadManager.shared.startWithBufferLengthListener(buffer as never, buffer.length, (index, connected, id) => onConnection(index, connected, id));
		},
	};
}
