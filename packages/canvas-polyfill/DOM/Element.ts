import { Node } from './Node';
import { eachDescendant, Frame, StackLayout, View, ViewBase } from '@nativescript/core';
import setValue from 'set-value';
import querySelector from 'query-selector';
import { HTMLCollection } from './HTMLCollection';
import { Canvas } from '@nativescript/canvas';
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

export function parseChildren(element, children) {
	if (element) {
		const length = element?.childNodes?.length ?? 0;

		for (let i = 0; i < length; i++) {
			const node = element.childNodes.item(i);
			if (node) {
				switch (node.nodeName) {
					case 'circle':
						{
							const circle = new SVGCircleElement() as unknown as Element;
							circle.nativeElement.__domElement = node;
							children.push(circle);
						}
						break;
					case 'image': {
						const image = new SVGImageElement() as unknown as Element;
						image.nativeElement.__domElement = node;
						children.push(image);
					}
					case 'line':
						{
							const line = new SVGLineElement() as unknown as Element;
							line.nativeElement.__domElement = node;
							children.push(line);
						}
						break;
					case 'polyline':
						{
							const polyline = new SVGPolylineElement() as unknown as Element;
							polyline.nativeElement.__domElement = node;
							children.push(polyline);
						}
						break;
					case 'polygon':
						{
							const polygon = new SVGPolygonElement() as unknown as Element;
							polygon.nativeElement.__domElement = node;
							children.push(polygon);
						}
						break;
					case 'g':
						{
							const g = new SVGGElement() as unknown as Element;
							g.nativeElement.__domElement = node;
							children.push(g);
						}
						break;
					case 'path':
						{
							const path = new SVGPathElement() as unknown as Element;
							path.nativeElement.__domElement = node;
							children.push(path);
						}
						break;
					case 'rect':
						{
							const rect = new SVGRectElement() as unknown as Element;
							rect.nativeElement.__domElement = node;
							children.push(rect);
						}
						break;
					case 'stop':
						{
							const stop = new SVGStopElement() as unknown as Element;
							stop.nativeElement.__domElement = node;
							children.push(stop);
						}
						break;
					case 'use':
						{
							const use = new SVGUseElement() as unknown as Element;
							use.nativeElement.__domElement = node;
							children.push(use);
						}
						break;
					default:
						//	ret.push(node);
						break;
				}
			}
		}
	}
}

function getElementsByClassName(v, clsName) {
	var retVal = [];
	if (!v) {
		return retVal;
	}

	if (v.classList.contains(clsName)) {
		retVal.push(v);
	}

	const classNameCallback = function (child) {
		if (child.classList.contains(clsName)) {
			retVal.push(child);
		}

		// Android patch for ListView
		if (child._realizedItems && child._realizedItems.size !== child._childrenCount) {
			for (var key in child._realizedItems) {
				if (child._realizedItems.hasOwnProperty(key)) {
					classNameCallback(child._realizedItems[key]);
				}
			}
		}

		return true;
	};

	eachDescendant(v, classNameCallback);

	// Android patch for ListView
	if (v._realizedItems && v._realizedItems.size !== v._childrenCount) {
		for (var key in v._realizedItems) {
			if (v._realizedItems.hasOwnProperty(key)) {
				classNameCallback(v._realizedItems[key]);
			}
		}
	}

	return retVal;
}

function getElementsByTagName(v, tagName) {
	// TagName is case-Insensitive
	var tagNameLC = tagName.toLowerCase();

	var retVal = [],
		allTags = false;
	if (!v) {
		return retVal;
	}

	if (tagName === '*') {
		allTags = true;
	}

	if ((v.typeName && v.typeName.toLowerCase() === tagNameLC) || allTags) {
		retVal.push(v);
	}

	const tagNameCallback = function (child) {
		if ((child.typeName && child.typeName.toLowerCase() === tagNameLC) || allTags) {
			retVal.push(child);
		}

		// Android patch for ListView
		if (child._realizedItems && child._realizedItems.size !== child._childrenCount) {
			for (var key in child._realizedItems) {
				if (child._realizedItems.hasOwnProperty(key)) {
					tagNameCallback(child._realizedItems[key]);
				}
			}
		}

		return true;
	};

	eachDescendant(v, tagNameCallback);

	// Android patch for ListView
	if (v._realizedItems && v._realizedItems.size !== v._childrenCount) {
		for (var key in v._realizedItems) {
			if (v._realizedItems.hasOwnProperty(key)) {
				tagNameCallback(v._realizedItems[key]);
			}
		}
	}

	return retVal;
}

type NativeElement = View & { __domElement?: Element };
export class Element extends Node {
	private _classList = new Set();
	_nativeElement: NativeElement;
	private _attrs: Map<string, any> = new Map<string, unknown>();
	private _attributeOriginalValue: Map<string, unknown> = new Map();
	private _jsBuffer: Float32Array;
	private _id: string = '';

	_children = new HTMLCollection();

	_namespaceURI: string | null = null;

	nodeType = Node.ELEMENT_NODE;

	get namespaceURI() {
		return this._namespaceURI;
	}

	set id(value) {
		this._id = value;
		setValue(this.nativeElement, 'id', value);
		setValue(this.nativeElement?.__domElement, 'id', value);
	}

	get id() {
		return this._id;
	}

	set nativeElement(value: NativeElement) {
		this._nativeElement = value;
		if (value) {
			this._emitter = new WeakRef(value);
		} else {
			this._emitter = value as never;
		}
	}

	get nativeElement(): NativeElement {
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
		if (this._children.length) {
			return this._children;
		}
		if (this.ownerDocument?.nodeName === '#document' && this.ownerDocument?.body?.nodeName?.toLowerCase?.() === 'html') {
			this._children.push(document.documentElement);
			this._children.push(document.head);
			this._children.push(document.body);
		}
		const element = (<any>this)?.__domElement ?? (<any>this).nativeElement?.__domElement;
		if (element) {
			parseChildren(element, this._children);
			return this._children;
		}

		return [];
	}

	hasAttribute(key) {
		if (this.nativeElement) {
			return Object.hasOwn(this.nativeElement, key);
		}
		return this._attrs.has(key);
	}

	getAttribute(key: string): string {
		if (this.nativeElement) {
			return this.nativeElement[key] ?? null;
		}
		return this._attrs.get(key) ?? null;
	}

	setAttribute(key: string, value: unknown) {
		if (this.nativeElement) {
			if (!this._attributeOriginalValue.has(key)) {
				this._attributeOriginalValue.set(key, this.nativeElement[key]);
				setValue(this.nativeElement, key, value);
				this.nativeElement?.__domElement?.setAttribute(key, value);
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
			this.nativeElement?.__domElement?.removeAttribute(key);
		} else {
			setValue(this._attrs, key, originalValue);
		}
	}

	setAttributeNS() {}

	removeAttributeNS() {}

	getElementById(id: string) {
		if (this.__instance) {
			return this.__instance.getElementById(id);
		}
		const topmost = Frame.topmost();
		if (topmost) {
			let nativeElement;
			if (topmost.currentPage && topmost.currentPage.modal) {
				nativeElement = topmost.getViewById(id);
			} else {
				nativeElement = topmost.currentPage.modal.getViewById(id);
			}

			if (nativeElement) {
				if (nativeElement instanceof Canvas) {
					const canvas = new HTMLCanvasElement();
					(<any>canvas).nativeElement = nativeElement;
					return canvas;
				} else if (nativeElement instanceof StackLayout) {
					const div = new HTMLDivElement();
					(<any>div).nativeElement = nativeElement;
					return div;
				}
				const element = new HTMLElement();
				(<any>element).nativeElement = nativeElement;
				return element;
			}
		}
		return null;
	}

	getElementsByTagName(tagname: string) {
		if (this.__instance) {
			return this.__instance.getElementsByTagName(tagname);
		}
		const topmost = Frame.topmost();
		if (topmost) {
			let view;
			if (topmost.currentPage && topmost.currentPage.modal) {
				view = topmost;
			} else {
				view = topmost.currentPage.modal;
			}
			const elements = getElementsByTagName(view, tagname);

			return elements;
		}
		return [];
	}

	get __instance() {
		return (<any>this.nativeElement?.__domElement?.ownerDocument)?.__instance;
	}

	getElementsByClassName(className: string) {
		if (this.__instance) {
			return this.__instance.getElementsByClassName(className);
		}
		const topmost = Frame.topmost();
		if (topmost) {
			let view;
			if (topmost.currentPage && topmost.currentPage.modal) {
				view = topmost;
			} else {
				view = topmost.currentPage.modal;
			}
			const elements = getElementsByClassName(view, className);

			return elements;
		}
		return [];
	}

	querySelector(selector: string) {
		const context = (<any>this)._xmlDom?.documentElement ?? (<any>this)._xmlDom ?? this.__instance ?? this.nativeElement?.__domElement;
		const selection = querySelector(selector, context);
		if (Array.isArray(selection)) {
			const item = selection[0];
			if (item) {
				switch (item.nodeName) {
					case 'linearGradient': {
						const ret = new SVGLinearGradientElement() as any;
						ret.__instance = item;
						return ret;
					}
					case 'svg': {
						const ret = new SVGSVGElement() as any;
						ret.__domElement = item;
						return ret;
					}
				}
			}
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
		if (this.nativeElement) {
			return this.nativeElement.__domElement?.attributes;
		}
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
		if (this.nativeElement) {
			return this.nativeElement['innerWidth'] ?? (this.nativeElement['width'] as never);
		}
		return this['width'];
	}

	get innerHeight() {
		if (this.nativeElement) {
			return this.nativeElement['innerHeight'] ?? (this.nativeElement['height'] as never);
		}
		return this['height'];
	}
	/*

	get width() {
		if (this.nativeElement) {
			return this.nativeElement['width'] as never;
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
			return this.nativeElement['height'] as never;
		}
		return this._height;
	}

	set height(value: number) {
		this._height = value;
		if (this.nativeElement) {
			setValue(this.nativeElement, 'height', value);
		}
	}
	*/

	get ontouchstart() {
		return {};
	}

	setPointerCapture(id: string) {}

	releasePointerCapture(id: string) {}
}
