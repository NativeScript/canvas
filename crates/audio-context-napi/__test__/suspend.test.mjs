// OfflineAudioContext suspend(when)/resume(): the render thread holds at a quantum.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import url from 'node:url';

const here = path.dirname(url.fileURLToPath(import.meta.url));
const root = path.resolve(here, '../../..');
const file = { win32: 'audio_context_napi.dll', darwin: 'libaudio_context_napi.dylib', linux: 'libaudio_context_napi.so' }[process.platform];
const addon = [process.env.AUDIO_CONTEXT_NAPI_ADDON, path.join(root, 'target', 'debug', file), path.join(root, 'target', 'release-napi', file)].filter(Boolean).find((f) => fs.existsSync(f));
const module = { exports: {} };
process.dlopen(module, addon);
const Audio = module.exports;

test('changes made while suspended apply from the suspend frame', async () => {
	const context = Audio.AudioContext.offline(1, 128 * 32, 48000);
	const seen = [];
	let source;
	context.setOnsuspend((frame) => {
		seen.push([frame, context.currentTime]);
		if (frame === 128 * 8) {
			source = context.createConstantSource();
			source.connect(context.destination());
			source.start(context.currentTime);
			// Scheduled while held, after rendering started.
			assert.equal(context.suspendAt((128 * 20) / 48000), 128 * 20);
		} else {
			source.stop(context.currentTime);
		}
		context.resumeRendering();
	});
	assert.equal(context.suspendAt((128 * 8) / 48000), 128 * 8);
	const samples = (await context.startRendering()).channelData(0);
	assert.deepEqual(seen, [
		[128 * 8, (128 * 8) / 48000],
		[128 * 20, (128 * 20) / 48000],
	]);
	assert.equal(samples[128 * 8 - 1], 0);
	assert.equal(samples[128 * 8], 1);
	assert.equal(samples[128 * 20 - 1], 1);
	assert.equal(samples[128 * 20], 0);
});

test('rounds up to a quantum and suspends at 0', async () => {
	const context = Audio.AudioContext.offline(1, 128 * 4, 44100);
	assert.equal(context.suspendAt(1 / 44100), 128);
	assert.equal(context.suspendAt(0), 0);
	const frames = [];
	context.setOnsuspend((frame) => {
		frames.push(frame);
		setTimeout(() => context.resumeRendering(), 5);
	});
	await context.startRendering();
	assert.deepEqual(frames, [0, 128]);
});

test('rejects invalid suspends and an early resume', () => {
	const context = Audio.AudioContext.offline(1, 128 * 4, 48000);
	assert.throws(() => context.resumeRendering(), /InvalidStateError: rendering has not started/);
	assert.throws(() => context.suspendAt(-1), /InvalidStateError/);
	assert.throws(() => context.suspendAt((128 * 4) / 48000), /InvalidStateError: suspendTime is at or past the end/);
	context.suspendAt(128 / 48000);
	assert.throws(() => context.suspendAt(100 / 48000), /InvalidStateError: a suspend is already scheduled/);
});

test('a suspend at or before the held frame is rejected', async () => {
	const context = Audio.AudioContext.offline(1, 128 * 8, 48000);
	let error;
	context.setOnsuspend(() => {
		try {
			context.suspendAt((128 * 2) / 48000);
		} catch (e) {
			error = e;
		}
		context.resumeRendering();
	});
	context.suspendAt((128 * 2) / 48000);
	await context.startRendering();
	assert.match(String(error), /InvalidStateError: suspendTime is not after the current time/);
	assert.throws(() => context.suspendAt(0), /closed/);
});
