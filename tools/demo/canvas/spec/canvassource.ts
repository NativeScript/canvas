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

/** Opaque blue, used for WebGPU clears. */
const BLUE: [number, number, number, number] = [0, 0, 255, 255];

declare const __WINDOWS__: boolean;
const WINDOWS = typeof __WINDOWS__ !== 'undefined' && __WINDOWS__;

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
		/** `at` is in readPixels coordinates: y counts up from texture row 0. */
		function uploadAndReadBack(source: any, size = 32, upload?: (gl: any) => void, at: [number, number] = [size / 2, size / 2]) {
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
			if (upload) {
				upload(gl);
			} else {
				gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, source);
			}

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
			gl.readPixels(at[0], at[1], 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
			return { pixels };
		}

		test('texImage2D honours UNPACK_PREMULTIPLY_ALPHA_WEBGL for a 2d canvas', () => {
			const source = make2DSource(32, 'rgba(255, 0, 0, 0.5)');
			const straight = uploadAndReadBack(source);
			ok(straight && !straight.error, `texImage2D raised GL error ${straight?.error}`);
			ok(pixelNear(straight.pixels, [255, 0, 0, 128]), `straight alpha, got ${describePixel(straight.pixels)}`);
			const premultiplied = uploadAndReadBack(source, 32, (gl) => {
				gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, true);
				equal(gl.getParameter(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL), true, 'getParameter(UNPACK_PREMULTIPLY_ALPHA_WEBGL)');
				gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, source);
			});
			ok(premultiplied && !premultiplied.error, `texImage2D raised GL error ${premultiplied?.error}`);
			ok(pixelNear(premultiplied.pixels, [128, 0, 0, 128]), `premultiplied, got ${describePixel(premultiplied.pixels)}`);
		});

		for (const flipY of [false, true]) {
			test(`texSubImage2D uploads a premultiplied 2d canvas${flipY ? ' with UNPACK_FLIP_Y_WEBGL' : ''}`, () => {
				const { canvas, ctx } = make2D(32, 32);
				ctx.fillStyle = 'red';
				ctx.fillRect(0, 0, 32, 16);
				ctx.fillStyle = 'blue';
				ctx.fillRect(0, 16, 32, 16);
				const result = uploadAndReadBack(
					canvas,
					32,
					(gl) => {
						gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, true);
						gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, flipY);
						gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 32, 32, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
						gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, gl.RGBA, gl.UNSIGNED_BYTE, canvas);
					},
					[16, 4],
				);
				ok(result && !result.error, `texSubImage2D raised GL error ${result?.error}`);
				// Texture row 0 is the canvas's top row, or its bottom row when flipped.
				const expected = flipY ? BLUE : RED;
				ok(pixelNear(result.pixels, expected), `expected ${flipY ? 'blue' : 'red'} in row 4, got ${describePixel(result.pixels)}`);
			});
		}

		for (const flipY of [false, true]) {
			test(`texSubImage2D uploads a straight-alpha 2d canvas${flipY ? ' with UNPACK_FLIP_Y_WEBGL' : ''}`, () => {
				const { canvas, ctx } = make2D(32, 32);
				ctx.fillStyle = 'rgba(255, 0, 0, 0.5)';
				ctx.fillRect(0, 0, 32, 16);
				ctx.fillStyle = 'rgba(0, 0, 255, 0.25)';
				ctx.fillRect(0, 16, 32, 16);
				const result = uploadAndReadBack(
					canvas,
					32,
					(gl) => {
						gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, flipY);
						gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 32, 32, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
						gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, gl.RGBA, gl.UNSIGNED_BYTE, canvas);
					},
					[16, 4],
				);
				ok(result && !result.error, `texSubImage2D raised GL error ${result?.error}`);
				const expected = flipY ? [0, 0, 255, 64] : [255, 0, 0, 128];
				ok(pixelNear(result.pixels, expected), `expected ${describePixel(expected)} in row 4, got ${describePixel(result.pixels)}`);
			});
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

		// Node-API hosts only so far; the iOS and Android bindings do not take a WebGPU source yet.
		if (WINDOWS) {
			/** A canvas whose WebGPU context has been cleared to opaque blue. */
			async function makeWebGPUSource(size = 32) {
				const adapter = await navigator.gpu?.requestAdapter();
				const device = await adapter?.requestDevice();
				if (!device) {
					return null;
				}
				const canvas = makeCanvas(size, size);
				const ctx = canvas.getContext('webgpu') as any;
				ctx.configure({ device, format: navigator.gpu.getPreferredCanvasFormat(), alphaMode: 'opaque' });
				const encoder = device.createCommandEncoder();
				const pass = encoder.beginRenderPass({
					colorAttachments: [{ view: ctx.getCurrentTexture().createView(), clearValue: { r: 0, g: 0, b: 1, a: 1 }, loadOp: 'clear', storeOp: 'store' }],
				});
				pass.end();
				device.queue.submit([encoder.finish()]);
				return canvas;
			}

			test('texImage2D accepts a webgpu canvas', async () => {
				const source = await makeWebGPUSource(32);
				ok(source, 'no webgpu device');
				const result = uploadAndReadBack(source);
				ok(result, 'could not create a webgl context');
				ok(!result.error, `texImage2D raised GL error ${result.error}`);
				ok(pixelNear(result.pixels, BLUE), `expected blue, got ${describePixel(result.pixels)}`);
			});

			test('texSubImage2D accepts a webgpu canvas', async () => {
				const source = await makeWebGPUSource(32);
				ok(source, 'no webgpu device');
				const result = uploadAndReadBack(source, 32, (gl) => {
					gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 32, 32, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
					gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, gl.RGBA, gl.UNSIGNED_BYTE, source);
				});
				ok(result, 'could not create a webgl context');
				ok(!result.error, `texSubImage2D raised GL error ${result.error}`);
				ok(pixelNear(result.pixels, BLUE), `expected blue, got ${describePixel(result.pixels)}`);
			});
		}
	});

	suite('canvassource.webgpu', () => {
		async function getDevice() {
			const adapter = await navigator.gpu?.requestAdapter();
			const device = await adapter?.requestDevice();
			return device;
		}

		/** Copy `source` into a texture and read a pixel back (the centre by default). */
		async function copyAndReadBack(device: any, source: any, size = 32, options: { flipY?: boolean; premultipliedAlpha?: boolean; at?: [number, number]; after?: () => void } = {}) {
			const texture = device.createTexture({
				size: [size, size, 1],
				format: 'rgba8unorm',
				usage: GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC | GPUTextureUsage.RENDER_ATTACHMENT,
			});

			device.queue.copyExternalImageToTexture({ source, flipY: options.flipY ?? false }, { texture, premultipliedAlpha: options.premultipliedAlpha ?? false }, [size, size, 1]);
			options.after?.();

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

			const [x, y] = options.at ?? [size / 2, size / 2];
			const offset = y * bytesPerRow + x * 4;
			return pixels.subarray(offset, offset + 4);
		}

		test('copyExternalImageToTexture honours premultipliedAlpha for a 2d canvas', async () => {
			const device = await getDevice();
			ok(device, 'no webgpu device');

			const source = make2DSource(32, 'rgba(255, 0, 0, 0.5)');
			const straight = await copyAndReadBack(device, source, 32, { premultipliedAlpha: false });
			ok(pixelNear(straight, [255, 0, 0, 128]), `premultipliedAlpha: false, got ${describePixel(straight)}`);
			const premultiplied = await copyAndReadBack(device, source, 32, { premultipliedAlpha: true });
			ok(pixelNear(premultiplied, [128, 0, 0, 128]), `premultipliedAlpha: true, got ${describePixel(premultiplied)}`);
		});

		test('copyExternalImageToTexture flips a 2d canvas with flipY', async () => {
			const device = await getDevice();
			ok(device, 'no webgpu device');

			const { canvas, ctx } = make2D(32, 32);
			ctx.fillStyle = 'red';
			ctx.fillRect(0, 0, 32, 16);
			ctx.fillStyle = 'blue';
			ctx.fillRect(0, 16, 32, 16);
			const top = await copyAndReadBack(device, canvas, 32, { flipY: true, at: [16, 4] });
			ok(pixelNear(top, BLUE), `expected blue on top, got ${describePixel(top)}`);
		});

		test('copyExternalImageToTexture copies a 2d canvas as it was when called', async () => {
			const device = await getDevice();
			ok(device, 'no webgpu device');

			const { canvas, ctx } = make2D(32, 32);
			ctx.fillStyle = 'red';
			ctx.fillRect(0, 0, 32, 32);
			const pixel = await copyAndReadBack(device, canvas, 32, {
				after: () => {
					ctx.fillStyle = '#00ff00';
					ctx.fillRect(0, 0, 32, 32);
				},
			});
			ok(pixelNear(pixel, RED), `expected red, got ${describePixel(pixel)}`);
		});

		test('copyExternalImageToTexture accepts a 2d canvas', async () => {
			const device = await getDevice();
			ok(device, 'no webgpu device');

			const source = make2DSource(32, 'red');
			const pixel = await copyAndReadBack(device, source);
			ok(pixelNear(pixel, RED), `expected red, got ${describePixel(pixel)}`);
		});

		test('copyExternalImageToTexture keeps many 2d canvases apart across repeated uploads', async () => {
			const device = await getDevice();
			ok(device, 'no webgpu device');

			const sources = Array.from({ length: 6 }, () => make2D(32, 32));
			for (let round = 0; round < 3; round++) {
				for (let i = 0; i < sources.length; i++) {
					const expected: [number, number, number, number] = [i * 40, round * 100, 200, 255];
					const { canvas, ctx } = sources[i];
					ctx.fillStyle = `rgb(${expected[0]}, ${expected[1]}, ${expected[2]})`;
					ctx.fillRect(0, 0, 32, 32);
					const pixel = await copyAndReadBack(device, canvas);
					ok(pixelNear(pixel, expected), `canvas ${i}, round ${round}: expected ${describePixel(expected)}, got ${describePixel(pixel)}`);
				}
			}
		});

		test('copyExternalImageToTexture matches getImageData for many redrawn 2d canvases', async () => {
			const device = await getDevice();
			ok(device, 'no webgpu device');

			// Like PixiJS text: many odd sized canvases, redrawn and uploaded back to back with no sync in between.
			const draw = (ctx: any, i: number, round: number, width: number, height: number) => {
				ctx.clearRect(0, 0, width, height);
				ctx.fillStyle = `rgb(${(i * 20) % 256}, ${round * 90}, 160)`;
				ctx.fillRect(2, 2, width / 2, height - 4);
				ctx.fillStyle = 'white';
				ctx.font = '14px sans-serif';
				ctx.fillText(`T${i}r${round}`, 4, height - 6);
			};
			const sources = Array.from({ length: 12 }, (_, i) => make2D(40 + i * 7, 20 + i * 3));
			const copies: { texture: any; readback: any; bytesPerRow: number; bgra: boolean; width: number; height: number; i: number; round: number }[] = [];
			for (let round = 0; round < 5; round++) {
				// Frames go by between rounds, as in a render loop.
				await new Promise((resolve) => requestAnimationFrame(resolve));
				for (let i = 0; i < sources.length; i++) {
					const { canvas, ctx } = sources[i];
					// PixiJS resizes a text's canvas whenever its text changes.
					canvas.width = 40 + i * 7 + round * 5;
					canvas.height = 20 + i * 3 + (round % 2);
					const { width, height } = canvas;
					draw(ctx, i, round, width, height);
					const bgra = i % 2 === 1;
					const texture = device.createTexture({
						size: [width, height, 1],
						format: bgra ? 'bgra8unorm' : 'rgba8unorm',
						usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC | GPUTextureUsage.RENDER_ATTACHMENT,
					});
					device.queue.copyExternalImageToTexture({ source: canvas }, { texture, premultipliedAlpha: true }, [width, height, 1]);
					const bytesPerRow = Math.ceil((width * 4) / 256) * 256;
					const readback = device.createBuffer({ size: bytesPerRow * height, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
					copies.push({ texture, readback, bytesPerRow, bgra, width, height, i, round });
				}
			}
			const encoder = device.createCommandEncoder();
			for (const { texture, readback, bytesPerRow, width, height } of copies) {
				encoder.copyTextureToBuffer({ texture }, { buffer: readback, bytesPerRow }, { width, height, depthOrArrayLayers: 1 });
			}
			device.queue.submit([encoder.finish()]);

			for (const { texture, readback, bytesPerRow, bgra, width, height, i, round } of copies) {
				await readback.mapAsync(GPUMapMode.READ);
				const pixels = new Uint8Array(readback.getMappedRange().slice(0));
				readback.unmap();
				readback.destroy();
				texture.destroy();
				const reference = make2D(width, height).ctx;
				draw(reference, i, round, width, height);
				const expected = reference.getImageData(0, 0, width, height).data;
				const label = `canvas ${i}, round ${round}`;

				let bad = 0;
				let first = '';
				for (let y = 0; y < height; y++) {
					for (let x = 0; x < width; x++) {
						const e = (y * width + x) * 4;
						const a = expected[e + 3];
						const want = [Math.round((expected[e] * a) / 255), Math.round((expected[e + 1] * a) / 255), Math.round((expected[e + 2] * a) / 255), a];
						const o = y * bytesPerRow + x * 4;
						const got = bgra ? [pixels[o + 2], pixels[o + 1], pixels[o], pixels[o + 3]] : pixels.subarray(o, o + 4);
						if (!pixelNear(got, want, 12)) {
							if (!bad) {
								first = `at ${x},${y} expected ${describePixel(want)}, got ${describePixel(got)}`;
							}
							bad++;
						}
					}
				}
				ok(bad === 0, `${label}: ${bad} of ${width * height} pixels differ, first ${first}`);
			}
		});

		test('copyExternalImageToTexture waits for each 2d canvas, not a later one', async () => {
			const device = await getDevice();
			ok(device, 'no webgpu device');

			// A slow canvas copied first and a quick one after: the first copy must not finish early.
			const slow = make2D(1024, 1024);
			const quick = make2D(16, 16);
			const pending: { texture: any; readback: any; expected: number[]; label: string }[] = [];
			for (let round = 0; round < 6; round++) {
				const { ctx } = slow;
				ctx.clearRect(0, 0, 1024, 1024);
				ctx.shadowColor = 'rgba(0, 0, 0, 0.5)';
				ctx.shadowBlur = 40;
				for (let j = 0; j < 60; j++) {
					ctx.fillStyle = `rgb(${j * 4}, 80, 200)`;
					ctx.beginPath();
					ctx.arc((j * 97) % 1024, (j * 61) % 1024, 120, 0, Math.PI * 2);
					ctx.fill();
				}
				ctx.shadowBlur = 0;
				const expected = [round * 40, 255 - round * 40, 60, 255];
				ctx.fillStyle = `rgb(${expected[0]}, ${expected[1]}, ${expected[2]})`;
				ctx.fillRect(480, 480, 64, 64);
				const texture = device.createTexture({
					size: [1024, 1024, 1],
					format: 'rgba8unorm',
					usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC | GPUTextureUsage.RENDER_ATTACHMENT,
				});
				device.queue.copyExternalImageToTexture({ source: slow.canvas }, { texture, premultipliedAlpha: true }, [1024, 1024, 1]);

				quick.ctx.fillStyle = 'red';
				quick.ctx.fillRect(0, 0, 16, 16);
				const other = device.createTexture({
					size: [16, 16, 1],
					format: 'rgba8unorm',
					usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.RENDER_ATTACHMENT,
				});
				device.queue.copyExternalImageToTexture({ source: quick.canvas }, { texture: other, premultipliedAlpha: true }, [16, 16, 1]);

				const readback = device.createBuffer({ size: 256, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
				const encoder = device.createCommandEncoder();
				encoder.copyTextureToBuffer({ texture, origin: { x: 512, y: 512 } }, { buffer: readback, bytesPerRow: 256 }, { width: 1, height: 1, depthOrArrayLayers: 1 });
				device.queue.submit([encoder.finish()]);
				pending.push({ texture, readback, expected, label: `round ${round}` });
			}
			for (const { texture, readback, expected, label } of pending) {
				await readback.mapAsync(GPUMapMode.READ);
				const pixel = new Uint8Array(readback.getMappedRange().slice(0, 4));
				readback.unmap();
				readback.destroy();
				texture.destroy();
				ok(pixelNear(pixel, expected), `${label}: expected ${describePixel(expected)}, got ${describePixel(pixel)}`);
			}
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
