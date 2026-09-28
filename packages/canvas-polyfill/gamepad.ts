let gamepadModule: { getGamepads(): unknown[] } | null | undefined;

function dispatch(type: string, gamepad: unknown) {
	const g = global as any;
	// The iOS runtime has its own global EventTarget.
	if (typeof g.dispatchEvent === 'function' && typeof g.Event === 'function') {
		const event = new g.Event(type);
		event.gamepad = gamepad;
		g.dispatchEvent(event);
	}
	const emitter = g.emitter;
	emitter?.notify?.({ eventName: type, object: emitter, type, gamepad });
}

export function gamepads() {
	if (gamepadModule === undefined) {
		gamepadModule = null;
		try {
			// @ts-ignore
			const module = require('@nativescript/canvas-gamepad');
			module.gamepads.addListener((event) => dispatch(event.type, event.gamepad));
			gamepadModule = module;
		} catch (_e) {}
	}
	return gamepadModule;
}

export function hookGamepadListeners() {
	for (const host of new Set([global as any, (global as any).window])) {
		const add = host?.addEventListener;
		if (typeof add !== 'function' || add.__gamepadHook) {
			continue;
		}
		const hooked = function (this: unknown, type: string, ...rest: unknown[]) {
			if (type === 'gamepadconnected' || type === 'gamepaddisconnected') {
				gamepads();
			}
			return add.call(this, type, ...rest);
		};
		hooked.__gamepadHook = true;
		host.addEventListener = hooked;
	}
}
