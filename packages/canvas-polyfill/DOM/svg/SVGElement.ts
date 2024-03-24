import { Element } from '../Element';
import { Svg } from '@nativescript/canvas-svg';
import { SVGTransformList } from './SVGTransform';

function parseViewBox(value: string) {
	if (typeof value === 'string') {
		const vb = value.split(' ');
		const rect = { x: 0, y: 0, width: 0, height: 0 };
		if (vb?.length > 0) {
			const x = parseFloat(vb[0]);
			const y = parseFloat(vb[1]);
			const width = parseFloat(vb[2]);
			const height = parseFloat(vb[3]);
			if (isNaN(x)) {
				rect.x = x;
			}

			if (isNaN(y)) {
				rect.y = y;
			}

			if (isNaN(width)) {
				rect.width = width;
			}

			if (isNaN(height)) {
				rect.height = height;
			}
		}
		return {
			baseVal: rect,
			animVal: rect,
		};
	}
	return null;
}

function parseTransform(value: string) {
	if (typeof value === 'string') {
		const vb = value.split(' ');
		const list = new SVGTransformList();
		if (vb?.length > 0) {
		}
		return {
			baseVal: list,
			animVal: list,
		};
	}
}

export class SVGElement extends Element {
	//__internalElement: Svg;
	constructor(tagName: string) {
		super(tagName ?? '');
	}

	private get nativeValue(): Svg {
		return this.nativeElement as never;
	}

	get _xmlDom() {
		if ((<any>this).__instance) {
			return (<any>this).__instance;
		}
		if (this.nativeValue) {
			return this.nativeValue?._dom;
		}
		return null;
	}

	get viewBox() {
		const viewBox = (this._xmlDom?.documentElement ?? this._xmlDom)?.getAttribute?.('viewBox');
		return parseViewBox(viewBox);
	}

	get transform() {
		const transform = (this._xmlDom?.documentElement ?? this._xmlDom)?.getAttribute?.('transform');
		return parseTransform(transform);
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

	removeChild(view) {}

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
