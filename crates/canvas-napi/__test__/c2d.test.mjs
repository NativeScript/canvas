import assert from 'node:assert/strict';
import test from 'node:test';

import { CanvasModule } from './addon.mjs';

const { CanvasRenderingContext2D, ImageData, Path2D, DOMMatrix } = CanvasModule;

function context(width = 64, height = 64) {
	// (width, height, density, alpha, fontColor, ppi, direction)
	return CanvasRenderingContext2D.withCpu(width, height, 1, true, 0, 96, 0);
}

function pixel(ctx, x, y) {
	return Array.from(ctx.getImageData(x, y, 1, 1).data);
}

test('installs globalThis.CanvasModule', () => {
	assert.equal(globalThis.CanvasModule, CanvasModule);
});

test('fillRect with a colour string renders and reads back', () => {
	const ctx = context();
	ctx.fillStyle = '#ff0000';
	ctx.fillRect(0, 0, 32, 32);
	assert.deepEqual(pixel(ctx, 4, 4), [255, 0, 0, 255]);
	assert.deepEqual(pixel(ctx, 40, 40), [0, 0, 0, 0]);
	assert.equal(ctx.fillStyle, '#ff0000');
});

test('lineCap round-trips (was swapped)', () => {
	const ctx = context();
	for (const cap of ['butt', 'round', 'square']) {
		ctx.lineCap = cap;
		assert.equal(ctx.lineCap, cap);
	}
});

test('textBaseline and globalCompositeOperation are ints, as packages/canvas passes them', () => {
	const ctx = context();
	ctx.textBaseline = 5;
	assert.equal(ctx.textBaseline, 5);
	ctx.globalCompositeOperation = 2;
	assert.equal(ctx.globalCompositeOperation, 2);
});

test('fill / clip / isPointInPath take int fill rules and optional paths', () => {
	const ctx = context();
	const path = new Path2D();
	path.rect(10, 10, 20, 20);
	ctx.fillStyle = '#00ff00';
	ctx.fill(path, 0);
	assert.deepEqual(pixel(ctx, 15, 15), [0, 255, 0, 255]);
	assert.equal(ctx.isPointInPath(path, 15, 15, 0), true);
	assert.equal(ctx.isPointInPath(path, 50, 50), false);
	ctx.beginPath();
	ctx.rect(0, 0, 5, 5);
	assert.equal(ctx.isPointInPath(2, 2, 1), true);
	ctx.fill(-1); // an unknown rule is ignored
	ctx.clip(0);
	ctx.clip(path, 1);
});

test('gradients and patterns', () => {
	const ctx = context();
	const gradient = ctx.createLinearGradient(0, 0, 64, 0);
	gradient.addColorStop(0, 'red');
	gradient.addColorStop(1, 'blue');
	ctx.fillStyle = gradient;
	assert.ok(ctx.fillStyle instanceof CanvasModule.CanvasGradient);
	ctx.fillRect(0, 0, 64, 64);

	const source = context(8, 8);
	source.fillStyle = '#0000ff';
	source.fillRect(0, 0, 8, 8);
	const pattern = ctx.createPattern(source, 'repeat');
	assert.ok(pattern instanceof CanvasModule.CanvasPattern);
	ctx.fillStyle = pattern;
	ctx.fillRect(0, 0, 64, 64);
	assert.deepEqual(pixel(ctx, 20, 20), [0, 0, 255, 255]);
});

test('drawImage from another 2D context flushes the source first', () => {
	const source = context(16, 16);
	source.fillStyle = '#ffff00';
	source.fillRect(0, 0, 16, 16);
	const ctx = context();
	ctx.drawImage(source, 0, 0);
	assert.deepEqual(pixel(ctx, 8, 8), [255, 255, 0, 255]);
	ctx.drawImage(source, 16, 16, 32, 32);
	assert.deepEqual(pixel(ctx, 40, 40), [255, 255, 0, 255]);
	ctx.drawImage(source, 0, 0, 8, 8, 48, 48, 8, 8);
	assert.deepEqual(pixel(ctx, 50, 50), [255, 255, 0, 255]);
});

test('ImageData.data is zero-copy and stable', () => {
	const data = new ImageData(4, 4);
	assert.equal(data.data, data.data);
	assert.equal(data.data.length, 64);
	data.data.fill(255);
	const ctx = context();
	ctx.putImageData(data, 0, 0);
	assert.deepEqual(pixel(ctx, 1, 1), [255, 255, 255, 255]);

	const fromArray = new ImageData(new Uint8ClampedArray(16), 2);
	assert.equal(fromArray.height, 2);
});

test('transforms', () => {
	const ctx = context();
	ctx.translate(10, 20);
	const m = ctx.getTransform();
	assert.ok(m instanceof DOMMatrix);
	ctx.setTransform(new DOMMatrix());
	ctx.setTransform(1, 0, 0, 1, 5, 5);
	ctx.resetTransform();
});

test('text and measureText', () => {
	const ctx = context();
	ctx.font = '12px sans-serif';
	const metrics = ctx.measureText('hello');
	assert.ok(metrics.width > 0);
	ctx.fillText('hi', 5, 20);
	ctx.strokeText('hi', 5, 40);
});

test('non-standard members packages/canvas relies on', () => {
	const ctx = context();
	ctx.fillStyle = '#123456';
	ctx.fillRect(0, 0, 64, 64);
	assert.match(ctx.__toDataURL('image/png', 0.9), /^data:image\/png;base64,/);
	assert.match(ctx.__getPointer(), /^\d+$/);
	ctx.__makeDirty();
	ctx.__stopRaf();
	ctx.__startRaf();
	ctx.drawPaint('#ffffff');
	ctx.drawPoint(1, 1);
	ctx.drawPoints(1, [
		{ x: 0, y: 0 },
		{ x: 10, y: 10 },
	]);
	ctx.fillOval(0, 0, 10, 10);
	ctx.strokeOval(0, 0, 10, 10);
	CanvasModule.__flushAll?.();
});

test('create2DContextWithPointer wraps without taking ownership', () => {
	const ctx = context();
	ctx.fillStyle = '#ff00ff';
	ctx.fillRect(0, 0, 64, 64);
	const view = CanvasModule.create2DContextWithPointer(BigInt(ctx.__getPointer()));
	assert.deepEqual(pixel(view, 1, 1), [255, 0, 255, 255]);
	globalThis.gc?.();
	// The original still works after the wrapper may have been collected.
	assert.deepEqual(pixel(ctx, 1, 1), [255, 0, 255, 255]);
});

test('GC churn', () => {
	for (let i = 0; i < 2000; i++) {
		const ctx = context(8, 8);
		ctx.fillRect(0, 0, 8, 8);
		ctx.getImageData(0, 0, 8, 8);
		new Path2D().lineTo(i, i);
	}
	globalThis.gc?.();
});
