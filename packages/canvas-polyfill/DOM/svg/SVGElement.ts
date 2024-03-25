import { Element } from '../Element';
import { Svg } from '@nativescript/canvas-svg';
import { SVGTransformList } from './SVGTransform';
import { SVGAnimatedString } from './SVGAnimatedString';

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

	private _className = new SVGAnimatedString(this);

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

	get transform() {
		const transform = (this._xmlDom?.documentElement ?? this._xmlDom)?.getAttribute?.('transform');
		return parseTransform(transform);
	}

	// @ts-ignore
	get className() {
		return this._className;
	}

	set className(value: unknown) {}

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
		const dom = this._xmlDom?.documentElement ?? this._xmlDom;
		if (dom) {
			dom.setAttribute?.(key, value);
		} else {
			super.setAttribute(key, value);
		}
	}

	getAttribute(key) {
		const dom = this._xmlDom?.documentElement ?? this._xmlDom;
		if (dom) {
			return dom.getAttribute?.(key);
		}
		return super.getAttribute(key);
	}

	removeAttribute(key) {
		const dom = this._xmlDom?.documentElement ?? this._xmlDom;
		if (dom) {
			return dom.removeAttribute?.(key);
		}
	}
}
