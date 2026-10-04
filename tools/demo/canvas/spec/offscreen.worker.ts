import '@nativescript/canvas/worker';
import { OffscreenCanvas } from '@nativescript/canvas/worker';

declare const self: any;
declare const navigator: any;
declare function requestAnimationFrame(callback: (time: number) => void): number;

function canvasFor(args: any): OffscreenCanvas {
	if (args?.canvas) {
		return args.canvas;
	}
	return args?.handle ? OffscreenCanvas._fromHandle(args.handle) : new OffscreenCanvas(args?.width ?? 8, args?.height ?? 8);
}

function pixel2D(ctx: any, x: number, y: number): number[] {
	const d = ctx.getImageData(x, y, 1, 1).data;
	return [d[0], d[1], d[2], d[3]];
}

function pixelOf(source: any, width: number, height: number, x: number, y: number): number[] {
	const ctx = new OffscreenCanvas(width, height).getContext('2d') as any;
	ctx.drawImage(source, 0, 0);
	return pixel2D(ctx, x, y);
}

let looping: { ctx: any; frames: number } | null = null;

let held: { canvas: OffscreenCanvas; ctx: any } | null = null;

const ops: Record<string, (args: any) => any> = {
	'2d'(args) {
		const canvas = canvasFor(args);
		const ctx = canvas.getContext('2d', { threaded: args?.threaded }) as any;
		ctx.fillStyle = 'red';
		ctx.fillRect(0, 0, canvas.width, canvas.height);
		ctx.fillStyle = 'blue';
		ctx.fillRect(0, 0, 2, 2);
		return {
			width: canvas.width,
			height: canvas.height,
			corner: pixel2D(ctx, 1, 1),
			rest: pixel2D(ctx, canvas.width - 1, canvas.height - 1),
		};
	},

	resize(args) {
		const canvas = canvasFor(args);
		const ctx = canvas.getContext('2d') as any;
		canvas.width = 20;
		canvas.height = 10;
		ctx.fillStyle = 'lime';
		ctx.fillRect(0, 0, 20, 10);
		return { width: canvas.width, height: canvas.height, pixel: pixel2D(ctx, 19, 9), imageWidth: ctx.getImageData(0, 0, 20, 10).width };
	},

	webgl(args) {
		const canvas = canvasFor(args);
		const gl = canvas.getContext(args?.version ?? 'webgl', { preserveDrawingBuffer: !!args?.preserve }) as any;
		if (!gl) {
			return { context: false };
		}
		gl.viewport(0, 0, canvas.width, canvas.height);
		gl.clearColor(0, 1, 0, 1);
		gl.clear(gl.COLOR_BUFFER_BIT);
		const pixel = new Uint8Array(4);
		gl.readPixels(0, 0, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixel);
		return { context: true, pixel: Array.from(pixel), drawn: pixelOf(canvas, canvas.width, canvas.height, 2, 2) };
	},

	bitmaprenderer(args) {
		const source = new OffscreenCanvas(4, 4);
		const sctx = source.getContext('2d') as any;
		sctx.fillStyle = 'red';
		sctx.fillRect(0, 0, 4, 4);
		const bitmap = source.transferToImageBitmap();
		const canvas = canvasFor({ width: 4, height: 4, ...args });
		const ctx = canvas.getContext('bitmaprenderer') as any;
		ctx.transferFromImageBitmap(bitmap);
		return { pixel: pixelOf(canvas, 4, 4, 2, 2) };
	},

	async webgpu(args) {
		if (!navigator?.gpu) {
			return { skipped: 'no navigator.gpu' };
		}
		const adapter = await navigator.gpu.requestAdapter();
		if (!adapter) {
			return { skipped: 'no adapter' };
		}
		const device = await adapter.requestDevice();
		const canvas = canvasFor(args);
		const context = canvas.getContext('webgpu') as any;
		if (!context) {
			return { context: false };
		}
		context.configure({ device, format: navigator.gpu.getPreferredCanvasFormat(), alphaMode: 'premultiplied' });
		const encoder = device.createCommandEncoder();
		const pass = encoder.beginRenderPass({
			colorAttachments: [{ view: context.getCurrentTexture().createView(), clearValue: { r: 0, g: 0, b: 1, a: 1 }, loadOp: 'clear', storeOp: 'store' }],
		});
		pass.end();
		device.queue.submit([encoder.finish()]);
		const bitmap = canvas.transferToImageBitmap();
		return { context: true, width: bitmap.width, pixel: pixelOf(bitmap, canvas.width, canvas.height, 2, 2) };
	},

	async convertToBlob(args) {
		const canvas = canvasFor(args);
		const ctx = canvas.getContext('2d') as any;
		ctx.fillRect(0, 0, canvas.width, canvas.height);
		const blob = await canvas.convertToBlob();
		return { type: blob.type, size: blob.size };
	},

	async frames() {
		if (typeof requestAnimationFrame !== 'function') {
			return { frames: -1 };
		}
		let frames = 0;
		const start = Date.now();
		await new Promise<void>((resolve) => {
			const tick = () => {
				frames++;
				if (Date.now() - start > 300) {
					resolve();
				} else {
					requestAnimationFrame(tick);
				}
			};
			requestAnimationFrame(tick);
		});
		return { frames };
	},

	adoptTwice(args) {
		OffscreenCanvas._fromHandle(args.handle);
		try {
			OffscreenCanvas._fromHandle(args.handle);
			return { threw: false };
		} catch (e) {
			return { threw: true, name: e?.name };
		}
	},

	hold(args) {
		const canvas = canvasFor(args);
		held = { canvas, ctx: canvas.getContext('2d') };
		return { width: canvas.width };
	},

	drawHeld(args) {
		if (!held) {
			throw new Error('nothing held');
		}
		held.ctx.fillStyle = args?.color ?? 'red';
		held.ctx.fillRect(0, 0, held.canvas.width, held.canvas.height);
		return { pixel: pixel2D(held.ctx, 1, 1) };
	},

	sendBack(args) {
		const canvas = new OffscreenCanvas(args?.width ?? 8, args?.height ?? 8);
		return { canvas, received: args?.canvas instanceof OffscreenCanvas, transfer: [canvas] };
	},

	loop(args) {
		const canvas = canvasFor(args);
		looping = { ctx: canvas.getContext('2d'), frames: 0 };
		const tick = () => {
			if (!looping) {
				return;
			}
			looping.frames++;
			looping.ctx.fillStyle = looping.frames % 2 ? 'red' : 'blue';
			looping.ctx.fillRect(0, 0, canvas.width, canvas.height);
			requestAnimationFrame(tick);
		};
		requestAnimationFrame(tick);
		return { started: true };
	},
};

self.onmessage = async (event: any) => {
	const { id, op, args } = event.data ?? {};
	try {
		const run = ops[op];
		if (!run) {
			throw new Error(`unknown op ${op}`);
		}
		const result = await run(args);
		self.postMessage({ id, ok: true, result }, result?.transfer);
	} catch (e) {
		self.postMessage({ id, ok: false, name: e?.name ?? 'Error', message: e?.message ?? String(e) });
	}
};
