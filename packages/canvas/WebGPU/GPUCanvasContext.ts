import { Helpers } from '../helpers';
import { adapter_, contextPtr_, GPUTextureUsage, native_ } from './Constants';
import type { GPUDevice } from './GPUDevice';
import { GPUTexture } from './GPUTexture';
import type { GPUAdapter } from './GPUAdapter';
import type { GPUCanvasAlphaMode, GPUCanvasPresentMode, GPUExtent3D, GPUTextureFormat } from './Types';
export class GPUCanvasContext {
	_type;
	_canvas: any;
	static {
		Helpers.initialize();
	}

	[native_];
	[contextPtr_];

	constructor(context: any, contextOptions) {
		let nativeContext = '0';
		if (global.isAndroid) {
			nativeContext = context.getNativeContext().toString();
		}

		if (global.isIOS) {
			nativeContext = context.nativeContext.toString();
		}

		const ctxPtr = BigInt(nativeContext);
		this[native_] = global.CanvasModule.createWebGPUContextWithPointer(ctxPtr);
		this[contextPtr_] = context;
		this._type = 'webgpu';
	}

	get context() {
		return this[native_];
	}

	get contextPtr() {
		return this[contextPtr_];
	}

	get native() {
		return this[native_];
	}

	get canvas() {
		return this._canvas;
	}

	configure(options: { device: GPUDevice; format: GPUTextureFormat; usage?: number; viewFormats?: GPUTextureFormat[]; colorSpace?: 'display-p3' | 'srgb'; alphaMode?: GPUCanvasAlphaMode; presentMode?: GPUCanvasPresentMode; size?: GPUExtent3D }) {
		const opts = {
			usage: GPUTextureUsage.RENDER_ATTACHMENT,
			colorSpace: 'srgb',
			alphaMode: 'opaque',
			presentMode: 'fifo',
			...options,
		};
		if (__ANDROID__ || __IOS__) {
			const capabilities = this.getCapabilities(options?.device?.[adapter_]);

			if (!options.presentMode) {
				opts.presentMode = capabilities.presentModes[0];
			}

			if (!options.alphaMode) {
				opts.alphaMode = capabilities.alphaModes[0];
			} else {
				if (!capabilities.alphaModes.includes(options.alphaMode) && (options.alphaMode === 'opaque' || options.alphaMode === 'premultiplied')) {
					opts.alphaMode = capabilities.alphaModes[0];
					console.warn(`GPUCanvasContext: configure alphaMode ${options.alphaMode} unsupported falling back to ${capabilities.alphaModes[0]}`);
				}
			}

			if (__ANDROID__ && !capabilities.format.includes(options.format) && (options.format === 'bgra8unorm' || options.format === 'bgra8unorm-srgb')) {
				opts.format = capabilities.format[0];
				// fallback to rgba8unorm ... Android 🤪
				if (opts.format === 'rgba8unorm-srgb') {
					opts.format = 'rgba8unorm';
				}
				console.warn(`GPUCanvasContext: configure format ${options.format} unsupported falling back to ${opts.format}`);
			}

			if (__IOS__ && !capabilities.format.includes(options.format)) {
				opts.format = capabilities.format.filter((value) => {
					return value.indexOf('srgb') === -1;
				})[0];
				console.warn(`GPUCanvasContext: configure format ${options.format} unsupported falling back to ${opts.format}`);
			}

			if (__IOS__ && opts.usage > capabilities.usages) {
				opts.usage = capabilities.usages;
				console.warn(`GPUCanvasContext: configure usage unsupported falling back to ${capabilities.usages}`);
			}
		}

		opts.device = options?.device?.[native_];
		this[native_].configure(opts);
	}

	unconfigure() {
		this[native_].unconfigure();
	}

	private _currentTexture: GPUTexture;
	getCurrentTexture() {
		if (this._currentTexture) {
			return this._currentTexture;
		}
		const texture = this[native_].getCurrentTexture();

		if (texture) {
			this._currentTexture = GPUTexture.fromNative(texture);
		} else {
			this._currentTexture = null;
		}

		return this._currentTexture;
	}

	presentSurface() {
		if (this._currentTexture) {
			this[native_].presentSurface(this._currentTexture[native_]);
			this._currentTexture = null;
		} else {
			console.warn('call getCurrentTexture: before presentSurface');
		}
	}

	getCapabilities(adapter: GPUAdapter): {
		format: GPUTextureFormat[];
		presentModes: GPUCanvasPresentMode[];
		alphaModes: GPUCanvasAlphaMode;
		usages: number;
	} {
		return this[native_].getCapabilities(adapter[native_]);
	}
}
