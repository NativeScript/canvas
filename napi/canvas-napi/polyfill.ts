import '@nativescript/macos-node-api';
import { cancelAnimationFrame, requestAnimationFrame } from './utils/raf';
import { Event } from '@nativescript/foundation/dom/dom-utils';

import { CanvasRenderingContext2D, createImageBitmap, GPU, GPUAdapter, GPUBufferUsage, GPUDevice, GPUTextureUsage, ImageAsset, Path2D, WebGL2RenderingContext, WebGLRenderingContext } from './js-bindings.js';
import { ViewBase } from '@nativescript/foundation/views/view/view-base';
import { view } from '@nativescript/foundation/views/decorators/view';

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

function install2DPolyfills(window: any) {
	// @ts-ignore
	globalThis.CanvasRenderingContext2D = window.CanvasRenderingContext2D = CanvasRenderingContext2D;
	// @ts-ignore
	globalThis.Path2D = window.Path2D = Path2D;
}

function installWebGLPolyfills(window: any) {
	// @ts-ignore
	globalThis.WebGLRenderingContext = window.WebGLRenderingContext = WebGLRenderingContext;
}

function installWebGL2Polyfills(window: any) {
	// @ts-ignore
	globalThis.WebGL2RenderingContext = window.WebGL2RenderingContext = WebGL2RenderingContext;
}

// @ts-ignore
WebGLRenderingContext.prototype.__shaderSource = WebGLRenderingContext.prototype.shaderSource;

WebGLRenderingContext.prototype.shaderSource = function (shader, source) {
	// @ts-ignore
	this.__shaderSource(shader, source.replace(/precision\s+\w+\s+\w+;\s*/g, ''));
};

// @ts-ignore
WebGL2RenderingContext.prototype.__shaderSource = WebGL2RenderingContext.prototype.shaderSource;

WebGL2RenderingContext.prototype.shaderSource = function (shader, source) {
	// @ts-ignore
	this.__shaderSource(shader, source.replace(/precision\s+\w+\s+\w+;\s*/g, ''));
};

// @ts-ignore
GPUDevice.prototype.__createShaderModule = GPUDevice.prototype.createShaderModule;

function installWebGPUPolyfills(window: any) {
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

type ScreenOrientationType = 'portrait-primary' | 'portrait-secondary' | 'landscape-primary' | 'landscape-secondary';

class ScreenOrientation {
	angle: number;
	type: ScreenOrientationType;

	constructor(angle: number, type: ScreenOrientationType) {
		this.angle = angle;
		this.type = type;
	}
}

class Screen {
	get orientation() {
		const { width, height } = NSScreen.mainScreen.frame.size;
		if (width > height) {
			return new ScreenOrientation(0, 'landscape-primary');
		}
		return new ScreenOrientation(0, 'portrait-primary');
	}
}

const _navigator = navigator;

class Navigator {
	get appCodeName() {
		return _navigator.appCodeName;
	}

	appName() {
		return _navigator.appName;
	}

	product = 'NativeScript';
	_userAgent = '';
	get userAgent() {
		if (this._userAgent === '') {
			const processInfo = NSProcessInfo.processInfo;
			const version = processInfo.operatingSystemVersion;
			const osVersion = `Mac OS X ${version.majorVersion}.${version.minorVersion}.${version.patchVersion}`;
			//@ts-ignore
			this._userAgent = `Mozilla/5.0 (${_navigator.platform === 'MacIntel' ? 'Macintosh' : _navigator.platform}; ${_navigator.platform.replace('Mac', '')} ${osVersion};) (KHTML, like Gecko) Safari/537.36 NativeScript/` + `${globalThis.__runtimeVersion ?? '0.1.4'}`.replaceAll('"', '');
		}
		return this._userAgent;
	}

	get vendor() {
		return _navigator.vendor;
	}

	get vendorSub() {
		return _navigator.vendorSub;
	}

	_appVersion = '';

	get appVersion() {
		if (this._appVersion === '') {
			const processInfo = NSProcessInfo.processInfo;
			const version = processInfo.operatingSystemVersion;
			const osVersion = `Mac OS X ${version.majorVersion}.${version.minorVersion}.${version.patchVersion}`;
			// @ts-ignore
			this._appVersion = `5.0 (${_navigator.platform === 'MacIntel' ? 'Macintosh' : _navigator.platform}; ${_navigator.platform.replace('Mac', '')} ${osVersion};) (KHTML, like Gecko) Safari/537.36 NativeScript/` + `${globalThis.__runtimeVersion ?? '0.1.4'}`.replaceAll('"', '');
		}
		return this._appVersion;
	}

	get maxTouchPoints() {
		return _navigator.maxTouchPoints;
	}

	standalone = true;

	language() {
		return _navigator.language;
	}

	private _gpu = GPU.getInstance();
	get gpu() {
		return this._gpu;
	}

	get platform() {
		return _navigator.platform;
	}
}

@view({
	name: 'HTMLImageElement',
	tagName: 'img',
})
class HTMLImageElement extends ViewBase {
	_image: ImageAsset;
	complete = false;
	_onload: any;
	_onerror: any;

	constructor() {
		super();
		this._image = new ImageAsset();
	}

	get naturalWidth() {
		return this.width;
	}

	get naturalHeight() {
		return this.height;
	}

	get width() {
		return this._image.width;
	}

	get height() {
		return this._image.height;
	}

	get onload() {
		return this._onload;
	}

	set onload(value) {
		this._onload = value;
	}

	get onerror() {
		return this._onerror;
	}

	set onerror(value) {
		this._onerror = value;
	}

	_src = '';
	get src() {
		return this._src;
	}

	set src(value: string) {
		this._src = value;
		if (typeof value === 'string') {
			if (value.startsWith('data:')) {
				this._image
					.fromBase64(value.split('base64,')[1])
					.then(() => {
						this.complete = true;
						this.dispatchEvent(new Event('load'));
						this._onload?.();
					})
					.catch(() => {
						this.complete = false;
						this.dispatchEvent(new Event('error'));
						this._onerror?.();
					});
				return;
			} else if (value.startsWith('http')) {
				this._image
					.fromUrl(value)
					.then(() => {
						this.complete = true;
						this.dispatchEvent(new Event('load'));
						this._onload?.();
					})
					.catch(() => {
						this.complete = false;
						this.dispatchEvent(new Event('error'));
						this._onerror?.();
					});
				return;
			}

			this._image
				.fromFile(value)
				.then(() => {
					this.complete = true;
					this.dispatchEvent(new Event('load'));
					this._onload?.();
				})
				.catch((e) => {
					this.complete = false;
					this.dispatchEvent(new Event('error'));
					this._onerror?.();
				});
		}
	}
}

HTMLImageElement.register();

@view({
	name: 'HTMLVideoElement',
	tagName: 'video',
})
class HTMLVideoElement extends ViewBase {
	public controls: boolean = true;
	public loop: boolean = false;
	public autoplay: boolean = false;
	public playsinline: boolean = false;
	public src: string | undefined;
	public currentTime: number = 0;
	public readonly duration: number = 0;
	public muted: boolean = false;

	canPlayType(type: string): '' | 'probably' | 'maybe' {
		switch (type) {
			case 'video/mp4':
			case 'video/ogg':
				return 'probably';
			default:
				return '';
		}
	}
}

HTMLVideoElement.register();

@view({
	name: 'SVGRectElement',
	tagName: 'rect',
})
class SVGRectElement extends ViewBase {
	getBoundingClientRect() {
		const layout = this.yogaNode.getComputedLayout();
		return new DOMRect(layout.left, layout.top, layout.width, layout.height) as never;
	}
}

SVGRectElement.register();

function installPolyfills(window: any) {
	// NSEvent.addLocalMonitorForEventsMatchingMaskHandler(NSEventMask.KeyDown, (event) => {
	// 	console.log('event');
	// });
	//@ts-ignore
	/*
	document.__addEventListener = document.addEventListener;
	document.addEventListener = function(eventName, callback) {
		//@ts-ignore
		this.__addEventListener(eventName, callback);
	};
	*/
	globalThis.addEventListener = window.addEventListener = () => {};
	globalThis.removeEventListener = window.removeEventListener = () => {};
	if (!('console' in window)) {
		window.console = globalThis.console;
	}
	window.setTimeout = globalThis.setTimeout;
	window.clearTimeout = globalThis.clearTimeout;
	window.setInterval = globalThis.setInterval;
	window.clearInterval = globalThis.clearInterval;
	window.setImmediate = globalThis.setImmediate;
	window.clearImmediate = globalThis.clearImmediate;
	window.navigator = new Navigator() as never;
	Object.defineProperty(globalThis, 'navigator', {
		value: window.navigator,
		writable: true,
	});

	globalThis.Image = window.Image = HTMLImageElement as never;

	window.screen = new Screen();

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

	// todo remove after getComputedStyle
	if (typeof window.getComputedStyle === 'undefined') {
		window.getComputedStyle = function () {};
	}

	// @ts-ignore
	if (typeof document.defaultView.getComputedStyle === 'undefined') {
		// @ts-ignore
		document.defaultView.getComputedStyle = function () {};
	}

	//@ts-ignore
	globalThis.ProgressEvent = ProgressEvent;

	globalThis.devicePixelRatio = window.devicePixelRatio = NSScreen.mainScreen.backingScaleFactor;
	globalThis.requestAnimationFrame = window.requestAnimationFrame = requestAnimationFrame;
	globalThis.cancelAnimationFrame = window.cancelAnimationFrame = cancelAnimationFrame;
	globalThis.self = window;
	// @ts-ignore
	globalThis.createImageBitmap = function (source, sxOrOptions, sy, sw, sh, options) {
		if (source instanceof Blob) {
			return source.arrayBuffer().then((buffer) => {
				return createImageBitmap(new Uint8Array(buffer), sxOrOptions, sy, sw, sh, options as never);
			});
		} else {
			return createImageBitmap(source as never, sxOrOptions, sy, sw, sh, options as never);
		}
	};

	install2DPolyfills(window);
	installWebGLPolyfills(window);
	installWebGL2Polyfills(window);
	installWebGPUPolyfills(window);
}

installPolyfills(globalThis.window);
