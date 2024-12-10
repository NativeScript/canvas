import '@nativescript/macos-node-api';
import { createRequire } from 'node:module';
import utils from './utils';

// @ts-ignore
const require = createRequire(import.meta.url);

const { GPU, GPUDevice, GPUAdapter } = require('./canvas-napi.darwin-arm64.node');

const { requestAnimationFrame, cancelAnimationFrame } = utils;

function installPolyfills(window) {
	Object.defineProperty(window, 'devicePixelRatio', {
		value: NSScreen.mainScreen.backingScaleFactor,
		writable: true,
	});

	if (!('gpu' in navigator)) {
		Object.defineProperty(navigator, 'gpu', {
			value: GPU.getInstance(),
			writable: false,
		});
	}

	globalThis.window.devicePixelRatio = window.devicePixelRatio = NSScreen.mainScreen.backingScaleFactor;
	globalThis.requestAnimationFrame = window.requestAnimationFrame = requestAnimationFrame;
	globalThis.cancelAnimationFrame = window.cancelAnimationFrame = cancelAnimationFrame;

	globalThis.self = window;
}

export default installPolyfills;
