// Context loss on Windows: the thread's Direct3D 12 device removed (as by a driver reset), found
// by the next flush, reported to the listener; 2D contexts restore onto a new device. A file of
// its own: the removal affects every 2D canvas in the process.
import assert from 'node:assert/strict';
import test from 'node:test';

import { CanvasModule } from './addon.mjs';

const skip = process.platform !== 'win32' && 'Windows host only';
const { NSCCanvas, create2DContextWithPointer } = CanvasModule;

function hostContext(width, height) {
	const host = new NSCCanvas();
	host.setSurfaceSize(width, height);
	const pointer = host.create2DContext(true, true, false, false, 0, true, false, false, false, false, 0, false, 0);
	return { host, ctx: create2DContextWithPointer(BigInt(pointer)) };
}

function pixel(ctx, x, y) {
	return Array.from(ctx.getImageData(x, y, 1, 1).data);
}

test('a removed device loses every 2D context; restoreContext brings each back', { skip }, (t) => {
	const drawing = hostContext(16, 16);
	const idle = hostContext(8, 8);
	drawing.ctx.fillStyle = '#ff0000';
	drawing.ctx.fillRect(0, 0, 16, 16);
	CanvasModule.__flushAll();
	assert.equal(drawing.host.isContextLost(), false);

	let notified = 0;
	CanvasModule.__setContextLostListener(() => notified++);
	t.after(() => CanvasModule.__setContextLostListener(null));

	if (!CanvasModule.__simulateD3DDeviceRemoval()) {
		t.skip('ID3D12Device5::RemoveDevice unavailable');
		return;
	}
	// The next flush finds it and tells the listener once.
	drawing.ctx.fillRect(0, 0, 4, 4);
	CanvasModule.__flushAll();
	assert.equal(notified, 1);
	assert.equal(drawing.host.isContextLost(), true);
	// Canvases that did not draw are on the same device.
	assert.equal(idle.host.isContextLost(), true);
	// Drawing into a lost context is a no-op, not a crash (each such flush reports it again).
	drawing.ctx.fillRect(0, 0, 16, 16);
	CanvasModule.__flushAll();
	assert.equal(notified, 2);

	// Restored: cleared, default state, drawing again.
	assert.equal(drawing.host.restoreContext(), true);
	assert.equal(drawing.host.isContextLost(), false);
	assert.deepEqual(pixel(drawing.ctx, 8, 8), [0, 0, 0, 0]);
	assert.equal(drawing.ctx.fillStyle, '#000000');
	drawing.ctx.fillStyle = '#00ff00';
	drawing.ctx.fillRect(0, 0, 16, 16);
	CanvasModule.__flushAll();
	assert.deepEqual(pixel(drawing.ctx, 8, 8), [0, 255, 0, 255]);

	// The second restore joins the new device; a new canvas is made on it too.
	assert.equal(idle.host.restoreContext(), true);
	idle.ctx.fillStyle = '#0000ff';
	idle.ctx.fillRect(0, 0, 8, 8);
	assert.deepEqual(pixel(idle.ctx, 4, 4), [0, 0, 255, 255]);
	const fresh = hostContext(8, 8);
	assert.equal(fresh.host.isContextLost(), false);
	fresh.ctx.fillRect(0, 0, 8, 8);
	assert.deepEqual(pixel(fresh.ctx, 4, 4), [0, 0, 0, 255]);
	// Nothing is lost any more.
	assert.equal(notified, 2);
});

test('restoreContext on a context that is not lost is a no-op', { skip }, () => {
	const { host, ctx } = hostContext(8, 8);
	ctx.fillStyle = '#ff0000';
	ctx.fillRect(0, 0, 8, 8);
	assert.equal(host.restoreContext(), true);
	assert.deepEqual(pixel(ctx, 4, 4), [255, 0, 0, 255]);
	assert.equal(new NSCCanvas().restoreContext(), false);
});

test('an on-screen 2D context (headless panel) is restored into its panel', { skip }, (t) => {
	const host = new NSCCanvas(CanvasModule.__createHeadlessPanel());
	host.setSurfaceSize(32, 32);
	host.setViewSize(32, 32);
	host.setCompositionScale(1, 1);
	const ctx = create2DContextWithPointer(BigInt(host.create2DContext(true, true, false, false, 0, true, false, false, false, false, 0, false, 0)));
	ctx.fillStyle = '#ff0000';
	ctx.fillRect(0, 0, 32, 32);
	host.present();
	assert.deepEqual(pixel(ctx, 16, 16), [255, 0, 0, 255]);

	if (!CanvasModule.__simulateD3DDeviceRemoval()) {
		t.skip('ID3D12Device5::RemoveDevice unavailable');
		return;
	}
	ctx.fillRect(0, 0, 4, 4);
	CanvasModule.__flushAll();
	assert.equal(host.isContextLost(), true);
	assert.equal(host.restoreContext(), true);
	assert.equal(host.isContextLost(), false);
	ctx.fillStyle = '#00ff00';
	ctx.fillRect(0, 0, 32, 32);
	host.present();
	assert.deepEqual(pixel(ctx, 16, 16), [0, 255, 0, 255]);
});
