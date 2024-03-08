import { Element } from '../Element';
import { Circle } from '@nativescript/canvas';

export class SVGCircleElement extends Element {
	//__internalElement: Circle;
	constructor() {
		super('circle');
		let circle = undefined;
		if (arguments.length > 1) {
			circle = arguments[1];
		}

		if (circle instanceof Circle) {
			this.__internalElement = circle;
		} else {
			this.__internalElement = new Circle();
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
