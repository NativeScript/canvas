import { Observable } from '@nativescript/core';

export class EventTarget {
	_emitter?: WeakRef<Observable>;

	addEventListener(event: string, handler: any, options: AddEventListenerOptions = {}) {
		if (typeof options === 'boolean') {
			options = { capture: options };
		}
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
			switch (typeof handler) {
				case 'object':
					if (handler !== null) {
						emitter.addEventListener(event, handler[`on${event}`], this);
					}
					break;
				default:
					emitter.addEventListener(event, handler, this);
					break;
			}
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
			switch (typeof handler) {
				case 'object':
					if (handler !== null) {
						emitter.removeEventListener(event, handler[`on${event}`]);
					}
					break;
				default:
					emitter.removeEventListener(event, handler);
					break;
			}

			emitter.removeEventListener(event, handler);
		}
	}

	/**
	 * Dispatches an event to this EventTarget.
	 * https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/dispatchEvent
	 * @param event
	 * @returns false if event is cancelable, and at least one of the event handlers which received event called Event.preventDefault(). Otherwise true.
	 */
	dispatchEvent(event): boolean {
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

		return !event?.defaultPrevented;
	}
}
