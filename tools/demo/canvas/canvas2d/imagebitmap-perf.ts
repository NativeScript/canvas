import { Canvas } from '@nativescript/canvas';

/**
 * createImageBitmap benchmark. Each source takes a different route through the
 * binding -- decoded CPU pixels, a JS buffer, a GPU readback -- so each is timed
 * separately. One line per case so a run can be scraped off logcat:
 *
 *   IMGBM|<case>|<pixels>|<calls>|<bestMs>|<usPerCall>|<blockingUs>
 *
 * `usPerCall` is awaited round-trip latency; `blockingUs` is the time the call
 * itself holds the JS thread, which is what costs a frame.
 */

interface Case {
	name: string;
	make: () => Promise<any> | any;
	run: (source: any) => Promise<any>;
	calls?: number;
}

async function time(name: string, pixels: number, calls: number, run: () => Promise<any>) {
	for (let i = 0; i < 3; i++) {
		const bitmap = await run();
		bitmap?.close?.();
	}

	let best = Infinity;
	for (let rep = 0; rep < 3; rep++) {
		const start = performance.now();
		for (let i = 0; i < calls; i++) {
			const bitmap = await run();
			// Holding them all is what got the spec suite killed by lowmemorykiller.
			bitmap?.close?.();
		}
		const elapsed = performance.now() - start;
		if (elapsed < best) {
			best = elapsed;
		}
	}

	const usPerCall = (best * 1000) / calls;

	// Issue every call before awaiting any: what is left is the JS thread's share.
	let blocking = Infinity;
	for (let rep = 0; rep < 3; rep++) {
		const pending: Array<Promise<any>> = [];
		const start = performance.now();
		for (let i = 0; i < calls; i++) {
			pending.push(Promise.resolve(run()));
		}
		const elapsed = performance.now() - start;
		const bitmaps = await Promise.all(pending);
		for (const bitmap of bitmaps) {
			bitmap?.close?.();
		}
		if (elapsed < blocking) {
			blocking = elapsed;
		}
	}
	const blockingUs = (blocking * 1000) / calls;

	console.log(`IMGBM|${name}|${pixels}|${calls}|${best.toFixed(3)}|${usPerCall.toFixed(1)}|${blockingUs.toFixed(1)}`);
	return usPerCall;
}

function paint(width: number, height: number) {
	const canvas = Canvas.createCustomView();
	(canvas as any).width = width;
	(canvas as any).height = height;
	const ctx = canvas.getContext('2d') as any;
	for (let i = 0; i < 16; i++) {
		ctx.fillStyle = `rgb(${(i * 16) & 255}, ${(i * 7) & 255}, ${(i * 31) & 255})`;
		ctx.fillRect((i % 4) * (width / 4), Math.floor(i / 4) * (height / 4), width / 4, height / 4);
	}
	return { canvas, ctx };
}

function imageData(ctx: any, width: number, height: number) {
	return ctx.getImageData(0, 0, width, height);
}

export async function runImageBitmapPerf(width = 512, height = 512) {
	const pixels = width * height;
	console.log(`IMGBM|meta|size|${width}x${height}`);
	console.log(`IMGBM|header|case|pixels|calls|bestMs|usPerCall|blockingUs`);

	const { canvas, ctx } = paint(width, height);
	const data = imageData(ctx, width, height);

	const decoded = await createImageBitmap(data);

	await time('imagedata', pixels, 50, () => createImageBitmap(data));
	await time('bitmap-passthrough', pixels, 50, () => createImageBitmap(decoded));
	await time('bitmap-crop', pixels, 50, () => createImageBitmap(decoded, 0, 0, width / 2, height / 2));
	await time('bitmap-resize', pixels, 20, () => createImageBitmap(decoded, { resizeWidth: width / 2, resizeHeight: height / 2 } as any));
	await time('bitmap-flipY', pixels, 20, () => createImageBitmap(decoded, { imageOrientation: 'flipY' } as any));
	await time('canvas-readback', pixels, 20, () => createImageBitmap(canvas as any));

	decoded.close();
	try {
		(canvas as any).disposeNativeView();
	} catch (e) {}

	console.log('IMGBM|done');
}
