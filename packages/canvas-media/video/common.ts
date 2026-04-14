import { Screen, Application, Utils, CSSType, PercentLength } from '@nativescript/core';
import { MediaBase } from '../common';

@CSSType('Video')
export abstract class VideoBase extends MediaBase {
	public abstract controls: boolean;
	public loop: boolean;
	public autoplay: boolean;
	public playsinline: boolean;
	public abstract src: string;
	public abstract currentTime: number;
	public abstract readonly duration: number;
	public abstract muted: boolean;
	public static playingEvent = 'playing';
	public static playEvent = 'play';
	public static pauseEvent = 'pause';
	public static timeupdateEvent = 'timeupdate';
	public static durationchangeEvent = 'durationchange';
	public static loadeddataEvent = 'loadeddata';
	public static readonly HAVE_NOTHING = 0;
	public static readonly HAVE_METADATA = 1;
	public static readonly HAVE_CURRENT_DATA = 2;
	public static readonly HAVE_FUTURE_DATA = 3;
	public static readonly HAVE_ENOUGH_DATA = 4;
	public abstract readonly readyState;
	get _realSize(): { width: number; height: number } {
		return {
			width: this.getSize(this.style.width, this.getMeasuredWidth(), 'width'),
			height: this.getSize(this.style.height, this.getMeasuredHeight(), 'height'),
		};
	}

	private getSize(value, measuredSize, type): number {
		if (typeof value === 'string') {
			value = PercentLength.parse(value);
		}
		if (typeof value === 'number') {
			// treat as px
			return value || 0;
		} else if ((value !== null || true) && typeof value === 'object' && typeof value.value && typeof value.unit) {
			if (value.unit === 'px') {
				return value.value || 0;
			} else if (value.unit === 'dip') {
				return Utils.layout.toDevicePixels(value.value) || 0;
			} else if (value.unit === '%') {
				if (Application.orientation() === 'portrait') {
					return type === 'width' ? Screen.mainScreen.widthPixels : Screen.mainScreen.heightPixels * value.value || 0;
				} else if (Application.orientation() === 'landscape') {
					return type === 'width' ? Screen.mainScreen.widthPixels : Screen.mainScreen.heightPixels * value.value || 0;
				} else {
					return 0;
				}
			} else {
				return 0;
			}
		} else {
			return 0;
		}
	}

	abstract play(): Promise<void>;
	abstract pause(): void;
	abstract load(): void;
	_capturedListeners = {};
	_listeners = {};

	_notifyListener(type) {
		const capturedEvents = this._capturedListeners[type];
		if (Array.isArray(capturedEvents)) {
			const size = capturedEvents.length;
			for (let i = 0; i < size; i++) {
				const item = capturedEvents[i];
				if (typeof item === 'function') {
					item();
				}
			}
		}

		const events = this._listeners[type];
		if (Array.isArray(events)) {
			const size = events.length;
			for (let i = 0; i < size; i++) {
				const item = events[i];
				if (typeof this[type] === 'function') {
					this[type]();
				}
				if (typeof item === 'function') {
					item();
				}
			}
		}
	}
	_removeListener(type: string, listener: Function, listeners: any) {
		if (!this._listeners) {
			this._listeners = {};
		}
		if (typeof listener === 'function' && listeners && typeof listeners === 'object') {
			const currentEvents = listeners[type];
			if (Array.isArray(currentEvents)) {
				const size = currentEvents.length;
				let index = -1;
				for (let i = 0; i < size; i++) {
					const item = currentEvents[i];
					if (item === listener) {
						index = i;
						break;
					}
				}
				if (index > -1) {
					currentEvents.splice(index, 1);
				}
			}
		}
	}

	_listenerExist(type: string, listener: Function, listeners: any): boolean {
		let has = false;
		if (typeof listener === 'function' && listeners && typeof listeners === 'object') {
			const currentEvents = listeners[type];
			if (Array.isArray(currentEvents)) {
				const size = currentEvents.length;
				for (let i = 0; i < size; i++) {
					const item = currentEvents[i];
					if (item === listener) {
						has = true;
						break;
					}
				}
			}
		}
		return has;
	}
	addEventListener(type: string, listener: Function, useCapture: boolean | any) {
		if (!this._listeners) {
			this._listeners = {};
		}
		let isCapture = false;
		if (typeof useCapture === 'boolean') {
			isCapture = true;
		} else if (typeof useCapture === 'object') {
			isCapture = useCapture?.capture ?? false;
		}

		if (isCapture) {
			if (!this._listenerExist(type, listener, this._capturedListeners)) {
				if (Array.isArray(this._capturedListeners?.[type])) {
					this._capturedListeners?.[type]?.push?.(listener);
				} else {
					this._capturedListeners[type] = [listener];
				}
			}
		} else {
			if (!this._listenerExist(type, listener, this._listeners)) {
				if (Array.isArray(this._listeners?.[type])) {
					this._listeners?.[type]?.push?.(listener);
				} else {
					this._listeners[type] = [listener];
				}
			}
		}
	}
	removeEventListener(type: string, listener: Function, useCapture: boolean | any) {
		let isCapture = false;
		if (typeof useCapture === 'boolean') {
			isCapture = useCapture;
		} else if (typeof useCapture === 'object') {
			isCapture = useCapture?.capture ?? false;
		}

		if (isCapture) {
			this._removeListener(type, listener, this._capturedListeners);
		} else {
			this._removeListener(type, listener, this._listeners);
		}
	}

	_videoFrameCallbacks: Function[] = [];
	requestVideoFrameCallback(callback: Function) {
		if (typeof callback === 'function') {
			this._videoFrameCallbacks?.push?.(callback);
		}
	}

	cancelVideoFrameCallback() {
		if (this._videoFrameCallbacks.length > 0) {
			this._videoFrameCallbacks.pop();
		}
	}

	_notifyVideoFrameCallbacks() {
		if (this._videoFrameCallbacks.length !== 0) {
			const toRemove = this._videoFrameCallbacks.length;
			this._videoFrameCallbacks.forEach((cb) => {
				cb();
			});
			this._videoFrameCallbacks.splice(0, toRemove);
		}
	}
}
