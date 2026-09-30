/**
 * OffscreenCanvas against the HTML spec: its own API, each context it hands out, its use as an
 * image source by other contexts, and `canvas.transferControlToOffscreen()`.
 *
 * Where the result is pixels, the tests read them back rather than only checking that a call did
 * not throw.
 */

import { OffscreenCanvas, OffscreenCanvasRenderingContext2D } from '@nativescript/canvas';
import { suite, test, skip, ok, equal, throws, rejects, makeCanvas, make2D, pixelAt } from './harness';

declare const navigator: any;

const RED: [number, number, number, number] = [255, 0, 0, 255];
const GREEN: [number, number, number, number] = [0, 255, 0, 255];
const BLUE: [number, number, number, number] = [0, 0, 255, 255];
const CLEAR: [number, number, number, number] = [0, 0, 0, 0];

function pixelNear(actual: ArrayLike<number>, expected: ArrayLike<number>, tolerance = 8) {
	for (let i = 0; i < 4; i++) {
		if (Math.abs(actual[i] - expected[i]) > tolerance) {
			return false;
		}
	}
	return true;
}

function describePixel(pixel: ArrayLike<number>) {
	return `[${pixel[0]}, ${pixel[1]}, ${pixel[2]}, ${pixel[3]}]`;
}

function expectPixel(ctx: any, x: number, y: number, expected: ArrayLike<number>) {
	const pixel = pixelAt(ctx, x, y);
	ok(pixelNear(pixel, expected), `expected ${describePixel(expected)}, got ${describePixel(pixel)}`);
}

/** An OffscreenCanvas whose 2d context is filled with `colour`. */
function filled2D(width: number, height: number, colour: string) {
	const canvas = new OffscreenCanvas(width, height);
	const ctx = canvas.getContext('2d') as any;
	ok(ctx, 'no 2d context');
	ctx.fillStyle = colour;
	ctx.fillRect(0, 0, width, height);
	return { canvas, ctx };
}

/** An OffscreenCanvas whose WebGL context is cleared to green. */
function clearedWebGL(size: number, version: 'webgl' | 'webgl2' = 'webgl') {
	const canvas = new OffscreenCanvas(size, size);
	const gl = canvas.getContext(version) as any;
	ok(gl, `no ${version} context`);
	gl.viewport(0, 0, size, size);
	gl.clearColor(0, 1, 0, 1);
	gl.clear(gl.COLOR_BUFFER_BIT);
	return { canvas, gl };
}

function readGL(gl: any, x = 0, y = 0) {
	const pixel = new Uint8Array(4);
	gl.readPixels(x, y, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixel);
	return pixel;
}

export function registerOffscreenCanvasSpec() {
	suite('offscreen.api', () => {
		test('is a global', () => {
			equal(typeof (global as any).OffscreenCanvas, 'function');
		});

		test('keeps its size', () => {
			const canvas = new OffscreenCanvas(64, 32);
			equal(canvas.width, 64);
			equal(canvas.height, 32);
		});

		test('toStringTag is OffscreenCanvas', () => {
			equal(Object.prototype.toString.call(new OffscreenCanvas(1, 1)), '[object OffscreenCanvas]');
		});

		test('the constructor needs two arguments', () => {
			throws(() => new (OffscreenCanvas as any)(4), 'TypeError');
		});

		test('a negative size is a TypeError', () => {
			throws(() => new OffscreenCanvas(-1, 4), 'TypeError');
		});

		test('an unknown context id is a TypeError', () => {
			throws(() => new OffscreenCanvas(4, 4).getContext('nope' as any), 'TypeError');
		});

		test('getContext returns the same context for the same id', () => {
			const canvas = new OffscreenCanvas(4, 4);
			const ctx = canvas.getContext('2d');
			ok(ctx, 'no 2d context');
			equal(canvas.getContext('2d'), ctx);
		});

		test('getContext returns null for another id once it has a context', () => {
			const canvas = new OffscreenCanvas(4, 4);
			canvas.getContext('2d');
			equal(canvas.getContext('webgl'), null);
		});

		test('transferToImageBitmap without a context is an InvalidStateError', () => {
			throws(() => new OffscreenCanvas(4, 4).transferToImageBitmap(), 'InvalidStateError');
		});

		test('convertToBlob with a zero size is an IndexSizeError', async () => {
			const error = await rejects(new OffscreenCanvas(0, 4).convertToBlob());
			equal(error?.name, 'IndexSizeError');
		});
	});

	suite('offscreen.2d', () => {
		test('the context points back at the OffscreenCanvas', () => {
			const canvas = new OffscreenCanvas(4, 4);
			equal((canvas.getContext('2d') as any).canvas, canvas);
		});

		test('the context is an OffscreenCanvasRenderingContext2D', () => {
			ok(new OffscreenCanvas(4, 4).getContext('2d') instanceof (OffscreenCanvasRenderingContext2D as any));
		});

		test('a canvas 2d context is not an OffscreenCanvasRenderingContext2D', () => {
			const { ctx } = make2D(4, 4);
			ok(!(ctx instanceof (OffscreenCanvasRenderingContext2D as any)));
		});

		test('draws', () => {
			const { ctx } = filled2D(16, 16, 'red');
			expectPixel(ctx, 8, 8, RED);
		});

		test('setting the size resets the context', () => {
			const { canvas, ctx } = filled2D(16, 16, 'red');
			canvas.width = 8;
			equal(canvas.width, 8);
			equal(ctx.getImageData(0, 0, 8, 16).width, 8);
			expectPixel(ctx, 4, 4, CLEAR);
		});

		test('transferToImageBitmap hands over the frame and clears the canvas', () => {
			const { canvas, ctx } = filled2D(16, 8, 'red');
			const bitmap = canvas.transferToImageBitmap();
			equal(bitmap.width, 16);
			equal(bitmap.height, 8);
			expectPixel(ctx, 4, 4, CLEAR);

			const { ctx: target } = make2D(16, 8);
			target.drawImage(bitmap, 0, 0);
			expectPixel(target, 4, 4, RED);
		});

		test('convertToBlob encodes a png by default', async () => {
			const { canvas } = filled2D(8, 8, 'red');
			const blob = await canvas.convertToBlob();
			ok(blob.size > 0, 'empty blob');
			equal(blob.type, 'image/png');
		});

		test('convertToBlob encodes a jpeg', async () => {
			const { canvas } = filled2D(8, 8, 'red');
			const blob = await canvas.convertToBlob({ type: 'image/jpeg', quality: 0.8 });
			ok(blob.size > 0, 'empty blob');
			equal(blob.type, 'image/jpeg');
		});

		test('bitmaprenderer shows a transferred bitmap', () => {
			const { canvas } = filled2D(8, 8, 'red');
			const bitmap = canvas.transferToImageBitmap();
			const renderer = new OffscreenCanvas(8, 8).getContext('bitmaprenderer') as any;
			ok(renderer, 'no bitmaprenderer context');
			renderer.transferFromImageBitmap(bitmap);
			const { ctx: target } = make2D(8, 8);
			target.drawImage(renderer.canvas, 0, 0);
			expectPixel(target, 4, 4, RED);
		});
	});

	suite('offscreen.source', () => {
		test('drawImage takes an OffscreenCanvas', () => {
			const { canvas } = filled2D(16, 16, 'red');
			const { ctx } = make2D(16, 16);
			ctx.drawImage(canvas, 0, 0);
			expectPixel(ctx, 8, 8, RED);
		});

		test('createPattern takes an OffscreenCanvas', () => {
			const { canvas } = filled2D(4, 4, 'red');
			const { ctx } = make2D(16, 16);
			const pattern = ctx.createPattern(canvas, 'repeat');
			ok(pattern, 'no pattern');
			ctx.fillStyle = pattern;
			ctx.fillRect(0, 0, 16, 16);
			expectPixel(ctx, 10, 10, RED);
		});

		test('createImageBitmap takes an OffscreenCanvas', async () => {
			const { canvas } = filled2D(16, 8, 'red');
			const bitmap = await (global as any).createImageBitmap(canvas);
			equal(bitmap.width, 16);
			equal(bitmap.height, 8);
			const { ctx } = make2D(16, 8);
			ctx.drawImage(bitmap, 0, 0);
			expectPixel(ctx, 4, 4, RED);
		});

		test('drawImage takes a webgl OffscreenCanvas', () => {
			const { canvas } = clearedWebGL(16);
			const { ctx } = make2D(16, 16);
			ctx.drawImage(canvas, 0, 0);
			expectPixel(ctx, 8, 8, GREEN);
		});

		test('texImage2D takes an OffscreenCanvas', () => {
			const { canvas: source } = filled2D(4, 4, 'red');
			const target = makeCanvas(4, 4);
			const gl = target.getContext('webgl') as any;
			ok(gl, 'no webgl context');
			const texture = gl.createTexture();
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, source);
			equal(gl.getError(), 0);

			// Read the texture back through a framebuffer.
			const framebuffer = gl.createFramebuffer();
			gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
			gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0);
			equal(gl.checkFramebufferStatus(gl.FRAMEBUFFER), gl.FRAMEBUFFER_COMPLETE);
			const pixel = readGL(gl, 1, 1);
			ok(pixelNear(pixel, RED), `expected red, got ${describePixel(pixel)}`);
		});
	});

	suite('offscreen.webgl', () => {
		for (const version of ['webgl', 'webgl2'] as const) {
			test(`${version} draws`, () => {
				const { gl } = clearedWebGL(8, version);
				const pixel = readGL(gl);
				ok(pixelNear(pixel, GREEN), `expected green, got ${describePixel(pixel)}`);
			});

			test(`${version} transferToImageBitmap hands over the frame and clears the canvas`, () => {
				const { canvas, gl } = clearedWebGL(8, version);
				const bitmap = canvas.transferToImageBitmap();
				equal(bitmap.width, 8);
				const { ctx } = make2D(8, 8);
				ctx.drawImage(bitmap, 0, 0);
				expectPixel(ctx, 4, 4, GREEN);
				const pixel = readGL(gl);
				ok(pixelNear(pixel, CLEAR), `expected cleared, got ${describePixel(pixel)}`);
			});
		}

		test('transferToImageBitmap leaves the app its own framebuffer bound', () => {
			const { canvas, gl } = clearedWebGL(8);
			const texture = gl.createTexture();
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 8, 8, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
			const framebuffer = gl.createFramebuffer();
			gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
			gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0);
			canvas.transferToImageBitmap();
			// Still drawing into the texture: a clear lands there, not in the drawing buffer.
			gl.clearColor(0, 0, 1, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);
			const pixel = readGL(gl);
			ok(pixelNear(pixel, BLUE), `expected the texture's blue, got ${describePixel(pixel)}`);
			gl.bindFramebuffer(gl.FRAMEBUFFER, null);
			const drawingBuffer = readGL(gl);
			ok(pixelNear(drawingBuffer, CLEAR), `the drawing buffer should stay cleared, got ${describePixel(drawingBuffer)}`);
		});
	});

	suite('offscreen.webgpu', () => {
		if (!navigator?.gpu) {
			skip('draws', 'navigator.gpu is missing');
			return;
		}
		test('draws and transfers', async () => {
			const adapter = await navigator.gpu.requestAdapter();
			ok(adapter, 'no adapter');
			const device = await adapter.requestDevice();
			const canvas = new OffscreenCanvas(8, 8);
			const context = canvas.getContext('webgpu') as any;
			ok(context, 'no webgpu context');
			const format = navigator.gpu.getPreferredCanvasFormat();
			context.configure({ device, format, alphaMode: 'premultiplied' });
			const encoder = device.createCommandEncoder();
			const pass = encoder.beginRenderPass({
				colorAttachments: [{ view: context.getCurrentTexture().createView(), clearValue: { r: 0, g: 0, b: 1, a: 1 }, loadOp: 'clear', storeOp: 'store' }],
			});
			pass.end();
			device.queue.submit([encoder.finish()]);
			const bitmap = canvas.transferToImageBitmap();
			equal(bitmap.width, 8);
			const { ctx } = make2D(8, 8);
			ctx.drawImage(bitmap, 0, 0);
			expectPixel(ctx, 4, 4, BLUE);
		});
	});

	suite('offscreen.transfer', () => {
		test('takes the canvas size', () => {
			const canvas = makeCanvas(20, 10) as any;
			const offscreen = canvas.transferControlToOffscreen();
			equal(offscreen.width, 20);
			equal(offscreen.height, 10);
		});

		test('the canvas can no longer getContext', () => {
			const canvas = makeCanvas(4, 4) as any;
			canvas.transferControlToOffscreen();
			throws(() => canvas.getContext('2d'), 'InvalidStateError');
		});

		test('the canvas can no longer be resized', () => {
			const canvas = makeCanvas(4, 4) as any;
			canvas.transferControlToOffscreen();
			throws(() => (canvas.width = 5), 'InvalidStateError');
		});

		test('a canvas transfers once', () => {
			const canvas = makeCanvas(4, 4) as any;
			canvas.transferControlToOffscreen();
			throws(() => canvas.transferControlToOffscreen(), 'InvalidStateError');
		});

		test('a canvas with a context cannot transfer', () => {
			const { canvas } = make2D(4, 4);
			throws(() => (canvas as any).transferControlToOffscreen(), 'InvalidStateError');
		});

		test('the OffscreenCanvas draws into the canvas surface', () => {
			const canvas = makeCanvas(8, 8) as any;
			const offscreen = canvas.transferControlToOffscreen();
			const ctx = offscreen.getContext('2d');
			ctx.fillStyle = 'blue';
			ctx.fillRect(0, 0, 8, 8);
			expectPixel(ctx, 4, 4, BLUE);
			// The canvas shows what the OffscreenCanvas drew: it is an image of the same frame.
			const { ctx: target } = make2D(8, 8);
			target.drawImage(offscreen, 0, 0);
			expectPixel(target, 4, 4, BLUE);
		});

		test('resizing the OffscreenCanvas resizes the canvas', () => {
			const canvas = makeCanvas(8, 8) as any;
			const offscreen = canvas.transferControlToOffscreen();
			offscreen.getContext('2d');
			offscreen.width = 30;
			equal(canvas.width, 30);
		});
	});
}
