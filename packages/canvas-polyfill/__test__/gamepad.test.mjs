import { test } from 'node:test';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { build } from 'esbuild';

// Like the native backends, the fake reports already-connected pads synchronously when monitoring starts.
const fakeGamepad = `
const pad = { index: 0, id: 'Fake (STANDARD GAMEPAD)', connected: true };
let started = false;
export const gamepads = {
	addListener(listener) {
		if (!started) {
			started = true;
			listener({ type: 'gamepadconnected', gamepad: pad });
		}
	},
};
export function getGamepads() {
	return started ? [pad, null, null, null] : [null, null, null, null];
}
`;

async function loadPolyfill() {
	const entry = fileURLToPath(new URL('../gamepad.ts', import.meta.url));
	const { outputFiles } = await build({
		entryPoints: [entry],
		bundle: true,
		format: 'esm',
		write: false,
		platform: 'neutral',
		plugins: [
			{
				name: 'fake-canvas-gamepad',
				setup(b) {
					b.onResolve({ filter: /^@nativescript\/canvas-gamepad$/ }, () => ({ path: 'fake', namespace: 'fake' }));
					b.onLoad({ filter: /.*/, namespace: 'fake' }, () => ({ contents: fakeGamepad, loader: 'js' }));
				},
			},
		],
	});
	return import('data:text/javascript;base64,' + Buffer.from(outputFiles[0].text).toString('base64') + '#' + Math.random());
}

test('first gamepadconnected listener sees pads that were already connected', async () => {
	const target = new EventTarget();
	globalThis.addEventListener = target.addEventListener.bind(target);
	globalThis.removeEventListener = target.removeEventListener.bind(target);
	globalThis.dispatchEvent = target.dispatchEvent.bind(target);
	globalThis.window = globalThis;

	const { hookGamepadListeners } = await loadPolyfill();
	hookGamepadListeners();

	const seen = [];
	window.addEventListener('gamepadconnected', (e) => seen.push(e.gamepad.id));

	assert.deepEqual(seen, ['Fake (STANDARD GAMEPAD)']);
});
