import { Node } from './Node';
import { ViewBase } from '@nativescript/core';
import setValue from 'set-value';
import querySelector from 'query-selector';

declare const NSCanvas;

export class DOMRectReadOnly {
	readonly bottom: number;
	readonly height: number;
	readonly left: number;
	readonly right: number;
	readonly top: number;
	readonly width: number;
	readonly x: number;
	readonly y: number;

	constructor(x: number, y: number, width: number, height: number, top?: number, right?: number, bottom?: number, left?: number) {
		this.x = x;
		this.y = y;
		this.width = width;
		this.height = height;
		this.left = left ?? x;
		this.top = top ?? y;
		this.right = right ?? x + width;
		this.bottom = bottom ?? y + height;
	}
}

export class DOMRect extends DOMRectReadOnly {
	constructor(x: number, y: number, width: number, height: number, top?: number, right?: number, bottom?: number, left?: number) {
		super(x, y, width, height, top, right, bottom, left);
	}
}

export class Element extends Node {
	private _classList = new Set();
	_nativeElement: ViewBase;
	private _width: number;
	private _height: number;
	private _attrs: Map<string, any> = new Map<string, unknown>();
	private _attributeOriginalValue: Map<string, unknown> = new Map();
	private _jsBuffer: Float32Array;

	set nativeElement(value) {
		this._nativeElement = value;
		if (value) {
			this._emitter = new WeakRef(value);
		} else {
			this._emitter = value as never;
		}
	}

	get nativeElement() {
		return this._nativeElement;
	}

	private get _boundingClientRect() {
		if (this._jsBuffer === undefined) {
			this._jsBuffer = new Float32Array(8);
		}
		return this._jsBuffer;
	}

	constructor(tagName: string) {
		super(tagName.toUpperCase());
	}

	get classList() {
		return this._classList;
	}

	get tagName() {
		return this.nodeName;
	}

	get children() {
		const element = (<any>this)._xmlDom?.documentElement ?? (<any>this)._xmlDom;
		if (element) {
			const ret = [];
			const length = element?.childNodes?.length ?? 0;

			for (let i = 0; i < length; i++) {
				const node = element.childNodes.item(i);
				if (node) {
					switch (node.nodeName) {
						case 'line':
							{
								const line = new SVGLineElement() as any;
								line.__instance = node;
								ret.push(line);
							}
							break;
						case 'polyline':
							{
								const polyline = new SVGPolylineElement() as any;
								polyline.__instance = node;
								ret.push(polyline);
							}
							break;
						case 'g':
							{
								const g = new SVGGElement() as any;
								g.__instance = node;
								ret.push(g);
							}
							break;
						case 'path':
							{
								const path = new SVGPathElement() as any;
								path.__instance = node;
								ret.push(path);
							}
							break;
						case 'rect':
							{
								const rect = new SVGRectElement() as any;
								rect.__instance = node;
								ret.push(rect);
							}
							break;
					}
				}
			}

			return ret;
		}

		return [];
	}

	getAttribute(key: string): unknown {
		if (this.nativeElement) {
			return this.nativeElement[key];
		}
		return this._attrs.get(key);
	}

	setAttribute(key: string, value: unknown) {
		if (this.nativeElement) {
			if (!this._attributeOriginalValue.has(key)) {
				this._attributeOriginalValue.set(key, this.nativeElement[key]);
				setValue(this.nativeElement, key, value);
			}
		} else {
			if (!this._attributeOriginalValue.has(key)) {
				this._attributeOriginalValue.set(key, this._attrs.get(key));
				setValue(this._attrs, key, value);
			}
		}
	}

	removeAttribute(key: string) {
		const originalValue = this._attributeOriginalValue.get(key);
		this._attributeOriginalValue.delete(key);

		if (this.nativeElement) {
			setValue(this.nativeElement, key, originalValue);
		} else {
			setValue(this._attrs, key, originalValue);
		}
	}

	setAttributeNS() {}

	removeAttributeNS() {}

	querySelector(selector: string) {
		const selection = querySelector(selector, this);
		if (Array.isArray(selection)) {
			return selection[0];
		}
		return null;
	}

	querySelectorAll(selector: string) {
		return querySelector(selector, this);
	}

	getBoundingClientRect() {
		const nativeElement = this['nativeElement'];
		if (this.nodeName === 'CANVAS') {
			return (<any>nativeElement).getBoundingClientRect();
		}
		if (nativeElement) {
			if (global.isIOS) {
				NSCanvas.getBoundingClientRect(nativeElement.nativeView, this._boundingClientRect);
			}

			if (globalThis.isAndroid) {
				nativeElement.nativeView.getBoundingClientRect(this._boundingClientRect);
			}

			const rectBuffer = this._boundingClientRect;
			return new DOMRect(rectBuffer[6], rectBuffer[7], rectBuffer[4], rectBuffer[5], rectBuffer[0], rectBuffer[1], rectBuffer[2], rectBuffer[3]);
		}
		return {
			left: 0,
			top: 0,
			right: this.innerWidth,
			bottom: this.innerHeight,
			x: 0,
			y: 0,
			width: this.innerWidth,
			height: this.innerHeight,
		};
	}

	get attributes() {
		return [];
	}

	get clientWidth() {
		return this.innerWidth;
	}

	get clientHeight() {
		return this.innerHeight;
	}

	get offsetWidth() {
		return this.innerWidth;
	}

	get offsetHeight() {
		return this.innerHeight;
	}

	get innerWidth() {
		return this.width;
	}

	get innerHeight() {
		return this.height;
	}

	get width() {
		if (this.nativeElement) {
			return this.nativeElement['width'];
		}
		return this._width;
	}

	set width(value: number) {
		this._width = value;
		if (this.nativeElement) {
			setValue(this.nativeElement, 'width', value);
		}
	}

	get height() {
		if (this.nativeElement) {
			return this.nativeElement['height'];
		}
		return this._height;
	}

	set height(value: number) {
		this._height = value;
		if (this.nativeElement) {
			setValue(this.nativeElement, 'height', value);
		}
	}

	get ontouchstart() {
		return {};
	}

	setPointerCapture(id: string) {}

	releasePointerCapture(id: string) {}
}
