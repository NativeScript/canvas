import { Element } from '../Element';
import { Rect } from '@nativescript/canvas';

export class SVGRectElement extends Element {
	// __internalElement: Rect;
	constructor() {
		super('rect');
		let rect = undefined;
		if (arguments.length > 1) {
			rect = arguments[1];
		}

		if (rect instanceof Rect) {
			this.__internalElement = rect;
		} else {
			this.__internalElement = new Rect();
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
