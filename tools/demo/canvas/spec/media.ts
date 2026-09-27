import { AudioContext } from '@nativescript/audio-context';
import { suite, test, ok, equal, closeTo, rejects, make2D, makeCanvas, pixelAt } from './harness';

declare const document: any, navigator: any, GPUTextureUsage: any, GPUBufferUsage: any, GPUMapMode: any;

// ms-appx: the packaged app's own files, whatever `~/` resolves to.
const CLIP = 'ms-appx:///app/assets/file-assets/webgpu/pano.mp4';
const TONE = 'ms-appx:///app/assets/file-assets/audio/gs-16b-1c-44100hz.wav';
const MAGENTA = [255, 0, 255, 255];

function once(target: any, type: string, timeoutMs = 10000): Promise<void> {
	return new Promise((resolve, reject) => {
		let done = false;
		const timer = setTimeout(() => {
			done = true;
			reject(new Error(`no ${type} event within ${timeoutMs}ms`));
		}, timeoutMs);
		target.addEventListener(type, () => {
			if (!done) {
				done = true;
				clearTimeout(timer);
				resolve();
			}
		});
	});
}

function nextFrame(video: any, timeoutMs = 5000): Promise<void> {
	return new Promise((resolve, reject) => {
		const timer = setTimeout(() => reject(new Error(`no video frame within ${timeoutMs}ms`)), timeoutMs);
		video.requestVideoFrameCallback(() => {
			clearTimeout(timer);
			resolve();
		});
	});
}

const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

function rms(samples: Float32Array) {
	let sum = 0;
	for (let i = 0; i < samples.length; i++) sum += samples[i] * samples[i];
	return Math.sqrt(sum / samples.length);
}

function isMagenta(pixel: ArrayLike<number>) {
	return pixel[0] === MAGENTA[0] && pixel[1] === MAGENTA[1] && pixel[2] === MAGENTA[2];
}

/** The texture's top-left pixel after `encode` renders or copies into it. */
async function firstPixel(device: any, texture: any, size: number) {
	const bytesPerRow = 256;
	const readback = device.createBuffer({ size: bytesPerRow * size, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
	const encoder = device.createCommandEncoder();
	encoder.copyTextureToBuffer({ texture }, { buffer: readback, bytesPerRow }, { width: size, height: size, depthOrArrayLayers: 1 });
	device.queue.submit([encoder.finish()]);
	await readback.mapAsync(GPUMapMode.READ);
	const pixel = new Uint8Array(readback.getMappedRange().slice(0, 4));
	readback.unmap();
	return pixel;
}

const EXTERNAL_WGSL = `
@group(0) @binding(0) var frame: texture_external;
@group(0) @binding(1) var frameSampler: sampler;

@vertex
fn vs(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
	let corners = array(vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0), vec2<f32>(-1.0, 3.0));
	return vec4<f32>(corners[index], 0.0, 1.0);
}

@fragment
fn fs(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
	return textureSampleBaseClampToEdge(frame, frameSampler, position.xy / 64.0);
}
`;

let playing: Promise<any> | null = null;

/** One muted, looping clip for the suite, playing with a frame decoded. */
function playingVideo(): Promise<any> {
	playing ??= (async () => {
		const video = document.createElement('video');
		video.muted = true;
		video.loop = true;
		const loaded = once(video, 'loadeddata');
		video.src = CLIP;
		await loaded;
		await video.play();
		await nextFrame(video);
		return video;
	})();
	return playing;
}

export function registerMediaSpec() {
	suite('media.video', () => {
		test('canvas-polyfill backs <video> with canvas-media', () => {
			const video = document.createElement('video');
			ok(video._video, 'HTMLVideoElement._video');
			equal(video.canPlayType('video/mp4'), 'maybe');
			equal(video.canPlayType('video/mp4; codecs="avc1.42E01E"'), 'maybe');
			equal(video.canPlayType('video/x-nothing'), '');
		});

		test('loads, plays and reports time', async () => {
			const video = await playingVideo();
			const inner = video._video;
			ok(video.readyState >= 2, `readyState ${video.readyState}`);
			ok(inner.duration > 0, `duration ${inner.duration}`);
			ok(inner.videoWidth > 0 && inner.videoHeight > 0, `${inner.videoWidth}x${inner.videoHeight}`);
			await once(video, 'timeupdate');
			ok(video.currentTime > 0, `currentTime ${video.currentTime}`);
			equal(inner.paused, false, 'paused');
		});

		test('drawImage(video) draws the current frame', async () => {
			const video = await playingVideo();
			const { ctx } = make2D(64, 64);
			ctx.fillStyle = 'magenta';
			ctx.fillRect(0, 0, 64, 64);
			ctx.drawImage(video, 0, 0, 64, 64);
			const pixel = pixelAt(ctx, 32, 32);
			ok(!isMagenta(pixel), `the frame was not drawn: [${pixel.join(', ')}]`);
			equal(pixel[3], 255, 'opaque');
		});

		test('texImage2D(video) uploads the current frame', async () => {
			const video = await playingVideo();
			const canvas = makeCanvas(64, 64);
			const gl: any = canvas.getContext('webgl');
			ok(gl, 'webgl context');
			const texture = gl.createTexture();
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, video);
			equal(gl.getError(), gl.NO_ERROR, 'texImage2D error');
			const framebuffer = gl.createFramebuffer();
			gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
			gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0);
			equal(gl.checkFramebufferStatus(gl.FRAMEBUFFER), gl.FRAMEBUFFER_COMPLETE, 'framebuffer');
			const inner = video._video;
			const pixel = new Uint8Array(4);
			gl.readPixels(inner.videoWidth >> 1, inner.videoHeight >> 1, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixel);
			equal(pixel[3], 255, 'the texture holds the frame (opaque)');
		});

		test('captureFrame() returns an RGBA VideoFrame', async () => {
			const video = await playingVideo();
			const frame = video.captureFrame();
			ok(frame, 'a frame');
			ok(frame.codedWidth > 0 && frame.codedHeight > 0, `${frame.codedWidth}x${frame.codedHeight}`);
			equal(frame.format, 'RGBA');
			equal(frame.pixelData.length, frame.codedWidth * frame.codedHeight * 4, 'pixel bytes');
			frame.close();
		});

		test('copyExternalImageToTexture(video) uploads the current frame', async () => {
			const video = await playingVideo();
			const adapter = await navigator.gpu.requestAdapter();
			const device = await adapter.requestDevice();
			const size = 64;
			const texture = device.createTexture({
				size: [size, size, 1],
				format: 'rgba8unorm',
				usage: GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC | GPUTextureUsage.RENDER_ATTACHMENT,
			});
			const clear = device.createCommandEncoder();
			clear.beginRenderPass({ colorAttachments: [{ view: texture.createView(), clearValue: { r: 1, g: 0, b: 1, a: 1 }, loadOp: 'clear', storeOp: 'store' }] }).end();
			device.queue.submit([clear.finish()]);
			ok(video._video.supportsGPUFrames(device.__frameDevice), 'frames are shared with the device (zero-copy)');
			// Only a frame decoded since the last hand-out is copied.
			await nextFrame(video);
			device.queue.copyExternalImageToTexture({ source: video }, { texture }, [size, size, 1]);
			const pixel = await firstPixel(device, texture, size);
			ok(!isMagenta(pixel), `the frame did not reach the texture: [${Array.from(pixel).join(', ')}]`);
		});

		test('importExternalTexture(video) samples the current frame', async () => {
			const video = await playingVideo();
			const adapter = await navigator.gpu.requestAdapter();
			const device = await adapter.requestDevice();
			await nextFrame(video);
			const external = device.importExternalTexture({ source: video });
			ok(external, 'a GPUExternalTexture');
			const size = 64;
			const target = device.createTexture({ size: [size, size, 1], format: 'rgba8unorm', usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC });
			const module = device.createShaderModule({ code: EXTERNAL_WGSL });
			const pipeline = device.createRenderPipeline({
				layout: 'auto',
				vertex: { module, entryPoint: 'vs' },
				fragment: { module, entryPoint: 'fs', targets: [{ format: 'rgba8unorm' }] },
				primitive: { topology: 'triangle-list' },
			});
			const bindGroup = device.createBindGroup({
				layout: pipeline.getBindGroupLayout(0),
				entries: [
					{ binding: 0, resource: external },
					{ binding: 1, resource: device.createSampler() },
				],
			});
			const encoder = device.createCommandEncoder();
			const pass = encoder.beginRenderPass({ colorAttachments: [{ view: target.createView(), clearValue: { r: 1, g: 0, b: 1, a: 1 }, loadOp: 'clear', storeOp: 'store' }] });
			pass.setPipeline(pipeline);
			pass.setBindGroup(0, bindGroup);
			pass.draw(3);
			pass.end();
			device.queue.submit([encoder.finish()]);
			const pixel = await firstPixel(device, target, size);
			ok(!isMagenta(pixel), `the frame was not sampled: [${Array.from(pixel).join(', ')}]`);
			equal(pixel[3], 255, 'opaque');
		});

		test('seeking fires seeked at the new time', async () => {
			const video = await playingVideo();
			const seeked = once(video._video, 'seeked');
			video.currentTime = 1;
			await seeked;
			closeTo(video.currentTime, 1, 0.5, 'currentTime after seeking');
		});

		test('a missing source fires error and rejects play()', async () => {
			const video = document.createElement('video');
			const failed = once(video, 'error');
			video.src = 'ms-appx:///app/assets/missing.mp4';
			const played = video.play();
			await failed;
			await rejects(played, 'play() on a missing source');
		});
	});

	suite('media.audio', () => {
		test('loads, plays and pauses a clip', async () => {
			const audio = document.createElement('audio');
			const inner = audio._audio;
			ok(inner, 'HTMLAudioElement._audio');
			audio.muted = true;
			const loaded = once(audio, 'loadedmetadata');
			audio.src = TONE;
			await loaded;
			ok(inner.duration > 0, `duration ${inner.duration}`);
			const started = once(audio, 'playing');
			await audio.play();
			await started;
			equal(inner.paused, false, 'paused while playing');
			audio.pause();
			equal(inner.paused, true, 'paused after pause()');
		});

		test('createMediaElementSource routes the element into the graph', async () => {
			const audio = document.createElement('audio');
			audio.loop = true;
			const loaded = once(audio, 'loadedmetadata');
			audio.src = TONE;
			await loaded;
			const context = new AudioContext();
			const source = context.createMediaElementSource(audio);
			equal(source.mediaElement, audio, 'mediaElement');
			const analyser = context.createAnalyser();
			// Rendered, but not heard.
			const silence = context.createGain();
			silence.gain.value = 0;
			source.connect(analyser);
			analyser.connect(silence);
			silence.connect(context.destination);
			await context.resume();
			await audio.play();
			await wait(800);
			const samples = new Float32Array(analyser.fftSize);
			analyser.getFloatTimeDomainData(samples);
			const level = rms(samples);
			audio.pause();
			source.disposeMediaElementSource();
			await context.close();
			const tapped = audio._audio?._media?._tap?.framesTapped;
			ok(level > 0.01, `the element's audio did not reach the graph (rms ${level.toFixed(4)}, frames tapped ${tapped})`);
		});
	});
}
