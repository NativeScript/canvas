import { SVGGeometryElement } from './SVGGeometryElement';
import { Circle } from '@nativescript/canvas-svg';
import { SVGAnimatedLength } from './SVGAnimatedLength';

export class SVGCircleElement extends SVGGeometryElement {
	private _cx = new SVGAnimatedLength(this, 'cx');
	private _cy = new SVGAnimatedLength(this, 'cy');
	private _r = new SVGAnimatedLength(this, 'r');
	constructor() {
		super('circle');
		this.nativeElement = new Circle();
	}

	get cx() {
		return this._cx;
	}

	get cy() {
		return this._cy;
	}

	get r() {
		return this._r;
	}
}
