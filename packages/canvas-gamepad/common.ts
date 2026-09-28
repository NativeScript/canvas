export const MAX_GAMEPADS = 4;
export const MAX_AXES = 8;
export const MAX_BUTTONS = 32;

export const SLOT_CONNECTED = 0;
export const SLOT_SEQUENCE = 1;
export const SLOT_MAPPING = 2;
export const SLOT_AXIS_COUNT = 3;
export const SLOT_BUTTON_COUNT = 4;
export const SLOT_AXES = 5;
export const SLOT_BUTTONS = SLOT_AXES + MAX_AXES;
export const SLOT_STRIDE = 80;
export const BUFFER_LENGTH = MAX_GAMEPADS * SLOT_STRIDE;

export const BUTTON_PRESSED = 1;
export const BUTTON_TOUCHED = 2;

export const MAPPING_STANDARD = 1;

export type GamepadMappingType = '' | 'standard' | 'xr-standard';

export interface GamepadBackend {
	start(buffer: Float32Array, onConnection: (index: number, connected: boolean, id: string) => void): void;
	poll?(buffer: Float32Array): void;
}

export class GamepadButton {
	pressed = false;
	touched = false;
	value = 0;
}

export class Gamepad {
	readonly axes: number[] = [];
	readonly buttons: GamepadButton[] = [];
	readonly mapping: GamepadMappingType;
	readonly vibrationActuator = null;
	readonly hapticActuators = [];
	connected = true;
	timestamp = 0;

	constructor(
		readonly index: number,
		readonly id: string,
		buffer: Float32Array,
	) {
		const base = index * SLOT_STRIDE;
		this.mapping = buffer[base + SLOT_MAPPING] === MAPPING_STANDARD ? 'standard' : '';
		const axisCount = Math.min(buffer[base + SLOT_AXIS_COUNT], MAX_AXES);
		const buttonCount = Math.min(buffer[base + SLOT_BUTTON_COUNT], MAX_BUTTONS);
		for (let i = 0; i < axisCount; i++) {
			this.axes.push(0);
		}
		for (let i = 0; i < buttonCount; i++) {
			this.buttons.push(new GamepadButton());
		}
		this._update(buffer, performance.now());
	}

	_update(buffer: Float32Array, timestamp: number) {
		const base = this.index * SLOT_STRIDE;
		const axes = this.axes;
		for (let i = 0, n = axes.length, o = base + SLOT_AXES; i < n; i++) {
			axes[i] = buffer[o + i];
		}
		const buttons = this.buttons;
		for (let i = 0, n = buttons.length, o = base + SLOT_BUTTONS; i < n; i++, o += 2) {
			const button = buttons[i];
			const flags = buffer[o + 1];
			button.value = buffer[o];
			button.pressed = (flags & BUTTON_PRESSED) !== 0;
			button.touched = (flags & BUTTON_TOUCHED) !== 0;
		}
		this.timestamp = timestamp;
	}
}

export class GamepadEvent {
	constructor(
		readonly type: 'gamepadconnected' | 'gamepaddisconnected',
		readonly gamepad: Gamepad,
	) {}
}

export type GamepadListener = (event: GamepadEvent) => void;

export class Gamepads {
	private readonly _buffer = new Float32Array(BUFFER_LENGTH);
	private readonly _pads: (Gamepad | null)[] = new Array(MAX_GAMEPADS).fill(null);
	private readonly _sequences = new Float32Array(MAX_GAMEPADS).fill(-1);
	private readonly _listeners = new Set<GamepadListener>();
	private _started = false;

	constructor(private readonly _backend: GamepadBackend) {}

	start() {
		if (this._started) {
			return;
		}
		this._started = true;
		this._backend.start(this._buffer, (index, connected, id) => this._onConnection(index, connected, id));
	}

	getGamepads(): (Gamepad | null)[] {
		this.start();
		const buffer = this._buffer;
		this._backend.poll?.(buffer);
		const pads = this._pads;
		let now = -1;
		for (let i = 0; i < MAX_GAMEPADS; i++) {
			const pad = pads[i];
			if (pad === null) {
				continue;
			}
			const sequence = buffer[i * SLOT_STRIDE + SLOT_SEQUENCE];
			if (sequence !== this._sequences[i]) {
				this._sequences[i] = sequence;
				if (now < 0) {
					now = performance.now();
				}
				pad._update(buffer, now);
			}
		}
		return pads.slice();
	}

	addListener(listener: GamepadListener) {
		this._listeners.add(listener);
		this.start();
	}

	removeListener(listener: GamepadListener) {
		this._listeners.delete(listener);
	}

	private _onConnection(index: number, connected: boolean, id: string) {
		if (index < 0 || index >= MAX_GAMEPADS) {
			return;
		}
		const previous = this._pads[index];
		if (previous) {
			previous.connected = false;
			this._pads[index] = null;
			this._emit(new GamepadEvent('gamepaddisconnected', previous));
		}
		if (connected) {
			const pad = new Gamepad(index, id, this._buffer);
			this._sequences[index] = this._buffer[index * SLOT_STRIDE + SLOT_SEQUENCE];
			this._pads[index] = pad;
			this._emit(new GamepadEvent('gamepadconnected', pad));
		}
	}

	private _emit(event: GamepadEvent) {
		for (const listener of this._listeners) {
			try {
				listener(event);
			} catch (e) {
				console.error(e);
			}
		}
	}
}
