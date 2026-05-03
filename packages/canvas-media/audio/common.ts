import { CSSType } from '@nativescript/core';
import { MediaBase } from '../common';

function warnAudioListener(label: string, _error?: unknown) {
	try {
		console.warn(label);
	} catch {}
}

@CSSType('Audio')
export abstract class AudioBase extends MediaBase {
	public abstract controls: boolean;
	public abstract loop: boolean;
	public abstract autoplay: boolean;

	public static playingEvent = 'playing';
	public static playEvent = 'play';
	public static pauseEvent = 'pause';
	public static canplayEvent = 'canplay';
	public static canplaythroughEvent = 'canplaythrough';
	public static timeupdateEvent = 'timeupdate';
	public static durationchangeEvent = 'durationchange';
	public static loadedmetadataEvent = 'loadedmetadata';
	public static loadeddataEvent = 'loadeddata';
	public static endedEvent = 'ended';
	public static errorEvent = 'error';

	public static readonly HAVE_NOTHING = 0;
	public static readonly HAVE_METADATA = 1;
	public static readonly HAVE_CURRENT_DATA = 2;
	public static readonly HAVE_FUTURE_DATA = 3;
	public static readonly HAVE_ENOUGH_DATA = 4;

	abstract play(): Promise<void>;
	abstract pause(): void;
	abstract load(): void;

	_capturedListeners = {} as Record<string, Function[]>;
	_listeners = {} as Record<string, Function[]>;

	addEventListener(type: string, listener: Function, useCapture?: boolean | any) {
		if (!this._listeners) this._listeners = {};
		let isCapture = false;
		if (typeof useCapture === 'boolean') isCapture = useCapture;
		else if (typeof useCapture === 'object') isCapture = useCapture?.capture ?? false;

		const target = isCapture ? this._capturedListeners : this._listeners;
		if (!this._listenerExist(type, listener, target)) {
			if (Array.isArray(target[type])) {
				target[type].push(listener);
			} else {
				target[type] = [listener];
			}
		}
	}

	removeEventListener(type: string, listener: Function, useCapture?: boolean | any) {
		let isCapture = false;
		if (typeof useCapture === 'boolean') isCapture = useCapture;
		else if (typeof useCapture === 'object') isCapture = useCapture?.capture ?? false;
		const target = isCapture ? this._capturedListeners : this._listeners;
		this._removeListener(type, listener, target);
	}

	private _invokeListener(type: string, listener: Function, source: string) {
		try {
			listener.call(this);
		} catch (error) {
			warnAudioListener(`Audio.${type} ${source} listener failed`, error);
		}
	}

	_notifyListener(type: string) {
		const captured = this._capturedListeners?.[type];
		if (Array.isArray(captured)) {
			for (let i = 0; i < captured.length; i++) {
				const cb = captured[i];
				if (typeof cb === 'function') this._invokeListener(type, cb, 'captured');
			}
		}

		const handler = (this as any)[`on${type}`];
		if (typeof handler === 'function') {
			this._invokeListener(type, handler, 'property');
		}

		const events = this._listeners?.[type];
		if (Array.isArray(events)) {
			for (let i = 0; i < events.length; i++) {
				const cb = events[i];
				if (typeof cb === 'function') this._invokeListener(type, cb, 'registered');
			}
		}
	}

	_removeListener(type: string, listener: Function, listenersObj: any) {
		if (!listenersObj) return;
		const arr = listenersObj[type];
		if (!Array.isArray(arr)) return;
		const idx = arr.indexOf(listener as any);
		if (idx > -1) arr.splice(idx, 1);
	}

	_listenerExist(type: string, listener: Function, listenersObj: any): boolean {
		if (!listenersObj) return false;
		const arr = listenersObj[type];
		if (!Array.isArray(arr)) return false;
		return arr.indexOf(listener as any) !== -1;
	}
}
