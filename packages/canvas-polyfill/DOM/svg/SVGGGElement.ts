import { Element } from '../Element';
import { G } from '@nativescript/canvas';

export class SVGGElement extends Element {
	__internalElement: G;
	constructor() {
		super('g');
		let g = undefined;
		if (arguments.length > 1) {
			g = arguments[1];
		}

		if (g instanceof G) {
			this.__internalElement = g;
		} else {
			this.__internalElement = new G();
		}
	}

	set width(value) {
		this.__internalElement.width = value;
	}

	get width() {
		return this.__internalElement.getMeasuredWidth();
	}

	set height(value) {
		this.__internalElement.height = value;
	}

	get height() {
		return this.__internalElement.getMeasuredHeight();
	}

	setAttribute(key, value) {
		this.__internalElement._dom.documentElement.setAttribute(key, value);
	}

	getAttribute(key) {
		return this.__internalElement._dom.documentElement.getAttribute(key);
	}

	removeAttribute(key, value) {
		this.__internalElement._dom.documentElement.removeAttribute(key, value);
	}
}
