import { Helpers } from '../helpers';
import { adapter_, contextPtr_, GPUTextureUsage, native_ } from './Constants';
import type { GPUDevice } from './GPUDevice';
import { GPUTexture } from './GPUTexture';
import type { GPUAdapter } from './GPUAdapter';
import type { GPUCanvasAlphaMode, GPUCanvasPresentMode, GPUExtent3D, GPUTextureFormat } from './Types';
export class GPUCanvasContext {
	_type;
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
			}
		}

		opts.device = options?.device?.[native_];
		this[native_].configure(opts);
	}

	unconfigure() {
		this[native_].unconfigure();
	}

	getCurrentTexture() {
		const texture = this[native_].getCurrentTexture();
		if (texture) {
			return GPUTexture.fromNative(texture);
		}

		return null;
	}

	presentSurface(texture: GPUTexture) {
		this[native_].presentSurface(texture?.[native_]);
	}

	getCapabilities(adapter: GPUAdapter): {
		format: GPUTextureFormat[];
		presentModes: ('autoVsync' | 'autoNoVsync' | 'fifo' | 'fifoRelaxed' | 'immediate' | 'mailbox')[];
		alphaModes: GPUCanvasAlphaMode;
		usages: number;
	} {
		return this[native_].getCapabilities(adapter[native_]);
	}
}
