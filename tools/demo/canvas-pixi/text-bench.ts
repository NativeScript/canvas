import { Canvas } from '@nativescript/canvas';
import { Application, DOMAdapter, Text } from 'pixi.js';
import '@nativescript/canvas-pixi';

/**
 * What a 2D canvas costs as a texture source, in the shape PixiJS uses for Text.
 *
 * TXUP|<sink>|<mode>|<w>x<h>|draw|<us/iter>
 * TXUP|<sink>|<mode>|<w>x<h>|draw+upload|<us/iter>
 * TXUP|<sink>|<mode>|<w>x<h>|upload|<us/iter>      (the difference)
 *
 * mode `gpu` is a default 2D context, `direct` is `{ threaded: false }`, `cpu` is willReadFrequently.
 */
export async function runTextUploadBench(canvas: any, sink: 'webgl' | 'webgl2' | 'webgpu') {
	const sizes: [number, number][] = [
		[128, 32],
		[256, 64],
		[512, 128],
		[1024, 256],
	];
	const iterations = 120;
	const runs = 5;

	const upload = await makeSink(canvas, sink);
	if (!upload) {
		console.log(`TXUP|${sink}|error|no context`);
		return;
	}

	for (const [w, h] of sizes) {
		for (const mode of ['gpu', 'direct', 'cpu'] as const) {
			const source = new Canvas();
			source.width = w;
			source.height = h;
			const ctx = source.getContext('2d', mode === 'cpu' ? { willReadFrequently: true } : mode === 'direct' ? { threaded: false } : undefined) as any;
			ctx.font = `${Math.round(h * 0.6)}px sans-serif`;
			ctx.fillStyle = '#ffffff';

			upload.allocate(w, h);

			let n = 0;
			const draw = () => {
				ctx.clearRect(0, 0, w, h);
				ctx.fillText('Score ' + n++, 4, h * 0.75);
			};

			// Warm both paths (shaping cache, surface creation, first upload).
			for (let i = 0; i < 10; i++) {
				draw();
				upload.upload(source);
			}
			await upload.finish();

			const drawOnly: number[] = [];
			const drawUpload: number[] = [];
			for (let r = 0; r < runs; r++) {
				let t0 = performance.now();
				for (let i = 0; i < iterations; i++) {
					draw();
				}
				drawOnly.push(((performance.now() - t0) * 1000) / iterations);

				t0 = performance.now();
				for (let i = 0; i < iterations; i++) {
					draw();
					upload.upload(source);
				}
				await upload.finish();
				drawUpload.push(((performance.now() - t0) * 1000) / iterations);
			}

			const d = median(drawOnly);
			const du = median(drawUpload);
			const tag = `TXUP|${sink}|${mode}|${w}x${h}`;
			console.log(`${tag}|draw|${d.toFixed(1)}`);
			console.log(`${tag}|draw+upload|${du.toFixed(1)}`);
			console.log(`${tag}|upload|${(du - d).toFixed(1)}`);
		}
	}
	console.log(`TXUP|${sink}|done`);
}

interface Sink {
	allocate(w: number, h: number): void;
	upload(source: any): void;
	finish(): Promise<void>;
}

async function makeSink(canvas: any, sink: 'webgl' | 'webgl2' | 'webgpu'): Promise<Sink | null> {
	if (sink === 'webgpu') {
		const adapter = await navigator.gpu?.requestAdapter();
		const device = await adapter?.requestDevice();
		if (!device) return null;
		let texture: GPUTexture | null = null;
		let size = [0, 0];
		return {
			allocate(w, h) {
				texture?.destroy();
				size = [w, h];
				texture = device.createTexture({
					size: { width: w, height: h },
					format: 'rgba8unorm',
					usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.RENDER_ATTACHMENT,
				});
			},
			upload(source) {
				// gpuUploadImageSource's call.
				device.queue.copyExternalImageToTexture({ source }, { texture, origin: { x: 0, y: 0, z: 0 }, premultipliedAlpha: true }, { width: size[0], height: size[1] });
			},
			finish() {
				return device.queue.onSubmittedWorkDone();
			},
		};
	}

	const gl = canvas.getContext(sink) as WebGLRenderingContext;
	if (!gl) return null;
	const texture = gl.createTexture();
	gl.bindTexture(gl.TEXTURE_2D, texture);
	return {
		allocate(w, h) {
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, w, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
		},
		upload(source) {
			// glUploadImageResource's call when the pooled texture already has the size.
			gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, gl.RGBA, gl.UNSIGNED_BYTE, source);
		},
		async finish() {
			gl.finish();
		},
	};
}

/**
 * A PixiJS scene whose Text objects change every frame.
 *
 * PXTEXT|<renderer>|<mode>|<count>|render-ms|<median>|p90|<p90>|fps|<fps>
 *
 * mode `gpu`: Pixi's own canvases as they are today; `direct`: its 2D contexts are `{ threaded: false }`;
 * `cpu`: they are willReadFrequently; `static`: the same texts never change (the scene without text uploads).
 */
export async function runPixiTextBench(canvas: any, preference: 'webgl' | 'webgpu', mode: 'gpu' | 'direct' | 'cpu' | 'static', count = 40, frames = 300) {
	if (mode === 'cpu' || mode === 'direct') {
		const extra = mode === 'cpu' ? { willReadFrequently: true } : { threaded: false };
		const adapter = DOMAdapter.get();
		const createCanvas = adapter.createCanvas.bind(adapter);
		adapter.createCanvas = (w?: number, h?: number) => {
			const c = createCanvas(w, h) as any;
			const getContext = c.getContext.bind(c);
			c.getContext = (type: string, options?: any) => getContext(type, type === '2d' ? { ...options, ...extra } : options);
			return c;
		};
	}

	const app = new Application();
	await app.init({
		canvas,
		preference,
		width: canvas.width,
		height: canvas.height,
		background: '#1099bb',
		autoStart: false,
	});
	app.ticker.stop();

	const texts: Text[] = [];
	const columns = 4;
	const cellW = canvas.width / columns;
	const cellH = canvas.height / Math.ceil(count / columns);
	for (let i = 0; i < count; i++) {
		const t = new Text({
			text: 'Score 0',
			style: { fontFamily: 'sans-serif', fontSize: Math.min(48, cellH * 0.5), fill: 0xffffff },
		});
		t.x = (i % columns) * cellW + 8;
		t.y = Math.floor(i / columns) * cellH + 8;
		app.stage.addChild(t);
		texts.push(t);
	}

	const renderType = (app.renderer as any).name ?? app.renderer.constructor?.name ?? preference;
	const tag = `PXTEXT|${renderType}|${mode}|${count}`;
	const warmup = 30;
	const render: number[] = [];
	const stamps: number[] = [];
	let frame = 0;

	await new Promise<void>((resolve) => {
		const tick = () => {
			frame++;
			if (mode !== 'static') {
				for (let i = 0; i < count; i++) {
					texts[i].text = 'Score ' + (frame * count + i);
				}
			}
			const t0 = performance.now();
			app.renderer.render(app.stage);
			const t1 = performance.now();
			if (frame > warmup) {
				render.push(t1 - t0);
				stamps.push(t1);
			}
			if (frame < warmup + frames) {
				requestAnimationFrame(tick);
			} else {
				resolve();
			}
		};
		requestAnimationFrame(tick);
	});

	const sorted = [...render].sort((a, b) => a - b);
	const p90 = sorted[Math.floor(sorted.length * 0.9)];
	const fps = ((stamps.length - 1) * 1000) / (stamps[stamps.length - 1] - stamps[0]);
	console.log(`${tag}|render-ms|${median(render).toFixed(2)}|p90|${p90.toFixed(2)}|fps|${fps.toFixed(1)}`);
	console.log(`PXTEXT|done`);
}

function median(values: number[]) {
	const s = [...values].sort((a, b) => a - b);
	const m = s.length >> 1;
	return s.length % 2 ? s[m] : (s[m - 1] + s[m]) / 2;
}
