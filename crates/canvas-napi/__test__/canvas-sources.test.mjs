// A canvas as an image source for another context (Windows: D3D12 2D, ANGLE WebGL, wgpu):
// createImageBitmap, WebGL texImage2D and WebGPU copyExternalImageToTexture read its pixels, the
// top row of the image first.
import assert from 'node:assert/strict';
import test from 'node:test';

import { CanvasModule } from './addon.mjs';

const skip = process.platform !== 'win32' && 'Windows host only';
const { NSCCanvas, create2DContextWithPointer } = CanvasModule;

const RED = [255, 0, 0, 255];
const BLUE = [0, 0, 255, 255];
const GREEN = [0, 255, 0, 255];
const TEXTURE_2D = 0x0de1;
const RGBA = 0x1908;
const UNSIGNED_BYTE = 0x1401;
const FRAMEBUFFER = 0x8d40;
const COLOR_ATTACHMENT0 = 0x8ce0;

/** A D3D12 2D canvas on a (headless) panel: red top half, blue bottom half. */
function twoTone2D(size = 32) {
	const host = new NSCCanvas(CanvasModule.__createHeadlessPanel());
	host.setSurfaceSize(size, size);
	const ctx = create2DContextWithPointer(BigInt(host.create2DContext(true, true, false, false, 0, true, false, false, false, false, 0, false, 0)));
	ctx.fillStyle = 'red';
	ctx.fillRect(0, 0, size, size / 2);
	ctx.fillStyle = 'blue';
	ctx.fillRect(0, size / 2, size, size / 2);
	return { host, ctx };
}

/** A WebGL canvas: blue top half, green bottom half (GL's y runs up). */
function twoToneWebGL(size = 32) {
	const gl = CanvasModule.createWebGLContext({ version: 1 }, size, size);
	gl.clearColor(0, 1, 0, 1);
	gl.clear(0x4000);
	gl.enable(0x0c11); // SCISSOR_TEST
	gl.scissor(0, size / 2, size, size / 2);
	gl.clearColor(0, 0, 1, 1);
	gl.clear(0x4000);
	gl.disable(0x0c11);
	return gl;
}

/** Texture rows 0 and size-1 after texImage2D(source): row 0 is the image's top row. */
function uploadRows(source, size = 32) {
	const gl = CanvasModule.createWebGLContext({ version: 1 }, size, size);
	const texture = gl.createTexture();
	gl.bindTexture(TEXTURE_2D, texture);
	gl.texImage2D(TEXTURE_2D, 0, RGBA, RGBA, UNSIGNED_BYTE, source);
	assert.equal(gl.getError(), 0);
	gl.bindFramebuffer(FRAMEBUFFER, gl.createFramebuffer());
	gl.framebufferTexture2D(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, texture, 0);
	const first = new Uint8Array(4);
	const last = new Uint8Array(4);
	gl.readPixels(size / 2, 0, 1, 1, RGBA, UNSIGNED_BYTE, first);
	gl.readPixels(size / 2, size - 1, 1, 1, RGBA, UNSIGNED_BYTE, last);
	return [Array.from(first), Array.from(last)];
}

test('createImageBitmap(D3D12 2D context) reads its pixels', { skip }, async () => {
	const { ctx } = twoTone2D(20);
	const bitmap = await new Promise((resolve, reject) => CanvasModule.createImageBitmap(ctx, (error, bitmap) => (error ? reject(new Error(error)) : resolve(bitmap))));
	assert.deepEqual([bitmap.width, bitmap.height], [20, 20]);
	const out = CanvasModule.CanvasRenderingContext2D.withCpu(20, 20, 1, true, 0, 96, 0);
	out.drawImage(bitmap, 0, 0);
	assert.deepEqual(Array.from(out.getImageData(10, 2, 1, 1).data), RED);
	assert.deepEqual(Array.from(out.getImageData(10, 17, 1, 1).data), BLUE);
});

test('texImage2D(2D canvas) uploads it, top row first', { skip }, () => {
	const { ctx } = twoTone2D();
	assert.deepEqual(uploadRows(ctx), [RED, BLUE]);
});

test('texImage2D(WebGL canvas) uploads its drawing buffer, top row first', { skip }, () => {
	const source = twoToneWebGL();
	// A framebuffer bound in the source does not change what is read.
	source.bindFramebuffer(FRAMEBUFFER, source.createFramebuffer());
	assert.deepEqual(uploadRows(source), [BLUE, GREEN]);
});

test('copyExternalImageToTexture from 2D and WebGL canvases, flipY and origin', { skip }, async (t) => {
	const gpu = new CanvasModule.GPU();
	const adapter = await new Promise((resolve, reject) => gpu.requestAdapter({}, (error, adapter) => (error ? reject(error) : resolve(adapter))));
	if (!adapter) {
		t.skip('no WebGPU adapter');
		return;
	}
	const device = await new Promise((resolve, reject) => adapter.requestDevice({}, (error, device) => (error ? reject(error) : resolve(device))));
	const errors = [];
	device.setuncapturederror((type, message) => errors.push(message));

	/** Texels (x, y) of `source` copied into a size x size texture. */
	async function copy(source, size, options = {}, points) {
		const texture = device.createTexture({ size: [size, size, 1], format: 'rgba8unorm', usage: 0x01 | 0x02 | 0x10 });
		device.queue.copyExternalImageToTexture({ source, ...options }, { texture }, [size, size, 1]);
		const readback = device.createBuffer({ size: 256 * size, usage: 0x0008 | 0x0001 });
		const encoder = device.createCommandEncoder();
		encoder.copyTextureToBuffer({ texture }, { buffer: readback, bytesPerRow: 256 }, { width: size, height: size, depthOrArrayLayers: 1 });
		device.queue.submit([encoder.finish()]);
		await readback.mapAsync(1);
		const bytes = new Uint8Array(readback.getMappedRange());
		return points.map(([x, y]) => Array.from(bytes.subarray(y * 256 + x * 4, y * 256 + x * 4 + 4)));
	}

	const { ctx } = twoTone2D(32);
	assert.deepEqual(await copy(ctx, 32, {}, [[16, 0], [16, 31]]), [RED, BLUE]);
	assert.deepEqual(await copy(ctx, 32, { flipY: true }, [[16, 0], [16, 31]]), [BLUE, RED]);
	// A 16x16 window from (0, 16): the blue half.
	assert.deepEqual(await copy(ctx, 16, { origin: { x: 0, y: 16 } }, [[8, 0], [8, 15]]), [BLUE, BLUE]);
	assert.deepEqual(await copy(twoToneWebGL(32), 32, {}, [[16, 0], [16, 31]]), [BLUE, GREEN]);
	assert.deepEqual(errors, []);
});
