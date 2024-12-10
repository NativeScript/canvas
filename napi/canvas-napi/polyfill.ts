import '@nativescript/macos-node-api';
import { createRequire } from 'node:module';

// @ts-ignore
const require = createRequire(import.meta.url);

const { GPU, GPUDevice, GPUAdapter } = require('./canvas-napi.darwin-arm64.node');

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
}

export default installPolyfills;
