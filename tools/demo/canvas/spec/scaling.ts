/**
 * The CSS-pixel / device-pixel contract.
 *
 * Libraries like Pixi and Chart.js all assume the web's split: `clientWidth` and
 * `getBoundingClientRect()` are CSS pixels, `canvas.width` is the device-pixel backing
 * store, and `devicePixelRatio` converts between them. Get that wrong in either direction
 * and everything renders at the wrong size — which is what "the scaling is bad" looks like
 * from the outside.
 *
 * These check invariants a library would rely on, rather than restating the
 * implementation: that clientWidth tracks layout and not the backing store, that the
 * bounding rect agrees with it, and that resizing a canvas actually resizes its surface.
 */

import { Screen } from '@nativescript/core';
import { suite, test, ok, equal, closeTo, getPageCanvas, make2D, pixelAt } from './harness';

declare const window: any;

export function registerScalingSpec() {
	suite('scaling.contract', () => {
		test('devicePixelRatio matches the screen scale', () => {
			const dpr = window?.devicePixelRatio;
			ok(typeof dpr === 'number' && dpr > 0, `devicePixelRatio is ${dpr}`);
			closeTo(dpr, Screen.mainScreen.scale, 0.01, `devicePixelRatio ${dpr} vs screen scale ${Screen.mainScreen.scale}`);
		});

		test('clientWidth is CSS pixels, never the device-pixel backing store', () => {
			const canvas: any = getPageCanvas();
			ok(canvas, 'no page canvas');
			ok(canvas.clientWidth > 0, `clientWidth is ${canvas.clientWidth} for a laid-out canvas`);

			// The give-away for a device-pixel `clientWidth` is that it tracks the backing
			// store rather than the layout: set a backing store far larger than the view
			// and a CSS-pixel clientWidth must not move.
			const before = canvas.clientWidth;
			const restore = canvas.width;
			canvas.width = before * Screen.mainScreen.scale * 4;
			const after = canvas.clientWidth;
			canvas.width = restore;

			equal(after, before, `clientWidth changed with canvas.width (${before} -> ${after}); it must reflect layout, not the backing store`);
		});

		test('getBoundingClientRect agrees with clientWidth', () => {
			const canvas: any = getPageCanvas();
			ok(canvas, 'no page canvas');
			if (typeof canvas.getBoundingClientRect !== 'function') {
				throw new Error('getBoundingClientRect is missing');
			}

			// Layout libraries size themselves from this rect; a zero here reads as "the
			// element has no room" and collapses the render.
			const rect = canvas.getBoundingClientRect();
			ok(rect.width > 0 && rect.height > 0, `getBoundingClientRect returned ${rect.width}x${rect.height} for a laid-out canvas`);
			closeTo(rect.width, canvas.clientWidth, 2, `rect.width=${rect.width} vs clientWidth=${canvas.clientWidth}`);
			closeTo(rect.height, canvas.clientHeight, 2, `rect.height=${rect.height} vs clientHeight=${canvas.clientHeight}`);
		});

		test('canvas.width is the backing store, independent of layout', () => {
			const { canvas, ctx } = make2D(10, 10);

			// An offscreen canvas has no layout, so width/height are purely the backing
			// store and must round-trip exactly.
			(canvas as any).width = 123;
			(canvas as any).height = 45;
			equal((canvas as any).width, 123, 'canvas.width should round-trip');
			equal((canvas as any).height, 45, 'canvas.height should round-trip');

			// And drawing addresses that backing store in its own pixels.
			ctx.fillStyle = 'red';
			ctx.fillRect(0, 0, 123, 45);
			const pixel = pixelAt(ctx, 120, 40);
			equal(pixel[0], 255, `expected the fill to reach (120,40), got [${pixel.join(', ')}]`);
		});
	});

	suite('scaling.chartjs', () => {
		test('the Chart.js platform reports CSS pixels from getMaximumSize', () => {
			let NativeScriptPlatform: any;
			try {
				// eslint-disable-next-line @typescript-eslint/no-var-requires
				NativeScriptPlatform = require('@nativescript/canvas-chartjs').NativeScriptPlatform;
			} catch (e) {
				throw new Error('@nativescript/canvas-chartjs is not resolvable: ' + e);
			}

			const canvas: any = getPageCanvas();
			ok(canvas, 'no page canvas');

			const platform = new NativeScriptPlatform();
			const size = platform.getMaximumSize(canvas);

			// Chart.js multiplies this by getDevicePixelRatio() itself when it sizes the
			// backing store, so returning device pixels here scales the chart twice.
			closeTo(size.width, canvas.clientWidth, 2, `getMaximumSize().width=${size.width} but clientWidth=${canvas.clientWidth} (scale=${Screen.mainScreen.scale}) — a DPR-scaled value here is applied twice`);
			closeTo(size.height, canvas.clientHeight, 2, `getMaximumSize().height=${size.height} vs clientHeight=${canvas.clientHeight}`);
		});

		test('the Chart.js platform reports the screen scale as the pixel ratio', () => {
			let NativeScriptPlatform: any;
			try {
				// eslint-disable-next-line @typescript-eslint/no-var-requires
				NativeScriptPlatform = require('@nativescript/canvas-chartjs').NativeScriptPlatform;
			} catch (e) {
				throw new Error('@nativescript/canvas-chartjs is not resolvable: ' + e);
			}

			const platform = new NativeScriptPlatform();
			closeTo(platform.getDevicePixelRatio(), Screen.mainScreen.scale, 0.01, 'getDevicePixelRatio should be the screen scale');
		});
	});
}
