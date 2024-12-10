import '@nativescript/macos-node-api';
import { createRequire } from 'node:module';

// @ts-ignore
const require = createRequire(import.meta.url);

const { GPU, GPUDevice, GPUAdapter } = require('./canvas-napi.darwin-arm64.node');
import utils from './utils';

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

	Object.defineProperty(window, 'requestAnimationFrame', {
		value: requestAnimationFrame,
		writable: true,
	});

	Object.defineProperty(window, 'cancelAnimationFrame', {
		value: cancelAnimationFrame,
		writable: true,
	});

	globalThis.self = window;

	console.log(navigator.gpu);
}

export default installPolyfills;
