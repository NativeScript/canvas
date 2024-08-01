import { Element } from './Element';
import { ViewBase } from '@nativescript/core';
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
		return 0;
	}
	set width(value) {
		console.log('style.width', value);
	}

	get height() {
		return 0;
	}
	set height(value) {
		console.log('style.height', value);
	}
}

export class HTMLElement extends Element {
	private _style: Style;

	constructor(tagName: string = '') {
		super(tagName ?? '');
		this._style = new Style();
	}

	get style() {
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
}
