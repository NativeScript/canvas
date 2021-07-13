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

export class Node {
	emitter: any;
	style: any;
	className: any;
	nodeName: string;
	_canvas: any;
	__internalElement: any;
	_styleMap = new Map();
	_id;
	set id(value){
		this._id = value;
		if(this.__internalElement){
			this.__internalElement._dom.documentElement.setAttribute('id', value);
		}
	}
	get id(){
		return this._id;
	}
	constructor(nodeName) {
		this.emitter = fromObject({});
		this.addEventListener = this.addEventListener.bind(this);
		this.removeEventListener = this.removeEventListener.bind(this);
		this._checkEmitter = this._checkEmitter.bind(this);

		this.style = {
			setProperty: (key, val) => {
				// todo check key + val
				this._styleMap.set(key, val);
				if (this.__internalElement) {
					this.__internalElement._dom.documentElement.setAttribute('style', this._styleToString());
				}
			},
			getPropertyValue: (key) => {
				return this._styleMap.get(key);
			},
		};
		this.className = {
			baseVal: '',
		};
		this.nodeName = nodeName;
	}

	get ownerDocument() {
		return window.document;
	}

	_styleToString() {
		let style = '';
		this._styleMap.forEach((val, key) => {
			style = `${style}${key}:${val};`;
		});
		return style;
	}

	_checkEmitter() {
		if (!this.emitter || !(this.emitter.on || this.emitter.addEventListener || this.emitter.addListener)) {
			this.emitter = fromObject({});
		}
	}

	addEventListener(eventName, listener, thisArg) {
		this._checkEmitter();
		thisArg = thisArg || this;
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
		thisArg = thisArg || this;
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

	getBoundingClientRect() {
		if (this._canvas) {
			return {
				left: 0,
				top: 0,
				right: this._canvas.innerWidth,
				bottom: this._canvas.innerHeight,
				x: 0,
				y: 0,
				width: this._canvas.innerWidth,
				height: this._canvas.innerHeight,
			};
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
