import { Element } from '../Element';
import { Svg } from '@nativescript/canvas';

export class SVGElement extends Element {
	//__internalElement: Svg;
	constructor() {
		super('svg');
		let svg = undefined;
		if (arguments.length > 1) {
			svg = arguments[1];
		}

		if (svg instanceof Svg) {
			this.__internalElement = svg;
		} else {
			this.__internalElement = new Svg();
		}
	}
	
	//@ts-ignore
	get nativeElement() {
		return this.__internalElement;
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

	appendChild(view) {
		this.__internalElement._dom.documentElement.appendChild(view.__internalElement._dom);
		this.__internalElement.__redraw();
		return view;
	}

	insertBefore(view) {
		const v = arguments[0];
		const c = arguments[1];
		if (v && c) {
			this.__internalElement._dom.documentElement.insertBefore(v.__internalElement._dom, c.__internalElement._dom);
			return v;
		}
	}

	removeChild(view) {
		console.log(this.nodeName, 'removeChild', view);
	}
}
