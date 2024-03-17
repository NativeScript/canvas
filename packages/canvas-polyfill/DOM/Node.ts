import { EventTarget } from './EventTarget';
import setValue from 'set-value';
import { ViewBase } from '@nativescript/core';

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
		const nativeElement = this.nativeElement.deref();
		if (nativeElement !== null) {
			setValue(nativeElement, key, val);
		}
	}

	getPropertyValue(key: string | symbol) {
		const nativeElement = this.nativeElement.deref();
		if (nativeElement !== null) {
			return nativeElement[key];
		}
		return this._values.get(key);
	}
}

let id = 0;

export class Node extends EventTarget {
	className: any;
	readonly nodeName: string;

	_id: string;
	_style = new Style();

	constructor(nodeName: string) {
		super();

		this.className = {
			baseVal: ''
		};
		this.nodeName = nodeName;
	}

	set id(value) {
		this._id = value;
		const nativeElement = this['nativeElement'];
		if (nativeElement) {
			setValue(nativeElement, 'id', value);
		}
	}

	get id() {
		return this._id;
	}

	get ownerDocument() {
		return window.document;
	}

	get style() {

		return this._style;
	}

	appendChild(view) {
	}

	insertBefore(view) {
	}

	removeChild(view) {
	}

	setAttributeNS() {
	}
}
