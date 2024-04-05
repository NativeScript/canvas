import { SVGGraphicsElement } from './SVGGraphicsElement';
import { Image } from '@nativescript/canvas-svg';
import { SVGAnimatedLength } from './SVGAnimatedLength';
export class SVGImageElement extends SVGGraphicsElement {
	private _x = new SVGAnimatedLength(this, 'x');
	private _y = new SVGAnimatedLength(this, 'y');
	// @ts-ignore
	private _width = new SVGAnimatedLength(this, 'width');
	// @ts-ignore
	private _height = new SVGAnimatedLength(this, 'height');

	constructor() {
		super('image');
		this.nativeElement = new Image() as never;
	}

	get decoding() {
		return (this.getAttribute('decoding') as never) ?? 'default';
	}

	set decoding(value: 'sync' | 'async' | 'default') {
		this.setAttribute('decoding', value);
	}

	get x() {
		return this._x;
	}

	get y() {
		return this._y;
	}

	// @ts-ignore
	get width() {
		return this._width;
	}

	// @ts-ignore
	set width(value: unknown) {
		// noop ??
	}

	// @ts-ignore
	get height() {
		return this._height;
	}

	// @ts-ignore
	set height(value: unknown) {}
}
