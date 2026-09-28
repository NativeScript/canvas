import { MAX_GAMEPADS, SLOT_CONNECTED, SLOT_STRIDE, type GamepadBackend } from './common';

declare const __non_webpack_require__: (specifier: string) => any;

interface GamepadPoller {
	poll(buffer: Float32Array): number;
	id(index: number): string;
}

const Native: { NSCGamepadPoller: new (onConnectionChanged: () => void) => GamepadPoller } = __non_webpack_require__('system_lib://canvasgamepad.node');

export function createBackend(): GamepadBackend {
	let poller: GamepadPoller;
	let notify: (index: number, connected: boolean, id: string) => void;
	let target: Float32Array;

	const poll = (buffer: Float32Array) => {
		const changed = poller.poll(buffer);
		for (let i = 0; changed !== 0 && i < MAX_GAMEPADS; i++) {
			if (changed & (1 << i)) {
				const connected = buffer[i * SLOT_STRIDE + SLOT_CONNECTED] === 1;
				notify(i, connected, connected ? poller.id(i) : '');
			}
		}
	};

	return {
		start(buffer, onConnection) {
			target = buffer;
			notify = onConnection;
			poller = new Native.NSCGamepadPoller(() => poll(target));
			poll(buffer);
		},
		poll,
	};
}
