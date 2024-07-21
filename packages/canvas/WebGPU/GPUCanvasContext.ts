import { Helpers } from '../helpers';
import { contextPtr_, native_ } from './Constants';
import type { GPUDevice } from './GPUDevice';
import { GPUTexture } from './GPUTexture';
import type { GPUAdapter } from './GPUAdapter';
import type { GPUTextureFormat } from './Types';
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

	configure(options: { device: GPUDevice; format: any; usage?: number /* default=0x10 - GPUTextureUsage.RENDER_ATTACHMENT */; viewFormats?: number[] /* default=[] */; colorSpace?: 'display-p3' | 'srgb' /* default="srgb" */; alphaMode?: 'opaque' | 'premultiplied' | 'postmultiplied' | 'inherit'; presentModes?: 'autoVsync' | 'autoNoVsync' | 'fifo' | 'fifoRelaxed' | 'immediate' | 'mailbox' }) {
		const opts = {
			usage: global.GPUTextureUsage.RENDER_ATTACHMENT,
			colorSpace: 'srgb',
			alphaMode: 'opaque',
			presentMode: 'fifo',
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

	presentSurface(texture: GPUTexture) {
		this[native_].presentSurface(texture?.[native_]);
	}

	getCapabilities(adapter: GPUAdapter): {
		format: GPUTextureFormat[];
		presentModes: ('autoVsync' | 'autoNoVsync' | 'fifo' | 'fifoRelaxed' | 'immediate' | 'mailbox')[];
		alphaModes: 'opaque' | 'premultiplied' | 'postmultiplied' | 'inherit';
		usages: number;
	} {
		return this[native_].getCapabilities(adapter[native_]);
	}
}
