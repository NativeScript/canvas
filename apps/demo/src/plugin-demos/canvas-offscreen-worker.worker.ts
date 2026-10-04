import '@nativescript/canvas/worker';
import { OffscreenCanvas } from '@nativescript/canvas/worker';

declare const self: any;
declare function requestAnimationFrame(callback: (time: number) => void): number;

function report(kind: string, canvas: OffscreenCanvas, frames: number) {
	if (frames % 120 === 1) {
		self.postMessage({ kind, frames, width: canvas.width, height: canvas.height });
	}
}

function draw2D(canvas: OffscreenCanvas) {
	const ctx = canvas.getContext('2d') as any;
	let frames = 0;
	const tick = (time: number) => {
		frames++;
		const { width, height } = canvas;
		ctx.clearRect(0, 0, width, height);
		const radius = Math.min(width, height) / 4;
		const x = width / 2 + Math.cos(time / 500) * (width / 2 - radius);
		ctx.fillStyle = 'rgba(220, 40, 60, 0.85)';
		ctx.beginPath();
		ctx.arc(x, height / 2, radius, 0, Math.PI * 2);
		ctx.fill();
		report('2d', canvas, frames);
		requestAnimationFrame(tick);
	};
	requestAnimationFrame(tick);
}

function drawWebGL(canvas: OffscreenCanvas) {
	const gl = canvas.getContext('webgl', { alpha: false }) as any;
	if (!gl) {
		self.postMessage({ kind: 'webgl', frames: -1, width: 0, height: 0 });
		return;
	}
	let frames = 0;
	const tick = (time: number) => {
		frames++;
		gl.viewport(0, 0, canvas.width, canvas.height);
		gl.clearColor(0.1, 0.5 + 0.4 * Math.sin(time / 400), 0.3, 1);
		gl.clear(gl.COLOR_BUFFER_BIT);
		report('webgl', canvas, frames);
		requestAnimationFrame(tick);
	};
	requestAnimationFrame(tick);
}

self.onmessage = (event: any) => {
	const { kind, canvas, handle } = event.data ?? {};
	const offscreen: OffscreenCanvas = canvas ?? OffscreenCanvas._fromHandle(handle);
	if (kind === 'webgl') {
		drawWebGL(offscreen);
	} else {
		draw2D(offscreen);
	}
};
