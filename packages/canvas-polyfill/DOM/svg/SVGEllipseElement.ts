import { SVGGeometryElement } from './SVGGeometryElement';
import { Ellipse } from '@nativescript/canvas-svg';
import { SVGAnimatedLength } from './SVGAnimatedLength';
export class SVGEllipseElement extends SVGGeometryElement {
	private _cx = new SVGAnimatedLength(this, 'cx');
	private _cy = new SVGAnimatedLength(this, 'cy');
	private _rx = new SVGAnimatedLength(this, 'rx');
	private _ry = new SVGAnimatedLength(this, 'ry');
	constructor() {
		super('ellipse');
		this.nativeElement = new Ellipse();
	}

	get cx() {
		return this._cx;
	}

	get cy() {
		return this._cy;
	}

	get rx() {
		return this._rx;
	}

	get ry() {
		return this._ry;
	}
}
