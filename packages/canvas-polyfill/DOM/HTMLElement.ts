import { DOMMatrix } from '@nativescript/canvas';
import { Element } from './Element';
import { Utils, ViewBase } from '@nativescript/core';
import setValue from 'set-value';

const transformRegex = /(?:(translate(?:3d|X|Y|Z)?)\s*\(\s*(-?\d*\.?\d+(?:px|%|em|rem|vw|vh)?)\s*(?:,\s*(-?\d*\.?\d+(?:px|%|em|rem|vw|vh)?)\s*(?:,\s*(-?\d*\.?\d+(?:px|%|em|rem|vw|vh)?)\s*)?)?\))/g;

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

	__item(index: number) {
		return Array.from(this._values.values())[index];
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

	private _transform: DOMMatrix = new DOMMatrix();
	private _transformString: string = '';

	get transform() {
		return this._transformString;
	}

	set transform(value) {
		if (!value) {
			this._transform = new DOMMatrix();
			this._transformString = '';
			this._values.delete('transform');
		} else {
			const matches = value.matchAll(transformRegex);
			for (const match of matches) {
				switch (match[1]) {
					case 'translate':
						this._transform = this._transform.translate(parseFloat(match[2]), parseFloat(match[3]));
						this._transformString = value;
						this._values.set('transform', value);
						break;
					case 'translate3d':
						{
							const matrix = new DOMMatrix();
							matrix.m41 = parseFloat(match[2]);
							matrix.m42 = parseFloat(match[3]);
							matrix.m43 = parseFloat(match[4]);
							this._transform.multiplySelf(matrix);
							this._transformString = value;
							this._values.set('transform', value);
						}
						break;
					case 'translateX':
						this._transform = this._transform.translate(parseFloat(match[2]), 0);
						this._transformString = value;
						this._values.set('transform', value);
						break;
					case 'translateY':
						this._transform = this._transform.translate(0, parseFloat(match[2]));
						this._transformString = value;
						this._values.set('transform', value);
						break;
					case 'translateZ':
						{
							const matrix = new DOMMatrix();
							matrix.m43 = parseFloat(match[2]);
							this._transform.multiplySelf(matrix);
							this._transformString = value;
							this._values.set('transform', value);
						}
						break;
				}
			}
		}
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
