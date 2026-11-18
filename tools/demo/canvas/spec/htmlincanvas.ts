// The grid is set to the canvas's CSS size: one grid unit is one DIP.

import { Button, Color, GridLayout, StackLayout, View } from '@nativescript/core';
import { Canvas } from '@nativescript/canvas';
import { suite, test, skip, ok, equal, closeTo, throws, getPageCanvas, pixelAt } from './harness';

declare const navigator: any;

const RED: [number, number, number, number] = [255, 0, 0, 255];
const BLUE: [number, number, number, number] = [0, 0, 255, 255];

function near(actual: ArrayLike<number>, expected: ArrayLike<number>, tolerance = 8) {
	for (let i = 0; i < 4; i++) {
		if (Math.abs(actual[i] - expected[i]) > tolerance) {
			return false;
		}
	}
	return true;
}

function show(pixel: ArrayLike<number>) {
	return `[${Array.from(pixel).join(', ')}]`;
}

// Not rAF: core runs one requested outside a frame as the next macrotask.
function nextFrame(): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, 17));
}

function wait(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

function nextPaint(canvas: any, accept: (event: any) => boolean = () => true, timeout = 3000): Promise<any> {
	return new Promise((resolve, reject) => {
		const timer = setTimeout(() => {
			canvas.off('paint', listener);
			reject(new Error('no paint event within ' + timeout + ' ms'));
		}, timeout);
		const listener = (event: any) => {
			if (!accept(event)) {
				return;
			}
			clearTimeout(timer);
			canvas.off('paint', listener);
			resolve(event);
		};
		canvas.on('paint', listener);
	});
}

function block(width: number, height: number, color: string) {
	const view = new StackLayout();
	view.width = width;
	view.height = height;
	view.backgroundColor = new Color(color);
	view.horizontalAlignment = 'left';
	(view as any).drawable = true;
	return view;
}

const mounted: any[] = [];

async function mount(children: View[], width = 200, height = 100, contextType: string | null = '2d'): Promise<{ canvas: any; ctx: any }> {
	const grid = getPageCanvas()?.parent as GridLayout;
	if (!grid) {
		throw new Error('no page to mount the canvas in');
	}
	const canvas: any = new Canvas();
	canvas.content = 'drawable';
	canvas.style.width = width as any;
	canvas.style.height = height as any;
	canvas.horizontalAlignment = 'left';
	canvas.verticalAlignment = 'top';
	for (const child of children) {
		canvas.addChild(child);
	}
	GridLayout.setRow(canvas, 2);
	const loaded = new Promise<void>((resolve) => canvas.once('loaded', () => resolve()));
	grid.addChild(canvas);
	mounted.push(canvas);
	await loaded;
	await nextFrame();
	canvas.width = width;
	canvas.height = height;
	await nextFrame();
	await nextFrame();
	for (let i = 0; i < 60 && !(canvas.getMeasuredWidth() > 0); i++) {
		await nextFrame();
	}
	const ctx = contextType ? canvas.getContext(contextType) : null;
	return { canvas, ctx };
}

// A requested paint can come before the first snapshot.
async function snapshotted(canvas: any, ...views: View[]) {
	for (let i = 0; i < 120; i++) {
		if (views.every((view) => canvas._drawables?.snapshot(view))) {
			return;
		}
		await nextFrame();
	}
	throw new Error('no snapshot within 120 frames');
}

function unmountAll() {
	for (const canvas of mounted.splice(0)) {
		(canvas.parent as GridLayout)?.removeChild(canvas);
	}
}

function mountedTest(name: string, body: () => Promise<void>) {
	test(name, async () => {
		try {
			await body();
		} finally {
			unmountAll();
		}
	});
}

export function registerHtmlInCanvasSpec() {
	suite('htmlincanvas.api', () => {
		test('content reflects only "drawable"', () => {
			const canvas: any = new Canvas();
			equal(canvas.content, 'fallback');
			canvas.content = 'Drawable';
			equal(canvas.content, 'drawable');
			canvas.content = 'nonsense';
			equal(canvas.content, 'fallback');
		});

		test('children are a canvas view tree', () => {
			const canvas: any = new Canvas();
			const a = block(10, 10, 'red');
			const b = block(10, 10, 'blue');
			canvas.addChild(a);
			canvas.insertChild(b, 0);
			equal(canvas.getChildrenCount(), 2);
			equal(canvas.getChildAt(0), b);
			equal(a.parent, canvas);
			canvas.removeChild(b);
			equal(canvas.getChildrenCount(), 1);
			equal(b.parent, undefined);
		});

		test('a canvas with no drawable content gets no host view', async () => {
			const grid = getPageCanvas()?.parent as GridLayout;
			const canvas: any = new Canvas();
			canvas.style.width = 50 as any;
			canvas.style.height = 50 as any;
			GridLayout.setRow(canvas, 2);
			const loaded = new Promise<void>((resolve) => canvas.once('loaded', () => resolve()));
			grid.addChild(canvas);
			try {
				await loaded;
				await nextFrame();
				equal(canvas._drawables, null, 'no controller for a plain canvas');
			} finally {
				grid.removeChild(canvas);
			}
		});

		mountedTest('children do not size the canvas', async () => {
			const tall = block(50, 400, 'red');
			const { canvas } = await mount([tall], 200, 100);
			closeTo(canvas.getActualSize().height, 100, 1);
			closeTo(tall.getActualSize().height, 400, 1);
		});
	});

	suite('htmlincanvas.paint', () => {
		mountedTest('paint fires with a drawable child once it is laid out', async () => {
			const { canvas } = await mount([]);
			const child = block(50, 50, 'red');
			const paint = nextPaint(canvas, (e) => e.changedElements.indexOf(child) > -1);
			canvas.addChild(child);
			const event = await paint;
			ok(Array.isArray(event.changedElements), 'changedElements is an array');
			ok(Object.isFrozen(event.changedElements), 'changedElements is frozen');
			equal(event.type, 'paint');
		});

		mountedTest('requestPaint fires a paint event with nothing changed', async () => {
			const { canvas } = await mount([]);
			canvas.requestPaint();
			const event = await nextPaint(canvas);
			equal(event.changedElements.length, 0);
		});

		mountedTest('onpaint is called too', async () => {
			const { canvas } = await mount([]);
			let called = 0;
			canvas.onpaint = () => called++;
			canvas.requestPaint();
			await nextPaint(canvas);
			equal(called, 1);
		});

		mountedTest('a changed drawable fires paint again, naming it', async () => {
			const child = block(50, 50, 'red');
			const { canvas } = await mount([child]);
			canvas.requestPaint();
			await nextPaint(canvas);
			await wait(100);
			child.backgroundColor = new Color('blue');
			const event = await nextPaint(canvas, (e) => e.changedElements.indexOf(child) > -1);
			ok(event.changedElements.indexOf(child) > -1, 'the child is in changedElements');
		});

		mountedTest('a fallback canvas takes no snapshots', async () => {
			const child = block(50, 50, 'red');
			const { canvas, ctx } = await mount([child]);
			canvas.content = 'fallback';
			throws(() => ctx.drawElementImage(child, 0, 0), 'InvalidStateError');
		});
	});

	suite('htmlincanvas.2d', () => {
		mountedTest('drawElementImage draws the snapshot', async () => {
			const child = block(50, 50, 'red');
			const { canvas, ctx } = await mount([child]);
			await snapshotted(canvas, child);
			ctx.drawElementImage(child, 0, 0);
			const inside = pixelAt(ctx, 25, 25);
			ok(near(inside, RED), `expected red inside, got ${show(inside)}`);
			const outside = pixelAt(ctx, 75, 25);
			equal(outside[3], 0, `expected nothing outside, got ${show(outside)}`);
		});

		mountedTest('drawElementImage scales to a destination size', async () => {
			const child = block(20, 20, 'blue');
			const { canvas, ctx } = await mount([child]);
			await snapshotted(canvas, child);
			ctx.drawElementImage(child, 100, 0, 80, 80);
			const pixel = pixelAt(ctx, 170, 70);
			ok(near(pixel, BLUE), `expected blue, got ${show(pixel)}`);
		});

		mountedTest('drawElementImage draws only the source rect', async () => {
			const child = new StackLayout();
			child.width = 40;
			child.height = 20;
			child.horizontalAlignment = 'left';
			(child as any).drawable = true;
			const top = block(40, 10, 'red');
			const bottom = block(40, 10, 'blue');
			// Nested drawables are left out of their parent's snapshot.
			(top as any).drawable = (bottom as any).drawable = undefined;
			child.addChild(top);
			child.addChild(bottom);
			const { canvas, ctx } = await mount([child]);
			await snapshotted(canvas, child);
			// The bottom half (blue) into a 40x40 square.
			ctx.drawElementImage(child, 0, 10, 40, 10, 0, 0, 40, 40);
			const pixel = pixelAt(ctx, 20, 5);
			ok(near(pixel, BLUE), `expected blue, got ${show(pixel)}`);
		});

		mountedTest('drawElementImage throws before the first snapshot', async () => {
			const { canvas, ctx } = await mount([]);
			const late = block(30, 30, 'red');
			canvas.addChild(late);
			throws(() => ctx.drawElementImage(late, 0, 0), 'InvalidStateError');
		});

		mountedTest('drawElementImage throws for a child that is not drawable', async () => {
			const child = block(30, 30, 'red');
			(child as any).drawable = undefined;
			const { canvas, ctx } = await mount([child]);
			await wait(100);
			throws(() => ctx.drawElementImage(child, 0, 0), 'InvalidStateError');
		});

		mountedTest('drawElementImage throws for a view outside the canvas', async () => {
			const { canvas, ctx } = await mount([]);
			const stranger = block(30, 30, 'red');
			throws(() => ctx.drawElementImage(stranger, 0, 0), 'InvalidStateError');
		});

		mountedTest('drawElementImage sets the element transform', async () => {
			const child = block(50, 50, 'red');
			const { canvas, ctx } = await mount([child]);
			await snapshotted(canvas, child);
			ctx.translate(10, 5);
			ctx.scale(2, 2);
			ctx.drawElementImage(child, 30, 20);
			const m = canvas.getElementTransform(child);
			closeTo(m.a, 2, 1e-3);
			closeTo(m.d, 2, 1e-3);
			closeTo(m.e, 70, 1e-3);
			closeTo(m.f, 45, 1e-3);
		});

		mountedTest('preserveElementGeometry leaves the transform alone', async () => {
			const child = block(50, 50, 'red');
			const { canvas, ctx } = await mount([child]);
			await snapshotted(canvas, child);
			ctx.drawElementImage(child, 30, 20, { preserveElementGeometry: true });
			const m = canvas.getElementTransform(child);
			equal(m.e, 0);
			equal(m.f, 0);
		});

		mountedTest('the default size is in canvas grid units', async () => {
			const child = block(50, 25, 'red');
			const { canvas } = await mount([child]);
			canvas.width = 400;
			canvas.height = 200;
			await nextFrame();
			await nextFrame();
			const size = canvas.getElementImageDefaultSize(child);
			closeTo(size.width, 100, 0.5);
			closeTo(size.height, 50, 0.5);
		});

		mountedTest('captureElementImage outlives the frame and closes', async () => {
			const child = block(50, 50, 'red');
			const { canvas, ctx } = await mount([child]);
			await snapshotted(canvas, child);
			const image = canvas.captureElementImage(child);
			equal(Object.prototype.toString.call(image), '[object ElementImage]');
			child.backgroundColor = new Color('blue');
			await wait(300);
			ctx.drawElementImage(image, 0, 0);
			const pixel = pixelAt(ctx, 25, 25);
			ok(near(pixel, RED), `expected the captured red, got ${show(pixel)}`);
			image.close();
			throws(() => ctx.drawElementImage(image, 0, 0), 'InvalidStateError');
		});
	});

	suite('htmlincanvas.gpu', () => {
		mountedTest('snapshots live on the GPU and draw from there', async () => {
			const child = block(50, 50, 'red');
			const { canvas, ctx } = await mount([child]);
			await snapshotted(canvas, child);
			const snapshot = canvas._drawables.nativeSnapshot(child);
			if (__ANDROID__ && android.os.Build.VERSION.SDK_INT >= 29) {
				ok(snapshot?.getHardwareBuffer?.(), 'expected a HardwareBuffer');
				const drawn = (org as any).nativescript.canvas.NSCDrawableHost.draw2D(long(ctx.context.__getPointer()), snapshot, 0, 0, snapshot.getWidth(), snapshot.getHeight(), 100, 0, 50, 50);
				ok(drawn, 'expected the GPU draw to be taken');
				ctx.context.__makeDirty();
				const pixel = pixelAt(ctx, 125, 25);
				ok(near(pixel, RED), `expected red from the GPU draw, got ${show(pixel)}`);
			} else if (__APPLE__) {
				ok(snapshot, 'expected a native snapshot');
				ok(snapshot.draw2D(ctx.context.__getPointer(), 0, 0, snapshot.width, snapshot.height, 100, 0, 50, 50), 'expected the Metal draw to be taken');
				ctx.context.__makeDirty?.();
				const pixel = pixelAt(ctx, 125, 25);
				ok(near(pixel, RED), `expected red from the Metal draw, got ${show(pixel)}`);
			}
			snapshot?.close?.();
		});
	});

	suite('htmlincanvas.hittest', () => {
		mountedTest('touches reach the element where it is drawn, not where it is laid out', async () => {
			const button = new Button();
			button.text = 'hit';
			button.width = 60;
			button.height = 40;
			button.horizontalAlignment = 'left';
			(button as any).drawable = true;
			const { canvas, ctx } = await mount([button]);
			await snapshotted(canvas, button);
			ctx.drawElementImage(button, 120, 50);

			if (__APPLE__) {
				const native = canvas.nativeView as UIView;
				const drawn = native.hitTestWithEvent(CGPointMake(150, 70), null);
				ok(drawn === button.nativeView || drawn?.isDescendantOfView(button.nativeView), 'the drawn position hits the button');
				const laidOut = native.hitTestWithEvent(CGPointMake(20, 20), null);
				ok(laidOut !== button.nativeView && !laidOut?.isDescendantOfView?.(button.nativeView), 'the layout position does not');
			} else if (__ANDROID__) {
				const density = canvas.getMeasuredWidth() / 200;
				let touched = 0;
				(button.nativeView as android.view.View).setOnTouchListener(
					new android.view.View.OnTouchListener({
						onTouch() {
							touched++;
							return true;
						},
					}),
				);
				const send = (x: number, y: number) => {
					const time = android.os.SystemClock.uptimeMillis();
					const down = android.view.MotionEvent.obtain(time, time, android.view.MotionEvent.ACTION_DOWN, x * density, y * density, 0);
					const up = android.view.MotionEvent.obtain(time, time + 10, android.view.MotionEvent.ACTION_UP, x * density, y * density, 0);
					(canvas.nativeView as android.view.View).dispatchTouchEvent(down);
					(canvas.nativeView as android.view.View).dispatchTouchEvent(up);
					down.recycle();
					up.recycle();
				};
				send(20, 20);
				equal(touched, 0, 'the layout position does not reach the button');
				send(150, 70);
				equal(touched, 2, 'the drawn position reaches the button (down + up)');
			}
		});

		mountedTest('clearElementGeometry stops hit testing', async () => {
			const button = new Button();
			button.text = 'hit';
			button.width = 60;
			button.height = 40;
			button.horizontalAlignment = 'left';
			(button as any).drawable = true;
			const { canvas, ctx } = await mount([button]);
			await snapshotted(canvas, button);
			ctx.drawElementImage(button, 0, 0);
			canvas.clearElementGeometry(button);
			const m = canvas.getElementTransform(button);
			ok(m.a === 1 && m.b === 0 && m.c === 0 && m.d === 1 && m.e === 0 && m.f === 0, 'identity once cleared');
			if (__APPLE__) {
				const hit = (canvas.nativeView as UIView).hitTestWithEvent(CGPointMake(20, 20), null);
				ok(hit !== button.nativeView && !hit?.isDescendantOfView?.(button.nativeView), 'no longer hits the button');
			}
		});
	});

	suite('htmlincanvas.webgl', () => {
		mountedTest('texElementSubImage2D uploads the snapshot', async () => {
			const child = block(16, 16, 'red');
			const { canvas, ctx: gl } = await mount([child], 200, 100, 'webgl');
			ok(gl, 'no webgl context');
			await snapshotted(canvas, child);
			const texture = gl.createTexture();
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 16, 16, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
			gl.texElementSubImage2D(gl.TEXTURE_2D, 0, 0, 0, child);
			const fb = gl.createFramebuffer();
			gl.bindFramebuffer(gl.FRAMEBUFFER, fb);
			gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0);
			const pixel = new Uint8Array(4);
			gl.readPixels(8, 8, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixel);
			ok(near(pixel, RED), `expected red, got ${show(pixel)}`);
		});
	});

	suite('htmlincanvas.webgpu', () => {
		mountedTest('drawElementImageToTexture copies the snapshot', async () => {
			const adapter = await navigator.gpu?.requestAdapter();
			if (!adapter) {
				console.log('SPEC|htmlincanvas.webgpu|no adapter: nothing to check');
				return;
			}
			const device = await adapter.requestDevice();
			const child = block(16, 16, 'red');
			const { canvas } = await mount([child], 200, 100, null);
			await snapshotted(canvas, child);
			const GPUTextureUsage = (global as any).GPUTextureUsage;
			const GPUBufferUsage = (global as any).GPUBufferUsage;
			const GPUMapMode = (global as any).GPUMapMode;
			const texture = device.createTexture({ size: [16, 16], format: 'rgba8unorm', usage: GPUTextureUsage.COPY_DST | GPUTextureUsage.COPY_SRC | GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.TEXTURE_BINDING });
			device.queue.drawElementImageToTexture({ source: child }, { texture });
			const buffer = device.createBuffer({ size: 256 * 16, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
			const encoder = device.createCommandEncoder();
			encoder.copyTextureToBuffer({ texture }, { buffer, bytesPerRow: 256 }, [16, 16]);
			device.queue.submit([encoder.finish()]);
			await buffer.mapAsync(GPUMapMode.READ);
			const bytes = new Uint8Array(buffer.getMappedRange());
			const pixel = bytes.slice(8 * 256 + 8 * 4, 8 * 256 + 8 * 4 + 4);
			buffer.unmap();
			ok(near(pixel, RED), `expected red, got ${show(pixel)}`);
		});
	});
}
