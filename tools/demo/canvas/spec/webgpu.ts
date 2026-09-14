/**
 * WebGPU: adapter/device acquisition, canvas context configuration, and render
 * passes read back through a buffer. Every test is async and needs a device.
 */

import { suite, test, ok, equal, notEqual, arrayEqual, rejects, makeCanvas, getPageCanvas } from './harness';

declare const navigator: any;

let devicePromise: Promise<any> | null = null;

async function getDevice() {
	if (!devicePromise) {
		devicePromise = (async () => {
			if (!navigator?.gpu) {
				throw new Error('navigator.gpu is missing');
			}
			const adapter = await navigator.gpu.requestAdapter();
			if (!adapter) {
				throw new Error('requestAdapter() returned null');
			}
			const device = await adapter.requestDevice();
			if (!device) {
				throw new Error('requestDevice() returned null');
			}
			return { adapter, device };
		})();
	}
	return devicePromise;
}

export function registerWebGPUSpec() {
	suite('webgpu.adapter', () => {
		test('navigator.gpu exists', () => {
			ok(navigator?.gpu, 'navigator.gpu is missing');
		});

		test('getPreferredCanvasFormat returns a spec format', () => {
			const format = navigator.gpu.getPreferredCanvasFormat();
			ok(format === 'bgra8unorm' || format === 'rgba8unorm', `unexpected preferred format: ${format}`);
		});

		test('requestAdapter resolves to an adapter', async () => {
			const { adapter } = await getDevice();
			ok(adapter, 'no adapter');
			ok(adapter.limits, 'adapter.limits is missing');
			ok(adapter.features, 'adapter.features is missing');
		});

		test('adapter.limits reports the required minimums', async () => {
			const { adapter } = await getDevice();
			// Every WebGPU implementation must meet at least the default limits.
			ok(adapter.limits.maxTextureDimension2D >= 8192, `maxTextureDimension2D is ${adapter.limits.maxTextureDimension2D}`);
			ok(adapter.limits.maxBindGroups >= 4, `maxBindGroups is ${adapter.limits.maxBindGroups}`);
			ok(adapter.limits.maxBufferSize >= 268435456, `maxBufferSize is ${adapter.limits.maxBufferSize}`);
		});

		test('adapter.features is a set-like', async () => {
			const { adapter } = await getDevice();
			equal(typeof adapter.features.has, 'function', 'features.has');
			equal(adapter.features.has('not-a-real-feature'), false, 'an unknown feature must not be present');
		});

		test('requestDevice resolves to a device with a queue', async () => {
			const { device } = await getDevice();
			ok(device, 'no device');
			ok(device.queue, 'device.queue is missing');
			equal(typeof device.queue.submit, 'function', 'queue.submit');
			equal(typeof device.createBuffer, 'function', 'createBuffer');
		});
	});

	// A WebGPU surface needs a real window, so these run against the page's own
	// canvas rather than an offscreen one. That canvas can only hand out one
	// context, so the whole group shares it.
	suite('webgpu.canvas', () => {
		function surfaceCanvas() {
			const canvas = getPageCanvas();
			if (!canvas) {
				throw new Error('no on-screen canvas: WebGPU needs a real surface');
			}
			return canvas;
		}

		test('getContext("webgpu") returns a context', async () => {
			await getDevice();
			const ctx = surfaceCanvas().getContext('webgpu') as any;
			ok(ctx, 'no webgpu context');
			equal(typeof ctx.configure, 'function', 'configure');
			equal(typeof ctx.getCurrentTexture, 'function', 'getCurrentTexture');
		});

		test('getContext("webgpu") is idempotent', async () => {
			await getDevice();
			const canvas = surfaceCanvas();
			equal(canvas.getContext('webgpu'), canvas.getContext('webgpu'));
		});

		test('a webgpu canvas refuses a 2d context', async () => {
			await getDevice();
			const canvas = surfaceCanvas();
			ok(canvas.getContext('webgpu'), 'precondition');
			equal(canvas.getContext('2d'), null);
		});

		test('getCapabilities lists the surface formats', async () => {
			const { adapter } = await getDevice();
			const ctx = surfaceCanvas().getContext('webgpu') as any;
			const caps = ctx.getCapabilities(adapter);
			ok(caps, 'no capabilities');
			ok(Array.isArray(caps.format) || Array.isArray(caps.formats), 'formats should be a list');
		});

		test('configure then getCurrentTexture returns a texture of the canvas size', async () => {
			const { device } = await getDevice();
			const canvas = surfaceCanvas();
			const ctx = canvas.getContext('webgpu') as any;
			ctx.configure({ device, format: navigator.gpu.getPreferredCanvasFormat(), alphaMode: 'premultiplied' });
			const texture = ctx.getCurrentTexture();
			ok(texture, 'no current texture');
			equal(texture.width, (canvas as any).width, 'width');
			equal(texture.height, (canvas as any).height, 'height');
			ok(texture.createView(), 'createView');
		});
	});

	suite('webgpu.device', () => {
		test('createBuffer reports its size and usage', async () => {
			const { device } = await getDevice();
			const buffer = device.createBuffer({ size: 256, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
			ok(buffer, 'no buffer');
			equal(Number(buffer.size), 256, 'size');
			buffer.destroy();
		});

		test('writeBuffer then map reads the bytes back', async () => {
			const { device } = await getDevice();
			const size = 16;
			const upload = device.createBuffer({ size, usage: GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST });
			const readback = device.createBuffer({ size, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });

			const source = new Uint8Array(size);
			for (let i = 0; i < size; i++) {
				source[i] = i * 3;
			}
			device.queue.writeBuffer(upload, 0, source);

			const encoder = device.createCommandEncoder();
			encoder.copyBufferToBuffer(upload, 0, readback, 0, size);
			device.queue.submit([encoder.finish()]);

			await readback.mapAsync(GPUMapMode.READ);
			const mapped = new Uint8Array(readback.getMappedRange().slice(0));
			readback.unmap();
			arrayEqual(mapped, Array.from(source) as any);
		});

		test('a shader module compiles', async () => {
			const { device } = await getDevice();
			const module = device.createShaderModule({
				code: `
@vertex fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4f {
  var p = array(vec2f(-1.0, -1.0), vec2f(3.0, -1.0), vec2f(-1.0, 3.0));
  return vec4f(p[i], 0.0, 1.0);
}
@fragment fn fs() -> @location(0) vec4f { return vec4f(0.0, 1.0, 0.0, 1.0); }
`,
			});
			ok(module, 'no shader module');
			const info = await module.getCompilationInfo?.();
			if (info) {
				const errors = (info.messages ?? []).filter((m: any) => m.type === 'error');
				equal(errors.length, 0, errors.map((m: any) => m.message).join('; '));
			}
		});

		test('a render pass clears a texture to an exact colour', async () => {
			const { device } = await getDevice();
			const size = 4;
			const texture = device.createTexture({
				size: { width: size, height: size, depthOrArrayLayers: 1 },
				format: 'rgba8unorm',
				usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
			});

			const encoder = device.createCommandEncoder();
			const pass = encoder.beginRenderPass({
				colorAttachments: [
					{
						view: texture.createView(),
						clearValue: { r: 1, g: 0, b: 0, a: 1 },
						loadOp: 'clear',
						storeOp: 'store',
					},
				],
			});
			pass.end();

			// 256-byte row alignment is required by copyTextureToBuffer.
			const bytesPerRow = 256;
			const readback = device.createBuffer({ size: bytesPerRow * size, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
			encoder.copyTextureToBuffer({ texture }, { buffer: readback, bytesPerRow }, { width: size, height: size, depthOrArrayLayers: 1 });
			device.queue.submit([encoder.finish()]);

			await readback.mapAsync(GPUMapMode.READ);
			const pixels = new Uint8Array(readback.getMappedRange().slice(0));
			readback.unmap();
			arrayEqual(pixels.subarray(0, 4), [255, 0, 0, 255]);
		});

		test('a full render pipeline draws a green triangle', async () => {
			const { device } = await getDevice();
			const size = 4;
			const module = device.createShaderModule({
				code: `
@vertex fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4f {
  var p = array(vec2f(-1.0, -1.0), vec2f(3.0, -1.0), vec2f(-1.0, 3.0));
  return vec4f(p[i], 0.0, 1.0);
}
@fragment fn fs() -> @location(0) vec4f { return vec4f(0.0, 1.0, 0.0, 1.0); }
`,
			});

			const pipeline = device.createRenderPipeline({
				layout: 'auto',
				vertex: { module, entryPoint: 'vs' },
				fragment: { module, entryPoint: 'fs', targets: [{ format: 'rgba8unorm' }] },
				primitive: { topology: 'triangle-list' },
			});
			ok(pipeline, 'no pipeline');

			const texture = device.createTexture({
				size: { width: size, height: size, depthOrArrayLayers: 1 },
				format: 'rgba8unorm',
				usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
			});

			const encoder = device.createCommandEncoder();
			const pass = encoder.beginRenderPass({
				colorAttachments: [
					{
						view: texture.createView(),
						clearValue: { r: 0, g: 0, b: 0, a: 1 },
						loadOp: 'clear',
						storeOp: 'store',
					},
				],
			});
			pass.setPipeline(pipeline);
			pass.draw(3);
			pass.end();

			const bytesPerRow = 256;
			const readback = device.createBuffer({ size: bytesPerRow * size, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
			encoder.copyTextureToBuffer({ texture }, { buffer: readback, bytesPerRow }, { width: size, height: size, depthOrArrayLayers: 1 });
			device.queue.submit([encoder.finish()]);

			await readback.mapAsync(GPUMapMode.READ);
			const pixels = new Uint8Array(readback.getMappedRange().slice(0));
			readback.unmap();
			arrayEqual(pixels.subarray(0, 4), [0, 255, 0, 255]);
		});

		test('an invalid pipeline surfaces through popErrorScope', async () => {
			const { device } = await getDevice();
			if (typeof device.pushErrorScope !== 'function') {
				throw new Error('device.pushErrorScope is missing');
			}
			device.pushErrorScope('validation');
			try {
				device.createRenderPipeline({
					layout: 'auto',
					vertex: { module: device.createShaderModule({ code: `@vertex fn vs() -> @builtin(position) vec4f { return vec4f(0.0); }` }), entryPoint: 'nope' },
					fragment: undefined,
				});
			} catch (e) {
				// Some implementations throw immediately; either is acceptable.
			}
			const error = await device.popErrorScope();
			ok(error, 'a validation error was expected from an unknown entry point');
		});
	});

	suite('webgpu.video', () => {
		/** A colour no frame of the test clip contains, so "unchanged" is unambiguous. */
		const SENTINEL = [255, 0, 255, 255];

		/**
		 * Upload the video's current frame until one lands, then report what arrived.
		 *
		 * The destination is cleared to a sentinel before each attempt, so a frame counts
		 * as having arrived when the pixels stop being the sentinel. Testing for
		 * "not black" instead would hang on a clip that opens on a dark frame.
		 */
		async function uploadUntilFrame(device: any, video: any, size = 64) {
			const texture = device.createTexture({
				size: [size, size, 1],
				format: 'rgba8unorm',
				// RENDER_ATTACHMENT is required of copyExternalImageToTexture destinations
				// by the spec, and is what lets the zero-copy path blit into this texture.
				usage: GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC | GPUTextureUsage.RENDER_ATTACHMENT,
			});

			const bytesPerRow = 256;
			const readback = device.createBuffer({
				size: bytesPerRow * size,
				usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
			});

			for (let attempt = 0; attempt < 150; attempt++) {
				const clear = device.createCommandEncoder();
				clear
					.beginRenderPass({
						colorAttachments: [
							{
								view: texture.createView(),
								clearValue: { r: 1, g: 0, b: 1, a: 1 },
								loadOp: 'clear',
								storeOp: 'store',
							},
						],
					})
					.end();
				device.queue.submit([clear.finish()]);

				device.queue.copyExternalImageToTexture({ source: video }, { texture }, [size, size, 1]);

				const encoder = device.createCommandEncoder();
				encoder.copyTextureToBuffer({ texture }, { buffer: readback, bytesPerRow }, { width: size, height: size, depthOrArrayLayers: 1 });
				device.queue.submit([encoder.finish()]);

				await readback.mapAsync(GPUMapMode.READ);
				const pixels = new Uint8Array(readback.getMappedRange().slice(0));
				readback.unmap();

				for (let i = 0; i < pixels.length; i += 4) {
					if (pixels[i] !== SENTINEL[0] || pixels[i + 1] !== SENTINEL[1] || pixels[i + 2] !== SENTINEL[2]) {
						return { pixels, attempt };
					}
				}

				await new Promise((resolve) => setTimeout(resolve, 32));
			}

			return null;
		}

		async function makeVideo() {
			const video: any = document.createElement('video');
			video.loop = true;
			video.muted = true;
			video.autoplay = true;
			video.src = '~/assets/file-assets/webgpu/pano.mp4';
			try {
				await video.play();
			} catch (e) {
				// Playback may resolve late or not at all on some devices; the upload
				// loop below is what actually decides whether frames arrive.
			}
			return video;
		}

		/** Say exactly which link in the zero-copy chain is missing, not just that it is. */
		function reportPath(device: any, video: any) {
			const inner = video._video;
			let detail = '';

			if (__ANDROID__) {
				let utils: any;
				try {
					utils = (global as any).org?.nativescript?.canvas?.Utils;
				} catch (e) {}
				detail = ` Utils=${!!utils} bridge=${typeof utils?.hardwareBufferPointer} helper=${typeof inner?._instance?.supportsGPUFrames}`;
				try {
					detail += ` helperSays=${inner?._instance?.supportsGPUFrames()}`;
				} catch (e) {
					detail += ` helperSays=threw:${e}`;
				}
			} else {
				detail = ` metalDevice=${device.__metalDevice ?? 0}`;
			}

			const zeroCopy = !!inner?.supportsGPUFrames?.(device.__metalDevice ?? 0);
			console.log(`SPEC|info|webgpu.video zero-copy ${zeroCopy ? 'active' : 'unavailable'};${detail}`);
			return zeroCopy;
		}

		test('a decoded video frame reaches a texture', async () => {
			const { device } = await getDevice();
			const video = await makeVideo();

			const started = Date.now();
			const result = await uploadUntilFrame(device, video);
			reportPath(device, video);
			ok(result, 'no video frame reached the texture within ~5s');
			console.log(`SPEC|info|webgpu.video first frame after ${result.attempt + 1} upload(s), ${Date.now() - started}ms`);
		});

		test('repeated uploads keep producing frames', async () => {
			const { device } = await getDevice();
			const video = await makeVideo();

			const first = await uploadUntilFrame(device, video);
			ok(first, 'no first frame');

			// A frame the decoder never released — an ImageReader starved of buffers on
			// Android, say — shows up as the second upload never completing.
			const second = await uploadUntilFrame(device, video);
			ok(second, 'a second frame never arrived: the decoder may be starved of buffers');
		});
	});
}
