import { EventData, Observable, Page } from '@nativescript/core';
import { Canvas, OffscreenCanvas } from '@nativescript/canvas';

declare const Worker: any;

// Two views drawn by a Worker: a transparent 2D canvas over the page's tint, and an opaque WebGL one.
export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new OffscreenWorkerModel();
}

class OffscreenWorkerModel extends Observable {
	private worker: any = null;
	private sent = new Set<string>();

	constructor() {
		super();
		this.set('status', 'waiting for the canvases…');
	}

	canvasLoaded(args: EventData) {
		const canvas = args.object as Canvas;
		const kind = canvas.id === 'webgl' ? 'webgl' : '2d';
		if (this.sent.has(kind)) {
			return;
		}
		this.sent.add(kind);
		this.worker ??= this.startWorker();
		const offscreen = canvas.transferControlToOffscreen();
		const transferable = typeof (global as any).__nsRegisterTransferable === 'function';
		// Runtimes without transfer hooks take a handle instead.
		const message = transferable ? { kind, canvas: offscreen } : { kind, handle: OffscreenCanvas._toHandle(offscreen) };
		this.worker.postMessage(message, transferable ? [offscreen] : undefined);
	}

	private startWorker() {
		const worker = new Worker('./canvas-offscreen-worker.worker.ts');
		worker.onmessage = (event: any) => {
			const { kind, frames, width, height } = event.data ?? {};
			console.log(`OFFSCREEN_WORKER|${kind}|frames=${frames}|${width}x${height}`);
			this.set('status', `${kind}: ${frames} frames at ${width}x${height}`);
		};
		return worker;
	}
}
