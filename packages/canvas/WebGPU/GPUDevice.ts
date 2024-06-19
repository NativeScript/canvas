import { Observable } from '@nativescript/core';
import type { GPUBufferUsage } from './Constants';
import { GPUBuffer } from './GPUBuffer';

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

export class GPUDevice extends EventTarget {
	_native;
	label = '';
	_lostPromise: Promise<GPUDeviceLostInfo>;
	get lost() {
		if (!this._lostPromise) {
			this._lostPromise = new Promise((resolve, reject) => {
				this._native.lost
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

	get limits() {
		return this._native.limits;
	}

	get features() {
		return this._native.features;
	}

	destroy() {
		this._native.destroy();
	}

	createBuffer(descriptor: { label?: string; mappedAtCreation?: boolean; size: number; usage: number }) {
		const buffer = this._native.createBuffer(descriptor);
		if (buffer) {
			const ret = new GPUBuffer();
			if (descriptor.mappedAtCreation) {
				ret['_mapState'] = 'mapped';
			}
			ret.native_ = buffer;
			return ret;
		}
		return undefined;
	}
}
