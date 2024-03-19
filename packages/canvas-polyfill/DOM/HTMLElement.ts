import { Element } from './Element';
import { ViewBase } from '@nativescript/core';
import setValue from 'set-value';

export class Style {
	proxy;

	constructor() {
		const values = new Map();
		this.proxy = new Proxy(this, {
			set(target, prop, value) {
				target.setProperty(prop, value);
				return true;
			},
			get(target, prop, receiver) {
				return target.getPropertyValue(prop);
			}
		});
		this._values = values;
	}

	_values: Map<any, any>;
	nativeElement: WeakRef<ViewBase>;

	setProperty(key: string | symbol, val: unknown) {
		this._values.set(key, val);
		// const nativeElement = this.nativeElement?.deref?.();
		// if (nativeElement !== null) {
		// 	setValue(nativeElement, key, val);
		// }
	}

	getPropertyValue(key: string | symbol) {
		// const nativeElement = this.nativeElement?.deref?.();
		// if (nativeElement !== null) {
		// 	return nativeElement[key];
		// }
		return this._values.get(key);
	}
}

export class HTMLElement extends Element {

	private _style = new Style();

	constructor(tagName: string = '') {
		super(tagName ?? '');
	}

	get style() {
		if (this.nativeElement) {
			return this.nativeElement.style;
		}
		return this._style;
	}

}
