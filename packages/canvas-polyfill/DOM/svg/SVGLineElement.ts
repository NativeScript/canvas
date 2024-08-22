import { SVGAnimatedLength } from './SVGAnimatedLength';
import { SVGGeometryElement } from './SVGGeometryElement';
import { Line } from '@nativescript/canvas-svg';

export class SVGLineElement extends SVGGeometryElement {
	_x1 = new SVGAnimatedLength(this, 'x1');
	_y1 = new SVGAnimatedLength(this, 'y1');
	_x2 = new SVGAnimatedLength(this, 'x2');
	_y2 = new SVGAnimatedLength(this, 'y2');
	constructor() {
		super('line');
		this.nativeElement = new Line() as never;
	}

	get x1() {
		return this._x1;
	}

	get y1() {
		return this._y1;
	}

	get x2() {
		return this._x2;
	}

	get y2() {
		return this._y2;
	}
}
