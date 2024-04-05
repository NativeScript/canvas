import { SVGGradientElement } from './SVGGradientElement';
import { SVGAnimatedLength } from './SVGAnimatedLength';
import { RadialGradient } from '@nativescript/canvas-svg';
export class SVGRadialGradientElement extends SVGGradientElement {
	private _cx = new SVGAnimatedLength(this, 'cx');
	private _cy = new SVGAnimatedLength(this, 'cy');
	private _r = new SVGAnimatedLength(this, 'r');
	private _ry = new SVGAnimatedLength(this, 'ry');

	private _fx = new SVGAnimatedLength(this, 'fx');
	private _fy = new SVGAnimatedLength(this, 'fy');
	constructor() {
		super('radialGradient');
		this.nativeElement = new RadialGradient() as never;
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

	get ry() {
		return this._ry;
	}

	get fx() {
		return this._fx;
	}

	get fy() {
		return this._fy;
	}
}
