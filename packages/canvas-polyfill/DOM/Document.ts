import { Element } from './Element';
import { HTMLVideoElement } from './HTMLVideoElement';
import { HTMLImageElement } from './HTMLImageElement';
import { HTMLCanvasElement } from './HTMLCanvasElement';
import { SVGElement, SVGCircleElement, SVGRectElement, SVGGElement, SVGPathElement } from './svg';
import { Text } from './Text';
import { Canvas } from '@nativescript/canvas';
import { Frame } from '@nativescript/core';

export class Document extends Element {
	private body: any;
	documentElement: any;
	private readyState: string;
	private head: any;
	private defaultView: any;

	constructor() {
		super('#document');
		this.body = new Element('BODY');
		this.documentElement = new Element('HTML');
		this.readyState = 'complete';
		this.head = new Element('HEAD');
		this.defaultView = (global as any).window;
	}

	createElement(tagName) {
		switch ((tagName || '').toLowerCase()) {
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
				return new SVGElement();
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

	createElementNS(...args) {
		let name;
		let namespaceURI;
		if (Array.isArray(args) && args.length === 1) {
			name = args[0];
		} else if (Array.isArray(args) && args.length > 1) {
			name = args[1];
			namespaceURI = args[0];
		} else {
			name = args;
		}
		const element = this.createElement(name) as any;
		element.namespaceURI = namespaceURI;
		element.toDataURL = () => ({});
		return element;
	}

	createTextNode(data) {
		return new Text(data);
	}

	getElementById(id) {
		const topmost = Frame.topmost();
		if (id instanceof Canvas) {
			const canvas = new HTMLCanvasElement();
			canvas._canvas = id;
			return canvas;
		}
		if (topmost) {
			const nativeElement = topmost.getViewById(id);
			if (nativeElement) {
				const element = new Element('div');
				element.nativeElement = nativeElement;
				return element;
			}
		}
		return new Element('div');
	}
}
