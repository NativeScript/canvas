import { Element } from './Element';
import { Utils, ViewBase } from '@nativescript/core';
import setValue from 'set-value';

export class Style {
	private _values: Map<string, any>;
	nativeElement: WeakRef<ViewBase>;

	constructor() {
		this._values = new Map();
	}

	private _nativeElement() {
		if (__ANDROID__) {
			return this.nativeElement?.get?.();
		}
		if (__IOS__) {
			return this.nativeElement?.deref?.();
		}
		return undefined;
	}

	get width() {
		return this._values.get('width');
	}
	set width(value) {
		this._values.set('width', value);
	}

	get height() {
		return this._values.get('height');
	}
	set height(value) {
		this._values.set('height', value);
	}
}

export class HTMLElement extends Element {
	private _style: Style;

	constructor(tagName: string = '') {
		super(tagName ?? '');
		this._style = new Style();
	}

	get style() {
		if (this._nativeElement) {
			return this._nativeElement.style;
		}
		return this._style;
	}

	set nativeElement(value) {
		this._nativeElement = value;
		if (value) {
			this._emitter = new WeakRef(value);
			this._style.nativeElement = new WeakRef(value);
		} else {
			this._emitter = value as never;
			this._style.nativeElement = value as never;
		}
	}

	get nativeElement() {
		return this._nativeElement;
	}

	get isConnected() {
		return this.nativeElement?.isConnected ?? !!this.nativeElement?.parent ?? false;
	}

	get lang() {
		if (__ANDROID__) {
			const ctx = Utils.android.getApplicationContext();
			return androidx.core.os.ConfigurationCompat.getLocales(ctx.getResources().getConfiguration()).get(0).getLanguage();
		}

		if (__IOS__) {
			return NSLocale.currentLocale.languageCode;
		}

		return 'unknown';
	}

	set lang(value: string) {
		if (__ANDROID__) {
			try {
				const ctx = Utils.android.getApplicationContext();
				ctx.getResources().getConfiguration().setLocale(new java.util.Locale(value));
			} catch (error) {}
		}
	}
}
