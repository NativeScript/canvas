import { test } from 'node:test';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { build } from 'esbuild';

const entry = fileURLToPath(new URL('../common.ts', import.meta.url));
const { outputFiles } = await build({ entryPoints: [entry], bundle: true, format: 'esm', write: false, platform: 'neutral' });
const common = await import('data:text/javascript;base64,' + Buffer.from(outputFiles[0].text).toString('base64'));
const { Gamepads, SLOT_STRIDE, SLOT_CONNECTED, SLOT_SEQUENCE, SLOT_MAPPING, SLOT_AXIS_COUNT, SLOT_BUTTON_COUNT, SLOT_AXES, SLOT_BUTTONS, BUTTON_PRESSED, BUTTON_TOUCHED, BUFFER_LENGTH } = common;

function fakeBackend() {
	const backend = {
		buffer: null,
		notify: null,
		polls: 0,
		start(buffer, onConnection) {
			backend.buffer = buffer;
			backend.notify = onConnection;
		},
		connect(index, id) {
			const base = index * SLOT_STRIDE;
			backend.buffer[base + SLOT_CONNECTED] = 1;
			backend.buffer[base + SLOT_MAPPING] = 1;
			backend.buffer[base + SLOT_AXIS_COUNT] = 4;
			backend.buffer[base + SLOT_BUTTON_COUNT] = 17;
			backend.notify(index, true, id);
		},
		disconnect(index) {
			backend.buffer.fill(0, index * SLOT_STRIDE, (index + 1) * SLOT_STRIDE);
			backend.notify(index, false, '');
		},
		write(index, axes, buttons) {
			const base = index * SLOT_STRIDE;
			axes.forEach((v, i) => (backend.buffer[base + SLOT_AXES + i] = v));
			for (const [i, value, flags] of buttons) {
				backend.buffer[base + SLOT_BUTTONS + i * 2] = value;
				backend.buffer[base + SLOT_BUTTONS + i * 2 + 1] = flags;
			}
			backend.buffer[base + SLOT_SEQUENCE] += 1;
		},
	};
	return backend;
}

test('layout fits the standard mapping', () => {
	assert.ok(SLOT_BUTTONS + 32 * 2 <= SLOT_STRIDE);
	assert.equal(BUFFER_LENGTH, 4 * SLOT_STRIDE);
});

test('does not start native until used', () => {
	const backend = fakeBackend();
	new Gamepads(backend);
	assert.equal(backend.buffer, null);
});

test('reports four slots, null until connected', () => {
	const backend = fakeBackend();
	const pads = new Gamepads(backend);
	assert.deepEqual(pads.getGamepads(), [null, null, null, null]);
	assert.equal(backend.buffer.length, BUFFER_LENGTH);
});

test('connect, read state, disconnect', () => {
	const backend = fakeBackend();
	const pads = new Gamepads(backend);
	const events = [];
	pads.addListener((e) => events.push([e.type, e.gamepad.index, e.gamepad.id]));

	backend.connect(1, 'Pad (STANDARD GAMEPAD)');
	assert.deepEqual(events, [['gamepadconnected', 1, 'Pad (STANDARD GAMEPAD)']]);

	let pad = pads.getGamepads()[1];
	assert.equal(pad.mapping, 'standard');
	assert.equal(pad.axes.length, 4);
	assert.equal(pad.buttons.length, 17);
	assert.equal(pad.buttons[0].pressed, false);

	backend.write(1, [0.5, -1, 0, 0.25], [
		[0, 1, BUTTON_PRESSED | BUTTON_TOUCHED],
		[7, 0.05, BUTTON_TOUCHED],
	]);
	pad = pads.getGamepads()[1];
	assert.deepEqual(pad.axes, [0.5, -1, 0, 0.25]);
	assert.deepEqual({ ...pad.buttons[0] }, { pressed: true, touched: true, value: 1 });
	assert.equal(pad.buttons[7].pressed, false);
	assert.equal(pad.buttons[7].touched, true);
	assert.ok(Math.abs(pad.buttons[7].value - 0.05) < 1e-6);

	backend.disconnect(1);
	assert.equal(pad.connected, false);
	assert.deepEqual(events.at(-1), ['gamepaddisconnected', 1, 'Pad (STANDARD GAMEPAD)']);
	assert.equal(pads.getGamepads()[1], null);
});

test('timestamp only moves when the sequence changes', async () => {
	const backend = fakeBackend();
	const pads = new Gamepads(backend);
	pads.start();
	backend.connect(0, 'Pad');
	const first = pads.getGamepads()[0].timestamp;
	await new Promise((r) => setTimeout(r, 5));
	assert.equal(pads.getGamepads()[0].timestamp, first);
	backend.write(0, [1], []);
	assert.ok(pads.getGamepads()[0].timestamp > first);
});

test('poll-only backends are polled on every getGamepads', () => {
	const backend = fakeBackend();
	backend.poll = () => backend.polls++;
	const pads = new Gamepads(backend);
	pads.getGamepads();
	pads.getGamepads();
	assert.equal(backend.polls, 2);
});

test('a listener that throws does not stop the others', () => {
	const backend = fakeBackend();
	const pads = new Gamepads(backend);
	let called = false;
	const error = console.error;
	console.error = () => {};
	try {
		pads.addListener(() => {
			throw new Error('boom');
		});
		pads.addListener(() => (called = true));
		backend.connect(0, 'Pad');
	} finally {
		console.error = error;
	}
	assert.ok(called);
});
