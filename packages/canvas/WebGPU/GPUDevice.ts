import { Observable } from '@nativescript/core';
import { GPUBuffer } from './GPUBuffer';
import { GPUTexture } from './GPUTexture';
import { native_ } from './Constants';
import { GPUShaderModule } from './GPUShaderModule';

// Doing so because :D
export class EventTarget {
	_emitter?: WeakRef<Observable>;

	addEventListener(event: string, handler: any, options: AddEventListenerOptions = {}) {
		const { capture, once } = options;
		if (capture) {
			//   debug("Bubble propagation is not supported");
		}
		if (once) {
			const oldHandler = handler;
			const self = this;
			handler = (...args: any) => {
				oldHandler.call(null, ...args);
				self.removeEventListener(event, handler);
			};
		}
		let emitter: Observable;

		if (global.isAndroid) {
			emitter = this._emitter?.get?.();
		}

		if (global.isIOS) {
			emitter = this._emitter?.deref?.();
		}
		if (emitter !== null && emitter !== undefined) {
			emitter.addEventListener(event, handler, this);
		}
	}

	removeEventListener(event: string, handler?: any) {
		let emitter: Observable;

		if (global.isAndroid) {
			emitter = this._emitter?.get?.();
		}

		if (global.isIOS) {
			emitter = this._emitter?.deref?.();
		}

		if (emitter !== null && emitter !== undefined) {
			emitter.removeEventListener(event, handler);
		}
	}

	dispatchEvent(event) {
		let emitter: Observable;

		if (global.isAndroid) {
			emitter = this._emitter?.get?.();
		}

		if (global.isIOS) {
			emitter = this._emitter?.deref?.();
		}

		if (emitter !== null && emitter !== undefined) {
			emitter.notify({ ...event, eventName: event.type, object: emitter });
		}
	}
}

interface GPUExtent3DDict {
	width: number;
	height?: number /* default=1 */;
	depthOrArrayLayers?: number /* default=1 */;
}
type GPUExtent3D = [number, number, number] | GPUExtent3DDict;

export class GPUDevice extends EventTarget {
	[native_];
	label = '';
	_lostPromise: Promise<GPUDeviceLostInfo>;

	static fromNative(device) {
		if (device) {
			const ret = new GPUDevice();
			ret[native_] = device;
			return ret;
		}
		return null;
	}
	get lost() {
		if (!this._lostPromise) {
			this._lostPromise = new Promise((resolve, reject) => {
				this[native_].lost
					.then((info) => {
						console.log('lost', info);
						const ret = new GPUDeviceLostInfo();
						ret._native = info;
						resolve(ret);
					})
					.reject((error) => {
						reject(error);
					});
			});
		}

		return this._lostPromise;
	}

	get native() {
		return this[native_];
	}
	get limits() {
		return this.native.limits;
	}

	get features() {
		return this.native.features;
	}

	destroy() {
		this.native.destroy();
	}

	createBuffer(descriptor: { label?: string; mappedAtCreation?: boolean; size: number; usage: number }) {
		const buffer = this.native.createBuffer(descriptor);
		if (buffer) {
			return GPUBuffer.fromNative(buffer, descriptor.mappedAtCreation);
		}
		return undefined;
	}

	createTexture(descriptor: { size: GPUExtent3D; mipLevelCount?: number /* default=1 */; sampleCount?: number /* default=1 */; dimension?: '1d' | '2d' | '3d' /* default="2d" */; format; usage; viewFormats?: any[] /* default=[] */ }) {
		const sizeIsArray = Array.isArray(descriptor.size);

		const opts = {
			mipLevelCount: descriptor?.mipLevelCount ?? 1,
			sampleCount: descriptor?.sampleCount ?? 1,
			dimension: descriptor?.dimension ?? '2d',
			format: descriptor?.format,
			usage: descriptor?.usage,
			viewFormats: descriptor?.viewFormats ?? [],
			width: sizeIsArray ? descriptor.size[0] : (<GPUExtent3DDict>descriptor.size)?.width,
			height: sizeIsArray ? descriptor.size[1] ?? 1 : (<GPUExtent3DDict>descriptor.size)?.height ?? 1,
			depthOrArrayLayers: sizeIsArray ? descriptor.size[2] ?? 1 : (<GPUExtent3DDict>descriptor.size)?.depthOrArrayLayers ?? 1,
		};

		const texture = this.native.createTexture(opts);

		if (texture) {
			return GPUTexture.fromNative(texture);
		}
		return undefined;
	}

	createShaderModule(desc: { label?: string; code: string; sourceMap?: object; compilationHints?: any[] }) {
		const module = this.native.createShaderModule(desc);
		if (module) {
			return GPUShaderModule.fromNative(module);
		}

		return undefined;
	}
}
