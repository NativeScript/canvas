/**
 * A canvas used as an image source by another drawing context.
 *
 * On the web every canvas is a `CanvasImageSource`, so any of them can be drawn into a 2d
 * context, uploaded as a WebGL texture, or copied into a WebGPU texture, regardless of
 * which kind of context it owns. These tests walk that matrix and check the pixels that
 * come out the other side, rather than just that the call did not throw — silently drawing
 * nothing is the failure mode that matters here.
 */

import { suite, test, ok, equal, makeCanvas, make2D, pixelAt } from './harness';

declare const navigator: any;

/** Opaque red, as a 2d fill and as the expected readback. */
const RED: [number, number, number, number] = [255, 0, 0, 255];
/** Opaque green, used for WebGL clears so the two sources cannot be confused. */
const GREEN: [number, number, number, number] = [0, 255, 0, 255];

/** A canvas whose 2d context has been filled with a solid colour. */
function make2DSource(size = 32, colour = 'red') {
	const { canvas, ctx } = make2D(size, size);
	ctx.fillStyle = colour;
	ctx.fillRect(0, 0, size, size);
	return canvas;
}

/** A canvas whose WebGL context has been cleared to a solid colour. */
function makeWebGLSource(size = 32, version: 'webgl' | 'webgl2' = 'webgl') {
	const canvas = makeCanvas(size, size);
	const gl = canvas.getContext(version) as any;
	if (!gl) {
		return null;
	}
	gl.viewport(0, 0, size, size);
	gl.clearColor(0, 1, 0, 1);
	gl.clear(gl.COLOR_BUFFER_BIT);
	gl.flush();
	return { canvas, gl };
}

/** Compare a pixel with a small tolerance — a GPU blit need not be bit exact. */
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

export function registerCanvasSourceSpec() {
	suite('canvassource.2d', () => {
		test('drawImage accepts a 2d canvas', () => {
			const source = make2DSource(32, 'red');
			const { ctx } = make2D(32, 32);

			ctx.drawImage(source, 0, 0);

			const pixel = pixelAt(ctx, 16, 16);
			ok(pixelNear(pixel, RED), `expected red, got ${describePixel(pixel)}`);
		});

		test('drawImage accepts a webgl canvas', () => {
			const source = makeWebGLSource(32);
			ok(source, 'could not create a webgl context');

			const { ctx } = make2D(32, 32);
			ctx.drawImage(source.canvas, 0, 0);

			const pixel = pixelAt(ctx, 16, 16);
			ok(pixelNear(pixel, GREEN), `expected green, got ${describePixel(pixel)}`);
		});

		test('drawImage scales a canvas source to the destination rect', () => {
			const source = make2DSource(32, 'red');
			const { ctx } = make2D(64, 64);

			// Into the bottom-right quadrant only, so the untouched half proves the
			// destination rect was honoured rather than the whole canvas being painted.
			ctx.drawImage(source, 32, 32, 32, 32);

			const inside = pixelAt(ctx, 48, 48);
			const outside = pixelAt(ctx, 8, 8);
			ok(pixelNear(inside, RED), `expected red inside the rect, got ${describePixel(inside)}`);
			equal(outside[3], 0, 'the area outside the destination rect should be untouched');
		});

		test('drawImage takes a source sub-rect from a canvas', () => {
			// Left half red, right half blue: taking the right half must yield blue.
			const { canvas: source, ctx: sourceCtx } = make2D(32, 32);
			sourceCtx.fillStyle = 'red';
			sourceCtx.fillRect(0, 0, 16, 32);
			sourceCtx.fillStyle = 'blue';
			sourceCtx.fillRect(16, 0, 16, 32);

			const { ctx } = make2D(16, 32);
			ctx.drawImage(source, 16, 0, 16, 32, 0, 0, 16, 32);

			const pixel = pixelAt(ctx, 8, 16);
			ok(pixelNear(pixel, [0, 0, 255, 255]), `expected blue, got ${describePixel(pixel)}`);
		});

		test('createPattern accepts a canvas', () => {
			const source = make2DSource(16, 'red');
			const { ctx } = make2D(32, 32);

			const pattern = ctx.createPattern(source, 'repeat');
			ok(pattern, 'createPattern returned null for a canvas source');

			ctx.fillStyle = pattern;
			ctx.fillRect(0, 0, 32, 32);

			const pixel = pixelAt(ctx, 24, 24);
			ok(pixelNear(pixel, RED), `expected the pattern to tile red, got ${describePixel(pixel)}`);
		});
	});

	suite('canvassource.webgl', () => {
		/** Upload `source` as a texture and read back the pixel it renders to. */
		function uploadAndReadBack(source: any, size = 32) {
			const canvas = makeCanvas(size, size);
			const gl = canvas.getContext('webgl') as any;
			if (!gl) {
				return null;
			}

			const texture = gl.createTexture();
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, source);

			const error = gl.getError();
			if (error !== gl.NO_ERROR) {
				return { error };
			}

			// Render the texture to the framebuffer so readPixels sees what was uploaded.
			const program = gl.createProgram();
			const vs = gl.createShader(gl.VERTEX_SHADER);
			gl.shaderSource(
				vs,
				`attribute vec2 position;
varying vec2 uv;
void main() {
  uv = position * 0.5 + 0.5;
  gl_Position = vec4(position, 0.0, 1.0);
}`,
			);
			gl.compileShader(vs);
			const fs = gl.createShader(gl.FRAGMENT_SHADER);
			gl.shaderSource(
				fs,
				`precision mediump float;
uniform sampler2D tex;
varying vec2 uv;
void main() { gl_FragColor = texture2D(tex, uv); }`,
			);
			gl.compileShader(fs);
			gl.attachShader(program, vs);
			gl.attachShader(program, fs);
			gl.linkProgram(program);
			gl.useProgram(program);

			const buffer = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3]), gl.STATIC_DRAW);
			const position = gl.getAttribLocation(program, 'position');
			gl.enableVertexAttribArray(position);
			gl.vertexAttribPointer(position, 2, gl.FLOAT, false, 0, 0);

			gl.viewport(0, 0, size, size);
			gl.clearColor(0, 0, 0, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);
			gl.drawArrays(gl.TRIANGLES, 0, 3);
			gl.finish();

			const pixels = new Uint8Array(4);
			gl.readPixels(size / 2, size / 2, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
			return { pixels };
		}

		test('texImage2D accepts a 2d canvas', () => {
			const source = make2DSource(32, 'red');
			const result = uploadAndReadBack(source);
			ok(result, 'could not create a webgl context');
			ok(!result.error, `texImage2D raised GL error ${result.error}`);
			ok(pixelNear(result.pixels, RED), `expected red, got ${describePixel(result.pixels)}`);
		});

		test('texImage2D accepts a webgl canvas', () => {
			const source = makeWebGLSource(32);
			ok(source, 'could not create the source webgl context');

			const result = uploadAndReadBack(source.canvas);
			ok(result, 'could not create a webgl context');
			ok(!result.error, `texImage2D raised GL error ${result.error}`);
			ok(pixelNear(result.pixels, GREEN), `expected green, got ${describePixel(result.pixels)}`);
		});
	});

	suite('canvassource.webgpu', () => {
		async function getDevice() {
			const adapter = await navigator.gpu?.requestAdapter();
			const device = await adapter?.requestDevice();
			return device;
		}

		/** Copy `source` into a texture and read the centre pixel back. */
		async function copyAndReadBack(device: any, source: any, size = 32) {
			const texture = device.createTexture({
				size: [size, size, 1],
				format: 'rgba8unorm',
				usage: GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC | GPUTextureUsage.RENDER_ATTACHMENT,
			});

			device.queue.copyExternalImageToTexture({ source }, { texture }, [size, size, 1]);

			const bytesPerRow = 256;
			const readback = device.createBuffer({
				size: bytesPerRow * size,
				usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
			});
			const encoder = device.createCommandEncoder();
			encoder.copyTextureToBuffer({ texture }, { buffer: readback, bytesPerRow }, { width: size, height: size, depthOrArrayLayers: 1 });
			device.queue.submit([encoder.finish()]);

			await readback.mapAsync(GPUMapMode.READ);
			const pixels = new Uint8Array(readback.getMappedRange().slice(0));
			readback.unmap();

			const centre = (size / 2) * bytesPerRow + (size / 2) * 4;
			return pixels.subarray(centre, centre + 4);
		}

		test('copyExternalImageToTexture accepts a 2d canvas', async () => {
			const device = await getDevice();
			ok(device, 'no webgpu device');

			const source = make2DSource(32, 'red');
			const pixel = await copyAndReadBack(device, source);
			ok(pixelNear(pixel, RED), `expected red, got ${describePixel(pixel)}`);
		});

		test('copyExternalImageToTexture accepts a webgl canvas', async () => {
			const device = await getDevice();
			ok(device, 'no webgpu device');

			const source = makeWebGLSource(32);
			ok(source, 'could not create the source webgl context');

			const pixel = await copyAndReadBack(device, source.canvas);
			ok(pixelNear(pixel, GREEN), `expected green, got ${describePixel(pixel)}`);
		});
	});
}
