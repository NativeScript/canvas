import '@nativescript/macos-node-api';
import { createRequire } from 'node:module';
import utils from './utils';
import { Event } from '@nativescript/foundation/dom/dom-utils.js';

// @ts-ignore
const require = createRequire(import.meta.url);

const { GPU, GPUDevice, GPUAdapter, GPUTextureUsage, GPUBufferUsage } = require('./canvas-napi.darwin-arm64.node');

const { requestAnimationFrame, cancelAnimationFrame } = utils;

export class ProgressEvent extends Event {
	lengthComputable: boolean;
	loaded: number;
	total: number;

	constructor(
		type: string,
		data: { lengthComputable: boolean; loaded: number; total: number; target: any } = {
			lengthComputable: false,
			loaded: 0,
			total: 0,
			target: {},
		}
	) {
		super(type);
		this.lengthComputable = data.lengthComputable ?? false;
		this.loaded = data.loaded ?? 0;
		this.total = data.total ?? 0;
	}
}

function installPolyfills(window) {
	window.addEventListener = () => {};
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

	globalThis.GPUTextureUsage = GPUTextureUsage;
	globalThis.GPUBufferUsage = GPUBufferUsage;
	globalThis.ProgressEvent = ProgressEvent;
}

installPolyfills(globalThis.window);

export default installPolyfills;
