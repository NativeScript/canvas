// A muted MediaPlayer on the WebGPU demo's clip; frames are copied on WARP where there is no GPU.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import url from 'node:url';

const here = path.dirname(url.fileURLToPath(import.meta.url));
const root = path.resolve(here, '../../..');
const file = 'canvas_media_napi.dll';
const addon = [process.env.CANVAS_MEDIA_NAPI_ADDON, path.join(root, 'target', 'debug', file), path.join(root, 'target', 'release-napi', file)].filter(Boolean).find((f) => fs.existsSync(f));
const module = { exports: {} };
process.dlopen(module, addon);
const Media = module.exports;
const clip = url.pathToFileURL(path.join(root, 'tools', 'demo', 'canvas', 'assets', 'webgpu', 'pano.mp4')).href;

/** Resolves with the events seen once `done(events)` is true; rejects on `error` or timeout. */
function watch(frames, done, timeoutMs = 20000) {
	const events = [];
	let bridge;
	const promise = new Promise((resolve, reject) => {
		const timer = setTimeout(() => reject(new Error(`timed out; saw ${events.map((e) => e[0]).join(', ')}`)), timeoutMs);
		bridge = new Media.NSCMediaPlayerBridge(
			Media.__createTestPlayer(clip),
			(type, detail) => {
				events.push([type, detail]);
				if (type === 'error') {
					clearTimeout(timer);
					reject(new Error(detail));
				} else if (done(events, bridge)) {
					clearTimeout(timer);
					resolve(events);
				}
			},
			frames,
		);
	});
	return { promise, bridge: () => bridge };
}

test('exports the bridge index.windows.ts builds on', () => {
	assert.equal(typeof Media.NSCMediaPlayerBridge, 'function');
	assert.throws(() => new Media.NSCMediaPlayerBridge('0x0', () => {}), /Invalid MediaPlayer pointer/);
});

test('delivers the player events on the JS thread', async () => {
	const { promise, bridge } = watch(false, (events) => events.some(([type, state]) => type === 'state' && state === '3'));
	const events = await promise;
	assert.ok(events.some(([type]) => type === 'opened'), 'opened');
	// Without frames nothing is copied.
	assert.equal(bridge().frameId, 0);
	assert.equal(bridge().readPixels(), null);
	bridge().close();
});

test('copies frames and reads them back as RGBA', async () => {
	const { promise, bridge } = watch(true, (events) => events.filter(([type]) => type === 'frame').length >= 3);
	await promise;
	const b = bridge();
	const [width, height] = [b.videoWidth, b.videoHeight];
	assert.ok(width > 0 && height > 0, `${width}x${height}`);
	assert.ok(b.frameId >= 3);
	const pixels = b.readPixels();
	assert.equal(pixels.length, width * height * 4);
	let lit = 0;
	for (let i = 0; i < pixels.length; i += 4) {
		assert.equal(pixels[i + 3], 255);
		if (pixels[i] + pixels[i + 1] + pixels[i + 2] > 0) lit++;
	}
	assert.ok(lit > (width * height) / 10, 'the frame has content');
	// Shared for WebGPU: a descriptor naming the latest frame, on this device's adapter.
	assert.equal(b.sharesFrames, true);
	assert.ok(b.adapterLuid > 0, `adapter ${b.adapterLuid}`);
	const shared = b.gpuFrame();
	assert.ok(shared.address > 0);
	assert.equal(shared.width, width);
	assert.equal(shared.height, height);
	shared.close();
	shared.close();
	// No SurfaceImageSource attached.
	assert.equal(b.present(), false);
	b.close();
	const id = b.frameId;
	await new Promise((resolve) => setTimeout(resolve, 300));
	assert.equal(b.frameId, id, 'no frames after close');
	assert.equal(b.readPixels().length, width * height * 4, 'the last frame stays readable');
});
