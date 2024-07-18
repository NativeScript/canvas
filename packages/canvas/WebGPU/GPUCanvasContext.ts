import { Helpers } from '../helpers';
import { contextPtr_, native_ } from './Constants';
import { GPUDevice } from './GPUDevice';
import { GPUTexture } from './GPUTexture';
import { GPUAdapter } from './GPUAdapter';
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

	configure(options: { device: GPUDevice; format: any; usage?: number /* default=0x10 - GPUTextureUsage.RENDER_ATTACHMENT */; viewFormats?: number[] /* default=[] */; colorSpace?: 'display-p3' | 'srgb' /* default="srgb" */; alphaMode?: 'opaque' | 'premultiplied' }) {
		const opts = {
			usage: global.GPUTextureUsage.RENDER_ATTACHMENT,
			colorSpace: 'srgb',
			alphaMode: 'opaque',
			...options,
		};

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

	presentSurface(texture) {
		this[native_].presentSurface(texture?.[native_]);
	}

	capabilities(adapter: GPUAdapter) {
		return this[native_].capabilities(adapter[native_]);
	}
}
