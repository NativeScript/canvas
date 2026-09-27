// The raw fast members (src/fast.rs) that replace napi-rs's on the hottest 2D and WebGL calls:
// same results, WebIDL argument coercion, and a foreign receiver still throws.
import assert from 'node:assert/strict';
import test from 'node:test';

import { CanvasModule } from './addon.mjs';

const { CanvasRenderingContext2D, DOMMatrix } = CanvasModule;

function context() {
	return CanvasRenderingContext2D.withCpu(32, 32, 1, true, 0, 96, 0);
}

function pixel(ctx, x, y) {
	return Array.from(ctx.getImageData(x, y, 1, 1).data);
}

test('2D fast members draw and transform like the napi-rs ones', () => {
	const ctx = context();
	ctx.translate(4, 2);
	ctx.scale(2, 1);
	const m = ctx.getTransform();
	assert.deepEqual([m.a, m.d, m.e, m.f], [2, 1, 4, 2]);
	ctx.resetTransform();
	ctx.save();
	ctx.rotate(Math.PI / 2);
	ctx.restore();
	assert.equal(ctx.getTransform().b, 0);

	ctx.fillStyle = '#ff0000';
	ctx.fillRect(0, 0, 8, 8);
	assert.deepEqual(pixel(ctx, 4, 4), [255, 0, 0, 255]);
	ctx.clearRect(0, 0, 4, 4);
	assert.deepEqual(pixel(ctx, 1, 1), [0, 0, 0, 0]);

	ctx.beginPath();
	ctx.moveTo(16, 16);
	ctx.lineTo(31, 16);
	ctx.arc(24, 24, 4, 0, Math.PI * 2);
	ctx.rect(0, 16, 4, 4);
	ctx.closePath();
	ctx.fill();
	assert.deepEqual(pixel(ctx, 2, 18), [255, 0, 0, 255]);
});

test('lineWidth / globalAlpha accessors, with WebIDL coercion', () => {
	const ctx = context();
	ctx.lineWidth = 3;
	assert.equal(ctx.lineWidth, 3);
	ctx.lineWidth = '5';
	assert.equal(ctx.lineWidth, 5);
	ctx.globalAlpha = 0.5;
	assert.equal(ctx.globalAlpha, 0.5);
	// Numbers given as strings are coerced, as the V8 bindings' NumberValue does.
	ctx.translate('2', '3');
	const m = ctx.getTransform();
	assert.deepEqual([m.e, m.f], [2, 3]);
});

test('a foreign receiver throws instead of being reinterpreted', () => {
	const ctx = context();
	assert.throws(() => ctx.fillRect.call(new DOMMatrix(), 0, 0, 1, 1), /Illegal invocation/);
	assert.throws(() => ctx.fillRect.call({}, 0, 0, 1, 1), /Illegal invocation/);
	const descriptor = Object.getOwnPropertyDescriptor(CanvasRenderingContext2D.prototype, 'lineWidth');
	assert.throws(() => descriptor.get.call({}), /Illegal invocation/);
});

test('WebGL fast members, and a null uniform location is a no-op', { skip: process.platform !== 'win32' && 'ANGLE on Windows' }, () => {
	for (const gl of [CanvasModule.createWebGLContext({ version: 1 }, 8, 8), CanvasModule.createWebGL2Context({ version: 2 }, 8, 8)]) {
		gl.viewport(0, 0, 8, 8);
		gl.enable(0x0c11);
		gl.disable(0x0c11);
		gl.clearColor(0, 1, 0, 1);
		gl.clear(0x4000);
		const px = new Uint8Array(4);
		gl.readPixels(4, 4, 1, 1, 0x1908, 0x1401, px);
		assert.deepEqual(Array.from(px), [0, 255, 0, 255]);
		gl.uniform4f(null, 1, 2, 3, 4);
		gl.uniformMatrix4fv(null, false, new Float32Array(16));
		gl.bindBuffer(0x8892, null);
		gl.useProgram(null);
		assert.equal(gl.getError(), 0);
		assert.throws(() => gl.clear.call({}, 0x4000), /Illegal invocation/);
	}
});

test('fillStyle / strokeStyle: colours (cached), invalid ones ignored, gradients and patterns through napi-rs', () => {
	const ctx = context();
	ctx.fillStyle = '#00ff00';
	assert.equal(ctx.fillStyle, '#00ff00');
	ctx.fillRect(0, 0, 4, 4);
	ctx.fillStyle = 'not a colour';
	assert.equal(ctx.fillStyle, '#00ff00');
	for (const colour of ['red', 'blue', 'red', 'blue']) {
		ctx.fillStyle = colour;
		ctx.fillRect(8, 8, 2, 2);
		assert.deepEqual(pixel(ctx, 9, 9), colour === 'red' ? [255, 0, 0, 255] : [0, 0, 255, 255]);
	}
	// A colour cached for fill is independent of stroke.
	ctx.strokeStyle = 'red';
	assert.equal(ctx.strokeStyle, '#ff0000');
	ctx.save();
	ctx.fillStyle = '#123456';
	ctx.restore();
	ctx.fillStyle = '#123456';
	assert.equal(ctx.fillStyle, '#123456');
	// Long strings take the heap path.
	ctx.fillStyle = 'rgba(' + ' '.repeat(80) + '1, 2, 3, 1)';
	const gradient = ctx.createLinearGradient(0, 0, 32, 0);
	gradient.addColorStop(0, 'red');
	gradient.addColorStop(1, 'blue');
	ctx.fillStyle = gradient;
	assert.equal(typeof ctx.fillStyle, 'object');
	ctx.fillRect(0, 16, 32, 4);
	// The first pixel samples just inside the ramp.
	assert.ok(pixel(ctx, 0, 17)[0] > 240);
});
