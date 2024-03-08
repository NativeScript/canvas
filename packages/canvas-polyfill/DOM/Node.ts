import { fromObject, Observable } from '@nativescript/core';

(Observable as any).prototype.addListener = function (eventNames, callback, thisArg) {
	this.on(eventNames, callback, thisArg);
};

(Observable as any).prototype.removeListener = function (eventNames, callback, thisArg) {
	this.off(eventNames, callback, thisArg);
};

(Observable as any).prototype.emit = function (eventNames, thisArg) {
	this.notify({
		eventName: eventNames,
		object: null,
	});
};

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
			},
		});
		this._values = values;
	}
	_values: Map<any, any>;
	__internalElement;
	setProperty(key, val) {
		console.log(this.__internalElement, key, val);
		// todo check key + val
		this._values.set(key, val);
		if (this.__internalElement) {
			this.__internalElement._dom.documentElement.setAttribute('style', this._styleToString());
		}
	}
	getPropertyValue(key) {
		return this._values.get(key);
	}

	_styleToString() {
		let style = '';
		this._values.forEach((val, key) => {
			style = `${style}${key}:${val};`;
		});
		return style;
	}
}

export class Node {
	emitter: any;
	className: any;
	nodeName: string;
	_canvas: any;
	___internalElement: any;
	set __internalElement(value) {
		this.___internalElement = value;
		this._style.__internalElement = value;
	}
	get __internalElement() {
		return this.___internalElement;
	}

	_id;
	_style = new Style();
	set id(value) {
		this._id = value;
		if (this.__internalElement) {
			this.__internalElement._dom.documentElement.setAttribute('id', value);
		}
	}
	get id() {
		return this._id;
	}
	constructor(nodeName) {
		this.emitter = fromObject({});
		this.addEventListener = this.addEventListener.bind(this);
		this.removeEventListener = this.removeEventListener.bind(this);
		this._checkEmitter = this._checkEmitter.bind(this);
		this.className = {
			baseVal: '',
		};
		this.nodeName = nodeName;
	}

	get ownerDocument() {
		return window.document;
	}

	get style() {
		return this._style;
	}

	_checkEmitter() {
		if (!this.emitter || !(this.emitter.on || this.emitter.addEventListener || this.emitter.addListener)) {
			this.emitter = fromObject({});
		}
	}

	addEventListener(eventName, listener, thisArg) {
		this._checkEmitter();
		thisArg = typeof thisArg === 'object' ? thisArg : this;
		if (this.emitter.on) {
			this.emitter.on(eventName, listener, thisArg);
		} else if (this.emitter.addEventListener) {
			this.emitter.addEventListener(eventName, listener, thisArg);
		} else if (this.emitter.addListener) {
			this.emitter.addListener(eventName, listener, thisArg);
		}
	}

	removeEventListener(eventName, listener, thisArg) {
		this._checkEmitter();
		thisArg = typeof thisArg === 'object' ? thisArg : this;
		if (this.emitter.off) {
			this.emitter.off(eventName, listener, thisArg);
		} else if (this.emitter.removeEventListener) {
			this.emitter.removeEventListener(eventName, listener, thisArg);
		} else if (this.emitter.removeListener) {
			this.emitter.removeListener(eventName, listener, thisArg);
		}
	}

	appendChild(view) {
		// console.log('appendChild', view);
	}

	insertBefore(view) {
		// console.log('insertBefore', view);
	}

	removeChild(view) {
		// console.log('removeChild', view);
	}

	setAttributeNS() {}

	querySelectorAll(selector) {
		return (this as any)._instance?.querySelectorAll?.(selector) ?? [];
	}

	querySelector(selector) {
		const ret = (this as any)._instance?.querySelectorAll?.(selector);
		let element = ret?.[0] ?? null;
		if (ret === undefined) {
			const items = (this as any)._instance.getElementsByTagName(selector);
			element = items[0];
		}

		if (element) {
			return new (Element as any)(element);
		}
		return null;
	}

	getBoundingClientRect() {
		if (this._canvas) {
			return this._canvas.getBoundingClientRect();
		}
		return {
			left: 0,
			top: 0,
			right: window.innerWidth,
			bottom: window.innerHeight,
			x: 0,
			y: 0,
			width: window.innerWidth,
			height: window.innerHeight,
		};
	}
}
