import { Element } from './Element';
import { ViewBase } from '@nativescript/core';
import setValue from 'set-value';

export class Style {
	_proxy;
	constructor() {
		const values = new Map();
		this._values = values;
		this._proxy = new Proxy(this, {
			set(target, prop, value) {
				target.setProperty(prop, value);
				return true;
			},
			get(target, prop, receiver) {
				return target.getPropertyValue(prop);
			},
		});
	}

	_values: Map<any, any>;
	nativeElement: WeakRef<ViewBase>;

	setProperty(key: string | symbol, val: unknown) {
		const nativeElement = this.nativeElement?.deref?.();
		let value = val;
		if (typeof value === 'string' && value.includes('px')) {
			value = value.replace('px', '');
		}
		if (nativeElement !== null) {
			setValue(nativeElement, key, val);
		}
		this._values.set(key, val);
	}

	getPropertyValue(key: string | symbol) {
		const nativeElement = this.nativeElement?.deref?.();
		if (nativeElement !== null) {
			return nativeElement[key];
		}
		return this._values.get(key);
	}
}

export class HTMLElement extends Element {
	private _style: Style;

	constructor(tagName: string = '') {
		super(tagName ?? '');
		this._style = new Style();
	}

	get style() {
		if (this.nativeElement) {
			return this.nativeElement.style;
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
}
