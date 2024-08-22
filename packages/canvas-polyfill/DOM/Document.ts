import { HTMLElement } from './HTMLElement';
import { HTMLVideoElement } from './HTMLVideoElement';
import { HTMLImageElement } from './HTMLImageElement';
import { HTMLCanvasElement } from './HTMLCanvasElement';
import { HTMLDivElement } from './HTMLDivElement';
import { Text } from './Text';
import { Canvas } from '@nativescript/canvas';
import { ContentView, Frame, StackLayout, eachDescendant, knownFolders } from '@nativescript/core';
import { Node } from './Node';
import { Element, parseChildren } from './Element';
import { SVGSVGElement } from './svg/SVGSVGElement';
import { SVGElement } from './svg/SVGElement';
import { HTMLUnknownElement } from './HTMLUnknownElement';
import { HTMLCollection } from './HTMLCollection';

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

export class Document extends Node {
	readonly body: Element = null;
	_documentElement: Element;
	readonly readyState: string;
	readonly head: Element = null;
	__instance: Document;
	get defaultView() {
		return global.window;
	}

	// set as app root
	baseURI: string;

	constructor() {
		super('#document');
		this.body = new Element('BODY');
		this._documentElement = new Element('HTML');
		this.readyState = 'complete';
		this.head = new Element('HEAD');
		this.baseURI = knownFolders.currentApp().path;
	}

	get children() {
		const children = new HTMLCollection();
		children.push(this._documentElement);
		if (this.body) {
			children.push(this.head);
			children.push(this.body);
		}
		if (this.__instance) {
			parseChildren(this.__instance, children);
		}
		return children;
	}

	_createElement(namespaceURI: string | null, tagName: string, options?: {}) {
		if (namespaceURI === 'http://www.w3.org/2000/svg') {
			switch ((tagName || '').toLowerCase()) {
				case 'svg':
					return new SVGSVGElement();
				case 'rect':
					return new SVGRectElement();
				case 'circle':
					return new SVGCircleElement();
				case 'g':
					return new SVGGElement();
				case 'path':
					return new SVGPathElement();
				case 'image':
					return new SVGImageElement();
				case 'polyline':
					return new SVGPolylineElement();
				case 'polygon':
					return new SVGPolygonElement();
				case 'ellipse':
					return new SVGEllipseElement();
				case 'line':
					return new SVGLineElement();
				case 'text':
					return new SVGTextElement();
				case 'radialGradient':
					return new SVGRadialGradientElement();
				case 'linearGradient':
					return new SVGLinearGradientElement();
				case 'use':
					return new SVGUseElement();
				case 'stop':
					return new SVGStopElement();
				default:
					return new SVGElement(tagName ?? `${tagName}`);
			}
		}
		switch ((tagName || '').toLowerCase()) {
			case 'div':
				return new HTMLDivElement();
			case 'video':
				return new HTMLVideoElement();
			case 'img':
				return new HTMLImageElement();
			case 'canvas':
				return new HTMLCanvasElement();
			case 'iframe':
				// Return nothing to keep firebase working.
				return null;
			default:
				return new HTMLUnknownElement(tagName ?? 'null');
		}
	}

	createElement(tagName: string, options?: {}) {
		const element = this._createElement(null, tagName, options) as Element;
		if (element) {
			if (!element.nativeElement) {
				// set dummy nativeElement
				element.nativeElement = new ContentView();
				element.nativeElement.__domElement = new DOMParser().parseFromString(`<${element.nodeName.toLowerCase()}/>`, 'text/html').documentElement as never;
			}

			element._ownerDocument = this as never;
		}
		return element;
	}

	createElementNS(namespaceURI: string | null, qualifiedName?: string, options?: {}) {
		const element = this._createElement(namespaceURI, qualifiedName, options) as Element;
		element._namespaceURI = namespaceURI;
		if (element) {
			if (!element.nativeElement) {
				// set dummy nativeElement
				element.nativeElement = new ContentView();
				element.nativeElement.__domElement = new DOMParser().parseFromString(`<${element.nodeName.toLowerCase()}/>`, 'image/svg+xml').documentElement as never;
			}
			element._ownerDocument = this as never;
		}
		return element;
	}

	createTextNode(data) {
		const text = new Text(data);
		text._ownerDocument = this as never;
		return text;
	}

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
					canvas.nativeElement = nativeElement;
					return canvas;
				} else if (nativeElement instanceof StackLayout) {
					const div = new HTMLDivElement();
					div.nativeElement = nativeElement;
					return div;
				}
				const element = new HTMLElement();
				element.nativeElement = nativeElement;
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

	get documentElement() {
		return this._documentElement;
	}

	//@ts-ignore
	set documentElement(value) {}

	querySelectorAll(selector) {
		if (this.__instance) {
			return this.__instance.querySelectorAll?.(selector) ?? [];
		}
		return [];
	}

	querySelector(selector) {
		const ret = this.__instance?.querySelectorAll?.(selector);
		// let element = ret?.[0] ?? null;
		// if (ret === undefined) {
		// 	const items = (this as any)._instance.getElementsByTagName(selector);
		// 	element = items[0];
		// }
		//
		// if (element) {
		// 	return new (Element as any)(element);
		// }
		return null;
	}
}
