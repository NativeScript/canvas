import { Observable } from '@nativescript/core';

export class EventTarget {
	_emitter?: WeakRef<Observable>;

	private _getEmitter(): Observable | undefined {
		if (__ANDROID__) {
			return this._emitter?.get?.();
		}
		return this._emitter?.deref?.();
	}

	addEventListener(event: string, handler: any, options: AddEventListenerOptions = {}) {
		if (typeof options === 'boolean') {
			options = { capture: options };
		}
		const { once } = options;
		if (once) {
			const originalHandler = handler;
			const self = this;
			handler = (...args: any) => {
				originalHandler.call(null, ...args);
				self.removeEventListener(event, handler);
			};
		}
		const emitter = this._getEmitter();
		if (emitter != null) {
			if (typeof handler === 'object' && handler !== null) {
				emitter.addEventListener(event, handler[`on${event}`], this);
			} else {
				emitter.addEventListener(event, handler, this);
			}
		}
	}

	removeEventListener(event: string, handler?: any) {
		const emitter = this._getEmitter();
		if (emitter != null) {
			if (typeof handler === 'object' && handler !== null) {
				emitter.removeEventListener(event, handler[`on${event}`]);
			} else {
				emitter.removeEventListener(event, handler);
			}
		}
	}

	/**
	 * Dispatches an event to this EventTarget.
	 * https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/dispatchEvent
	 */
	dispatchEvent(event): boolean {
		const emitter = this._getEmitter();
		if (emitter != null) {
			emitter.notify({ ...event, eventName: event.type, object: emitter });
		}
		return !event?.defaultPrevented;
	}
}
