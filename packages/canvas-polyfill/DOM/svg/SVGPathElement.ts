import { Element } from '../Element';
import { Path } from '@nativescript/canvas';

export class SVGPathElement extends Element {
	__internalElement: Path;
	constructor() {
		super('path');
		let path = undefined;
		if (arguments.length > 1) {
			path = arguments[1];
		}

		if (path instanceof Path) {
			this.__internalElement = path;
		} else {
			this.__internalElement = new Path();
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
