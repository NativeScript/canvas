import { Canvas } from '@nativescript/canvas';
import { GridLayout, Utils } from '@nativescript/core';
import { suite, test, ok, equal, getPageCanvas, pixelEqual } from './harness';

function wait(ms: number) {
	return new Promise<void>((resolve) => setTimeout(resolve, ms));
}

function frames(count: number) {
	return new Promise<void>((resolve) => {
		const step = () => (--count <= 0 ? resolve() : requestAnimationFrame(step));
		requestAnimationFrame(step);
	});
}

async function mount(): Promise<Canvas> {
	const parent = getPageCanvas()?.parent as GridLayout;
	if (!parent) {
		throw new Error('no page layout to mount a canvas in');
	}
	const canvas = new Canvas();
	canvas.style.width = 32;
	canvas.style.height = 32;
	GridLayout.setRow(canvas, 1);
	const ready = new Promise<void>((resolve) => canvas.once('ready', () => resolve()));
	parent.addChild(canvas);
	await Promise.race([ready, wait(3000)]);
	await frames(2);
	return canvas;
}

function unmount(canvas: Canvas) {
	(canvas.parent as GridLayout)?.removeChild(canvas);
}

async function collect() {
	Utils.GC();
	await frames(3);
}

function glPixel(gl: any): number[] {
	const pixel = new Uint8Array(4);
	gl.readPixels(0, 0, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixel);
	return Array.from(pixel);
}

async function webglKeepsDrawing(type: 'webgl' | 'webgl2') {
	const canvas = await mount();
	const gl = canvas.getContext(type) as any;
	ok(gl, `no ${type} context`);
	gl.clearColor(1, 0, 0, 1);
	gl.clear(gl.COLOR_BUFFER_BIT);
	unmount(canvas);
	await frames(3);
	gl.clearColor(0, 1, 0, 1);
	gl.clear(gl.COLOR_BUFFER_BIT);
	equal(glPixel(gl).join(','), '0,255,0,255');
	await collect();
	gl.clear(gl.COLOR_BUFFER_BIT);
	equal(glPixel(gl).join(','), '0,255,0,255');
}

function renderFrame(device: any, ctx: any) {
	const texture = ctx.getCurrentTexture();
	ok(texture, 'no current texture');
	const encoder = device.createCommandEncoder();
	const pass = encoder.beginRenderPass({
		colorAttachments: [{ view: texture.createView(), clearValue: { r: 0, g: 1, b: 0, a: 1 }, loadOp: 'clear', storeOp: 'store' }],
	});
	pass.end();
	device.queue.submit([encoder.finish()]);
	return texture;
}

export function registerLifecycleSpec() {
	suite('lifecycle', () => {
		test('a 2d context keeps drawing after its canvas leaves the page', async () => {
			const canvas = await mount();
			const ctx = canvas.getContext('2d') as any;
			ok(ctx, 'no 2d context');
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 16, 16);
			unmount(canvas);
			await frames(3);
			ctx.fillStyle = '#00ff00';
			ctx.fillRect(0, 0, 16, 16);
			pixelEqual(ctx, 4, 4, [0, 255, 0, 255]);
			await collect();
			ctx.fillStyle = '#0000ff';
			ctx.fillRect(0, 0, 16, 16);
			pixelEqual(ctx, 4, 4, [0, 0, 255, 255]);
		});

		test('a webgl context keeps drawing after its canvas leaves the page', () => webglKeepsDrawing('webgl'));

		test('a webgl2 context keeps drawing after its canvas leaves the page', () => webglKeepsDrawing('webgl2'));

		test('a webgpu context keeps rendering after its canvas leaves the page', async () => {
			const adapter = await navigator.gpu.requestAdapter();
			const device = await adapter.requestDevice();
			const format = navigator.gpu.getPreferredCanvasFormat();
			const canvas = await mount();
			const ctx = canvas.getContext('webgpu') as any;
			ok(ctx, 'no webgpu context');
			ctx.configure({ device, format, alphaMode: 'premultiplied' });
			renderFrame(device, ctx);
			await frames(1);
			const width = (canvas as any).width;
			unmount(canvas);
			for (let i = 0; i < 3; i++) {
				equal(renderFrame(device, ctx).width, width, 'width');
				await frames(1);
			}
			ok(ctx.getCapabilities(adapter)?.format?.length, 'no formats once detached');
			ctx.configure({ device, format, alphaMode: 'premultiplied' });
			renderFrame(device, ctx);
			await collect();
			renderFrame(device, ctx);
		});

		test('a webgpu context survives its surface going away and coming back', async () => {
			const adapter = await navigator.gpu.requestAdapter();
			const device = await adapter.requestDevice();
			const canvas = await mount();
			const ctx = canvas.getContext('webgpu') as any;
			ctx.configure({ device, format: navigator.gpu.getPreferredCanvasFormat(), alphaMode: 'premultiplied' });
			renderFrame(device, ctx);
			canvas.visibility = 'collapse';
			await frames(3);
			renderFrame(device, ctx);
			await frames(1);
			canvas.visibility = 'visible';
			await frames(3);
			renderFrame(device, ctx);
			await frames(1);
			unmount(canvas);
		});

		test('contexts dropped with their canvas are collected safely', async () => {
			for (const type of ['2d', 'webgl', 'webgl2']) {
				const canvas = await mount();
				const ctx = canvas.getContext(type) as any;
				ok(ctx, `no ${type} context`);
				unmount(canvas);
			}
			await collect();
			await wait(200);
			await collect();
		});
	});
}
