import { SVGAnimatedLengthEmpty } from './SVGAnimatedLength';
import { SVGGeometryElement } from './SVGGeometryElement';
import { Rect } from '@nativescript/canvas-svg';
import { SVGAnimatedLength } from './SVGAnimatedLength';
export class SVGRectElement extends SVGGeometryElement {
	_x = new SVGAnimatedLength(this, 'x');
	_y = new SVGAnimatedLength(this, 'y');
	// @ts-ignore
	_width = new SVGAnimatedLength(this, 'width');
	// @ts-ignore
	_height = new SVGAnimatedLength(this, 'height');
	_rx = new SVGAnimatedLength(this, 'rx');
	_ry = new SVGAnimatedLength(this, 'ry');

	constructor() {
		super('rect');
		this.nativeElement = new Rect();
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

	get rx() {
		return this._rx;
	}

	get ry() {
		return this._rx;
	}
}
