import { Element } from '../Element';
import { Svg } from '@nativescript/canvas-svg';

export class SVGElement extends Element {
	//__internalElement: Svg;
	constructor(tagName: string) {
		super(tagName ?? '');
	}

	private get nativeValue(): Svg {
		return this.nativeElement as never;
	}


	appendChild(view) {
		if (view?.nativeElement?._dom) {
			this.nativeValue?._dom?.documentElement?.appendChild?.(view.nativeElement._dom);
			(<any>this.nativeValue).__redraw();
			return view;
		}
		return null;
	}

	insertBefore(view) {
		// const v = arguments[0];
		// const c = arguments[1];
		// if (v && c) {
		// 	this.__internalElement._dom.documentElement.insertBefore(v.__internalElement._dom, c.__internalElement._dom);
		// 	return v;
		// }
	}

	removeChild(view) {
	}


	setAttribute(key, value) {
		super.setAttribute(key, value);
		if (this.nativeElement) {
			this.nativeValue._dom.documentElement.setAttribute(key, value);
		}
	}

	getAttribute(key) {
		if (this.nativeElement) {
			return this.nativeValue._dom.documentElement.getAttribute(key) ?? super.getAttribute(key);
		}
		return super.getAttribute(key);
	}

	removeAttribute(key) {
		if (this.nativeElement) {
			this.nativeValue._dom.documentElement.removeAttribute(key);
		}
	}
}
