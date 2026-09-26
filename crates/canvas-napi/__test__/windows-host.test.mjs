// CanvasModule.NSCCanvas on Windows without a SwapChainPanel: a D3D12 2D context, offscreen.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import url from 'node:url';

import { CanvasModule } from './addon.mjs';

const skip = process.platform !== 'win32' && 'Windows host only';
const { NSCCanvas, create2DContextWithPointer, CanvasRenderingContext2D } = CanvasModule;

function hostContext(width, height) {
	const host = new NSCCanvas();
	host.setSurfaceSize(width, height);
	// The iOS view's argument list; fontColor 0, willReadFrequently false, sRGB.
	const pointer = host.create2DContext(true, true, false, false, 0, true, false, false, false, false, 0, false, 0);
	return { host, ctx: create2DContextWithPointer(BigInt(pointer)) };
}

function pixel(ctx, x, y) {
	return Array.from(ctx.getImageData(x, y, 1, 1).data);
}

function drawAndCheck() {
	const { host, ctx } = hostContext(128, 64);
	ctx.fillStyle = '#3366ff';
	ctx.fillRect(0, 0, 64, 64);
	ctx.fillStyle = 'rgba(255,0,0,0.5)';
	ctx.beginPath();
	ctx.arc(96, 32, 24, 0, Math.PI * 2);
	ctx.fill();
	assert.deepEqual(pixel(ctx, 10, 10), [51, 102, 255, 255]);
	assert.deepEqual(pixel(ctx, 96, 32), [255, 0, 0, 127]);
	assert.deepEqual(pixel(ctx, 127, 0), [0, 0, 0, 0]);
	assert.match(ctx.__toDataURL('image/png'), /^data:image\/png;base64,iVBOR/);
	// No panel: presenting is a flush.
	host.present();
	return { host, ctx };
}

test('NSCCanvas: draws and reads back offscreen', { skip }, () => {
	drawAndCheck();
});

test('NSCCanvas: create2DContext returns the same context every time', { skip }, () => {
	const { host } = hostContext(8, 8);
	const args = [true, true, false, false, 0, true, false, false, false, false, 0, false, 0];
	assert.equal(host.create2DContext(...args), host.create2DContext(...args));
});

test('NSCCanvas: setSurfaceSize resizes and clears the context', { skip }, () => {
	const { host, ctx } = drawAndCheck();
	host.setSurfaceSize(32, 16);
	assert.equal(host.surfaceWidth, 32);
	assert.equal(host.surfaceHeight, 16);
	assert.equal(host.drawingBufferWidth, 32);
	assert.equal(host.drawingBufferHeight, 16);
	assert.deepEqual(pixel(ctx, 1, 1), [0, 0, 0, 0]);
	ctx.fillStyle = '#00ff00';
	ctx.fillRect(0, 0, 32, 16);
	assert.deepEqual(pixel(ctx, 31, 15), [0, 255, 0, 255]);
});

test('NSCCanvas: drawImage into a CPU canvas reads the GPU canvas back', { skip }, () => {
	const { ctx } = drawAndCheck();
	const cpu = CanvasRenderingContext2D.withCpu(128, 64, 1, true, 0, 96, 0);
	cpu.drawImage(ctx, 0, 0);
	assert.deepEqual(pixel(cpu, 10, 10), [51, 102, 255, 255]);
});

test('NSCCanvas: 300x150 until sized, like the web; surfaceWidth/Height set one axis', { skip }, () => {
	const host = new NSCCanvas();
	assert.equal(host.surfaceWidth, 300);
	assert.equal(host.surfaceHeight, 150);
	host.surfaceWidth = 64;
	assert.deepEqual([host.surfaceWidth, host.surfaceHeight], [64, 150]);
	host.surfaceHeight = 32;
	assert.deepEqual([host.surfaceWidth, host.surfaceHeight], [64, 32]);
	host.setSurfaceSize(0, NaN);
	assert.deepEqual([host.surfaceWidth, host.surfaceHeight], [1, 1]);
});

test('NSCCanvas: fit is an int CanvasFit; unknown values are ignored', { skip }, () => {
	const { host } = hostContext(16, 16);
	assert.equal(host.fit, 2);
	for (const fit of [0, 1, 3, 4]) {
		host.fit = fit;
		assert.equal(host.fit, fit);
	}
	host.fit = 9;
	assert.equal(host.fit, 4);
	// Offscreen there is no swapchain to transform: layout calls are accepted and ignored.
	host.setViewSize(100, 50);
	host.setCompositionScale(1.5, 1.5);
});

test('drawing schedules its own flush (no __flushAll needed)', { skip }, async () => {
	const { ctx } = hostContext(8, 8);
	ctx.fillRect(0, 0, 8, 8);
	await Promise.resolve();
	ctx.fillRect(0, 0, 4, 4);
	await new Promise((resolve) => setImmediate(resolve));
	assert.deepEqual(pixel(ctx, 1, 1), [0, 0, 0, 255]);
});

test('NSCCanvas: rejects a malformed panel pointer', { skip }, () => {
	assert.throws(() => new NSCCanvas('not a pointer'), /Invalid SwapChainPanel pointer/);
	assert.throws(() => new NSCCanvas('0x0'), /Invalid SwapChainPanel pointer/);
});

// The device is created once per thread, so WARP (CI machines without a GPU) needs its own process.
test('NSCCanvas: renders on WARP', { skip: skip || (process.env.CANVAS_FORCE_WARP && 'already on WARP') }, () => {
	const file = url.fileURLToPath(import.meta.url);
	const result = spawnSync(process.execPath, ['--test', '--test-name-pattern=offscreen', file], {
		env: { ...process.env, CANVAS_FORCE_WARP: '1' },
		encoding: 'utf8',
	});
	assert.equal(result.status, 0, result.stdout + result.stderr);
});
