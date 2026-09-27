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
	const { host, ctx } = hostContext(8, 8);
	ctx.fillRect(0, 0, 8, 8);
	await Promise.resolve();
	ctx.fillRect(0, 0, 4, 4);
	await new Promise((resolve) => setImmediate(resolve));
	assert.deepEqual(pixel(ctx, 1, 1), [0, 0, 0, 255]);
	// The host owns the context: a host collected during the awaits frees it under `ctx`.
	assert.equal(host.surfaceWidth, 8);
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

// WebGL on ANGLE: the view owns the state, packages/canvas wraps its pointer.
const GL_COLOR_BUFFER_BIT = 0x4000;
const GL_RGBA = 0x1908;
const GL_UNSIGNED_BYTE = 0x1401;
// (type, alpha, antialias, depth, failIfMajorPerformanceCaveat, powerPreference, premultipliedAlpha,
//  preserveDrawingBuffer, stencil, desynchronized, xrCompatible, isCanvas, colorSpace), as on iOS.
const glArgs = (type) => [type, true, true, true, false, 0, true, false, false, false, false, false, 0];

function hostWebGL(type, width, height) {
	const host = new NSCCanvas();
	host.setSurfaceSize(width, height);
	host.initContext(...glArgs(type));
	const version = type === 'webgl2' ? 2 : 1;
	const create = version === 2 ? CanvasModule.createWebGL2Context : CanvasModule.createWebGLContext;
	const gl = create({ version }, BigInt(host.nativeContext), 1, -16777216, 160, 0);
	return { host, gl };
}

function glPixel(gl, x, y) {
	const out = new Uint8Array(4);
	gl.readPixels(x, y, 1, 1, GL_RGBA, GL_UNSIGNED_BYTE, out);
	return Array.from(out);
}

for (const type of ['webgl', 'webgl2']) {
	test(`NSCCanvas: ${type} clears, reads back and presents`, { skip }, () => {
		const { host, gl } = hostWebGL(type, 64, 32);
		assert.notEqual(host.nativeContext, '0');
		assert.equal(gl.drawingBufferWidth, 64);
		assert.equal(gl.drawingBufferHeight, 32);
		gl.clearColor(0, 0.5, 1, 1);
		gl.clear(GL_COLOR_BUFFER_BIT);
		const [r, g, b, a] = glPixel(gl, 1, 1);
		assert.deepEqual([r, b, a], [0, 255, 255]);
		assert.ok(g >= 127 && g <= 128, `green ${g}`);
		host.present();
		assert.match(gl.__toDataURL('image/png'), /^data:image\/png;base64,/);
		// Texture-backed on Windows: single-sampled, so antialias is reported as off.
		assert.equal(gl.getContextAttributes().antialias, false);
	});

	test(`NSCCanvas: ${type} resize keeps the context, clears the buffer`, { skip }, () => {
		const { host, gl } = hostWebGL(type, 16, 16);
		gl.clearColor(1, 0, 0, 1);
		gl.clear(GL_COLOR_BUFFER_BIT);
		host.setSurfaceSize(40, 20);
		assert.equal(gl.drawingBufferWidth, 40);
		assert.equal(gl.drawingBufferHeight, 20);
		gl.viewport(0, 0, 40, 20);
		gl.clearColor(0, 1, 0, 1);
		gl.clear(GL_COLOR_BUFFER_BIT);
		assert.deepEqual(glPixel(gl, 39, 19), [0, 255, 0, 255]);
	});
}

test('NSCCanvas: one context kind per view', { skip }, () => {
	const { host } = hostWebGL('webgl', 8, 8);
	assert.throws(() => host.create2DContext(true, true, false, false, 0, true, false, false, false, false, 0, false, 0), /already has a WebGL or WebGPU context/);
});

test('createWebGLContext: options pick the version; width/height create an offscreen context', { skip }, () => {
	assert.equal(CanvasModule.createWebGLContext({ version: 2 }, 16, 16), null);
	const gl = CanvasModule.createWebGLContext({ version: 1, alpha: false }, 16, 8, 1, -16777216, 160, 0);
	assert.equal(gl.drawingBufferWidth, 16);
	assert.equal(gl.getContextAttributes().alpha, false);
	assert.ok(gl.__getSupportedExtensions().length > 0);
	const gl2 = CanvasModule.createWebGL2Context({ version: 2 }, 8, 8, 1, -16777216, 160, 0);
	assert.equal(gl2.drawingBufferHeight, 8);
});
