/**
 * createImageBitmap / ImageBitmap / ImageBitmapRenderingContext conformance.
 */

import { suite, test, ok, equal, notEqual, closeTo, throws, rejects, makeCanvas, make2D, pixelAt, pixelEqual } from './harness';

/** A 20x10 ImageData: red left half, blue right half, opaque. */
function twoTone(ctx: any, width = 20, height = 10) {
	const data = ctx.createImageData(width, height);
	for (let y = 0; y < height; y++) {
		for (let x = 0; x < width; x++) {
			const i = (y * width + x) * 4;
			const left = x < width / 2;
			data.data[i] = left ? 255 : 0;
			data.data[i + 1] = 0;
			data.data[i + 2] = left ? 0 : 255;
			data.data[i + 3] = 255;
		}
	}
	return data;
}

/** A 20x10 canvas: red top half, blue bottom half. */
function twoToneCanvas() {
	const { canvas, ctx } = make2D(20, 10);
	ctx.fillStyle = '#ff0000';
	ctx.fillRect(0, 0, 20, 5);
	ctx.fillStyle = '#0000ff';
	ctx.fillRect(0, 5, 20, 5);
	return canvas;
}

export function registerImageBitmapSpec() {
	suite('imagebitmap.create', () => {
		test('createImageBitmap(ImageData) keeps the size', async () => {
			const { ctx } = make2D();
			const bitmap = await createImageBitmap(twoTone(ctx));
			equal(bitmap.width, 20, 'width');
			equal(bitmap.height, 10, 'height');
		});

		test('createImageBitmap(ImageData) keeps the pixels', async () => {
			const { ctx } = make2D();
			const bitmap = await createImageBitmap(twoTone(ctx));
			const dest = make2D(20, 10);
			dest.ctx.drawImage(bitmap, 0, 0);
			pixelEqual(dest.ctx, 3, 5, [255, 0, 0, 255], 4, 'the red half');
			pixelEqual(dest.ctx, 16, 5, [0, 0, 255, 255], 4, 'the blue half');
		});

		test('createImageBitmap(canvas) works', async () => {
			const bitmap = await createImageBitmap(twoToneCanvas() as any);
			equal(bitmap.width, 20, 'width');
			equal(bitmap.height, 10, 'height');
		});

		test('createImageBitmap(ImageBitmap) works', async () => {
			const { ctx } = make2D();
			const first = await createImageBitmap(twoTone(ctx));
			const second = await createImageBitmap(first);
			equal(second.width, 20, 'width');
			equal(second.height, 10, 'height');
		});

		test('createImageBitmap(encoded bytes) decodes', async () => {
			const bitmap = await createImageBitmap(PNG_4x2.slice(0) as any);
			equal(bitmap.width, 4, 'width');
			equal(bitmap.height, 2, 'height');
		});

		test('createImageBitmap(Blob) decodes', async () => {
			const blob = new Blob([PNG_4x2.slice(0)], { type: 'image/png' });
			const bitmap = await createImageBitmap(blob as any);
			equal(bitmap.width, 4, 'width');
			equal(bitmap.height, 2, 'height');
		});

		test('createImageBitmap with a crop rect crops', async () => {
			const { ctx } = make2D();
			const bitmap = await createImageBitmap(twoTone(ctx), 10, 0, 10, 10);
			equal(bitmap.width, 10, 'width');
			equal(bitmap.height, 10, 'height');
			const dest = make2D(10, 10);
			dest.ctx.drawImage(bitmap, 0, 0);
			pixelEqual(dest.ctx, 5, 5, [0, 0, 255, 255], 4, 'the crop should be the blue half');
		});

		test('createImageBitmap with a crop rect outside the source pads with transparent', async () => {
			const { ctx } = make2D();
			const bitmap = await createImageBitmap(twoTone(ctx), -5, -5, 10, 10);
			equal(bitmap.width, 10, 'width');
			equal(bitmap.height, 10, 'height');
		});

		test('resizeWidth and resizeHeight are both honoured', async () => {
			const { ctx } = make2D();
			const bitmap = await createImageBitmap(twoTone(ctx), { resizeWidth: 40, resizeHeight: 60 } as any);
			equal(bitmap.width, 40, 'resizeWidth');
			equal(bitmap.height, 60, 'resizeHeight -- a copy/paste of resizeWidth here would pass width and fail height');
		});

		test('resizeWidth alone scales proportionally', async () => {
			const { ctx } = make2D();
			const bitmap = await createImageBitmap(twoTone(ctx), { resizeWidth: 40 } as any);
			equal(bitmap.width, 40, 'width');
			equal(bitmap.height, 20, 'height should follow the 20x10 aspect ratio');
		});

		test('resizeHeight alone scales proportionally', async () => {
			const { ctx } = make2D();
			const bitmap = await createImageBitmap(twoTone(ctx), { resizeHeight: 20 } as any);
			equal(bitmap.width, 40, 'width should follow the 20x10 aspect ratio');
			equal(bitmap.height, 20, 'height');
		});

		test('imageOrientation flipY flips the source', async () => {
			const bitmap = await createImageBitmap(twoToneCanvas() as any, { imageOrientation: 'flipY' } as any);
			const dest = make2D(20, 10);
			dest.ctx.drawImage(bitmap, 0, 0);
			pixelEqual(dest.ctx, 10, 2, [0, 0, 255, 255], 6, 'the blue half should now be on top');
			pixelEqual(dest.ctx, 10, 8, [255, 0, 0, 255], 6, 'the red half should now be at the bottom');
		});

		test('createImageBitmap(null) rejects', async () => {
			await rejects(createImageBitmap(null as any), 'createImageBitmap(null) must reject');
		});

		test('createImageBitmap({}) rejects rather than hanging', async () => {
			await rejects(Promise.race([createImageBitmap({} as any), new Promise((_, reject) => setTimeout(() => reject(new Error('timed out: the promise never settled')), 3000))]), 'an unsupported source must reject');
		});

		test('a zero-width crop rect rejects', async () => {
			const { ctx } = make2D();
			await rejects(createImageBitmap(twoTone(ctx), 0, 0, 0, 10), 'a zero crop width must reject');
		});

		test('a detached source rejects', async () => {
			const { ctx } = make2D();
			const bitmap = await createImageBitmap(twoTone(ctx));
			bitmap.close();
			await rejects(createImageBitmap(bitmap), 'a closed ImageBitmap must not be a valid source');
		});
	});

	suite('imagebitmap.lifecycle', () => {
		test('close() detaches the bitmap', async () => {
			const { ctx } = make2D();
			const bitmap = await createImageBitmap(twoTone(ctx));
			equal(bitmap.width, 20, 'precondition');
			bitmap.close();
			equal(bitmap.width, 0, 'width after close');
			equal(bitmap.height, 0, 'height after close');
		});

		test('close() is idempotent', async () => {
			const { ctx } = make2D();
			const bitmap = await createImageBitmap(twoTone(ctx));
			bitmap.close();
			bitmap.close();
			equal(bitmap.width, 0);
		});
	});

	// ------------------------------------------------------- bitmaprenderer
	suite('bitmaprenderer', () => {
		test('getContext("bitmaprenderer") returns a context', () => {
			const canvas = makeCanvas(50, 50);
			const ctx = canvas.getContext('bitmaprenderer');
			ok(ctx, 'no bitmaprenderer context');
			equal(typeof (ctx as any).transferFromImageBitmap, 'function', 'transferFromImageBitmap');
		});

		test('getContext("bitmaprenderer") is idempotent', () => {
			const canvas = makeCanvas(50, 50);
			equal(canvas.getContext('bitmaprenderer'), canvas.getContext('bitmaprenderer'));
		});

		test('a bitmaprenderer canvas refuses a 2d context', () => {
			const canvas = makeCanvas(50, 50);
			ok(canvas.getContext('bitmaprenderer'), 'precondition');
			equal(canvas.getContext('2d'), null, 'a canvas handed out two different context types');
		});

		test('a 2d canvas refuses a bitmaprenderer context', () => {
			const canvas = makeCanvas(50, 50);
			ok(canvas.getContext('2d'), 'precondition');
			equal(canvas.getContext('bitmaprenderer'), null);
		});

		test('ctx.canvas points back at the canvas', () => {
			const canvas = makeCanvas(50, 50);
			equal((canvas.getContext('bitmaprenderer') as any).canvas, canvas);
		});

		test('transferFromImageBitmap paints the bitmap', async () => {
			const source = make2D();
			const bitmap = await createImageBitmap(twoTone(source.ctx));
			const canvas = makeCanvas(20, 10);
			const renderer = canvas.getContext('bitmaprenderer') as any;
			renderer.transferFromImageBitmap(bitmap);

			// Read the result back through a 2D copy of the canvas.
			const probe = make2D(20, 10);
			probe.ctx.drawImage(canvas as any, 0, 0);
			pixelEqual(probe.ctx, 3, 5, [255, 0, 0, 255], 4, 'the red half');
			pixelEqual(probe.ctx, 16, 5, [0, 0, 255, 255], 4, 'the blue half');
		});

		test('transferFromImageBitmap resizes the canvas to the bitmap', async () => {
			const source = make2D();
			const bitmap = await createImageBitmap(twoTone(source.ctx));
			const canvas = makeCanvas(50, 50);
			(canvas.getContext('bitmaprenderer') as any).transferFromImageBitmap(bitmap);
			equal((canvas as any).width, 20, 'width');
			equal((canvas as any).height, 10, 'height');
		});

		test('transferFromImageBitmap detaches the bitmap', async () => {
			const source = make2D();
			const bitmap = await createImageBitmap(twoTone(source.ctx));
			const canvas = makeCanvas(20, 10);
			(canvas.getContext('bitmaprenderer') as any).transferFromImageBitmap(bitmap);
			equal(bitmap.width, 0, 'width after transfer');
			equal(bitmap.height, 0, 'height after transfer');
		});

		test('re-transferring a detached bitmap throws InvalidStateError', async () => {
			const source = make2D();
			const bitmap = await createImageBitmap(twoTone(source.ctx));
			const canvas = makeCanvas(20, 10);
			const renderer = canvas.getContext('bitmaprenderer') as any;
			renderer.transferFromImageBitmap(bitmap);
			throws(() => renderer.transferFromImageBitmap(bitmap), 'InvalidStateError');
		});

		test('transferFromImageBitmap(null) blanks the canvas', async () => {
			const source = make2D();
			const bitmap = await createImageBitmap(twoTone(source.ctx));
			const canvas = makeCanvas(20, 10);
			const renderer = canvas.getContext('bitmaprenderer') as any;
			renderer.transferFromImageBitmap(bitmap);
			renderer.transferFromImageBitmap(null);

			const probe = make2D(20, 10);
			probe.ctx.drawImage(canvas as any, 0, 0);
			pixelEqual(probe.ctx, 10, 5, [0, 0, 0, 0], 2, 'the canvas should be transparent black again');
		});

		test('a second transfer replaces the first', async () => {
			const source = make2D();
			const first = await createImageBitmap(twoTone(source.ctx));
			const canvas = makeCanvas(20, 10);
			const renderer = canvas.getContext('bitmaprenderer') as any;
			renderer.transferFromImageBitmap(first);

			const green = make2D(8, 8);
			green.ctx.fillStyle = '#00ff00';
			green.ctx.fillRect(0, 0, 8, 8);
			const second = await createImageBitmap(green.canvas as any);
			renderer.transferFromImageBitmap(second);

			equal((canvas as any).width, 8, 'the canvas should have taken the new size');
			const probe = make2D(8, 8);
			probe.ctx.drawImage(canvas as any, 0, 0);
			pixelEqual(probe.ctx, 4, 4, [0, 255, 0, 255], 4, 'the new bitmap');
		});

		test('transferFromImageBitmap rejects a non-ImageBitmap', () => {
			const canvas = makeCanvas(20, 10);
			const renderer = canvas.getContext('bitmaprenderer') as any;
			throws(() => renderer.transferFromImageBitmap({} as any), 'TypeError');
		});

		test('alpha:false composites onto opaque black', async () => {
			const translucent = make2D(10, 10);
			translucent.ctx.fillStyle = 'rgba(255, 0, 0, 0.5)';
			translucent.ctx.fillRect(0, 0, 10, 10);
			const bitmap = await createImageBitmap(translucent.canvas as any);

			const canvas = makeCanvas(10, 10);
			const renderer = canvas.getContext('bitmaprenderer', { alpha: false }) as any;
			renderer.transferFromImageBitmap(bitmap);

			const probe = make2D(10, 10);
			probe.ctx.drawImage(canvas as any, 0, 0);
			const [, , , a] = pixelAt(probe.ctx, 5, 5);
			equal(a, 255, 'an alpha:false bitmaprenderer must be opaque');
		});

		test('createPattern accepts an ImageBitmap', async () => {
			const bitmap = await createImageBitmap(twoToneCanvas() as any);
			const { ctx } = make2D(40, 40);
			const pattern = ctx.createPattern(bitmap, 'repeat');
			ok(pattern, 'no pattern');
			ctx.fillStyle = pattern;
			ctx.fillRect(0, 0, 40, 40);
			pixelEqual(ctx, 10, 2, [255, 0, 0, 255], 4, 'the red half of the tile');
			pixelEqual(ctx, 10, 7, [0, 0, 255, 255], 4, 'the blue half of the tile');
		});

		test('toDataURL works on a bitmaprenderer canvas', async () => {
			const source = make2D();
			const bitmap = await createImageBitmap(twoTone(source.ctx));
			const canvas = makeCanvas(20, 10);
			(canvas.getContext('bitmaprenderer') as any).transferFromImageBitmap(bitmap);
			const url = (canvas as any).toDataURL('image/png');
			ok(typeof url === 'string' && url.indexOf('data:image/png') === 0, `unexpected data URL: ${String(url).slice(0, 40)}`);
			ok(url.length > 100, 'the data URL looks empty');
		});
	});
}

/**
 * A 4x2 PNG (red/blue columns), base64-decoded at load. Keeping the bytes inline
 * means the encoded-source tests do not depend on a file or the network.
 */
const PNG_4x2 = (() => {
	const b64 = 'iVBORw0KGgoAAAANSUhEUgAAAAQAAAACCAYAAAB/qH1jAAAAFElEQVR42mP4z8DwH4Sh1H8GdAEABykP8SfadKYAAAAASUVORK5CYII=';
	const lookup = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
	const clean = b64.replace(/=+$/, '');
	const out = new Uint8Array((clean.length * 3) >> 2);
	let bits = 0;
	let value = 0;
	let index = 0;
	for (let i = 0; i < clean.length; i++) {
		value = (value << 6) | lookup.indexOf(clean[i]);
		bits += 6;
		if (bits >= 8) {
			bits -= 8;
			out[index++] = (value >> bits) & 0xff;
		}
	}
	return out;
})();
