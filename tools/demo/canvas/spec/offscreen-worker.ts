// `offscreen.worker.ts` is the Worker half.

import { ImageAsset, OffscreenCanvas } from '@nativescript/canvas';
import { suite, test, skip, ok, equal, throws, rejects, make2D, makeCanvas, pixelAt } from './harness';

declare const Worker: any;

const RED = [255, 0, 0, 255];
const GREEN = [0, 255, 0, 255];
const BLUE = [0, 0, 255, 255];

function near(actual: ArrayLike<number>, expected: ArrayLike<number>, tolerance = 8) {
	for (let i = 0; i < 4; i++) {
		if (Math.abs(actual[i] - expected[i]) > tolerance) {
			return false;
		}
	}
	return true;
}

function expectNear(actual: ArrayLike<number>, expected: ArrayLike<number>, what: string) {
	ok(actual && near(actual, expected), `${what}: expected [${Array.from(expected).join(', ')}], got [${actual ? Array.from(actual).join(', ') : actual}]`);
}

class WorkerClient {
	private worker = new Worker('./offscreen.worker.ts');
	private next = 1;
	private calls = new Map<number, { resolve: (value: any) => void; reject: (error: any) => void }>();

	constructor() {
		this.worker.onmessage = (event: any) => {
			const { id, ok, result, name, message } = event.data ?? {};
			const call = this.calls.get(id);
			if (!call) {
				return;
			}
			this.calls.delete(id);
			if (ok) {
				call.resolve(result);
			} else {
				const error: any = new Error(message);
				error.name = name;
				call.reject(error);
			}
		};
		this.worker.onerror = (error: any) => {
			for (const call of this.calls.values()) {
				call.reject(new Error(`worker error: ${error?.message ?? error}`));
			}
			this.calls.clear();
		};
	}

	call(op: string, args?: any, timeout = 10000): Promise<any> {
		const id = this.next++;
		return new Promise((resolve, reject) => {
			const timer = setTimeout(() => {
				this.calls.delete(id);
				reject(new Error(`worker op ${op} timed out`));
			}, timeout);
			this.calls.set(id, {
				resolve: (value) => {
					clearTimeout(timer);
					resolve(value);
				},
				reject: (error) => {
					clearTimeout(timer);
					reject(error);
				},
			});
			this.worker.postMessage({ id, op, args });
		});
	}

	terminate() {
		this.worker.terminate();
	}
}

let client: WorkerClient | null = null;

function worker() {
	client ??= new WorkerClient();
	return client;
}

function handleOf(width: number, height: number) {
	return OffscreenCanvas._toHandle(new OffscreenCanvas(width, height));
}

function pixelOfDataURL(url: string, width: number, height: number, x: number, y: number) {
	const comma = url.indexOf(',');
	ok(comma > 0 && comma < url.length - 1, `not an image: ${url.substring(0, 32)}`);
	const [, buffer] = (global as any).CanvasModule.__base64Decode(url.substring(comma + 1));
	const asset = new ImageAsset();
	ok(asset.loadFromEncodedBytesSync(new Uint8Array(buffer)), 'the data URL did not decode');
	const { ctx } = make2D(width, height);
	ctx.drawImage(asset, 0, 0);
	return pixelAt(ctx, x, y);
}

/** A Worker's frames reach the view after it answers. */
async function eventually(check: () => void, timeout = 2000) {
	const started = Date.now();
	for (;;) {
		try {
			check();
			return;
		} catch (e) {
			if (Date.now() - started > timeout) {
				throw e;
			}
		}
		await new Promise((resolve) => setTimeout(resolve, 50));
	}
}

function transferred(width: number, height: number) {
	const canvas = makeCanvas(width, height) as any;
	const offscreen = canvas.transferControlToOffscreen();
	return { canvas, offscreen };
}

export function registerOffscreenWorkerSpec() {
	suite('offscreen.worker.handle', () => {
		test('a handle is received once', () => {
			const handle = handleOf(4, 4);
			const received = OffscreenCanvas._fromHandle(handle);
			equal(received.width, 4);
			throws(() => OffscreenCanvas._fromHandle(handle), 'DataCloneError');
		});

		test('the sent canvas is detached', async () => {
			const canvas = new OffscreenCanvas(4, 4);
			OffscreenCanvas._toHandle(canvas);
			equal(canvas.width, 0);
			equal(canvas.height, 0);
			throws(() => canvas.getContext('2d'), 'InvalidStateError');
			throws(() => (canvas.width = 2), 'InvalidStateError');
			throws(() => OffscreenCanvas._toHandle(canvas), 'DataCloneError');
			const error = await rejects(canvas.convertToBlob());
			equal(error?.name, 'InvalidStateError');
		});

		test('a canvas with a context cannot be sent', () => {
			const canvas = new OffscreenCanvas(4, 4);
			canvas.getContext('2d');
			throws(() => OffscreenCanvas._toHandle(canvas), 'InvalidStateError');
		});

		test('a released handle is gone', () => {
			const handle = handleOf(4, 4);
			equal(OffscreenCanvas._releaseHandle(handle), true);
			equal(OffscreenCanvas._releaseHandle(handle), false);
			throws(() => OffscreenCanvas._fromHandle(handle), 'DataCloneError');
		});

		test('a received canvas draws here too', () => {
			const canvas = OffscreenCanvas._fromHandle(handleOf(8, 8));
			const ctx = canvas.getContext('2d') as any;
			ctx.fillStyle = 'red';
			ctx.fillRect(0, 0, 8, 8);
			expectNear(pixelAt(ctx, 4, 4), RED, 'pixel');
		});
	});

	suite('offscreen.worker.transfer', () => {
		if (!(global as any).CanvasModule?.OffscreenSurface) {
			skip('a transferred canvas draws through its view', 'no OffscreenSurface on this host yet');
			return;
		}

		test('a transferred canvas draws through its view', async () => {
			const { canvas, offscreen } = transferred(8, 8);
			ok((offscreen as any)._surface?.hasView, 'not backed by the view');
			const ctx = offscreen.getContext('2d');
			ctx.fillStyle = 'red';
			ctx.fillRect(0, 0, 8, 8);
			// Shown once committed, at the next flush.
			await eventually(() => expectNear(pixelOfDataURL(canvas.toDataURL(), 8, 8, 4, 4), RED, 'the view'));
		});

		test('2d from the Worker shows in the view', async () => {
			const { canvas, offscreen } = transferred(10, 10);
			const result = await worker().call('2d', { handle: OffscreenCanvas._toHandle(offscreen) });
			expectNear(result.rest, RED, 'in the Worker');
			await eventually(() => {
				const url = canvas.toDataURL();
				expectNear(pixelOfDataURL(url, 10, 10, 1, 1), BLUE, 'the view, corner');
				expectNear(pixelOfDataURL(url, 10, 10, 8, 8), RED, 'the view, rest');
			});
		});

		for (const version of ['webgl', 'webgl2']) {
			test(`${version} from the Worker shows in the view`, async () => {
				const { canvas, offscreen } = transferred(8, 8);
				const result = await worker().call('webgl', { version, preserve: true, handle: OffscreenCanvas._toHandle(offscreen) });
				ok(result.context, `no ${version} context`);
				expectNear(result.pixel, GREEN, 'in the Worker');
				await eventually(() => expectNear(pixelOfDataURL(canvas.toDataURL(), 8, 8, 4, 4), GREEN, 'the view'));
			});
		}

		test('resizing in the Worker resizes the view', async () => {
			const { canvas, offscreen } = transferred(8, 8);
			await worker().call('resize', { handle: OffscreenCanvas._toHandle(offscreen) });
			await eventually(() => {
				equal(canvas.width, 20, 'width');
				equal(canvas.height, 10, 'height');
			});
		});

		test('the Worker draws on after the view is gone', async () => {
			const { canvas, offscreen } = transferred(8, 8);
			await worker().call('hold', { handle: OffscreenCanvas._toHandle(offscreen) });
			canvas.disposeNativeView();
			await new Promise((resolve) => setTimeout(resolve, 100));
			const result = await worker().call('drawHeld', { color: 'blue' });
			expectNear(result.pixel, BLUE, 'in the Worker');
		});
	});

	suite('offscreen.worker', () => {
		test('the Worker has requestAnimationFrame', async () => {
			const { frames } = await worker().call('frames');
			ok(frames > 3, `expected frames, got ${frames}`);
		});

		for (const threaded of [true, false]) {
			test(`2d draws in the Worker (${threaded ? 'threaded' : 'direct'}, made there)`, async () => {
				const result = await worker().call('2d', { width: 8, height: 6, threaded });
				equal(result.width, 8);
				equal(result.height, 6);
				expectNear(result.corner, BLUE, 'corner');
				expectNear(result.rest, RED, 'rest');
			});
		}

		test('2d draws in the Worker (sent)', async () => {
			const result = await worker().call('2d', { handle: handleOf(10, 10) });
			equal(result.width, 10);
			expectNear(result.corner, BLUE, 'corner');
			expectNear(result.rest, RED, 'rest');
		});

		test('a sent canvas is received once, in the Worker too', async () => {
			const result = await worker().call('adoptTwice', { handle: handleOf(4, 4) });
			equal(result.threw, true);
			equal(result.name, 'DataCloneError');
		});

		test('resizing in the Worker resizes the bitmap', async () => {
			const result = await worker().call('resize', { handle: handleOf(8, 8) });
			equal(result.width, 20);
			equal(result.height, 10);
			equal(result.imageWidth, 20);
			expectNear(result.pixel, GREEN, 'bottom-right pixel');
		});

		for (const version of ['webgl', 'webgl2']) {
			for (const sent of [false, true]) {
				test(`${version} draws in the Worker (${sent ? 'sent' : 'made there'})`, async () => {
					const result = await worker().call('webgl', sent ? { version, handle: handleOf(8, 8) } : { version });
					ok(result.context, `no ${version} context`);
					expectNear(result.pixel, GREEN, 'readPixels');
					expectNear(result.drawn, GREEN, 'drawn as an image');
				});
			}
		}

		test('bitmaprenderer shows a bitmap in the Worker', async () => {
			const result = await worker().call('bitmaprenderer');
			expectNear(result.pixel, RED, 'pixel');
		});

		for (const sent of [false, true]) {
			test(`webgpu draws in the Worker (${sent ? 'sent' : 'made there'})`, async () => {
				const result = await worker().call('webgpu', sent ? { handle: handleOf(8, 8) } : {});
				if (result.skipped) {
					console.log(`SPEC|note|offscreen.worker|webgpu: ${result.skipped}`);
					return;
				}
				ok(result.context, 'no webgpu context');
				equal(result.width, 8);
				expectNear(result.pixel, BLUE, 'pixel');
			});
		}

		test('convertToBlob in the Worker', async () => {
			const result = await worker().call('convertToBlob', { width: 4, height: 4 });
			equal(result.type, 'image/png');
			ok(result.size > 0, 'an empty blob');
		});

		test('terminating a drawing Worker leaves the main thread drawing', async () => {
			const drawing = new WorkerClient();
			await drawing.call('loop', { handle: handleOf(16, 16) });
			await new Promise((resolve) => setTimeout(resolve, 200));
			drawing.terminate();
			await new Promise((resolve) => setTimeout(resolve, 200));
			const { ctx } = make2D(4, 4);
			ctx.fillStyle = 'red';
			ctx.fillRect(0, 0, 4, 4);
			expectNear(pixelAt(ctx, 2, 2), RED, 'main thread pixel');
		});

		test('(cleanup) the shared Worker ends', () => {
			client?.terminate();
			client = null;
		});
	});
}
