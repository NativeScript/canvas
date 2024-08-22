import { Element } from '../Element';
import { Svg } from '@nativescript/canvas-svg';
import { SVGAnimatedString } from './SVGAnimatedString';
import { Text } from '../Text';
import { SVGAnimatedTransformList } from './SVGAnimatedTransformList';

export class SVGElement extends Element {
	private _className = new SVGAnimatedString(this, 'className');
	__domElement: Element;
	constructor(tagName: string) {
		super(tagName ?? '');
	}

	private get nativeValue(): Svg {
		return this.nativeElement as never;
	}

	// @ts-ignore
	get className() {
		return this._className;
	}

	set className(value: unknown) {}

	_appendChild(view, redraw = true) {
		if (view?.nativeElement?.__domElement) {
			this.nativeValue?.__domElement?.appendChild?.(view.nativeElement.__domElement);
			if (redraw) {
				(<any>this.nativeValue).__redraw();
			}
			(<any>this.nativeValue)?.addChild?.(view.nativeElement);
			return view;
		} else if (view instanceof Text) {
			const dom = this.__domElement ?? this.nativeElement.__domElement;
			const text = dom.ownerDocument?.createTextNode?.(view.data);
			view.__domNode = text;
			dom?.appendChild?.(text);
			return view;
		} else if (view.__domNode) {
			const dom = this.__domElement ?? this.nativeElement.__domElement;
			dom?.appendChild?.(view.__domNode);
			return view;
		}
		return null;
	}

	appendChild(view) {
		return this._appendChild(view, true);
	}

	append(views: Array<any>) {
		for (const view of arguments) {
			this._appendChild(view, false);
		}
		(<any>this.nativeValue).__redraw();
	}

	insertBefore(view) {}

	removeChild(view) {}

	setAttribute(key, value) {
		const dom = this.nativeElement.__domElement;
		if (dom) {
			dom.setAttribute?.(key, value);
		} else {
			super.setAttribute(key, value);
		}
	}

	getAttribute(key): string | null {
		const dom = this.nativeElement.__domElement;
		if (dom) {
			return (dom.getAttribute?.(key) as never) ?? null;
		}
		return (super.getAttribute(key) as never) ?? null;
	}

	removeAttribute(key) {
		const dom = this.nativeElement.__domElement;
		if (dom) {
			return dom.removeAttribute?.(key);
		}
	}
}
