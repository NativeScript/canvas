import '@nativescript/macos-node-api';
import { requestAnimationFrame, cancelAnimationFrame } from './utils/raf';
import { Event } from '@nativescript/foundation/dom/dom-utils';

import { GPU, GPUDevice, GPUAdapter, GPUTextureUsage, GPUBufferUsage, CanvasRenderingContext2D, WebGLRenderingContext, WebGL2RenderingContext, createImageBitmap, Path2D } from './js-bindings.js';

function fixup_shader_code(code: string) {
	let ret = `${code}`;
	if (!ret.includes('#storage')) {
		if (ret.includes('atomic_storage')) ret = '#storage atomic_storage array<atomic<i32>>\n\n' + ret;
		if (ret.includes('float_storage')) ret = '#storage float_storage array<vec4<f32>>\n\n' + ret;
	}

	ret = ret.replace(/@stage\(compute\)/g, '@compute');
	ret = ret.replace(/^type /gm, 'alias ');
	ret = ret.replace(/^let /gm, 'const ');
	ret = ret.replace(/alias\s+bool([2-4])\s*=\s*vec\1<\s*bool\s*>\s*;/gm, '');
	ret = ret.replace(/alias\s+float([2-4])x([2-4])\s*=\s*mat\1x\2<\s*f32\s*>\s*;/gm, '');

	ret = ret.replace(/i32\(\s*(\d+)\.\d+\s*\)/g, 'i32($1)');

	// diagnostic is unsupport atm, remove after https://github.com/gfx-rs/wgpu/pull/6148
	ret = ret.replace(/diagnostic\s*\([^)]*\)/g, '');

	// todo: remove after wgpu adds support for textureLoad with u32
	ret = ret.replace(/textureLoad\(\s*[^,]+,\s*[^,]+,\s*[^,]+,\s*0u\s*\)/g, (match) => {
		return match.replace(/0u/, '0');
	});

	// patch switch issue
	ret = ret.replace(/case\s+(\d+)\s*:\s*\{/g, 'case $1u: {');

	return ret;
}

export class ProgressEvent extends Event {
	lengthComputable: boolean;
	loaded: number;
	total: number;

	constructor(type: string, data: { lengthComputable: boolean; loaded: number; total: number }) {
		super(type);
		this.lengthComputable = data.lengthComputable ?? false;
		this.loaded = data.loaded ?? 0;
		this.total = data.total ?? 0;
	}
}

function install2DPolyfills() {
	// @ts-ignore
	globalThis.CanvasRenderingContext2D = CanvasRenderingContext2D;
	// @ts-ignore
	globalThis.Path2D = Path2D;
}

function installWebGLPolyfills() {
	// @ts-ignore
	globalThis.WebGLRenderingContext = WebGLRenderingContext;
}

function installWebGL2Polyfills() {
	// @ts-ignore
	globalThis.WebGL2RenderingContext = WebGL2RenderingContext;
}

// @ts-ignore
GPUDevice.prototype.__createShaderModule = GPUDevice.prototype.createShaderModule;

function installWebGPUPolyfills() {
	// @ts-ignore
	globalThis.GPUTextureUsage = GPUTextureUsage;
	// @ts-ignore
	globalThis.GPUBufferUsage = GPUBufferUsage;
	// @ts-ignore
	globalThis.GPUAdapter = GPUAdapter;
	GPUDevice.prototype.createShaderModule = function (descriptor) {
		const desc = {
			...descriptor,
			code: fixup_shader_code(descriptor.code),
		};

		// @ts-ignore
		return this.__createShaderModule(desc);
	};
}

function installPolyfills(window: any) {
	globalThis.addEventListener = window.addEventListener = () => {};
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

	//@ts-ignore
	globalThis.ProgressEvent = ProgressEvent;

	globalThis.devicePixelRatio = window.devicePixelRatio = NSScreen.mainScreen.backingScaleFactor;
	globalThis.requestAnimationFrame = window.requestAnimationFrame = requestAnimationFrame;
	globalThis.cancelAnimationFrame = window.cancelAnimationFrame = cancelAnimationFrame;
	globalThis.self = window;
	// @ts-ignore
	globalThis.createImageBitmap = function (source, sxOroptions, sy, sw, sh, options) {
		if (source instanceof Blob) {
			return source.arrayBuffer().then((buffer) => {
				return createImageBitmap(new Uint8Array(buffer), sxOroptions, sy, sw, sh, options as never);
			});
		} else {
			return createImageBitmap(source as never, sxOroptions, sy, sw, sh, options as never);
		}
	};

	install2DPolyfills();
	installWebGLPolyfills();
	installWebGL2Polyfills();
	installWebGPUPolyfills();
}

installPolyfills(globalThis.window);
