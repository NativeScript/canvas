/**
 * 2D context loss on Windows: the Direct3D 12 device removed (as by a driver reset) through
 * `CanvasModule.__simulateD3DDeviceRemoval()`. Runs on its own (`suite: contextlost`): D3D12
 * has one device per adapter, so a WebGPU device held by another suite is the removed device too,
 * and no new device can be made while it is alive; and after the removal the other GPU suites
 * would have none.
 */

import { suite, test, skip, ok, equal, make2D, pixelEqual } from './harness';

declare const __WINDOWS__: boolean;

const WINDOWS = typeof __WINDOWS__ !== 'undefined' && __WINDOWS__;

function removeDevice() {
	ok((global as any).CanvasModule?.__simulateD3DDeviceRemoval?.(), 'could not remove the Direct3D 12 device');
}

function wait(ms: number) {
	return new Promise<void>((resolve) => setTimeout(resolve, ms));
}

export function registerContextLossSpec() {
	suite('contextlost', () => {
		if (!WINDOWS) {
			skip('device loss', 'Windows (Direct3D 12) only');
			return;
		}

		test('a removed device fires contextlost then contextrestored; the context draws again', async () => {
			const { canvas, ctx } = make2D(32, 32);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 32, 32);
			const events: string[] = [];
			const restored = new Promise<void>((resolve) => {
				canvas.addEventListener('contextlost', () => events.push('contextlost'));
				canvas.addEventListener('contextrestored', () => {
					events.push('contextrestored');
					resolve();
				});
			});
			removeDevice();
			// The next flush finds the loss.
			ctx.fillRect(0, 0, 4, 4);
			// Restoring retries until a new device can be made.
			await Promise.race([restored, wait(16000)]);
			equal(events.join(','), 'contextlost,contextrestored');
			// Restored cleared, in the default state.
			pixelEqual(ctx, 16, 16, [0, 0, 0, 0]);
			equal(ctx.fillStyle, '#000000');
			ctx.fillStyle = '#00ff00';
			ctx.fillRect(0, 0, 32, 32);
			pixelEqual(ctx, 16, 16, [0, 255, 0, 255]);
		});

		test('preventDefault() on contextlost keeps the context lost', async () => {
			const { canvas, ctx } = make2D(16, 16);
			let restoredEvents = 0;
			const lost = new Promise<void>((resolve) => {
				canvas.addEventListener('contextlost', (event: any) => {
					event.preventDefault();
					resolve();
				});
			});
			canvas.addEventListener('contextrestored', () => restoredEvents++);
			removeDevice();
			ctx.fillRect(0, 0, 4, 4);
			await Promise.race([lost, wait(3000)]);
			await wait(100);
			equal(restoredEvents, 0);
			equal((canvas as any)._canvas.isContextLost(), true);
			// The page restores it when it is ready.
			equal((canvas as any)._canvas.restoreContext(), true);
		});
	});
}
