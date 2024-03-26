import { SVGGradientElement } from './SVGGradientElement';
import { LinearGradient } from '@nativescript/canvas-svg';
import { SVGAnimatedLength } from './SVGAnimatedLength';
export class SVGLinearGradientElement extends SVGGradientElement {
	private _x1 = new SVGAnimatedLength(this, 'x1');
	private _y1 = new SVGAnimatedLength(this, 'y1');
	private _x2 = new SVGAnimatedLength(this, 'x2');
	private _y2 = new SVGAnimatedLength(this, 'y2');

	constructor() {
		super('linearGradient');
		this.nativeElement = new LinearGradient();
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
