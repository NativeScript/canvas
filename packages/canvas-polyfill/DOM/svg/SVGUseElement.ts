import { SVGGraphicsElement } from './SVGGraphicsElement';
import { Use } from '@nativescript/canvas-svg';
import { SVGAnimatedString } from './SVGAnimatedString';
import { SVGAnimatedLength } from './SVGAnimatedLength';
export class SVGUseElement extends SVGGraphicsElement {
	_href = new SVGAnimatedString(this, 'href');
	_x = new SVGAnimatedLength(this, 'x');
	_y = new SVGAnimatedLength(this, 'y');
	// @ts-ignore
	_width = new SVGAnimatedLength(this, 'width');
	// @ts-ignore
	_height = new SVGAnimatedLength(this, 'height');
	constructor() {
		super('use');
		this.nativeElement = new Use() as never;
	}

	get href() {
		return this._href;
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
	get height() {
		return this._height;
	}
}
