import { HTMLElement } from './HTMLElement';
import { HTMLVideoElement } from './HTMLVideoElement';
import { HTMLImageElement } from './HTMLImageElement';
import { HTMLCanvasElement } from './HTMLCanvasElement';
import { HTMLDivElement } from './HTMLDivElement';
import { Text } from './Text';
import { Canvas } from '@nativescript/canvas';
import { Frame, StackLayout } from '@nativescript/core';
import { Node } from './Node';
import { Element } from './Element';
import { SVGSVGElement } from './svg/SVGSVGElement';
export class Document extends Node {
	readonly body: Element;
	private _documentElement: Element;
	private readyState: string;
	readonly head: Element;
	__instance: Document;
	get defaultView() {
		return global.window;
	}

	constructor() {
		super('#document');
		this.body = new Element('BODY');
		this.documentElement = new Element('HTML');
		this.readyState = 'complete';
		this.head = new Element('HEAD');
	}

	createElement(tagName: string) {
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
			case 'svg':
				return new SVGSVGElement('svg');
			case 'rect':
				return new SVGRectElement();
			case 'circle':
				return new SVGCircleElement();
			case 'g':
				return new SVGGElement();
			case 'path':
				return new SVGPathElement();
			default:
				return new Element(tagName);
		}
	}

	createElementNS(namespaceURI: string, qualifiedName?: string, options?: {}) {
		const element = this.createElement(qualifiedName) as any;
		element.namespaceURI = namespaceURI;
		if (qualifiedName?.toLowerCase?.() !== 'canvas') {
			element.toDataURL = () => ({});
		}
		return element;
	}

	createTextNode(data) {
		return new Text(data);
	}

	getElementById(id: string) {
		if (this.__instance) {
			return this.__instance.getElementById(id);
		}
		const topmost = Frame.topmost();
		if (topmost) {
			const nativeElement = topmost.getViewById(id);
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
		return [];
	}

	get documentElement() {
		if (this._documentElement) {
			return this._documentElement;
		}
		if (this.__instance) {
			return this.__instance?.documentElement;
		}
		return null;
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
